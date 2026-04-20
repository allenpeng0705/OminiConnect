//! Integration tests for **AI gateway forwarding**: `default_backend` / `routes[].backend_base`,
//! full-path join to the backend, and **adapter `type`** (`openai` vs `anthropic`) only on
//! `POST /v1/chat/completions`.
//!
//! Complements `panda-config` `effective_*` unit tests with real `dispatch` → mock HTTP upstream.

use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use panda_config::PandaConfig;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use super::test_proxy_state;
use crate::dispatch;

fn find_headers_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|i| i + 4)
}

fn parse_content_length(headers: &str) -> Option<usize> {
    for line in headers.lines() {
        let lower = line.to_ascii_lowercase();
        if lower.starts_with("content-length:") {
            return line.split(':').nth(1).and_then(|s| s.trim().parse().ok());
        }
    }
    None
}

/// Accept one connection, read a full HTTP/1.x request (headers + Content-Length body), return request line + body.
/// Response body is sent to Panda (`text_ok` = plain `ok`; Anthropic adapter tests need JSON).
async fn mock_upstream_capture_one(
    listener: &TcpListener,
    response_body: &[u8],
    json: bool,
) -> (String, Vec<u8>) {
    let (mut sock, _) = listener.accept().await.expect("mock accept");
    let mut buf: Vec<u8> = Vec::new();
    let mut tmp = [0u8; 8192];
    let header_end = loop {
        let n = sock.read(&mut tmp).await.expect("mock read");
        assert!(n > 0, "mock eof before headers");
        buf.extend_from_slice(&tmp[..n]);
        if let Some(end) = find_headers_end(&buf) {
            break end;
        }
        assert!(buf.len() < 512 * 1024, "headers too large");
    };
    let headers_str = std::str::from_utf8(&buf[..header_end]).expect("utf8 headers");
    let request_line = headers_str.lines().next().unwrap_or("").to_string();
    let mut body = buf[header_end..].to_vec();
    if let Some(cl) = parse_content_length(headers_str) {
        while body.len() < cl {
            let n = sock.read(&mut tmp).await.expect("body read");
            assert!(n > 0, "eof before full body");
            body.extend_from_slice(&tmp[..n]);
        }
        body.truncate(cl);
    }
    let ct = if json {
        "application/json; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    };
    let resp = format!(
        "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: {ct}\r\nContent-Length: {}\r\n\r\n",
        response_body.len()
    );
    let _ = sock.write_all(resp.as_bytes()).await;
    let _ = sock.write_all(response_body).await;
    (request_line, body)
}

async fn spawn_panda_dispatch(
    state: Arc<crate::ProxyState>,
    accepts: usize,
) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("panda bind");
    let addr = listener.local_addr().unwrap();
    let st = Arc::clone(&state);
    let server = tokio::spawn(async move {
        for _ in 0..accepts {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let io = TokioIo::new(stream);
            let st2 = Arc::clone(&st);
            let svc = service_fn(move |req| {
                let s = Arc::clone(&st2);
                dispatch(req, s)
            });
            let _ = http1::Builder::new()
                .serve_connection(io, svc)
                .with_upgrades()
                .await;
        }
    });
    (addr, server)
}

async fn tcp_request(addr: std::net::SocketAddr, raw: &[u8]) -> String {
    let mut c = TcpStream::connect(addr).await.expect("connect panda");
    c.write_all(raw).await.unwrap();
    let mut buf = vec![];
    c.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).into_owned()
}

