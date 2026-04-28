//! Application state shared across all handlers.

use std::sync::Arc;

use sqlx::Any;

use crate::db::{
    SqlxAgentRepo, SqlxApiKeyRepo, SqlxConnectorRepo, SqlxCustomToolRepo, SqlxDepartmentScopeRepo,
    SqlxSessionRepo, SqlxToolExecutionRepo, SqlxUserRepo,
};
use crate::llm::{LiteLLMClient, LiteLLMConfig};
use crate::panda::{PandaAIGatewayClient, PandaAPIGatewayClient, PandaConfig, PandaMCPGatewayClient};
use crate::tools::ToolRegistry;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<dyn crate::db::UserRepository>,
    pub sessions: Arc<dyn crate::db::SessionRepository>,
    pub api_keys: Arc<dyn crate::db::ApiKeyRepository>,
    pub connectors: Arc<dyn crate::db::ConnectorRepository>,
    pub agents: Arc<dyn crate::db::AgentRepository>,
    pub tool_executions: Arc<dyn crate::db::ToolExecutionRepository>,
    pub oauth_vault: Arc<omini_connect_oauth_vault::OAuthVault>,
    pub tools: Arc<ToolRegistry>,
    pub custom_tools: Arc<dyn crate::db::CustomToolRepository>,
    pub department_scopes: Arc<dyn crate::db::DepartmentScopeRepository>,

    // LLM and Panda configuration
    pub llm_config: LiteLLMConfig,
    pub llm_client: LiteLLMClient,
    pub panda_config: PandaConfig,
    pub panda_ai_gateway: Option<PandaAIGatewayClient>,
    pub panda_mcp_gateway: Option<PandaMCPGatewayClient>,
    pub panda_api_gateway: Option<PandaAPIGatewayClient>,
}

impl AppState {
    pub async fn new(pool: sqlx::pool::Pool<Any>, tools: ToolRegistry) -> Self {
        let user_repo = SqlxUserRepo::new(pool.clone());
        let session_repo = SqlxSessionRepo::new(pool.clone());
        let api_key_repo = SqlxApiKeyRepo::new(pool.clone());
        let connector_repo = SqlxConnectorRepo::new(pool.clone());
        let agent_repo = SqlxAgentRepo::new(pool.clone());
        let tool_exec_repo = SqlxToolExecutionRepo::new(pool.clone());
        let custom_tool_repo = SqlxCustomToolRepo::new(pool.clone());
        let dept_scope_repo = SqlxDepartmentScopeRepo::new(pool.clone());

        // Create SQLx-backed token store for OAuth token persistence
        let token_store_backend = Arc::new(omini_connect_oauth_vault::SqlxTokenStoreBackend::new(
            pool.clone(),
        ));
        let token_store = Arc::new(omini_connect_oauth_vault::TokenStore::new(
            token_store_backend,
        ));
        let oauth_vault = Arc::new(omini_connect_oauth_vault::OAuthVault::new(token_store));

        // Initialize LLM client
        let llm_config = LiteLLMConfig::from_env();
        let llm_client = LiteLLMClient::from_env();

        // Initialize Panda configuration
        let panda_config = PandaConfig::from_env();
        let panda_ai_gateway = if panda_config.is_ai_gateway_enabled() {
            Some(PandaAIGatewayClient::new(panda_config.ai_gateway.clone()))
        } else {
            None
        };
        let panda_mcp_gateway = if panda_config.is_mcp_gateway_enabled() {
            Some(PandaMCPGatewayClient::new(panda_config.mcp_gateway.clone()))
        } else {
            None
        };
        let panda_api_gateway = if panda_config.is_api_gateway_enabled() {
            Some(PandaAPIGatewayClient::new(panda_config.api_gateway.clone()))
        } else {
            None
        };

        Self {
            users: Arc::new(user_repo),
            sessions: Arc::new(session_repo),
            api_keys: Arc::new(api_key_repo),
            connectors: Arc::new(connector_repo),
            agents: Arc::new(agent_repo),
            tool_executions: Arc::new(tool_exec_repo),
            oauth_vault,
            tools: Arc::new(tools),
            custom_tools: Arc::new(custom_tool_repo),
            department_scopes: Arc::new(dept_scope_repo),
            llm_config,
            llm_client,
            panda_config,
            panda_ai_gateway,
            panda_mcp_gateway,
            panda_api_gateway,
        }
    }
}
