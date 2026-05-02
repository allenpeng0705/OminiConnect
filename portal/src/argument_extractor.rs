//! Schema-guided argument extraction for natural language tool calls.
//!
//! Extracts parameters from NL queries using a tiered approach:
//! 1. Enum values and literal parameter name matches
//! 2. Schema-guided entity scoring against parameter descriptions
//! 3. Post-processing (defaults, required params, confidence)

use crate::tools::Tool;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Threshold for entity-param assignment (must beat this to be considered).
const SCORE_THRESHOLD: f64 = 0.1;

/// Ownership-related keywords in parameter descriptions.
const OWNERSHIP_KEYWORDS: &[&str] = &["owner", "organization", "user", "account", "creator", "author"];

/// Stop words that shouldn't be considered as entity candidates.
const STOP_WORDS: &[&str] = &[
    "a", "an", "the", "my", "me", "i", "in", "on", "at", "to", "for",
    "of", "with", "by", "from", "is", "are", "was", "were", "be", "been",
    "being", "have", "has", "had", "do", "does", "did", "will", "would",
    "could", "should", "may", "might", "must", "shall", "can", "need",
    "list", "show", "get", "find", "search", "create", "update", "delete",
    "remove", "add", "edit", "view", "see", "display", "fetch", "retrieve",
];

// ============================================================================
// Public API
// ============================================================================

pub struct ExtractResult {
    /// Extracted arguments as JSON object.
    pub arguments: Value,
    /// Confidence score 0.0-1.0.
    pub confidence: f64,
    /// Required parameters that were not extracted.
    pub missing_required: Vec<String>,
}

impl ExtractResult {
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.7 && self.missing_required.is_empty()
    }
}

/// Extract arguments from a natural language query for a given tool.
pub fn extract_arguments(query: &str, tool: &Tool) -> ExtractResult {
    let query_lower = query.to_lowercase();

    // Phase 1: Extract enum values and literal param matches
    let mut args = extract_enum_and_literal_params(&query_lower, tool);

    // Phase 2: Extract entity mentions using schema-guided matching
    // Use original query for entity extraction to preserve case (Strategy 2/3 need original case)
    let entity_args = extract_entity_arguments(query, tool, &args);
    for (k, v) in entity_args {
        args.entry(k).or_insert(v);
    }

    // Phase 3: Post-processing
    post_process(args, tool)
}

// ============================================================================
// Tier 1: Enum and Literal Parameter Extraction
// ============================================================================

fn extract_enum_and_literal_params(query: &str, tool: &Tool) -> HashMap<String, Value> {
    let mut args = HashMap::new();

    for (param_name, param_spec) in &tool.input_schema.properties {
        let param_lower = param_name.to_lowercase();

        // Check for enum values first (highest confidence)
        if let Some(enum_values) = param_spec.get("enum").and_then(|e| e.as_array()) {
            for enum_val in enum_values {
                if let Some(s) = enum_val.as_str() {
                    if query.contains(&s.to_lowercase()) {
                        args.insert(param_name.clone(), Value::String(s.to_string()));
                        break;
                    }
                }
            }
        }
        // Check if parameter name appears in query
        else if query.contains(&param_lower) {
            if let Some(value) = extract_value_after_word(query, &param_lower) {
                args.insert(param_name.clone(), Value::String(value));
            }
        }
    }

    args
}

fn extract_value_after_word(query: &str, word: &str) -> Option<String> {
    let patterns = &[format!("{} ", word), format!(" {} ", word), format!("{}'s ", word)];

    for pat in patterns {
        if let Some(idx) = query.find(pat) {
            let after = &query[idx + pat.len()..];
            let end = after.find(' ').unwrap_or(after.len()).min(50);
            let val = after[..end]
                .trim_end_matches(|c: char| c.is_ascii_punctuation() || c == '\n' || c == '\'');
            if !val.is_empty() && !STOP_WORDS.contains(&val) {
                return Some(val.to_string());
            }
        }
    }
    None
}

// ============================================================================
// Tier 2: Schema-Guided Entity Extraction
// ============================================================================

#[derive(Clone, Debug)]
struct EntityCandidate {
    token: String,
    normalized: String,
    position: usize,
    is_possessive: bool,
}

