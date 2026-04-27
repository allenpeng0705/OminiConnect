"""
OminiConnect Python SDK.

Quick start::

    from ominiconnect import OminiConnect

    client = OminiConnect(api_key="sk-xxxxx")

    # List connected platforms
    print(client.connectors.list())

    # Call any API directly — Maton style
    user = client.call("github", "GET", "/user")

    # Use tools (discoverable + scope-checked)
    result = client.tools.execute("github_list_repos", {"sort": "updated"})

    # Create named API keys
    key = client.api_keys.create("my-agent")
"""

from .client import OminiConnect
from .errors import (
    AuthError,
    ConnectorNotFoundError,
    NetworkError,
    OminiConnectError,
    RateLimitedError,
    ScopeInsufficientError,
    ToolNotFoundError,
    UpstreamError,
)
from .llm import LlmManager
from .models import (
    ApiKeyCreated,
    ApiKeySummary,
    AvailableTool,
    Connector,
    LlmExecuteResponse,
    LlmToolsResponse,
    McpCallResponse,
    McpToolsListResponse,
    McpTool,
    PlatformTools,
    ToolkitsResponse,
    ToolExecuteResult,
    ToolsSearchResponse,
    ToolSummary,
    Toolkit,
)
from .tools import ToolManager

__all__ = [
    # Client
    "OminiConnect",
    # Managers
    "ToolManager",
    "LlmManager",
    # Errors
    "OminiConnectError",
    "AuthError",
    "ConnectorNotFoundError",
    "ToolNotFoundError",
    "ScopeInsufficientError",
    "UpstreamError",
    "NetworkError",
    "RateLimitedError",
    # Models
    "Connector",
    "ApiKeyCreated",
    "ApiKeySummary",
    "ToolkitsResponse",
    "Toolkit",
    "ToolSummary",
    "ToolsSearchResponse",
    "ToolExecuteResult",
    "McpTool",
    "McpToolsListResponse",
    "McpCallResponse",
    "AvailableTool",
    "PlatformTools",
    "LlmToolsResponse",
    "LlmExecuteResponse",
]
