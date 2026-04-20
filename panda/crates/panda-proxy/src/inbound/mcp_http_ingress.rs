//! MCP **server** JSON-RPC 2.0 over HTTP for [`panda_config::ApiGatewayIngressBackend::Mcp`] routes.
//!
//! Implements **[Streamable HTTP](https://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http)**
//! (2025-03-26): POST + GET listener SSE, DELETE session, `Mcp-Session-Id`, Origin checks, 202 for
//! notification-only POSTs, JSON-RPC batches (sequential), and SSE or JSON responses for requests.

use std::convert::Infallible;

use bytes::Bytes;
use http::header::{self, HeaderName, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Method, Request, Response, StatusCode};
use serde_json::{json, Map, Value};

use super::mcp;
use super::mcp_openai;
use super::mcp_streamable_http::{
    self, mcp_accept_missing_stream_response, mcp_origin_rejection_response,
    mcp_session_header_missing_response, mcp_session_unknown_response,
    mcp_streamable_get_listener_response, read_session_id, validate_origin_for_streamable,
};

use crate::{
    collect_body_bounded, enforce_jwt_if_required, json_response,
    mcp_ingress_emit_jsonrpc_envelope, ProxyError, ProxyState,
};

fn empty_accepted_response() -> Response<crate::BoxBody> {
    let body = Full::new(Bytes::new())
        .map_err(|never: std::convert::Infallible| match never {})
        .boxed_unsync();
    Response::builder()
        .status(StatusCode::ACCEPTED)
        .body(body)
        .unwrap()
}

fn empty_204_response() -> Response<crate::BoxBody> {
    let body = Full::new(Bytes::new())
        .map_err(|never: std::convert::Infallible| match never {})
        .boxed_unsync();
    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(body)
        .unwrap()
}

fn mcp_options_response() -> Response<crate::BoxBody> {
    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header(header::ALLOW, "GET, POST, DELETE, OPTIONS")
        .body(
            Full::new(Bytes::new())
                .map_err(|never: std::convert::Infallible| match never {})
                .boxed_unsync(),
        )
        .unwrap()
}

fn method_not_allowed_mcp() -> Response<crate::BoxBody> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header(header::ALLOW, "GET, POST, DELETE, OPTIONS")
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(
            Full::new(Bytes::copy_from_slice(
                b"use GET (SSE listener), POST (JSON-RPC), or DELETE (session) on MCP endpoint",
            ))
            .map_err(|never: std::convert::Infallible| match never {})
            .boxed_unsync(),
        )
        .unwrap()
}

fn mcp_post_accept_valid_streamable(headers: &http::HeaderMap) -> bool {
    let Some(a) = headers.get(header::ACCEPT).and_then(|v| v.to_str().ok()) else {
        return false;
    };
    let l = a.to_ascii_lowercase();
    l.contains("application/json") && l.contains("text/event-stream")
}

fn mcp_post_accept_invalid_response() -> Response<crate::BoxBody> {
    crate::text_response(
        StatusCode::NOT_ACCEPTABLE,
        "Accept must list both application/json and text/event-stream (MCP Streamable HTTP)",
    )
}

fn mcp_request_accepts_event_stream(headers: &http::HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|a| {
            let l = a.to_ascii_lowercase();
            l.contains("text/event-stream") || l.contains("application/x-ndjson")
        })
        .unwrap_or(false)
}

fn inject_mcp_session_header(resp: &mut Response<crate::BoxBody>, session_id: &str) {
    if let Ok(v) = HeaderValue::from_str(session_id) {
        resp.headers_mut().insert(
            HeaderName::from_static(mcp_streamable_http::MCP_SESSION_ID_HEADER),
            v,
        );
    }
}

fn jsonrpc_result(
    state: &ProxyState,
    accept_sse: bool,
    id: Value,
    result: Value,
    new_session: Option<&str>,
    parts: &http::request::Parts,
) -> Response<crate::BoxBody> {
    let session_for_buffer = new_session
        .map(|s| s.to_string())
        .or_else(|| read_session_id(&parts.headers));
    let buf = if accept_sse {
        session_for_buffer
            .as_deref()
            .map(|s| (state.mcp_streamable_sessions.as_ref(), s))
    } else {
        None
    };
    let mut r = mcp_ingress_emit_jsonrpc_envelope(
        accept_sse,
        StatusCode::OK,
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result,
        }),
        buf,
    );
    if let Some(s) = new_session {
        inject_mcp_session_header(&mut r, s);
    }
    r
}

