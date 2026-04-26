# Microsoft Admin Tools

Provider: `microsoft-admin` | Engine: `nango` | Auth: OAuth2 via Nango (Admin consent)

## Overview

These tools wrap the Microsoft Graph Admin API. They allow AI agents to manage users, groups, and directory roles with admin privileges. **Requires Microsoft OAuth2 with admin consent.**

## Authentication

**Nango OAUTH2 (Admin)**:
- User authenticates via Nango Connect with Microsoft (admin consent required)
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `microsoft_admin_list_users` | List all users (admin) | GET | /v1.0/users |
| `microsoft_admin_get_user` | Get user details (admin) | GET | /v1.0/users/{userId} |
| `microsoft_admin_create_user` | Create a user (admin) | POST | /v1.0/users |
| `microsoft_admin_update_user` | Update a user (admin) | PATCH | /v1.0/users/{userId} |
| `microsoft_admin_delete_user` | Delete a user (admin) | DELETE | /v1.0/users/{userId} |
| `microsoft_admin_list_groups` | List groups (admin) | GET | /v1.0/groups |
| `microsoft_admin_get_group` | Get group details (admin) | GET | /v1.0/groups/{groupId} |
| `microsoft_admin_create_group` | Create a group (admin) | POST | /v1.0/groups |
| `microsoft_admin_list_directory` | List directory roles | GET | /v1.0/directoryRoles |
| `microsoft_admin_assign_role` | Assign directory role | POST | /v1.0/directoryRoles/{roleId}/members/$ref |

---

## Tool Details

### microsoft_admin_list_users

**What it does**: Lists all users in the organization with admin privileges.

**When to use**: User management, directory browsing.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List all users created this month"

---

### microsoft_admin_get_user

**What it does**: Gets detailed information about any user in the organization.

**When to use**: User investigation, admin user details.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@example.com"

---

### microsoft_admin_create_user

**What it does**: Creates a new user in the organization.

**When to use**: Provision new users, create employee accounts.

**Arguments**:
- `accountEnabled` (optional): Enable user account (default true)
- `displayName` (required): Display name
- `mailNickname` (required): Mail nickname
- `userPrincipalName` (required): User principal name (email)
- `passwordProfile` (required): Password profile with password

**Example LLM prompt**: "Create a new user for John Doe with email john@company.com"

---

### microsoft_admin_update_user

**What it does**: Updates an existing user in the organization.

**When to use**: Modify user attributes, update department.

**Arguments**:
- `userId` (required): User ID
- `displayName` (optional): Display name
- `department` (optional): Department
- `jobTitle` (optional): Job title

**Example LLM prompt**: "Update user john@example.com to be in the Engineering department"

---

### microsoft_admin_delete_user

**What it does**: Deletes a user from the organization.

**When to use**: Remove former employees, deprovision users.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Delete user john@example.com"

---

### microsoft_admin_list_groups

**What it does**: Lists all groups in the organization.

**When to use**: Group management, find security groups.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all security groups"

---

### microsoft_admin_get_group

**What it does**: Gets details of a specific group.

**When to use**: Check group membership, group settings.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 12345678-1234-1234-1234-123456789012"

---

### microsoft_admin_create_group

**What it does**: Creates a new group in the organization.

**When to use**: Create teams, set up security groups.

**Arguments**:
- `displayName` (required): Group display name
- `mailNickname` (required): Mail nickname
- `description` (optional): Group description
- `securityEnabled` (optional): Enable security group (default true)
- `mailEnabled` (optional): Enable mail group (default false)

**Example LLM prompt**: "Create a new security group for Project Alpha"

---

### microsoft_admin_list_directory

**What it does**: Lists all directory roles in the organization.

**When to use**: Find available admin roles.

**Arguments**: None

**Example LLM prompt**: "List all directory roles"

---

### microsoft_admin_assign_role

**What it does**: Assigns a directory role to a user.

**When to use**: Grant admin privileges, assign roles.

**Arguments**:
- `roleId` (required): Directory role ID
- `userId` (required): User ID to assign

**Example LLM prompt**: "Assign Global Reader role to john@example.com"

---

## Microsoft Admin Notes

- **Admin consent required**: App must be granted admin consent
- **Powerful operations**: Create/delete users, assign roles
- **Directory roles**: Built-in roles (Global Admin, User Admin, etc.)
- **Security groups**: For access control and Teams
- **Audit trail**: Actions are logged in Azure AD
