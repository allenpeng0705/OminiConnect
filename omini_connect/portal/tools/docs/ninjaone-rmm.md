# NinjaOne RMM Tools

Provider: `ninjaone-rmm` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the NinjaOne RMM API. They allow AI agents to manage devices, monitor alerts, run automation scripts, and view dashboard information. **Requires NinjaOne OAuth2 Client Credentials authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses NinjaOne OAuth client credentials
- Client ID and Secret passed via Nango
- Token URL: `https://app.ninjarmm.com/ws/oauth/token`
- Base URL: `https://app.ninjarmm.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ninjaone_list_devices` | List devices | GET | /v1/devices |
| `ninjaone_get_device` | Get device details | GET | /v1/devices/{id} |
| `ninjaone_list_alerts` | List alerts | GET | /v1/alerts |
| `ninjaone_list_policies` | List policies | GET | /v1/policies |
| `ninjaone_list_schedules` | List schedules | GET | /v1/schedules |
| `ninjaone_list_scripts` | List automation scripts | GET | /v1/scripts |
| `ninjaone_run_script` | Run automation script | POST | /v1/devices/{deviceId}/scripts/{scriptId}/run |
| `ninjaone_list_users` | List users | GET | /v1/users |
| `ninjaone_get_dashboard` | Get dashboard summary | GET | /v1/dashboard |
| `ninjaone_list_locations` | List locations | GET | /v1/locations |

---

## Tool Details

### ninjaone_list_devices

**What it does**: Lists all devices monitored by NinjaOne RMM.

**When to use**: Browse monitored devices, check device status.

**Arguments**:
- `page` (optional): Page number (default 1)
- `size` (optional): Page size (default 50)
- `status` (optional): Filter by status (online, offline)

**Example LLM prompt**: "List all offline devices"

---

### ninjaone_get_device

**What it does**: Gets detailed information for a specific device.

**When to use**: View device specs, check last backup, view issues.

**Arguments**:
- `id` (required): Device ID

**Example LLM prompt**: "Get details for device 12345"

---

### ninjaone_list_alerts

**What it does**: Lists all alerts in NinjaOne RMM.

**When to use**: Monitor issues, track warnings.

**Arguments**:
- `device_id` (optional): Filter by device ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all critical alerts"

---

### ninjaone_list_policies

**What it does**: Lists all management policies.

**When to use**: View policy settings, understand management rules.

**Arguments**: None

**Example LLM prompt**: "List all management policies"

---

### ninjaone_list_schedules

**What it does**: Lists all scheduled tasks.

**When to use**: View maintenance schedules, upcoming tasks.

**Arguments**: None

**Example LLM prompt**: "List all scheduled maintenance tasks"

---

### ninjaone_list_scripts

**What it does**: Lists all automation scripts.

**When to use**: Browse available automations, find scripts.

**Arguments**: None

**Example LLM prompt**: "List all available automation scripts"

---

### ninjaone_run_script

**What it does**: Runs an automation script on a device.

**When to use**: Execute maintenance, run automated tasks.

**Arguments**:
- `deviceId` (required): Device ID
- `scriptId` (required): Script ID
- `parameters` (optional): Script parameters

**Example LLM prompt**: "Run the backup script on device 12345"

---

### ninjaone_list_users

**What it does**: Lists all users in NinjaOne.

**When to use**: View user directory, manage access.

**Arguments**: None

**Example LLM prompt**: "List all NinjaOne users"

---

### ninjaone_get_dashboard

**What it does**: Gets dashboard summary and overview.

**When to use**: Get quick status overview, system health.

**Arguments**: None

**Example LLM prompt**: "Get the dashboard summary"

---

### ninjaone_list_locations

**What it does**: Lists all locations in NinjaOne.

**When to use**: View organizational structure, geographic分布.

**Arguments**: None

**Example LLM prompt**: "List all locations"

---

## NinjaOne RMM Notes

- **OAuth2 CC**: Uses client credentials flow for service authentication
- **Device IDs**: Numeric identifiers for monitored devices
- **Script execution**: Scripts run asynchronously on devices
- **Alert severity**: Critical, warning, info levels
- **Rate limits**: Implement backoff for heavy API usage
