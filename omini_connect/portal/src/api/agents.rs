//! Agent registration API — allows AI agents to register themselves and get dedicated API keys.

use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};

use crate::app::AppState;
use crate::auth::models::{Agent, AgentResponse, AgentSummary, RegisterAgentRequest};

/// Error helper.
fn error(status: StatusCode, msg: &str) -> Response {
    let body = serde_json::json!({ "error": msg }).to_string();
    let mut resp = axum::response::Response::new(body.into());
    *resp.status_mut() = status;
    resp
}

/// Authenticate via Bearer token and return (owner_username, agent_id).
async fn auth_agent(
    state: &Arc<AppState>,
    headers: &HeaderMap,
) -> Result<(String, Option<String>), Response> {
    use axum::http::header::AUTHORIZATION;

    let api_key = match headers.get(AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        Some(v) => v.strip_prefix("Bearer ").unwrap_or(v),
        None => return Err(error(StatusCode::UNAUTHORIZED, "missing authorization header")),
    };

    let api_keys = state
        .api_keys
        .list_all()
        .await
        .map_err(|e| error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to list API keys: {}", e)))?;

    for ak in api_keys {
        if bcrypt::verify(api_key, &ak.key_hash).ok() == Some(true) {
            return Ok((ak.username, ak.agent_id));
        }
    }

    Err(error(StatusCode::UNAUTHORIZED, "invalid API key"))
}

/// Register a new agent. Requires Bearer auth (human user).
/// Returns the agent details and the raw API key (shown once).
pub async fn register_agent(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<RegisterAgentRequest>,
) -> Response {
    let (owner, _) = match auth_agent(&state, &headers).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    // Generate agent ID and API key
    let agent_id = uuid::Uuid::new_v4().to_string();
    let raw_key = uuid::Uuid::new_v4().to_string();
    let key_hash = bcrypt::hash(&raw_key, bcrypt::DEFAULT_COST).unwrap_or_default();
    let now = chrono::Utc::now();

    let agent = Agent {
        id: agent_id.clone(),
        name: req.name.clone(),
        description: req.description.unwrap_or_default(),
        owner_username: owner.clone(),
        active: true,
        created_at: now,
    };

    // Insert agent
    if let Err(e) = state.agents.insert(&agent).await {
        tracing::error!("DB error inserting agent: {}", e);
        return error(StatusCode::INTERNAL_SERVER_ERROR, "failed to create agent");
    }

    // Create API key for the agent
    let api_key = crate::auth::models::ApiKey {
        key_hash,
        username: owner.clone(),
        label: format!("agent:{}", agent_id),
        created_at: now,
        agent_id: Some(agent_id.clone()),
    };

    if let Err(e) = state.api_keys.insert(&api_key).await {
        tracing::error!("DB error inserting agent API key: {}", e);
        return error(StatusCode::INTERNAL_SERVER_ERROR, "failed to create agent API key");
    }

    tracing::info!("Registered agent {} for owner {}", agent_id, owner);

    let resp = AgentResponse {
        id: agent.id,
        name: agent.name,
        description: agent.description,
        owner_username: agent.owner_username,
        active: agent.active,
        created_at: agent.created_at,
        api_key: Some(raw_key),
    };

    (StatusCode::CREATED, Json(resp)).into_response()
}

/// List all agents for the authenticated user.
pub async fn list_agents(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let (owner, _) = match auth_agent(&state, &headers).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    match state.agents.list_by_owner(&owner).await {
        Ok(agents) => {
            let summaries: Vec<AgentSummary> = agents.into_iter().map(|a| a.into()).collect();
            Json(serde_json::json!({ "agents": summaries })).into_response()
        }
        Err(e) => error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to list agents: {}", e)),
    }
}

/// Get a specific agent by ID.
pub async fn get_agent(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
) -> Response {
    let (owner, _) = match auth_agent(&state, &headers).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    match state.agents.get(&agent_id).await {
        Ok(Some(agent)) if agent.owner_username == owner => {
            let summary: AgentSummary = agent.into();
            Json(serde_json::json!({ "agent": summary })).into_response()
        }
        Ok(Some(_)) => error(StatusCode::FORBIDDEN, "not your agent"),
        Ok(None) => error(StatusCode::NOT_FOUND, "agent not found"),
        Err(e) => error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to get agent: {}", e)),
    }
}

/// Deactivate an agent (does not delete — preserves audit trail).
pub async fn deactivate_agent(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
) -> Response {
    let (owner, _) = match auth_agent(&state, &headers).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    // Verify ownership
    match state.agents.get(&agent_id).await {
        Ok(Some(agent)) if agent.owner_username != owner => {
            return error(StatusCode::FORBIDDEN, "not your agent");
        }
        Ok(None) => return error(StatusCode::NOT_FOUND, "agent not found"),
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to get agent: {}", e)),
        _ => {}
    }

    match state.agents.set_active(&agent_id, false).await {
        Ok(_) => {
            tracing::info!("Deactivated agent {}", agent_id);
            Json(serde_json::json!({ "ok": true })).into_response()
        }
        Err(e) => error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to deactivate agent: {}", e)),
    }
}

/// Delete an agent and revoke its API key.
pub async fn delete_agent(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
) -> Response {
    let (owner, _) = match auth_agent(&state, &headers).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    // Verify ownership
    match state.agents.get(&agent_id).await {
        Ok(Some(agent)) if agent.owner_username != owner => {
            return error(StatusCode::FORBIDDEN, "not your agent");
        }
        Ok(None) => return error(StatusCode::NOT_FOUND, "agent not found"),
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to get agent: {}", e)),
        _ => {}
    }

    // Delete the agent
    if let Err(e) = state.agents.delete(&agent_id).await {
        return error(StatusCode::INTERNAL_SERVER_ERROR, &format!("failed to delete agent: {}", e));
    }

    // Revoke all API keys for this agent
    // API keys for agents have label "agent:{id}"
    match state.api_keys.list_by_username(&owner).await {
        Ok(keys) => {
            for ak in keys {
                if ak.agent_id.as_ref() == Some(&agent_id) {
                    if let Err(e) = state.api_keys.delete(&ak.key_hash).await {
                        tracing::warn!("Failed to revoke agent key {}: {}", ak.key_hash, e);
                    }
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to list keys for agent revocation: {}", e);
        }
    }

    tracing::info!("Deleted agent {} and revoked its API keys", agent_id);
    Json(serde_json::json!({ "ok": true })).into_response()
}

/// Router for agent management API.
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/agents", post(register_agent))
        .route("/agents", get(list_agents))
        .route("/agents/:id", get(get_agent))
        .route("/agents/:id", delete(delete_agent))
        .route("/agents/:id/deactivate", post(deactivate_agent))
}