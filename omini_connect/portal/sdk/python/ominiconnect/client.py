"""
OminiConnect Python SDK client.
"""

from __future__ import annotations

import requests

from .agents import AgentManager
from .tools import ToolManager


class OminiConnect:
    """Main SDK client for OminiConnect portal.

    Args:
        api_key: Bearer API key from portal.
        base_url: Portal base URL. Defaults to http://localhost:8080.
        timeout: Request timeout in seconds.
    """

    def __init__(
        self,
        api_key: str,
        base_url: str = "http://localhost:8080",
        timeout: float = 30.0,
    ):
        self._api_key = api_key
        self._base_url = base_url.rstrip("/")
        self._timeout = timeout
        self._session = requests.Session()
        self._session.headers["Authorization"] = f"Bearer {api_key}"
        self._session.headers["Content-Type"] = "application/json"

        self.agents = AgentManager(self)
        self.tools = ToolManager(self)

    def _get(self, path: str, params: dict | None = None):
        resp = self._session.get(f"{self._base_url}{path}", params=params, timeout=self._timeout)
        resp.raise_for_status()
        return resp.json()

    def _post(self, path: str, data: dict | None = None):
        resp = self._session.post(f"{self._base_url}{path}", json=data, timeout=self._timeout)
        resp.raise_for_status()
        return resp.json()

    def _delete(self, path: str):
        resp = self._session.delete(f"{self._base_url}{path}", timeout=self._timeout)
        resp.raise_for_status()
        return resp.json()