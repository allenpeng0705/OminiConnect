//! Documentation generation for WeChat Work connector tools.

use crate::tools::WeChatWorkTool;

/// Generate markdown documentation for all WeChat Work tools
pub fn generate_markdown() -> String {
    let tools = WeChatWorkTool::all_tools();
    let mut output = String::from("# WeChat Work Connector Tools\n\n");
    output.push_str(&format!("Total tools: {}\n\n", tools.len()));

    let customer_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.starts_with("external_contact_"))
        .collect();
    let message_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.starts_with("message_"))
        .collect();
    let user_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.starts_with("user_") || t.name.starts_with("department_"))
        .collect();

    if !customer_tools.is_empty() {
        output.push_str("## Customer\n\n");
        for tool in customer_tools {
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

    if !user_tools.is_empty() {
        output.push_str("## User\n\n");
        for tool in user_tools {
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
    let tools = WeChatWorkTool::all_tools();
    let mut output = String::from("# WeChat Work Tool Summary\n\n");

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
        assert!(docs.contains("# WeChat Work Connector Tools"));
        assert!(docs.contains("## Customer"));
        assert!(docs.contains("## Messaging"));
        assert!(docs.contains("## User"));
    }

    #[test]
    fn test_generate_summary() {
        let summary = generate_summary();
        assert!(!summary.is_empty());
        assert!(summary.contains("# WeChat Work Tool Summary"));
    }
}
