//! **Control plane + ingress + streamable MCP** scenario tests (see `docs/testing_mcp_api_gateway.md`,
//! `docs/panda_scenarios_summary.md`).
//!
//! **`repo_root_panda_yaml_dispatch_health_smoke`** loads the repository root [`panda.yaml`](../../../../panda.yaml)
//! (same file as `crates/panda-config/tests/scenario_profiles.rs::scenario_d_repo_root_panda_yaml_parses`) and
//! verifies **`IngressRouter::try_new` + `dispatch`** accept **`GET /health`** — end-to-end proof the checked-in
//! profile stays compatible with the proxy stack.
//!
//! # Scenario matrix (this file)
//!
//! | ID | What we verify | Pass criteria |
//! |----|------------------|---------------|
//! | **CP-RO-1** | Read-only secret (`read_only_secret_envs`) | GET `/v1/status`, `/v1/runtime/summary`, `/v1/mcp/config`, `/v1/api_gateway/ingress/routes`, `/v1/api_gateway/ingress/routes/export` → **200** |
//! | **CP-RO-2** | Same creds cannot mutate | POST routes, POST import, DELETE routes → **403** |
//! | **CP-RW-1** | Full admin secret still mutates | POST dynamic route with **write** secret → **200** |
//! | **TN-1** | `tenant_id` on dynamic row | Without `tenant_resolution_header` value → row ignored (**404** vs scoped path) |
//! | **TN-2** | Global dynamic row (no `tenant_id`) | No header → **410** `gone` when row matches |
//! | **TN-3** | Matching header | `x-tenant-id: acme` → tenant row **410** |
//! | **TN-4** | Wrong tenant | Other tenant → **404** |
//! | **SSE-1** | `Last-Event-ID` on GET listener | After `initialize` (id 1) + `ping` (id 2), GET with `Last-Event-ID: 1` replays **only** id **2** before `: mcp-listener` |

use std::sync::Arc;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use panda_config::PandaConfig;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

use super::{mcp_streamable_accept_value, parse_mcp_session_id_from_raw_http, test_proxy_state};
use crate::api_gateway::ingress::IngressRouter;
use crate::dispatch;
use crate::inbound::mcp;
use crate::ProxyState;

/// Accepts `accepts` TCP connections and serves each in a **nested** task so a long-lived GET (SSE)
/// does not block the next `accept` (same pattern as `ingress_mcp_streamable_get_listener_and_delete_session`).
async fn spawn_dispatch_stack(
    state: Arc<ProxyState>,
    accepts: usize,
) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().unwrap();
    let st = Arc::clone(&state);
    let server = tokio::spawn(async move {
        for _ in 0..accepts {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let io = TokioIo::new(stream);
            let st2 = Arc::clone(&st);
            tokio::spawn(async move {
                let svc = service_fn(move |req| {
                    let s = Arc::clone(&st2);
                    dispatch(req, s)
                });
                let _ = http1::Builder::new()
                    .serve_connection(io, svc)
                    .with_upgrades()
                    .await;
            });
        }
    });
    (addr, server)
}

async fn tcp_exchange(addr: std::net::SocketAddr, req: &str) -> String {
    let mut c = TcpStream::connect(addr).await.expect("connect");
    c.write_all(req.as_bytes()).await.unwrap();
    let mut buf = vec![];
    c.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).into_owned()
}

