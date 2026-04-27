//! Sensitivity rules engine

use crate::config::{ConditionConfig, HybridConfig, PatternConfig, RuleAction, RuleConfig};
use glob::Pattern;
use regex::Regex;
use std::collections::HashSet;

/// Request context for rule evaluation
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    /// Content to analyze
    pub content: Option<String>,

    /// Tools being called
    pub tools: Vec<String>,

    /// Tenant ID
    pub tenant_id: Option<String>,

    /// User ID
    pub user_id: Option<String>,

    /// User groups
    pub user_groups: Vec<String>,

    /// Wasm sensitivity score (0-100)
    pub wasm_sensitivity_score: Option<u8>,

    /// PII types detected by Wasm
    pub pii_types: HashSet<PiiType>,

    /// Keyword categories detected
    pub keyword_categories: HashSet<String>,
}

/// PII types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PiiType {
    Phone,
    ChineseId,
    Email,
    Name,
    Address,
    BankAccount,
}

impl PiiType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "phone" | "chinese_phone" => Some(PiiType::Phone),
            "id" | "chinese_id" | "national_id" => Some(PiiType::ChineseId),
            "email" => Some(PiiType::Email),
            "name" => Some(PiiType::Name),
            "address" => Some(PiiType::Address),
            "bank_account" | "bankaccount" => Some(PiiType::BankAccount),
            _ => None,
        }
    }
}

/// Rule evaluation result
#[derive(Debug, Clone)]
pub struct RuleResult {
    /// Whether a rule matched
    pub matched: bool,

    /// Rule that matched
    pub rule_name: Option<String>,

    /// Action to take
    pub action: RoutingDecision,

    /// Whether to fallback on error
    pub fallback_to_local: bool,
}

/// Routing decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingDecision {
    /// Route to local LLM
    RouteToLocal,
    /// Route to specific target
    RouteToTarget(&'static str),
    /// Route to cloud (default)
    RouteToCloud,
    /// Ask Wasm policy
    AskWasmPolicy,
}

impl RuleResult {
    pub fn no_match() -> Self {
        Self {
            matched: false,
            rule_name: None,
            action: RoutingDecision::RouteToCloud,
            fallback_to_local: false,
        }
    }

    pub fn matched(action: RoutingDecision) -> Self {
        Self {
            matched: true,
            rule_name: None,
            action,
            fallback_to_local: false,
        }
    }

    pub fn with_rule_name(mut self, name: &str) -> Self {
        self.rule_name = Some(name.to_string());
        self
    }
}

/// Compiled regex pattern
#[derive(Debug, Clone)]
struct CompiledPattern {
    pattern: Regex,
}

/// Rules engine for evaluating routing decisions
#[derive(Debug, Clone)]
pub struct RulesEngine {
    rules: Vec<RuleConfig>,
    compiled_patterns: Vec<CompiledPattern>,
}

impl RulesEngine {
    /// Create a new rules engine from config
    pub fn new(config: &HybridConfig) -> Self {
        let mut engine = Self {
            rules: config.rules.clone(),
            compiled_patterns: Vec::new(),
        };
        engine.compile_patterns();
        engine
    }

    /// Compile regex patterns for efficiency
    fn compile_patterns(&mut self) {
        for rule in &self.rules {
            for condition in &rule.conditions {
                if let ConditionConfig::ContentMatches { pattern } = condition {
                    if let PatternConfig::Regex { pattern: p } = pattern {
                        if let Ok(re) = Regex::new(p) {
                            self.compiled_patterns.push(CompiledPattern { pattern: re });
                        }
                    }
                }
            }
        }
    }

    /// Evaluate rules against a request context
    pub fn evaluate(&self, ctx: &RequestContext) -> RuleResult {
        // Rules are already sorted by priority in config
        for rule in &self.rules {
            if let Some(result) = self.evaluate_rule(rule, ctx) {
                return result;
            }
        }

        RuleResult::no_match()
    }

    /// Evaluate a single rule
    fn evaluate_rule(&self, rule: &RuleConfig, ctx: &RequestContext) -> Option<RuleResult> {
        // All conditions must match (AND logic)
        for condition in &rule.conditions {
            if !self.evaluate_condition(condition, ctx) {
                return None;
            }
        }

        // Rule matched
        let decision = match &rule.action {
            RuleAction::RouteToLocal => RoutingDecision::RouteToLocal,
            RuleAction::RouteToTarget { target } => {
                RoutingDecision::RouteToTarget(Box::leak(target.clone().into_boxed_str()))
            }
            RuleAction::RouteToCloud => RoutingDecision::RouteToCloud,
            RuleAction::AskWasmPolicy => RoutingDecision::AskWasmPolicy,
        };

        Some(RuleResult::matched(decision).with_rule_name(&rule.name))
    }

