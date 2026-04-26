# Grammarly SCIM Tools

Provider: `grammarly-scim` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Grammarly SCIM API. They allow AI agents to manage users and groups for automated provisioning. SCIM (System for Cross-domain Identity Management) enables automated user lifecycle management.

## Authentication

**Nango API Key**:
- User provides Grammarly SCIM API key via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Uses SCIM 2.0 protocol

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `grammarly_scim_list_users` | List SCIM users | GET | /v2/scim/Users |
| `grammarly_scim_get_user` | Get SCIM user details | GET | /v2/scim/Users/{id} |
| `grammarly_scim_create_user` | Create SCIM user | POST | /v2/scim/Users |
| `grammarly_scim_update_user` | Update SCIM user | PUT | /v2/scim/Users/{id} |
| `grammarly_scim_delete_user` | Delete SCIM user | DELETE | /v2/scim/Users/{id} |
| `grammarly_scim_list_groups` | List SCIM groups | GET | /v2/scim/Groups |
| `grammarly_scim_get_group` | Get SCIM group details | GET | /v2/scim/Groups/{id} |
| `grammarly_scim_create_group` | Create SCIM group | POST | /v2/scim/Groups |
| `grammarly_scim_update_group` | Update SCIM group | PUT | /v2/scim/Groups/{id} |
| `grammarly_scim_list_resources` | List SCIM resources | GET | /v2/scim/Resources |

---

## Tool Details

### grammarly_scim_list_users

**What it does**: Lists all SCIM users in the Grammarly organization.

**When to use**: View all provisioned users, find specific users.

**Arguments**:
- `filter` (optional): Filter expression
- `startIndex` (optional): Start index (default 1)
- `count` (optional): Max results (default 20)

**Example LLM prompt**: "List all SCIM users"

---

### grammarly_scim_get_user

**What it does**: Gets detailed information about a specific SCIM user.

**When to use**: View user attributes and group memberships.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get SCIM user with ID abc123"

---

### grammarly_scim_create_user

**What it does**: Creates a new SCIM user in the Grammarly organization.

**When to use**: Provision a new user account.

**Arguments**:
- `userName` (required): Username (email)
- `name` (optional): Name object with givenName and familyName
- `emails` (optional): Email addresses
- `active` (optional): User active status (default true)

**Example LLM prompt**: "Create SCIM user for john@example.com"

---

### grammarly_scim_update_user

**What it does**: Updates an existing SCIM user.

**When to use**: Modify user attributes or deactivate user.

**Arguments**:
- `id` (required): User ID
- `userName` (optional): New username
- `name` (optional): New name object
- `active` (optional): New active status

**Example LLM prompt**: "Update SCIM user abc123 to deactivate"

---

### grammarly_scim_delete_user

**What it does**: Deletes a SCIM user from the Grammarly organization.

**When to use**: Remove a provisioned user.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete SCIM user abc123"

---

### grammarly_scim_list_groups

**What it does**: Lists all SCIM groups in the Grammarly organization.

**When to use**: View all groups, find specific groups.

**Arguments**:
- `startIndex` (optional): Start index (default 1)
- `count` (optional): Max results (default 20)

**Example LLM prompt**: "List all SCIM groups"

---

### grammarly_scim_get_group

**What it does**: Gets detailed information about a specific SCIM group.

**When to use**: View group members and attributes.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get SCIM group with ID grp456"

---

### grammarly_scim_create_group

**What it does**: Creates a new SCIM group.

**When to use**: Create a new team or permission group.

**Arguments**:
- `displayName` (required): Group display name
- `members` (optional): List of member IDs

**Example LLM prompt**: "Create SCIM group 'Engineering'"

---

### grammarly_scim_update_group

**What it does**: Updates an existing SCIM group.

**When to use**: Modify group name or membership.

**Arguments**:
- `id` (required): Group ID
- `displayName` (optional): New display name
- `members` (optional): New member list

**Example LLM prompt**: "Update SCIM group grp456 to add member abc123"

---

### grammarly_scim_list_resources

**What it does**: Lists all SCIM resource types available.

**When to use**: Discover available SCIM endpoints.

**Arguments**: None

**Example LLM prompt**: "List all available SCIM resources"

---

## Grammarly SCIM API Notes

- **API Base URL**: https://sso.grammarly.com
- **Protocol**: SCIM 2.0
- **Content-Type**: application/scim+json
- **Users**: Core identity resources with email, name, and active status
- **Groups**: Collections of users for role assignment
- **Filtering**: Supports SCIM filter expressions
