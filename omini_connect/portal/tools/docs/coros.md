# Coros Tools

Provider: `coros` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Coros API. They allow AI agents to access fitness activities, workouts, device information, and athlete profiles. Coros is a sports wearables company specializing in GPS sport watches for endurance athletes.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Coros
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `athlete:read`, `athlete:write`, `workout:read`, `device:read`, `team:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `coros_list_activities` | List user activities | GET | /v1/activities |
| `coros_get_activity` | Get a specific activity | GET | /v1/activities/{activity_id} |
| `coros_list_workouts` | List user workouts | GET | /v1/workouts |
| `coros_get_workout` | Get a specific workout | GET | /v1/workouts/{workout_id} |
| `coros_list_devices` | List paired devices | GET | /v1/devices |
| `coros_get_device` | Get device details | GET | /v1/devices/{device_id} |
| `coros_list_athlete_profile` | Get athlete profile | GET | /v1/athlete |
| `coros_update_athlete_profile` | Update athlete profile | PUT | /v1/athlete |
| `coros_list_teams` | List teams | GET | /v1/teams |
| `coros_get_team` | Get team details | GET | /v1/teams/{team_id} |

---

## Tool Details

### coros_list_activities

**What it does**: Lists all activities recorded with Coros devices.

**When to use**: Browse workout history, find activities by date, analyze training patterns.

**Arguments**:
- `start_date` (optional): ISO 8601 date - filter activities from this date
- `end_date` (optional): ISO 8601 date - filter activities until this date
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all my activities from the last month"

---

### coros_get_activity

**What it does**: Gets detailed activity information including GPS data, heart rate zones, and performance metrics.

**When to use**: Analyze specific workouts, get detailed metrics for a training session.

**Arguments**:
- `activity_id` (required): Activity ID

**Example LLM prompt**: "Get details for activity abc123"

---

### coros_list_workouts

**What it does**: Lists all planned and completed structured workouts.

**When to use**: View training plan, find upcoming workouts, see completed training sessions.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all my workouts"

---

### coros_get_workout

**What it does**: Gets detailed workout information including intervals, targets, and workout type.

**When to use**: View workout structure before training, understand interval targets.

**Arguments**:
- `workout_id` (required): Workout ID

**Example LLM prompt**: "Get details for workout xyz789"

---

### coros_list_devices

**What it does**: Lists all Coros devices paired to the account.

**When to use**: Check which devices are connected, verify device sync status.

**Arguments**: None

**Example LLM prompt**: "List all my Coros devices"

---

### coros_get_device

**What it does**: Gets device details including firmware version and settings.

**When to use**: Check device status, verify firmware is up to date.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get details for device d123"

---

### coros_list_athlete_profile

**What it does**: Gets the athlete's profile including physical metrics and training zones.

**When to use**: View current athlete settings, check training zone configuration.

**Arguments**: None

**Example LLM prompt**: "Get my athlete profile"

---

### coros_update_athlete_profile

**What it does**: Updates the athlete's profile including height, weight, and heart rate zones.

**When to use**: Update physical metrics after weight changes, adjust heart rate zones after testing.

**Arguments**:
- `height` (optional): Height in cm
- `weight` (optional): Weight in kg
- `max_heart_rate` (optional): Maximum heart rate
- `threshold_heart_rate` (optional): Threshold heart rate
- `max_power` (optional): Maximum power (FTP)

**Example LLM prompt**: "Update my athlete profile with weight 75kg and max heart rate 185"

---

### coros_list_teams

**What it does**: Lists all teams the athlete belongs to.

**When to use**: View team memberships, find training groups.

**Arguments**: None

**Example LLM prompt**: "List all teams I'm a member of"

---

### coros_get_team

**What it does**: Gets team details including member list and team statistics.

**When to use**: View team activities, check team leaderboard.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team t456"

---

## Coros API Notes

- **Activities**: Recorded workouts with GPS, heart rate, power data - runs, rides, swims, etc.
- **Workouts**: Structured training plans with intervals and targets
- **Devices**: Coros watches and sensors paired to the account
- **Athlete Profile**: Personal metrics used for training calculations
- **Teams**: Social feature for training groups and clubs
- **Date Filtering**: Use ISO 8601 dates (YYYY-MM-DD) for filtering activities
