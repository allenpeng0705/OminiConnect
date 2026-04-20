//! End-to-end workflow tests: **client → API gateway (ingress) → MCP gateway → API gateway (egress) → backends**.
//! Backends covered: plain REST (`http_tool` / `http_tools`), **remote MCP JSON-RPC** (`remote_mcp_url`),
//! and **stdio** MCP (`tests/mcp_mock_stdio.py` + Python on `PATH`).
//! Toggle layers via YAML to match different deployment shapes.
//!
//! Mock tasks are often spawned with [`tokio::spawn`] and **not** awaited: an extra blocking `accept` is normal
//! (same pattern as `ingress_mcp_http_tools_call_uses_tool_cache_second_hit`).

use std::path::PathBuf;
use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use panda_config::PandaConfig;
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use super::{mcp_streamable_accept_value, parse_mcp_session_id_from_raw_http, test_proxy_state};
use crate::api_gateway::egress::EgressClient;
use crate::inbound::mcp;
use crate::ProxyState;

async fn spawn_dispatch_stack(
    state: Arc<ProxyState>,
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
                crate::dispatch(req, s)
            });
            let _ = http1::Builder::new()
                .serve_connection(io, svc)
                .with_upgrades()
                .await;
        }
    });
    (addr, server)
}

fn mcp_init_body() -> &'static str {
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"wf","version":"0"}}}"#
}

/// One JSON-RPC POST per TCP connection: `initialize`, `notifications/initialized`, `tools/list`, `tools/call`.
/// Matches [`crate::inbound::mcp_http_remote`] test harness.
async fn mock_remote_mcp_upstream(listener: &TcpListener) {
    let Ok((mut sock, _)) = listener.accept().await else {
        return;
    };
    let mut buf = vec![0u8; 64 * 1024];
    let Ok(n) = sock.read(&mut buf).await else {
        return;
    };
    let req = std::str::from_utf8(&buf[..n]).expect("utf8");
    let Some(body_start) = req.find("\r\n\r\n") else {
        return;
    };
    let body = req[body_start + 4..].trim();
    let Ok(v) = serde_json::from_str::<Value>(body) else {
        return;
    };
    let method = v.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let mid = v
        .get("id")
        .and_then(|x| {
            x.as_u64()
                .or_else(|| x.as_str().and_then(|s| s.parse().ok()))
        })
        .unwrap_or(0);
    let (status_line, resp_body) = match method {
        "initialize" => (
            "HTTP/1.1 200 OK",
            format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":{mid},\"result\":{{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{{}},\"serverInfo\":{{\"name\":\"mock\"}}}}}}"
            ),
        ),
        "notifications/initialized" => ("HTTP/1.1 200 OK", String::new()),
        "tools/list" => (
            "HTTP/1.1 200 OK",
            format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":{mid},\"result\":{{\"tools\":[{{\"name\":\"alpha\",\"description\":\"d\",\"inputSchema\":{{\"type\":\"object\"}}}}]}}}}"
            ),
        ),
        "tools/call" => (
            "HTTP/1.1 200 OK",
            format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":{mid},\"result\":{{\"content\":[{{\"type\":\"text\",\"text\":\"remote_ok\"}}],\"isError\":false}}}}"
            ),
        ),
        _ => ("HTTP/1.1 400 Bad Request", "{}".to_string()),
    };
    let cl = resp_body.len();
    let resp = if cl == 0 {
        format!("{status_line}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n")
    } else {
        format!(
            "{status_line}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {cl}\r\n\r\n{resp_body}"
        )
    };
    let _ = sock.write_all(resp.as_bytes()).await;
}

fn detect_python_cmd() -> Option<&'static str> {
    if std::process::Command::new("python3")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        Some("python3")
    } else if std::process::Command::new("python")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        Some("python")
    } else {
        None
    }
}