/// Extract potential entity tokens from the query.
/// Looks for capitalized words, quoted strings, and slash-separated values.
fn extract_entity_candidates(query: &str) -> Vec<EntityCandidate> {
    let mut candidates = Vec::new();

    // Strategy 1: Handle slash-separated values: "openai/gpt-4", "owner/repo-name"
    if let Some(slash_pos) = query.find('/') {
        if slash_pos > 0 && slash_pos < query.len() - 1 {
            let before = query[..slash_pos].trim();
            let after_raw = &query[slash_pos + 1..];
            // Take the last word before slash as entity
            let before_entity = before.split_whitespace().last().unwrap_or(before).trim();
            // Take first word after slash
            let after_entity = after_raw
                .split(|c: char| c.is_whitespace() || c == ',' || c == '.' || c == '!' || c == '?')
                .next()
                .unwrap_or(after_raw.trim())
                .trim();

            if before_entity.len() >= 2
                && !STOP_WORDS.contains(&before_entity.to_lowercase().as_str())
            {
                candidates.push(EntityCandidate {
                    token: before_entity.to_string(),
                    normalized: before_entity.to_lowercase(),
                    position: 0,
                    is_possessive: false,
                });
            }
            if !after_entity.is_empty()
                && after_entity.len() >= 2
                && !STOP_WORDS.contains(&after_entity.to_lowercase().as_str())
            {
                candidates.push(EntityCandidate {
                    token: after_entity.to_string(),
                    normalized: after_entity.to_lowercase(),
                    position: 1,
                    is_possessive: false,
                });
            }

            // If slash found candidates, return early
            if !candidates.is_empty() {
                return candidates;
            }
        }
    }

    // Strategy 2: Handle possessive constructs: "Tesla's repos"
    // Pattern: find "'s" and take the word before it
    if let Some(apos_pos) = query.find("'s") {
        if apos_pos >= 2 {
            let _word = &query[apos_pos - 1..];
            // Walk back to find the start of the word
            let mut start = apos_pos - 1;
            while start > 0 && query[..start].chars().last().map(|c| c.is_alphabetic()).unwrap_or(false) {
                start -= 1;
            }
            let entity_word = &query[start..apos_pos];
                    if entity_word.len() >= 2
                && !STOP_WORDS.contains(&entity_word.to_lowercase().as_str())
            {
                candidates.push(EntityCandidate {
                    token: entity_word.to_string(),
                    normalized: entity_word.to_lowercase(),
                    position: 0,
                    is_possessive: true,
                });
            }
        }
    }

    // Strategy 3: Extract capitalized words that appear to be proper nouns
    // (Skip if we already found possessive entity)
    if candidates.is_empty() {
        let mut position = 0;
        let mut current_word = String::new();
        let mut prev_was_alphanumeric = false;

        for (_i, c) in query.char_indices() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                let is_upper = c.is_uppercase();

                if !prev_was_alphanumeric && is_upper {
                    // New capitalized word starting here
                    if !current_word.is_empty() {
                        let trimmed = current_word.trim_matches(|ch: char| ch == '\'' || ch == '-');
                        if trimmed.len() >= 2 && !STOP_WORDS.contains(&trimmed.to_lowercase().as_str()) {
                            candidates.push(EntityCandidate {
                                token: trimmed.to_string(),
                                normalized: trimmed.to_lowercase(),
                                position,
                                is_possessive: false,
                            });
                            position += 1;
                        }
                    }
                    current_word.clear();
                }

                if is_upper || !current_word.is_empty() {
                    current_word.push(c);
                }
                prev_was_alphanumeric = true;
            } else {
                if !current_word.is_empty() && prev_was_alphanumeric {
                    let trimmed = current_word.trim_matches(|ch: char| ch == '\'' || ch == '-');
                    if trimmed.len() >= 2 && !STOP_WORDS.contains(&trimmed.to_lowercase().as_str()) {
                        candidates.push(EntityCandidate {
                            token: trimmed.to_string(),
                            normalized: trimmed.to_lowercase(),
                            position,
                            is_possessive: false,
                        });
                        position += 1;
                    }
                    current_word.clear();
                }
                prev_was_alphanumeric = false;
            }
        }

        // Handle final word
        if !current_word.is_empty() {
            let trimmed = current_word.trim_matches(|ch: char| ch == '\'' || ch == '-');
            if trimmed.len() >= 2 && !STOP_WORDS.contains(&trimmed.to_lowercase().as_str()) {
                candidates.push(EntityCandidate {
                    token: trimmed.to_string(),
                    normalized: trimmed.to_lowercase(),
                    position,
                    is_possessive: false,
                });
            }
        }
    }

    candidates
}

