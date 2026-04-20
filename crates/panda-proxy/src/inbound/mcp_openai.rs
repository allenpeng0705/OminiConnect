//! Map MCP tool descriptors to OpenAI Chat Completions `tools` JSON.

use super::mcp::McpToolDescriptor;

/// OpenAI function names allow `[a-zA-Z0-9_-]+`; normalize other characters.
pub fn sanitize_openai_function_name(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn openai_function_name(server: &str, tool: &str) -> String {
    sanitize_openai_function_name(&format!("mcp_{server}_{tool}"))
}

/// JSON array suitable for the `tools` field on chat completion requests.
pub fn openai_tools_json_value(descriptors: &[McpToolDescriptor]) -> serde_json::Value {
    let tools: Vec<serde_json::Value> = descriptors
        .iter()
        .map(|d| {
            let fname = openai_function_name(&d.server, &d.name);
            let params = if d.input_schema.is_null() {
                serde_json::json!({})
            } else {
                d.input_schema.clone()
            };
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": fname,
                    "description": d.description.as_deref().unwrap_or(""),
                    "parameters": params,
                }
            })
        })
        .collect();
    serde_json::Value::Array(tools)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inbound::mcp::McpToolDescriptor;

    #[test]
    fn sanitize_replaces_invalid_chars() {
        assert_eq!(sanitize_openai_function_name("a b.c"), "a_b_c");
    }

    #[test]
    fn openai_tools_serializes_one_function() {
        let d = McpToolDescriptor {
            server: "srv".into(),
            name: "do_thing".into(),
            description: Some("does a thing".into()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "q": { "type": "string" } }
            }),
        };
        let v = openai_tools_json_value(std::slice::from_ref(&d));
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let name = arr[0]["function"]["name"].as_str().unwrap();
        assert_eq!(name, "mcp_srv_do_thing");
    }
}
