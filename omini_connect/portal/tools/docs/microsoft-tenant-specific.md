# Microsoft Tenant Specific Tools

Provider: `microsoft-tenant-specific` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Microsoft Graph API for a specific tenant. They allow AI agents to manage users, groups, applications, and audit logs in a dedicated tenant. **Requires Microsoft OAuth2 with tenant-specific access.**

## Authentication

**Nango OAUTH2 (Tenant-Specific)**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ms_tenant_list_users` | List all users | GET | /v1.0/users |
| `ms_tenant_get_user` | Get user details | GET | /v1.0/users/{userId} |
| `ms_tenant_list_groups` | List groups | GET | /v1.0/groups |
| `ms_tenant_get_group` | Get group details | GET | /v1.0/groups/{groupId} |
| `ms_tenant_list_applications` | List applications | GET | /v1.0/applications |
| `ms_tenant_get_application` | Get application details | GET | /v1.0/applications/{applicationId} |
| `ms_tenant_list_service_principals` | List service principals | GET | /v1.0/servicePrincipals |
| `ms_tenant_list_directory_roles` | List directory roles | GET | /v1.0/directoryRoles |
| `ms_tenant_list_audit_logs` | List audit logs | GET | /v1.0/auditLogs/directoryAudits |
| `ms_tenant_list_sign_ins` | List sign-ins | GET | /v1.0/auditLogs/signIns |

---

## Tool Details

### ms_tenant_list_users

**What it does**: Lists all users in the specific tenant.

**When to use**: User management, directory browsing.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all users in the tenant"

---

### ms_tenant_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, check user properties.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@example.com"

---

### ms_tenant_list_groups

**What it does**: Lists all groups in the tenant.

**When to use**: Browse groups, find groups.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all groups in the tenant"

---

### ms_tenant_get_group

**What it does**: Gets details of a specific group.

**When to use**: Check group membership, group details.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 12345678-1234-1234-1234-123456789012"

---

### ms_tenant_list_applications

**What it does**: Lists all registered applications in the tenant.

**When to use**: App inventory, application management.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all applications in the tenant"

---

### ms_tenant_get_application

**What it does**: Gets details of a specific application.

**When to use**: Check app registration details.

**Arguments**:
- `applicationId` (required): Application ID

**Example LLM prompt**: "Get details for application 12345678-1234-1234-1234-123456789012"

---

### ms_tenant_list_service_principals

**What it does**: Lists all service principals in the tenant.

**When to use**: Find service principals, manage app access.

**Arguments**:
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all service principals"

---

### ms_tenant_list_directory_roles

**What it does**: Lists all directory roles in the tenant.

**When to use**: Find available roles, understand permissions.

**Arguments**: None

**Example LLM prompt**: "List all directory roles"

---

### ms_tenant_list_audit_logs

**What it does**: Lists audit logs for the tenant.

**When to use**: Security auditing, compliance tracking.

**Arguments**:
- `$filter` (optional): OData filter expression
- `startDateTime` (optional): Start date
- `endDateTime` (optional): End date

**Example LLM prompt**: "List audit logs from the last 7 days"

---

### ms_tenant_list_sign_ins

**What it does**: Lists sign-in logs for the tenant.

**When to use**: Security monitoring, user activity tracking.

**Arguments**:
- `$filter` (optional): OData filter expression
- `startDateTime` (optional): Start date
- `endDateTime` (optional): End date

**Example LLM prompt**: "List sign-ins for user john@example.com"

---

## Tenant-Specific Notes

- **Dedicated tenant**: Isolated to specific organization
- **Admin access**: Requires tenant-admin permissions
- **Audit logs**: Full audit trail of directory changes
- **Sign-in logs**: User authentication events
- **Graph API**: Standard Microsoft Graph endpoints
