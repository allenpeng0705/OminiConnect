# Kandji Tools

Provider: `kandji` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Kandji API. They allow AI agents to manage devices, blueprints, apps, and library items. **Requires Kandji API token.**

## Authentication

**API Key via Nango**:
- User provides their Kandji API token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{subdomain}.kandji.io`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `kandji_list_devices` | List devices | GET | /api/v1/devices |
| `kandji_get_device` | Get device details | GET | /api/v1/devices/{device_id} |
| `kandji_list_blueprints` | List blueprints | GET | /api/v1/blueprints |
| `kandji_get_blueprint` | Get blueprint details | GET | /api/v1/blueprints/{blueprint_id} |
| `kandji_list_apps` | List apps | GET | /api/v1/apps |
| `kandji_get_app` | Get app details | GET | /api/v1/apps/{app_id} |
| `kandji_list_library_items` | List library items | GET | /api/v1/library |
| `kandji_get_library_item` | Get library item details | GET | /api/v1/library/{library_item_id} |
| `kandji_list_users` | List users | GET | /api/v1/users |
| `kandji_get_device_activity` | Get device activity | GET | /api/v1/devices/{device_id}/activity |

---

## Tool Details

### kandji_list_devices

**What it does**: Lists all devices in Kandji.

**When to use**: Find devices, view device list.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List all devices in Kandji"

---

### kandji_get_device

**What it does**: Gets details for a specific device.

**When to use**: Get device information, view device status.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get details for device abc123"

---

### kandji_list_blueprints

**What it does**: Lists all blueprints.

**When to use**: View blueprints, find configuration profiles.

**Arguments**: None

**Example LLM prompt**: "List all blueprints in Kandji"

---

### kandji_get_blueprint

**What it does**: Gets details for a specific blueprint.

**When to use**: Get blueprint information.

**Arguments**:
- `blueprint_id` (required): Blueprint ID

**Example LLM prompt**: "Get details for blueprint xyz789"

---

### kandji_list_apps

**What it does**: Lists all apps.

**When to use**: View apps, find available software.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all apps in Kandji"

---

### kandji_get_app

**What it does**: Gets details for a specific app.

**When to use**: Get app information.

**Arguments**:
- `app_id` (required): App ID

**Example LLM prompt**: "Get details for app a1"

---

### kandji_list_library_items

**What it does**: Lists all library items.

**When to use**: View library, find configurations.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all library items in Kandji"

---

### kandji_get_library_item

**What it does**: Gets details for a specific library item.

**When to use**: Get library item information.

**Arguments**:
- `library_item_id` (required): Library Item ID

**Example LLM prompt**: "Get details for library item l1"

---

### kandji_list_users

**What it does**: Lists all users.

**When to use**: View users, manage access.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all users in Kandji"

---

### kandji_get_device_activity

**What it does**: Gets activity for a specific device.

**When to use**: Track device activity, troubleshoot issues.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get activity for device abc123"

---

## Kandji API Notes

- **Device Management**: MDM solution for Apple devices
- **Blueprints**: Configuration profiles and settings
- **Library Items**: Reusable configurations and scripts
- **Pagination**: Use limit and offset parameters
