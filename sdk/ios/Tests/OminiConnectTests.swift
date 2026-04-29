import XCTest
@testable import OminiConnect

// MARK: - Mock URLProtocol

final class MockURLProtocol: URLProtocol {
    static var mockResponse: (data: Data, response: HTTPURLResponse)?
    static var mockError: Error?

    override class func canInit(with request: URLRequest) -> Bool { true }
    override class func canonicalRequest(for request: URLRequest) -> URLRequest { request }

    override func startLoading() {
        if let error = MockURLProtocol.mockError {
            client?.urlProtocol(self, didFailWithError: error)
            return
        }
        guard let mock = MockURLProtocol.mockResponse else {
            client?.urlProtocol(self, didFailWithError: URLError(.unknown))
            return
        }
        client?.urlProtocol(self, didReceive: mock.response, cacheStoragePolicy: .notAllowed)
        client?.urlProtocol(self, didLoad: mock.data)
        client?.urlProtocolDidFinishLoading(self)
    }

    override func stopLoading() {}
}

// MARK: - Tests

final class OminiConnectTests: XCTestCase {

    private var client: OminiConnect!

    override func setUp() async throws {
        try await super.setUp()
        client = OminiConnect(apiKey: "test-key-123", baseURL: "http://test.local")
        XCTAssertNotNil(client.connectors)
        XCTAssertNotNil(client.tools)
        XCTAssertNotNil(client.apiKeys)
        XCTAssertNotNil(client.llm)
    }

    override func tearDown() {
        MockURLProtocol.mockResponse = nil
        MockURLProtocol.mockError = nil
        client = nil
    }

    // MARK: Client Initialization

    func testClientInitialization() async throws {
        let c = OminiConnect(apiKey: "sk-test", baseURL: "http://localhost:9000")
        XCTAssertNotNil(c.connectors)
        XCTAssertNotNil(c.tools)
        XCTAssertNotNil(c.apiKeys)
        XCTAssertNotNil(c.llm)
    }

    func testClientInitializationStripsTrailingSlash() async throws {
        let c = OminiConnect(apiKey: "sk-test", baseURL: "http://localhost:9000/")
        XCTAssertNotNil(c.connectors)
    }

    // MARK: ConnectorsManager

    func testConnectorsListParsesResponse() async throws {
        let json = """
        {"platform": "github", "enabled": true, "scopes": ["repo", "user"], "created_at": "2026-01-01T00:00:00Z"}
        """.data(using: .utf8)!
        let connector = try JSONDecoder().decode(Connector.self, from: json)
        XCTAssertEqual(connector.platform, "github")
        XCTAssertEqual(connector.enabled, true)
        XCTAssertEqual(connector.scopes, ["repo", "user"])
    }

    // MARK: ToolsManager

    func testToolsManagerListParsesResponse() async throws {
        let json = """
        {"toolkits": [{"slug": "github", "name": "GitHub", "logo": null, "provider": "github", "tools": []}]}
        """.data(using: .utf8)!
        let resp = try JSONDecoder().decode(ToolkitsResponse.self, from: json)
        XCTAssertEqual(resp.toolkits.count, 1)
        XCTAssertEqual(resp.toolkits[0].slug, "github")
    }

    func testToolsManagerExecuteParsesResult() async throws {
        let json = """
        {"ok": true, "body": {"name": "test-repo"}, "call_id": "call-123", "status": "completed", "duration_ms": 50}
        """.data(using: .utf8)!
        let result = try JSONDecoder().decode(ToolExecuteResult.self, from: json)
        XCTAssertTrue(result.ok)
        XCTAssertEqual(result.callId, "call-123")
        XCTAssertEqual(result.status, "completed")
        XCTAssertEqual(result.durationMs, 50)
    }

    // MARK: ApiKeysManager

    func testApiKeyCreatedParsing() async throws {
        let json = """
        {"key": "sk-live-abc123", "label": "test-key", "created_at": "2026-01-01T00:00:00Z"}
        """.data(using: .utf8)!
        let key = try JSONDecoder().decode(ApiKeyCreated.self, from: json)
        XCTAssertEqual(key.key, "sk-live-abc123")
        XCTAssertEqual(key.label, "test-key")
    }

    func testApiKeySummaryParsing() async throws {
        let json = """
        {"key_hash": "abc123hash", "label": "test-key", "created_at": "2026-01-01T00:00:00Z"}
        """.data(using: .utf8)!
        let key = try JSONDecoder().decode(ApiKeySummary.self, from: json)
        XCTAssertEqual(key.keyHash, "abc123hash")
        XCTAssertEqual(key.label, "test-key")
    }

    // MARK: Error Handling

    func testOminiConnectErrorDescriptions() {
        XCTAssertEqual(OminiConnectError.auth.errorDescription, "Invalid or missing API key")
        XCTAssertEqual(OminiConnectError.connectorNotFound("github").errorDescription, "Platform not connected: github")
        XCTAssertEqual(OminiConnectError.toolNotFound("github_list_repos").errorDescription, "Tool not found: github_list_repos")
        XCTAssertEqual(OminiConnectError.scopeInsufficient.errorDescription, "Insufficient scopes for this operation")
        XCTAssertEqual(OminiConnectError.upstream(statusCode: 500, body: "server error").errorDescription, "Upstream error 500: server error")
        XCTAssertEqual(OminiConnectError.rateLimited.errorDescription, "Rate limited — back off and retry")
    }

    func testAnyCodableValueDecodesVariousTypes() async throws {
        // Int
        let intData = "42".data(using: .utf8)!
        let intVal = try JSONDecoder().decode(AnyCodableValue.self, from: intData)
        XCTAssertEqual(intVal.value as? Int, 42)

        // String
        let strData = "\"hello\"".data(using: .utf8)!
        let strVal = try JSONDecoder().decode(AnyCodableValue.self, from: strData)
        XCTAssertEqual(strVal.value as? String, "hello")

        // Bool
        let boolData = "true".data(using: .utf8)!
        let boolVal = try JSONDecoder().decode(AnyCodableValue.self, from: boolData)
        XCTAssertEqual(boolVal.value as? Bool, true)

        // Double
        let doubleData = "3.14".data(using: .utf8)!
        let doubleVal = try JSONDecoder().decode(AnyCodableValue.self, from: doubleData)
        XCTAssertEqual(doubleVal.value as? Double, 3.14)
    }
}
