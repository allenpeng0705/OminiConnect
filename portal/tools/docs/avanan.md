# Avanan Tools

Provider: `avanan` | Engine: `nango` | Auth: TWO_STEP via Nango (custom signature-based auth)

## Overview

These tools wrap the Avanan API. They allow AI agents to manage users, security policies, alerts, and compliance reports. Avanan is a cloud email security platform that provides protection for Microsoft 365, Google Workspace, and other services.

## Authentication

**Nango TWO_STEP Auth**:
- User provides credentials (appId and secretKey)
- Nango generates signature-based tokens
- Token stored in Nango, accessed via `connection_ref`
- Requires subdomain configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `avanan_get_account_info` | Get account information | GET | /v1.0/account |
| `avanan_list_users` | List users in account | GET | /v1.0/users |
| `avanan_get_user` | Get user details | GET | /v1.0/users/{userId} |
| `avanan_list_policies` | List security policies | GET | /v1.0/policies |
| `avanan_get_policy` | Get policy details | GET | /v1.0/policies/{policyId} |
| `avanan_list_alerts` | List security alerts | GET | /v1.0/alerts |
| `avanan_get_alert` | Get alert details | GET | /v1.0/alerts/{alertId} |
| `avanan_list_domains` | List managed domains | GET | /v1.0/domains |
| `avanan_list_reports` | List available reports | GET | /v1.0/reports |
| `avanan_get_report` | Get report data | GET | /v1.0/reports/{reportId} |

---

## Tool Details

### avanan_get_account_info

**What it does**: Retrieves account information and configuration details.

**When to use**: Check account status and settings.

**Arguments**: None required

**Example LLM prompt**: "Get my Avanan account information"

---

### avanan_list_users

**What it does**: Lists all users in the protected account.

**When to use**: Find users, check user security status.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Users per page (default 50)

**Example LLM prompt**: "List all users in Avanan"

---

### avanan_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Check individual user security posture, view user activity.

**Arguments**:
- `userId` (required): User ID or email address

**Example LLM prompt**: "Get details for user john@company.com"

---

### avanan_list_policies

**What it does**: Lists all security policies configured in Avanan.

**When to use**: Review security policies, understand protection rules.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Items per page (default 20)

**Example LLM prompt**: "List all security policies in Avanan"

---

### avanan_get_policy

**What it does**: Gets details of a specific security policy.

**When to use**: Understand policy configuration and rules.

**Arguments**:
- `policyId` (required): Policy ID

**Example LLM prompt**: "Get details for policy ID 12345"

---

### avanan_list_alerts

**What it does**: Lists security alerts and incidents.

**When to use**: Monitor threats, review security incidents.

**Arguments**:
- `status` (optional): Filter by status (open, resolved, all)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Items per page (default 20)

**Example LLM prompt**: "List all open security alerts"

---

### avanan_get_alert

**What it does**: Gets detailed information about a specific alert.

**When to use**: Investigate specific security incidents.

**Arguments**:
- `alertId` (required): Alert ID

**Example LLM prompt**: "Get details for alert A-12345"

---

### avanan_list_domains

**What it does**: Lists all domains managed by Avanan.

**When to use**: Verify domain coverage, check protected domains.

**Arguments**: None required

**Example LLM prompt**: "List all domains protected by Avanan"

---

### avanan_list_reports

**What it does**: Lists available compliance and security reports.

**When to use**: Find reports for audit and compliance needs.

**Arguments**:
- `type` (optional): Report type filter

**Example LLM prompt**: "List available compliance reports"

---

### avanan_get_report

**What it does**: Retrieves data for a specific report.

**When to use**: Generate compliance reports, export security data.

**Arguments**:
- `reportId` (required): Report ID
- `startDate` (optional): Report start date
- `endDate` (optional): Report end date

**Example LLM prompt**: "Get the security report for the last 30 days"

---

## Avanan API Notes

- **Subdomain**: Each Avanan deployment has a unique subdomain
- **Alert Lifecycle**: Alerts can be open, investigating, resolved, or dismissed
- **Policy Types**: Includes malware, phishing, DLP, and compliance policies
- **User Status**: Users can be active, suspended, or quarantined