async fn raw_mcp_post(
    addr: std::net::SocketAddr,
    path: &str,
    headers_extra: &str,
    body: &str,
) -> String {
    let req = format!(
        "POST {} HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{}\r\n{}",
        path,
        mcp_streamable_accept_value(),
        body.len(),
        headers_extra,
        body
    );
    let mut c = TcpStream::connect(addr).await.expect("connect panda");
    c.write_all(req.as_bytes()).await.unwrap();
    let mut buf = vec![];
    c.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).into_owned()
}

#[tokio::test]
async fn workflow_init_only_http_tools_config_reaches_200() {
    let yaml = r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: true
    corporate:
      default_base: 'http://127.0.0.1:9'
    allowlist:
      allow_hosts: ['127.0.0.1:9']
      allow_path_prefixes: ['/']
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: corp
      enabled: true
      http_tools:
        - path: /corp/service-a
          method: GET
          tool_name: from_a
"#;
    let cfg = Arc::new(PandaConfig::from_yaml_str(yaml).expect("yaml"));
    let egress = EgressClient::connect(&cfg.api_gateway.egress)
        .await
        .expect("egress")
        .expect("some");
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    state.egress = Some(egress);
    state.mcp = Some(
        mcp::McpRuntime::connect(cfg.as_ref(), state.egress.as_ref())
            .await
            .unwrap()
            .expect("mcp"),
    );
    let state = Arc::new(state);
    let (panda_addr, panda_task) = spawn_dispatch_stack(Arc::clone(&state), 1).await;
    let s0 = raw_mcp_post(panda_addr, "/mcp", "", mcp_init_body()).await;
    assert!(s0.contains("200 OK"), "init: {s0}");
    panda_task.await.ok();
}

#[tokio::test]
async fn workflow_full_stack_two_http_tools_two_mock_paths() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Same shape as `ingress_mcp_http_tools_call_uses_tool_cache_second_hit` (known-good Hyper + mock timing).
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();
    let hit = Arc::new(AtomicUsize::new(0));
    let hit_spawn = Arc::clone(&hit);
    tokio::spawn(async move {
        for _ in 0..2 {
            let Ok((mut sock, _)) = listener.accept().await else {
                break;
            };
            let mut buf = vec![0u8; 16_384];
            let Ok(n) = sock.read(&mut buf).await else {
                continue;
            };
            let _req = std::str::from_utf8(&buf[..n]).expect("utf8");
            let i = hit_spawn.fetch_add(1, Ordering::SeqCst);
            // Order matches two egress GETs (Connection: close → new TCP per call in this harness).
            let body_json = if i == 0 {
                r#"{"service":"A"}"#
            } else {
                r#"{"service":"B"}"#
            };
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: application/json\r\n\r\n{}",
                body_json.len(),
                body_json
            );
            let _ = sock.write_all(resp.as_bytes()).await;
        }
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: true
    timeout_ms: 5000
    pool_idle_timeout_ms: 0
    corporate:
      default_base: 'http://127.0.0.1:{port}'
    allowlist:
      allow_hosts: ['127.0.0.1:{port}']
      allow_path_prefixes: ['/corp']
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: corp
      enabled: true
      http_tools:
        - path: /corp/service-a
          method: GET
          tool_name: from_a
        - path: /corp/service-b
          method: GET
          tool_name: from_b
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).expect("yaml"));
    let egress = EgressClient::connect(&cfg.api_gateway.egress)
            .await
        .expect("egress try_new")
        .expect("egress some");
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    state.egress = Some(egress);
    state.mcp = Some(
        mcp::McpRuntime::connect(cfg.as_ref(), state.egress.as_ref())
            .await
            .unwrap()
            .expect("mcp"),
    );
    let state = Arc::new(state);

    let listener_panda = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr_panda = listener_panda.local_addr().unwrap();
    let st = Arc::clone(&state);
    let server = tokio::spawn(async move {
        for _ in 0..3 {
            let (stream, _) = listener_panda.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let st2 = Arc::clone(&st);
            let svc = service_fn(move |req| {
                let s = Arc::clone(&st2);
                crate::dispatch(req, s)
            });
            let _ = http1::Builder::new()
                .serve_connection(io, svc)
                .with_upgrades()
                .await;
        }
    });

    let body_init = mcp_init_body();
    let req_init = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        body_init.len(),
        body_init
    );
    let mut c0 = TcpStream::connect(addr_panda).await.unwrap();
    c0.write_all(req_init.as_bytes()).await.unwrap();
    let mut buf0 = vec![];
    c0.read_to_end(&mut buf0).await.unwrap();
    let s0 = String::from_utf8_lossy(&buf0);
    assert!(s0.contains("200 OK"), "{s0}");
    let sid = parse_mcp_session_id_from_raw_http(&s0).expect("session id");

    let body_a = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mcp_corp_from_a","arguments":{}}}"#;
    let req_a = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nMcp-Session-Id: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        sid,
        body_a.len(),
        body_a
    );
    let mut c1 = TcpStream::connect(addr_panda).await.unwrap();
    c1.write_all(req_a.as_bytes()).await.unwrap();
    let mut b1 = vec![];
    c1.read_to_end(&mut b1).await.unwrap();
    let sa = String::from_utf8_lossy(&b1);
    assert!(sa.contains("200 OK"), "{sa}");
    assert!(sa.contains("service") && sa.contains("A"), "{sa}");

    let body_b = r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"mcp_corp_from_b","arguments":{}}}"#;
    let req_b = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nMcp-Session-Id: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        sid,
        body_b.len(),
        body_b
    );
    let mut c2 = TcpStream::connect(addr_panda).await.unwrap();
    c2.write_all(req_b.as_bytes()).await.unwrap();
    let mut b2 = vec![];
    c2.read_to_end(&mut b2).await.unwrap();
    let sb = String::from_utf8_lossy(&b2);
    assert!(sb.contains("200 OK"), "{sb}");
    assert!(sb.contains("service") && sb.contains("B"), "{sb}");

    server.await.unwrap();
}

