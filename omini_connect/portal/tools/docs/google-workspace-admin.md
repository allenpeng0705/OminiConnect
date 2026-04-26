# Google Workspace Admin Tools

Provider: `google-workspace-admin` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Workspace Admin API. They allow AI agents to manage users, groups, organizational units, and domains. **Requires Google OAuth2 with Admin SDK permissions.**

## Authentication

**Nango OAUTH2 (Google Workspace Admin)**:
- User authenticates via OAuth2 with Admin SDK scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://admin.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_workspace_admin_list_users` | List users | GET | /admin/directory/v1/users |
| `google_workspace_admin_get_user` | Get user details | GET | /admin/directory/v1/users/{user_key} |
| `google_workspace_admin_create_user` | Create user | POST | /admin/directory/v1/users |
| `google_workspace_admin_update_user` | Update user | PATCH | /admin/directory/v1/users/{user_key} |
| `google_workspace_admin_delete_user` | Delete user | DELETE | /admin/directory/v1/users/{user_key} |
| `google_workspace_admin_list_groups` | List groups | GET | /admin/directory/v1/groups |
| `google_workspace_admin_get_group` | Get group details | GET | /admin/directory/v1/groups/{group_key} |
| `google_workspace_admin_list_org_units` | List org units | GET | /admin/directory/v1/orgunits |
| `google_workspace_admin_get_org_unit` | Get org unit details | GET | /admin/directory/v1/orgunits/{org_unit_path} |
| `google_workspace_admin_list_domains` | List domains | GET | /admin/directory/v1/domains |

---

## Tool Details

### google_workspace_admin_list_users

**What it does**: Lists users in the organization.

**When to use**: Browse organization users.

**Arguments**:
- `domain` (optional): Domain filter
- `maxResults` (optional): Max results (default 100)

**Example LLM prompt**: "List all users in my organization"

---

### google_workspace_admin_get_user

**What it does**: Gets user details.

**When to use**: Get user info.

**Arguments**:
- `user_key` (required): User email or ID

**Example LLM prompt**: "Get details for user john@company.com"

---

### google_workspace_admin_create_user

**What it does**: Creates a new user.

**When to use**: Add new users to organization.

**Arguments**:
- `primaryEmail` (required): Primary email
- `name` (required): Name object
- `password` (required): Password

**Example LLM prompt**: "Create user john@company.com with name John Doe"

---

### google_workspace_admin_update_user

**What it does**: Updates a user.

**When to use**: Modify user details.

**Arguments**:
- `user_key` (required): User email or ID
- `name` (optional): New name object

**Example LLM prompt**: "Update user john@company.com"

---

### google_workspace_admin_delete_user

**What it does**: Deletes a user.

**When to use**: Remove users from organization.

**Arguments**:
- `user_key` (required): User email or ID

**Example LLM prompt**: "Delete user john@company.com"

---

### google_workspace_admin_list_groups

**What it does**: Lists groups in the organization.

**When to use**: Browse organization groups.

**Arguments**:
- `domain` (optional): Domain filter

**Example LLM prompt**: "List all groups"

---

### google_workspace_admin_get_group

**What it does**: Gets group details.

**When to use**: Get group info.

**Arguments**:
- `group_key` (required): Group email or ID

**Example LLM prompt**: "Get details for group engineering@company.com"

---

### google_workspace_admin_list_org_units

**What it does**: Lists organizational units.

**When to use**: See organization structure.

**Arguments**:
- `customer` (optional): Customer ID

**Example LLM prompt**: "List all org units"

---

### google_workspace_admin_get_org_unit

**What it does**: Gets organizational unit details.

**When to use**: Get OU info.

**Arguments**:
- `customer` (optional): Customer ID
- `org_unit_path` (required): Org unit path

**Example LLM prompt**: "Get org unit /engineering"

---

### google_workspace_admin_list_domains

**What it does**: Lists domains in the organization.

**When to use**: See verified domains.

**Arguments**:
- `customer` (optional): Customer ID

**Example LLM prompt**: "List all domains"

---

## Google Workspace Admin API Notes

- **Admin SDK**: Requires admin privileges
- **User keys**: Can use email or unique ID
- **Org units**: Hierarchical organization structure
- **Domains**: Verified domains in the organization
- **Customer ID**: Usually the domain or a unique ID
