"""
Tool registry and execution via OminiConnect API.
"""

from __future__ import annotations


class ToolManager:
    """Browse and execute tools via OminiConnect MCP."""

    def __init__(self, client: "OminiConnect"):
        self._client = client

    def list(self, platform: str | None = None) -> dict:
        """List available tools.

        Args:
            platform: Optional platform filter (e.g. "github", "slack").

        Returns:
            ``{"tools": [...]}`` with ``scope_satisfied`` per tool.
        """
        params = {}
        if platform:
            params["platform"] = platform
        return self._client._get("/api/tools", params)

    def search(self, q: str, platform: str | None = None) -> dict:
        """Search tools by name/description/tags.

        Args:
            q: Search query string.
            platform: Optional platform filter.

        Returns:
            ``{"tools": [...]}``.
        """
        params = {"q": q}
        if platform:
            params["platform"] = platform
        return self._client._get("/api/tools/search", params)

    def execute(
        self,
        tool_slug: str,
        arguments: dict | None = None,
        callback_url: str | None = None,
        platform: str | None = None,
    ) -> dict:
        """Execute a tool and return its result.

        Args:
            tool_slug: Tool identifier (e.g. ``"github_list_repos"``).
            arguments: Tool input parameters.
            callback_url: Optional async callback URL (fires and forgets).
            platform: Optional explicit platform override.

        Returns:
            Tool execution result or ``{"call_id": "...", "status": "accepted"}``.
        """
        payload = {
            "tool_slug": tool_slug,
            "arguments": arguments or {},
        }
        if callback_url:
            payload["callback_url"] = callback_url
        if platform:
            payload["platform"] = platform

        return self._client._post("/api/tools/execute", payload)

    def execute_async(self, tool_slug: str, arguments: dict | None = None, callback_url: str | None = None) -> dict:
        """Execute a tool asynchronously, returning immediately.

        The result is POSTed to ``callback_url`` when ready.
        """
        return self.execute(tool_slug, arguments, callback_url=callback_url)