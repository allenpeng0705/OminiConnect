//! MCP **Streamable HTTP** transport helpers ([spec 2025-03-26](https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http)):
//! in-memory session IDs, Origin checks, GET listener SSE (keepalive + optional replay), and per-session
//! SSE event buffers for `Last-Event-ID` reconnect.

use std::collections::{HashMap, VecDeque};
use std::convert::Infallible;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use bytes::Bytes;
use futures_util::stream::{self, StreamExt};
use http::header::{self, HeaderMap, HeaderValue};
use http_body_util::{BodyExt, StreamBody};
use hyper::body::Frame;
use hyper::{Response, StatusCode};
use tokio::time::MissedTickBehavior;
use tokio_stream::wrappers::IntervalStream;

use panda_config::McpStreamableHttpConfig;

use crate::{text_response, BoxBody};

/// HTTP header carrying the MCP session id (case-insensitive; canonical form per spec examples).
pub const MCP_SESSION_ID_HEADER: &str = "mcp-session-id";

#[derive(Debug)]
struct SseEvent {
    id: u64,
    payload: Bytes,
}

#[derive(Debug)]
struct SessionRecord {
    touched: Instant,
    next_event_id: u64,
    ring: VecDeque<SseEvent>,
}

impl SessionRecord {
    fn touch(&mut self) {
        self.touched = Instant::now();
    }
}

#[derive(Debug)]
pub struct McpStreamableSessionStore {
    inner: Mutex<HashMap<String, SessionRecord>>,
    ttl: Duration,
    ring_cap: usize,
    keepalive_interval: Duration,
}

impl Default for McpStreamableSessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl McpStreamableSessionStore {
    pub fn new() -> Self {
        Self::from_config(&McpStreamableHttpConfig::default())
    }

    pub fn from_config(cfg: &McpStreamableHttpConfig) -> Self {
        Self::with_options(
            cfg.sse_ring_max_events,
            Duration::from_secs(cfg.session_ttl_seconds),
            Duration::from_secs(cfg.sse_keepalive_interval_seconds),
        )
    }

    pub(crate) fn with_options(ring_cap: usize, ttl: Duration, keepalive: Duration) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            ttl,
            ring_cap,
            keepalive_interval: keepalive,
        }
    }

    fn prune(&self, map: &mut HashMap<String, SessionRecord>) {
        let now = Instant::now();
        map.retain(|_, rec| now.duration_since(rec.touched) < self.ttl);
    }

    pub fn create_session(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let mut g = self.inner.lock().expect("mcp session mutex");
        self.prune(&mut g);
        g.insert(
            id.clone(),
            SessionRecord {
                touched: Instant::now(),
                next_event_id: 1,
                ring: VecDeque::new(),
            },
        );
        id
    }

    pub fn remove_session(&self, id: &str) -> bool {
        let mut g = self.inner.lock().expect("mcp session mutex");
        self.prune(&mut g);
        g.remove(id).is_some()
    }

    /// Returns false if unknown or expired session.
    pub fn validate_and_touch(&self, id: &str) -> bool {
        let mut g = self.inner.lock().expect("mcp session mutex");
        self.prune(&mut g);
        if let Some(rec) = g.get_mut(id) {
            rec.touch();
            true
        } else {
            false
        }
    }

    /// Append one SSE `message` event (JSON-RPC envelope as `data:`) for GET listener replay.
    /// Returns the exact bytes to send on the POST response (same as buffered for GET replay).
    pub fn append_sse_message_event(&self, session_id: &str, json_envelope: &str) -> Option<Bytes> {
        let mut g = self.inner.lock().expect("mcp session mutex");
        self.prune(&mut g);
        let rec = g.get_mut(session_id)?;
        rec.touch();
        let id = rec.next_event_id;
        rec.next_event_id = rec.next_event_id.saturating_add(1);
        let chunk = format!("id: {id}\nevent: message\ndata: {json_envelope}\n\n");
        let payload = Bytes::copy_from_slice(chunk.as_bytes());
        if rec.ring.len() >= self.ring_cap {
            rec.ring.pop_front();
        }
        rec.ring.push_back(SseEvent {
            id,
            payload: payload.clone(),
        });
        Some(payload)
    }

    /// Events with id strictly greater than `after_id` (for `Last-Event-ID` reconnect).
    pub fn replay_events_after(&self, session_id: &str, after_id: u64) -> Vec<Bytes> {
        let g = self.inner.lock().expect("mcp session mutex");
        let Some(rec) = g.get(session_id) else {
            return Vec::new();
        };
        rec.ring
            .iter()
            .filter(|e| e.id > after_id)
            .map(|e| e.payload.clone())
            .collect()
    }
}

