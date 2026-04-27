package ai.ominiconnect

import kotlinx.coroutines.*
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder
import org.json.JSONArray
import org.json.JSONObject
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

// ─── Errors ─────────────────────────────────────────────────────────────────

sealed class OminiConnectError(message: String, val statusCode: Int = 0, val body: String = "") : Exception(message) {
    class Auth(message: String = "Invalid or missing API key") : OminiConnectError(message)
    class ConnectorNotFound(platform: String) : OminiConnectError("Platform not connected: $platform")
    class ToolNotFound(slug: String) : OminiConnectError("Tool not found: $slug")
    class ScopeInsufficient(message: String = "Insufficient scopes for this operation") : OminiConnectError(message)
    class Upstream(statusCode: Int, body: String, message: String = "Upstream error $statusCode") : OminiConnectError(message, statusCode, body)
    class Network(cause: Throwable) : OminiConnectError("Network error: ${cause.message}")
    class RateLimited(message: String = "Rate limited — back off and retry") : OminiConnectError(message)
}

// ─── Types ──────────────────────────────────────────────────────────────────

data class Connector(
    val platform: String,
    val enabled: Boolean,
    val scopes: List<String>,
    val createdAt: String
)

data class ApiKeyCreated(
    val key: String,
    val label: String,
    val createdAt: String
)

data class ApiKeySummary(
    val keyHash: String,
    val label: String,
    val createdAt: String
)

data class ToolSummary(
    val slug: String,
    val name: String,
    val description: String,
    val method: String,
    val endpoint: String,
    val scopes: List<String>,
    val scopeSatisfied: String,
    val tags: List<String>
)

data class Toolkit(
    val slug: String,
    val name: String,
    val logo: String?,
    val provider: String,
    val tools: List<ToolSummary>
)

data class ToolkitsResponse(
    val toolkits: List<Toolkit>
)

data class ToolsSearchResponse(
    val tools: List<ToolSummary>,
    val query: String
)

data class ToolExecuteResult(
    val ok: Boolean,
    val body: Any?,
    val error: String?,
    val callId: String?,
    val status: String?,
    val durationMs: Long?
)

// ─── Client ─────────────────────────────────────────────────────────────────

