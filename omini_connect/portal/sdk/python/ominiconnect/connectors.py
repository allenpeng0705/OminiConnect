"""
Connectors API — manage OAuth/API key connections to platforms.
"""

from __future__ import annotations

from .models import Connector


class ConnectorsManager:
    """Manage connected platforms.

    Example::

        client = OminiConnect("sk-xxxxx")
        client.connectors.list()
        client.connectors.get("github")
        client.connectors.delete("github")

    Note:
        Creating a connector typically starts an OAuth flow in the browser.
        For API key-based platforms, credentials are sent directly.
        Most users will connect platforms via the portal UI instead.
    """

    def __init__(self, client: "OminiConnect"):
        self._client = client

    def list(self) -> list[Connector]:
        """List all connected platforms for this user.

        Returns:
            List of Connector dicts with platform, enabled, scopes.
        """
        return self._client._get("/api/connectors")

    def get(self, platform: str) -> Connector:
        """Get details for a specific connected platform.

        Raises:
            ConnectorNotFoundError: if platform is not connected.
        """
        from .errors import ConnectorNotFoundError
        try:
            return self._client._get(f"/api/connectors/{platform}")
        except Exception as e:
            raise ConnectorNotFoundError(f"Platform '{platform}' is not connected") from e

    def delete(self, platform: str) -> dict:
        """Remove a connected platform.

        Returns:
            {"ok": True} on success.
        """
        return self._client._delete(f"/api/connectors/{platform}")
