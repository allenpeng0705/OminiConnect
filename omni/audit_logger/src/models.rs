//! Audit event models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Category of audit event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AuditCategory {
    /// Policy violation (content moderation, keyword block, etc.)
    PolicyViolation,
    /// Data residency/PIPL violation
    DataResidencyViolation,
    /// PII detected and scrubbed
    PiiScrubbed,
    /// Tool invocation
    ToolInvocation,
    /// Token access (OAuth)
    TokenAccess,
    /// Wasm policy action
    PolicyAction,
}

/// Severity level of the audit event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Platform that generated the event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Feishu,
    DingTalk,
    WeChatWork,
    Unknown,
}

/// Policy type that was violated or applied
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyType {
    ContentModeration,
    KeywordFilter,
    DataResidency,
    PiiScrub,
}

/// Core audit event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event ID
    pub id: Uuid,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Event category
    pub category: AuditCategory,
    /// Severity level
    pub severity: Severity,
    /// Platform that generated the event
    pub platform: Platform,
    /// User or tenant ID
    pub user_id: Option<String>,
    /// Tenant ID for multi-tenant tracking
    pub tenant_id: Option<String>,
    /// Policy type (if applicable)
    pub policy_type: Option<PolicyType>,
    /// What was blocked/allowed/redacted
    pub action: AuditAction,
    /// Raw content that triggered the event (may be scrubbed)
    pub content_preview: Option<String>,
    /// Additional context/metadata
    pub metadata: serde_json::Value,
}

/// Action taken by policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    /// Request allowed through
    Allowed,
    /// Request blocked
    Blocked,
    /// Content redacted/scrubbed
    Redacted,
    /// Content modified
    Modified,
    /// Warning issued
    Warning,
}

/// Summary of audit events for a time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_events: u64,
    pub events_by_category: std::collections::HashMap<AuditCategory, u64>,
    pub events_by_severity: std::collections::HashMap<Severity, u64>,
    pub policy_violations: u64,
    pub pii_scrubbed_count: u64,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        category: AuditCategory,
        severity: Severity,
        platform: Platform,
        action: AuditAction,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            category,
            severity,
            platform,
            user_id: None,
            tenant_id: None,
            policy_type: None,
            action,
            content_preview: None,
            metadata: serde_json::json!({}),
        }
    }

    /// Set user ID
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Set tenant ID
    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Set policy type
    pub fn with_policy_type(mut self, policy_type: PolicyType) -> Self {
        self.policy_type = Some(policy_type);
        self
    }

    /// Set content preview (will be truncated to 500 chars)
    pub fn with_content_preview(mut self, content: impl Into<String>) -> Self {
        let preview = content.into();
        self.content_preview = Some(
            if preview.len() > 500 {
                format!("{}...", &preview[..500])
            } else {
                preview
            }
        );
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Ok(val) = serde_json::to_value(value) {
            self.metadata[key.into()] = val;
        }
        self
    }
}
