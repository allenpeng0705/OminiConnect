# Keeper SCIM Tools

Provider: `keeper-scim` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Keeper SCIM API. They allow AI agents to manage users, groups, and teams. **Requires Keeper SCIM API key.**

## Authentication

**API Key via Nango**:
- User provides their Keeper SCIM API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://keepersecurity.com/api/rest/scim/v2/{node}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `keeper_scim_list_users` | List users | GET | /Users |
| `keeper_scim_get_user` | Get user details | GET | /Users/{user_id} |
| `keeper_scim_create_user` | Create a user | POST | /Users |
| `keeper_scim_update_user` | Update a user | PUT | /Users/{user_id} |
| `keeper_scim_delete_user` | Delete a user | DELETE | /Users/{user_id} |
| `keeper_scim_list_groups` | List groups | GET | /Groups |
| `keeper_scim_get_group` | Get group details | GET | /Groups/{group_id} |
| `keeper_scim_create_group` | Create a group | POST | /Groups |
| `keeper_scim_list_teams` | List teams | GET | /Teams |
| `keeper_scim_get_team` | Get team details | GET | /Teams/{team_id} |

---

## Tool Details

### keeper_scim_list_users

**What it does**: Lists all users in Keeper.

**When to use**: Find users, view user list.

**Arguments**:
- `filter` (optional): SCIM filter query

**Example LLM prompt**: "List all users in Keeper"

---

### keeper_scim_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information, view user profile.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### keeper_scim_create_user

**What it does**: Creates a new user.

**When to use**: Add users to Keeper.

**Arguments**:
- `userName` (required): Username (email)
- `name` (optional): User name object
- `emails` (optional): Email addresses

**Example LLM prompt**: "Create a user with username john@example.com"

---

### keeper_scim_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user information.

**Arguments**:
- `user_id` (required): User ID
- `name` (optional): User name object

**Example LLM prompt**: "Update user abc123"

---

### keeper_scim_delete_user

**What it does**: Deletes a user.

**When to use**: Remove users from Keeper.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Delete user abc123"

---

### keeper_scim_list_groups

**What it does**: Lists all groups.

**When to use**: View groups, manage permissions.

**Arguments**: None

**Example LLM prompt**: "List all groups in Keeper"

---

### keeper_scim_get_group

**What it does**: Gets details for a specific group.

**When to use**: Get group information.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group xyz789"

---

### keeper_scim_create_group

**What it does**: Creates a new group.

**When to use**: Add groups to Keeper.

**Arguments**:
- `displayName` (required): Group display name

**Example LLM prompt**: "Create a group named 'Engineering'"

---

### keeper_scim_list_teams

**What it does**: Lists all teams.

**When to use**: View teams, organize users.

**Arguments**: None

**Example LLM prompt**: "List all teams in Keeper"

---

### keeper_scim_get_team

**What it does**: Gets details for a specific team.

**When to use**: Get team information.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team t1"

---

## Keeper SCIM Notes

- **SCIM Protocol**: Standard for user management
- **Node**: Keeper enterprise node ID required
- **Users**: Managed via SCIM standard operations
- **Groups/Teams**: Organize users for access control
