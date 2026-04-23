//! DingTalk tool definitions.

use serde_json::{json, Value};

/// DingTalk tool definition
#[derive(Debug, Clone)]
pub struct DingTalkTool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema
    pub input_schema: Value,
}

impl DingTalkTool {
    /// Get all available DingTalk tools
    pub fn all_tools() -> Vec<DingTalkTool> {
        vec![
            // Workflow tools
            Self {
                name: "workflow_list".to_string(),
                description: "List approval workflows".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "process_code": {
                            "type": "string",
                            "description": "Process definition code"
                        }
                    }
                }),
            },
            Self {
                name: "workflow_instance_create".to_string(),
                description: "Create a new approval instance".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "process_code": {
                            "type": "string",
                            "description": "Process definition code"
                        },
                        "requester": {
                            "type": "string",
                            "description": "Requester user ID"
                        },
                        "approvers": {
                            "type": "array",
                            "description": "List of approver user IDs",
                            "items": { "type": "string" }
                        },
                        "data": {
                            "type": "object",
                            "description": "Form data as key-value pairs"
                        }
                    },
                    "required": ["process_code", "requester"]
                }),
            },
            Self {
                name: "workflow_instance_detail".to_string(),
                description: "Get approval instance details".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "process_instance_id": {
                            "type": "string",
                            "description": "Process instance ID"
                        }
                    },
                    "required": ["process_instance_id"]
                }),
            },
            // Messaging tools
            Self {
                name: "message_send".to_string(),
                description: "Send a message to a user or group".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "receive_id": {
                            "type": "string",
                            "description": "Recipient user or chat ID"
                        },
                        "msg_type": {
                            "type": "string",
                            "description": "Message type: text, markdown, action_card, feed_card"
                        },
                        "content": {
                            "type": "string",
                            "description": "Message content"
                        }
                    },
                    "required": ["receive_id", "msg_type", "content"]
                }),
            },
            Self {
                name: "message_list".to_string(),
                description: "List messages in a chat".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "chat_id": {
                            "type": "string",
                            "description": "Chat ID"
                        },
                        "cursor": {
                            "type": "integer",
                            "description": "Cursor for pagination"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Number of messages to return"
                        }
                    },
                    "required": ["chat_id"]
                }),
            },
            // User tools
            Self {
                name: "user_get".to_string(),
                description: "Get user information by ID".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "userid": {
                            "type": "string",
                            "description": "User ID"
                        }
                    },
                    "required": ["userid"]
                }),
            },
            Self {
                name: "user_list".to_string(),
                description: "List users in a department".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dept_id": {
                            "type": "integer",
                            "description": "Department ID"
                        }
                    },
                    "required": ["dept_id"]
                }),
            },
        ]
    }
}
