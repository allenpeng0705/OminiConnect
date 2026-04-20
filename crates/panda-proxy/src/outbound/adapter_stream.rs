//! Anthropic `text/event-stream` (Messages API) → OpenAI `chat.completion.chunk` SSE.
//!
//! Maps `text_delta` to `delta.content`, `input_json_delta` + `content_block_start` (tool_use) to
//! `delta.tool_calls`, and `message_delta.stop_reason` to the final `finish_reason`.

use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use hyper::body::{Body, Frame, Incoming};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::PandaBodyError;

pub struct AnthropicToOpenAiSseBody<B = Incoming> {
    inner: B,
    buf: BytesMut,
    pending: VecDeque<Bytes>,
    openai_id: String,
    model: String,
    role_sent: bool,
    finished: bool,
    created: u64,
    /// Anthropic content block `index` → OpenAI `tool_calls[].index` (0-based tool ordinal).
    anthropic_block_to_tool_index: HashMap<u64, u32>,
    next_tool_index: u32,
    /// Last `message_delta.delta.stop_reason` before `message_stop`.
    pending_stop_reason: Option<String>,
}

impl<B> AnthropicToOpenAiSseBody<B> {
    pub fn new(inner: B, model_hint: Option<String>) -> Self {
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            inner,
            buf: BytesMut::new(),
            pending: VecDeque::new(),
            openai_id: format!("chatcmpl-{}", Uuid::new_v4()),
            model: model_hint.unwrap_or_else(|| "claude".to_string()),
            role_sent: false,
            finished: false,
            created,
            anthropic_block_to_tool_index: HashMap::new(),
            next_tool_index: 0,
            pending_stop_reason: None,
        }
    }

    fn enqueue_finish(&mut self) {
        if self.finished {
            return;
        }
        self.finished = true;
        self.pending.push_back(finish_chunk(
            &self.openai_id,
            &self.model,
            self.created,
            self.pending_stop_reason.as_deref(),
        ));
        self.pending.push_back(Bytes::from("data: [DONE]\n\n"));
    }

    fn ensure_role(&mut self) {
        if !self.role_sent {
            self.role_sent = true;
            self.pending
                .push_back(role_chunk(&self.openai_id, &self.model, self.created));
        }
    }

    fn process_json_event(&mut self, v: &Value) {
        let Some(typ) = v.get("type").and_then(|x| x.as_str()) else {
            return;
        };
        match typ {
            "message_start" => {
                if let Some(m) = v.get("message") {
                    if let Some(mid) = m.get("model").and_then(|x| x.as_str()) {
                        self.model = mid.to_string();
                    }
                }
                if !self.role_sent {
                    self.role_sent = true;
                    self.pending
                        .push_back(role_chunk(&self.openai_id, &self.model, self.created));
                }
            }
            "content_block_start" => {
                let Some(idx) = v.get("index").and_then(|x| x.as_u64()) else {
                    return;
                };
                let Some(cb) = v.get("content_block") else {
                    return;
                };
                let cb_type = cb.get("type").and_then(|x| x.as_str()).unwrap_or("");
                if cb_type == "tool_use" {
                    let id = cb.get("id").and_then(|x| x.as_str()).unwrap_or("");
                    let name = cb.get("name").and_then(|x| x.as_str()).unwrap_or("");
                    let openai_i = self.next_tool_index;
                    self.next_tool_index = self.next_tool_index.saturating_add(1);
                    self.anthropic_block_to_tool_index.insert(idx, openai_i);
                    self.ensure_role();
                    self.pending.push_back(tool_calls_header_chunk(
                        &self.openai_id,
                        &self.model,
                        self.created,
                        openai_i,
                        id,
                        name,
                    ));
                }
            }
            "content_block_delta" => {
                if let Some(t) = anthropic_delta_text(v) {
                    self.ensure_role();
                    self.pending.push_back(content_chunk(
                        &self.openai_id,
                        &self.model,
                        self.created,
                        t,
                    ));
                } else if let Some((idx, frag)) = anthropic_delta_input_json(v) {
                    self.ensure_role();
                    if let Some(&openai_i) = self.anthropic_block_to_tool_index.get(&idx) {
                        if !frag.is_empty() {
                            self.pending.push_back(tool_calls_arguments_chunk(
                                &self.openai_id,
                                &self.model,
                                self.created,
                                openai_i,
                                frag,
                            ));
                        }
                    }
                }
            }
            "message_delta" => {
                if let Some(sr) = v
                    .get("delta")
                    .and_then(|d| d.get("stop_reason"))
                    .and_then(|x| x.as_str())
                {
                    self.pending_stop_reason = Some(sr.to_string());
                }
            }
            "message_stop" => {
                self.enqueue_finish();
            }
            _ => {}
        }
    }

    fn process_line(&mut self, line: &[u8]) {
        if line.is_empty() {
            return;
        }
        let rest = if line.len() >= 5 && &line[..5] == b"data:" {
            trim_bytes(&line[5..])
        } else {
            return;
        };
        if rest == b"[DONE]" {
            self.enqueue_finish();
            return;
        }
        if let Ok(v) = serde_json::from_slice::<Value>(rest) {
            self.process_json_event(&v);
        }
    }

    fn process_buffer(&mut self) {
        while let Some(pos) = self.buf.iter().position(|&b| b == b'\n') {
            let mut line = self.buf.split_to(pos + 1);
            if line.last() == Some(&b'\n') {
                line.truncate(line.len().saturating_sub(1));
            }
            if line.last() == Some(&b'\r') {
                line.truncate(line.len().saturating_sub(1));
            }
            self.process_line(&line[..]);
        }
    }
}

