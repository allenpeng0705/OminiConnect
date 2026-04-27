import pytest
from unittest.mock import patch, MagicMock

from ominiconnect import OminiConnect
from ominiconnect.errors import AuthError, UpstreamError, RateLimitedError


class TestClientInitialization:
    """Test OminiConnect client initialization."""

    def test_client_requires_api_key(self):
        client = OminiConnect(api_key="test-key-123")
        assert client._api_key == "test-key-123"

    def test_client_default_base_url(self):
        client = OminiConnect(api_key="test-key")
        assert client._base_url == "http://localhost:9000"

    def test_client_custom_base_url(self):
        client = OminiConnect(api_key="test-key", base_url="https://api.example.com")
        assert client._base_url == "https://api.example.com"

    def test_client_strips_trailing_slash(self):
        client = OminiConnect(api_key="test-key", base_url="https://api.example.com/")
        assert client._base_url == "https://api.example.com"

    def test_client_has_connectors_manager(self):
        client = OminiConnect(api_key="test-key")
        assert hasattr(client, "connectors")

    def test_client_has_tools_manager(self):
        client = OminiConnect(api_key="test-key")
        assert hasattr(client, "tools")

    def test_client_has_api_keys_manager(self):
        client = OminiConnect(api_key="test-key")
        assert hasattr(client, "api_keys")

    def test_client_timeout_default(self):
        client = OminiConnect(api_key="test-key")
        assert client._timeout == 30.0

    def test_client_timeout_custom(self):
        client = OminiConnect(api_key="test-key", timeout=60.0)
        assert client._timeout == 60.0


class TestClientCall:
    """Test Maton-style direct API calls."""

    def test_call_get_request(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.content = b'{"login": "testuser", "id": 123}'
        mock_response.json.return_value = {"login": "testuser", "id": 123}

        with patch.object(client._session, "post", return_value=mock_response):
            result = client.call("github", "GET", "/user")
            assert result["login"] == "testuser"

    def test_call_get_with_params(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.content = b'[{"name": "repo1"}, {"name": "repo2"}]'
        mock_response.json.return_value = [{"name": "repo1"}, {"name": "repo2"}]

        with patch.object(client._session, "post", return_value=mock_response):
            result = client.call("github", "GET", "/user/repos", params={"sort": "updated"})
            assert len(result) == 2

    def test_call_post_with_body(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.content = b'{"ok": true}'
        mock_response.json.return_value = {"ok": True, "ts": "1234567890"}

        with patch.object(client._session, "post", return_value=mock_response):
            result = client.call("slack", "POST", "/api/chat.postMessage", body={
                "channel": "C0123",
                "text": "Hello!"
            })
            assert result["ok"] is True


class TestConnectorsManager:
    """Test connectors manager methods."""

    def test_list_connectors(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.content = b'[{"platform": "github", "enabled": true, "scopes": ["repo"], "created_at": "2024-01-01"}]'
        mock_response.json.return_value = [
            {"platform": "github", "enabled": True, "scopes": ["repo"], "created_at": "2024-01-01"}
        ]

        with patch.object(client._session, "get", return_value=mock_response) as mock_get:
            result = client.connectors.list()
            assert len(result) == 1
            assert result[0]["platform"] == "github"
            mock_get.assert_called_once()

    def test_delete_connector(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.content = b'{"ok": true}'
        mock_response.json.return_value = {"ok": True}

        with patch.object(client._session, "delete", return_value=mock_response) as mock_delete:
            client.connectors.delete("github")
            mock_delete.assert_called_once()


class TestToolsManager:
    """Test tools manager methods."""

    def test_list_tools(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "toolkits": [
                {
                    "slug": "github",
                    "name": "GitHub",
                    "provider": "github",
                    "tools": [
                        {
                            "slug": "github_list_repos",
                            "name": "List Repositories",
                            "description": "...",
                            "method": "GET",
                            "endpoint": "/user/repos",
                            "scopes": ["repo"],
                            "scope_satisfied": "yes",
                            "tags": ["github"],
                        }
                    ]
                }
            ]
        }

        with patch.object(client._session, "get", return_value=mock_response) as mock_get:
            result = client.tools.list()
            assert "toolkits" in result
            assert len(result["toolkits"]) == 1
            assert result["toolkits"][0]["slug"] == "github"

    def test_search_tools(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "tools": [
                {
                    "slug": "github_list_repos",
                    "name": "List Repositories",
                    "description": "...",
                    "method": "GET",
                    "endpoint": "/user/repos",
                    "scopes": ["repo"],
                    "scope_satisfied": "yes",
                    "tags": ["github"],
                }
            ],
            "query": "list repos"
        }

        with patch.object(client._session, "get", return_value=mock_response) as mock_get:
            result = client.tools.search("list repos")
            assert len(result["tools"]) == 1
            assert result["tools"][0]["slug"] == "github_list_repos"

    def test_execute_tool(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "ok": True,
            "body": [{"name": "repo1"}],
            "call_id": "call_123",
            "status": "completed",
            "duration_ms": 234,
        }

        with patch.object(client._session, "post", return_value=mock_response) as mock_post:
            result = client.tools.execute("github_list_repos", arguments={"sort": "updated"})
            assert result["ok"] is True
            assert result["duration_ms"] == 234


class TestApiKeysManager:
    """Test API keys manager methods."""

    def test_create_api_key(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 201
        mock_response.ok = True
        mock_response.json.return_value = {
            "key": "sk_om_newkey123",
            "label": "test-agent",
            "created_at": "2024-01-15T10:00:00Z"
        }

        with patch.object(client._session, "post", return_value=mock_response) as mock_post:
            result = client.api_keys.create("test-agent")
            assert result["key"] == "sk_om_newkey123"
            assert result["label"] == "test-agent"

    def test_list_api_keys(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = [
            {"key_hash": "$2b$12$...", "label": "test-agent", "created_at": "2024-01-15T10:00:00Z"}
        ]

        with patch.object(client._session, "get", return_value=mock_response) as mock_get:
            result = client.api_keys.list()
            assert len(result) == 1
            assert "key" not in result[0]
            assert result[0]["key_hash"] == "$2b$12$..."

    def test_delete_api_key(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.content = b'{"ok": true}'
        mock_response.json.return_value = {"ok": True}

        with patch.object(client._session, "delete", return_value=mock_response) as mock_delete:
            client.api_keys.delete("$2b$12$hash")
            mock_delete.assert_called_once()
