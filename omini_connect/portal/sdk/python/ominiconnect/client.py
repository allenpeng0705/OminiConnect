"""
OminiConnect Python SDK client.
"""

from __future__ import annotations

from typing import Any, Dict, List, Optional

import requests

from .connectors import ConnectorsManager
from .errors import (
    AuthError,
    ConnectorNotFoundError,
    NetworkError,
    OminiConnectError,
    RateLimitedError,
    UpstreamError,
)
from .models import ApiKeyCreated, ApiKeySummary
from .tools import ToolManager


class OminiConnect:
    """Main SDK client for OminiConnect.

    Args:
        api_key: Bearer API key from the portal.
        base_url: Portal base URL. Defaults to http://localhost:9000.
        timeout: Request timeout in seconds (default 30).

    Example::

        from ominiconnect import OminiConnect

        client = OminiConnect(api_key="sk-xxxxx")

        # Check what's connected
        print(client.connectors.list())

        # Call any connected API directly — Maton style
        user = client.call("github", "GET", "/user")
        repos = client.call("github", "GET", "/user/repos", params={"sort": "updated"})
        client.call("slack", "POST", "/api/chat.postMessage", body={"channel": "X", "text": "hi"})

        # Or use tools (discoverable, scope-checked)
        tools = client.tools.list(platform="github")
        result = client.tools.execute("github_list_repos", {"sort": "updated"})

        # Create named API keys for different agents
        key = client.api_keys.create("pr-reviewer-agent")
        print(key["key"])  # shown once — store securely
    """

    def __init__(
        self,
        api_key: str,
        base_url: str = "http://localhost:9000",
        timeout: float = 30.0,
    ):
        self._api_key = api_key
        self._base_url = base_url.rstrip("/")
        self._timeout = timeout
        self._session = requests.Session()
        self._session.headers["Authorization"] = f"Bearer {api_key}"
        self._session.headers["Content-Type"] = "application/json"

        self.tools = ToolManager(self)
        self.connectors = ConnectorsManager(self)
        self.api_keys = ApiKeysManager(self)

    # ─── HTTP helpers ────────────────────────────────────────────────────

    def _request(
        self,
        method: str,
        path: str,
        *,
        params: Optional[Dict[str, Any]] = None,
        body: Optional[Any] = None,
    ) -> Any:
        """Make an HTTP request with structured error handling."""
        url = f"{self._base_url}{path}"
        try:
            if method == "GET":
                resp = self._session.get(url, params=params, timeout=self._timeout)
            elif method == "POST":
                resp = self._session.post(url, json=body, timeout=self._timeout)
            elif method == "DELETE":
                resp = self._session.delete(url, timeout=self._timeout)
            elif method == "PUT":
                resp = self._session.put(url, json=body, timeout=self._timeout)
            elif method == "PATCH":
                resp = self._session.patch(url, json=body, timeout=self._timeout)
            else:
                raise ValueError(f"Unsupported HTTP method: {method}")
        except requests.exceptions.Timeout as e:
            raise NetworkError(f"request timed out after {self._timeout}s: {e}") from e
        except requests.exceptions.ConnectionError as e:
            raise NetworkError(f"connection failed: {e}") from e
        except requests.exceptions.RequestException as e:
            raise NetworkError(f"request failed: {e}") from e

        if resp.status_code == 401:
            raise AuthError("invalid or missing API key")
        if resp.status_code == 404:
            raise ConnectorNotFoundError(f"not found at {path}")
        if resp.status_code == 429:
            raise RateLimitedError("rate limited — back off before retrying")

        if not resp.ok:
            body_text = resp.text[:500]
            raise UpstreamError(resp.status_code, body_text)

        if resp.content:
            return resp.json()
        return None

    def _get(self, path: str, params: Optional[Dict[str, Any]] = None) -> Any:
        return self._request("GET", path, params=params)

    def _post(self, path: str, body: Optional[Any] = None) -> Any:
        return self._request("POST", path, body=body)

    def _delete(self, path: str) -> Any:
        return self._request("DELETE", path)

    # ─── Maton-style direct call ────────────────────────────────────────

    def call(
        self,
        platform: str,
        method: str,
        path: str,
        *,
        params: Optional[Dict[str, Any]] = None,
        body: Optional[Dict[str, Any]] = None,
    ) -> Any:
        """Call any connected platform's API directly — Maton style.

        This is the simplest way to use OminiConnect. No tool schemas needed.

        Args:
            platform: The platform slug (e.g. "github", "slack", "feishu").
            method: HTTP verb: "GET", "POST", "PUT", "DELETE", "PATCH".
            path: The platform API path (e.g. "/user/repos", "/api/chat.postMessage").
            params: Query string parameters (optional).
            body: Request body for POST/PUT/PATCH (optional).

        Returns:
            The parsed JSON response from the upstream API.

        Raises:
            AuthError: Invalid API key.
            ConnectorNotFoundError: Platform not connected.
            UpstreamError: The platform API returned a non-2xx response.
            NetworkError: Connection failure or timeout.

        Example::

            # GET with query params
            repos = client.call("github", "GET", "/user/repos", params={"sort": "updated"})

            # POST with body
            client.call("slack", "POST", "/api/chat.postMessage", body={
                "channel": "C0123",
                "text": "Hello from OminiConnect!"
            })

            # DELETE
            client.call("github", "DELETE", f"/repos/{owner}/{repo}")
        """
        payload: Dict[str, Any] = {
            "method": method.upper(),
            "path": path,
        }
        if params:
            # Convert params values to JSON-compatible strings
            payload["params"] = params
        if body is not None:
            payload["body"] = body

        return self._post(f"/api/call/{platform}", payload)


# ─── API Keys ─────────────────────────────────────────────────────────────


class ApiKeysManager:
    """Manage API keys for this account.

    Create named keys for different agents or use cases. Each key is
    independent — revoke one without affecting others.
    """

    def __init__(self, client: OminiConnect):
        self._client = client

    def create(self, label: str) -> ApiKeyCreated:
        """Create a new named API key.

        Args:
            label: A human-readable name for this key (e.g. "pr-reviewer-agent").

        Returns:
            {"key": "raw-key", "label": "...", "created_at": "..."}.
            The raw key is shown **only once** — store it securely.

        Example::

            key = client.api_keys.create("slack-bot")
            print(key["key"])  # "a1b2c3..." — save this!
        """
        return self._client._post("/auth/apikey", {"label": label})

    def list(self) -> List[ApiKeySummary]:
        """List all API keys (raw key is never returned).

        Returns:
            List of {"key_hash": "...", "label": "...", "created_at": "..."}.
        """
        return self._client._get("/auth/apikey")

    def delete(self, key_hash: str) -> dict:
        """Revoke an API key.

        Args:
            key_hash: The key's hash (from list()).

        Returns:
            {"ok": True} on success.
        """
        return self._client._delete(f"/auth/apikey/{key_hash}")
