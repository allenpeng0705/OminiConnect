# Lucid SCIM Tools

Provider: `lucid-scim` | Engine: `nango` | Auth: API_KEY via Nango (SCIM token)

## Overview

These tools wrap the Lucid SCIM API. They allow AI agents to manage users and groups via SCIM provisioning. **Requires Lucid SCIM token.**

## Authentication

**Nango SCIM API_KEY**:
- User provides SCIM token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Headers: `authorization: Bearer ${apiKey}`, `accept: application/json`, `content-type: application/json`
- Base URL: `https://users.lucid.app/scim`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lucid_scim_list_users` | List all users (SCIM) | GET | /v2/Users |
| `lucid_scim_get_user` | Get user details (SCIM) | GET | /v2/Users/{id} |
| `lucid_scim_create_user` | Create a new user (SCIM) | POST | /v2/Users |
| `lucid_scim_update_user` | Update a user (SCIM) | PUT | /v2/Users/{id} |
| `lucid_scim_delete_user` | Delete a user (SCIM) | DELETE | /v2/Users/{id} |
| `lucid_scim_list_groups` | List all groups (SCIM) | GET | /v2/Groups |
| `lucid_scim_get_group` | Get group details (SCIM) | GET | /v2/Groups/{id} |
| `lucid_scim_create_group` | Create a group (SCIM) | POST | /v2/Groups |
| `lucid_scim_update_group` | Update a group (SCIM) | PUT | /v2/Groups/{id} |
| `lucid_scim_delete_group` | Delete a group (SCIM) | DELETE | /v2/Groups/{id} |

---

## Tool Details

### lucid_scim_list_users

**What it does**: Lists all user accounts in Lucid via SCIM API.

**When to use**: Provision users, audit user list, find specific users.

**Arguments**:
- `filter` (optional): Filter expression (e.g., userName eq 'john@example.com')
- `startIndex` (optional): Starting index for pagination (default 1)
- `count` (optional): Number of results to return (default 100)

**Example LLM prompt**: "List all users in Lucid SCIM"

---

### lucid_scim_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, verify user exists.

**Arguments**:
- `id` (required): User ID (SCIM id)

**Example LLM prompt**: "Get details for user with ID 12345"

---

### lucid_scim_create_user

**What it does**: Creates a new user account in Lucid via SCIM API.

**When to use**: Provision new team members, sync users from IdP.

**Arguments**:
- `userName` (required): Username (email)
- `name` (optional): User name object with givenName, familyName
- `emails` (optional): Email addresses
- `active` (optional): Whether user is active (default true)

**Example LLM prompt**: "Create a new user john@example.com in Lucid"

---

### lucid_scim_update_user

**What it does**: Updates an existing user account via SCIM API.

**When to use**: Update user details, activate/deactivate users.

**Arguments**:
- `id` (required): User ID (SCIM id)
- `userName` (optional): Username (email)
- `name` (optional): User name object
- `active` (optional): Whether user is active

**Example LLM prompt**: "Update user 12345 to be active"

---

### lucid_scim_delete_user

**What it does**: Deletes a user account via SCIM API.

**When to use**: Deprovision users, remove former employees.

**Arguments**:
- `id` (required): User ID (SCIM id)

**Example LLM prompt**: "Delete user 12345 from Lucid"

---

### lucid_scim_list_groups

**What it does**: Lists all groups in Lucid via SCIM API.

**When to use**: List teams, find groups for user assignment.

**Arguments**:
- `filter` (optional): Filter expression
- `startIndex` (optional): Starting index for pagination
- `count` (optional): Number of results to return

**Example LLM prompt**: "List all groups in Lucid SCIM"

---

### lucid_scim_get_group

**What it does**: Gets detailed information about a specific group.

**When to use**: Get group members, verify group exists.

**Arguments**:
- `id` (required): Group ID (SCIM id)

**Example LLM prompt**: "Get details for group engineering"

---

### lucid_scim_create_group

**What it does**: Creates a new group in Lucid via SCIM API.

**When to use**: Create teams, set up departments.

**Arguments**:
- `displayName` (required): Group display name
- `members` (optional): List of member user IDs

**Example LLM prompt**: "Create a group called engineering"

---

### lucid_scim_update_group

**What it does**: Updates an existing group via SCIM API.

**When to use**: Rename groups, update group membership.

**Arguments**:
- `id` (required): Group ID (SCIM id)
- `displayName` (optional): Group display name
- `members` (optional): List of member user IDs

**Example LLM prompt**: "Add user 12345 to group engineering"

---

### lucid_scim_delete_group

**What it does**: Deletes a group via SCIM API.

**When to use**: Remove teams, clean up unused groups.

**Arguments**:
- `id` (required): Group ID (SCIM id)

**Example LLM prompt**: "Delete the old-contractors group"

---

## Lucid SCIM Notes

- **SCIM version**: Uses SCIM 2.0 protocol
- **User IDs**: Stable SCIM IDs, not email addresses
- **Group membership**: Managed via members array
- **Filtering**: Supports SCIM filtering expressions
- **Rate limits**: Implement backoff for bulk operations
