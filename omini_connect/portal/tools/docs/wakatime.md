# Wakatime Tools

Provider: `wakatime` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Wakatime is a time tracking tool for developers. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wakatime
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wakatime_get_current_user` | Get current user info | GET | /api/v1/users/current |
| `wakatime_list_summaries` | Get time tracking summaries | GET | /api/v1/users/current/summaries |
| `wakatime_list_all_time_since_today` | Get all time tracked since today | GET | /api/v1/users/current/all_time_since_today |
| `wakatime_list_leaders` | Get leaderboard data | GET | /api/v1/leaders |
| `wakatime_list_goals` | List all goals | GET | /api/v1/goals |
| `wakatime_get_goal` | Get goal details | GET | /api/v1/goals/{id} |
| `wakatime_list_projects` | List all projects | GET | /api/v1/users/current/projects |
| `wakatime_list_entities` | List coded entities by date range | GET | /api/v1/users/current/entities |
| `wakatime_get_stats` | Get coding statistics | GET | /api/v1/users/current/stats |
| `wakatime_list_heartbeats` | List heartbeat data | GET | /api/v1/users/current/heartbeats |

---

## Tool Details

### wakatime_get_current_user

**What it does**: Get current user info

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_summaries

**What it does**: Get time tracking summaries

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_all_time_since_today

**What it does**: Get all time tracked since today

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_leaders

**What it does**: Get leaderboard data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_goals

**What it does**: List all goals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_get_goal

**What it does**: Get goal details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_projects

**What it does**: List all projects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_entities

**What it does**: List coded entities by date range

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_get_stats

**What it does**: Get coding statistics

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wakatime_list_heartbeats

**What it does**: List heartbeat data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://wakatime.com`
- Docs: https://nango.dev/docs/integrations/all/wakatime