class OminiConnect(
    private val apiKey: String,
    private val baseUrl: String = "http://localhost:9000"
) {
    private val base = baseUrl.trimEnd('/')

    val connectors = ConnectorsManager(this)
    val tools = ToolsManager(this)
    val apiKeys = ApiKeysManager(this)

    private val scope = Dispatchers.IO + SupervisorJob()

    suspend fun <T> request(
        method: String,
        path: String,
        body: Map<String, Any?>? = null,
        params: Map<String, String>? = null
    ): T = withContext(scope) {
        val queryString = params?.entries?.joinToString("&") {
            "${URLEncoder.encode(it.key, "UTF-8")}=${URLEncoder.encode(it.value, "UTF-8")}"
        } ?: ""
        val urlStr = if (queryString.isNotEmpty()) "$base$path?$queryString" else "$base$path"

        val url = URL(urlStr)
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = method
        conn.setRequestProperty("Authorization", "Bearer $apiKey")
        conn.setRequestProperty("Content-Type", "application/json")
        conn.doInput = true
        conn.connectTimeout = 30000
        conn.readTimeout = 30000

        body?.let {
            conn.doOutput = true
            conn.outputStream.use { os ->
                os.write(JSONObject(it.filterValues { v -> v != null }).toString().toByteArray())
            }
        }

        try {
            val responseCode = conn.responseCode
            val responseBody = conn.inputStream.use { it.bufferedReader().readText() }

            when (responseCode) {
                401 -> throw OminiConnectError.Auth()
                429 -> throw OminiConnectError.RateLimited()
                !in 200..299 -> throw OminiConnectError.Upstream(responseCode, responseBody)
            }

            @Suppress("UNCHECKED_CAST")
            return@withContext JSONObject(responseBody).toMap() as T
        } catch (e: OminiConnectError) {
            throw e
        } catch (e: Exception) {
            throw OminiConnectError.Network(e)
        }
    }

    suspend fun requestNoContent(method: String, path: String, body: Map<String, Any?>? = null) {
        withContext(scope) {
            val url = URL("$base$path")
            val conn = url.openConnection() as HttpURLConnection
            conn.requestMethod = method
            conn.setRequestProperty("Authorization", "Bearer $apiKey")
            conn.setRequestProperty("Content-Type", "application/json")

            body?.let {
                conn.doOutput = true
                conn.outputStream.use { os ->
                    os.write(JSONObject(it.filterValues { v -> v != null }).toString().toByteArray())
                }
            }

            try {
                val responseCode = conn.responseCode
                when (responseCode) {
                    401 -> throw OminiConnectError.Auth()
                    429 -> throw OminiConnectError.RateLimited()
                    !in 200..299 -> throw OminiConnectError.Upstream(responseCode, "")
                }
            } catch (e: OminiConnectError) {
                throw e
            } catch (e: Exception) {
                throw OminiConnectError.Network(e)
            }
        }
    }

    /**
     * Maton-style direct API call — the simplest way to use OminiConnect.
     */
    suspend fun call(
        platform: String,
        method: String,
        path: String,
        params: Map<String, String>? = null,
        body: Map<String, Any>? = null
    ): Any = withContext(scope) {
        val payload = mutableMapOf<String, Any>(
            "method" to method.uppercase(),
            "path" to path
        )
        params?.let { payload["params"] = it }
        body?.let { payload["body"] = it }

        val url = URL("$base/api/call/$platform")
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = "POST"
        conn.setRequestProperty("Authorization", "Bearer $apiKey")
        conn.setRequestProperty("Content-Type", "application/json")
        conn.doOutput = true
        conn.connectTimeout = 30000
        conn.readTimeout = 30000

        conn.outputStream.use { os ->
            os.write(JSONObject(payload).toString().toByteArray())
        }

        val responseCode = conn.responseCode
        val responseBody = conn.inputStream.use { it.bufferedReader().readText() }

        when (responseCode) {
            401 -> throw OminiConnectError.Auth()
            429 -> throw OminiConnectError.RateLimited()
            !in 200..299 -> throw OminiConnectError.Upstream(responseCode, responseBody)
        }

        return@withContext JSONObject(responseBody)
    }

    // JSON parsing helpers
    fun parseConnector(json: JSONObject) = Connector(
        platform = json.getString("platform"),
        enabled = json.getBoolean("enabled"),
        scopes = json.getJSONArray("scopes").let { arr -> (0 until arr.length()).map { arr.getString(it) } },
        createdAt = json.getString("created_at")
    )

    fun parseToolkitsResponse(json: JSONObject): ToolkitsResponse {
        val toolkitsArray = json.getJSONArray("toolkits")
        val toolkits = (0 until toolkitsArray.length()).map { parseToolkit(toolkitsArray.getJSONObject(it)) }
        return ToolkitsResponse(toolkits)
    }

    private fun parseToolkit(json: JSONObject) = Toolkit(
        slug = json.getString("slug"),
        name = json.getString("name"),
        logo = if (json.has("logo") && !json.isNull("logo")) json.getString("logo") else null,
        provider = json.getString("provider"),
        tools = json.getJSONArray("tools").let { arr -> (0 until arr.length()).map { parseToolSummary(arr.getJSONObject(it)) } }
    )

    private fun parseToolSummary(json: JSONObject) = ToolSummary(
        slug = json.getString("slug"),
        name = json.getString("name"),
        description = json.getString("description"),
        method = json.getString("method"),
        endpoint = json.getString("endpoint"),
        scopes = json.getJSONArray("scopes").let { arr -> (0 until arr.length()).map { arr.getString(it) } },
        scopeSatisfied = json.getString("scope_satisfied"),
        tags = json.getJSONArray("tags").let { arr -> (0 until arr.length()).map { arr.getString(it) } }
    )

    fun parseToolsSearchResponse(json: JSONObject) = ToolsSearchResponse(
        tools = json.getJSONArray("tools").let { arr -> (0 until arr.length()).map { parseToolSummary(arr.getJSONObject(it)) } },
        query = json.getString("query")
    )

    fun parseToolExecuteResult(json: JSONObject) = ToolExecuteResult(
        ok = json.getBoolean("ok"),
        body = if (json.has("body") && !json.isNull("body")) json.get("body") else null,
        error = if (json.has("error") && !json.isNull("error")) json.getString("error") else null,
        callId = if (json.has("call_id") && !json.isNull("call_id")) json.getString("call_id") else null,
        status = if (json.has("status") && !json.isNull("status")) json.getString("status") else null,
        durationMs = if (json.has("duration_ms") && !json.isNull("duration_ms")) json.getLong("duration_ms") else null
    )

    fun parseApiKeyCreated(json: JSONObject) = ApiKeyCreated(
        key = json.getString("key"),
        label = json.getString("label"),
        createdAt = json.getString("created_at")
    )

    fun parseApiKeySummary(json: JSONObject) = ApiKeySummary(
        keyHash = json.getString("key_hash"),
        label = json.getString("label"),
        createdAt = json.getString("created_at")
    )

    fun parseConnectorList(json: Any?): List<Connector> {
        if (json == null) return emptyList()
        val arr = json as? JSONArray ?: return emptyList()
        return (0 until arr.length()).map { parseConnector(arr.getJSONObject(it)) }
    }

    fun parseApiKeySummaryList(json: Any?): List<ApiKeySummary> {
        if (json == null) return emptyList()
        val arr = json as? JSONArray ?: return emptyList()
        return (0 until arr.length()).map { parseApiKeySummary(arr.getJSONObject(it)) }
    }
}