#[tokio::test]
async fn forward_get_uses_default_backend_and_preserves_full_path() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_addr = upstream.local_addr().unwrap();
    let mock = tokio::spawn(async move {
        let (line, body) = mock_upstream_capture_one(&upstream, b"ok", false).await;
        assert!(line.starts_with("GET /v1/health?x=1 "));
        assert!(body.is_empty());
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{up_addr}'
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (panda_addr, server) = spawn_panda_dispatch(state, 1).await;

    let raw = b"GET /v1/health?x=1 HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n";
    let resp = tcp_request(panda_addr, raw).await;
    assert!(resp.contains("200"), "{resp}");

    server.await.ok();
    mock.await.ok();
}

#[tokio::test]
async fn forward_longest_prefix_route_picks_backend_base() {
    let up_default = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_chat = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let d_addr = up_default.local_addr().unwrap();
    let c_addr = up_chat.local_addr().unwrap();

    let mock = tokio::spawn(async move {
        let (line, _) = mock_upstream_capture_one(&up_chat, b"ok", false).await;
        assert!(
            line.starts_with("GET /v1/chat/completions "),
            "unexpected: {line}"
        );

        let (line2, _) = mock_upstream_capture_one(&up_default, b"ok", false).await;
        assert!(
            line2.starts_with("GET /v1/embeddings "),
            "unexpected: {line2}"
        );
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{d_addr}'
tpm:
  enforce_budget: false
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://{c_addr}'
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (panda_addr, server) = spawn_panda_dispatch(state, 2).await;

    let raw = b"GET /v1/chat/completions HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n";
    let _ = tcp_request(panda_addr, raw).await;

    let raw2 = b"GET /v1/embeddings HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n";
    let _ = tcp_request(panda_addr, raw2).await;

    server.await.ok();
    mock.await.ok();
}

#[tokio::test]
async fn forward_post_non_chat_json_not_rewritten_to_anthropic() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_addr = upstream.local_addr().unwrap();

    let mock = tokio::spawn(async move {
        let (line, body) = mock_upstream_capture_one(&upstream, b"ok", false).await;
        assert!(
            line.starts_with("POST /v1/embeddings "),
            "path should stay OpenAI embeddings: {line}"
        );
        let v: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert!(v.get("input").is_some(), "embeddings body: {v}");
        assert!(
            v.get("max_tokens").is_none(),
            "Anthropic adapter would add different fields: {v}"
        );
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{up_addr}'
adapter:
  provider: anthropic
routes:
  - path_prefix: /v1/chat
    backend_base: 'http://{up_addr}'
    type: anthropic
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (panda_addr, server) = spawn_panda_dispatch(state, 1).await;

    let body = r#"{"model":"m","input":"x"}"#;
    let req = format!(
        "POST /v1/embeddings HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let resp = tcp_request(panda_addr, req.as_bytes()).await;
    assert!(resp.contains("200"), "{resp}");

    server.await.ok();
    mock.await.ok();
}

#[tokio::test]
async fn forward_post_chat_openai_preserves_path_and_openai_json() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_addr = upstream.local_addr().unwrap();

    let mock = tokio::spawn(async move {
        let (line, body) = mock_upstream_capture_one(&upstream, b"ok", false).await;
        assert!(
            line.starts_with("POST /v1/chat/completions "),
            "{line}"
        );
        let v: serde_json::Value = serde_json::from_slice(&body).expect("json body");
        assert_eq!(v.get("model").and_then(|x| x.as_str()), Some("gpt-test"));
        assert!(v.get("messages").is_some());
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{up_addr}'
adapter:
  provider: openai
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (panda_addr, server) = spawn_panda_dispatch(state, 1).await;

    let body = r#"{"model":"gpt-test","messages":[{"role":"user","content":"hi"}]}"#;
    let req = format!(
        "POST /v1/chat/completions HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let resp = tcp_request(panda_addr, req.as_bytes()).await;
    assert!(resp.contains("200"), "{resp}");

    server.await.ok();
    mock.await.ok();
}

#[tokio::test]
async fn forward_post_chat_anthropic_rewrites_to_messages_path() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_addr = upstream.local_addr().unwrap();

    let anthropic_resp = br#"{"model":"claude-test","content":[{"type":"text","text":"y"}]}"#;
    let mock = tokio::spawn(async move {
        let (line, body) = mock_upstream_capture_one(&upstream, anthropic_resp, true).await;
        assert!(
            line.starts_with("POST /v1/messages "),
            "Anthropic API path expected, got: {line}"
        );
        let v: serde_json::Value = serde_json::from_slice(&body).expect("json body");
        assert!(v.get("model").is_some());
        assert!(v.get("messages").is_some());
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{up_addr}'
adapter:
  provider: anthropic
  anthropic_version: '2023-06-01'
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (panda_addr, server) = spawn_panda_dispatch(state, 1).await;

    let body = r#"{"model":"claude-test","messages":[{"role":"user","content":"hello"}]}"#;
    let req = format!(
        "POST /v1/chat/completions HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let resp = tcp_request(panda_addr, req.as_bytes()).await;
    assert!(resp.contains("200"), "{resp}");

    server.await.ok();
    mock.await.ok();
}

#[tokio::test]
async fn forward_get_chat_path_not_anthropic_even_when_provider_anthropic() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_addr = upstream.local_addr().unwrap();

    let mock = tokio::spawn(async move {
        let (line, body) = mock_upstream_capture_one(&upstream, b"ok", false).await;
        assert!(
            line.starts_with("GET /v1/chat/completions "),
            "GET must not rewrite to /v1/messages: {line}"
        );
        assert!(body.is_empty());
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{up_addr}'
adapter:
  provider: anthropic
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (panda_addr, server) = spawn_panda_dispatch(state, 1).await;

    let raw = b"GET /v1/chat/completions HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n";
    let resp = tcp_request(panda_addr, raw).await;
    assert!(resp.contains("200"), "{resp}");

    server.await.ok();
    mock.await.ok();
}
