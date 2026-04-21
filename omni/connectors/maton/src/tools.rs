//! Maton.ai MCP tools

use serde::{Deserialize, Serialize};

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatonTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// List all connections
pub fn maton_list_connections_tool() -> MatonTool {
    MatonTool {
        name: "maton_list_connections".to_string(),
        description: "List all API connections managed by Maton.ai. Shows which services (Slack, Gmail, Google Calendar, HubSpot, etc.) are connected.".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "app": {
                    "type": "string",
                    "description": "Filter by app name (e.g., 'slack', 'google-mail', 'hubspot')"
                },
                "status": {
                    "type": "string",
                    "enum": ["ACTIVE", "PENDING", "FAILED"],
                    "description": "Filter by connection status"
                }
            },
            "required": []
        }),
    }
}

/// Create a new connection (starts OAuth flow)
pub fn maton_create_connection_tool() -> MatonTool {
    MatonTool {
        name: "maton_create_connection".to_string(),
        description: "Create a new connection to a service via Maton.ai OAuth flow. Returns a URL to complete authorization.".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "app": {
                    "type": "string",
                    "description": "App name (e.g., 'slack', 'google-mail', 'hubspot', 'notion', 'salesforce')"
                }
            },
            "required": ["app"]
        }),
    }
}

/// Make a direct API call to a connected service
pub fn maton_gateway_call_tool() -> MatonTool {
    MatonTool {
        name: "maton_gateway_call".to_string(),
        description: "Make a direct API call to any connected service through Maton.ai gateway. First call maton_list_connections to see which services (Slack, Gmail, LinkedIn, HubSpot, etc.) are connected. Then use this tool with the app name, HTTP method, and the service's native API path. Examples: app='slack', method='POST', path='/api/chat.postMessage', body={'channel': 'C0123', 'text': 'Hello!'}; app='google-mail', method='GET', path='/gmail/v1/users/me/messages?maxResults=10'; app='hubspot', method='POST', path='/crm/v3/objects/contacts', body={'properties': {'email': 'test@example.com'}}".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "app": {
                    "type": "string",
                    "description": "App name (e.g., 'slack', 'google-mail', 'hubspot', 'airtable', 'notion')"
                },
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "PATCH", "DELETE"],
                    "description": "HTTP method"
                },
                "path": {
                    "type": "string",
                    "description": "Native API path (e.g., '/api/chat.postMessage', '/gmail/v1/users/me/messages')"
                },
                "body": {
                    "type": "object",
                    "description": "Request body (for POST/PUT/PATCH)",
                    "additionalProperties": true
                }
            },
            "required": ["app", "method", "path"]
        }),
    }
}

/// Get all available tools
pub fn all_tools() -> Vec<MatonTool> {
    vec![
        maton_list_connections_tool(),
        maton_create_connection_tool(),
        maton_gateway_call_tool(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tools() {
        let tools = all_tools();
        assert_eq!(tools.len(), 3);
        assert_eq!(tools[0].name, "maton_list_connections");
        assert_eq!(tools[1].name, "maton_create_connection");
        assert_eq!(tools[2].name, "maton_gateway_call");
    }
}
