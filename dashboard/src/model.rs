//! Dashboard data models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Audit event from the audit logger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub category: String,
    pub severity: String,
    pub platform: String,
    pub user_id: Option<String>,
    pub tenant_id: Option<String>,
    pub policy_type: Option<String>,
    pub action: String,
    pub content_preview: Option<String>,
}

/// Dashboard statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_events: u64,
    pub policy_violations: u64,
    pub pii_scrubbed: u64,
    pub events_today: u64,
    pub events_by_severity: SeverityCounts,
    pub events_by_platform: PlatformCounts,
}

/// Severity breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityCounts {
    pub info: u64,
    pub warning: u64,
    pub error: u64,
    pub critical: u64,
}

/// Platform breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCounts {
    pub feishu: u64,
    pub dingtalk: u64,
    pub wechatwork: u64,
}

/// Recent event for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentEvent {
    pub id: String,
    pub time_ago: String,
    pub severity: String,
    pub category: String,
    pub platform: String,
    pub action: String,
    pub preview: String,
}

/// Policy violation for detailed view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub platform: String,
    pub user_id: Option<String>,
    pub policy_type: String,
    pub action: String,
    pub content_preview: Option<String>,
    pub details: serde_json::Value,
}
