use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ToolTelemetry {
    pub tool: String,
    pub attempts: u64,
    pub executions: u64,
    pub missing_required_failures: u64,
    pub schema_validation_failures: u64,
    pub retries: u64,
    pub unknown_param_ignored: u64,
    pub coercions: u64,
    pub missing_fields: HashMap<String, u64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LlmToolTelemetry {
    pub total_attempts: u64,
    pub total_executions: u64,
    pub total_missing_required_failures: u64,
    pub total_schema_validation_failures: u64,
    pub total_retries: u64,
    pub attempts_with_provider_context: u64,
    pub attempts_without_provider_context: u64,
    pub executions_with_provider_context: u64,
    pub executions_without_provider_context: u64,
    pub missing_required_with_provider_context: u64,
    pub missing_required_without_provider_context: u64,
    pub schema_failures_with_provider_context: u64,
    pub schema_failures_without_provider_context: u64,
    pub by_tool: HashMap<String, ToolTelemetry>,
}

impl LlmToolTelemetry {
    fn ensure_tool(&mut self, tool: &str) -> &mut ToolTelemetry {
        self.by_tool
            .entry(tool.to_string())
            .or_insert_with(|| ToolTelemetry {
                tool: tool.to_string(),
                ..Default::default()
            })
    }

    pub fn record_attempt(&mut self, tool: &str, provider_context_enabled: bool) {
        self.total_attempts += 1;
        if provider_context_enabled {
            self.attempts_with_provider_context += 1;
        } else {
            self.attempts_without_provider_context += 1;
        }
        self.ensure_tool(tool).attempts += 1;
    }

    pub fn record_execution(&mut self, tool: &str, provider_context_enabled: bool) {
        self.total_executions += 1;
        if provider_context_enabled {
            self.executions_with_provider_context += 1;
        } else {
            self.executions_without_provider_context += 1;
        }
        self.ensure_tool(tool).executions += 1;
    }

    pub fn record_missing_required(
        &mut self,
        tool: &str,
        missing_fields: &[String],
        retried: bool,
        provider_context_enabled: bool,
    ) {
        self.total_missing_required_failures += 1;
        if provider_context_enabled {
            self.missing_required_with_provider_context += 1;
        } else {
            self.missing_required_without_provider_context += 1;
        }
        if retried {
            self.total_retries += 1;
        }
        let item = self.ensure_tool(tool);
        item.missing_required_failures += 1;
        if retried {
            item.retries += 1;
        }
        for field in missing_fields {
            *item.missing_fields.entry(field.clone()).or_insert(0) += 1;
        }
    }

    pub fn record_schema_failure(&mut self, tool: &str, retried: bool, provider_context_enabled: bool) {
        self.total_schema_validation_failures += 1;
        if provider_context_enabled {
            self.schema_failures_with_provider_context += 1;
        } else {
            self.schema_failures_without_provider_context += 1;
        }
        if retried {
            self.total_retries += 1;
        }
        let item = self.ensure_tool(tool);
        item.schema_validation_failures += 1;
        if retried {
            item.retries += 1;
        }
    }

    pub fn record_normalization_warnings(&mut self, tool: &str, warnings: &[String]) {
        let item = self.ensure_tool(tool);
        for warning in warnings {
            if warning.starts_with("ignored_unknown_parameter") {
                item.unknown_param_ignored += 1;
            }
            if warning.starts_with("coerced_to_") || warning.starts_with("wrapped_in_array") {
                item.coercions += 1;
            }
        }
    }
}
