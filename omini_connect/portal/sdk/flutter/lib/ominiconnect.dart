import 'dart:convert';
import 'package:http/http.dart' as http;

/// OminiConnect Flutter SDK.
/// Connect AI agents to external platforms via OAuth.
library ominiconnect;

/// Thrown for all OminiConnect errors.
class OminiConnectException implements Exception {
  final String message;
  final int? statusCode;
  final String? body;

  OminiConnectException(this.message, {this.statusCode, this.body});

  @override
  String toString() => 'OminiConnectException: $message';

  bool get isAuth => statusCode == 401;
  bool get isRateLimited => statusCode == 429;
  bool get isNotFound => statusCode == 404;
}

/// Raised when API key is invalid or missing.
class AuthException extends OminiConnectException {
  AuthException() : super('Invalid or missing API key', statusCode: 401);
}

/// Raised when a platform is not connected.
class ConnectorNotFoundException extends OminiConnectException {
  ConnectorNotFoundException(String platform)
      : super('Platform not connected: $platform', statusCode: 404);
}

/// Raised when a tool is not found.
class ToolNotFoundException extends OminiConnectException {
  ToolNotFoundException(String slug)
      : super('Tool not found: $slug', statusCode: 404);
}

/// Raised when scopes are insufficient.
class ScopeInsufficientException extends OminiConnectException {
  ScopeInsufficientException()
      : super('Insufficient scopes for this operation', statusCode: 403);
}

// ─── Types ──────────────────────────────────────────────────────────────────

/// A connected platform.
class Connector {
  final String platform;
  final bool enabled;
  final List<String> scopes;
  final DateTime createdAt;

  Connector({
    required this.platform,
    required this.enabled,
    required this.scopes,
    required this.createdAt,
  });

  factory Connector.fromJson(Map<String, dynamic> json) {
    return Connector(
      platform: json['platform'] as String,
      enabled: json['enabled'] as bool,
      scopes: List<String>.from(json['scopes'] ?? []),
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }
}

/// API key returned on creation — store securely, shown only once.
class ApiKeyCreated {
  final String key;
  final String label;
  final DateTime createdAt;

  ApiKeyCreated({
    required this.key,
    required this.label,
    required this.createdAt,
  });

