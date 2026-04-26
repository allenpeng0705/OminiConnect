# Loom SCIM Tools

Provider: `loom-scim` | Engine: `nango` | Auth: API_KEY via Nango (SCIM token)

## Overview

These tools wrap the Loom SCIM API. They allow AI agents to manage users and groups via SCIM provisioning. **Requires Loom SCIM token.**

## Authentication

**Nango SCIM API_KEY**:
- User provides SCIM token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.domain}/scim`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `loom_scim_list_users` | List all users (SCIM) | GET | /Users |
| `loom_scim_get_user` | Get user details (SCIM) | GET | /Users/{id} |
| `loom_scim_create_user` | Create a new user (SCIM) | POST | /Users |
| `loom_scim_update_user` | Update a user (SCIM) | PUT | /Users/{id} |
| `loom_scim_delete_user` | Delete a user (SCIM) | DELETE | /Users/{id} |
| `loom_scim_list_groups` | List all groups (SCIM) | GET | /Groups |
| `loom_scim_get_group` | Get group details (SCIM) | GET | /Groups/{id} |
| `loom_scim_create_group` | Create a group (SCIM) | POST | /Groups |
| `loom_scim_update_group` | Update a group (SCIM) | PUT | /Groups/{id} |
| `loom_scim_delete_group` | Delete a group (SCIM) | DELETE | /Groups/{id} |

---

## Tool Details

### loom_scim_list_users

**What it does**: Lists all user accounts in Loom via SCIM API.

**When to use**: Provision users, audit user list, find specific users.

**Arguments**:
- `filter` (optional): Filter expression (e.g., userName eq 'john@example.com')
- `startIndex` (optional): Starting index for pagination (default 1)
- `count` (optional): Number of results to return (default 100)

**Example LLM prompt**: "List all users in Loom SCIM"

---

### loom_scim_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, verify user exists.

**Arguments**:
- `id` (required): User ID (SCIM id)

**Example LLM prompt**: "Get details for user with ID 12345"

---

### loom_scim_create_user

**What it does**: Creates a new user account in Loom via SCIM API.

**When to use**: Provision new team members automatically.

**Arguments**:
- `userName` (required): Username (email)
- `name` (optional): User name object with givenName, familyName
- `emails` (optional): Email addresses
- `active` (optional): Whether user is active (default true)

**Example LLM prompt**: "Create a new user john@example.com in Loom"

---

### loom_scim_update_user

**What it does**: Updates an existing user account via SCIM API.

**When to use**: Update user details, activate/deactivate users.

**Arguments**:
- `id` (required): User ID (SCIM id)
- `userName` (optional): Username (email)
- `name` (optional): User name object
- `active` (optional): Whether user is active

**Example LLM prompt**: "Deactivate user 12345 in Loom"

---

### loom_scim_delete_user

**What it does**: Deletes a user account via SCIM API.

**When to use**: Deprovision users, remove former employees.

**Arguments**:
- `id` (required): User ID (SCIM id)

**Example LLM prompt**: "Delete user 12345 from Loom"

---

### loom_scim_list_groups

**What it does**: Lists all groups in Loom via SCIM API.

**When to use**: List teams, find groups for user assignment.

**Arguments**:
- `filter` (optional): Filter expression
- `startIndex` (optional): Starting index for pagination
- `count` (optional): Number of results to return

**Example LLM prompt**: "List all groups in Loom SCIM"

---

### loom_scim_get_group

**What it does**: Gets detailed information about a specific group.

**When to use**: Get group members, verify group exists.

**Arguments**:
- `id` (required): Group ID (SCIM id)

**Example LLM prompt**: "Get details for group engineering"

---

### loom_scim_create_group

**What it does**: Creates a new group in Loom via SCIM API.

**When to use**: Create teams, set up departments.

**Arguments**:
- `displayName` (required): Group display name
- `members` (optional): List of member user IDs

**Example LLM prompt**: "Create a group called engineering"

---

### loom_scim_update_group

**What it does**: Updates an existing group via SCIM API.

**When to use**: Rename groups, update group membership.

**Arguments**:
- `id` (required): Group ID (SCIM id)
- `displayName` (optional): Group display name
- `members` (optional): List of member user IDs

**Example LLM prompt**: "Add user 12345 to group engineering"

---

### loom_scim_delete_group

**What it does**: Deletes a group via SCIM API.

**When to use**: Remove teams, clean up unused groups.

**Arguments**:
- `id` (required): Group ID (SCIM id)

**Example LLM prompt**: "Delete the old-contractors group"

---

## Loom SCIM Notes

- **SCIM standard**: Uses SCIM 2.0 protocol
- **User IDs**: Stable SCIM IDs, not email addresses
- **Group membership**: Managed via members array
- **Filtering**: Supports SCIM filtering expressions
- **Rate limits**: Implement backoff for bulk operations
