// OminiConnect iOS SDK
// Connect AI agents to external platforms via OAuth

import Foundation

// ─── Errors ─────────────────────────────────────────────────────────────────

public enum OminiConnectError: Error, LocalizedError {
    case auth
    case connectorNotFound(String)
    case toolNotFound(String)
    case scopeInsufficient
    case upstream(statusCode: Int, body: String)
    case network(Error)
    case rateLimited

    public var errorDescription: String? {
        switch self {
        case .auth: return "Invalid or missing API key"
        case .connectorNotFound(let platform): return "Platform not connected: \(platform)"
        case .toolNotFound(let slug): return "Tool not found: \(slug)"
        case .scopeInsufficient: return "Insufficient scopes for this operation"
        case .upstream(let code, let body): return "Upstream error \(code): \(body)"
        case .network(let err): return "Network error: \(err.localizedDescription)"
        case .rateLimited: return "Rate limited — back off and retry"
        }
    }
}

// ─── Types ──────────────────────────────────────────────────────────────────

public struct Connector: Codable {
    public let platform: String
    public let enabled: Bool
    public let scopes: [String]
    public let createdAt: String

    enum CodingKeys: String, CodingKey {
        case platform, enabled, scopes
        case createdAt = "created_at"
    }

    public init(platform: String, enabled: Bool, scopes: [String], createdAt: String) {
        self.platform = platform
        self.enabled = enabled
        self.scopes = scopes
        self.createdAt = createdAt
    }
}

public struct ApiKeyCreated: Codable {
    public let key: String
    public let label: String
    public let createdAt: String

    enum CodingKeys: String, CodingKey {
        case key, label
        case createdAt = "created_at"
    }

    public init(key: String, label: String, createdAt: String) {
        self.key = key
        self.label = label
        self.createdAt = createdAt
    }
}

public struct ApiKeySummary: Codable {
    public let keyHash: String
    public let label: String
    public let createdAt: String

    enum CodingKeys: String, CodingKey {
        case keyHash = "key_hash"
        case label
        case createdAt = "created_at"
    }

    public init(keyHash: String, label: String, createdAt: String) {
        self.keyHash = keyHash
        self.label = label
        self.createdAt = createdAt
    }
}

public struct ToolSummary: Codable {
    public let slug: String
    public let name: String
    public let description: String
    public let method: String
    public let endpoint: String
    public let scopes: [String]
    public let scopeSatisfied: String
    public let tags: [String]

    enum CodingKeys: String, CodingKey {
        case slug, name, description, method, endpoint, scopes, tags
        case scopeSatisfied = "scope_satisfied"
    }

    public init(slug: String, name: String, description: String, method: String, endpoint: String, scopes: [String], scopeSatisfied: String, tags: [String]) {
        self.slug = slug
        self.name = name
        self.description = description
        self.method = method
        self.endpoint = endpoint
        self.scopes = scopes
        self.scopeSatisfied = scopeSatisfied
        self.tags = tags
    }
}

public struct Toolkit: Codable {
    public let slug: String
    public let name: String
    public let logo: String?
    public let provider: String
    public let tools: [ToolSummary]

    public init(slug: String, name: String, logo: String?, provider: String, tools: [ToolSummary]) {
        self.slug = slug
        self.name = name
        self.logo = logo
        self.provider = provider
        self.tools = tools
    }
}

public struct ToolkitsResponse: Codable {
    public let toolkits: [Toolkit]

    public init(toolkits: [Toolkit]) {
        self.toolkits = toolkits
    }
}

public struct ToolsSearchResponse: Codable {
    public let tools: [ToolSummary]
    public let query: String

    public init(tools: [ToolSummary], query: String) {
        self.tools = tools
        self.query = query
    }
}

public struct ToolExecuteResult: Codable {
    public let ok: Bool
    public let body: AnyCodableValue?
    public let error: String?
    public let callId: String?
    public let status: String?
    public let durationMs: Int?

    enum CodingKeys: String, CodingKey {
        case ok, body, error
        case callId = "call_id"
        case status
        case durationMs = "duration_ms"
    }

    public init(ok: Bool, body: AnyCodableValue?, error: String?, callId: String?, status: String?, durationMs: Int?) {
        self.ok = ok
        self.body = body
        self.error = error
        self.callId = callId
        self.status = status
        self.durationMs = durationMs
    }
}

/// Type-erased Codable value for flexible body types
public struct AnyCodableValue: Codable {
    public let value: Any

