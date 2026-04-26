"""
Agent management via OminiConnect API.
"""

from __future__ import annotations


class AgentManager:
    """Manage AI agents registered with OminiConnect."""

    def __init__(self, client: "OminiConnect"):
        self._client = client

    def register(self, name: str, description: str | None = None) -> dict:
        """Register a new agent and get its API key.

        Args:
            name: Human-readable agent name.
            description: Optional description.

        Returns:
            Agent object with ``api_key`` field (shown once).
        """
        payload = {"name": name}
        if description:
            payload["description"] = description
        return self._client._post("/api/agents", payload)

    def list(self) -> dict:
        """List all agents for the authenticated user."""
        return self._client._get("/api/agents")

    def get(self, agent_id: str) -> dict:
        """Get a specific agent by ID."""
        return self._client._get(f"/api/agents/{agent_id}")

    def delete(self, agent_id: str) -> dict:
        """Delete an agent and revoke its API key."""
        return self._client._delete(f"/api/agents/{agent_id}")

    def deactivate(self, agent_id: str) -> dict:
        """Deactivate an agent (preserves audit trail)."""
        return self._client._post(f"/api/agents/{agent_id}/deactivate", None)