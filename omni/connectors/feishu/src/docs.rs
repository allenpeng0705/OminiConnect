//! Documentation generation for Feishu connector tools.

use crate::tools::FeishuTool;

/// Generate markdown documentation for all Feishu tools
pub fn generate_markdown() -> String {
    let tools = FeishuTool::all_tools();
    let mut output = String::from("# Feishu Connector Tools\n\n");
    output.push_str(&format!("Total tools: {}\n\n", tools.len()));

    let calendar_tools: Vec<_> = tools.iter().filter(|t| t.name.starts_with("calendar_")).collect();
    let bitable_tools: Vec<_> = tools.iter().filter(|t| t.name.starts_with("bitable_")).collect();
    let message_tools: Vec<_> = tools.iter().filter(|t| t.name.starts_with("message_")).collect();

    if !calendar_tools.is_empty() {
        output.push_str("## Calendar\n\n");
        for tool in calendar_tools {
            output.push_str(&format!(
                "### `{}`\n\n{}\n\n**Input Schema:**\n```json\n{}\n```\n\n",
                tool.name,
                tool.description,
                serde_json::to_string_pretty(&tool.input_schema).unwrap_or_default()
            ));
        }
    }

    if !bitable_tools.is_empty() {
        output.push_str("## Bitable\n\n");
        for tool in bitable_tools {
            output.push_str(&format!(
                "### `{}`\n\n{}\n\n**Input Schema:**\n```json\n{}\n```\n\n",
                tool.name,
                tool.description,
                serde_json::to_string_pretty(&tool.input_schema).unwrap_or_default()
            ));
        }
    }

    if !message_tools.is_empty() {
        output.push_str("## Messaging\n\n");
        for tool in message_tools {
            output.push_str(&format!(
                "### `{}`\n\n{}\n\n**Input Schema:**\n```json\n{}\n```\n\n",
                tool.name,
                tool.description,
                serde_json::to_string_pretty(&tool.input_schema).unwrap_or_default()
            ));
        }
    }

    output
}

/// Generate a summary table of all tools
pub fn generate_summary() -> String {
    let tools = FeishuTool::all_tools();
    let mut output = String::from("# Feishu Tool Summary\n\n");

    output.push_str("| Tool Name | Description |\n");
    output.push_str("|-----------|-------------|\n");

    for tool in tools {
        output.push_str(&format!("| `{}` | {} |\n", tool.name, tool.description));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_markdown() {
        let docs = generate_markdown();
        assert!(!docs.is_empty());
        assert!(docs.contains("# Feishu Connector Tools"));
        assert!(docs.contains("## Calendar"));
        assert!(docs.contains("## Bitable"));
        assert!(docs.contains("## Messaging"));
    }

    #[test]
    fn test_generate_summary() {
        let summary = generate_summary();
        assert!(!summary.is_empty());
        assert!(summary.contains("# Feishu Tool Summary"));
        assert!(summary.contains("| `calendar_list`"));
    }
}