/// **CP-RO-1 / CP-RO-2 / CP-RW-1**: read-only vs write control plane credentials.
#[tokio::test]
async fn control_plane_read_only_matrix_and_write_still_mutates() {
    const WRITE_ENV: &str = "PANDA_TEST_CP_RW_MATRIX";
    const RO_ENV: &str = "PANDA_TEST_CP_RO_MATRIX";
    std::env::set_var(WRITE_ENV, "write-secret");
    std::env::set_var(RO_ENV, "read-secret");

    let raw = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
observability:
  correlation_header: x-request-id
  admin_secret_env: {WRITE_ENV}
control_plane:
  enabled: true
  read_only_secret_envs:
    - {RO_ENV}
api_gateway:
  ingress:
    enabled: true
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&raw).unwrap());
    let ingress = IngressRouter::try_new(&cfg.api_gateway.ingress).expect("ingress router");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    let state = Arc::new(state);

    // 9 connections: 5 GETs (read-only OK), 3 mutating attempts (403), 1 write POST (200).
    let (addr, server) = spawn_dispatch_stack(Arc::clone(&state), 9).await;

    let ro = "read-secret";
    let rw = "write-secret";

    let r_status = tcp_exchange(
        addr,
        &format!(
            "GET /ops/control/v1/status HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-panda-admin-secret: {ro}\r\n\r\n"
        ),
    )
    .await;
    assert!(
        r_status.contains("200 OK") && r_status.contains("control_plane"),
        "CP-RO-1 status: {r_status}"
    );

    let r_routes = tcp_exchange(
        addr,
        &format!(
            "GET /ops/control/v1/api_gateway/ingress/routes HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-panda-admin-secret: {ro}\r\n\r\n"
        ),
    )
    .await;
    assert!(
        r_routes.contains("200 OK") && r_routes.contains("ingress_enabled"),
        "CP-RO-1 routes: {r_routes}"
    );

    let r_export = tcp_exchange(
        addr,
        &format!(
            "GET /ops/control/v1/api_gateway/ingress/routes/export HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-panda-admin-secret: {ro}\r\n\r\n"
        ),
    )
    .await;
    assert!(
        r_export.contains("200 OK") && r_export.contains("routes"),
        "CP-RO-1 export: {r_export}"
    );

    let r_runtime = tcp_exchange(
        addr,
        &format!(
            "GET /ops/control/v1/runtime/summary HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-panda-admin-secret: {ro}\r\n\r\n"
        ),
    )
    .await;
    assert!(
        r_runtime.contains("200 OK")
            && r_runtime.contains("panda_control_plane_runtime_summary")
            && r_runtime.contains("\"identity\"")
            && r_runtime.contains("\"tpm\""),
        "CP-RO-1 runtime summary: {r_runtime}"
    );

    let r_mcp_cfg = tcp_exchange(
        addr,
        &format!(
            "GET /ops/control/v1/mcp/config HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-panda-admin-secret: {ro}\r\n\r\n"
        ),
    )
    .await;
    assert!(
        r_mcp_cfg.contains("200 OK") && r_mcp_cfg.contains("panda_control_plane_mcp_config"),
        "CP-RO-1 mcp config: {r_mcp_cfg}"
    );

    let post_body = br#"{"path_prefix":"/z-ro-deny","backend":"gone"}"#;
    let post_mut = format!(
        "POST /ops/control/v1/api_gateway/ingress/routes HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\nx-panda-admin-secret: {ro}\r\n\r\n{}",
        post_body.len(),
        String::from_utf8_lossy(post_body)
    );
    let r_post_ro = tcp_exchange(addr, &post_mut).await;
    assert!(
        r_post_ro.contains("403"),
        "CP-RO-2 POST routes read-only: {r_post_ro}"
    );

    let import_body = br#"{"version":1,"routes":[]}"#;
    let post_import = format!(
        "POST /ops/control/v1/api_gateway/ingress/routes/import?mode=merge HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\nx-panda-admin-secret: {ro}\r\n\r\n{}",
        import_body.len(),
        String::from_utf8_lossy(import_body)
    );
    let r_import_ro = tcp_exchange(addr, &post_import).await;
    assert!(
        r_import_ro.contains("403"),
        "CP-RO-2 POST import read-only: {r_import_ro}"
    );

    let del = format!(
        "DELETE /ops/control/v1/api_gateway/ingress/routes?path_prefix=%2Fz-ro-deny HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-panda-admin-secret: {ro}\r\n\r\n"
    );
    let r_del_ro = tcp_exchange(addr, &del).await;
    assert!(
        r_del_ro.contains("403"),
        "CP-RO-2 DELETE routes read-only: {r_del_ro}"
    );

    let ok_body = br#"{"path_prefix":"/z-rw-ok","backend":"gone"}"#;
    let post_rw = format!(
        "POST /ops/control/v1/api_gateway/ingress/routes HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\nx-panda-admin-secret: {rw}\r\n\r\n{}",
        ok_body.len(),
        String::from_utf8_lossy(ok_body)
    );
    let r_rw = tcp_exchange(addr, &post_rw).await;
    assert!(
        r_rw.contains("200 OK") && r_rw.contains("\"ok\":true"),
        "CP-RW-1 write secret mutates: {r_rw}"
    );

    server.await.ok();
    std::env::remove_var(WRITE_ENV);
    std::env::remove_var(RO_ENV);
}

