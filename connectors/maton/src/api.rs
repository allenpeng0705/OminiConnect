//! Maton.ai API client

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatonError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error: {code} - {message}")]
    Api { code: i32, message: String },

    #[error("Missing connection for app: {0}")]
    MissingConnection(String),
}

/// Connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub connection_id: String,
    pub status: String,
    pub creation_time: String,
    pub last_updated_time: String,
    pub url: Option<String>,
    pub app: String,
    pub method: Option<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// List connections response
#[derive(Debug, Deserialize)]
struct ListConnectionsResponse {
    connections: Vec<Connection>,
}

/// Get connection response
#[derive(Debug, Deserialize)]
struct GetConnectionResponse {
    connection: Connection,
}

/// Maton.ai API client
#[derive(Debug, Clone)]
pub struct MatonClient {
    api_key: String,
    http: Client,
    /// Gateway base URL for making API calls
    gateway_url: String,
    /// Control base URL for connection management
    control_url: String,
}

impl MatonClient {
    /// Create a new Maton.ai client
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            http: Client::new(),
            gateway_url: "https://gateway.maton.ai".to_string(),
            control_url: "https://ctrl.maton.ai".to_string(),
        }
    }

    /// List connections, optionally filtered by app and status
    pub async fn list_connections(
        &self,
        app: Option<&str>,
        status: Option<&str>,
    ) -> Result<Vec<Connection>, MatonError> {
        let mut url = format!("{}/connections", self.control_url);

        let mut params = vec![];
        if let Some(a) = app {
            params.push(format!("app={}", a));
        }
        if let Some(s) = status {
            params.push(format!("status={}", s));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let status_code = response.status().as_u16();
        if status_code != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MatonError::Api {
                code: status_code as i32,
                message: body,
            });
        }

        let result: ListConnectionsResponse = response.json().await?;
        Ok(result.connections)
    }

    /// Create a new connection for an app (starts OAuth flow)
    pub async fn create_connection(&self, app: &str) -> Result<Connection, MatonError> {
        let response = self
            .http
            .post(format!("{}/connections", self.control_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "app": app }))
            .send()
            .await?;

        let status_code = response.status().as_u16();
        if status_code != 200 && status_code != 201 {
            let body = response.text().await.unwrap_or_default();
            return Err(MatonError::Api {
                code: status_code as i32,
                message: body,
            });
        }

        let result: GetConnectionResponse = response.json().await?;
        Ok(result.connection)
    }

    /// Get a specific connection
    pub async fn get_connection(&self, connection_id: &str) -> Result<Connection, MatonError> {
        let response = self
            .http
            .get(format!(
                "{}/connections/{}",
                self.control_url, connection_id
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let status_code = response.status().as_u16();
        if status_code != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MatonError::Api {
                code: status_code as i32,
                message: body,
            });
        }

        let result: GetConnectionResponse = response.json().await?;
        Ok(result.connection)
    }

    /// Delete a connection
    pub async fn delete_connection(&self, connection_id: &str) -> Result<(), MatonError> {
        let response = self
            .http
            .delete(format!(
                "{}/connections/{}",
                self.control_url, connection_id
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let status_code = response.status().as_u16();
        if status_code != 200 && status_code != 204 {
            let body = response.text().await.unwrap_or_default();
            return Err(MatonError::Api {
                code: status_code as i32,
                message: body,
            });
        }

        Ok(())
    }

    /// Make a request to a connected service via the gateway
    /// app_name: e.g., "slack", "google-mail", "hubspot"
    /// path: the native API path, e.g., "/api/chat.postMessage"
    pub async fn gateway_call(
        &self,
        app_name: &str,
        method: &str,
        path: &str,
        body: Option<serde_json::Value>,
        connection_id: Option<&str>,
    ) -> Result<serde_json::Value, MatonError> {
        let url = format!("{}{}{}", self.gateway_url, app_name, path);

        let mut request = self
            .http
            .request(
                reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET),
                &url,
            )
            .header("Authorization", format!("Bearer {}", self.api_key));

        if let Some(conn_id) = connection_id {
            request = request.header("Maton-Connection", conn_id);
        }

        if let Some(b) = body {
            request = request.header("Content-Type", "application/json").json(&b);
        }

        let response = request.send().await?;

        let status_code = response.status().as_u16();
        if status_code == 400 {
            return Err(MatonError::MissingConnection(app_name.to_string()));
        }
        if status_code == 401 {
            return Err(MatonError::Api {
                code: 401,
                message: "Invalid or missing Maton API key".to_string(),
            });
        }
        if status_code == 429 {
            return Err(MatonError::Api {
                code: 429,
                message: "Rate limited (10 req/sec per account)".to_string(),
            });
        }
        if status_code == 500 {
            return Err(MatonError::Api {
                code: 500,
                message: "Maton.ai internal server error".to_string(),
            });
        }
        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(MatonError::Api {
                code: status_code as i32,
                message: body,
            });
        }

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let _client = MatonClient::new("test-key");
    }
}
