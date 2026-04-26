# Garmin Tools

Provider: `garmin` | Engine: `nango` | Auth: OAUTH1 via Nango

## Overview

These tools wrap the Garmin Connect API. They allow AI agents to access fitness activities, sleep data, body composition, heart rate, stress, and other health metrics. **Requires Garmin OAuth authentication.**

## Authentication

**Nango OAUTH1**:
- User authenticates via OAuth 1.0 with Garmin Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://apis.garmin.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `garmin_list_activities` | List activities | GET | /activitylist-service/activities |
| `garmin_get_activity` | Get activity details | GET | /activitylist-service/activities/{activityId} |
| `garmin_list_sleep_data` | List sleep data | GET | /sleep-data-service/sleep |
| `garmin_list_body_composition` | List body composition | GET | /body-composition-service/users/{userId}/weights |
| `garmin_list_daily_summaries` | List daily summaries | GET | /daily-summary-service/users/{userId}/dailySummaries |
| `garmin_list_pulse_ox` | List pulse ox data | GET | /pulse-ox-user-summary-service/users/{userId}/dailyPulseOxSummaries |
| `garmin_list_stress` | List stress data | GET | /stress-data-service/users/{userId}/stressDailySummaries |
| `garmin_list_heart_rate` | List heart rate data | GET | /heartrate-service/users/{userId}/dailyHeartSummaries |
| `garmin_list_respiration` | List respiration data | GET | /respiration-service/users/{userId}/dailyRespirationSummaries |
| `garmin_list_spO2` | List SpO2 data | GET | /pulse-ox-user-summary-service/users/{userId}/dailyPulseOxSummaries |

---

## Tool Details

### garmin_list_activities

**What it does**: Lists user's fitness activities from Garmin Connect.

**When to use**: Browse workout history, runs, rides, etc.

**Arguments**:
- `start` (optional): Start index (default 0)
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my recent activities from Garmin"

---

### garmin_get_activity

**What it does**: Gets detailed information about a specific activity.

**When to use**: Get workout details, splits, metrics.

**Arguments**:
- `activityId` (required): Activity ID

**Example LLM prompt**: "Get details for activity 123456789"

---

### garmin_list_sleep_data

**What it does**: Lists user's sleep data from Garmin Connect.

**When to use**: Analyze sleep patterns and quality.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my sleep data for the past week"

---

### garmin_list_body_composition

**What it does**: Lists user's body composition data (weight, BMI, etc).

**When to use**: Track weight and body metrics over time.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my weight trend for the past month"

---

### garmin_list_daily_summaries

**What it does**: Lists daily health summaries for the user.

**When to use**: Get daily steps, calories, floors climbed.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get my daily summaries for this week"

---

### garmin_list_pulse_ox

**What it does**: Lists pulse oximetry data from Garmin Connect.

**When to use**: Monitor blood oxygen levels during sleep.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my pulse ox data"

---

### garmin_list_stress

**What it does**: Lists stress level data from Garmin Connect.

**When to use**: Analyze stress patterns throughout the day.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my stress levels this week"

---

### garmin_list_heart_rate

**What it does**: Lists heart rate data from Garmin Connect.

**When to use**: Monitor resting heart rate and exercise HR.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my heart rate data for today"

---

### garmin_list_respiration

**What it does**: Lists respiration data from Garmin Connect.

**When to use**: Track breathing patterns during sleep.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my respiration data"

---

### garmin_list_spO2

**What it does**: Lists blood oxygen saturation data from Garmin Connect.

**When to use**: Monitor blood oxygen levels overnight.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show my SpO2 levels"

---

## Garmin API Notes

- **OAuth1 authentication**: Uses HMAC-SHA1 signature
- **Date formats**: Use YYYY-MM-DD for all dates
- **Activity types**: Running, cycling, swimming, fitness, etc.
- **Health metrics**: All health data is read-only