/// **TN-1 … TN-4**: global vs tenant-scoped dynamic ingress rows with `tenant_resolution_header`.
#[tokio::test]
async fn ingress_tenant_global_row_vs_scoped_row() {
    const SECRET_ENV: &str = "PANDA_TEST_TN_MATRIX";
    std::env::set_var(SECRET_ENV, "tenant-cp-secret");

    let raw = format!(
        r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
observability:
  correlation_header: x-request-id
  admin_secret_env: {SECRET_ENV}
api_gateway:
  ingress:
    enabled: true
control_plane:
  enabled: true
  tenant_resolution_header: x-tenant-id
"#
    );
    let cfg = Arc::new(PandaConfig::from_yaml_str(&raw).unwrap());
    let ingress = IngressRouter::try_new(&cfg.api_gateway.ingress).expect("ingress router");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    let state = Arc::new(state);

    // 4 connections: 2 POSTs (scoped + global), 4 GETs
    let (addr, server) = spawn_dispatch_stack(Arc::clone(&state), 6).await;
    let sec = "tenant-cp-secret";

    let body_tenant = br#"{"tenant_id":"acme","path_prefix":"/z-tn-scoped","backend":"gone"}"#;
    let post1 = format!(
        "POST /ops/control/v1/api_gateway/ingress/routes HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\nx-panda-admin-secret: {sec}\r\n\r\n{}",
        body_tenant.len(),
        String::from_utf8_lossy(body_tenant)
    );
    let r1 = tcp_exchange(addr, &post1).await;
    assert!(r1.contains("200 OK"), "register tenant row: {r1}");

    let body_global = br#"{"path_prefix":"/z-tn-global","backend":"gone"}"#;
    let post2 = format!(
        "POST /ops/control/v1/api_gateway/ingress/routes HTTP/1.1\r\nHost: z\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\nx-panda-admin-secret: {sec}\r\n\r\n{}",
        body_global.len(),
        String::from_utf8_lossy(body_global)
    );
    let r2 = tcp_exchange(addr, &post2).await;
    assert!(r2.contains("200 OK"), "register global row: {r2}");

    // TN-2: global row matches without tenant header
    let g_global = tcp_exchange(
        addr,
        "GET /z-tn-global/x HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(
        g_global.contains("410"),
        "TN-2 global row without header: {g_global}"
    );

    // TN-1: scoped row invisible without header
    let g_scoped_noh = tcp_exchange(
        addr,
        "GET /z-tn-scoped/x HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(
        g_scoped_noh.contains("404"),
        "TN-1 scoped without tenant header: {g_scoped_noh}"
    );

    // TN-3: scoped row with matching tenant
    let g_acme = tcp_exchange(
        addr,
        "GET /z-tn-scoped/x HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-tenant-id: acme\r\n\r\n",
    )
    .await;
    assert!(
        g_acme.contains("410"),
        "TN-3 scoped with acme: {g_acme}"
    );

    // TN-4: wrong tenant
    let g_other = tcp_exchange(
        addr,
        "GET /z-tn-scoped/x HTTP/1.1\r\nHost: z\r\nConnection: close\r\nx-tenant-id: other\r\n\r\n",
    )
    .await;
    assert!(
        g_other.contains("404"),
        "TN-4 wrong tenant: {g_other}"
    );

    server.await.ok();
    std::env::remove_var(SECRET_ENV);
}

