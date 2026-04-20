use serde_json::{json, Value};

pub fn is_anthropic_provider(cfg: &panda_config::PandaConfig, ingress_path: &str) -> bool {
    cfg.effective_adapter_provider(ingress_path) == "anthropic"
}

/// Map OpenAI Chat Completions JSON to Anthropic Messages API request body.
/// Preserves: string/array `content`, `system` messages (top-level `system`), vision parts,
/// and tool rounds (`tool_calls` / `role: tool` / legacy `function`).
pub fn openai_chat_to_anthropic(body: &[u8]) -> anyhow::Result<(Vec<u8>, bool)> {
    let v: serde_json::Value = serde_json::from_slice(body)?;
    let streaming = v.get("stream").and_then(|x| x.as_bool()).unwrap_or(false);
    let model = v
        .get("model")
        .and_then(|x| x.as_str())
        .unwrap_or("claude-3-5-sonnet-latest");
    let max_tokens = v.get("max_tokens").and_then(|x| x.as_u64()).unwrap_or(1024);

    let mut system_texts: Vec<String> = Vec::new();
    let mut anthropic_messages: Vec<Value> = Vec::new();
    let mut pending_tool_results: Vec<Value> = Vec::new();

    let flush_tool_results = |buf: &mut Vec<Value>, out: &mut Vec<Value>| {
        if buf.is_empty() {
            return;
        }
        out.push(json!({
            "role": "user",
            "content": Value::Array(std::mem::take(buf))
        }));
    };

    if let Some(messages) = v.get("messages").and_then(|m| m.as_array()) {
        for m in messages {
            let role = m.get("role").and_then(|x| x.as_str()).unwrap_or("user");

            if role == "system" {
                if let Some(s) = message_text_only(m) {
                    if !s.is_empty() {
                        system_texts.push(s);
                    }
                }
                continue;
            }

            if role == "tool" || role == "function" {
                let tool_call_id = m
                    .get("tool_call_id")
                    .or_else(|| m.get("name"))
                    .and_then(|x| x.as_str())
                    .unwrap_or("");
                let content = tool_result_content_string(m.get("content"));
                let mut block = json!({
                    "type": "tool_result",
                    "tool_use_id": tool_call_id,
                    "content": content
                });
                if let Some(obj) = block.as_object_mut() {
                    if m.get("is_error").and_then(|x| x.as_bool()).unwrap_or(false) {
                        obj.insert("is_error".to_string(), json!(true));
                    }
                }
                pending_tool_results.push(block);
                continue;
            }

            flush_tool_results(&mut pending_tool_results, &mut anthropic_messages);

            match role {
                "user" => {
                    let content = openai_message_to_anthropic_user_content(m)?;
                    anthropic_messages.push(json!({ "role": "user", "content": content }));
                }
                "assistant" => {
                    let content = openai_assistant_to_anthropic_content(m)?;
                    anthropic_messages.push(json!({ "role": "assistant", "content": content }));
                }
                _ => {
                    // e.g. "developer" — treat as user text for compatibility
                    let content = openai_message_to_anthropic_user_content(m)?;
                    anthropic_messages.push(json!({ "role": "user", "content": content }));
                }
            }
        }
    }

    flush_tool_results(&mut pending_tool_results, &mut anthropic_messages);

    if anthropic_messages.is_empty() {
        anthropic_messages.push(json!({
            "role": "user",
            "content": [{"type": "text", "text": ""}]
        }));
    }

    let mut out = json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": anthropic_messages
    });

    if !system_texts.is_empty() {
        if let Some(obj) = out.as_object_mut() {
            obj.insert("system".to_string(), json!(system_texts.join("\n\n")));
        }
    }

    if let Some(tools) = v.get("tools").and_then(|t| t.as_array()) {
        let mapped_tools: Vec<serde_json::Value> = tools
            .iter()
            .filter_map(|t| {
                let f = t.get("function")?;
                let name = f.get("name")?.as_str()?;
                let input_schema = f
                    .get("parameters")
                    .cloned()
                    .unwrap_or_else(|| json!({"type":"object","properties":{}}));
                Some(json!({
                    "name": name,
                    "input_schema": input_schema
                }))
            })
            .collect();
        if !mapped_tools.is_empty() {
            if let Some(obj) = out.as_object_mut() {
                obj.insert("tools".to_string(), serde_json::Value::Array(mapped_tools));
            }
        }
    }
    if let Some(tc) = v.get("tool_choice") {
        let mapped_choice = if tc.is_string() {
            tc.as_str().map(|s| match s {
                "none" => json!({"type":"none"}),
                "required" => json!({"type":"any"}),
                _ => json!({"type":"auto"}),
            })
        } else {
            tc.get("function")
                .and_then(|f| f.get("name"))
                .and_then(|n| n.as_str())
                .map(|name| json!({"type":"tool","name":name}))
        };
        if let Some(choice) = mapped_choice {
            if let Some(obj) = out.as_object_mut() {
                obj.insert("tool_choice".to_string(), choice);
            }
        }
    }
    if streaming {
        if let Some(obj) = out.as_object_mut() {
            obj.insert("stream".to_string(), json!(true));
        }
    }
    Ok((serde_json::to_vec(&out)?, streaming))
}

