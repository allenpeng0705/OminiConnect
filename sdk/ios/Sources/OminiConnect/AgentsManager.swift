import Foundation

// ─── Agents ─────────────────────────────────────────────────────────────────

public class AgentsManager {
    private let client: OminiConnect

    init(client: OminiConnect) {
        self.client = client
    }

    /// Register a new agent and get its API key.
    /// The raw key is returned ONLY here — store securely.
    public func register(name: String, description: String? = nil) async throws -> AgentCreated {
        struct RegisterRequest: Encodable {
            let name: String
            let description: String?
        }

        let payload = RegisterRequest(name: name, description: description)
        return try await client.request("POST", "/api/agents", body: payload)
    }

    /// List all agents for the authenticated user.
    public func list() async throws -> [Agent] {
        try await client.request("GET", "/api/agents")
    }

    /// Get a specific agent by ID.
    public func get(agentId: String) async throws -> Agent {
        try await client.request("GET", "/api/agents/\(agentId)")
    }

    /// Delete an agent and revoke its API key.
    public func delete(agentId: String) async throws {
        try await client.requestNoContent("DELETE", "/api/agents/\(agentId)")
    }

    /// Deactivate an agent (preserves audit trail).
    public func deactivate(agentId: String) async throws -> Agent {
        try await client.request("POST", "/api/agents/\(agentId)/deactivate")
    }
}
