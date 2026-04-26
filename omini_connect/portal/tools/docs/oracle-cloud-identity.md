# Oracle Cloud Identity Tools

Provider: `oracle-cloud-identity` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Oracle Identity Cloud Service API. They allow AI agents to manage users, groups, applications, and view audit logs. **Requires Oracle Cloud Identity OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses Oracle Cloud Identity OAuth client credentials
- Client ID passed via Nango
- Token URL: `https://{serviceInstance}.identity.oraclecloud.com/oauth2/v1/token`
- Base URL: `https://{serviceInstance}.identity.oraclecloud.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `oracle_list_users` | List users | GET | /admin/v1/Users |
| `oracle_get_user` | Get user details | GET | /admin/v1/Users/{userId} |
| `oracle_create_user` | Create user | POST | /admin/v1/Users |
| `oracle_update_user` | Update user | PUT | /admin/v1/Users/{userId} |
| `oracle_delete_user` | Delete user | DELETE | /admin/v1/Users/{userId} |
| `oracle_list_groups` | List groups | GET | /admin/v1/Groups |
| `oracle_add_user_to_group` | Add user to group | POST | /admin/v1/Groups/{groupId}/members |
| `oracle_list_applications` | List applications | GET | /admin/v1/applications |
| `oracle_get_reports` | Get reports | GET | /admin/v1/reports |
| `oracle_get_audit_logs` | Get audit logs | GET | /admin/v1/auditLogs |

---

## Tool Details

### oracle_list_users

**What it does**: Lists all users in Oracle Cloud Identity.

**When to use**: Browse user directory, find users.

**Arguments**:
- `limit` (optional): Number of users (default 50)

**Example LLM prompt**: "List all users in Oracle Cloud Identity"

---

### oracle_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user profile, status, group memberships.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user ABC123"

---

### oracle_create_user

**What it does**: Creates a new user in Oracle Cloud Identity.

**When to use**: Provision new users, onboard employees.

**Arguments**:
- `userName` (required): Username
- `email` (required): Email address
- `displayName` (optional): Display name

**Example LLM prompt**: "Create a new user for john@company.com"

---

### oracle_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user attributes, change status.

**Arguments**:
- `userId` (required): User ID
- `displayName` (optional): Display name
- `active` (optional): Active status

**Example LLM prompt**: "Deactivate user ABC123"

---

### oracle_delete_user

**What it does**: Deletes a user from Oracle Cloud Identity.

**When to use**: Remove users, offboard employees.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Delete user ABC123"

---

### oracle_list_groups

**What it does**: Lists all groups in Oracle Cloud Identity.

**When to use**: Browse group directory, find groups.

**Arguments**:
- `limit` (optional): Number of groups (default 50)

**Example LLM prompt**: "List all groups"

---

### oracle_add_user_to_group

**What it does**: Adds a user to a group.

**When to use**: Grant permissions, manage membership.

**Arguments**:
- `groupId` (required): Group ID
- `userId` (required): User ID

**Example LLM prompt**: "Add user ABC123 to group XYZ456"

---

### oracle_list_applications

**What it does**: Lists all applications in Oracle Cloud Identity.

**When to use**: Browse app catalog, find applications.

**Arguments**: None

**Example LLM prompt**: "List all applications"

---

### oracle_get_reports

**What it does**: Gets reports from Oracle Cloud Identity.

**When to use**: Generate analytics, export data.

**Arguments**: None

**Example LLM prompt**: "Get available reports"

---

### oracle_get_audit_logs

**What it does**: Gets audit logs from Oracle Cloud Identity.

**When to use**: Audit user activity, security investigations.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get audit logs for the past week"

---

## Oracle Cloud Identity Notes

- **Service Instance**: IDCS service instance identifier (format: `idcs-{32-hex}`)
- **OAuth2 CC**: Uses client credentials flow with basic auth
- **SCIM compliance**: Uses SCIM 2.0 protocol
- **User IDs**: Usually in GUID format
- **Audit**: Comprehensive audit trail for compliance
