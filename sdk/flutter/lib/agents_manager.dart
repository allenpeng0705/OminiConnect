part of 'ominiconnect.dart';

/// An AI agent registered with OminiConnect.
class Agent {
  final String id;
  final String name;
  final String? description;
  final String? apiKey;
  final DateTime createdAt;
  final bool enabled;

  Agent({
    required this.id,
    required this.name,
    this.description,
    this.apiKey,
    required this.createdAt,
    required this.enabled,
  });

  factory Agent.fromJson(Map<String, dynamic> json) {
    return Agent(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      apiKey: json['api_key'] as String?,
      createdAt: DateTime.parse(json['created_at'] as String),
      enabled: json['enabled'] as bool,
    );
  }
}

/// Response from agent registration — includes the raw API key (shown once).
class AgentCreated {
  final String id;
  final String name;
  final String? description;
  final String apiKey;
  final DateTime createdAt;
  final bool enabled;

  AgentCreated({
    required this.id,
    required this.name,
    this.description,
    required this.apiKey,
    required this.createdAt,
    required this.enabled,
  });

  factory AgentCreated.fromJson(Map<String, dynamic> json) {
    return AgentCreated(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      apiKey: json['api_key'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      enabled: json['enabled'] as bool,
    );
  }
}

/// Manager for AI agents.
class AgentsManager {
  final OminiConnect _client;
  final String _base;

  AgentsManager(this._client, this._base);

  /// Register a new agent and get its API key.
  /// The raw key is returned ONLY here — store securely.
  Future<AgentCreated> register(String name, {String? description}) async {
    final body = <String, dynamic>{'name': name};
    if (description != null) body['description'] = description;
    final data = await _client._request('POST', '/api/agents', body: body);
    return AgentCreated.fromJson(data as Map<String, dynamic>);
  }

  /// List all agents for the authenticated user.
  Future<List<Agent>> list() async {
    final data = await _client._request('GET', '/api/agents');
    return (data as List<dynamic>)
        .map((a) => Agent.fromJson(a as Map<String, dynamic>))
        .toList();
  }

  /// Get a specific agent by ID.
  Future<Agent> get(String agentId) async {
    final data = await _client._request('GET', '/api/agents/$agentId');
    return Agent.fromJson(data as Map<String, dynamic>);
  }

  /// Delete an agent and revoke its API key.
  Future<void> delete(String agentId) async {
    await _client._requestNoContent('DELETE', '/api/agents/$agentId');
  }

  /// Deactivate an agent (preserves audit trail).
  Future<Agent> deactivate(String agentId) async {
    final data = await _client._request('POST', '/api/agents/$agentId/deactivate');
    return Agent.fromJson(data as Map<String, dynamic>);
  }
}