/// Ingress MCP `tools/call` → **remote MCP** (`remote_mcp_url`) → mock JSON-RPC server (via egress).
#[tokio::test]
async fn workflow_ingress_remote_mcp_tools_call_via_egress() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        for _ in 0..3 {
            mock_remote_mcp_upstream(&listener).await;
        }
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: true
    timeout_ms: 5000
    pool_idle_timeout_ms: 0
    corporate:
      default_base: 'http://127.0.0.1:{port}'
    allowlist:
      allow_hosts: ['127.0.0.1:{port}']
      allow_path_prefixes: ['/']
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: remote1
      enabled: true
      remote_mcp_url: 'http://127.0.0.1:{port}/mcp'
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).expect("yaml"));
    let egress = EgressClient::connect(&cfg.api_gateway.egress)
        .await
        .expect("egress")
        .expect("some");
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    state.egress = Some(egress);
    state.mcp = Some(
        mcp::McpRuntime::connect(cfg.as_ref(), state.egress.as_ref())
            .await
            .unwrap()
            .expect("mcp"),
    );
    let state = Arc::new(state);

    let listener_panda = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr_panda = listener_panda.local_addr().unwrap();
    let st = Arc::clone(&state);
    let server = tokio::spawn(async move {
        for _ in 0..2 {
            let (stream, _) = listener_panda.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let st2 = Arc::clone(&st);
            let svc = service_fn(move |req| {
                let s = Arc::clone(&st2);
                crate::dispatch(req, s)
            });
            let _ = http1::Builder::new()
                .serve_connection(io, svc)
                .with_upgrades()
                .await;
        }
    });

    let body_init = mcp_init_body();
    let req_init = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        body_init.len(),
        body_init
    );
    let mut c0 = TcpStream::connect(addr_panda).await.unwrap();
    c0.write_all(req_init.as_bytes()).await.unwrap();
    let mut buf0 = vec![];
    c0.read_to_end(&mut buf0).await.unwrap();
    let s0 = String::from_utf8_lossy(&buf0);
    assert!(s0.contains("200 OK"), "{s0}");
    let sid = parse_mcp_session_id_from_raw_http(&s0).expect("session id");

    let body_call = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mcp_remote1_alpha","arguments":{}}}"#;
    let req_call = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nMcp-Session-Id: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        sid,
        body_call.len(),
        body_call
    );
    let mut c1 = TcpStream::connect(addr_panda).await.unwrap();
    c1.write_all(req_call.as_bytes()).await.unwrap();
    let mut b1 = vec![];
    c1.read_to_end(&mut b1).await.unwrap();
    let s1 = String::from_utf8_lossy(&b1);
    assert!(s1.contains("200 OK"), "{s1}");
    assert!(
        s1.contains("remote_ok"),
        "expected remote MCP tool result in response: {s1}"
    );

    server.await.unwrap();
}

