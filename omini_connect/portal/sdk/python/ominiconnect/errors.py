"""
OminiConnect structured errors.
"""

from __future__ import annotations


class OminiConnectError(Exception):
    """Base error for all OminiConnect exceptions."""
    pass


class AuthError(OminiConnectError):
    """Invalid or missing API key."""
    pass


class ConnectorNotFoundError(OminiConnectError):
    """Platform not connected — call client.connectors.create() first."""
    pass


class ToolNotFoundError(OminiConnectError):
    """Tool slug does not exist."""
    pass


class ScopeInsufficientError(OminiConnectError):
    """Connector is missing required OAuth scopes for this tool."""
    pass


class UpstreamError(OminiConnectError):
    """Provider API returned a non-2xx response.

    Attributes:
        status_code: HTTP status code from the upstream API.
        body: Response body from the upstream API (raw string).
    """

    def __init__(self, status_code: int, body: str):
        super().__init__(f"upstream error {status_code}: {body[:200]}")
        self.status_code = status_code
        self.body = body


class NetworkError(OminiConnectError):
    """Connection failure or timeout."""
    pass


class RateLimitedError(OminiConnectError):
    """Too many requests — back off and retry."""
    pass