fn jsonrpc_error_status(
    state: &ProxyState,
    accept_sse: bool,
    status: StatusCode,
    id: Value,
    code: i32,
    message: &str,
    mcp_session_id: Option<String>,
) -> Response<crate::BoxBody> {
    let buf = if accept_sse {
        mcp_session_id
            .as_deref()
            .map(|s| (state.mcp_streamable_sessions.as_ref(), s))
    } else {
        None
    };
    mcp_ingress_emit_jsonrpc_envelope(
        accept_sse,
        status,
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": code, "message": message }
        }),
        buf,
    )
}

fn jsonrpc_object_is_request_with_id(obj: &Map<String, Value>) -> bool {
    obj.get("method").and_then(|m| m.as_str()).is_some() && obj.contains_key("id")
}

fn jsonrpc_object_is_notification(obj: &Map<String, Value>) -> bool {
    obj.get("method").is_some() && !obj.contains_key("id")
}

fn jsonrpc_object_is_client_response_only(obj: &Map<String, Value>) -> bool {
    !obj.contains_key("method")
        && obj.contains_key("id")
        && (obj.contains_key("result") || obj.contains_key("error"))
}

fn last_event_id_header(headers: &http::HeaderMap) -> Option<&str> {
    headers
        .get("Last-Event-ID")
        .or_else(|| headers.get("last-event-id"))
        .and_then(|v| v.to_str().ok())
}

#[cfg(test)]
fn tool_call_mcp_result(res: &mcp::McpToolCallResult) -> Value {
    let content = match &res.content {
        Value::Array(_) => res.content.clone(),
        Value::String(s) => json!([{ "type": "text", "text": s }]),
        other if other.is_null() => json!([]),
        other => json!([{ "type": "text", "text": other.to_string() }]),
    };
    json!({
        "content": content,
        "isError": res.is_error,
    })
}

async fn read_response_json_envelope(resp: Response<crate::BoxBody>) -> Value {
    let (_parts, body) = resp.into_parts();
    match body.collect().await {
        Ok(collected) => {
            let bytes = collected.to_bytes();
            serde_json::from_slice(&bytes).unwrap_or(json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": { "code": -32603, "message": "failed to read JSON-RPC body" }
            }))
        }
        Err(_) => json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": { "code": -32603, "message": "failed to collect body" }
        }),
    }
}

