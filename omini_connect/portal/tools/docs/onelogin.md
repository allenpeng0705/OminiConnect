# OneLogin Tools

Provider: `onelogin` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the OneLogin API for identity and access management. They allow AI agents to manage users, groups, roles, and view audit events. **Requires OneLogin OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses OneLogin OAuth client credentials
- Client ID passed via Nango
- Token URL: `https://{subdomain}.onelogin.com/auth/oauth2/v2/token`
- Base URL: `https://{subdomain}.onelogin.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `onelogin_list_users` | List users | GET | /api/v2/users |
| `onelogin_get_user` | Get user details | GET | /api/v2/users/{id} |
| `onelogin_create_user` | Create user | POST | /api/v2/users |
| `onelogin_update_user` | Update user | PUT | /api/v2/users/{id} |
| `onelogin_list_groups` | List groups | GET | /api/v2/groups |
| `onelogin_list_roles` | List roles | GET | /api/v2/roles |
| `onelogin_assign_role` | Assign role to user | POST | /api/v2/users/{id}/roles |
| `onelogin_list_applications` | List applications | GET | /api/v2/apps |
| `onelogin_get_reports` | Get reports | GET | /api/v2/reports |
| `onelogin_get_events` | Get events | GET | /api/v2/events |

---

## Tool Details

### onelogin_list_users

**What it does**: Lists all users in OneLogin.

**When to use**: Browse user directory, find users.

**Arguments**:
- `limit` (optional): Number of users (default 50)

**Example LLM prompt**: "List all users in OneLogin"

---

### onelogin_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user profile, status, assigned roles.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user ID 12345"

---

### onelogin_create_user

**What it does**: Creates a new user in OneLogin.

**When to use**: Provision new users, onboard employees.

**Arguments**:
- `email` (required): User email
- `first_name` (required): First name
- `last_name` (required): Last name

**Example LLM prompt**: "Create a new user for John Doe with email john@company.com"

---

### onelogin_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user attributes, change status.

**Arguments**:
- `id` (required): User ID
- `email` (optional): User email
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update user 12345 with new department"

---

### onelogin_list_groups

**What it does**: Lists all groups in OneLogin.

**When to use**: Browse group directory, find groups.

**Arguments**: None

**Example LLM prompt**: "List all groups in OneLogin"

---

### onelogin_list_roles

**What it does**: Lists all roles in OneLogin.

**When to use**: Browse available roles, understand permissions.

**Arguments**: None

**Example LLM prompt**: "List all roles"

---

### onelogin_assign_role

**What it does**: Assigns a role to a user.

**When to use**: Grant permissions, assign app access.

**Arguments**:
- `id` (required): User ID
- `role_id` (required): Role ID to assign

**Example LLM prompt**: "Assign role 10 to user 12345"

---

### onelogin_list_applications

**What it does**: Lists all applications in OneLogin.

**When to use**: Browse app catalog, find applications.

**Arguments**: None

**Example LLM prompt**: "List all applications"

---

### onelogin_get_reports

**What it does**: Gets reports from OneLogin.

**When to use**: Generate analytics, export data.

**Arguments**: None

**Example LLM prompt**: "Get available reports"

---

### onelogin_get_events

**What it does**: Gets events/audit logs from OneLogin.

**When to use**: Audit user activity, security investigations.

**Arguments**:
- `limit` (optional): Number of events (default 50)

**Example LLM prompt**: "Get recent events for user 12345"

---

## OneLogin Notes

- **Subdomain**: Your OneLogin subdomain
- **Client credentials**: Uses OAuth2 client credentials flow
- **User IDs**: Numeric identifiers
- **Roles**: Define permissions and app access
- **Events**: Audit log for security monitoring
