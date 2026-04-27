"""
LLM-powered tool execution API.
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional

from .models import AvailableTool, LlmExecuteResponse, LlmToolsResponse


class LlmManager:
    """LLM-powered tool execution via OminiConnect.

    The LLM endpoint parses natural-language queries and routes them to
    appropriate tools, returning structured execution results.
    """

    def __init__(self, client: "OminiConnect"):
        self._client = client

    def execute(
        self,
        query: str,
        platform: Optional[str] = None,
    ) -> LlmExecuteResponse:
        """Execute a natural-language query via the LLM endpoint.

        Args:
            query: The natural-language query (e.g. "list my github repos").
            platform: Optional platform filter (e.g. "github", "slack").

        Returns:
            LlmExecuteResponse with execution results.

        Example::

            result = client.llm.execute("list my github repos")
            if result["ok"]:
                print(result["result"])
        """
        payload: Dict[str, Any] = {"query": query}
        if platform:
            payload["platform"] = platform

        return self._client._post("/api/llm", payload)

    def list_available_tools(
        self,
        platform: Optional[str] = None,
    ) -> LlmToolsResponse:
        """List available tools grouped by platform for LLM consumption.

        Args:
            platform: Optional platform filter (e.g. "github", "slack").

        Returns:
            LlmToolsResponse mapping platform slugs to PlatformTools.

        Example::

            resp = client.llm.list_available_tools()
            for platform, info in resp["platforms"].items():
                if info["connected"]:
                    print(f"{platform}: {len(info['tools'])} tools")
        """
        params: Dict[str, Any] = {}
        if platform:
            params["platform"] = platform

        return self._client._get("/api/llm/tools", params)