/// Core JSON-RPC dispatch after streamable session / JSON shape checks.
#[allow(clippy::too_many_arguments)]
async fn dispatch_jsonrpc_object(
    state: &ProxyState,
    rt: &std::sync::Arc<mcp::McpRuntime>,
    parts: &http::request::Parts,
    obj: &Map<String, Value>,
    correlation_id: &str,
    ingress_path: &str,
    accept_sse: bool,
    force_json_body: bool,
    new_session_for_initialize: Option<&str>,
) -> Result<Response<crate::BoxBody>, Infallible> {
    if obj.get("jsonrpc").and_then(|x| x.as_str()) != Some("2.0") {
        let id = obj.get("id").cloned().unwrap_or(Value::Null);
        return Ok(jsonrpc_error_status(
            state,
            accept_sse,
            StatusCode::BAD_REQUEST,
            id,
            -32600,
            "jsonrpc must be \"2.0\"",
            read_session_id(&parts.headers),
        ));
    }

    let method = obj
        .get("method")
        .and_then(|m| m.as_str())
        .unwrap_or("")
        .to_string();

    if method.is_empty() {
        let id = obj.get("id").cloned().unwrap_or(Value::Null);
        return Ok(jsonrpc_error_status(
            state,
            accept_sse,
            StatusCode::BAD_REQUEST,
            id,
            -32600,
            "missing method",
            read_session_id(&parts.headers),
        ));
    }

    let is_notification = !obj.contains_key("id");
    let id = obj.get("id").cloned().unwrap_or(Value::Null);
    let params = obj.get("params").cloned().unwrap_or(Value::Null);

    match method.as_str() {
        "notifications/initialized" | "notifications/cancelled" if is_notification => {
            return Ok(empty_accepted_response());
        }
        "notifications/initialized" | "notifications/cancelled" => {
            return Ok(jsonrpc_error_status(
                state,
                accept_sse,
                StatusCode::BAD_REQUEST,
                id,
                -32600,
                "notifications must omit id",
                read_session_id(&parts.headers),
            ));
        }
        _ => {}
    }

    if is_notification {
        return Ok(empty_accepted_response());
    }

    let sse = accept_sse && !force_json_body;

    match method.as_str() {
        "initialize" => {
            let result = json!({
                "protocolVersion": "2025-03-26",
                "capabilities": { "tools": {}, "resources": {}, "prompts": {} },
                "serverInfo": {
                    "name": "panda",
                    "version": env!("CARGO_PKG_VERSION"),
                }
            });
            let sid = new_session_for_initialize
                .map(std::string::ToString::to_string)
                .unwrap_or_else(|| state.mcp_streamable_sessions.create_session());
            Ok(jsonrpc_result(
                state,
                sse,
                id,
                result,
                Some(sid.as_str()),
                parts,
            ))
        }
        "ping" => Ok(jsonrpc_result(state, sse, id, json!({}), None, parts)),
        "tools/list" => match rt.list_all_tools().await {
            Ok(tools) => {
                let mcp_tools: Vec<Value> = tools
                    .iter()
                    .map(|t| {
                        let name = mcp_openai::openai_function_name(&t.server, &t.name);
                        json!({
                            "name": name,
                            "description": t.description.as_deref().unwrap_or(""),
                            "inputSchema": t.input_schema.clone(),
                        })
                    })
                    .collect();
                Ok(jsonrpc_result(
                    state,
                    sse,
                    id,
                    json!({ "tools": mcp_tools }),
                    None,
                    parts,
                ))
            }
            Err(e) => Ok(jsonrpc_error_status(
                state,
                sse,
                StatusCode::INTERNAL_SERVER_ERROR,
                id,
                -32603,
                &format!("tools/list failed: {e:#}"),
                read_session_id(&parts.headers),
            )),
        },
        "tools/call" => {
            let Some(params_o) = params.as_object() else {
                return Ok(jsonrpc_error_status(
                    state,
                    sse,
                    StatusCode::BAD_REQUEST,
                    id,
                    -32602,
                    "params must be an object",
                    read_session_id(&parts.headers),
                ));
            };
            let Some(tool_name) = params_o.get("name").and_then(|n| n.as_str()) else {
                return Ok(jsonrpc_error_status(
                    state,
                    sse,
                    StatusCode::BAD_REQUEST,
                    id,
                    -32602,
                    "missing params.name",
                    read_session_id(&parts.headers),
                ));
            };
            let Some((server, tool)) =
                mcp::parse_openai_function_name(tool_name, &state.config.mcp.servers)
            else {
                return Ok(jsonrpc_error_status(
                    state,
                    sse,
                    StatusCode::BAD_REQUEST,
                    id,
                    -32602,
                    "unknown tool (expected mcp_{server}_{tool} name)",
                    read_session_id(&parts.headers),
                ));
            };
            let arguments = params_o
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            let ctx = crate::mcp_http_ingress_build_context(
                &parts.headers,
                ingress_path,
                correlation_id,
                state,
            )
            .await;
            Ok(crate::mcp_http_ingress_execute_tools_call(
                state,
                rt,
                &ctx,
                ingress_path,
                server,
                tool,
                tool_name,
                arguments,
                id,
                sse,
                read_session_id(&parts.headers),
            )
            .await)
        }
        "resources/list" => Ok(jsonrpc_result(
            state,
            sse,
            id,
            json!({ "resources": [] }),
            None,
            parts,
        )),
        "prompts/list" => Ok(jsonrpc_result(
            state,
            sse,
            id,
            json!({ "prompts": [] }),
            None,
            parts,
        )),
        _ => Ok(jsonrpc_error_status(
            state,
            sse,
            StatusCode::OK,
            id,
            -32601,
            "Method not found",
            read_session_id(&parts.headers),
        )),
    }
}