fn message_text_only(m: &Value) -> Option<String> {
    let c = m.get("content")?;
    match c {
        Value::String(s) => Some(s.clone()),
        Value::Array(parts) => {
            let mut out = String::new();
            for p in parts {
                if p.get("type").and_then(|t| t.as_str()) == Some("text") {
                    if let Some(t) = p.get("text").and_then(|x| x.as_str()) {
                        if !out.is_empty() {
                            out.push('\n');
                        }
                        out.push_str(t);
                    }
                }
            }
            if out.is_empty() {
                None
            } else {
                Some(out)
            }
        }
        _ => None,
    }
}

fn tool_result_content_string(content: Option<&Value>) -> String {
    match content {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.clone(),
        Some(v) => v.to_string(),
    }
}

/// OpenAI user (or unknown) message → Anthropic `content` (string or blocks).
fn openai_message_to_anthropic_user_content(m: &Value) -> anyhow::Result<Value> {
    let c = m.get("content").cloned().unwrap_or(Value::Null);
    openai_content_to_anthropic(&c)
}

fn openai_content_to_anthropic(content: &Value) -> anyhow::Result<Value> {
    match content {
        Value::String(s) => Ok(json!(s)),
        Value::Array(parts) => {
            let mut blocks = Vec::new();
            for p in parts {
                let t = p.get("type").and_then(|x| x.as_str()).unwrap_or("");
                match t {
                    "text" => {
                        if let Some(text) = p.get("text").and_then(|x| x.as_str()) {
                            blocks.push(json!({"type":"text","text": text}));
                        }
                    }
                    "input_text" => {
                        if let Some(text) = p.get("text").and_then(|x| x.as_str()) {
                            blocks.push(json!({"type":"text","text": text}));
                        }
                    }
                    "image_url" => {
                        if let Some(block) = openai_image_part_to_anthropic(p) {
                            blocks.push(block);
                        }
                    }
                    "image" => {
                        if let Some(block) = openai_image_part_to_anthropic(p) {
                            blocks.push(block);
                        }
                    }
                    _ => {
                        // Preserve unknown structured parts as text JSON (lossy but visible)
                        blocks.push(json!({"type":"text","text": p.to_string()}));
                    }
                }
            }
            if blocks.is_empty() {
                Ok(json!([{"type":"text","text":""}]))
            } else if blocks.len() == 1
                && blocks[0].get("type").and_then(|x| x.as_str()) == Some("text")
            {
                Ok(blocks[0].get("text").cloned().unwrap_or(json!("")))
            } else {
                Ok(Value::Array(blocks))
            }
        }
        Value::Null => Ok(json!("")),
        _ => Ok(json!(content.to_string())),
    }
}

fn openai_image_part_to_anthropic(p: &Value) -> Option<Value> {
    let url = p
        .get("image_url")
        .and_then(|u| u.get("url").and_then(|x| x.as_str()))
        .or_else(|| p.get("url").and_then(|x| x.as_str()))?;
    let url = url.trim();
    if url.is_empty() {
        return None;
    }
    if let Some((mime, b64)) = parse_data_image_url(url) {
        return Some(json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": mime,
                "data": b64
            }
        }));
    }
    if url.starts_with("http://") || url.starts_with("https://") {
        return Some(json!({
            "type": "image",
            "source": {
                "type": "url",
                "url": url
            }
        }));
    }
    None
}

fn parse_data_image_url(url: &str) -> Option<(String, String)> {
    let rest = url.strip_prefix("data:")?;
    let (meta, b64) = rest.split_once(',')?;
    let mime = meta
        .strip_suffix(";base64")
        .or_else(|| meta.split(';').next())?;
    if !mime.starts_with("image/") {
        return None;
    }
    Some((mime.to_string(), b64.to_string()))
}

