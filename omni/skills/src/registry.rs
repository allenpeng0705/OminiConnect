//! Skill registry for managing installed skills

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

/// Skill entry in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEntry {
    /// Skill ID
    pub id: String,
    /// Path to skill definition
    pub path: String,
    /// Whether enabled
    pub enabled: bool,
    /// Configuration overrides
    pub config: serde_json::Value,
}

/// Registry errors
#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Skill not found: {0}")]
    NotFound(String),
    #[error("Registry error: {0}")]
    Internal(String),
}

/// Skill registry for managing installed skills
#[derive(Debug, Clone, Default)]
pub struct SkillRegistry {
    /// Installed skills
    entries: HashMap<String, SkillEntry>,
    /// Registry file path
    path: Option<String>,
}

impl SkillRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Load registry from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, RegistryError> {
        let contents = std::fs::read_to_string(path.as_ref())?;
        let entries: HashMap<String, SkillEntry> = serde_json::from_str(&contents)?;
        Ok(Self {
            entries,
            path: Some(path.as_ref().to_string_lossy().into_owned()),
        })
    }

    /// Save registry to file
    pub fn save(&self) -> Result<(), RegistryError> {
        let path = self.path.as_ref()
            .ok_or_else(|| RegistryError::Internal("No path set".to_string()))?;
        let contents = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Add a skill to the registry
    pub fn add(&mut self, entry: SkillEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    /// Remove a skill from the registry
    pub fn remove(&mut self, id: &str) -> Result<(), RegistryError> {
        self.entries.remove(id)
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))?;
        Ok(())
    }

    /// Get a skill entry
    pub fn get(&self, id: &str) -> Option<&SkillEntry> {
        self.entries.get(id)
    }

    /// List all installed skills
    pub fn list(&self) -> Vec<&SkillEntry> {
        self.entries.values().collect()
    }

    /// List enabled skills
    pub fn enabled(&self) -> Vec<&SkillEntry> {
        self.entries.values().filter(|e| e.enabled).collect()
    }

    /// Enable a skill
    pub fn enable(&mut self, id: &str) -> Result<(), RegistryError> {
        let entry = self.entries.get_mut(id)
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))?;
        entry.enabled = true;
        Ok(())
    }

    /// Disable a skill
    pub fn disable(&mut self, id: &str) -> Result<(), RegistryError> {
        let entry = self.entries.get_mut(id)
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))?;
        entry.enabled = false;
        Ok(())
    }

    /// Update skill configuration
    pub fn update_config(&mut self, id: &str, config: serde_json::Value) -> Result<(), RegistryError> {
        let entry = self.entries.get_mut(id)
            .ok_or_else(|| RegistryError::NotFound(id.to_string()))?;
        entry.config = config;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_registry_operations() {
        let mut registry = SkillRegistry::new();

        // Add a skill
        registry.add(SkillEntry {
            id: "test_skill".to_string(),
            path: "/path/to/skill".to_string(),
            enabled: true,
            config: serde_json::json!({}),
        });

        assert!(registry.get("test_skill").is_some());

        // Disable
        registry.disable("test_skill").unwrap();
        assert!(!registry.get("test_skill").unwrap().enabled);

        // Enable
        registry.enable("test_skill").unwrap();
        assert!(registry.get("test_skill").unwrap().enabled);

        // Remove
        registry.remove("test_skill").unwrap();
        assert!(registry.get("test_skill").is_none());
    }

    #[test]
    fn test_registry_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("registry.json");

        let mut registry = SkillRegistry::new();
        registry.add(SkillEntry {
            id: "test_skill".to_string(),
            path: "/path/to/skill".to_string(),
            enabled: true,
            config: serde_json::json!({"key": "value"}),
        });

        // Save
        registry.path = Some(path.to_string_lossy().into_owned());
        registry.save().unwrap();

        // Load in new registry
        let loaded = SkillRegistry::load(&path).unwrap();
        assert!(loaded.get("test_skill").is_some());
        assert_eq!(loaded.get("test_skill").unwrap().id, "test_skill");
    }

    #[test]
    fn test_enabled_skills() {
        let mut registry = SkillRegistry::new();

        registry.add(SkillEntry {
            id: "skill1".to_string(),
            path: "/path/1".to_string(),
            enabled: true,
            config: serde_json::json!({}),
        });

        registry.add(SkillEntry {
            id: "skill2".to_string(),
            path: "/path/2".to_string(),
            enabled: false,
            config: serde_json::json!({}),
        });

        let enabled = registry.enabled();
        assert_eq!(enabled.len(), 1);
        assert_eq!(enabled[0].id, "skill1");
    }
}
