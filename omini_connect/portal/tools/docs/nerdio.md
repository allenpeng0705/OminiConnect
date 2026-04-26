# Nerdio Tools

Provider: `nerdio` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Nerdio Manager API for Azure Virtual Desktop management. They allow AI agents to manage virtual desktops, hosts, sessions, users, and policies. **Requires Nerdio OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses Azure AD app registration credentials
- Client ID and Secret passed via Nango
- Token URL: `https://login.microsoftonline.com/{tenantId}/oauth2/v2.0/token`
- Base URL: `https://{hostname}/rest-api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nerdio_list_desktops` | List virtual desktops | GET | /v1/desktops |
| `nerdio_get_desktop` | Get desktop details | GET | /v1/desktops/{id} |
| `nerdio_list_hosts` | List host servers | GET | /v1/hosts |
| `nerdio_list_sessions` | List active sessions | GET | /v1/sessions |
| `nerdio_list_users` | List users | GET | /v1/users |
| `nerdio_get_user` | Get user details | GET | /v1/users/{id} |
| `nerdio_list_policies` | List management policies | GET | /v1/policies |
| `nerdio_list_reports` | List available reports | GET | /v1/reports |
| `nerdio_list_settings` | List system settings | GET | /v1/settings |
| `nerdio_get_status` | Get system status | GET | /v1/status |

---

## Tool Details

### nerdio_list_desktops

**What it does**: Lists all virtual desktops in the environment.

**When to use**: Browse desktops, check provisioning status.

**Arguments**:
- `status` (optional): Filter by status (running, stopped, provisioning)
- `page` (optional): Page number

**Example LLM prompt**: "List all desktops in stopped status"

---

### nerdio_get_desktop

**What it does**: Gets detailed information for a specific desktop.

**When to use**: Check desktop specs, view desktop history.

**Arguments**:
- `id` (required): Desktop ID

**Example LLM prompt**: "Get details for desktop ABC123"

---

### nerdio_list_hosts

**What it does**: Lists all host servers in the environment.

**When to use**: View infrastructure, check host health.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all hosts with their status"

---

### nerdio_list_sessions

**What it does**: Lists all active user sessions.

**When to use**: Monitor active users, track session usage.

**Arguments**:
- `desktop_id` (optional): Filter by desktop ID

**Example LLM prompt**: "List all active sessions"

---

### nerdio_list_users

**What it does**: Lists all users in the Nerdio environment.

**When to use**: Browse user directory, find users.

**Arguments**:
- `page` (optional): Page number

**Example LLM prompt**: "List all users in the system"

---

### nerdio_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user permissions, desktop assignments.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user user@company.com"

---

### nerdio_list_policies

**What it does**: Lists all management policies.

**When to use**: View policy settings, understand management rules.

**Arguments**: None

**Example LLM prompt**: "List all management policies"

---

### nerdio_list_reports

**What it does**: Lists available management reports.

**When to use**: Find relevant reports, generate analytics.

**Arguments**: None

**Example LLM prompt**: "What reports are available for usage analytics"

---

### nerdio_list_settings

**What it does**: Lists system settings and configuration.

**When to use**: Review system configuration, check defaults.

**Arguments**: None

**Example LLM prompt**: "List all system settings"

---

### nerdio_get_status

**What it does**: Gets overall system status and health.

**When to use**: Check system health, verify services running.

**Arguments**: None

**Example LLM prompt**: "Get overall system status"

---

## Nerdio API Notes

- **Azure AD**: Requires Azure App Registration with correct permissions
- **Hostnames**: Nerdio Manager instance URL format
- **Tenant ID**: Azure Active Directory Tenant ID
- **Scope**: Application ID URI scope from Azure registration