// ─── Connectors ───────────────────────────────────────────────────────────────

class ConnectorsManager(private val client: OminiConnect) {
    suspend fun list(): List<Connector> {
        val json = client.request<Map<String, Any>>("GET", "/api/connectors")
        val arr = json["data"] as? JSONArray ?: JSONArray()
        return (0 until arr.length()).map { client.parseConnector(arr.getJSONObject(it)) }
    }

    suspend fun delete(platform: String) {
        client.requestNoContent("DELETE", "/api/connectors/$platform")
    }
}

// ─── API Keys ────────────────────────────────────────────────────────────────

class ApiKeysManager(private val client: OminiConnect) {
    suspend fun create(label: String): ApiKeyCreated {
        val json = client.request<Map<String, Any>>("POST", "/auth/apikey", body = mapOf("label" to label))
        return client.parseApiKeyCreated(JSONObject(json))
    }

    suspend fun list(): List<ApiKeySummary> {
        val json = client.request<Map<String, Any>>("GET", "/auth/apikey")
        return client.parseApiKeySummaryList(json["data"])
    }

    suspend fun delete(keyHash: String) {
        client.requestNoContent("DELETE", "/auth/apikey/$keyHash")
    }
}

// ─── Tools ─────────────────────────────────────────────────────────────────

class ToolsManager(private val client: OminiConnect) {
    suspend fun list(platform: String? = null): ToolkitsResponse {
        val params = platform?.let { mapOf("platform" to it) }
        val json = client.request<Map<String, Any>>("GET", "/api/tools", params = params)
        return client.parseToolkitsResponse(JSONObject(json))
    }

    suspend fun search(q: String, platform: String? = null, filterScope: String? = null): ToolsSearchResponse {
        val params = mutableMapOf("q" to q)
        platform?.let { params["platform"] = it }
        filterScope?.let { params["filter_scope"] = it }
        val json = client.request<Map<String, Any>>("GET", "/api/tools/search", params = params)
        return client.parseToolsSearchResponse(JSONObject(json))
    }

    suspend fun execute(
        toolSlug: String,
        platform: String,
        arguments: Map<String, Any>? = null,
        callbackUrl: String? = null
    ): ToolExecuteResult {
        val body = mutableMapOf<String, Any>(
            "tool_slug" to toolSlug,
            "platform" to platform,
            "arguments" to (arguments ?: emptyMap<String, Any>())
        )
        callbackUrl?.let { body["callback_url"] = it }
        val json = client.request<Map<String, Any>>("POST", "/api/tools/execute", body = body)
        return client.parseToolExecuteResult(JSONObject(json))
    }
}