async fn handle_post_batch(
    state: &ProxyState,
    rt: &std::sync::Arc<mcp::McpRuntime>,
    parts: &http::request::Parts,
    arr: &[Value],
    correlation_id: &str,
    ingress_path: &str,
    accept_sse: bool,
) -> Result<Response<crate::BoxBody>, Infallible> {
    if arr.is_empty() {
        return Ok(jsonrpc_error_status(
            state,
            accept_sse,
            StatusCode::BAD_REQUEST,
            Value::Null,
            -32600,
            "empty JSON-RPC batch",
            read_session_id(&parts.headers),
        ));
    }

    let has_initialize = arr.iter().any(|v| {
        v.as_object()
            .and_then(|o| o.get("method").and_then(|m| m.as_str()))
            == Some("initialize")
    });
    if has_initialize && arr.len() > 1 {
        return Ok(jsonrpc_error_status(
            state,
            accept_sse,
            StatusCode::BAD_REQUEST,
            Value::Null,
            -32600,
            "batch must not mix initialize with other JSON-RPC messages",
            read_session_id(&parts.headers),
        ));
    }

    let needs_response_array = arr.iter().any(|v| {
        v.as_object()
            .map(jsonrpc_object_is_request_with_id)
            .unwrap_or(false)
    });

    if !needs_response_array {
        for v in arr {
            let Some(obj) = v.as_object() else {
                return Ok(jsonrpc_error_status(
                    state,
                    accept_sse,
                    StatusCode::BAD_REQUEST,
                    Value::Null,
                    -32600,
                    "batch elements must be JSON objects",
                    read_session_id(&parts.headers),
                ));
            };
            if jsonrpc_object_is_request_with_id(obj) {
                continue;
            }
            if jsonrpc_object_is_notification(obj) {
                continue;
            }
            if jsonrpc_object_is_client_response_only(obj) {
                continue;
            }
            return Ok(jsonrpc_error_status(
                state,
                accept_sse,
                StatusCode::BAD_REQUEST,
                Value::Null,
                -32600,
                "invalid JSON-RPC batch element",
                read_session_id(&parts.headers),
            ));
        }
        return Ok(empty_accepted_response());
    }

    let header_sid = read_session_id(&parts.headers);
    let mut implicit_sid: Option<String> = None;

    let mut out: Vec<Value> = Vec::new();
    for v in arr {
        let Some(obj) = v.as_object() else {
            return Ok(jsonrpc_error_status(
                state,
                accept_sse,
                StatusCode::BAD_REQUEST,
                Value::Null,
                -32600,
                "batch elements must be JSON objects",
                read_session_id(&parts.headers),
            ));
        };

        if jsonrpc_object_is_notification(obj) || jsonrpc_object_is_client_response_only(obj) {
            continue;
        }

        if !jsonrpc_object_is_request_with_id(obj) {
            return Ok(jsonrpc_error_status(
                state,
                accept_sse,
                StatusCode::BAD_REQUEST,
                Value::Null,
                -32600,
                "invalid JSON-RPC batch element",
                read_session_id(&parts.headers),
            ));
        }

        let method = obj
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        if method != "initialize" {
            let sid = implicit_sid.as_deref().or(header_sid.as_deref());
            let Some(sid) = sid else {
                return Ok(mcp_session_header_missing_response());
            };
            if !state.mcp_streamable_sessions.validate_and_touch(sid) {
                return Ok(mcp_session_unknown_response());
            }
        }

        let init_sid_storage: Option<String> = if method == "initialize" {
            Some(state.mcp_streamable_sessions.create_session())
        } else {
            None
        };

        let resp = dispatch_jsonrpc_object(
            state,
            rt,
            parts,
            obj,
            correlation_id,
            ingress_path,
            accept_sse,
            true,
            init_sid_storage.as_deref(),
        )
        .await?;

        if method == "initialize" {
            implicit_sid = init_sid_storage.clone();
        }

        out.push(read_response_json_envelope(resp).await);
    }

    Ok(json_response(StatusCode::OK, Value::Array(out)))
}