/// OpenAI assistant message: optional text + `tool_calls` → Anthropic content blocks.
fn openai_assistant_to_anthropic_content(m: &Value) -> anyhow::Result<Value> {
    let mut blocks = Vec::new();

    if let Some(c) = m.get("content") {
        if !c.is_null() {
            match c {
                Value::String(s) if !s.is_empty() => {
                    blocks.push(json!({"type":"text","text": s}));
                }
                Value::Array(parts) => {
                    for p in parts {
                        let t = p.get("type").and_then(|x| x.as_str()).unwrap_or("");
                        if t == "text" {
                            if let Some(text) = p.get("text").and_then(|x| x.as_str()) {
                                blocks.push(json!({"type":"text","text": text}));
                            }
                        } else {
                            blocks.push(json!({"type":"text","text": p.to_string()}));
                        }
                    }
                }
                Value::String(_) => {}
                other => {
                    blocks.push(json!({"type":"text","text": other.to_string()}));
                }
            }
        }
    }

    if let Some(tc) = m.get("tool_calls").and_then(|x| x.as_array()) {
        for call in tc {
            let id = call.get("id").and_then(|x| x.as_str()).unwrap_or_default();
            let (name, args_str) = if call.get("function").is_some() {
                let f = call.get("function").unwrap();
                (
                    f.get("name").and_then(|x| x.as_str()).unwrap_or(""),
                    f.get("arguments").and_then(|x| x.as_str()).unwrap_or("{}"),
                )
            } else {
                (
                    call.get("name").and_then(|x| x.as_str()).unwrap_or(""),
                    call.get("arguments")
                        .and_then(|x| x.as_str())
                        .unwrap_or("{}"),
                )
            };
            let input: Value = serde_json::from_str(args_str).unwrap_or_else(|_| json!({}));
            blocks.push(json!({
                "type": "tool_use",
                "id": id,
                "name": name,
                "input": input
            }));
        }
    }

    if blocks.is_empty() {
        Ok(json!([{"type":"text","text":""}]))
    } else if blocks.len() == 1 && blocks[0].get("type").and_then(|x| x.as_str()) == Some("text") {
        Ok(blocks[0].get("text").cloned().unwrap_or(json!("")))
    } else {
        Ok(Value::Array(blocks))
    }
}

