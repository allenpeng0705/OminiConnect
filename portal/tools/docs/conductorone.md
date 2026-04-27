# ConductorOne Tools

Provider: `conductorone` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the ConductorOne API. ConductorOne is an identity and access management platform for automating access reviews and provisioning. **Requires ConductorOne OAuth2 Client Credentials.**

## Authentication

**Nango OAuth2_CC**:
- Uses Client Credentials flow for service integration
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.hostname}/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `conductorone_list_users` | List users | GET | /api/v1/users |
| `conductorone_get_user` | Get user details | GET | /api/v1/users/{id} |
| `conductorone_list_groups` | List groups | GET | /api/v1/groups |
| `conductorone_get_group` | Get group details | GET | /api/v1/groups/{id} |
| `conductorone_list_entitlements` | List entitlements | GET | /api/v1/entitlements |
| `conductorone_get_entitlement` | Get entitlement details | GET | /api/v1/entitlements/{id} |
| `conductorone_list_grants` | List grants | GET | /api/v1/grants |
| `conductorone_create_grant` | Create a grant | POST | /api/v1/grants |
| `conductorone_revoke_grant` | Revoke a grant | DELETE | /api/v1/grants/{id} |
| `conductorone_list_activities` | List activities | GET | /api/v1/activities |

---

## Tool Details

### conductorone_list_users

**What it does**: Lists all users in the directory.

**When to use**: View identity directory.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all ConductorOne users"

---

### conductorone_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile and entitlements.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 123 details"

---

### conductorone_list_groups

**What it does**: Lists all groups in the directory.

**When to use**: View group structure.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all groups"

---

### conductorone_get_group

**What it does**: Gets details of a specific group.

**When to use**: View group members.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get group 456 details"

---

### conductorone_list_entitlements

**What it does**: Lists all entitlements in the system.

**When to use**: View available access rights.

**Arguments**:
- `user_id` (optional): Filter by user
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List entitlements for user 123"

---

### conductorone_get_entitlement

**What it does**: Gets details of a specific entitlement.

**When to use**: View entitlement details.

**Arguments**:
- `id` (required): Entitlement ID

**Example LLM prompt**: "Get entitlement 789 details"

---

### conductorone_list_grants

**What it does**: Lists all grants (access assignments).

**When to use**: View current access grants.

**Arguments**:
- `user_id` (optional): Filter by user
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List grants for user 123"

---

### conductorone_create_grant

**What it does**: Creates a new grant (assigns access).

**When to use**: Grant a user access to a resource.

**Arguments**:
- `user_id` (required): User ID
- `entitlement_id` (required): Entitlement ID

**Example LLM prompt**: "Grant user 123 access to entitlement 789"

---

### conductorone_revoke_grant

**What it does**: Revokes a specific grant.

**When to use**: Remove a user's access.

**Arguments**:
- `id` (required): Grant ID

**Example LLM prompt**: "Revoke grant 101"

---

### conductorone_list_activities

**What it does**: Lists all activity logs.

**When to use**: Audit access changes.

**Arguments**:
- `user_id` (optional): Filter by user
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List activities for user 123"

---

## ConductorOne API Notes

- **Hostname**: Extracted from Client ID (e.g., acme.conductor.one)
- **Users**: Identity directory users
- **Groups**: Groups for organizing access
- **Entitlements**: Available access rights (permissions)
- **Grants**: Active access assignments
- **Activities**: Audit log of access changes
