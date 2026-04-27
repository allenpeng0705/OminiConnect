# Strava (Mobile) Tools

Provider: `strava` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the Strava (Mobile) API. They allow AI agents to interact with Strava (Mobile) functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_athlete` | Get current athlete profile | GET | /athlete |
| `list_activities` | List athlete activities | GET | /activities |
| `get_activity` | Get activity details | GET | /activities/{id} |
| `list_clubs` | List athlete clubs | GET | /clubs |
| `get_club` | Get club details | GET | /clubs/{id} |
| `list_gear` | List athlete gear | GET | /gear |
| `get_segment_efforts` | Get segment efforts | GET | /segments/{id}/all_efforts |
| `list_zones` | List athlete zones | GET | /zones |
| `get_stats` | Get athlete statistics | GET | /athletes/{id}/stats |
| `upload_activity` | Upload activity data | POST | /uploads |

---

## Tool Details

### get_athlete

**What it does**: Get current athlete profile

**When to use**: Use this tool when you need to get current athlete profile.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get athlete to..."

---

### list_activities

**What it does**: List athlete activities

**When to use**: Use this tool when you need to list athlete activities.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list activities to..."

---

### get_activity

**What it does**: Get activity details

**When to use**: Use this tool when you need to get activity details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get activity to..."

---

### list_clubs

**What it does**: List athlete clubs

**When to use**: Use this tool when you need to list athlete clubs.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list clubs to..."

---

### get_club

**What it does**: Get club details

**When to use**: Use this tool when you need to get club details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get club to..."

---

### list_gear

**What it does**: List athlete gear

**When to use**: Use this tool when you need to list athlete gear.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list gear to..."

---

### get_segment_efforts

**What it does**: Get segment efforts

**When to use**: Use this tool when you need to get segment efforts.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get segment efforts to..."

---

### list_zones

**What it does**: List athlete zones

**When to use**: Use this tool when you need to list athlete zones.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list zones to..."

---

### get_stats

**What it does**: Get athlete statistics

**When to use**: Use this tool when you need to get athlete statistics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get stats to..."

---

### upload_activity

**What it does**: Upload activity data

**When to use**: Use this tool when you need to upload activity data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload activity to..."

---

## Strava (Mobile) API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://www.strava.com
- **API prefix**: /api/v3
- **Rate limits**: Check provider documentation for specific limits
