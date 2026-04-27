# Notion (SCIM) Tools

Provider: `notion-scim` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Notion SCIM API for workspace user and group management. They allow AI agents to provision users, manage groups, and automate identity lifecycle management. **Requires Notion SCIM API key authentication.**

## Authentication

**API Key**:
- User provides Notion SCIM API key
- Key passed via `Authorization: Bearer` header
- Base URL: `https://api.notion.com/scim`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `notion_scim_list_users` | List users | GET | /v2/Users |
| `notion_scim_get_user` | Get user details | GET | /v2/Users/{id} |
| `notion_scim_create_user` | Create user | POST | /v2/Users |
| `notion_scim_update_user` | Update user | PUT | /v2/Users/{id} |
| `notion_scim_delete_user` | Delete user | DELETE | /v2/Users/{id} |
| `notion_scim_list_groups` | List groups | GET | /v2/Groups |
| `notion_scim_get_group` | Get group details | GET | /v2/Groups/{id} |
| `notion_scim_create_group` | Create group | POST | /v2/Groups |
| `notion_scim_update_group` | Update group | PUT | /v2/Groups/{id} |
| `notion_scim_delete_group` | Delete group | DELETE | /v2/Groups/{id} |

---

## Tool Details

### notion_scim_list_users

**What it does**: Lists all users in the Notion workspace via SCIM.

**When to use**: Browse user directory, get user count.

**Arguments**:
- `startIndex` (optional): Starting index (default 1)
- `count` (optional): Number of users to return (default 100)

**Example LLM prompt**: "List all users in the Notion workspace"

---

### notion_scim_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user attributes, check user status.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user with ID 123"

---

### notion_scim_create_user

**What it does**: Creates a new user in the Notion workspace.

**When to use**: Provision new team members, automate onboarding.

**Arguments**:
- `userName` (required): User email (must be unique)
- `name` (optional): User name object
- `emails` (optional): User emails
- `active` (optional): User active status (default true)

**Example LLM prompt**: "Create a new user for john@company.com"

---

### notion_scim_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user attributes, change status.

**Arguments**:
- `id` (required): User ID
- `name` (optional): User name object
- `active` (optional): User active status

**Example LLM prompt**: "Deactivate user with ID 123"

---

### notion_scim_delete_user

**What it does**: Deletes a user from the Notion workspace.

**When to use**: Remove users, automate offboarding.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user with ID 123"

---

### notion_scim_list_groups

**What it does**: Lists all groups in the Notion workspace.

**When to use**: Browse groups, find team groups.

**Arguments**:
- `startIndex` (optional): Starting index (default 1)
- `count` (optional): Number of groups to return (default 100)

**Example LLM prompt**: "List all groups"

---

### notion_scim_get_group

**What it does**: Gets detailed information for a specific group.

**When to use**: View group members, check group settings.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get details for group with ID 456"

---

### notion_scim_create_group

**What it does**: Creates a new group in the Notion workspace.

**When to use**: Create team groups, organize users.

**Arguments**:
- `displayName` (required): Group display name
- `members` (optional): Group member user IDs

**Example LLM prompt**: "Create a group called Engineering"

---

### notion_scim_update_group

**What it does**: Updates an existing group.

**When to use**: Modify group, add/remove members.

**Arguments**:
- `id` (required): Group ID
- `displayName` (optional): Group display name
- `members` (optional): Group member user IDs

**Example LLM prompt**: "Add user 123 to group 456"

---

### notion_scim_delete_group

**What it does**: Deletes a group from the Notion workspace.

**When to use**: Remove groups, clean up organization.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Delete group with ID 456"

---

## Notion SCIM Notes

- **SCIM standard**: Uses RFC 7644 SCIM protocol
- **User IDs**: UUIDs assigned by Notion
- **Email uniqueness**: userName must be unique across workspace
- **Group management**: Groups contain user memberships
- **Soft delete**: Users may be deactivated rather than hard deleted
