//! Configuration types for hybrid inference

use serde::{Deserialize, Serialize};

/// Hybrid inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridConfig {
    /// Enable hybrid inference
    pub enabled: bool,

    /// Local LLM settings
    pub local_llm: LocalLlmConfig,

    /// Routing targets (cloud LLM endpoints)
    pub targets: Vec<TargetConfig>,

    /// Cloud fallback configuration
    pub cloud_fallback: CloudFallbackConfig,

    /// Routing rules (evaluated in priority order)
    pub rules: Vec<RuleConfig>,

    /// Observability settings
    #[serde(default)]
    pub observability: ObservabilityConfig,
}

impl Default for HybridConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            local_llm: LocalLlmConfig::default(),
            targets: Vec::new(),
            cloud_fallback: CloudFallbackConfig::default(),
            rules: Vec::new(),
            observability: ObservabilityConfig::default(),
        }
    }
}

/// Local LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalLlmConfig {
    /// Enable local LLM
    pub enabled: bool,

    /// Endpoint (Ollama/vLLM)
    pub endpoint: String,

    /// Model name
    pub model: String,

    /// Request timeout in milliseconds
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

impl Default for LocalLlmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://localhost:11434/v1/chat/completions".to_string(),
            model: "qwen2.5:7b-instruct".to_string(),
            timeout_ms: 30000,
        }
    }
}

fn default_timeout_ms() -> u64 {
    30000
}

/// Routing target (cloud LLM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Unique target name
    pub name: String,

    /// Provider type
    pub provider: ProviderType,

    /// Upstream endpoint
    pub upstream: String,

    /// Environment variable name for API key
    pub api_key_name: String,
}

/// Provider type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    /// OpenAI Chat Completions
    #[serde(rename = "openai")]
    OpenAI,
    /// Anthropic Messages API
    #[serde(rename = "anthropic")]
    Anthropic,
    /// OpenAI-compatible (DeepSeek, Qwen, etc.)
    #[serde(rename = "openai_compatible")]
    OpenAICompatible,
    /// Ollama (local)
    #[serde(rename = "ollama")]
    Ollama,
}

impl Default for ProviderType {
    fn default() -> Self {
        ProviderType::OpenAI
    }
}

/// Cloud fallback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFallbackConfig {
    /// Enable fallback
    pub enabled: bool,

    /// Target name to use for fallback
    pub target: String,
}

impl Default for CloudFallbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            target: "general".to_string(),
        }
    }
}

/// Routing rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Rule name
    pub name: String,

    /// Priority (higher = evaluated first)
    pub priority: u8,

    /// Conditions that must match
    pub conditions: Vec<ConditionConfig>,

    /// Action to take
    #[serde(flatten)]
    pub action: RuleAction,
}

/// Condition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConditionConfig {
    /// Content contains PII
    ContentContainsPii,

    /// Content matches pattern
    ContentMatches { pattern: PatternConfig },

    /// Request uses specific tool(s)
    ToolIs { tools: Vec<String> },

    /// Request from specific tenant(s)
    TenantIs { tenants: Vec<String> },

    /// User in group(s)
    UserInGroup { groups: Vec<String> },

    /// Wasm sensitivity score >= threshold
    WasmSensitivityScoreGte { score: u8 },

    /// Always match
    Always,
}

/// Pattern for content matching
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PatternConfig {
    /// Simple keyword list
    Keywords { values: Vec<String> },

    /// Regex pattern
    Regex { pattern: String },
}

/// Rule action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum RuleAction {
    /// Route to local LLM
    RouteToLocal,

    /// Route to specific target
    RouteToTarget { target: String },

    /// Route to cloud
    RouteToCloud,

    /// Use Wasm policy decision
    AskWasmPolicy,
}

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Log routing decisions
    #[serde(default)]
    pub log_routing_decisions: bool,

    /// Enable metrics
    #[serde(default)]
    pub metrics_enabled: bool,

    /// Audit log path
    #[serde(default)]
    pub audit_log_path: Option<String>,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            log_routing_decisions: true,
            metrics_enabled: true,
            audit_log_path: None,
        }
    }
}

impl HybridConfig {
    /// Load configuration from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Load configuration from YAML
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// Get target by name
    pub fn get_target(&self, name: &str) -> Option<&TargetConfig> {
        self.targets.iter().find(|t| t.name == name)
    }

    /// Get sorted rules by priority (highest first)
    pub fn sorted_rules(&self) -> Vec<&RuleConfig> {
        let mut rules: Vec<&RuleConfig> = self.rules.iter().collect();
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = HybridConfig::default();
        assert!(!config.enabled);
        assert!(config.local_llm.enabled);
        assert_eq!(config.local_llm.model, "qwen2.5:7b-instruct");
    }

    #[test]
    fn test_sorted_rules() {
        let config: HybridConfig = serde_json::from_str(r#"{
            "enabled": true,
            "local_llm": {
                "enabled": true,
                "endpoint": "http://localhost:11434",
                "model": "test",
                "timeout_ms": 30000
            },
            "targets": [],
            "cloud_fallback": {
                "enabled": true,
                "target": "general"
            },
            "rules": [
                {"name": "low", "priority": 1, "conditions": [{"type": "always"}], "action": "route_to_local"},
                {"name": "high", "priority": 100, "conditions": [{"type": "always"}], "action": "route_to_local"},
                {"name": "medium", "priority": 50, "conditions": [{"type": "always"}], "action": "route_to_local"}
            ]
        }"#).unwrap();

        let sorted = config.sorted_rules();
        assert_eq!(sorted[0].name, "high");
        assert_eq!(sorted[1].name, "medium");
        assert_eq!(sorted[2].name, "low");
    }
}