pub fn anthropic_to_openai_chat(body: &[u8], model_hint: Option<&str>) -> anyhow::Result<Vec<u8>> {
    let v: serde_json::Value = serde_json::from_slice(body)?;
    let model = v
        .get("model")
        .and_then(|x| x.as_str())
        .or(model_hint)
        .unwrap_or("unknown-model");

    let mut text_parts: Vec<String> = Vec::new();
    let mut tool_calls: Vec<Value> = Vec::new();

    if let Some(arr) = v.get("content").and_then(|x| x.as_array()) {
        for block in arr {
            let t = block.get("type").and_then(|x| x.as_str()).unwrap_or("");
            match t {
                "text" => {
                    if let Some(tx) = block.get("text").and_then(|x| x.as_str()) {
                        text_parts.push(tx.to_string());
                    }
                }
                "tool_use" => {
                    let id = block.get("id").and_then(|x| x.as_str()).unwrap_or("");
                    let name = block.get("name").and_then(|x| x.as_str()).unwrap_or("");
                    let args = block.get("input").cloned().unwrap_or_else(|| json!({}));
                    let args_str =
                        serde_json::to_string(&args).unwrap_or_else(|_| "{}".to_string());
                    tool_calls.push(json!({
                        "id": id,
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": args_str
                        }
                    }));
                }
                _ => {}
            }
        }
    }

    let content = if text_parts.is_empty() {
        String::new()
    } else {
        text_parts.join("\n")
    };

    let mut message = json!({
        "role": "assistant",
        "content": content
    });
    if !tool_calls.is_empty() {
        if let Some(obj) = message.as_object_mut() {
            obj.insert("tool_calls".to_string(), Value::Array(tool_calls));
        }
    }

    let out = json!({
        "id": "chatcmpl-panda-adapter",
        "object": "chat.completion",
        "created": (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)),
        "model": model,
        "choices": [{
            "index": 0,
            "message": message,
            "finish_reason": if v.get("stop_reason").and_then(|x| x.as_str()) == Some("tool_use") {
                "tool_calls"
            } else {
                "stop"
            }
        }]
    });
    Ok(serde_json::to_vec(&out)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_openai_to_anthropic_shape() {
        let body = br#"{"model":"gpt-4o-mini","max_tokens":50,"messages":[{"role":"user","content":"hi"}]}"#;
        let (out, streaming) = openai_chat_to_anthropic(body).unwrap();
        assert!(!streaming);
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(v["model"], "gpt-4o-mini");
        assert_eq!(v["messages"][0]["role"], "user");
    }

    #[test]
    fn maps_openai_streaming_to_anthropic_stream_request() {
        let body =
            br#"{"model":"gpt-4o-mini","stream":true,"messages":[{"role":"user","content":"hi"}]}"#;
        let (out, streaming) = openai_chat_to_anthropic(body).unwrap();
        assert!(streaming);
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(v["stream"], true);
    }

    #[test]
    fn maps_openai_tools_to_anthropic_tools() {
        let body = br#"{
            "model":"gpt-4o-mini",
            "messages":[{"role":"user","content":"weather?"}],
            "tools":[{"type":"function","function":{"name":"get_weather","parameters":{"type":"object","properties":{"city":{"type":"string"}}}}}],
            "tool_choice":"required"
        }"#;
        let (out, _) = openai_chat_to_anthropic(body).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(v["tools"][0]["name"], "get_weather");
        assert_eq!(v["tool_choice"]["type"], "any");
    }

    #[test]
    fn maps_system_messages_to_top_level_system() {
        let body = br#"{
            "model":"claude",
            "messages":[
                {"role":"system","content":"You are helpful."},
                {"role":"user","content":"Hi"}
            ]
        }"#;
        let (out, _) = openai_chat_to_anthropic(body).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(v["system"], "You are helpful.");
        assert_eq!(v["messages"].as_array().unwrap().len(), 1);
        assert_eq!(v["messages"][0]["role"], "user");
    }

    #[test]
    fn maps_multimodal_content_text_and_image_url() {
        let body = br#"{
            "model":"gpt-4o",
            "messages":[{
                "role":"user",
                "content":[
                    {"type":"text","text":"what is this"},
                    {"type":"image_url","image_url":{"url":"https://example.com/a.png"}}
                ]
            }]
        }"#;
        let (out, _) = openai_chat_to_anthropic(body).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        let content = &v["messages"][0]["content"];
        let arr = content.as_array().expect("array content");
        assert_eq!(arr[0]["type"], "text");
        assert_eq!(arr[1]["type"], "image");
        assert_eq!(arr[1]["source"]["type"], "url");
        assert_eq!(arr[1]["source"]["url"], "https://example.com/a.png");
    }

    #[test]
    fn maps_data_url_image_to_base64_source() {
        let body = format!(
            r#"{{"model":"gpt-4o","messages":[{{"role":"user","content":[{{"type":"image_url","image_url":{{"url":"data:image/png;base64,{}"}}}}]}}]}}"#,
            base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &[0x89, 0x50, 0x4e, 0x47]
            )
        );
        let (out, _) = openai_chat_to_anthropic(body.as_bytes()).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        let img = &v["messages"][0]["content"][0];
        assert_eq!(img["type"], "image");
        assert_eq!(img["source"]["type"], "base64");
        assert_eq!(img["source"]["media_type"], "image/png");
    }

    #[test]
    fn maps_assistant_tool_calls_and_tool_results() {
        let body = br#"{
            "model":"gpt-4o-mini",
            "messages":[
                {"role":"user","content":"weather in NYC?"},
                {"role":"assistant","content":"","tool_calls":[{"id":"call_1","type":"function","function":{"name":"get_weather","arguments":"{\"city\":\"NYC\"}"}}]},
                {"role":"tool","tool_call_id":"call_1","content":"72F sunny"}
            ]
        }"#;
        let (out, _) = openai_chat_to_anthropic(body).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        let msgs = v["messages"].as_array().unwrap();
        assert_eq!(msgs.len(), 3);
        let asst = &msgs[1]["content"];
        let asst_arr = asst.as_array().expect("assistant blocks");
        assert_eq!(asst_arr[0]["type"], "tool_use");
        assert_eq!(asst_arr[0]["id"], "call_1");
        assert_eq!(asst_arr[0]["name"], "get_weather");
        let tool_user = &msgs[2]["content"];
        let tr = tool_user.as_array().unwrap();
        assert_eq!(tr[0]["type"], "tool_result");
        assert_eq!(tr[0]["tool_use_id"], "call_1");
        assert_eq!(tr[0]["content"], "72F sunny");
    }

    #[test]
    fn maps_anthropic_to_openai_shape() {
        let body = br#"{"model":"claude","content":[{"type":"text","text":"hello"}]}"#;
        let out = anthropic_to_openai_chat(body, None).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(v["object"], "chat.completion");
        assert_eq!(v["choices"][0]["message"]["content"], "hello");
    }

    #[test]
    fn maps_anthropic_tool_use_to_openai_tool_calls() {
        let body = br#"{
            "model":"claude",
            "stop_reason":"tool_use",
            "content":[
                {"type":"tool_use","id":"tu_1","name":"get_weather","input":{"city":"NYC"}}
            ]
        }"#;
        let out = anthropic_to_openai_chat(body, None).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
        assert_eq!(v["choices"][0]["finish_reason"], "tool_calls");
        let tc = &v["choices"][0]["message"]["tool_calls"][0];
        assert_eq!(tc["id"], "tu_1");
        assert_eq!(tc["function"]["name"], "get_weather");
        assert!(tc["function"]["arguments"]
            .as_str()
            .unwrap()
            .contains("NYC"));
    }
}
