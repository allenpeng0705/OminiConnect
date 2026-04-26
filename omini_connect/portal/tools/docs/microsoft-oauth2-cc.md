# Microsoft OAuth2 CC Tools

Provider: `microsoft-oauth2-cc` | Engine: `nango` | Auth: OAUTH2_CC via Nango (Client Credentials)

## Overview

These tools wrap the Microsoft Graph API using Client Credentials. They allow AI agents to manage users, groups, applications, and audit logs organization-wide. **Requires Microsoft Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client_id and client_secret via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ms_cc_list_users` | List all users | GET | /v1.0/users |
| `ms_cc_get_user` | Get user details | GET | /v1.0/users/{userId} |
| `ms_cc_list_groups` | List groups | GET | /v1.0/groups |
| `ms_cc_get_group` | Get group details | GET | /v1.0/groups/{groupId} |
| `ms_cc_list_applications` | List applications | GET | /v1.0/applications |
| `ms_cc_get_application` | Get application details | GET | /v1.0/applications/{applicationId} |
| `ms_cc_list_service_principals` | List service principals | GET | /v1.0/servicePrincipals |
| `ms_cc_list_audit_logs` | List audit logs | GET | /v1.0/auditLogs/directoryAudits |
| `ms_cc_list_sign_ins` | List sign-ins | GET | /v1.0/auditLogs/signIns |
| `ms_cc_list_reports` | Get usage reports | GET | /v1.0/reports/getOffice365ActivationsUserDetail |

---

## Tool Details

### ms_cc_list_users

**What it does**: Lists all users in the organization.

**When to use**: User directory, organization-wide user management.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List all users in the organization"

---

### ms_cc_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, check user properties.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@example.com"

---

### ms_cc_list_groups

**What it does**: Lists all groups in the organization.

**When to use**: Browse groups, find groups.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all groups"

---

### ms_cc_get_group

**What it does**: Gets details of a specific group.

**When to use**: Check group membership, group details.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 12345678-1234-1234-1234-123456789012"

---

### ms_cc_list_applications

**What it does**: Lists all registered applications.

**When to use**: App inventory, application management.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all registered applications"

---

### ms_cc_get_application

**What it does**: Gets details of a specific application.

**When to use**: Check app registration details.

**Arguments**:
- `applicationId` (required): Application ID

**Example LLM prompt**: "Get details for application 12345678-1234-1234-1234-123456789012"

---

### ms_cc_list_service_principals

**What it does**: Lists all service principals in the directory.

**When to use**: Find service principals, manage app access.

**Arguments**:
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all service principals"

---

### ms_cc_list_audit_logs

**What it does**: Lists audit logs for the organization.

**When to use**: Security auditing, compliance tracking.

**Arguments**:
- `$filter` (optional): OData filter expression
- `startDateTime` (optional): Start date
- `endDateTime` (optional): End date

**Example LLM prompt**: "List audit logs from the last 7 days"

---

### ms_cc_list_sign_ins

**What it does**: Lists sign-in logs for the organization.

**When to use**: Security monitoring, user activity tracking.

**Arguments**:
- `$filter` (optional): OData filter expression
- `startDateTime` (optional): Start date
- `endDateTime` (optional): End date

**Example LLM prompt**: "List sign-ins for user john@example.com"

---

### ms_cc_list_reports

**What it does**: Gets usage reports for the organization.

**When to use**: Usage analytics, license management.

**Arguments**:
- `period` (optional): Reporting period (D7, D30, D90, etc.)

**Example LLM prompt**: "Get Office 365 activation report for last 30 days"

---

## Client Credentials Notes

- **Machine-to-machine**: No user context required
- **Organization-wide**: Access to all resources
- **Audit logs**: Directory audit and sign-in logs
- **Reports**: Usage and adoption reports
- **Permissions**: Requires app-level Graph permissions
