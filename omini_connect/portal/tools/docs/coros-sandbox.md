# Coros (Sandbox) Tools

Provider: `coros-sandbox` | Engine: `nango` | Auth: OAuth via Nango (Sandbox)

## Overview

These tools wrap the Coros Sandbox API for testing purposes. They provide the same functionality as the production Coros API but use a separate test environment.

## Authentication

**Nango OAuth (Sandbox)**:
- User authenticates via Nango Connect with Coros Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `athlete:read`, `athlete:write`, `workout:read`, `device:read`, `team:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `coros_sandbox_list_activities` | List user activities | GET | /v1/activities |
| `coros_sandbox_get_activity` | Get a specific activity | GET | /v1/activities/{activity_id} |
| `coros_sandbox_list_workouts` | List user workouts | GET | /v1/workouts |
| `coros_sandbox_get_workout` | Get a specific workout | GET | /v1/workouts/{workout_id} |
| `coros_sandbox_list_devices` | List paired devices | GET | /v1/devices |
| `coros_sandbox_get_device` | Get device details | GET | /v1/devices/{device_id} |
| `coros_sandbox_list_athlete_profile` | Get athlete profile | GET | /v1/athlete |
| `coros_sandbox_update_athlete_profile` | Update athlete profile | PUT | /v1/athlete |
| `coros_sandbox_list_teams` | List teams | GET | /v1/teams |
| `coros_sandbox_get_team` | Get team details | GET | /v1/teams/{team_id} |

---

## Tool Details

### coros_sandbox_list_activities

**What it does**: Lists all activities recorded in the sandbox environment.

**When to use**: Test activity listing, develop against mock data.

**Arguments**:
- `start_date` (optional): ISO 8601 date - filter activities from this date
- `end_date` (optional): ISO 8601 date - filter activities until this date
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all activities in sandbox"

---

### coros_sandbox_get_activity

**What it does**: Gets detailed activity information.

**When to use**: Test activity retrieval, verify activity data structure.

**Arguments**:
- `activity_id` (required): Activity ID

**Example LLM prompt**: "Get details for activity abc123"

---

### coros_sandbox_list_workouts

**What it does**: Lists all workouts in sandbox.

**When to use**: Test workout listing functionality.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all workouts in sandbox"

---

### coros_sandbox_get_workout

**What it does**: Gets workout details.

**When to use**: Test workout retrieval.

**Arguments**:
- `workout_id` (required): Workout ID

**Example LLM prompt**: "Get details for workout xyz789"

---

### coros_sandbox_list_devices

**What it does**: Lists sandbox devices.

**Arguments**: None

**Example LLM prompt**: "List all sandbox devices"

---

### coros_sandbox_get_device

**What it does**: Gets device details.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get details for device d123"

---

### coros_sandbox_list_athlete_profile

**What it does**: Gets athlete profile.

**Arguments**: None

**Example LLM prompt**: "Get my sandbox athlete profile"

---

### coros_sandbox_update_athlete_profile

**What it does**: Updates athlete profile.

**Arguments**:
- `height` (optional): Height in cm
- `weight` (optional): Weight in kg
- `max_heart_rate` (optional): Maximum heart rate
- `threshold_heart_rate` (optional): Threshold heart rate
- `max_power` (optional): Maximum power (FTP)

**Example LLM prompt**: "Update sandbox profile with weight 80kg"

---

### coros_sandbox_list_teams

**What it does**: Lists teams.

**Arguments**: None

**Example LLM prompt**: "List all sandbox teams"

---

### coros_sandbox_get_team

**What it does**: Gets team details.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team t456"

---

## Sandbox Environment Notes

- **Testing Only**: This environment is for testing and development
- **Same API Structure**: Uses identical endpoints and data structures as production
- **Sandbox URLs**: Uses opentest.coros.com instead of open.coros.com
- **Mock Data**: May contain synthetic or test data