/// Extract path parameter names from endpoint pattern.
fn extract_path_params(endpoint: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut in_param = false;
    let mut current = String::new();

    for c in endpoint.chars() {
        if c == '{' {
            in_param = true;
            current.clear();
        } else if c == '}' && in_param {
            if !current.is_empty() {
                params.push(current.clone());
            }
            in_param = false;
            current.clear();
        } else if in_param {
            current.push(c);
        }
    }

    params
}

/// Score how well an entity matches a parameter based on description and context.
fn score_entity_param_match(
    entity: &EntityCandidate,
    param_name: &str,
    param_desc: &str,
    endpoint: &str,
    existing_args: &HashMap<String, Value>,
) -> f64 {
    // Skip if already assigned
    if existing_args.contains_key(param_name) {
        return 0.0;
    }

    let mut score = 0.0;
    let desc_lower = param_desc.to_lowercase();
    let token_lower = &entity.normalized;

    // 1. Direct token match in description (strong signal)
    if desc_lower.contains(token_lower) {
        score += 0.5;
    }

    // 2. Possessive 's pattern: "Tesla's repos" -> entity is owner/org
    if entity.is_possessive {
        for indicator in OWNERSHIP_KEYWORDS {
            if desc_lower.contains(indicator) {
                score += 0.3;
                break;
            }
        }
    }

    // 3. Position alignment with endpoint path parameters
    let path_params = extract_path_params(endpoint);
    if let Some(pos) = path_params.iter().position(|p| p == param_name) {
        // If multiple entities, their positions should align with path param positions
        score += 0.2 * (1.0 - (pos as f64 * 0.1));
    }

    // 4. Description word overlap
    let desc_words: HashSet<&str> = desc_lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3 && !STOP_WORDS.contains(w))
        .collect();

    let entity_words: HashSet<&str> = token_lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1)
        .collect();

    let overlap = desc_words.intersection(&entity_words).count() as f64;
    if overlap > 0.0 {
        score += (overlap * 0.15).min(0.25);
    }

    // 5. Parameter name appears in description (param is self-describing)
    if desc_lower.contains(param_name) {
        score += 0.1;
    }

    // 6. Boost for type=string params vs other types (entities are strings)
    // (This is handled by only calling this for string params in the first place)

    score
}

/// Assign entities to parameters using greedy matching.
fn assign_entities_to_params(
    mut scored: Vec<(String, EntityCandidate, f64)>,
) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    let mut used_entities: HashSet<usize> = HashSet::new();
    let mut used_params: HashSet<String> = HashSet::new();

    // Sort by score descending
    scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    for (param_name, entity, score) in scored {
        if score < SCORE_THRESHOLD {
            continue;
        }
        if used_entities.contains(&entity.position) {
            continue;
        }
        if used_params.contains(&param_name) {
            continue;
        }

        // Strip possessive marker from token for the value
        let value = entity.token.trim_end_matches(|c: char| c == '\'' || c == 's').to_string();
        result.insert(param_name.clone(), Value::String(value));
        used_entities.insert(entity.position);
        used_params.insert(param_name);
    }

    result
}

