"""
Tool registry API — discover and execute tools.
"""

from __future__ import annotations

from typing import Any, Dict, Optional

from .errors import ScopeInsufficientError, ToolNotFoundError
from .models import ToolkitsResponse, ToolExecuteResult, ToolsSearchResponse


class ToolManager:
    """Discover and execute tools via OminiConnect.

    Tools are pre-defined API operations (e.g. "list GitHub repos",
    "send a Slack message") with structured arguments and scope checking.

    Use ``client.call()`` for direct passthrough — tools are optional.
    """

    def __init__(self, client: "OminiConnect"):
        self._client = client

    def list(self, platform: Optional[str] = None) -> ToolkitsResponse:
        """List available tools for connected platforms.

        Args:
            platform: Optional platform filter (e.g. "github", "slack").

        Returns:
            {"toolkits": [{"slug", "name", "logo", "provider", "tools": [...]}]}.

        Example::

            result = client.tools.list()
            for toolkit in result["toolkits"]:
                print(toolkit["name"], len(toolkit["tools"]))

            # Filter by platform
            github = client.tools.list(platform="github")
        """
        params = {}
        if platform:
            params["platform"] = platform
        return self._client._get("/api/tools", params)

    def search(
        self,
        q: str,
        platform: Optional[str] = None,
        filter_scope: Optional[Literal["yes", "no", "any"]] = None,
    ) -> ToolsSearchResponse:
        """Search tools by name, description, or tags.

        Args:
            q: Search query string.
            platform: Optional platform filter.
            filter_scope: "yes" = only tools with satisfied scopes,
                         "no" = only tools with missing scopes,
                         "any" = all tools (default).

        Returns:
            {"tools": [...], "query": "..."}.
        """
        params: Dict[str, Any] = {"q": q}
        if platform:
            params["platform"] = platform
        if filter_scope:
            params["filter_scope"] = filter_scope
        return self._client._get("/api/tools/search", params)

    def execute(
        self,
        tool_slug: str,
        arguments: Optional[Dict[str, Any]] = None,
        callback_url: Optional[str] = None,
    ) -> ToolExecuteResult:
        """Execute a tool by slug with structured arguments.

        Args:
            tool_slug: Tool identifier (e.g. "github_list_repos").
            arguments: Tool input parameters (per the tool's input_schema).
            callback_url: Optional webhook URL. If provided, returns
                          immediately with {"call_id"} and POSTs the
                          result to the URL when complete.

        Returns:
            Sync: {"ok": true, "body": ...}
            Async: {"ok": true, "call_id": "...", "status": "pending"}

        Raises:
            ToolNotFoundError: Tool slug does not exist.
            ScopeInsufficientError: Connector is missing required scopes.
            ConnectorNotFoundError: Platform not connected.

        Example::

            result = client.tools.execute(
                "github_list_repos",
                {"sort": "updated", "per_page": 5}
            )
            if result["ok"]:
                print(result["body"])
        """
        payload: Dict[str, Any] = {
            "tool_slug": tool_slug,
            "arguments": arguments or {},
        }
        if callback_url:
            payload["callback_url"] = callback_url

        resp = self._client._post("/api/tools/execute", payload)

        # Map error shapes from the API
        # The API returns 4xx for tool-level errors with JSON error body
        # but the execute endpoint returns error info in the response
        return resp


# Silence unused import for Literal in type hints
from typing import Literal
