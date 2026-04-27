// OminiConnect iOS SDK - LLM Manager
// Query LLMs with tool execution support

import Foundation

// ─── LLM Types ────────────────────────────────────────────────────────────────

public struct LlmExecuteResponse: Codable {
    public let ok: Bool
    public let tool: String?
    public let toolName: String?
    public let arguments: [String: LlmAnyCodable]?
    public let explanation: String?
    public let result: LlmAnyCodable?
    public let error: String?
    public let message: String?
    public let candidates: [CandidateTool]?
    public let availableToolsHint: String?

    public init(
        ok: Bool, tool: String?, toolName: String?, arguments: [String: LlmAnyCodable]?,
        explanation: String?, result: LlmAnyCodable?, error: String?, message: String?,
        candidates: [CandidateTool]?, availableToolsHint: String?
    ) {
        self.ok = ok
        self.tool = tool
        self.toolName = toolName
        self.arguments = arguments
        self.explanation = explanation
        self.result = result
        self.error = error
        self.message = message
        self.candidates = candidates
        self.availableToolsHint = availableToolsHint
    }
}

public struct CandidateTool: Codable {
    public let tool: String
    public let name: String
    public let matchReason: String

    public init(tool: String, name: String, matchReason: String) {
        self.tool = tool
        self.name = name
        self.matchReason = matchReason
    }
}

public struct LlmToolsResponse: Codable {
    public let platforms: [String: PlatformTools]

    public init(platforms: [String: PlatformTools]) {
        self.platforms = platforms
    }
}

public struct PlatformTools: Codable {
    public let connected: Bool
    public let tools: [AvailableTool]?

    public init(connected: Bool, tools: [AvailableTool]?) {
        self.connected = connected
        self.tools = tools
    }
}

public struct AvailableTool: Codable {
    public let slug: String
    public let name: String
    public let description: String
    public let exampleQueries: [String]
    public let scopes: [String]
    public let scopeSatisfied: String

    enum CodingKeys: String, CodingKey {
        case slug, name, description, exampleQueries, scopes
        case scopeSatisfied = "scope_satisfied"
    }

    public init(slug: String, name: String, description: String, exampleQueries: [String], scopes: [String], scopeSatisfied: String) {
        self.slug = slug
        self.name = name
        self.description = description
        self.exampleQueries = exampleQueries
        self.scopes = scopes
        self.scopeSatisfied = scopeSatisfied
    }
}

/// Type-erased Codable value for flexible body types (encode/decode both directions)
public struct LlmAnyCodable: Codable {
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
        } else if let arrayVal = try? container.decode([LlmAnyCodable].self) {
            value = arrayVal.map { $0.value }
        } else if let dictVal = try? container.decode([String: LlmAnyCodable].self) {
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
        } else if let arrayVal = value as? [Any] {
            try container.encode(arrayVal.map { LlmAnyCodable($0) })
        } else if let dictVal = value as? [String: Any] {
            try container.encode(dictVal.mapValues { LlmAnyCodable($0) })
        } else {
            try container.encodeNil()
        }
    }
}

// ─── LLM Manager ──────────────────────────────────────────────────────────────

public class LlmManager {
    private let client: OminiConnect

    public init(client: OminiConnect) {
        self.client = client
    }

    /// Execute an LLM query with optional platform context.
    /// The query is processed by the OminiConnect LLM engine which may:
    /// - Return a direct answer
    /// - Suggest tool calls with arguments
    /// - Execute a tool and return the result
    public func execute(query: String, platform: String? = nil) async throws -> LlmExecuteResponse {
        struct ExecuteRequest: Encodable {
            let query: String
            let platform: String?
        }
        let payload = ExecuteRequest(query: query, platform: platform)
        return try await client.request("POST", "/api/llm", body: payload)
    }

    /// List available tools for LLM context, optionally filtered by platform.
    /// Returns platform info with connected status and available tools.
    public func listAvailableTools(platform: String? = nil) async throws -> LlmToolsResponse {
        var params: [String: String]? = nil
        if let p = platform {
            params = ["platform": p]
        }
        return try await client.request("GET", "/api/llm/tools", params: params)
    }
}