/// **SSE-1**: GET listener replays buffered POST SSE events after `Last-Event-ID`.
#[tokio::test]
async fn ingress_mcp_streamable_last_event_id_replays_only_newer_events() {
    let cfg = Arc::new(
        PandaConfig::from_yaml_str(
            r#"listen: '127.0.0.1:0'
default_backend: 'http://127.0.0.1:1'
api_gateway:
  ingress:
    enabled: true
mcp:
  enabled: true
  advertise_tools: true
  servers:
    - name: stubsrv
      enabled: true
"#,
        )
        .unwrap(),
    );
    let ingress = IngressRouter::try_new(&cfg.api_gateway.ingress).expect("ingress router");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    state.mcp = mcp::McpRuntime::connect(cfg.as_ref(), None).await.unwrap();
    let state = Arc::new(state);

    // init, ping, GET with Last-Event-ID (long read)
    let (addr, server) = spawn_dispatch_stack(Arc::clone(&state), 3).await;
    let acc = mcp_streamable_accept_value();

    let body_init = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#;
    let req_init = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {acc}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body_init}",
        body_init.len()
    );
    let s1 = tcp_exchange(addr, &req_init).await;
    assert!(s1.contains("200 OK"), "init: {s1}");
    let sid = parse_mcp_session_id_from_raw_http(&s1).expect("session id");

    let body_ping = r#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#;
    let req_ping = format!(
        "POST /mcp HTTP/1.1\r\nHost: z\r\nAccept: {acc}\r\nMcp-Session-Id: {sid}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body_ping}",
        body_ping.len()
    );
    let s2 = tcp_exchange(addr, &req_ping).await;
    assert!(s2.contains("200 OK"), "ping: {s2}");

    let req_get = format!(
        "GET /mcp HTTP/1.1\r\nHost: z\r\nAccept: text/event-stream\r\nMcp-Session-Id: {sid}\r\nLast-Event-ID: 1\r\nConnection: close\r\n\r\n"
    );
    let mut c = TcpStream::connect(addr).await.unwrap();
    c.write_all(req_get.as_bytes()).await.unwrap();
    let mut gbuf = vec![0u8; 8192];
    let gn = timeout(Duration::from_secs(2), c.read(&mut gbuf))
        .await
        .expect("GET read timed out")
        .expect("read");
    let gtxt = String::from_utf8_lossy(&gbuf[..gn]);

    // Replay: only event id 2 (ping JSON-RPC), not id 1 (initialize)
    assert!(
        gtxt.contains("id: 2") && gtxt.contains("\"id\":2"),
        "SSE-1 expected replay of id 2: {gtxt}"
    );
    assert!(
        !gtxt.lines().any(|l| l.trim() == "id: 1"),
        "SSE-1 must not replay SSE id 1 after Last-Event-ID: 1: {gtxt}"
    );
    assert!(
        gtxt.contains("mcp-listener"),
        "SSE-1 listener preamble after replay: {gtxt}"
    );

    server.await.ok();
}

/// Repository root [`panda.yaml`](../../../../panda.yaml) must parse and work with built-in ingress (`GET /health`).
#[tokio::test]
async fn repo_root_panda_yaml_dispatch_health_smoke() {
    let yaml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../panda.yaml"));
    let cfg = Arc::new(PandaConfig::from_yaml_str(yaml).expect("repo panda.yaml must parse"));
    let ingress = IngressRouter::try_new(&cfg.api_gateway.ingress).expect("ingress");
    let mut state = test_proxy_state(Arc::clone(&cfg)).await;
    state.ingress_router = Some(ingress);
    let state = Arc::new(state);

    let (addr, server) = spawn_dispatch_stack(Arc::clone(&state), 1).await;
    let body = tcp_exchange(
        addr,
        "GET /health HTTP/1.1\r\nHost: z\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(
        body.contains("200 OK") && body.contains("ok"),
        "repo panda.yaml health: {body}"
    );
    server.await.ok();
}
