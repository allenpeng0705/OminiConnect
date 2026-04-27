package ai.ominiconnect

import kotlinx.coroutines.test.runTest
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import org.junit.Assert.*
import org.junit.Test

class OminiConnectTest {

    @Test
    fun testClientInitialization() {
        val client = OminiConnect("test-key")
        assertNotNull(client.connectors)
        assertNotNull(client.tools)
        assertNotNull(client.apiKeys)
    }

    @Test
    fun testClientCustomBaseUrl() {
        val client = OminiConnect("test-key", "https://api.example.com")
        assertNotNull(client.connectors)
    }

    @Test
    fun testListConnectors() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""[{"platform": "github", "enabled": true, "scopes": ["repo"], "created_at": "2024-01-01"}]""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val connectors = client.connectors.list()

        assertEquals(1, connectors.size)
        assertEquals("github", connectors[0].platform)
        assertTrue(connectors[0].enabled)

        server.shutdown()
    }

    @Test
    fun testDeleteConnector() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse().setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        client.connectors.delete("github")

        val request = server.takeRequest()
        assertEquals("DELETE", request.method)
        assertEquals("/api/connectors/github", request.path)

        server.shutdown()
    }

    @Test
    fun testListTools() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""
                {"toolkits": [{"slug": "github", "name": "GitHub", "provider": "github", "tools": []}]}
            """.trimIndent())
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.tools.list()

        assertEquals(1, result.toolkits.size)
        assertEquals("github", result.toolkits[0].slug)

        server.shutdown()
    }

    @Test
    fun testSearchTools() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""
                {"tools": [{"slug": "github_list_repos", "name": "List Repos", "description": "desc", "method": "GET", "endpoint": "/repos", "scopes": [], "scopeSatisfied": "yes", "tags": []}], "query": "list repos"}
            """.trimIndent())
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.tools.search("list repos")

        assertEquals(1, result.tools.size)
        assertEquals("github_list_repos", result.tools[0].slug)
        assertEquals("list repos", result.query)

        server.shutdown()
    }

    @Test
    fun testExecuteTool() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""{"ok": true, "body": {"id": 1}, "call_id": "call_123", "status": "completed", "duration_ms": 234}""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.tools.execute("github_list_repos", "github")

        assertTrue(result.ok)
        assertEquals(234L, result.durationMs)

        server.shutdown()
    }

    @Test
    fun testCreateApiKey() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""{"key": "sk_om_newkey", "label": "test-agent", "created_at": "2024-01-15T10:00:00Z"}""")
            .setResponseCode(201))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.apiKeys.create("test-agent")

        assertEquals("sk_om_newkey", result.key)
        assertEquals("test-agent", result.label)

        server.shutdown()
    }

    @Test
    fun testListApiKeys() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""[{"key_hash": "$2b$12$hash", "label": "test-agent", "created_at": "2024-01-15T10:00:00Z"}]""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.apiKeys.list()

        assertEquals(1, result.size)
        assertEquals("test-agent", result[0].label)
        // Raw key must never be in response
        assertEquals("$2b$12$hash", result[0].keyHash)

        server.shutdown()
    }

    @Test(expected = OminiConnectError.Auth::class)
    fun testAuthErrorOn401() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse().setResponseCode(401))

        val client = OminiConnect("test-key", server.url("/").toString())
        try {
            client.connectors.list()
        } finally {
            server.shutdown()
        }
    }

    @Test
    fun testCallDirectApi() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""{"login": "testuser", "id": 123}""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.call("github", "GET", "/user")

        val request = server.takeRequest()
        assertEquals("POST", request.method)
        assertEquals("/api/call/github", request.path)
        assertNotNull(result)

        server.shutdown()
    }

    @Test
    fun testLlmExecute() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""{"ok": true, "tool": "github_list_repos", "tool_name": "List Repositories", "arguments": {"sort": "updated"}}""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.llm.execute("list my github repos")

        assertTrue(result.ok)
        assertEquals("github_list_repos", result.tool)
        assertEquals("List Repositories", result.toolName)

        server.shutdown()
    }

    @Test
    fun testLlmExecuteAmbiguous() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""{"ok": false, "message": "Ambiguous query", "candidates": [{"tool": "github_list_repos", "name": "List Repositories", "match_reason": "keyword match"}], "available_tools_hint": "be more specific"}""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.llm.execute("list stuff")

        assertFalse(result.ok)
        assertNotNull(result.candidates)
        assertEquals(1, result.candidates?.size)

        server.shutdown()
    }

    @Test
    fun testLlmListAvailableTools() = runTest {
        val server = MockWebServer()
        server.start()
        server.enqueue(MockResponse()
            .setBody("""{"platforms": {"github": {"connected": true, "tools": [{"slug": "github_list_repos", "name": "List Repositories", "description": "desc", "example_queries": [], "scopes": ["repo"], "scope_satisfied": "yes"}]}, "slack": {"connected": false}} }""")
            .setResponseCode(200))

        val client = OminiConnect("test-key", server.url("/").toString())
        val result = client.llm.listAvailableTools()

        assertTrue(result.platforms["github"]?.connected == true)
        assertEquals(1, result.platforms["github"]?.tools?.size)
        assertEquals("github_list_repos", result.platforms["github"]?.tools?.get(0)?.slug)
        assertFalse(result.platforms["slack"]?.connected == true)

        server.shutdown()
    }
}