/// **Stdio** MCP (`tests/mcp_mock_stdio.py` `ping`) + **`http_tool`** egress on the same ingress `/mcp` surface.
#[tokio::test]
async fn workflow_stdio_python_and_http_tool_ingress() {
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/mcp_mock_stdio.py");
    if !script.is_file() {
        return;
    }
    let Some(py) = detect_python_cmd() else {
        return;
    };
    let script_s = script.to_str().expect("utf8 path").replace('\\', "/");

    let rest = TcpListener::bind("127.0.0.1:0").await.expect("rest bind");
    let rest_port = rest.local_addr().unwrap().port();
    tokio::spawn(async move {
        let Ok((mut sock, _)) = rest.accept().await else {
            return;
        };
        let mut buf = vec![0u8; 16_384];
        let Ok(n) = sock.read(&mut buf).await else {
            return;
        };
        let req = std::str::from_utf8(&buf[..n]).expect("utf8");
        assert!(
            req.contains("GET /api/hi "),
            "expected GET /api/hi, got {}",
            req.chars().take(160).collect::<String>()
        );
        let body = r#"{"via":"rest"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: application/json\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = sock.write_all(resp.as_bytes()).await;
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: true
    timeout_ms: 15000
    pool_idle_timeout_ms: 0
    corporate:
      default_base: 'http://127.0.0.1:{rest_port}'
    allowlist:
      allow_hosts: ['127.0.0.1:{rest_port}']
      allow_path_prefixes: ['/api']
mcp:
  enabled: true
  advertise_tools: true
  fail_open: true
  tool_timeout_ms: 15000
  servers:
    - name: mock
      enabled: true
      command: "{py}"
      args:
        - "{script_s}"
    - name: corp
      enabled: true
      http_tool:
        path: /api/hi
        method: GET
        tool_name: hi
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).expect("yaml"));
    let egress = EgressClient::connect(&cfg.api_gateway.egress)
        .await
        .expect("egress")
        .expect("some");
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    state.egress = Some(egress);
    state.mcp = Some(
        mcp::McpRuntime::connect(cfg.as_ref(), state.egress.as_ref())
            .await
            .unwrap()
            .expect("mcp"),
    );
    let state = Arc::new(state);

    let listener_panda = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr_panda = listener_panda.local_addr().unwrap();
    let st = Arc::clone(&state);
    let server = tokio::spawn(async move {
        for _ in 0..3 {
            let (stream, _) = listener_panda.accept().await.unwrap();
            let io = TokioIo::new(stream);
            let st2 = Arc::clone(&st);
            let svc = service_fn(move |req| {
                let s = Arc::clone(&st2);
                crate::dispatch(req, s)
            });
            let _ = http1::Builder::new()
                .serve_connection(io, svc)
                .with_upgrades()
                .await;
        }
    });

    let body_init = mcp_init_body();
    let req_init = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        body_init.len(),
        body_init
    );
    let mut c0 = TcpStream::connect(addr_panda).await.unwrap();
    c0.write_all(req_init.as_bytes()).await.unwrap();
    let mut buf0 = vec![];
    c0.read_to_end(&mut buf0).await.unwrap();
    let s0 = String::from_utf8_lossy(&buf0);
    assert!(s0.contains("200 OK"), "{s0}");
    let sid = parse_mcp_session_id_from_raw_http(&s0).expect("session id");

    let body_ping = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mcp_mock_ping","arguments":{}}}"#;
    let req_ping = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nMcp-Session-Id: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        sid,
        body_ping.len(),
        body_ping
    );
    let mut c1 = TcpStream::connect(addr_panda).await.unwrap();
    c1.write_all(req_ping.as_bytes()).await.unwrap();
    let mut b1 = vec![];
    c1.read_to_end(&mut b1).await.unwrap();
    let s1 = String::from_utf8_lossy(&b1);
    assert!(s1.contains("200 OK"), "{s1}");
    assert!(
        s1.contains("pong") || s1.contains("ping"),
        "expected stdio ping tool text: {s1}"
    );

    let body_hi = r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"mcp_corp_hi","arguments":{}}}"#;
    let req_hi = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {}\r\nMcp-Session-Id: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        mcp_streamable_accept_value(),
        sid,
        body_hi.len(),
        body_hi
    );
    let mut c2 = TcpStream::connect(addr_panda).await.unwrap();
    c2.write_all(req_hi.as_bytes()).await.unwrap();
    let mut b2 = vec![];
    c2.read_to_end(&mut b2).await.unwrap();
    let s2 = String::from_utf8_lossy(&b2);
    assert!(s2.contains("200 OK"), "{s2}");
    assert!(
        s2.contains("rest") || s2.contains("via"),
        "expected http_tool JSON in response: {s2}"
    );

    server.await.unwrap();
}

