# Mimecast Tools

Provider: `mimecast` | Engine: `nango` | Auth: OAUTH2_CC via Nango (Client Credentials)

## Overview

These tools wrap the Mimecast API. They allow AI agents to manage audit logs, managed URLs, templates, groups, and domains. **Requires Mimecast Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client credentials via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.services.mimecast.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mimecast_list_audit_logs` | List audit logs | POST | /api/audit/get-audit-events |
| `mimecast_list_managed_urls` | List managed URLs | POST | /api/managed-headers/get-managed-urls |
| `mimecast_create_managed_url` | Create managed URL | POST | /api/managed-headers/create-managed-url |
| `mimecast_list_templates` | List templates | POST | /api/templates/get-templates |
| `mimecast_get_template` | Get template details | POST | /api/templates/get-template |
| `mimecast_list_groups` | List groups | POST | /api/directory/list-groups |
| `mimecast_get_group` | Get group details | POST | /api/directory/get-group |
| `mimecast_list_domains` | List domains | POST | /api/domain/get-domains |
| `mimecast_list_accounts` | List accounts | POST | /api/account/get-accounts |
| `mimecast_get_account` | Get account details | POST | /api/account/get-account |

---

## Tool Details

### mimecast_list_audit_logs

**What it does**: Lists audit logs from Mimecast.

**When to use**: Security auditing, compliance tracking.

**Arguments**:
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)
- `event_type` (optional): Filter by event type
- `user` (optional): Filter by user email

**Example LLM prompt**: "List all audit logs from the last week"

---

### mimecast_list_managed_urls

**What it does**: Lists all managed URLs in Mimecast.

**When to use**: Review URL policies, find managed URLs.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all managed URLs"

---

### mimecast_create_managed_url

**What it does**: Creates a new managed URL entry.

**When to use**: Add URL to block/permit list.

**Arguments**:
- `url` (required): URL to manage
- `action` (required): Action to take (permit, block, warn)

**Example LLM prompt**: "Block URL https://suspicious-site.com"

---

### mimecast_list_templates

**What it does**: Lists all email templates in Mimecast.

**When to use**: Browse templates, find template IDs.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all email templates"

---

### mimecast_get_template

**What it does**: Gets details of a specific template.

**When to use**: Get template content, check template settings.

**Arguments**:
- `id` (required): Template ID

**Example LLM prompt**: "Get details for template 12345"

---

### mimecast_list_groups

**What it does**: Lists all distribution groups in Mimecast.

**When to use**: Manage distribution lists, find groups.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all distribution groups"

---

### mimecast_get_group

**What it does**: Gets details of a specific group.

**When to use**: Check group members, group settings.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get details for group 12345"

---

### mimecast_list_domains

**What it does**: Lists all configured domains in Mimecast.

**When to use**: Verify domain configuration.

**Arguments**: None

**Example LLM prompt**: "List all configured domains"

---

### mimecast_list_accounts

**What it does**: Lists all accounts in Mimecast.

**When to use**: Multi-account management.

**Arguments**: None

**Example LLM prompt**: "List all Mimecast accounts"

---

### mimecast_get_account

**What it does**: Gets details of a specific account.

**When to use**: Check account settings.

**Arguments**:
- `account` (required): Account key

**Example LLM prompt**: "Get details for account ABC123"

---

## Mimecast Notes

- **Email security**: Cloud-based email security
- **Managed URLs**: URL filtering and protection
- **Templates**: Email templates for consistent messaging
- **Groups**: Distribution groups for email routing
- **Audit logs**: Security and compliance logging
