# Whoop Tools

Provider: `whoop` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Whoop is a fitness tracking platform that monitors recovery and strain. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Whoop
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `whoop_get_user` | Get current user profile | GET | /v1/user |
| `whoop_get_activities` | Get workout activities | GET | /v1/activities |
| `whoop_get_sleep` | Get sleep data | GET | /v1/sleep |
| `whoop_get_recovery` | Get recovery metrics | GET | /v1/recovery |
| `whoop_get_strain` | Get daily strain score | GET | /v1/strain |
| `whoop_get_body_measurements` | Get body measurements | GET | /v1/measurement |
| `whoop_get_cycle` | Get daily cycle data | GET | /v1/cycle |
| `whoop_get_workouts` | Get workout data | GET | /v1/workout |
| `whoop_get_goals` | Get user goals | GET | /v1/goals |
| `whoop_get_all_metrics` | Get all metrics for a date range | GET | /v1/metrics |

---

## Tool Details

### whoop_get_user

**What it does**: Get current user profile

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_activities

**What it does**: Get workout activities

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_sleep

**What it does**: Get sleep data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_recovery

**What it does**: Get recovery metrics

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_strain

**What it does**: Get daily strain score

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_body_measurements

**What it does**: Get body measurements

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_cycle

**What it does**: Get daily cycle data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_workouts

**What it does**: Get workout data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_goals

**What it does**: Get user goals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### whoop_get_all_metrics

**What it does**: Get all metrics for a date range

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.prod.whoop.com/`
- Docs: https://nango.dev/docs/integrations/all/whoop
