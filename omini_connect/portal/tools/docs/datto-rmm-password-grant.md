# Datto RMM Password Grant Tools

Provider: `datto-rmm-password-grant` | Engine: `nango` | Auth: Password Grant via Nango

## Overview

These tools wrap the Datto RMM API using the Password Grant authentication flow. They allow AI agents to manage devices, alerts, policies, users, and tickets.

## Authentication

**Nango Password Grant (TWO_STEP)**:
- Uses username/password credentials for authentication
- Token stored in Nango, accessed via `connection_ref`
- Platform (region) configured per connection

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `datto_rmm_password_grant_list_devices` | List devices | GET | /devices |
| `datto_rmm_password_grant_get_device` | Get device details | GET | /devices/{deviceUid} |
| `datto_rmm_password_grant_list_alerts` | List alerts | GET | /alerts |
| `datto_rmm_password_grant_get_alert` | Get alert details | GET | /alerts/{alertId} |
| `datto_rmm_password_grant_list_policies` | List policies | GET | /policies |
| `datto_rmm_password_grant_get_policy` | Get policy details | GET | /policies/{policyId} |
| `datto_rmm_password_grant_list_users` | List users | GET | /users |
| `datto_rmm_password_grant_get_user` | Get user details | GET | /users/{userId} |
| `datto_rmm_password_grant_list_tickets` | List tickets | GET | /tickets |
| `datto_rmm_password_grant_get_ticket` | Get ticket details | GET | /tickets/{ticketId} |

---

## Tool Details

### datto_rmm_password_grant_list_devices

**What it does**: Lists all monitored devices in Datto RMM.

**When to use**: View device inventory, find devices by status, check monitored endpoints.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all monitored devices"

---

### datto_rmm_password_grant_get_device

**What it does**: Gets detailed device information.

**When to use**: Check device health, verify agent installation, review device configuration.

**Arguments**:
- `deviceUid` (required): Device UID

**Example LLM prompt**: "Get details for device d-123"

---

### datto_rmm_password_grant_list_alerts

**What it does**: Lists all alerts across monitored devices.

**When to use**: View active alerts, prioritize critical issues, track alert trends.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `severity` (optional): Filter by critical, warning, or info

**Example LLM prompt**: "List all critical alerts"

---

### datto_rmm_password_grant_get_alert

**What it does**: Gets detailed alert information.

**When to use**: Investigate specific alerts, understand alert causes, take corrective action.

**Arguments**:
- `alertId` (required): Alert ID

**Example LLM prompt**: "Get details for alert a-456"

---

### datto_rmm_password_grant_list_policies

**What it does**: Lists all policies for endpoint management.

**When to use**: View policy library, find applied policies, review security settings.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all policies"

---

### datto_rmm_password_grant_get_policy

**What it does**: Gets detailed policy configuration.

**When to use**: Review policy settings, check policy rules, verify policy assignments.

**Arguments**:
- `policyId` (required): Policy ID

**Example LLM prompt**: "Get details for policy p-789"

---

### datto_rmm_password_grant_list_users

**What it does**: Lists all users in Datto RMM.

**When to use**: View user list, find administrators, manage user access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all users"

---

### datto_rmm_password_grant_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user permissions, verify user details, manage user access.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user u-101"

---

### datto_rmm_password_grant_list_tickets

**What it does**: Lists all support tickets.

**When to use**: View open tickets, track support requests, manage support queue.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `status` (optional): Filter by open, closed, or pending

**Example LLM prompt**: "List all open tickets"

---

### datto_rmm_password_grant_get_ticket

**What it does**: Gets detailed ticket information.

**When to use**: Review ticket details, check ticket history, update ticket status.

**Arguments**:
- `ticketId` (required): Ticket ID

**Example LLM prompt**: "Get details for ticket t-202"

---

## Datto RMM Password Grant API Notes

- **Password Grant**: Alternative authentication using direct username/password credentials
- **Same API**: Identical API endpoints as standard Datto RMM OAuth flow
- **Devices**: Endpoints monitored by Datto RMM agent
- **Alerts**: Notifications from monitoring rules and thresholds
- **Policies**: Configuration templates applied to devices
- **Users**: Technician and administrator accounts
- **Tickets**: Support tickets for tracking issues
- **Platform**: Region-specific API endpoints
