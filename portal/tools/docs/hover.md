# Hover Tools

Provider: `hover` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Hover API. They allow AI agents to manage projects, captures, devices, and alerts. Hover is a home monitoring and management platform.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Hover
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://hover.to/oauth/authorize
- Token URL: https://hover.to/oauth/token
- Auth method: basic
- Scope: `all`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `hover_list_projects` | List projects | GET | /api/v1/projects |
| `hover_get_project` | Get project details | GET | /api/v1/projects/{id} |
| `hover_list_captures` | List captures | GET | /api/v1/captures |
| `hover_get_capture` | Get capture details | GET | /api/v1/captures/{id} |
| `hover_list_users` | List users | GET | /api/v1/users |
| `hover_get_user` | Get user details | GET | /api/v1/users/{id} |
| `hover_list_devices` | List devices | GET | /api/v1/devices |
| `hover_get_device` | Get device details | GET | /api/v1/devices/{id} |
| `hover_list_alerts` | List alerts | GET | /api/v1/alerts |
| `hover_list_locations` | List locations | GET | /api/v1/locations |

---

## Tool Details

### hover_list_projects

**What it does**: Lists all projects in Hover.

**When to use**: Browse home monitoring projects.

**Arguments**: None

**Example LLM prompt**: "List all projects"

---

### hover_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: View project details and captures.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get project with ID 123"

---

### hover_list_captures

**What it does**: Lists all captures in Hover.

**When to use**: Browse captured media.

**Arguments**:
- `project_id` (optional): Filter by project ID

**Example LLM prompt**: "List all captures for project 123"

---

### hover_get_capture

**What it does**: Gets detailed information about a specific capture.

**When to use**: View capture content.

**Arguments**:
- `id` (required): Capture ID

**Example LLM prompt**: "Get capture with ID 456"

---

### hover_list_users

**What it does**: Lists all users in Hover.

**When to use**: View team members.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### hover_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: View user profile.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user with ID 789"

---

### hover_list_devices

**What it does**: Lists all devices in Hover.

**When to use**: Browse connected devices.

**Arguments**: None

**Example LLM prompt**: "List all devices"

---

### hover_get_device

**What it does**: Gets detailed information about a specific device.

**When to use**: View device settings and status.

**Arguments**:
- `id` (required): Device ID

**Example LLM prompt**: "Get device with ID 101"

---

### hover_list_alerts

**What it does**: Lists all alerts in Hover.

**When to use**: View system alerts.

**Arguments**: None

**Example LLM prompt**: "List all alerts"

---

### hover_list_locations

**What it does**: Lists all locations in Hover.

**When to use**: Browse monitored locations.

**Arguments**: None

**Example LLM prompt**: "List all locations"

---

## Hover API Notes

- **API Base URL**: https://hover.to/api/v1
- **Auth Mode**: OAuth2 with `all` scope for full access
- **Projects**: Home monitoring projects
- **Captures**: Media captured by devices
- **Users**: Account team members
- **Devices**: Connected monitoring devices
- **Alerts**: System notifications
- **Locations**: Physical locations with devices
