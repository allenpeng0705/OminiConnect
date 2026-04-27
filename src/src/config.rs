//! Configuration for OminiConnect clients

use crate::error::{Result, SdkError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Platform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    /// Platform name (feishu, dingtalk, wechatwork)
    pub name: String,
    /// OAuth2 client ID
    pub client_id: String,
    /// OAuth2 client secret
    pub client_secret: String,
    /// Tenant ID (for multi-tenant setups)
    pub tenant_id: Option<String>,
}

/// OminiConnect configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server address
    pub server: String,
    /// OAuth2 vault address (if separate)
    pub vault_server: Option<String>,
    /// Platform configurations
    pub platforms: HashMap<String, PlatformConfig>,
    /// Skill configurations
    pub skills: Vec<SkillConfig>,
    /// Wasm policy paths
    pub policies: Vec<PolicyConfig>,
}

/// Skill configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    /// Skill name
    pub name: String,
    /// Enabled/disabled
    pub enabled: bool,
    /// Connector name
    pub connector: String,
    /// Tools to expose
    pub tools: Vec<String>,
    /// System prompt for this skill
    pub system_prompt: Option<String>,
}

/// Wasm policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Policy name
    pub name: String,
    /// Path to Wasm binary
    pub path: String,
    /// Policy type
    pub policy_type: String,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .map_err(|e| SdkError::Config(format!("Failed to read config: {}", e)))?;

        serde_yaml::from_str(&contents)
            .map_err(|e| SdkError::Config(format!("Failed to parse config: {}", e)))
    }

    /// Load configuration from a JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| SdkError::Config(format!("Failed to parse JSON: {}", e)))
    }

    /// Get platform config by name
    pub fn get_platform(&self, name: &str) -> Option<&PlatformConfig> {
        self.platforms.get(name)
    }

    /// Get enabled skills
    pub fn enabled_skills(&self) -> Vec<&SkillConfig> {
        self.skills.iter().filter(|s| s.enabled).collect()
    }
}

impl PlatformConfig {
    /// Create a new platform config
    pub fn new(
        name: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            tenant_id: None,
        }
    }

    /// Set tenant ID
    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_json() {
        let json = r#"{
            "server": "localhost:8080",
            "platforms": {
                "feishu": {
                    "name": "feishu",
                    "client_id": "test_id",
                    "client_secret": "test_secret"
                }
            },
            "skills": [],
            "policies": []
        }"#;

        let config = Config::from_json(json).unwrap();
        assert_eq!(config.server, "localhost:8080");
        assert!(config.platforms.contains_key("feishu"));
    }

    #[test]
    fn test_platform_config() {
        let platform = PlatformConfig::new("feishu", "id", "secret").with_tenant_id("tenant1");

        assert_eq!(platform.name, "feishu");
        assert_eq!(platform.tenant_id, Some("tenant1".to_string()));
    }
}
