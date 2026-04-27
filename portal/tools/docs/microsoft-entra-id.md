# Microsoft Entra ID Tools

Provider: `microsoft-entra-id` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft Entra ID API. They allow AI agents to manage users, groups, applications, and service principals. **Requires Microsoft Entra ID OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Entra ID)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `entra_list_users` | List all users | GET | /v1.0/users |
| `entra_get_user` | Get user details | GET | /v1.0/users/{userId} |
| `entra_list_groups` | List groups | GET | /v1.0/groups |
| `entra_get_group` | Get group details | GET | /v1.0/groups/{groupId} |
| `entra_list_applications` | List applications | GET | /v1.0/applications |
| `entra_get_application` | Get application details | GET | /v1.0/applications/{applicationId} |
| `entra_list_service_principals` | List service principals | GET | /v1.0/servicePrincipals |
| `entra_get_service_principal` | Get service principal details | GET | /v1.0/servicePrincipals/{servicePrincipalId} |
| `entra_list_directory_roles` | List directory roles | GET | /v1.0/directoryRoles |
| `entra_assign_user_to_role` | Assign user to directory role | POST | /v1.0/directoryRoles/{roleId}/members/$ref |

---

## Tool Details

### entra_list_users

**What it does**: Lists all users in Entra ID.

**When to use**: Browse users, find specific users.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$search` (optional): Search query
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all users in the Engineering department"

---

### entra_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, check user properties.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@example.com"

---

### entra_list_groups

**What it does**: Lists all groups in Entra ID.

**When to use**: Browse groups, find groups.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all groups"

---

### entra_get_group

**What it does**: Gets details of a specific group including members.

**When to use**: Check group membership, group details.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 12345678-1234-1234-1234-123456789012"

---

### entra_list_applications

**What it does**: Lists all registered applications in Entra ID.

**When to use**: Browse app registrations, find applications.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all applications"

---

### entra_get_application

**What it does**: Gets details of a specific application registration.

**When to use**: Check app settings, review app configuration.

**Arguments**:
- `applicationId` (required): Application ID

**Example LLM prompt**: "Get details for application 12345678-1234-1234-1234-123456789012"

---

### entra_list_service_principals

**What it does**: Lists all service principals in the directory.

**When to use**: Find service principals, manage app access.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all service principals"

---

### entra_get_service_principal

**What it does**: Gets details of a specific service principal.

**When to use**: Check service principal details.

**Arguments**:
- `servicePrincipalId` (required): Service Principal ID

**Example LLM prompt**: "Get details for service principal 12345678-1234-1234-1234-123456789012"

---

### entra_list_directory_roles

**What it does**: Lists all available directory roles in Entra ID.

**When to use**: Find available roles, understand permissions.

**Arguments**: None

**Example LLM prompt**: "List all directory roles"

---

### entra_assign_user_to_role

**What it does**: Assigns a user to a directory role.

**When to use**: Grant admin privileges.

**Arguments**:
- `roleId` (required): Directory Role ID
- `userId` (required): User ID to assign

**Example LLM prompt**: "Assign User Admin role to john@example.com"

---

## Entra ID Notes

- **Entra ID**: Microsoft's cloud identity platform (formerly Azure AD)
- **Application registrations**: App credentials management
- **Service principals**: App instances in tenants
- **Directory roles**: Built-in admin roles
- **Rate limits**: Graph API has rate limits
