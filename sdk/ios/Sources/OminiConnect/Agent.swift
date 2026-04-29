import Foundation

// ─── Agent Types ───────────────────────────────────────────────────────────────

public struct Agent: Codable {
    public let id: String
    public let name: String
    public let description: String?
    public let apiKey: String?
    public let createdAt: String
    public let enabled: Bool

    enum CodingKeys: String, CodingKey {
        case id, name, description
        case apiKey = "api_key"
        case createdAt = "created_at"
        case enabled
    }

    public init(id: String, name: String, description: String?, apiKey: String?, createdAt: String, enabled: Bool) {
        self.id = id
        self.name = name
        self.description = description
        self.apiKey = apiKey
        self.createdAt = createdAt
        self.enabled = enabled
    }
}

public struct AgentCreated: Codable {
    public let id: String
    public let name: String
    public let description: String?
    public let apiKey: String
    public let createdAt: String
    public let enabled: Bool

    enum CodingKeys: String, CodingKey {
        case id, name, description
        case apiKey = "api_key"
        case createdAt = "created_at"
        case enabled
    }

    public init(id: String, name: String, description: String?, apiKey: String, createdAt: String, enabled: Bool) {
        self.id = id
        self.name = name
        self.description = description
        self.apiKey = apiKey
        self.createdAt = createdAt
        self.enabled = enabled
    }
}