/// Extract entity arguments by matching against parameter descriptions.
fn extract_entity_arguments(
    query: &str,
    tool: &Tool,
    already_filled: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    let entities = extract_entity_candidates(query);
    if entities.is_empty() {
        return HashMap::new();
    }

    // Check if this is a slash-format query: entities were extracted from "a/b" pattern
    // For slash format, assign by position (entity[0] → first string param, etc.)
    if entities.iter().any(|e| e.normalized.contains('/')) || query.contains(" owner/") || query.contains(" /") {
        // Check if entities are from slash extraction (position 0 and 1)
        let slash_entities: Vec<&EntityCandidate> = entities
            .iter()
            .filter(|e| e.position < 2)
            .collect();

        if slash_entities.len() >= 2 {
            // Get string params in definition order
            let string_params: Vec<&str> = tool
                .input_schema
                .properties
                .iter()
                .filter(|(_, spec)| spec.get("type").and_then(|t| t.as_str()) == Some("string"))
                .map(|(name, _)| name.as_str())
                .collect();

            let mut result = already_filled.clone();
            for (i, entity) in slash_entities.iter().enumerate() {
                if i < string_params.len() {
                    let param = string_params[i];
                    if !result.contains_key(param) {
                        let value = entity.token.trim_end_matches(|c: char| c == '\'' || c == 's').to_string();
                        result.insert(param.to_string(), Value::String(value));
                    }
                }
            }
            return result;
        }
    }

    // Non-slash: use description-based scoring
    let mut scored: Vec<(String, EntityCandidate, f64)> = Vec::new();

    for (param_name, param_spec) in &tool.input_schema.properties {
        // Only consider string parameters for entity extraction
        if param_spec.get("type").and_then(|t| t.as_str()) != Some("string") {
            continue;
        }

        let param_desc = param_spec
            .get("description")
            .and_then(|d| d.as_str())
            .unwrap_or("");

        for entity in &entities {
            let score = score_entity_param_match(
                entity,
                &param_name.to_lowercase(),
                param_desc,
                &tool.endpoint,
                already_filled,
            );

            if score >= SCORE_THRESHOLD {
                scored.push((param_name.clone(), entity.clone(), score));
            }
        }
    }

    let result = assign_entities_to_params(scored);
    result
}

// ============================================================================
// Tier 3: Post-Processing
// ============================================================================

fn post_process(args: HashMap<String, Value>, tool: &Tool) -> ExtractResult {
    let mut final_args = args;

    // 1. Remove parameters that match their default values
    for (param_name, param_spec) in &tool.input_schema.properties {
        if let Some(default_val) = param_spec.get("default") {
            if let Some(extracted) = final_args.get(param_name) {
                if extracted == default_val {
                    final_args.remove(param_name);
                }
            }
        }
    }

    // 2. Identify missing required parameters
    let missing_required: Vec<String> = tool
        .input_schema
        .required
        .iter()
        .filter(|r| !final_args.contains_key(*r))
        .cloned()
        .collect();

    // 3. Calculate confidence
    let confidence = calculate_confidence(&final_args, tool);

    ExtractResult {
        arguments: Value::Object(final_args.into_iter().collect()),
        confidence,
        missing_required,
    }
}