fn anthropic_delta_text(v: &Value) -> Option<&str> {
    let delta = v.get("delta")?;
    match delta.get("type").and_then(|t| t.as_str()) {
        Some("text_delta") => delta.get("text").and_then(|x| x.as_str()),
        Some("input_json_delta") => None,
        _ => delta.get("text").and_then(|x| x.as_str()),
    }
}

/// `(anthropic_block_index, partial_json_fragment)` for tool input streaming.
fn anthropic_delta_input_json(v: &Value) -> Option<(u64, &str)> {
    let idx = v.get("index").and_then(|x| x.as_u64())?;
    let delta = v.get("delta")?;
    if delta.get("type").and_then(|t| t.as_str()) != Some("input_json_delta") {
        return None;
    }
    let frag = delta
        .get("partial_json")
        .and_then(|x| x.as_str())
        .unwrap_or("");
    Some((idx, frag))
}

fn trim_bytes(b: &[u8]) -> &[u8] {
    let mut s = b;
    while s.first().is_some_and(|x| x.is_ascii_whitespace()) {
        s = &s[1..];
    }
    while s.last().is_some_and(|x| x.is_ascii_whitespace()) {
        s = &s[..s.len() - 1];
    }
    s
}

fn role_chunk(id: &str, model: &str, created: u64) -> Bytes {
    let payload = json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": {"role": "assistant"},
            "finish_reason": null
        }]
    });
    let mut s = String::from("data: ");
    s.push_str(&payload.to_string());
    s.push_str("\n\n");
    Bytes::from(s)
}

fn content_chunk(id: &str, model: &str, created: u64, text: &str) -> Bytes {
    let payload = json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": {"content": text},
            "finish_reason": null
        }]
    });
    let mut s = String::from("data: ");
    s.push_str(&payload.to_string());
    s.push_str("\n\n");
    Bytes::from(s)
}

fn tool_calls_header_chunk(
    id: &str,
    model: &str,
    created: u64,
    tool_index: u32,
    tool_id: &str,
    name: &str,
) -> Bytes {
    let payload = json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": {
                "tool_calls": [{
                    "index": tool_index,
                    "id": tool_id,
                    "type": "function",
                    "function": {
                        "name": name,
                        "arguments": ""
                    }
                }]
            },
            "finish_reason": null
        }]
    });
    let mut s = String::from("data: ");
    s.push_str(&payload.to_string());
    s.push_str("\n\n");
    Bytes::from(s)
}

fn tool_calls_arguments_chunk(
    id: &str,
    model: &str,
    created: u64,
    tool_index: u32,
    arguments_fragment: &str,
) -> Bytes {
    let payload = json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": {
                "tool_calls": [{
                    "index": tool_index,
                    "function": {
                        "arguments": arguments_fragment
                    }
                }]
            },
            "finish_reason": null
        }]
    });
    let mut s = String::from("data: ");
    s.push_str(&payload.to_string());
    s.push_str("\n\n");
    Bytes::from(s)
}