pub(crate) fn read_session_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get(MCP_SESSION_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(std::string::ToString::to_string)
}

/// When `Origin` is present, require it to match `Host` (scheme-relative compare on host[:port]).
/// Absent `Origin` is allowed (non-browser MCP clients).
pub(crate) fn validate_origin_for_streamable<B>(
    req: &http::Request<B>,
) -> Result<(), &'static str> {
    let Some(origin_raw) = req.headers().get(header::ORIGIN) else {
        return Ok(());
    };
    let Ok(origin) = origin_raw.to_str() else {
        return Err("invalid Origin header encoding");
    };
    let origin = origin.trim();
    if origin.is_empty() || origin.eq_ignore_ascii_case("null") {
        return Ok(());
    }
    let after_scheme = if let Some(idx) = origin.find("://") {
        &origin[idx + 3..]
    } else {
        origin
    };
    let origin_host_port = after_scheme
        .split('/')
        .next()
        .unwrap_or(after_scheme)
        .split('?')
        .next()
        .unwrap_or(after_scheme)
        .to_ascii_lowercase();

    let Some(host_raw) = req.headers().get(header::HOST) else {
        return Err("missing Host header for Origin validation");
    };
    let Ok(host) = host_raw.to_str() else {
        return Err("invalid Host header encoding");
    };
    let host_norm = host.trim().to_ascii_lowercase();
    if origin_host_port != host_norm {
        return Err("Origin does not match Host");
    }
    Ok(())
}

fn parse_last_event_id(last_event_id: Option<&str>) -> u64 {
    last_event_id
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Long-lived SSE stream for GET listener: optional replay of buffered POST responses, then keepalive.
pub(crate) fn mcp_streamable_get_listener_response(
    store: &McpStreamableSessionStore,
    session_id: &str,
    last_event_id: Option<&str>,
) -> Response<BoxBody> {
    let after_id = parse_last_event_id(last_event_id);
    let replay = store.replay_events_after(session_id, after_id);
    let replay_stream = stream::iter(replay.into_iter().map(|b| Ok::<_, Infallible>(Frame::data(b))));
    let mut interval = tokio::time::interval(store.keepalive_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let open = stream::once(async {
        Ok::<_, Infallible>(Frame::data(Bytes::from_static(b": mcp-listener\n\n")))
    });
    let pings = IntervalStream::new(interval)
        .map(|_| Ok::<_, Infallible>(Frame::data(Bytes::from_static(b": ping\n\n"))));
    let stream = replay_stream.chain(open).chain(pings);
    let body = StreamBody::new(stream)
        .map_err(|never: Infallible| match never {})
        .boxed_unsync();
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/event-stream; charset=utf-8"),
        )
        .header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))
        .body(body)
        .unwrap()
}

pub(crate) fn mcp_origin_rejection_response() -> Response<BoxBody> {
    text_response(
        StatusCode::FORBIDDEN,
        "Origin header rejected (MCP Streamable HTTP DNS rebinding protection)",
    )
}

pub(crate) fn mcp_accept_missing_stream_response() -> Response<BoxBody> {
    text_response(
        StatusCode::NOT_ACCEPTABLE,
        "Accept must include text/event-stream for MCP GET listener",
    )
}

pub(crate) fn mcp_session_header_missing_response() -> Response<BoxBody> {
    text_response(
        StatusCode::BAD_REQUEST,
        "Mcp-Session-Id header required for this MCP request",
    )
}

pub(crate) fn mcp_session_unknown_response() -> Response<BoxBody> {
    text_response(StatusCode::NOT_FOUND, "unknown or expired Mcp-Session-Id")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_cap_evicts_oldest() {
        let s = McpStreamableSessionStore::with_options(
            2,
            Duration::from_secs(60),
            Duration::from_secs(20),
        );
        let sid = s.create_session();
        let _ = s.append_sse_message_event(&sid, r#"{"id":1}"#);
        let _ = s.append_sse_message_event(&sid, r#"{"id":2}"#);
        let _ = s.append_sse_message_event(&sid, r#"{"id":3}"#);
        let r = s.replay_events_after(&sid, 0);
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn append_then_replay_respects_last_event_id() {
        let s = McpStreamableSessionStore::new();
        let sid = s.create_session();
        let _ = s.append_sse_message_event(&sid, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#);
        let b2 = s
            .append_sse_message_event(&sid, r#"{"jsonrpc":"2.0","id":2,"result":{}}"#)
            .expect("second chunk");
        let r0 = s.replay_events_after(&sid, 0);
        assert_eq!(r0.len(), 2);
        let r_after_first = s.replay_events_after(&sid, 1);
        assert_eq!(r_after_first.len(), 1);
        assert_eq!(r_after_first[0], b2);
    }
}
