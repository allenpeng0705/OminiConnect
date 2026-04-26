# ConnectWise RMM Tools

Provider: `connectwise-rmm` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the ConnectWise RMM API. ConnectWise RMM is a remote monitoring and management platform for IT managed services. **Requires ConnectWise RMM OAuth2 Client Credentials.**

## Authentication

**Nango OAuth2_CC**:
- Uses Client Credentials flow for server integration
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://openapi.service.${connectionConfig.subdomain}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `connectwise_rmm_list_devices` | List devices | GET | /v1/devices |
| `connectwise_rmm_get_device` | Get device details | GET | /v1/devices/{id} |
| `connectwise_rmm_list_alerts` | List alerts | GET | /v1/alerts |
| `connectwise_rmm_get_alert` | Get alert details | GET | /v1/alerts/{id} |
| `connectwise_rmm_list_tickets` | List tickets | GET | /v1/tickets |
| `connectwise_rmm_get_ticket` | Get ticket details | GET | /v1/tickets/{id} |
| `connectwise_rmm_create_ticket` | Create a ticket | POST | /v1/tickets |
| `connectwise_rmm_list_policies` | List policies | GET | /v1/policies |
| `connectwise_rmm_get_policy` | Get policy details | GET | /v1/policies/{id} |
| `connectwise_rmm_list_users` | List users | GET | /v1/users |

---

## Tool Details

### connectwise_rmm_list_devices

**What it does**: Lists all monitored devices.

**When to use**: View all managed endpoints.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all ConnectWise RMM devices"

---

### connectwise_rmm_get_device

**What it does**: Gets details of a specific device.

**When to use**: View device status and health.

**Arguments**:
- `id` (required): Device ID

**Example LLM prompt**: "Get device 123 details"

---

### connectwise_rmm_list_alerts

**What it does**: Lists all alerts from monitored devices.

**When to use**: View active issues and warnings.

**Arguments**:
- `deviceId` (optional): Filter by device
- `severity` (optional): Filter by severity
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List critical alerts for device 123"

---

### connectwise_rmm_get_alert

**What it does**: Gets details of a specific alert.

**When to use**: View alert details and history.

**Arguments**:
- `id` (required): Alert ID

**Example LLM prompt**: "Get alert 456 details"

---

### connectwise_rmm_list_tickets

**What it does**: Lists all tickets.

**When to use**: View support tickets.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List open tickets"

---

### connectwise_rmm_get_ticket

**What it does**: Gets details of a specific ticket.

**When to use**: View ticket details.

**Arguments**:
- `id` (required): Ticket ID

**Example LLM prompt**: "Get ticket 789 details"

---

### connectwise_rmm_create_ticket

**What it does**: Creates a new ticket.

**When to use**: Log a new issue.

**Arguments**:
- `title` (required): Ticket title
- `description` (required): Ticket description
- `deviceId` (optional): Related device ID

**Example LLM prompt**: "Create a ticket for device issue"

---

### connectwise_rmm_list_policies

**What it does**: Lists all monitoring policies.

**When to use**: View policy configurations.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all policies"

---

### connectwise_rmm_get_policy

**What it does**: Gets details of a specific policy.

**When to use**: View policy settings.

**Arguments**:
- `id` (required): Policy ID

**Example LLM prompt**: "Get policy 101 details"

---

### connectwise_rmm_list_users

**What it does**: Lists all users.

**When to use**: View user management.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all users"

---

## ConnectWise RMM API Notes

- **Subdomain**: Your ConnectWise RMM API subdomain
- **Devices**: Endpoints being monitored (workstations, servers)
- **Alerts**: Notifications from monitoring agents
- **Policies**: Monitoring configurations applied to devices
- **Tickets**: Support tickets for issues