    public init(_ value: Any) {
        self.value = value
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let intVal = try? container.decode(Int.self) {
            value = intVal
        } else if let doubleVal = try? container.decode(Double.self) {
            value = doubleVal
        } else if let stringVal = try? container.decode(String.self) {
            value = stringVal
        } else if let boolVal = try? container.decode(Bool.self) {
            value = boolVal
        } else if let arrayVal = try? container.decode([AnyCodableValue].self) {
            value = arrayVal.map { $0.value }
        } else if let dictVal = try? container.decode([String: AnyCodableValue].self) {
            value = dictVal.mapValues { $0.value }
        } else {
            value = NSNull()
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        if let intVal = value as? Int {
            try container.encode(intVal)
        } else if let doubleVal = value as? Double {
            try container.encode(doubleVal)
        } else if let stringVal = value as? String {
            try container.encode(stringVal)
        } else if let boolVal = value as? Bool {
            try container.encode(boolVal)
        } else {
            try container.encodeNil()
        }
    }
}

// ─── Client ─────────────────────────────────────────────────────────────────

public class OminiConnect {
    private let baseURL: String
    private let apiKey: String
    private let session: URLSession

    public var connectors: ConnectorsManager!
    public var tools: ToolsManager!
    public var apiKeys: ApiKeysManager!
    public var llm: LlmManager!
    public var agents: AgentsManager!

    public init(apiKey: String, baseURL: String = "http://localhost:9000") {
        self.apiKey = apiKey
        self.baseURL = baseURL.trimmingCharacters(in: CharacterSet(charactersIn: "/"))
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        self.session = URLSession(configuration: config)

        // Swift requires all stored properties initialized before self can be used.
        // Use nil placeholder initializers then assign via a helper.
        self.connectors = nil
        self.tools = nil
        self.apiKeys = nil
        self.llm = nil
        self.agents = nil

        // Now assign properly (self is fully initialized now)
        self.connectors = ConnectorsManager(client: self)
        self.tools = ToolsManager(client: self)
        self.apiKeys = ApiKeysManager(client: self)
        self.llm = LlmManager(client: self)
        self.agents = AgentsManager(client: self)
    }

    // ─── HTTP layer ───────────────────────────────────────────────────────────

    func request<T: Decodable>(
        _ method: String,
        _ path: String,
        body: Encodable? = nil,
        params: [String: String]? = nil
    ) async throws -> T {
        var urlComponents = URLComponents(string: baseURL + path)!
        if let params = params, !params.isEmpty {
            urlComponents.queryItems = params.map { URLQueryItem(name: $0.key, value: $0.value) }
        }

        var request = URLRequest(url: urlComponents.url!)
        request.httpMethod = method
        request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        if let body = body {
            let encoder = JSONEncoder()
            request.httpBody = try encoder.encode(body)
        }

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw OminiConnectError.network(URLError(.badServerResponse))
        }

        if httpResponse.statusCode == 401 {
            throw OminiConnectError.auth
        }
        if httpResponse.statusCode == 429 {
            throw OminiConnectError.rateLimited
        }

        if !(200...299).contains(httpResponse.statusCode) {
            let bodyStr = String(data: data, encoding: .utf8) ?? ""
            throw OminiConnectError.upstream(statusCode: httpResponse.statusCode, body: bodyStr)
        }

        if httpResponse.statusCode == 204 || data.isEmpty {
            throw OminiConnectError.network(URLError(.cannotDecodeContentData))
        }

        let decoder = JSONDecoder()
        return try decoder.decode(T.self, from: data)
    }

    func requestNoContent(
        _ method: String,
        _ path: String,
        body: Encodable? = nil
    ) async throws {
        var urlComponents = URLComponents(string: baseURL + path)!

        var request = URLRequest(url: urlComponents.url!)
        request.httpMethod = method
        request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        if let body = body {
            let encoder = JSONEncoder()
            request.httpBody = try encoder.encode(body)
        }

        let (_, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw OminiConnectError.network(URLError(.badServerResponse))
        }

        if httpResponse.statusCode == 401 {
            throw OminiConnectError.auth
        }
        if httpResponse.statusCode == 429 {
            throw OminiConnectError.rateLimited
        }

        if !(200...299).contains(httpResponse.statusCode) {
            throw OminiConnectError.upstream(statusCode: httpResponse.statusCode, body: "")
        }
    }

