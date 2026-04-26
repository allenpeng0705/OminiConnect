# Ory Tools

Provider: `ory` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Ory Hydra API for identity and access management. They allow AI agents to manage identities, sessions, and OAuth2 clients. **Requires Ory OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses Ory OAuth2 client credentials
- Client credentials passed via Nango
- Token URL: `https://{projectSlug}.projects.oryapis.com/oauth2/token`
- Base URL: `https://{projectSlug}.projects.oryapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ory_list_identities` | List identities | GET | /hydra/admin/v1/clients |
| `ory_get_identity` | Get identity details | GET | /hydra/admin/v1/clients/{id} |
| `ory_create_identity` | Create identity | POST | /hydra/admin/v1/clients |
| `ory_update_identity` | Update identity | PUT | /hydra/admin/v1/clients/{id} |
| `ory_delete_identity` | Delete identity | DELETE | /hydra/admin/v1/clients/{id} |
| `ory_list_sessions` | List sessions | GET | /hydra/admin/v1/sessions |
| `ory_revoke_session` | Revoke session | DELETE | /hydra/admin/v1/sessions/{id}/revoke |
| `ory_list_oauth2_clients` | List OAuth2 clients | GET | /hydra/admin/v1/clients |
| `ory_create_oauth2_client` | Create OAuth2 client | POST | /hydra/admin/v1/clients |
| `ory_get_metrics` | Get metrics | GET | /hydra/admin/v1/metrics |

---

## Tool Details

### ory_list_identities

**What it does**: Lists all identities in Ory.

**When to use**: Browse identities, find users.

**Arguments**:
- `limit` (optional): Number of identities (default 50)

**Example LLM prompt**: "List all identities"

---

### ory_get_identity

**What it does**: Gets detailed information for a specific identity.

**When to use**: View identity details, session status.

**Arguments**:
- `id` (required): Identity ID

**Example LLM prompt**: "Get details for identity ABC123"

---

### ory_create_identity

**What it does**: Creates a new identity in Ory.

**When to use**: Provision new users, create service accounts.

**Arguments**:
- `client_id` (required): Client ID
- `client_name` (optional): Client name

**Example LLM prompt**: "Create a new identity called 'service-account'"

---

### ory_update_identity

**What it does**: Updates an existing identity.

**When to use**: Modify identity settings.

**Arguments**:
- `id` (required): Identity ID
- `client_name` (optional): Client name

**Example LLM prompt**: "Update identity ABC123 with new name"

---

### ory_delete_identity

**What it does**: Deletes an identity from Ory.

**When to use**: Remove users, clean up old accounts.

**Arguments**:
- `id` (required): Identity ID

**Example LLM prompt**: "Delete identity ABC123"

---

### ory_list_sessions

**What it does**: Lists all active sessions.

**When to use**: View active logins, monitor activity.

**Arguments**:
- `limit` (optional): Number of sessions (default 50)

**Example LLM prompt**: "List all active sessions"

---

### ory_revoke_session

**What it does**: Revokes a specific session.

**When to use**: Log out users, security incidents.

**Arguments**:
- `id` (required): Session ID

**Example LLM prompt**: "Revoke session S123456"

---

### ory_list_oauth2_clients

**What it does**: Lists all OAuth2 clients.

**When to use**: Browse OAuth clients, manage access.

**Arguments**: None

**Example LLM prompt**: "List all OAuth2 clients"

---

### ory_create_oauth2_client

**What it does**: Creates a new OAuth2 client.

**When to use**: Register applications, create API access.

**Arguments**:
- `client_id` (required): Client ID
- `client_name` (optional): Client name
- `grant_types` (optional): Grant types

**Example LLM prompt**: "Create an OAuth2 client for my application"

---

### ory_get_metrics

**What it does**: Gets Ory platform metrics.

**When to use**: Monitor platform health, usage stats.

**Arguments**: None

**Example LLM prompt**: "Get current platform metrics"

---

## Ory Notes

- **Project slug**: Your Ory project identifier
- **OAuth2 CC**: Uses client credentials flow
- **Identities**: User and service identities
- **Sessions**: Active authentication sessions
- **Hydra**: Ory's OAuth2 server component