fn finish_chunk(id: &str, model: &str, created: u64, anthropic_stop_reason: Option<&str>) -> Bytes {
    let finish = map_anthropic_stop_reason_to_openai(anthropic_stop_reason);
    let payload = json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": {},
            "finish_reason": finish
        }]
    });
    let mut s = String::from("data: ");
    s.push_str(&payload.to_string());
    s.push_str("\n\n");
    Bytes::from(s)
}

fn map_anthropic_stop_reason_to_openai(sr: Option<&str>) -> Value {
    match sr {
        Some("tool_use") => json!("tool_calls"),
        Some("max_tokens") => json!("length"),
        Some("end_turn") | None => json!("stop"),
        _ => json!("stop"),
    }
}

impl<B> Body for AnthropicToOpenAiSseBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    type Data = Bytes;
    type Error = PandaBodyError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();

        if let Some(b) = this.pending.pop_front() {
            return Poll::Ready(Some(Ok(Frame::data(b))));
        }

        if this.finished {
            return Poll::Ready(None);
        }

        let polled = Pin::new(&mut this.inner).poll_frame(cx);
        match polled {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => {
                if !this.buf.is_empty() {
                    let tail = std::mem::replace(&mut this.buf, BytesMut::new());
                    let mut bytes = tail.freeze();
                    if bytes.last() == Some(&b'\r') {
                        bytes = bytes.slice(..bytes.len().saturating_sub(1));
                    }
                    this.process_line(&bytes[..]);
                }
                this.process_buffer();
                if !this.finished {
                    this.enqueue_finish();
                }
                if let Some(b) = this.pending.pop_front() {
                    Poll::Ready(Some(Ok(Frame::data(b))))
                } else {
                    Poll::Ready(None)
                }
            }
            Poll::Ready(Some(Ok(frame))) => match frame.into_data() {
                Ok(mut data) => {
                    let data = data.copy_to_bytes(data.remaining());
                    this.buf.put_slice(&data);
                    this.process_buffer();
                    if let Some(b) = this.pending.pop_front() {
                        Poll::Ready(Some(Ok(Frame::data(b))))
                    } else {
                        Poll::Pending
                    }
                }
                Err(frame) => Poll::Ready(Some(Ok(frame))),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_anthropic_delta_to_openai_chunk_shape() {
        let v: Value = serde_json::from_str(
            r#"{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hi"}}"#,
        )
        .unwrap();
        let t = anthropic_delta_text(&v).unwrap();
        assert_eq!(t, "Hi");
    }

    #[test]
    fn input_json_delta_not_treated_as_text() {
        let v: Value = serde_json::from_str(
            r#"{"type":"content_block_delta","index":1,"delta":{"type":"input_json_delta","partial_json":"{\"x\":1}"}}"#,
        )
        .unwrap();
        assert!(anthropic_delta_text(&v).is_none());
        let (idx, frag) = anthropic_delta_input_json(&v).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(frag, "{\"x\":1}");
    }

    #[test]
    fn maps_stop_reason_tool_use_to_finish_tool_calls() {
        let f = map_anthropic_stop_reason_to_openai(Some("tool_use"));
        assert_eq!(f, json!("tool_calls"));
        let f2 = map_anthropic_stop_reason_to_openai(Some("end_turn"));
        assert_eq!(f2, json!("stop"));
        let f3 = map_anthropic_stop_reason_to_openai(Some("max_tokens"));
        assert_eq!(f3, json!("length"));
    }

    #[test]
    fn tool_calls_header_chunk_has_openai_shape() {
        let b = tool_calls_header_chunk("id1", "m", 1, 0, "toolu_1", "get_weather");
        let s = String::from_utf8_lossy(&b);
        assert!(s.contains("tool_calls"));
        assert!(s.contains("get_weather"));
        assert!(s.contains("\"index\":0"));
    }

    #[test]
    fn tool_calls_arguments_chunk_appends_fragment() {
        let b = tool_calls_arguments_chunk("id1", "m", 1, 0, "{\"a\"");
        let s = String::from_utf8_lossy(&b);
        assert!(s.contains("arguments"));
        let v: Value = serde_json::from_str(
            s.trim()
                .strip_prefix("data: ")
                .unwrap()
                .trim_end_matches("\n\n"),
        )
        .unwrap();
        assert_eq!(
            v["choices"][0]["delta"]["tool_calls"][0]["function"]["arguments"],
            "{\"a\""
        );
    }
}
