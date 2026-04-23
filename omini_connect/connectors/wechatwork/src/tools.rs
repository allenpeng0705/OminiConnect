//! WeChat Work tool definitions.

use serde_json::{json, Value};

/// WeChat Work tool definition
#[derive(Debug, Clone)]
pub struct WeChatWorkTool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema
    pub input_schema: Value,
}

impl WeChatWorkTool {
    /// Get all available WeChat Work tools
    pub fn all_tools() -> Vec<WeChatWorkTool> {
        vec![
            // Customer management tools
            Self {
                name: "external_contact_list".to_string(),
                description: "List external contacts (customers)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "userid": {
                            "type": "string",
                            "description": "User ID who added the customer"
                        }
                    }
                }),
            },
            Self {
                name: "external_contact_get".to_string(),
                description: "Get external contact details".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "external_userid": {
                            "type": "string",
                            "description": "External contact user ID"
                        }
                    },
                    "required": ["external_userid"]
                }),
            },
            Self {
                name: "external_contact_tag_list".to_string(),
                description: "List all tags for external contacts".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            Self {
                name: "external_contact_tag_add".to_string(),
                description: "Add a tag to external contact".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "external_userid": {
                            "type": "string",
                            "description": "External contact user ID"
                        },
                        "tag_id": {
                            "type": "string",
                            "description": "Tag ID"
                        }
                    },
                    "required": ["external_userid", "tag_id"]
                }),
            },
            // Messaging tools
            Self {
                name: "message_send".to_string(),
                description: "Send a message to a user or group".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "to_user": {
                            "type": "string",
                            "description": "Recipient user ID (multiple separated by |)"
                        },
                        "to_party": {
                            "type": "string",
                            "description": "Recipient party ID (multiple separated by |)"
                        },
                        "msg_type": {
                            "type": "string",
                            "description": "Message type: text, markdown, image, voice, video, file, textcard, news, mpnews"
                        },
                        "content": {
                            "type": "string",
                            "description": "Message content"
                        },
                        "agent_id": {
                            "type": "integer",
                            "description": "Application agent ID"
                        }
                    },
                    "required": ["msg_type", "content", "agent_id"]
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
                            "type": "string",
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
                name: "department_list".to_string(),
                description: "List departments".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "Department ID (root is 1)"
                        }
                    }
                }),
            },
            Self {
                name: "user_list_by_department".to_string(),
                description: "List users in a department".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "department_id": {
                            "type": "integer",
                            "description": "Department ID"
                        },
                        "fetch_child": {
                            "type": "boolean",
                            "description": "Fetch child departments"
                        }
                    },
                    "required": ["department_id"]
                }),
            },
        ]
    }
}
