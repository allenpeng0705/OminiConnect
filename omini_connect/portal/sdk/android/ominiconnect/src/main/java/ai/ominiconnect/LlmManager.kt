package ai.ominiconnect

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

// ─── LLM Types ────────────────────────────────────────────────────────────────

data class CandidateTool(
    val tool: String,
    val name: String,
    val matchReason: String
)

data class LlmExecuteResponse(
    val ok: Boolean,
    val tool: String? = null,
    val toolName: String? = null,
    val arguments: Map<String, Any>? = null,
    val explanation: String? = null,
    val result: Map<String, Any>? = null,
    val error: String? = null,
    val message: String? = null,
    val candidates: List<CandidateTool>? = null,
    val availableToolsHint: String? = null
)

data class AvailableTool(
    val slug: String,
    val name: String,
    val description: String,
    val exampleQueries: List<String>,
    val scopes: List<String>,
    val scopeSatisfied: String
)

data class PlatformTools(
    val connected: Boolean,
    val tools: List<AvailableTool>? = null
)

data class LlmToolsResponse(
    val platforms: Map<String, PlatformTools>
)

// ─── LLM Manager ────────────────────────────────────────────────────────────────

class LlmManager(private val client: OminiConnect) {
    suspend fun execute(query: String, platform: String? = null): LlmExecuteResponse {
        val body = mutableMapOf<String, Any>("query" to query)
        platform?.let { body["platform"] = it }
        val json = client.request<Map<String, Any>>("POST", "/api/llm", body = body)
        return client.parseLlmExecuteResponse(JSONObject(json))
    }

    suspend fun listAvailableTools(platform: String? = null): LlmToolsResponse {
        val params = platform?.let { mapOf("platform" to it) }
        val json = client.request<Map<String, Any>>("GET", "/api/llm/tools", params = params)
        return client.parseLlmToolsResponse(JSONObject(json))
    }
}