    /// Maton-style direct API call — the simplest way to use OminiConnect.
    public func call(
        platform: String,
        method: String,
        path: String,
        params: [String: String]? = nil,
        body: [String: Any]? = nil
    ) async throws -> Any {
        var payload: [String: Any] = ["method": method.uppercased(), "path": path]
        if let params = params {
            payload["params"] = params
        }
        if let body = body {
            payload["body"] = body
        }

        let url = URL(string: "\(baseURL)/api/call/\(platform)")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONSerialization.data(withJSONObject: payload)

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw OminiConnectError.network(URLError(.badServerResponse))
        }

        if httpResponse.statusCode == 401 {
            throw OminiConnectError.auth
        }
        if httpResponse.statusCode == 429 {
            throw OminiConnectError.rateLimited
        }

        if !(200...299).contains(httpResponse.statusCode) {
            let bodyStr = String(data: data, encoding: .utf8) ?? ""
            throw OminiConnectError.upstream(statusCode: httpResponse.statusCode, body: bodyStr)
        }

        return try JSONSerialization.jsonObject(with: data)
    }
}

// ─── Connectors ───────────────────────────────────────────────────────────────

public class ConnectorsManager {
    private let client: OminiConnect

    init(client: OminiConnect) {
        self.client = client
    }

    /// List all connected platforms.
    public func list() async throws -> [Connector] {
        try await client.request("GET", "/api/connectors")
    }

    /// Delete a connected platform.
    public func delete(platform: String) async throws {
        try await client.requestNoContent("DELETE", "/api/connectors/\(platform)")
    }
}

// ─── API Keys ────────────────────────────────────────────────────────────────

private struct CreateApiKeyRequest: Encodable {
    let label: String
}

public class ApiKeysManager {
    private let client: OminiConnect

    init(client: OminiConnect) {
        self.client = client
    }

    /// Create a new named API key. The raw key is returned ONLY here — store securely.
    public func create(label: String) async throws -> ApiKeyCreated {
        try await client.request("POST", "/auth/apikey", body: CreateApiKeyRequest(label: label))
    }

    /// List all API keys (raw key is never returned).
    public func list() async throws -> [ApiKeySummary] {
        try await client.request("GET", "/auth/apikey")
    }

    /// Revoke an API key.
    public func delete(keyHash: String) async throws {
        try await client.requestNoContent("DELETE", "/auth/apikey/\(keyHash)")
    }
}

// ─── Tools ──────────────────────────────────────────────────────────────────

public class ToolsManager {
    private let client: OminiConnect

    init(client: OminiConnect) {
        self.client = client
    }

    /// List available tools, optionally filtered by platform.
    public func list(platform: String? = nil) async throws -> ToolkitsResponse {
        var params: [String: String]? = nil
        if let p = platform {
            params = ["platform": p]
        }
        return try await client.request("GET", "/api/tools", params: params)
    }

    /// Search tools by query string.
    public func search(q: String, platform: String? = nil, filterScope: String? = nil) async throws -> ToolsSearchResponse {
        var params: [String: String] = ["q": q]
        if let p = platform { params["platform"] = p }
        if let f = filterScope { params["filter_scope"] = f }
        return try await client.request("GET", "/api/tools/search", params: params)
    }

    /// Execute a tool by slug with structured arguments.
    /// Set callbackURL for async execution (returns immediately).
    public func execute(
        toolSlug: String,
        platform: String,
        arguments: [String: Any]? = nil,
        callbackURL: String? = nil
    ) async throws -> ToolExecuteResult {
        struct ExecuteRequest: Encodable {
            let toolSlug: String
            let platform: String
            let arguments: [String: AnyCodable]
            let callbackURL: String?

            enum CodingKeys: String, CodingKey {
                case toolSlug = "tool_slug"
                case platform
                case arguments
                case callbackURL = "callback_url"
            }
        }

        var argsDict: [String: AnyCodable] = [:]
        if let arguments = arguments {
            for (k, v) in arguments {
                argsDict[k] = AnyCodable(v)
            }
        }

        let payload = ExecuteRequest(
            toolSlug: toolSlug,
            platform: platform,
            arguments: argsDict,
            callbackURL: callbackURL
        )

        return try await client.request("POST", "/api/tools/execute", body: payload)
    }
}

/// Helper for encoding arbitrary values
private struct AnyCodable: Encodable {
    let value: Any

    init(_ value: Any) {
        self.value = value
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        if let intVal = value as? Int {
            try container.encode(intVal)
        } else if let doubleVal = value as? Double {
            try container.encode(doubleVal)
        } else if let stringVal = value as? String {
            try container.encode(stringVal)
        } else if let boolVal = value as? Bool {
            try container.encode(boolVal)
        } else {
            try container.encodeNil()
        }
    }
}
