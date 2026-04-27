# Roam-scim Tools

Provider: `roam-scim` | Engine: `nango` | Auth: SCIM via Nango

## Overview

Roam Research SCIM API enables automated user and group provisioning for Roam Research. These tools allow AI agents to manage users and groups via SCIM protocol for directory synchronization.

## Authentication

**Nango SCIM**:
- User provides SCIM token and URL via Nango
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `users:read`, `users:write`, `groups:read`, `groups:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `roam-scim_list_users` | List all users | GET | /v2/users |
| `roam-scim_get_user` | Get user details | GET | /v2/users/{userId} |
| `roam-scim_create_user` | Create a user | POST | /v2/users |
| `roam-scim_update_user` | Update a user | PUT | /v2/users/{userId} |
| `roam-scim_delete_user` | Delete a user | DELETE | /v2/users/{userId} |
| `roam-scim_list_groups` | List all groups | GET | /v2/groups |
| `roam-scim_get_group` | Get group details | GET | /v2/groups/{groupId} |
| `roam-scim_create_group` | Create a group | POST | /v2/groups |
| `roam-scim_update_group` | Update a group | PUT | /v2/groups/{groupId} |
| `roam-scim_delete_group` | Delete a group | DELETE | /v2/groups/{groupId} |

---

## Tool Details

### roam-scim_list_users

**What it does**: Returns a list of all users.

**When to use**: Sync user directory, find users.

**Arguments**:
- `limit` (optional): Number of users (default 50)
- `filter` (optional): SCIM filter expression

**Example LLM prompt**: "List all users in Roam"

---

### roam-scim_get_user

**What it does**: Gets details of a specific user.

**When to use**: Get user information, email, status.

**Arguments**:
- `userId` (required): The user ID

**Example LLM prompt**: "Get details for user usr_abc123"

---

### roam-scim_create_user

**What it does**: Creates a new user via SCIM.

**When to use**: Provision new user accounts.

**Arguments**:
- `userName` (required): Unique username
- `emails` (required): Array of email objects
- `name` (optional): Name object with givenName, familyName
- `active` (optional): Active status (default true)

**Example LLM prompt**: "Create a user with username 'johndoe' and email john@example.com"

---

### roam-scim_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user attributes, deactivate account.

**Arguments**:
- `userId` (required): The user ID
- `userName` (optional): Updated username
- `emails` (optional): Updated emails
- `name` (optional): Updated name object
- `active` (optional): Active status

**Example LLM prompt**: "Update user usr_abc123 to deactivate"

---

### roam-scim_delete_user

**What it does**: Deletes a user.

**When to use**: Remove user accounts.

**Arguments**:
- `userId` (required): The user ID to delete

**Example LLM prompt**: "Delete user usr_abc123"

---

### roam-scim_list_groups

**What it does**: Returns a list of all groups.

**When to use**: View group directory.

**Arguments**:
- `limit` (optional): Number of groups (default 50)

**Example LLM prompt**: "List all groups"

---

### roam-scim_get_group

**What it does**: Gets details of a specific group.

**When to use**: Get group members.

**Arguments**:
- `groupId` (required): The group ID

**Example LLM prompt**: "Get details for group grp_xyz789"

---

### roam-scim_create_group

**What it does**: Creates a new group.

**When to use**: Create teams and departments.

**Arguments**:
- `displayName` (required): Group name
- `members` (optional): Array of user IDs

**Example LLM prompt**: "Create a group called 'Engineering'"

---

### roam-scim_update_group

**What it does**: Updates an existing group.

**When to use**: Modify group membership.

**Arguments**:
- `groupId` (required): The group ID
- `displayName` (optional): Updated name
- `members` (optional): Updated member list

**Example LLM prompt**: "Update group grp_xyz789 with new members"

---

### roam-scim_delete_group

**What it does**: Deletes a group.

**When to use**: Remove teams.

**Arguments**:
- `groupId` (required): The group ID to delete

**Example LLM prompt**: "Delete group grp_xyz789"

---

## Roam SCIM Notes

- SCIM is for automated user/group provisioning
- Users must have unique usernames and emails
- Groups contain members (user references)
- Active flag controls user login access
- Supports SCIM filtering for user queries
