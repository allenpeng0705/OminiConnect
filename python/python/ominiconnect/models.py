"""
OminiConnect typed response models.
Uses TypedDict so IDEs can provide autocompletion.
"""

from __future__ import annotations

from typing import Any, Dict, List, Literal, Optional, TypedDict


# ─── API Key ────────────────────────────────────────────────────────────────

class ApiKeyCreated(TypedDict):
    """Response from POST /auth/apikey — raw key shown once."""
    key: str
    label: str
    created_at: str


class ApiKeySummary(TypedDict):
    """API key listed via GET /auth/apikey."""
    key_hash: str
    label: str
    created_at: str


# ─── Connector ────────────────────────────────────────────────────────────

class Connector(TypedDict):
    """A connected platform."""
    platform: str
    enabled: bool
    scopes: List[str]
    created_at: str


# ─── Tools ────────────────────────────────────────────────────────────────

class ToolSummary(TypedDict):
    """A single tool definition."""
    slug: str
    name: str
    description: str
    method: str
    endpoint: str
    scopes: List[str]
    scope_satisfied: Literal["yes", "no", "unknown"]
    tags: List[str]


class Toolkit(TypedDict):
    """A group of tools for one platform."""
    slug: str
    name: str
    logo: Optional[str]
    provider: str
    tools: List[ToolSummary]


class ToolkitsResponse(TypedDict):
    """Response from GET /api/tools."""
    toolkits: List[Toolkit]


class ToolsSearchResponse(TypedDict):
    """Response from GET /api/tools/search."""
    tools: List[ToolSummary]
    query: str


class ToolExecuteResult(TypedDict):
    """Result from POST /api/tools/execute."""
    ok: bool
    body: Optional[Any]
    error: Optional[str]
    call_id: Optional[str]
    status: Optional[Literal["pending"]]
    duration_ms: Optional[int]


# ─── MCP ──────────────────────────────────────────────────────────────────

class McpTool(TypedDict):
    """Tool from MCP tools/list."""
    name: str
    description: str
    input_schema: Dict[str, Any]
    scope_satisfied: Optional[str]


class McpToolsListResponse(TypedDict):
    """MCP tools/list result."""
    tools: List[McpTool]


class McpCallResponse(TypedDict):
    """MCP tools/call result."""
    ok: bool
    body: Optional[Any]
    error: Optional[str]


# ─── LLM ────────────────────────────────────────────────────────────────────


class AvailableTool(TypedDict):
    """A single available tool for LLM consumption."""
    slug: str
    name: str
    description: str
    example_queries: List[str]
    scopes: List[str]
    scope_satisfied: str


class PlatformTools(TypedDict):
    """Tools and connection status for a single platform."""
    connected: bool
    tools: Optional[List[AvailableTool]]


class LlmToolsResponse(TypedDict):
    """Response from GET /api/llm/tools."""
    platforms: Dict[str, PlatformTools]


class LlmExecuteResponse(TypedDict):
    """Response from POST /api/llm."""
    ok: bool
    tool: Optional[str]
    tool_name: Optional[str]
    arguments: Optional[Dict[str, Any]]
    explanation: Optional[str]
    result: Optional[Dict[str, Any]]
    error: Optional[str]
    message: Optional[str]
    candidates: Optional[List[Dict[str, Any]]]
    available_tools_hint: Optional[str]
