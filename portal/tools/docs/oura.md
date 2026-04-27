# Oura Tools

Provider: `oura` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Oura Ring API for health and wellness tracking. They allow AI agents to access sleep data, activity metrics, readiness scores, and workout information. **Requires Oura OAuth authentication.**

## Authentication

**OAuth2**:
- User authenticates via Nango Connect with Oura
- Authorization URL: `https://cloud.ouraring.com/oauth/authorize`
- Token URL: `https://api.ouraring.com/oauth/token`
- Base URL: `https://api.ouraring.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `oura_list_sleep` | List sleep data | GET | /v2/usercollection/sleep |
| `oura_get_sleep` | Get sleep details | GET | /v2/usercollection/sleep |
| `oura_list_activity` | List activity data | GET | /v2/usercollection/activity |
| `oura_get_activity` | Get activity details | GET | /v2/usercollection/activity |
| `oura_list_readiness` | List readiness data | GET | /v2/usercollection/readiness |
| `oura_get_readiness` | Get readiness details | GET | /v2/usercollection/readiness |
| `oura_list_workouts` | List workouts | GET | /v2/usercollection/workout |
| `oura_get_workout` | Get workout details | GET | /v2/usercollection/workout |
| `oura_list_tags` | List tags | GET | /v2/usercollection/tag |
| `oura_get_user_info` | Get user info | GET | /v2/userinfo |

---

## Tool Details

### oura_list_sleep

**What it does**: Lists sleep data from Oura Ring.

**When to use**: Analyze sleep patterns, view trends.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my sleep data for the past week"

---

### oura_get_sleep

**What it does**: Gets detailed sleep information for a specific date.

**When to use**: View specific night's sleep analysis.

**Arguments**:
- `date` (required): Date (YYYY-MM-DD)

**Example LLM prompt**: "Get sleep data for January 15th"

---

### oura_list_activity

**What it does**: Lists activity data from Oura Ring.

**When to use**: Track daily activity, calories burned.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my activity data for this week"

---

### oura_get_activity

**What it does**: Gets detailed activity for a specific date.

**When to use**: View specific day's activity metrics.

**Arguments**:
- `date` (required): Date (YYYY-MM-DD)

**Example LLM prompt**: "Get activity data for January 15th"

---

### oura_list_readiness

**What it does**: Lists readiness data from Oura Ring.

**When to use**: Monitor recovery, optimize training.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my readiness scores for the past month"

---

### oura_get_readiness

**What it does**: Gets readiness for a specific date.

**When to use**: Check readiness score, plan activities.

**Arguments**:
- `date` (required): Date (YYYY-MM-DD)

**Example LLM prompt**: "Get readiness score for today"

---

### oura_list_workouts

**What it does**: Lists workout data from Oura Ring.

**When to use**: Track exercise, view workout history.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my workouts for this month"

---

### oura_get_workout

**What it does**: Gets detailed workout information.

**When to use**: View specific workout details.

**Arguments**:
- `workout_id` (required): Workout ID

**Example LLM prompt**: "Get details for workout 12345"

---

### oura_list_tags

**What it does**: Lists tags from Oura Ring.

**When to use**: View notes, annotations, life events.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my tags for this week"

---

### oura_get_user_info

**What it does**: Gets Oura user account information.

**When to use**: View account details, subscription status.

**Arguments**: None

**Example LLM prompt**: "Get my Oura account info"

---

## Oura Notes

- **Health tracking**: Oura Ring tracks sleep, activity, readiness
- **Date format**: ISO 8601 (YYYY-MM-DD)
- **Scopes**: daily (sleep, activity, readiness), workout
- **Readiness**: Composite score based on recovery metrics
- **Tags**: User annotations for life events