#[tokio::test]
async fn workflow_ingress_off_post_mcp_not_handled_by_mcp_ingress() {
    let yaml = r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: false
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: x
      enabled: true
"#;
    let cfg = Arc::new(PandaConfig::from_yaml_str(yaml).unwrap());
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.mcp = mcp::McpRuntime::connect(cfg.as_ref(), None).await.unwrap();
    let state = Arc::new(state);

    let (panda_addr, panda_task) = spawn_dispatch_stack(Arc::clone(&state), 1).await;
    let s = raw_mcp_post(panda_addr, "/mcp", "", mcp_init_body()).await;
    assert!(
        s.contains("502") || s.contains("Bad Gateway"),
        "without ingress, POST /mcp is proxied upstream, not MCP ingress; expected upstream failure: {s}"
    );
    panda_task.await.ok();
}

#[tokio::test]
async fn workflow_mcp_runtime_off_ingress_mcp_returns_unavailable() {
    let yaml = r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: stub
      enabled: true
"#;
    let cfg = Arc::new(PandaConfig::from_yaml_str(yaml).unwrap());
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    state.mcp = None;
    let state = Arc::new(state);

    let (panda_addr, panda_task) = spawn_dispatch_stack(Arc::clone(&state), 1).await;
    let s = raw_mcp_post(panda_addr, "/mcp", "", mcp_init_body()).await;
    assert!(
        s.contains("503") || s.contains("SERVICE_UNAVAILABLE"),
        "expected 503: {s}"
    );
    assert!(
        s.contains("MCP runtime not available") || s.contains("32000"),
        "jsonrpc hint: {s}"
    );
    panda_task.await.ok();
}

#[test]
fn workflow_http_tool_requires_egress_enabled() {
    let yaml = r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
  egress:
    enabled: false
mcp:
  enabled: true
  servers:
    - name: z
      enabled: true
      http_tool:
        path: /x
        method: GET
        tool_name: t
"#;
    let err = PandaConfig::from_yaml_str(yaml).unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("http_tool requires api_gateway.egress.enabled"),
        "{msg}"
    );
}
