//! Audit event storage

use crate::models::{AuditCategory, AuditEvent, AuditSummary, Platform, Severity};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Storage errors
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Event not found: {0}")]
    NotFound(String),
    #[error("Storage error: {0}")]
    Internal(String),
}

/// In-memory audit storage with optional disk persistence
#[derive(Debug, Clone)]
pub struct AuditStorage {
    events: Arc<RwLock<Vec<AuditEvent>>>,
    max_events: usize,
}

impl Default for AuditStorage {
    fn default() -> Self {
        Self::new(100_000) // Default: 100k events max
    }
}

impl AuditStorage {
    /// Create new audit storage with max events capacity
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::with_capacity(max_events.min(1000)))),
            max_events,
        }
    }

    /// Log an audit event
    pub async fn log(&self, event: AuditEvent) -> Result<(), StorageError> {
        let mut events = self.events.write().await;

        // Enforce max capacity
        if events.len() >= self.max_events {
            // Remove oldest 10%
            let remove_count = (self.max_events / 10).max(1);
            events.drain(0..remove_count);
        }

        events.push(event);
        Ok(())
    }

    /// Get all events
    pub async fn get_all(&self) -> Vec<AuditEvent> {
        self.events.read().await.clone()
    }

    /// Get events by category
    pub async fn get_by_category(&self, category: AuditCategory) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.category == category)
            .cloned()
            .collect()
    }

    /// Get events by platform
    pub async fn get_by_platform(&self, platform: Platform) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.platform == platform)
            .cloned()
            .collect()
    }

    /// Get events by severity
    pub async fn get_by_severity(&self, severity: Severity) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.severity == severity)
            .cloned()
            .collect()
    }

    /// Get policy violation events
    pub async fn get_policy_violations(&self) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| {
                matches!(
                    e.category,
                    AuditCategory::PolicyViolation | AuditCategory::DataResidencyViolation
                )
            })
            .cloned()
            .collect()
    }

    /// Get PII scrubbed events
    pub async fn get_pii_scrubbed(&self) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.category == AuditCategory::PiiScrubbed)
            .cloned()
            .collect()
    }

    /// Get events for a specific tenant
    pub async fn get_by_tenant(&self, tenant_id: &str) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.tenant_id.as_deref() == Some(tenant_id))
            .cloned()
            .collect()
    }

    /// Get events within a time range
    pub async fn get_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<AuditEvent> {
        self.events
            .read()
            .await
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Get summary statistics
    pub async fn get_summary(&self) -> AuditSummary {
        let events = self.events.read().await;

        let mut events_by_category: HashMap<AuditCategory, u64> = HashMap::new();
        let mut events_by_severity: HashMap<Severity, u64> = HashMap::new();
        let mut policy_violations = 0u64;
        let mut pii_scrubbed_count = 0u64;

        for event in events.iter() {
            *events_by_category
                .entry(event.category.clone())
                .or_insert(0) += 1;
            *events_by_severity
                .entry(event.severity.clone())
                .or_insert(0) += 1;

            if matches!(
                event.category,
                AuditCategory::PolicyViolation | AuditCategory::DataResidencyViolation
            ) {
                policy_violations += 1;
            }
            if event.category == AuditCategory::PiiScrubbed {
                pii_scrubbed_count += 1;
            }
        }

        AuditSummary {
            total_events: events.len() as u64,
            events_by_category,
            events_by_severity,
            policy_violations,
            pii_scrubbed_count,
        }
    }

    /// Clear all events
    pub async fn clear(&self) {
        self.events.write().await.clear();
    }

    /// Get event count
    pub async fn len(&self) -> usize {
        self.events.read().await.len()
    }

    /// Check if empty
    pub async fn is_empty(&self) -> bool {
        self.events.read().await.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_storage() {
        let storage = AuditStorage::new(1000);

        // Log an event
        let event = AuditEvent::new(
            AuditCategory::PolicyViolation,
            Severity::Error,
            Platform::Feishu,
            crate::AuditAction::Blocked,
        )
        .with_user_id("user123")
        .with_tenant_id("tenant456")
        .with_content_preview("Sensitive content");

        storage.log(event.clone()).await.unwrap();

        assert_eq!(storage.len().await, 1);

        // Get by platform
        let feishu_events = storage.get_by_platform(Platform::Feishu).await;
        assert_eq!(feishu_events.len(), 1);

        // Get by category
        let violations = storage.get_policy_violations().await;
        assert_eq!(violations.len(), 1);

        // Summary
        let summary = storage.get_summary().await;
        assert_eq!(summary.total_events, 1);
        assert_eq!(summary.policy_violations, 1);

        // Clear
        storage.clear().await;
        assert!(storage.is_empty().await);
    }
}
