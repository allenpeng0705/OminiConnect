# Kintone User API Tools

Provider: `kintone-user-api` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Kintone User API. They allow AI agents to manage users, departments, groups, and organizations. **Requires Kintone API token.**

## Authentication

**API Key via Nango**:
- User provides their Kintone API token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{subdomain}.kintone.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `kintone_user_api_list_users` | List users | GET | /v1/users.json |
| `kintone_user_api_get_user` | Get user details | GET | /v1/users/{user_id}.json |
| `kintone_user_api_list_departments` | List departments | GET | /v1/departments.json |
| `kintone_user_api_get_department` | Get department details | GET | /v1/departments/{department_id}.json |
| `kintone_user_api_list_groups` | List groups | GET | /v1/groups.json |
| `kintone_user_api_get_group` | Get group details | GET | /v1/groups/{group_id}.json |
| `kintone_user_api_get_organizations` | Get organizations | GET | /v1/organizations.json |
| `kintone_user_api_list_employees` | List employees | GET | /v1/employees.json |
| `kintone_user_api_get_user_organizations` | Get user organizations | GET | /v1/userOrganizations.json |
| `kintone_user_api_get_user_groups` | Get user groups | GET | /v1/userGroups.json |

---

## Tool Details

### kintone_user_api_list_users

**What it does**: Lists all users.

**When to use**: Find users, view user list.

**Arguments**:
- `offset` (optional): Offset for pagination (default: 0)
- `size` (optional): Number of results (default: 100)

**Example LLM prompt**: "List all users in Kintone"

---

### kintone_user_api_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information, view user profile.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### kintone_user_api_list_departments

**What it does**: Lists all departments.

**When to use**: View departments, organize structure.

**Arguments**:
- `offset` (optional): Offset for pagination (default: 0)
- `size` (optional): Number of results (default: 100)

**Example LLM prompt**: "List all departments in Kintone"

---

### kintone_user_api_get_department

**What it does**: Gets details for a specific department.

**When to use**: Get department information.

**Arguments**:
- `department_id` (required): Department ID

**Example LLM prompt**: "Get details for department xyz789"

---

### kintone_user_api_list_groups

**What it does**: Lists all groups.

**When to use**: View groups, manage permissions.

**Arguments**:
- `offset` (optional): Offset for pagination (default: 0)
- `size` (optional): Number of results (default: 100)

**Example LLM prompt**: "List all groups in Kintone"

---

### kintone_user_api_get_group

**What it does**: Gets details for a specific group.

**When to use**: Get group information.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group g1"

---

### kintone_user_api_get_organizations

**What it does**: Gets organization information.

**When to use**: View organization structure.

**Arguments**:
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "Get organization info"

---

### kintone_user_api_list_employees

**What it does**: Lists all employees.

**When to use**: View employees, find staff.

**Arguments**:
- `offset` (optional): Offset for pagination (default: 0)
- `size` (optional): Number of results (default: 100)

**Example LLM prompt**: "List all employees in Kintone"

---

### kintone_user_api_get_user_organizations

**What it does**: Gets organizations for a user.

**When to use**: Find user's organization.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get organizations for user abc123"

---

### kintone_user_api_get_user_groups

**What it does**: Gets groups for a user.

**When to use**: Find user's group memberships.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get groups for user abc123"

---

## Kintone User API Notes

- **User Management**: Separate API for user/org management
- **IDs**: String IDs for users, departments, groups
- **Pagination**: Use offset and size parameters
- **Organizations**: Hierarchical structure of departments