/// Handle one HTTP request for an ingress-classified MCP route (`backend: mcp`).
pub(crate) async fn handle_mcp_ingress_http(
    req: Request<Incoming>,
    state: &ProxyState,
    correlation_id: &str,
    ingress_path: &str,
) -> Result<Response<crate::BoxBody>, Infallible> {
    if req.method() == &Method::OPTIONS {
        return Ok(mcp_options_response());
    }

    if validate_origin_for_streamable(&req).is_err() {
        return Ok(mcp_origin_rejection_response());
    }

    match *req.method() {
        Method::DELETE => {
            if let Err(resp) = enforce_jwt_if_required(&req, state).await {
                return Ok(resp);
            }
            let Some(sid) = read_session_id(req.headers()) else {
                return Ok(mcp_session_header_missing_response());
            };
            if state.mcp_streamable_sessions.remove_session(&sid) {
                return Ok(empty_204_response());
            }
            Ok(mcp_session_unknown_response())
        }
        Method::GET => {
            if let Err(resp) = enforce_jwt_if_required(&req, state).await {
                return Ok(resp);
            }
            if !mcp_request_accepts_event_stream(req.headers()) {
                return Ok(mcp_accept_missing_stream_response());
            }
            let Some(sid) = read_session_id(req.headers()) else {
                return Ok(mcp_session_header_missing_response());
            };
            if !state.mcp_streamable_sessions.validate_and_touch(&sid) {
                return Ok(mcp_session_unknown_response());
            }
            let leid = last_event_id_header(req.headers());
            Ok(mcp_streamable_get_listener_response(
                state.mcp_streamable_sessions.as_ref(),
                sid.as_str(),
                leid,
            ))
        }
        Method::POST => {
            if let Err(resp) = enforce_jwt_if_required(&req, state).await {
                return Ok(resp);
            }
            if !mcp_post_accept_valid_streamable(req.headers()) {
                return Ok(mcp_post_accept_invalid_response());
            }

            let accept_sse = mcp_request_accepts_event_stream(req.headers());

            let Some(rt) = state.mcp.as_ref() else {
                return Ok(jsonrpc_error_status(
                    state,
                    accept_sse,
                    StatusCode::SERVICE_UNAVAILABLE,
                    Value::Null,
                    -32000,
                    "MCP runtime not available (mcp.enabled false or no connected servers)",
                    None,
                ));
            };

            let max_body = state.config.plugins.max_request_body_bytes;
            let (parts, body) = req.into_parts();
            let body = match collect_body_bounded(body, max_body).await {
                Ok(b) => b,
                Err(ProxyError::PayloadTooLarge(_)) => {
                    return Ok(jsonrpc_error_status(
                        state,
                        accept_sse,
                        StatusCode::PAYLOAD_TOO_LARGE,
                        Value::Null,
                        -32600,
                        "request body exceeds configured limit",
                        read_session_id(&parts.headers),
                    ));
                }
                Err(_) => {
                    return Ok(jsonrpc_error_status(
                        state,
                        accept_sse,
                        StatusCode::BAD_REQUEST,
                        Value::Null,
                        -32700,
                        "failed to read body",
                        read_session_id(&parts.headers),
                    ));
                }
            };

            let v: Value = match serde_json::from_slice(&body) {
                Ok(v) => v,
                Err(_) => {
                    return Ok(jsonrpc_error_status(
                        state,
                        accept_sse,
                        StatusCode::BAD_REQUEST,
                        Value::Null,
                        -32700,
                        "parse error",
                        read_session_id(&parts.headers),
                    ));
                }
            };

            if let Some(arr) = v.as_array() {
                return handle_post_batch(
                    state,
                    rt,
                    &parts,
                    arr.as_slice(),
                    correlation_id,
                    ingress_path,
                    accept_sse,
                )
                .await;
            }

            let Some(obj) = v.as_object() else {
                return Ok(jsonrpc_error_status(
                    state,
                    accept_sse,
                    StatusCode::BAD_REQUEST,
                    Value::Null,
                    -32600,
                    "invalid request",
                    read_session_id(&parts.headers),
                ));
            };

            let method = obj
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or("")
                .to_string();

            if method != "initialize" {
                let sid = read_session_id(&parts.headers);
                if sid.is_none() {
                    return Ok(mcp_session_header_missing_response());
                }
                if !state
                    .mcp_streamable_sessions
                    .validate_and_touch(sid.as_ref().unwrap())
                {
                    return Ok(mcp_session_unknown_response());
                }
            }

            dispatch_jsonrpc_object(
                state,
                rt,
                &parts,
                obj,
                correlation_id,
                ingress_path,
                accept_sse,
                false,
                None,
            )
            .await
        }
        _ => Ok(method_not_allowed_mcp()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_call_mcp_result_wraps_string() {
        let r = mcp::McpToolCallResult {
            content: json!("hello"),
            is_error: false,
        };
        let v = tool_call_mcp_result(&r);
        assert_eq!(v["isError"], false);
        let a = v["content"].as_array().unwrap();
        assert_eq!(a[0]["type"], "text");
        assert_eq!(a[0]["text"], "hello");
    }
}
