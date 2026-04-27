//! Audit log export functionality

use crate::models::AuditEvent;
use crate::storage::AuditStorage;
use std::io::Write;
use thiserror::Error;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// JSON Lines format (one JSON object per line)
    JsonLines,
    /// Comma-separated values
    Csv,
    /// Pretty-printed JSON
    JsonPretty,
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::JsonLines
    }
}

/// Export errors
#[derive(Debug, Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Export audit events to a writer
pub async fn export_events(
    storage: &AuditStorage,
    format: ExportFormat,
    writer: impl Write + Unpin,
) -> Result<u64, ExportError> {
    let events = storage.get_all().await;
    export_events_to_writer(&events, format, writer).await
}

/// Export events directly to a writer
pub async fn export_events_to_writer(
    events: &[AuditEvent],
    format: ExportFormat,
    mut writer: impl Write + Unpin,
) -> Result<u64, ExportError> {
    let count = events.len() as u64;

    match format {
        ExportFormat::JsonLines => {
            for event in events {
                serde_json::to_writer(&mut writer, event)?;
                writer.write_all(b"\n")?;
            }
        }
        ExportFormat::JsonPretty => {
            serde_json::to_writer_pretty(&mut writer, events)?;
            writer.write_all(b"\n")?;
        }
        ExportFormat::Csv => {
            let mut wtr = csv::Writer::from_writer(writer);

            // Write header
            wtr.write_record(&[
                "id",
                "timestamp",
                "category",
                "severity",
                "platform",
                "user_id",
                "tenant_id",
                "policy_type",
                "action",
                "content_preview",
            ])?;

            for event in events {
                wtr.write_record(&[
                    &event.id.to_string(),
                    &event.timestamp.to_rfc3339(),
                    &format!("{:?}", event.category),
                    &format!("{:?}", event.severity),
                    &format!("{:?}", event.platform),
                    event.user_id.as_deref().unwrap_or(""),
                    event.tenant_id.as_deref().unwrap_or(""),
                    event
                        .policy_type
                        .as_ref()
                        .map(|p| format!("{:?}", p))
                        .unwrap_or_default()
                        .as_str(),
                    &format!("{:?}", event.action),
                    event.content_preview.as_deref().unwrap_or(""),
                ])?;
            }
            wtr.flush()?;
        }
    }

    Ok(count)
}

/// Import events from JSON Lines format
pub async fn import_json_lines<R: std::io::Read + Unpin>(
    reader: R,
) -> Result<Vec<AuditEvent>, ExportError> {
    use std::io::BufRead;

    let reader = std::io::BufReader::new(reader);
    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            let event = serde_json::from_str(&line)?;
            events.push(event);
        }
    }

    Ok(events)
}

/// Generate a compliance report
pub async fn generate_compliance_report(storage: &AuditStorage) -> String {
    let summary = storage.get_summary().await;
    let events = storage.get_all().await;

    let mut report = String::new();
    report.push_str("# OminiConnect Compliance Audit Report\n\n");

    report.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().to_rfc3339()
    ));

    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total Events: {}\n", summary.total_events));
    report.push_str(&format!(
        "- Policy Violations: {}\n",
        summary.policy_violations
    ));
    report.push_str(&format!("- PII Scrubbed: {}\n", summary.pii_scrubbed_count));

    report.push_str("\n### Events by Category\n\n");
    for (cat, count) in &summary.events_by_category {
        report.push_str(&format!("- {:?}: {}\n", cat, count));
    }

    report.push_str("\n### Events by Severity\n\n");
    for (sev, count) in &summary.events_by_severity {
        report.push_str(&format!("- {:?}: {}\n", sev, count));
    }

    report.push_str("\n## Recent Events\n\n");
    let recent = events.iter().rev().take(20);
    for event in recent {
        report.push_str(&format!(
            "- [{}] {:?} - {:?} - {:?}\n",
            event.timestamp.format("%Y-%m-%d %H:%M:%S"),
            event.severity,
            event.category,
            event.action
        ));
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AuditAction, AuditCategory, Platform, Severity};

    #[tokio::test]
    async fn test_export_json_lines() {
        let storage = AuditStorage::default();
        storage
            .log(
                AuditEvent::new(
                    AuditCategory::PolicyViolation,
                    Severity::Error,
                    Platform::Feishu,
                    AuditAction::Blocked,
                )
                .with_user_id("test_user"),
            )
            .await
            .unwrap();

        let mut output = Vec::new();
        let count = export_events(&storage, ExportFormat::JsonLines, &mut output)
            .await
            .unwrap();

        assert_eq!(count, 1);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("policy_violation"));
    }

    #[tokio::test]
    async fn test_export_csv() {
        let storage = AuditStorage::default();
        storage
            .log(
                AuditEvent::new(
                    AuditCategory::PolicyViolation,
                    Severity::Error,
                    Platform::Feishu,
                    AuditAction::Blocked,
                )
                .with_user_id("test_user"),
            )
            .await
            .unwrap();

        let mut output = Vec::new();
        let count = export_events(&storage, ExportFormat::Csv, &mut output)
            .await
            .unwrap();

        assert_eq!(count, 1);
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("id,timestamp,category"));
        assert!(output_str.contains("test_user"));
    }

    #[tokio::test]
    async fn test_import_json_lines() {
        let data = r#"{"id":"550e8400-e29b-41d4-a716-446655440000","timestamp":"2024-01-01T00:00:00Z","category":"policy_violation","severity":"error","platform":"feishu","action":"blocked","metadata":{}}
{"id":"550e8400-e29b-41d4-a716-446655440001","timestamp":"2024-01-01T00:00:01Z","category":"pii_scrubbed","severity":"warning","platform":"ding_talk","action":"redacted","metadata":{}}
"#;

        let events = import_json_lines(data.as_bytes()).await.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].category, AuditCategory::PolicyViolation);
        assert_eq!(events[1].category, AuditCategory::PiiScrubbed);
    }
}