    /// Evaluate a single condition
    fn evaluate_condition(&self, condition: &ConditionConfig, ctx: &RequestContext) -> bool {
        match condition {
            ConditionConfig::ContentContainsPii => !ctx.pii_types.is_empty(),

            ConditionConfig::ContentMatches { pattern } => {
                if let Some(ref content) = ctx.content {
                    self.matches_pattern(pattern, content)
                } else {
                    false
                }
            }

            ConditionConfig::ToolIs { tools } => {
                if tools.is_empty() {
                    false
                } else {
                    ctx.tools.iter().any(|t| {
                        tools.iter().any(|pattern| match glob_match(pattern, t) {
                            Ok(matched) => matched,
                            Err(_) => false,
                        })
                    })
                }
            }

            ConditionConfig::TenantIs { tenants } => {
                if let Some(ref tenant_id) = ctx.tenant_id {
                    tenants
                        .iter()
                        .any(|pattern| match glob_match(pattern, tenant_id) {
                            Ok(matched) => matched,
                            Err(_) => false,
                        })
                } else {
                    false
                }
            }

            ConditionConfig::UserInGroup { groups } => {
                if groups.is_empty() {
                    false
                } else {
                    ctx.user_groups.iter().any(|g| {
                        groups.iter().any(|pattern| match glob_match(pattern, g) {
                            Ok(matched) => matched,
                            Err(_) => false,
                        })
                    })
                }
            }

            ConditionConfig::WasmSensitivityScoreGte { score } => ctx
                .wasm_sensitivity_score
                .map(|s| s >= *score)
                .unwrap_or(false),

            ConditionConfig::Always => true,
        }
    }

    /// Check if content matches a pattern
    fn matches_pattern(&self, pattern: &PatternConfig, content: &str) -> bool {
        match pattern {
            PatternConfig::Keywords { values } => values.iter().any(|kw| content.contains(kw)),
            PatternConfig::Regex { pattern: p } => {
                // Use pre-compiled regex if available, else compile on the fly
                if let Some(found) = self
                    .compiled_patterns
                    .iter()
                    .find(|cp| &cp.pattern.to_string() == p)
                {
                    found.pattern.is_match(content)
                } else {
                    Regex::new(p)
                        .map(|re| re.is_match(content))
                        .unwrap_or(false)
                }
            }
        }
    }
}

/// Glob-style pattern matching
fn glob_match(pattern: &str, value: &str) -> Result<bool, ()> {
    // Support simple glob patterns: * matches any characters, ? matches single char
    let pattern = Pattern::new(pattern).map_err(|_| ())?;
    Ok(pattern.matches(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> HybridConfig {
        serde_json::from_str(r#"{
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
                {
                    "name": "pii_block",
                    "priority": 100,
                    "conditions": [{"type": "content_contains_pii"}],
                    "action": "route_to_local"
                },
                {
                    "name": "sensitive_tools",
                    "priority": 90,
                    "conditions": [{"type": "tool_is", "tools": ["hr_*", "finance_*"]}],
                    "action": "route_to_local"
                },
                {
                    "name": "high_score",
                    "priority": 70,
                    "conditions": [{"type": "wasm_sensitivity_score_gte", "score": 70}],
                    "action": "route_to_local"
                },
                {
                    "name": "financial_keywords",
                    "priority": 60,
                    "conditions": [{"type": "content_matches", "pattern": {"type": "keywords", "values": ["工资", "薪酬"]}}],
                    "action": "route_to_local"
                },
                {
                    "name": "default",
                    "priority": 1,
                    "conditions": [{"type": "always"}],
                    "action": "route_to_cloud"
                }
            ]
        }"#).unwrap()
    }

    #[test]
    fn test_pii_detection() {
        let config = test_config();
        let engine = RulesEngine::new(&config);

        let mut ctx = RequestContext::default();
        ctx.pii_types.insert(PiiType::ChineseId);

        let result = engine.evaluate(&ctx);
        assert!(result.matched);
        assert_eq!(result.rule_name, Some("pii_block".to_string()));
        assert!(matches!(result.action, RoutingDecision::RouteToLocal));
    }

    #[test]
    fn test_tool_matching() {
        let config = test_config();
        let engine = RulesEngine::new(&config);

        let ctx = RequestContext {
            tools: vec!["hr_get_salary".to_string()],
            ..Default::default()
        };

        let result = engine.evaluate(&ctx);
        assert!(result.matched);
        assert_eq!(result.rule_name, Some("sensitive_tools".to_string()));
    }

    #[test]
    fn test_wasm_score_matching() {
        let config = test_config();
        let engine = RulesEngine::new(&config);

        let ctx = RequestContext {
            wasm_sensitivity_score: Some(85),
            ..Default::default()
        };

        let result = engine.evaluate(&ctx);
        assert!(result.matched);
        assert_eq!(result.rule_name, Some("high_score".to_string()));
    }

    #[test]
    fn test_keyword_matching() {
        let config = test_config();
        let engine = RulesEngine::new(&config);

        let ctx = RequestContext {
            content: Some("查询员工工资信息".to_string()),
            ..Default::default()
        };

        let result = engine.evaluate(&ctx);
        assert!(result.matched);
        assert_eq!(result.rule_name, Some("financial_keywords".to_string()));
    }

    #[test]
    fn test_default_rule() {
        let config = test_config();
        let engine = RulesEngine::new(&config);

        let ctx = RequestContext::default();

        let result = engine.evaluate(&ctx);
        assert!(result.matched);
        assert_eq!(result.rule_name, Some("default".to_string()));
        assert!(matches!(result.action, RoutingDecision::RouteToCloud));
    }

    #[test]
    fn test_priority_order() {
        let config = test_config();
        let engine = RulesEngine::new(&config);

        // Request with PII and high wasm score
        let ctx = RequestContext {
            content: Some("工资信息".to_string()),
            pii_types: vec![PiiType::ChineseId].into_iter().collect(),
            wasm_sensitivity_score: Some(90),
            ..Default::default()
        };

        // PII rule should win (priority 100 > 90 > 70 > 60)
        let result = engine.evaluate(&ctx);
        assert_eq!(result.rule_name, Some("pii_block".to_string()));
    }
}
