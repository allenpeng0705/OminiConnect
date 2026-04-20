//! Pairwise coverage for major **`dispatch`** forks (see `docs/panda_data_flow.md`).
//!
//! | Branch | “No” / skipped | “Yes” / active |
//! |--------|----------------|----------------|
//! | `control_plane.enabled` | Request does not match `control_plane_rest_path` → later stages run | Path under `path_prefix` handled in control-plane block (auth + JSON) |
//! | `api_gateway.ingress.enabled` | No `ingress_router` → ingress classification skipped | `classify_merged` runs; unmatched path → 404 |
//!
//! Related tests in `lib.rs`: `control_plane_status_requires_ops_secret_when_configured` (secret → 200),
//! `ingress_enabled_tcp_unknown_path_404_health_200`, `backend_routing_and_proxy::*`.
//! **Read-only CP + tenant ingress + SSE replay:** [`control_plane_and_streamable_scenarios`](./control_plane_and_streamable_scenarios.rs).

use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use panda_config::PandaConfig;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use super::test_proxy_state;
use crate::dispatch;

async fn spawn_dispatch_one_accept(state: Arc<crate::ProxyState>) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().unwrap();
    let st = Arc::clone(&state);
    let server = tokio::spawn(async move {
        let Ok((stream, _)) = listener.accept().await else {
            return;
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
    });
    (addr, server)
}

async fn raw_get(addr: std::net::SocketAddr, path: &str) -> String {
    let mut c = TcpStream::connect(addr).await.expect("connect");
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n"
    );
    c.write_all(req.as_bytes()).await.unwrap();
    let mut buf = vec![];
    c.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).into_owned()
}

/// **`control_plane.enabled: false`**: `/ops/control/...` is not a control-plane route — `control_plane_rest_path` is never matched.
/// With **`ingress.enabled: true`**, the path must match an ingress row; built-ins do not cover `/ops/control`, so we get **404** from ingress (not JSON from control plane).
#[tokio::test]
async fn when_control_plane_disabled_ops_prefix_is_not_control_plane_json() {
    let cfg = Arc::new(
        PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
control_plane:
  enabled: false
api_gateway:
  ingress:
    enabled: true
"#,
        )
        .unwrap(),
    );
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress router");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    let state = Arc::new(state);

    let (addr, server) = spawn_dispatch_one_accept(state).await;
    let resp = raw_get(addr, "/ops/control/v1/status").await;
    server.await.ok();

    assert!(
        resp.contains("404"),
        "expected ingress no-match 404, got: {resp}"
    );
    assert!(
        resp.contains("ingress: no matching route"),
        "expected ingress body, not control plane JSON: {resp}"
    );
    assert!(
        !resp.contains("\"phase\""),
        "must not return control plane status JSON: {resp}"
    );
}

/// **`control_plane.enabled: false`** and **`ingress.enabled: false`**: neither control plane nor ingress runs; request reaches **`forward_to_upstream`**.
/// The mock upstream must see the full path (proves we did not short-circuit on ingress 404).
#[tokio::test]
async fn when_control_plane_and_ingress_disabled_request_hits_default_backend() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let up_addr = upstream.local_addr().unwrap();

    let mock = tokio::spawn(async move {
        let (mut sock, _) = upstream.accept().await.expect("mock accept");
        let mut buf = vec![0u8; 4096];
        let n = sock.read(&mut buf).await.expect("read");
        let req = std::str::from_utf8(&buf[..n]).expect("utf8");
        assert!(
            req.starts_with("GET /ops/control/v1/status "),
            "upstream should see full path: {req}"
        );
        let _ = sock
            .write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 2\r\n\r\nok")
            .await;
    });

    let yaml = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://{up_addr}'
control_plane:
  enabled: false
tpm:
  enforce_budget: false
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&yaml).unwrap());
    let state = Arc::new(test_proxy_state(Arc::clone(&cfg)).await);
    let (addr, server) = spawn_dispatch_one_accept(state).await;

    let resp = raw_get(addr, "/ops/control/v1/status").await;
    server.await.ok();
    mock.await.ok();

    assert!(resp.contains("200 OK"), "expected proxy 200: {resp}");
    assert!(resp.contains("ok"), "{resp}");
}

/// **`control_plane.enabled: true`**: same path is handled before ingress; without auth → **401** (see `control_plane_status_requires_ops_secret_when_configured` for 200 with secret).
#[tokio::test]
async fn when_control_plane_enabled_status_without_secret_is_401() {
    const SECRET_ENV: &str = "PANDA_TEST_DISPATCH_BRANCH_CP401";
    std::env::set_var(SECRET_ENV, "branch-secret");
    let raw = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
observability:
  correlation_header: x-request-id
  admin_secret_env: {SECRET_ENV}
control_plane:
  enabled: true
api_gateway:
  ingress:
    enabled: true
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&raw).unwrap());
    let ingress = crate::api_gateway::ingress::IngressRouter::try_new(&cfg.api_gateway.ingress)
        .expect("ingress router");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    let state = Arc::new(state);

    let (addr, server) = spawn_dispatch_one_accept(state).await;
    let resp = raw_get(addr, "/ops/control/v1/status").await;
    server.await.ok();
    std::env::remove_var(SECRET_ENV);

    assert!(
        resp.contains("401"),
        "expected 401 without admin secret (control plane branch taken): {resp}"
    );
}
