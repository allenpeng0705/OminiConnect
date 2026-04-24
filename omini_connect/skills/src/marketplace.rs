//! Skill marketplace for discovering and managing skills

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Marketplace category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillCategory {
    ProjectManagement,
    CustomerService,
    Workflow,
    Communication,
    Analytics,
    Custom,
}

/// Skill listing in the marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillListing {
    /// Unique skill identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: String,
    /// Category
    pub category: SkillCategory,
    /// Supported platforms
    pub platforms: Vec<String>,
    /// Skill version
    pub version: String,
    /// Whether premium/paid
    pub premium: bool,
    /// Icon/name for UI
    pub icon: Option<String>,
}

/// Marketplace for discovering skills
#[derive(Debug, Clone)]
pub struct Marketplace {
    /// Available skills
    skills: HashMap<String, SkillListing>,
}

impl Marketplace {
    /// Create a new marketplace with built-in skills
    pub fn new() -> Self {
        let mut marketplace = Self {
            skills: HashMap::new(),
        };
        marketplace.register_builtin_skills();
        marketplace
    }

    /// Register a custom skill
    pub fn register(&mut self, listing: SkillListing) {
        self.skills.insert(listing.id.clone(), listing);
    }

    /// List all available skills
    pub fn list_skills(&self) -> Vec<&SkillListing> {
        self.skills.values().collect()
    }

    /// Get a skill by ID
    pub fn get(&self, id: &str) -> Option<&SkillListing> {
        self.skills.get(id)
    }

    /// Search skills by category
    pub fn by_category(&self, category: SkillCategory) -> Vec<&SkillListing> {
        self.skills
            .values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Search skills by platform
    pub fn by_platform(&self, platform: &str) -> Vec<&SkillListing> {
        self.skills
            .values()
            .filter(|s| s.platforms.contains(&platform.to_string()))
            .collect()
    }

    /// Search skills by query (name/description)
    pub fn search(&self, query: &str) -> Vec<&SkillListing> {
        let query_lower = query.to_lowercase();
        self.skills
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Register built-in enterprise skills
    fn register_builtin_skills(&mut self) {
        // Feishu Project Manager Skill
        self.register(SkillListing {
            id: "feishu_project_manager".to_string(),
            name: "Feishu Project Manager".to_string(),
            description: "Complete project management with Feishu. Includes calendar, tasks, and approval workflows.".to_string(),
            category: SkillCategory::ProjectManagement,
            platforms: vec!["feishu".to_string()],
            version: "1.0.0".to_string(),
            premium: false,
            icon: Some("calendar".to_string()),
        });

        // WeChat Customer Service Skill
        self.register(SkillListing {
            id: "wechat_customer_service".to_string(),
            name: "WeChat Customer Service".to_string(),
            description: "Customer relationship management and messaging for WeChat Work. PII scrubbing included.".to_string(),
            category: SkillCategory::CustomerService,
            platforms: vec!["wechatwork".to_string()],
            version: "1.0.0".to_string(),
            premium: false,
            icon: Some("users".to_string()),
        });

        // DingTalk Workflow Skill
        self.register(SkillListing {
            id: "dingtalk_workflow".to_string(),
            name: "DingTalk Workflow Automation".to_string(),
            description: "Automate approval workflows and notifications with DingTalk.".to_string(),
            category: SkillCategory::Workflow,
            platforms: vec!["dingtalk".to_string()],
            version: "1.0.0".to_string(),
            premium: false,
            icon: Some("git-pull-request".to_string()),
        });

        // Enterprise Communication Skill
        self.register(SkillListing {
            id: "enterprise_communication".to_string(),
            name: "Enterprise Communication Suite".to_string(),
            description: "Multi-platform messaging across Feishu, DingTalk, and WeChat Work with content moderation.".to_string(),
            category: SkillCategory::Communication,
            platforms: vec![
                "feishu".to_string(),
                "dingtalk".to_string(),
                "wechatwork".to_string(),
            ],
            version: "1.0.0".to_string(),
            premium: true,
            icon: Some("message-circle".to_string()),
        });
    }
}

impl Default for Marketplace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_list() {
        let marketplace = Marketplace::new();
        let skills = marketplace.list_skills();
        assert!(!skills.is_empty());
    }

    #[test]
    fn test_get_skill() {
        let marketplace = Marketplace::new();
        let skill = marketplace.get("feishu_project_manager");
        assert!(skill.is_some());
        assert_eq!(skill.unwrap().name, "Feishu Project Manager");
    }

    #[test]
    fn test_by_category() {
        let marketplace = Marketplace::new();
        let pm_skills = marketplace.by_category(SkillCategory::ProjectManagement);
        assert!(!pm_skills.is_empty());
    }

    #[test]
    fn test_search() {
        let marketplace = Marketplace::new();
        let results = marketplace.search("customer");
        assert!(!results.is_empty());
        assert!(results.iter().any(|s| s.id == "wechat_customer_service"));
    }

    #[test]
    fn test_by_platform() {
        let marketplace = Marketplace::new();
        let feishu_skills = marketplace.by_platform("feishu");
        assert!(!feishu_skills.is_empty());
    }
}
