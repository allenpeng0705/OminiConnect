# Figma SCIM Tools

Provider: `figma-scim` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Figma SCIM API. They allow AI agents to manage users and groups for Figma organization administration. Figma SCIM supports automated provisioning for enterprise teams.

## Authentication

**Nango API_KEY**:
- User provides API token and host configuration via Nango Connect
- Token is passed in the Authorization header as Bearer token
- Host determines the Figma instance URL
- SCIM base path: `https://{host}/scim/v2/{tenantId}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `figma_scim_list_users` | List users | GET | /Users |
| `figma_scim_get_user` | Get user details | GET | /Users/{id} |
| `figma_scim_create_user` | Create a user | POST | /Users |
| `figma_scim_update_user` | Update user details | PUT | /Users/{id} |
| `figma_scim_delete_user` | Delete a user | DELETE | /Users/{id} |
| `figma_scim_list_groups` | List groups | GET | /Groups |
| `figma_scim_get_group` | Get group details | GET | /Groups/{id} |
| `figma_scim_create_group` | Create a group | POST | /Groups |
| `figma_scim_update_group` | Update group | PUT | /Groups/{id} |
| `figma_scim_list_provisioning_logs` | List provisioning logs | GET | /Logs |

---

## Tool Details

### figma_scim_list_users

**What it does**: Lists all users in the organization.

**When to use**: Browse user directory, find user IDs.

**Arguments**:
- `start_index` (optional): Pagination start index (default 1)
- `count` (optional): Number of results (default 20)

**Example LLM prompt**: "List all Figma users"

---

### figma_scim_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user information, check status.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### figma_scim_create_user

**What it does**: Creates a new user in the organization.

**When to use**: Provision new team members automatically.

**Arguments**:
- `user_name` (required): Username (typically email)
- `display_name` (optional): Display name
- `email` (required): Email address

**Example LLM prompt**: "Create a new user for john@company.com"

---

### figma_scim_update_user

**What it does**: Updates user details.

**When to use**: Modify user information, update email.

**Arguments**:
- `id` (required): User ID
- `display_name` (optional): Display name
- `email` (optional): Email address

**Example LLM prompt**: "Update user abc123 with new email"

---

### figma_scim_delete_user

**What it does**: Deletes a user from the organization.

**When to use**: Remove users who leave the team.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user abc123"

---

### figma_scim_list_groups

**What it does**: Lists all groups in the organization.

**When to use**: Browse groups, find group IDs.

**Arguments**:
- `start_index` (optional): Pagination start index (default 1)

**Example LLM prompt**: "List all Figma groups"

---

### figma_scim_get_group

**What it does**: Gets details of a specific group.

**When to use**: View group members, settings.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get details for group xyz789"

---

### figma_scim_create_group

**What it does**: Creates a new group.

**When to use**: Create teams for organization structure.

**Arguments**:
- `display_name` (required): Group display name

**Example LLM prompt**: "Create a group called 'Design Team'"

---

### figma_scim_update_group

**What it does**: Updates group details.

**When to use**: Rename groups, modify settings.

**Arguments**:
- `id` (required): Group ID
- `display_name` (optional): New display name

**Example LLM prompt**: "Update group xyz789 to 'Engineering'"

---

### figma_scim_list_provisioning_logs

**What it does**: Lists provisioning activity logs.

**When to use**: Audit user provisioning, troubleshoot issues.

**Arguments**:
- `start_date` (optional): Start date filter
- `end_date` (optional): End date filter

**Example LLM prompt**: "List provisioning logs for the last week"

---

## Figma SCIM API Notes

- **SCIM Protocol**: Standard for identity provisioning
- **Users**: Team members in the Figma organization
- **Groups**: Teams for organizing users
- **Provisioning**: Automated user lifecycle management
- **Logs**: Audit trail for provisioning actions
- **Host**: Figma instance (www.figma.com for most)
