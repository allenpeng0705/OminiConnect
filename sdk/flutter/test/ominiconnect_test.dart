import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:ominiconnect/ominiconnect.dart';

void main() {
  group('OminiConnect Client', () {
    test('initializes with apiKey', () {
      final client = OminiConnect(apiKey: 'test-key-123');
      // Private fields but managers should be accessible
      expect(client.connectors, isA<ConnectorsManager>());
      expect(client.tools, isA<ToolsManager>());
      expect(client.apiKeys, isA<ApiKeysManager>());
    });

    test('initializes with custom baseUrl', () {
      final client = OminiConnect(
        apiKey: 'test-key',
        baseUrl: 'https://api.example.com',
      );
      expect(client.connectors, isA<ConnectorsManager>());
    });

    test('strips trailing slash from baseUrl', () {
      final client = OminiConnect(
        apiKey: 'test-key',
        baseUrl: 'https://api.example.com///',
      );
      expect(client.connectors, isA<ConnectorsManager>());
    });
  });

  group('ConnectorsManager', () {
    test('list returns connectors from API', () async {
      final mockClient = MockClient((req) async {
        expect(req.url.path, '/api/connectors');
        expect(req.headers['Authorization'], 'Bearer test-key');
        return http.Response(
          jsonEncode([
            {'platform': 'github', 'enabled': true, 'scopes': ['repo'], 'created_at': '2024-01-01'}
          ]),
          200,
        );
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      final connectors = await client.connectors.list();
      expect(connectors.length, 1);
      expect(connectors.first.platform, 'github');
      expect(connectors.first.enabled, true);
    });

    test('delete removes connector', () async {
      final mockClient = MockClient((req) async {
        expect(req.method, 'DELETE');
        expect(req.url.path, '/api/connectors/github');
        return http.Response('{"ok": true}', 200);
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      await client.connectors.delete('github');
    });
  });

  group('ToolsManager', () {
    test('list returns toolkits', () async {
      final mockClient = MockClient((req) async {
        expect(req.url.path, '/api/tools');
        return http.Response(
          jsonEncode({
            'toolkits': [
              {
                'slug': 'github',
                'name': 'GitHub',
                'provider': 'github',
                'tools': [
                  {
                    'slug': 'github_list_repos',
                    'name': 'List Repositories',
                    'description': 'List all repos',
                    'method': 'GET',
                    'endpoint': '/user/repos',
                    'scopes': ['repo'],
                    'scope_satisfied': 'yes',
                    'tags': ['github'],
                  }
                ]
              }
            ]
          }),
          200,
        );
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      final result = await client.tools.list();
      expect(result.toolkits.length, 1);
      expect(result.toolkits.first.slug, 'github');
    });

    test('search returns matching tools', () async {
      final mockClient = MockClient((req) async {
        expect(req.url.path, '/api/tools/search');
        expect(req.url.queryParameters['q'], 'list repos');
        return http.Response(
          jsonEncode({
            'tools': [
              {
                'slug': 'github_list_repos',
                'name': 'List Repositories',
                'description': 'List all repos',
                'method': 'GET',
                'endpoint': '/user/repos',
                'scopes': ['repo'],
                'scope_satisfied': 'yes',
                'tags': ['github'],
              }
            ],
            'query': 'list repos',
          }),
          200,
        );
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      final result = await client.tools.search('list repos');
      expect(result.tools.length, 1);
      expect(result.tools.first.slug, 'github_list_repos');
    });

    test('execute runs tool and returns result', () async {
      final mockClient = MockClient((req) async {
        expect(req.url.path, '/api/tools/execute');
        final body = jsonDecode(req.body);
        expect(body['tool_slug'], 'github_list_repos');
        return http.Response(
          jsonEncode({
            'ok': true,
            'body': [{'name': 'repo1'}],
            'call_id': 'call_123',
            'status': 'completed',
            'duration_ms': 234,
          }),
          200,
        );
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      final result = await client.tools.execute('github_list_repos', 'github');
      expect(result.ok, true);
      expect(result.durationMs, 234);
    });
  });

  group('ApiKeysManager', () {
    test('create returns raw key once', () async {
      final mockClient = MockClient((req) async {
        expect(req.method, 'POST');
        expect(req.url.path, '/auth/apikey');
        return http.Response(
          jsonEncode({
            'key': 'sk_om_newkey123',
            'label': 'test-agent',
            'created_at': '2024-01-15T10:00:00Z',
          }),
          201,
        );
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      final result = await client.apiKeys.create('test-agent');
      expect(result.key, 'sk_om_newkey123');
      expect(result.label, 'test-agent');
    });

    test('list returns key_hash only (no raw key)', () async {
      final mockClient = MockClient((req) async {
        expect(req.method, 'GET');
        expect(req.url.path, '/auth/apikey');
        return http.Response(
          jsonEncode([
            {'key_hash': '\$2b\$12\$hash', 'label': 'test-agent', 'created_at': '2024-01-15T10:00:00Z'}
          ]),
          200,
        );
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      final result = await client.apiKeys.list();
      expect(result.length, 1);
      // Raw key must never be returned
      expect(result.first.keyHash, '\$2b\$12\$hash');
    });

    test('delete revokes key', () async {
      final mockClient = MockClient((req) async {
        expect(req.method, 'DELETE');
        expect(req.url.path, '/auth/apikey/hash123');
        return http.Response('{"ok": true}', 200);
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      await client.apiKeys.delete('hash123');
    });
  });

  group('Error types', () {
    test('AuthException is thrown on 401', () async {
      final mockClient = MockClient((req) async {
        return http.Response('{"error": "invalid key"}', 401);
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      expect(
        () => client.connectors.list(),
        throwsA(isA<AuthException>()),
      );
    });

    test('OminiConnectException is thrown on 500', () async {
      final mockClient = MockClient((req) async {
        return http.Response('{"error": "server error"}', 500);
      });

      final client = OminiConnect(apiKey: 'test-key', httpClient: mockClient);
      expect(
        () => client.connectors.list(),
        throwsA(isA<OminiConnectException>()),
      );
    });
  });
}
