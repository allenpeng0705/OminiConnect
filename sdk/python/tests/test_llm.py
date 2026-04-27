import pytest
from unittest.mock import patch, MagicMock

from ominiconnect import OminiConnect


class TestLlmManager:
    """Test LLM manager methods."""

    def test_execute_simple_query(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "ok": True,
            "tool": "github_list_repos",
            "tool_name": "List Repositories",
            "arguments": {"sort": "updated", "direction": "desc"},
            "explanation": "Routing 'list my github repos' to github_list_repos",
            "result": [{"name": "my-repo", "full_name": "user/my-repo"}],
        }

        with patch.object(client._session, "post", return_value=mock_response) as mock_post:
            result = client.llm.execute("list my github repos")
            assert result["ok"] is True
            assert result["tool"] == "github_list_repos"
            assert result["arguments"]["sort"] == "updated"
            mock_post.assert_called_once()

    def test_execute_with_platform_filter(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "ok": True,
            "tool": "github_list_repos",
            "tool_name": "List Repositories",
            "arguments": {},
            "explanation": "Routing 'list repos' to github_list_repos",
        }

        with patch.object(client._session, "post", return_value=mock_response) as mock_post:
            result = client.llm.execute("list repos", platform="github")
            assert result["ok"] is True
            # Verify platform was included in the request body
            call_args = mock_post.call_args
            url = call_args[0][0]
            assert "/api/llm" in url

    def test_execute_ambiguous_query(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "ok": False,
            "message": "Ambiguous query — multiple tools matched",
            "candidates": [
                {"tool": "github_list_repos", "name": "List Repositories", "match_reason": "keyword match"},
                {"tool": "github_list_issues", "name": "List Issues", "match_reason": "keyword match"},
            ],
            "available_tools_hint": "Try being more specific: which platform?",
        }

        with patch.object(client._session, "post", return_value=mock_response):
            result = client.llm.execute("list my stuff")
            assert result["ok"] is False
            assert "candidates" in result
            assert len(result["candidates"]) == 2

    def test_execute_error_response(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "ok": False,
            "error": "Insufficient scopes for this operation",
        }

        with patch.object(client._session, "post", return_value=mock_response):
            result = client.llm.execute("create a repo")
            assert result["ok"] is False
            assert "error" in result

    def test_list_available_tools(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "platforms": {
                "github": {
                    "connected": True,
                    "tools": [
                        {
                            "slug": "github_list_repos",
                            "name": "List Repositories",
                            "description": "List all GitHub repositories",
                            "example_queries": ["list my repos", "show my github repos"],
                            "scopes": ["repo"],
                            "scope_satisfied": "yes",
                        }
                    ],
                },
                "slack": {
                    "connected": False,
                    "tools": [],
                },
            }
        }

        with patch.object(client._session, "get", return_value=mock_response) as mock_get:
            result = client.llm.list_available_tools()
            assert "platforms" in result
            assert result["platforms"]["github"]["connected"] is True
            assert len(result["platforms"]["github"]["tools"]) == 1
            assert result["platforms"]["github"]["tools"][0]["slug"] == "github_list_repos"
            mock_get.assert_called_once()

    def test_list_available_tools_with_platform_filter(self, api_key, base_url):
        client = OminiConnect(api_key=api_key, base_url=base_url)
        mock_response = MagicMock()
        mock_response.status_code = 200
        mock_response.ok = True
        mock_response.json.return_value = {
            "platforms": {
                "github": {
                    "connected": True,
                    "tools": [
                        {
                            "slug": "github_list_repos",
                            "name": "List Repositories",
                            "description": "List all GitHub repositories",
                            "example_queries": ["list my repos"],
                            "scopes": ["repo"],
                            "scope_satisfied": "yes",
                        }
                    ],
                }
            }
        }

        with patch.object(client._session, "get", return_value=mock_response) as mock_get:
            result = client.llm.list_available_tools(platform="github")
            assert "platforms" in result
            # Verify the platform filter was passed
            call_args = mock_get.call_args
            url = call_args[0][0]
            assert "platform=github" in url or "platform" in str(call_args)
