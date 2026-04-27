//! Documentation generation for DingTalk connector tools.

use crate::tools::DingTalkTool;

/// Generate markdown documentation for all DingTalk tools
pub fn generate_markdown() -> String {
    let tools = DingTalkTool::all_tools();
    let mut output = String::from("# DingTalk Connector Tools\n\n");
    output.push_str(&format!("Total tools: {}\n\n", tools.len()));

    let workflow_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.starts_with("workflow_"))
        .collect();
    let message_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.starts_with("message_"))
        .collect();
    let user_tools: Vec<_> = tools
        .iter()
        .filter(|t| t.name.starts_with("user_"))
        .collect();

    if !workflow_tools.is_empty() {
        output.push_str("## Workflow\n\n");
        for tool in workflow_tools {
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
    let tools = DingTalkTool::all_tools();
    let mut output = String::from("# DingTalk Tool Summary\n\n");

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
        assert!(docs.contains("# DingTalk Connector Tools"));
        assert!(docs.contains("## Workflow"));
        assert!(docs.contains("## Messaging"));
        assert!(docs.contains("## User"));
    }

    #[test]
    fn test_generate_summary() {
        let summary = generate_summary();
        assert!(!summary.is_empty());
        assert!(summary.contains("# DingTalk Tool Summary"));
    }
}
