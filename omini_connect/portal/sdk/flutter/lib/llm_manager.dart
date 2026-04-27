part of 'ominiconnect.dart';

/// Response from LLM execute.
class LlmExecuteResponse {
  final bool ok;
  final String? tool;
  final String? toolName;
  final Map<String, dynamic>? arguments;
  final String? explanation;
  final dynamic result;
  final String? error;
  final String? message;
  final List<CandidateTool>? candidates;
  final String? availableToolsHint;

  LlmExecuteResponse({
    required this.ok,
    this.tool,
    this.toolName,
    this.arguments,
    this.explanation,
    this.result,
    this.error,
    this.message,
    this.candidates,
    this.availableToolsHint,
  });

  factory LlmExecuteResponse.fromJson(Map<String, dynamic> json) {
    return LlmExecuteResponse(
      ok: json['ok'] as bool,
      tool: json['tool'] as String?,
      toolName: json['tool_name'] as String?,
      arguments: json['arguments'] != null
          ? Map<String, dynamic>.from(json['arguments'] as Map)
          : null,
      explanation: json['explanation'] as String?,
      result: json['result'],
      error: json['error'] as String?,
      message: json['message'] as String?,
      candidates: (json['candidates'] as List<dynamic>?)
          ?.map((c) => CandidateTool.fromJson(c as Map<String, dynamic>))
          .toList(),
      availableToolsHint: json['available_tools_hint'] as String?,
    );
  }
}

/// A candidate tool suggested by the LLM.
class CandidateTool {
  final String tool;
  final String name;
  final String matchReason;

  CandidateTool({
    required this.tool,
    required this.name,
    required this.matchReason,
  });

  factory CandidateTool.fromJson(Map<String, dynamic> json) {
    return CandidateTool(
      tool: json['tool'] as String,
      name: json['name'] as String,
      matchReason: json['match_reason'] as String,
    );
  }
}

/// Response from available tools list.
class LlmToolsResponse {
  final Map<String, PlatformTools> platforms;

  LlmToolsResponse({required this.platforms});

  factory LlmToolsResponse.fromJson(Map<String, dynamic> json) {
    return LlmToolsResponse(
      platforms: (json['platforms'] as Map<String, dynamic>).map(
        (key, value) => MapEntry(key, PlatformTools.fromJson(value as Map<String, dynamic>)),
      ),
    );
  }
}

/// Tools available for a platform.
class PlatformTools {
  final bool connected;
  final List<AvailableTool>? tools;

  PlatformTools({required this.connected, this.tools});

  factory PlatformTools.fromJson(Map<String, dynamic> json) {
    return PlatformTools(
      connected: json['connected'] as bool,
      tools: (json['tools'] as List<dynamic>?)
          ?.map((t) => AvailableTool.fromJson(t as Map<String, dynamic>))
          .toList(),
    );
  }
}

/// An available tool for LLM selection.
class AvailableTool {
  final String slug;
  final String name;
  final String description;
  final List<String> exampleQueries;
  final List<String> scopes;
  final String scopeSatisfied;

  AvailableTool({
    required this.slug,
    required this.name,
    required this.description,
    required this.exampleQueries,
    required this.scopes,
    required this.scopeSatisfied,
  });

  factory AvailableTool.fromJson(Map<String, dynamic> json) {
    return AvailableTool(
      slug: json['slug'] as String,
      name: json['name'] as String,
      description: json['description'] as String,
      exampleQueries: List<String>.from(json['example_queries'] ?? []),
      scopes: List<String>.from(json['scopes'] ?? []),
      scopeSatisfied: json['scope_satisfied'] as String,
    );
  }
}

/// Manager for LLM-powered operations.
class LlmManager {
  final OminiConnect _client;
  final String _base;

  LlmManager(this._client, this._base);

  /// Execute an LLM query and return the suggested tool call.
  Future<LlmExecuteResponse> execute(String query, {String? platform}) async {
    final body = <String, dynamic>{'query': query};
    if (platform != null) body['platform'] = platform;
    final data = await _client._request('POST', '/api/llm', body: body);
    return LlmExecuteResponse.fromJson(data as Map<String, dynamic>);
  }

  /// List available tools for LLM selection, optionally filtered by platform.
  Future<LlmToolsResponse> listAvailableTools({String? platform}) async {
    final data = await _client._request(
      'GET',
      '/api/llm/tools',
      params: platform != null ? {'platform': platform} : null,
    );
    return LlmToolsResponse.fromJson(data as Map<String, dynamic>);
  }
}