//! Skill definition and packaging
//!
//! A Skill packages together:
//! - One or more connectors
//! - Specific tools to expose
//! - System prompts
//! - Wasm policies

use crate::error::{Result, SdkError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Skill name (unique identifier)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Connector(s) required
    pub connectors: Vec<ConnectorRef>,
    /// Tools to expose
    pub tools: Vec<String>,
    /// System prompt for this skill
    pub system_prompt: Option<String>,
    /// Wasm policies to apply
    pub policies: Vec<String>,
    /// Metadata
    pub metadata: SkillMetadata,
}

/// Reference to a connector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorRef {
    /// Connector name (e.g., "feishu", "dingtalk")
    pub name: String,
    /// Required tools from this connector
    pub tools: Vec<String>,
}

/// Skill metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Version
    pub version: String,
    /// Author
    pub author: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Whether this is a premium skill
    pub premium: bool,
}

impl Skill {
    /// Create a new skill builder
    pub fn builder(name: impl Into<String>) -> SkillBuilder {
        SkillBuilder::new(name)
    }

    /// Load a skill from a YAML file
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| SdkError::Skill(format!("Failed to read skill file: {}", e)))?;
        Self::from_yaml(&contents)
    }

    /// Load a skill from YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml)
            .map_err(|e| SdkError::Skill(format!("Failed to parse skill YAML: {}", e)))
    }

    /// Load a skill from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| SdkError::Skill(format!("Failed to parse skill JSON: {}", e)))
    }

    /// Validate the skill definition
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(SdkError::Skill("Skill name cannot be empty".to_string()));
        }

        if self.connectors.is_empty() {
            return Err(SdkError::Skill(
                "At least one connector is required".to_string(),
            ));
        }

        if self.tools.is_empty() {
            return Err(SdkError::Skill("At least one tool is required".to_string()));
        }

        // Check for duplicate tools
        let mut seen = HashSet::new();
        for tool in &self.tools {
            if !seen.insert(tool) {
                return Err(SdkError::Skill(format!("Duplicate tool: {}", tool)));
            }
        }

        Ok(())
    }

    /// Get all tool names (fully qualified)
    pub fn all_tools(&self) -> Vec<String> {
        self.connectors
            .iter()
            .flat_map(|c| c.tools.iter().map(|t| format!("{}_{}", c.name, t)))
            .collect()
    }
}

/// Builder for creating skills
#[derive(Debug, Clone)]
pub struct SkillBuilder {
    name: String,
    description: String,
    connectors: Vec<ConnectorRef>,
    tools: Vec<String>,
    system_prompt: Option<String>,
    policies: Vec<String>,
    metadata: SkillMetadata,
}

impl SkillBuilder {
    /// Create a new skill builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            connectors: Vec::new(),
            tools: Vec::new(),
            system_prompt: None,
            policies: Vec::new(),
            metadata: SkillMetadata {
                version: "1.0.0".to_string(),
                author: None,
                tags: Vec::new(),
                premium: false,
            },
        }
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a connector
    pub fn add_connector(self, name: impl Into<String>) -> ConnectorBuilder {
        ConnectorBuilder {
            skill: self,
            connector_name: name.into(),
            connector_tools: Vec::new(),
        }
    }

    /// Set system prompt
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Add a policy
    pub fn add_policy(mut self, policy: impl Into<String>) -> Self {
        self.policies.push(policy.into());
        self
    }

    /// Set version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.metadata.version = version.into();
        self
    }

    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.metadata.author = Some(author.into());
        self
    }

    /// Add a tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.metadata.tags.push(tag.into());
        self
    }

    /// Mark as premium
    pub fn premium(mut self) -> Self {
        self.metadata.premium = true;
        self
    }

    /// Build the skill
    pub fn build(self) -> Result<Skill> {
        let skill = Skill {
            name: self.name,
            description: self.description,
            connectors: self.connectors,
            tools: self.tools,
            system_prompt: self.system_prompt,
            policies: self.policies,
            metadata: self.metadata,
        };
        skill.validate()?;
        Ok(skill)
    }
}

/// Builder for adding a connector to a skill
#[derive(Debug, Clone)]
pub struct ConnectorBuilder {
    skill: SkillBuilder,
    connector_name: String,
    connector_tools: Vec<String>,
}

impl ConnectorBuilder {
    /// Add a tool to this connector
    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.connector_tools.push(tool.into());
        self
    }

    /// Add multiple tools
    pub fn with_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for tool in tools {
            self.connector_tools.push(tool.into());
        }
        self
    }

    /// Finish adding this connector
    pub fn done(mut self) -> SkillBuilder {
        self.skill.connectors.push(ConnectorRef {
            name: self.connector_name,
            tools: self.connector_tools.clone(),
        });
        // Also add tools to the skill's tool list
        self.skill.tools.extend(self.connector_tools);
        self.skill
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_skill() {
        let skill = Skill::builder("feishu_pm")
            .description("Project management with Feishu")
            .add_connector("feishu")
            .with_tool("calendar_list")
            .with_tool("calendar_create")
            .done()
            .add_policy("content_moderation")
            .build()
            .unwrap();

        assert_eq!(skill.name, "feishu_pm");
        assert_eq!(skill.connectors.len(), 1);
        assert_eq!(skill.tools.len(), 2);
        assert_eq!(skill.policies.len(), 1);
    }

    #[test]
    fn test_skill_validation() {
        // Empty name should fail
        let result = Skill::builder("")
            .add_connector("feishu")
            .with_tool("calendar_list")
            .done()
            .build();
        assert!(result.is_err());

        // Empty connectors should fail
        let result = Skill::builder("test").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_skill_tools() {
        let skill = Skill::builder("wechat_service")
            .add_connector("wechatwork")
            .with_tool("external_contact_list")
            .with_tool("message_send")
            .done()
            .build()
            .unwrap();

        let tools = skill.all_tools();
        assert!(tools.contains(&"wechatwork_external_contact_list".to_string()));
        assert!(tools.contains(&"wechatwork_message_send".to_string()));
    }

    #[test]
    fn test_skill_yaml() {
        let yaml = r#"
name: test_skill
description: A test skill
connectors:
  - name: feishu
    tools:
      - calendar_list
tools:
  - calendar_list
policies: []
metadata:
  version: 1.0.0
  tags:
    - calendar
    - feishu
  premium: false
"#;

        let skill = Skill::from_yaml(yaml).unwrap();
        assert_eq!(skill.name, "test_skill");
        assert!(skill.metadata.tags.contains(&"calendar".to_string()));
    }
}
