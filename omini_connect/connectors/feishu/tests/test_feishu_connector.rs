//! Unit tests for Feishu connector

use omini_connect_feishu::FeishuTool;

#[test]
fn test_feishu_tools_list() {
    let tools = FeishuTool::all_tools();

    assert!(!tools.is_empty());

    // Verify we have the expected tools
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();

    assert!(tool_names.contains(&"calendar_list"));
    assert!(tool_names.contains(&"calendar_event_list"));
    assert!(tool_names.contains(&"bitable_list"));
    assert!(tool_names.contains(&"bitable_table_list"));
    assert!(tool_names.contains(&"bitable_record_list"));
    assert!(tool_names.contains(&"message_send"));
    assert!(tool_names.contains(&"message_list"));
}

#[test]
fn test_calendar_list_schema() {
    let tools = FeishuTool::all_tools();
    let calendar_tool = tools.iter().find(|t| t.name == "calendar_list").unwrap();

    // Verify input schema exists
    assert!(calendar_tool.input_schema.is_object());
}

#[test]
fn test_message_send_schema() {
    let tools = FeishuTool::all_tools();
    let message_tool = tools.iter().find(|t| t.name == "message_send").unwrap();

    // Verify required fields
    let schema = &message_tool.input_schema;
    let props = schema.get("properties").unwrap().as_object().unwrap();

    assert!(props.contains_key("receive_id"));
    assert!(props.contains_key("receive_id_type"));
    assert!(props.contains_key("msg_type"));
    assert!(props.contains_key("content"));
}