fn calculate_confidence(args: &HashMap<String, Value>, tool: &Tool) -> f64 {
    if args.is_empty() {
        return 0.0;
    }

    let required_count = tool.input_schema.required.len();
    let filled_required = args
        .iter()
        .filter(|(name, _)| tool.input_schema.required.contains(name))
        .count();

    if required_count == 0 {
        // No required params - medium-high confidence if we found anything
        let total_params = tool.input_schema.properties.len();
        let filled = args.len();
        return if total_params > 0 {
            (filled as f64 / total_params as f64).min(0.8)
        } else {
            0.5
        };
    }

    let required_coverage = filled_required as f64 / required_count as f64;
    let optional_filled = args.len() - filled_required;
    let optional_total = tool.input_schema.properties.len() - required_count;
    let optional_coverage = if optional_total > 0 {
        optional_filled as f64 / optional_total as f64
    } else {
        0.0
    };

    // Weighted: required coverage matters more
    required_coverage * 0.8 + optional_coverage * 0.2
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::{HttpMethod, InputSchema};

    fn make_tool_with_owner() -> Tool {
        Tool {
            slug: "github_list_repos".to_string(),
            name: "List Repositories".to_string(),
            description: "List repositories for a user or organization".to_string(),
            provider: "github".to_string(),
            endpoint: "/repos/{owner}/{repo}".to_string(),
            method: HttpMethod::GET,
            input_schema: InputSchema {
                schema_type: Some("object".to_string()),
                description: Some("Parameters".to_string()),
                properties: {
                    let mut p = HashMap::new();
                    p.insert(
                        "owner".to_string(),
                        serde_json::json!({
                            "type": "string",
                            "description": "Repository owner username or organization"
                        }),
                    );
                    p.insert(
                        "repo".to_string(),
                        serde_json::json!({
                            "type": "string",
                            "description": "Repository name"
                        }),
                    );
                    p.insert(
                        "sort".to_string(),
                        serde_json::json!({
                            "type": "string",
                            "description": "Sort order",
                            "enum": ["created", "updated", "pushed", "full_name"]
                        }),
                    );
                    p
                },
                required: vec!["owner".to_string()],
            },
            output_schema: None,
            scopes: vec![],
            tags: vec![],
            icon_url: None,
            example_queries: vec![],
        }
    }

    #[test]
    fn test_extract_tesla_as_owner() {
        let tool = make_tool_with_owner();
        let result = extract_arguments("list Tesla's repositories", &tool);

        assert!(
            result.arguments.get("owner").is_some(),
            "owner should be extracted"
        );
        assert_eq!(
            result.arguments.get("owner").unwrap().as_str().unwrap(),
            "Tesla"
        );
    }

    #[test]
    fn test_enum_extraction() {
        let tool = make_tool_with_owner();
        let result = extract_arguments("list repos sorted by updated", &tool);

        assert_eq!(
            result.arguments.get("sort").unwrap().as_str().unwrap(),
            "updated"
        );
    }

    #[test]
    fn test_slash_format_extraction() {
        let tool = make_tool_with_owner();
        let result = extract_arguments("show me openai/gpt-4", &tool);

        assert_eq!(
            result.arguments.get("owner").unwrap().as_str().unwrap(),
            "openai"
        );
        assert_eq!(
            result.arguments.get("repo").unwrap().as_str().unwrap(),
            "gpt-4"
        );
    }

    #[test]
    fn test_missing_required() {
        let tool = make_tool_with_owner();
        let result = extract_arguments("list repositories", &tool);

        assert!(result.missing_required.contains(&"owner".to_string()));
    }

    #[test]
    fn test_nps_find_parks() {
        // Test argument extraction for National Parks MCP tool
        let tool = Tool {
            slug: "nationalparks_find_parks".to_string(),
            name: "Find Parks".to_string(),
            description: "Search for U.S. National Parks".to_string(),
            provider: "mcp-nationalparks".to_string(),
            endpoint: "/parks".to_string(),
            method: HttpMethod::GET,
            input_schema: InputSchema {
                schema_type: Some("object".to_string()),
                description: Some("Parameters for searching national parks".to_string()),
                properties: {
                    let mut p = std::collections::HashMap::new();
                    p.insert("stateCode".to_string(), serde_json::json!({
                        "type": "string",
                        "description": "Filter by state code (e.g., 'CA' for California)"
                    }));
                    p.insert("q".to_string(), serde_json::json!({
                        "type": "string",
                        "description": "Search term to filter parks by name or description"
                    }));
                    p.insert("limit".to_string(), serde_json::json!({
                        "type": "integer",
                        "description": "Maximum number of parks to return",
                        "default": 10
                    }));
                    p
                },
                required: vec![],
            },
            output_schema: None,
            scopes: vec![],
            tags: vec!["mcp".to_string(), "national-parks".to_string()],
            icon_url: None,
            example_queries: vec![],
        };

        // Test that literal stateCode param name in query matches
        let result = extract_arguments("find parks with stateCode CA", &tool);
        assert_eq!(
            result.arguments.get("stateCode").unwrap().as_str().unwrap(),
            "ca"
        );

        // Test query param matches literal param name "q"
        let result = extract_arguments("find parks with q camping", &tool);
        assert_eq!(
            result.arguments.get("q").unwrap().as_str().unwrap(),
            "camping"
        );
    }

    #[test]
    fn test_confidence_with_required() {
        let tool = make_tool_with_owner();
        let result = extract_arguments("list Tesla's repositories", &tool);

        // owner is filled, repo is not required
        assert!(result.confidence > 0.0);
    }
}
