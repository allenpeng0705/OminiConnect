# Canva SCIM Tools

Provider: `canva-scim` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Canva SCIM API. Canva SCIM provides user and group provisioning for Canva for Teams organizations. **Requires Canva SCIM API key.**

## Authentication

**Nango API_KEY**:
- User provides their Canva SCIM API key
- Token passed via `Authorization: Bearer` header
- Base URL: `https://www.canva.com/_scim`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `canva_scim_list_users` | List users | GET | /v2/Users |
| `canva_scim_get_user` | Get user details | GET | /v2/Users/{id} |
| `canva_scim_create_user` | Create a user | POST | /v2/Users |
| `canva_scim_update_user` | Update a user | PUT | /v2/Users/{id} |
| `canva_scim_delete_user` | Delete a user | DELETE | /v2/Users/{id} |
| `canva_scim_list_groups` | List groups | GET | /v2/Groups |
| `canva_scim_get_group` | Get group details | GET | /v2/Groups/{id} |
| `canva_scim_create_group` | Create a group | POST | /v2/Groups |
| `canva_scim_update_group` | Update a group | PUT | /v2/Groups/{id} |
| `canva_scim_list_service_provider` | Get SP config | GET | /v2/ServiceProviderConfig |

---

## Tool Details

### canva_scim_list_users

**What it does**: Lists all users in the Canva organization.

**When to use**: View all team members, find specific users.

**Arguments**:
- `filter` (optional): SCIM filter expression
- `startIndex` (optional): Pagination start (default 1)
- `count` (optional): Max results (default 20)

**Example LLM prompt**: "List all users in my Canva organization"

---

### canva_scim_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile, email, group memberships.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 123 details"

---

### canva_scim_create_user

**What it does**: Creates a new user in the organization.

**When to use**: Provision new team members.

**Arguments**:
- `userName` (required): Username (typically email)
- `name` (optional): Object with `givenName` and `familyName`
- `emails` (optional): Array of email objects
- `active` (optional): Whether user is active (default true)

**Example LLM prompt**: "Create a new user john@example.com in Canva"

---

### canva_scim_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user details or deactivate users.

**Arguments**:
- `id` (required): User ID
- `name` (optional): Updated name object
- `active` (optional): Updated active status

**Example LLM prompt**: "Update user 123 to deactivate them"

---

### canva_scim_delete_user

**What it does**: Deletes a user from the organization.

**When to use**: Remove team members from Canva.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user 123 from Canva"

---

### canva_scim_list_groups

**What it does**: Lists all groups in the organization.

**When to use**: View team groups, find group IDs.

**Arguments**:
- `startIndex` (optional): Pagination start (default 1)
- `count` (optional): Max results (default 20)

**Example LLM prompt**: "List all groups in Canva"

---

### canva_scim_get_group

**What it does**: Gets details of a specific group.

**When to use**: View group members, find group for a user.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get group 456 details"

---

### canva_scim_create_group

**What it does**: Creates a new group.

**When to use**: Create new teams or departments.

**Arguments**:
- `displayName` (required): Group name
- `members` (optional): Array of member objects

**Example LLM prompt**: "Create a group called 'Design Team' in Canva"

---

### canva_scim_update_group

**What it does**: Updates an existing group.

**When to use**: Rename groups, add or remove members.

**Arguments**:
- `id` (required): Group ID
- `displayName` (optional): New group name
- `members` (optional): Updated member list

**Example LLM prompt**: "Update group 456 to rename it to 'Marketing Team'"

---

### canva_scim_list_service_provider

**What it does**: Gets the SCIM service provider configuration.

**When to use**: Verify SCIM capabilities and configuration.

**Arguments**: None

**Example LLM prompt**: "Get Canva SCIM service provider config"

---

## Canva SCIM Notes

- **SCIM Protocol**: Industry standard for user provisioning
- **Users**: Team members in Canva for Teams organization
- **Groups**: Teams or departments for organizing users
- **Active Flag**: Setting `active: false` deactivates but does not delete a user
