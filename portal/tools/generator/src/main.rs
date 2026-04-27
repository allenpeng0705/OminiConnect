//! Tool generator from OpenAPI specs.
//!
//! Usage:
//!     cargo run --bin generate-tools -- --spec ./openapi.json --provider github --output ../registry/github.yaml
//!
//! Allowlist options:
//!     --operation-ids "list_repos,get_user,create_issue"  # Explicit list
//!     --tags "repos,issues,users"  # Only operations with these tags
//!     --prefix "list_,get_,create_"  # Only operations starting with these prefixes

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize)]
struct Tool {
    slug: String,
    name: String,
    description: String,
    provider: String,
    endpoint: String,
    method: String,
    input_schema: InputSchema,
    scopes: Vec<String>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct InputSchema {
    #[serde(rename = "type")]
    schema_type: Option<String>,
    description: Option<String>,
    #[serde(default)]
    properties: HashMap<String, serde_json::Value>,
    #[serde(default)]
    required: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAPISpec {
    openapi: String,
    info: Info,
    paths: HashMap<String, PathItem>,
}

#[derive(Debug, Deserialize)]
struct Info {
    title: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct PathItem {
    #[serde(default)]
    get: Option<Operation>,
    #[serde(default)]
    post: Option<Operation>,
    #[serde(default)]
    put: Option<Operation>,
    #[serde(default)]
    delete: Option<Operation>,
    #[serde(default)]
    patch: Option<Operation>,
}

#[derive(Debug, Deserialize)]
struct Operation {
    operation_id: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    parameters: Vec<Parameter>,
    #[serde(default)]
    request_body: Option<RequestBody>,
}

#[derive(Debug, Deserialize)]
struct Parameter {
    name: String,
    #[serde(rename = "in")]
    location: String,
    description: Option<String>,
    required: Option<bool>,
    schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct RequestBody {
    #[serde(default)]
    content: HashMap<String, MediaType>,
}

#[derive(Debug, Deserialize)]
struct MediaType {
    schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct AllowlistConfig {
    operation_ids: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    prefixes: Option<Vec<String>>,
}

fn main() {
    // Parse command line args manually (simple approach)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: generate-tools --spec <openapi.json> --provider <name> --output <output.yaml>");
        println!("Options:");
        println!("  --operation-ids <comma,sep,list>  # Explicit operation IDs to include");
        println!("  --tags <comma,sep,tags>           # Only include operations with these tags");
        println!("  --prefixes <comma,sep,prefixes>   # Only include operations starting with these");
        std::process::exit(1);
    }

    let mut spec_path = None;
    let mut provider = None;
    let mut output_path = None;
    let mut operation_ids: Option<Vec<String>> = None;
    let mut tags: Option<Vec<String>> = None;
    let mut prefixes: Option<Vec<String>> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => { spec_path = Some(args[i + 1].clone()); i += 2; }
            "--provider" => { provider = Some(args[i + 1].clone()); i += 2; }
            "--output" => { output_path = Some(args[i + 1].clone()); i += 2; }
            "--operation-ids" => {
                operation_ids = Some(args[i + 1].split(',').map(|s| s.trim().to_string()).collect());
                i += 2;
            }
            "--tags" => {
                tags = Some(args[i + 1].split(',').map(|s| s.trim().to_string()).collect());
                i += 2;
            }
            "--prefixes" => {
                prefixes = Some(args[i + 1].split(',').map(|s| s.trim().to_string()).collect());
                i += 2;
            }
            _ => { i += 1; }
        }
    }

    let spec_path = spec_path.expect("--spec required");
    let provider = provider.expect("--provider required");
    let output_path = output_path.expect("--output required");

    println!("Loading OpenAPI spec from {}...", spec_path);
    let spec_content = std::fs::read_to_string(&spec_path)
        .expect("Failed to read OpenAPI spec file");
    let spec: OpenAPISpec = serde_json::from_str(&spec_content)
        .expect("Failed to parse OpenAPI JSON");

    println!("Generating tools for provider '{}'...", provider);

    let mut tools = Vec::new();

    for (path, path_item) in &spec.paths {
        let methods = [
            ("GET", &path_item.get),
            ("POST", &path_item.post),
            ("PUT", &path_item.put),
            ("DELETE", &path_item.delete),
            ("PATCH", &path_item.patch),
        ];

        for (method, operation_opt) in methods {
            let Some(operation) = operation_opt else { continue; };

            // Skip if no operation ID
            let Some(operation_id) = &operation.operation_id else { continue; };

            // Apply allowlist filters
            if !should_include(operation_id, &operation.tags, &operation_ids, &tags, &prefixes) {
                continue;
            }

            let tool = generate_tool(&provider, path, method, operation);
            tools.push(tool);
        }
    }

    println!("Generated {} tools", tools.len());

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&tools).expect("Failed to serialize tools to YAML");
    std::fs::write(&output_path, yaml).expect("Failed to write output file");

    println!("Written to {}", output_path);
}

fn should_include(
    operation_id: &str,
    operation_tags: &[String],
    operation_ids: &Option<Vec<String>>,
    tags: &Option<Vec<String>>,
    prefixes: &Option<Vec<String>>,
) -> bool {
    // Check explicit operation IDs
    if let Some(ids) = operation_ids {
        return ids.iter().any(|id| operation_id.contains(id));
    }

    // Check tags
    if let Some(tag_list) = tags {
        return operation_tags.iter().any(|t| tag_list.iter().any(|tag| t == tag));
    }

    // Check prefixes
    if let Some(prefix_list) = prefixes {
        return prefix_list.iter().any(|p| operation_id.starts_with(p));
    }

    // No filter - include all
    true
}

fn generate_tool(provider: &str, path: &str, method: &str, operation: &Operation) -> Tool {
    let slug = format!("{}_{}", provider, to_snake_case(operation.operation_id.as_ref().unwrap_or(&String::new())));

    let name = operation.summary.clone()
        .or(operation.operation_id.clone())
        .unwrap_or_else(|| format!("{} {}", method, path));

    let description = operation.description.clone()
        .or(operation.summary.clone())
        .unwrap_or_default();

    // Build input schema from parameters
    let mut properties = HashMap::new();
    let mut required = Vec::new();

    for param in &operation.parameters {
        let mut schema = param.schema.clone().unwrap_or(serde_json::Value::Null);
        if schema.is_null() {
            schema = serde_json::json!({ "type": "string" });
        }
        if let Some(desc) = &param.description {
            if let Some(obj) = schema.as_object_mut() {
                obj.insert("description".to_string(), serde_json::Value::String(desc.clone()));
            }
        }
        properties.insert(param.name.clone(), schema);
        if param.required.unwrap_or(false) {
            required.push(param.name.clone());
        }
    }

    // Handle request body for POST/PUT/PATCH
    if let Some(body) = &operation.request_body {
        if let Some(media_type) = body.content.get("application/json") {
            if let Some(schema) = &media_type.schema {
                if let Some(obj) = schema.as_object() {
                    // Merge body schema properties
                    if let Some(props) = obj.get("properties") {
                        if let Some(props_obj) = props.as_object() {
                            for (k, v) in props_obj {
                                properties.insert(k.clone(), v.clone());
                            }
                        }
                    }
                    if let Some(req) = obj.get("required") {
                        if let Some(req_arr) = req.as_array() {
                            for r in req_arr {
                                if let Some(r_str) = r.as_str() {
                                    if !required.contains(&r_str.to_string()) {
                                        required.push(r_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let input_schema = InputSchema {
        schema_type: Some("object".to_string()),
        description: None,
        properties,
        required,
    };

    // Extract scopes from security or tags (simplified - real implementation would look at OAuth2 security schemes)
    let scopes: Vec<String> = operation.tags.clone();

    let tags = operation.tags.clone();

    Tool {
        slug,
        name,
        description,
        provider: provider.to_string(),
        endpoint: path.to_string(),
        method: method.to_string(),
        input_schema,
        scopes,
        tags,
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    // Replace non-alphanumeric with underscores
    result.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect()
}
