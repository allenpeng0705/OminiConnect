//! Feishu tool definitions.

use serde_json::{json, Value};

/// Feishu tool definition
#[derive(Debug, Clone)]
pub struct FeishuTool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema
    pub input_schema: Value,
}

impl FeishuTool {
    /// Get all available Feishu tools
    pub fn all_tools() -> Vec<FeishuTool> {
        vec![
            // Calendar tools
            Self {
                name: "calendar_list".to_string(),
                description: "List all calendars for the user".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "page_size": {
                            "type": "integer",
                            "description": "Number of calendars to return (default 10)"
                        },
                        "page_token": {
                            "type": "string",
                            "description": "Token for pagination"
                        }
                    }
                }),
            },
            Self {
                name: "calendar_event_list".to_string(),
                description: "List events in a calendar".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "calendar_id": {
                            "type": "string",
                            "description": "Calendar ID"
                        },
                        "start_time": {
                            "type": "string",
                            "description": "Start time in RFC3339 format"
                        },
                        "end_time": {
                            "type": "string",
                            "description": "End time in RFC3339 format"
                        }
                    },
                    "required": ["calendar_id"]
                }),
            },
            // Bitable tools
            Self {
                name: "bitable_list".to_string(),
                description: "List all bitable apps".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Self {
                name: "bitable_table_list".to_string(),
                description: "List tables in a bitable app".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_token": {
                            "type": "string",
                            "description": "Bitable app token"
                        }
                    },
                    "required": ["app_token"]
                }),
            },
            Self {
                name: "bitable_record_list".to_string(),
                description: "List records in a bitable table".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_token": {
                            "type": "string",
                            "description": "Bitable app token"
                        },
                        "table_id": {
                            "type": "string",
                            "description": "Table ID"
                        },
                        "page_size": {
                            "type": "integer",
                            "description": "Number of records to return"
                        }
                    },
                    "required": ["app_token", "table_id"]
                }),
            },
            // Messaging tools
            Self {
                name: "message_send".to_string(),
                description: "Send a message to a chat or user".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "receive_id": {
                            "type": "string",
                            "description": "Recipient ID (user or chat ID)"
                        },
                        "receive_id_type": {
                            "type": "string",
                            "description": "Type of recipient: open_id, union_id, user_id, chat_id"
                        },
                        "msg_type": {
                            "type": "string",
                            "description": "Message type: text, post, image, file, audio, media, sticker"
                        },
                        "content": {
                            "type": "string",
                            "description": "Message content in JSON format"
                        }
                    },
                    "required": ["receive_id", "receive_id_type", "msg_type", "content"]
                }),
            },
            Self {
                name: "message_list".to_string(),
                description: "List messages in a chat".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "container_id": {
                            "type": "string",
                            "description": "Chat ID or user ID"
                        },
                        "container_id_type": {
                            "type": "string",
                            "description": "Type: p2p (user) or group (chat)"
                        },
                        "page_size": {
                            "type": "integer",
                            "description": "Number of messages to return"
                        }
                    },
                    "required": ["container_id", "container_id_type"]
                }),
            },
        ]
    }
}
