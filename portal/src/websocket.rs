//! WebSocket proxy for Nango Connect auth flow.
//!
//! Browsers can't reach Nango's WebSocket server directly when only the portal is publicly tunneled
//! (cloudflared → :9000). This handler bridges the browser to Nango's WebSocket server at
//! `ws://127.0.0.1:3003/` by proxying messages in both directions.

use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message as AxumMsg, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message as TungMsg};

use crate::app::AppState;

fn axum_to_tung(msg: AxumMsg) -> Option<TungMsg> {
    use tokio_tungstenite::tungstenite::Message;
    match msg {
        AxumMsg::Text(s) => Some(Message::Text(s.into())),
        AxumMsg::Binary(b) => Some(Message::Binary(Bytes::copy_from_slice(&b).into())),
        AxumMsg::Ping(d) => Some(Message::Ping(Bytes::copy_from_slice(&d).into())),
        AxumMsg::Pong(d) => Some(Message::Pong(Bytes::copy_from_slice(&d).into())),
        // CloseCode is a private enum in tungstenite — send a normal closure instead.
        AxumMsg::Close(_) => Some(Message::Close(None)),
    }
}

fn tung_to_axum(msg: TungMsg) -> Option<AxumMsg> {
    match msg {
        TungMsg::Text(s) => Some(AxumMsg::Text(s.as_str().to_string())),
        TungMsg::Binary(d) => Some(AxumMsg::Binary(d.as_slice().to_vec())),
        TungMsg::Ping(d) => Some(AxumMsg::Ping(d.as_slice().to_vec())),
        TungMsg::Pong(d) => Some(AxumMsg::Pong(d.as_slice().to_vec())),
        // Forward close frames as-is — Axum accepts them as-is and browser will close.
        TungMsg::Close(frame) => Some(AxumMsg::Close(frame.map(|f| {
            axum::extract::ws::CloseFrame {
                code: f.code.into(),
                reason: f.reason,
            }
        }))),
        _ => None,
    }
}

fn nango_ws_url() -> Option<String> {
    let base = std::env::var("NANGO_BASE_URL")
        .ok()?
        .trim()
        .trim_end_matches('/')
        .to_string();
    if base.is_empty() {
        return None;
    }
    let (scheme, host_and_path) = if let Some(r) = base.strip_prefix("https://") {
        ("wss", r)
    } else if let Some(r) = base.strip_prefix("http://") {
        ("ws", r)
    } else {
        return None;
    };
    let configured_path = std::env::var("NANGO_SERVER_WEBSOCKETS_PATH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "/".to_string());
    let path = if configured_path.starts_with('/') {
        configured_path
    } else {
        format!("/{configured_path}")
    };
    Some(format!(
        "{scheme}://{}{}",
        host_and_path.trim_end_matches('/'),
        path
    ))
}

/// GET /_nango_auth_ws — upgrades to WebSocket and proxies to Nango's auth WebSocket.
async fn ws_handler(
    State(_state): State<Arc<AppState>>,
    ws_upgrade: WebSocketUpgrade,
) -> impl IntoResponse {
    ws_upgrade.on_upgrade(move |browser_ws: WebSocket| async move {
        let Some(nango_ws_url) = nango_ws_url() else {
            tracing::warn!("nango_ws_proxy: NANGO_BASE_URL is not a valid HTTP(S) URL");
            return;
        };
        let (upstream_ws, _) = match connect_async(&nango_ws_url).await {
            Ok(ws) => ws,
            Err(e) => {
                tracing::warn!("nango_ws_proxy: upstream connect failed: {}", e);
                return;
            }
        };

        let (mut browser_out, mut browser_in) = browser_ws.split();
        let (mut upstream_out, mut upstream_in) = upstream_ws.split();

        loop {
            tokio::select! {
                msg = browser_in.next() => {
                    match msg {
                        Some(Ok(axum_msg)) => {
                            if let Some(tung_msg) = axum_to_tung(axum_msg) {
                                if upstream_out.send(tung_msg).await.is_err() { break; }
                            }
                        }
                        None => {
                            let _ = upstream_out.send(TungMsg::Close(None)).await;
                            break;
                        }
                        Some(Err(e)) => {
                            tracing::debug!("browser ws: {}", e);
                            break;
                        }
                    }
                }
                msg = upstream_in.next() => {
                    match msg {
                        Some(Ok(tung_msg)) => {
                            if let Some(axum_msg) = tung_to_axum(tung_msg) {
                                if browser_out.send(axum_msg).await.is_err() { break; }
                            }
                        }
                        None => {
                            let _ = browser_out.send(AxumMsg::Close(None)).await;
                            break;
                        }
                        Some(Err(e)) => {
                            tracing::debug!("upstream ws: {}", e);
                            break;
                        }
                    }
                }
            }
        }
    })
}

pub fn websocket_routes() -> axum::Router<Arc<AppState>> {
    axum::Router::new().route("/_nango_auth_ws", axum::routing::get(ws_handler))
}