  factory ApiKeyCreated.fromJson(Map<String, dynamic> json) {
    return ApiKeyCreated(
      key: json['key'] as String,
      label: json['label'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }
}

/// API key summary — raw key is never returned.
class ApiKeySummary {
  final String keyHash;
  final String label;
  final DateTime createdAt;

  ApiKeySummary({
    required this.keyHash,
    required this.label,
    required this.createdAt,
  });

  factory ApiKeySummary.fromJson(Map<String, dynamic> json) {
    return ApiKeySummary(
      keyHash: json['key_hash'] as String,
      label: json['label'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }
}

/// A tool from the registry.
class ToolSummary {
  final String slug;
  final String name;
  final String description;
  final String method;
  final String endpoint;
  final List<String> scopes;
  final String scopeSatisfied;
  final List<String> tags;

  ToolSummary({
    required this.slug,
    required this.name,
    required this.description,
    required this.method,
    required this.endpoint,
    required this.scopes,
    required this.scopeSatisfied,
    required this.tags,
  });

  factory ToolSummary.fromJson(Map<String, dynamic> json) {
    return ToolSummary(
      slug: json['slug'] as String,
      name: json['name'] as String,
      description: json['description'] as String,
      method: json['method'] as String,
      endpoint: json['endpoint'] as String,
      scopes: List<String>.from(json['scopes'] ?? []),
      scopeSatisfied: json['scope_satisfied'] as String,
      tags: List<String>.from(json['tags'] ?? []),
    );
  }
}

/// A toolkit grouping related tools.
class Toolkit {
  final String slug;
  final String name;
  final String? logo;
  final String provider;
  final List<ToolSummary> tools;

  Toolkit({
    required this.slug,
    required this.name,
    this.logo,
    required this.provider,
    required this.tools,
  });

  factory Toolkit.fromJson(Map<String, dynamic> json) {
    return Toolkit(
      slug: json['slug'] as String,
      name: json['name'] as String,
      logo: json['logo'] as String?,
      provider: json['provider'] as String,
      tools: (json['tools'] as List<dynamic>?)
              ?.map((t) => ToolSummary.fromJson(t as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }
}

/// Response from tools list.
class ToolkitsResponse {
  final List<Toolkit> toolkits;

  ToolkitsResponse({required this.toolkits});

  factory ToolkitsResponse.fromJson(Map<String, dynamic> json) {
    return ToolkitsResponse(
      toolkits: (json['toolkits'] as List<dynamic>?)
              ?.map((t) => Toolkit.fromJson(t as Map<String, dynamic>))
              .toList() ??
          [],
    );
  }
}

/// Response from tools search.
class ToolsSearchResponse {
  final List<ToolSummary> tools;
  final String query;

  ToolsSearchResponse({required this.tools, required this.query});

  factory ToolsSearchResponse.fromJson(Map<String, dynamic> json) {
    return ToolsSearchResponse(
      tools: (json['tools'] as List<dynamic>?)
              ?.map((t) => ToolSummary.fromJson(t as Map<String, dynamic>))
              .toList() ??
          [],
      query: json['query'] as String,
    );
  }
}

/// Result from tool execution.
class ToolExecuteResult {
  final bool ok;
  final dynamic body;
  final String? error;
  final String? callId;
  final String? status;
  final int? durationMs;

  ToolExecuteResult({
    required this.ok,
    this.body,
    this.error,
    this.callId,
    this.status,
    this.durationMs,
  });

  factory ToolExecuteResult.fromJson(Map<String, dynamic> json) {
    return ToolExecuteResult(
      ok: json['ok'] as bool,
      body: json['body'],
      error: json['error'] as String?,
      callId: json['call_id'] as String?,
      status: json['status'] as String?,
      durationMs: json['duration_ms'] as int?,
    );
  }
}

// ─── Client ─────────────────────────────────────────────────────────────────

class OminiConnect {
  final String apiKey;
  final String baseUrl;
  final http.Client _http;

  late final ConnectorsManager connectors;
  late final ToolsManager tools;
  late final ApiKeysManager apiKeys;

  OminiConnect({
    required this.apiKey,
    this.baseUrl = 'http://localhost:9000',
    http.Client? httpClient,
  }) : _http = httpClient ?? http.Client() {
    final trimmedBase = baseUrl.replaceAll(RegExp(r'/$'), '');
    connectors = ConnectorsManager(this, trimmedBase);
    tools = ToolsManager(this, trimmedBase);
    apiKeys = ApiKeysManager(this, trimmedBase);
  }

  Map<String, String> get _headers => {
        'Authorization': 'Bearer $apiKey',
        'Content-Type': 'application/json',
      };

  Future<dynamic> _request(
    String method,
    String path, {
    Map<String, dynamic>? body,
    Map<String, String>? params,
  }) async {
    var url = '$baseUrl$path';
    if (params != null && params.isNotEmpty) {
      final queryString =
          params.entries.map((e) => '${Uri.encodeComponent(e.key)}=${Uri.encodeComponent(e.value)}').join('&');
      url = '$url?$queryString';
    }

    final uri = Uri.parse(url);
    http.Response resp;

    if (body != null) {
      resp = await _http
          .request(
            method,
            uri,
            headers: _headers,
            body: jsonEncode(body),
          )
          .timeout(const Duration(seconds: 30));
    } else {
      resp = await _http
          .request(method, uri, headers: _headers)
          .timeout(const Duration(seconds: 30));
    }

    if (resp.statusCode == 401) throw AuthException();
    if (resp.statusCode == 429) throw OminiConnectException('Rate limited', statusCode: 429);
    if (resp.statusCode == 404) throw OminiConnectException('Not found', statusCode: 404);
    if (resp.statusCode >= 400) {
      throw OminiConnectException(
        'Upstream error ${resp.statusCode}',
        statusCode: resp.statusCode,
        body: resp.body,
      );
    }

    if (resp.body.isEmpty) return null;
    return jsonDecode(resp.body);
  }

  Future<void> _requestNoContent(String method, String path, {Map<String, dynamic>? body}) async {
    final uri = Uri.parse('$baseUrl$path');
    final resp = await (_http.request(
      method,
      uri,
      headers: _headers,
      body: body != null ? jsonEncode(body) : null,
    )).timeout(const Duration(seconds: 30));

    if (resp.statusCode == 401) throw AuthException();
    if (resp.statusCode == 429) throw OminiConnectException('Rate limited', statusCode: 429);
    if (resp.statusCode >= 400) {
      throw OminiConnectException(
        'Upstream error ${resp.statusCode}',
        statusCode: resp.statusCode,
      );
    }
  }

  /// Maton-style direct API call — the simplest way to use OminiConnect.
  ///
  /// Example:
  /// ```dart
  /// final repos = await client.call('github', 'GET', '/user/repos', params: {'sort': 'updated'});
  /// await client.call('slack', 'POST', '/api/chat.postMessage', body: {'channel': 'C0123', 'text': 'Hi!'});
  /// ```
  Future<dynamic> call(
    String platform,
    String method,
    String path, {
    Map<String, String>? params,
    Map<String, dynamic>? body,
  }) async {
    final payload = <String, dynamic>{
      'method': method.toUpperCase(),
      'path': path,
    };
    if (params != null) payload['params'] = params;
    if (body != null) payload['body'] = body;

    final uri = Uri.parse('$baseUrl/api/call/$platform');
    final resp = await _http
        .post(uri, headers: _headers, body: jsonEncode(payload))
        .timeout(const Duration(seconds: 30));

    if (resp.statusCode == 401) throw AuthException();
    if (resp.statusCode == 429) throw OminiConnectException('Rate limited', statusCode: 429);
    if (resp.statusCode >= 400) {
      throw OminiConnectException(
        'Upstream error ${resp.statusCode}',
        statusCode: resp.statusCode,
        body: resp.body,
      );
    }

    if (resp.body.isEmpty) return null;
    return jsonDecode(resp.body);
  }
}

// ─── Connectors ─────────────────────────────────────────────────────────────

class ConnectorsManager {
  final OminiConnect _client;
  final String _base;

  ConnectorsManager(this._client, this._base);

  /// List all connected platforms.
  Future<List<Connector>> list() async {
    final data = await _client._request('GET', '/api/connectors');
    return (data as List<dynamic>)
        .map((c) => Connector.fromJson(c as Map<String, dynamic>))
        .toList();
  }

  /// Delete a connected platform.
  Future<void> delete(String platform) async {
    await _client._requestNoContent('DELETE', '/api/connectors/$platform');
  }
}

// ─── API Keys ───────────────────────────────────────────────────────────────

class ApiKeysManager {
  final OminiConnect _client;
  final String _base;

  ApiKeysManager(this._client, this._base);

  /// Create a new named API key.
  /// The raw key is returned ONLY here — store securely.
  Future<ApiKeyCreated> create(String label) async {
    final data = await _client._request('POST', '/auth/apikey', body: {'label': label});
    return ApiKeyCreated.fromJson(data as Map<String, dynamic>);
  }

  /// List all API keys (raw key is never returned).
  Future<List<ApiKeySummary>> list() async {
    final data = await _client._request('GET', '/auth/apikey');
    return (data as List<dynamic>)
        .map((k) => ApiKeySummary.fromJson(k as Map<String, dynamic>))
        .toList();
  }

  /// Revoke an API key.
  Future<void> delete(String keyHash) async {
    await _client._requestNoContent('DELETE', '/auth/apikey/$keyHash');
  }
}

// ─── Tools ─────────────────────────────────────────────────────────────────

class ToolsManager {
  final OminiConnect _client;
  final String _base;

  ToolsManager(this._client, this._base);

  /// List available tools, optionally filtered by platform.
  Future<ToolkitsResponse> list({String? platform}) async {
    final data = await _client._request(
      'GET',
      '/api/tools',
      params: platform != null ? {'platform': platform} : null,
    );
    return ToolkitsResponse.fromJson(data as Map<String, dynamic>);
  }

  /// Search tools by query string.
  Future<ToolsSearchResponse> search(
    String q, {
    String? platform,
    String? filterScope,
  }) async {
    final params = <String, String>{'q': q};
    if (platform != null) params['platform'] = platform;
    if (filterScope != null) params['filter_scope'] = filterScope;
    final data = await _client._request('GET', '/api/tools/search', params: params);
    return ToolsSearchResponse.fromJson(data as Map<String, dynamic>);
  }

  /// Execute a tool by slug with structured arguments.
  /// Set [callbackUrl] for async execution (returns immediately).
  Future<ToolExecuteResult> execute(
    String toolSlug,
    String platform, {
    Map<String, dynamic>? arguments,
    String? callbackUrl,
  }) async {
    final body = <String, dynamic>{
      'tool_slug': toolSlug,
      'platform': platform,
      'arguments': arguments ?? {},
    };
    if (callbackUrl != null) body['callback_url'] = callbackUrl;
    final data = await _client._request('POST', '/api/tools/execute', body: body);
    return ToolExecuteResult.fromJson(data as Map<String, dynamic>);
  }
}
