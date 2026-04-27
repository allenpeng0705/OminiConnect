# Fitbit Tools

Provider: `fitbit` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Fitbit API. They allow AI agents to access user activity data, sleep records, body measurements, and fitness goals. Fitbit is a popular fitness tracking platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Fitbit
- Token stored in Nango, accessed via `connection_ref`
- OAuth2 with authorization header for token transmission
- User ID in API paths uses `-` for current user

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fitbit_get_user` | Get current user | GET | /1/user/-/profile.json |
| `fitbit_list_activities` | List activities | GET | /1/user/-/activities.json |
| `fitbit_get_activity` | Get activity details | GET | /1/user/-/activities/{id}.json |
| `fitbit_list_sleep` | List sleep records | GET | /1/user/-/sleep.json |
| `fitbit_get_sleep` | Get sleep details | GET | /1/user/-/sleep/date/{date}.json |
| `fitbit_list_body` | List body measurements | GET | /1/user/-/body.json |
| `fitbit_get_body` | Get body details | GET | /1/user/-/body/date/{date}.json |
| `fitbit_list_heart` | List heart rate data | GET | /1/user/-/heart.json |
| `fitbit_list_food` | List food logs | GET | /1/user/-/foods/log.json |
| `fitbit_get_goals` | Get user goals | GET | /1/user/-/goals.json |

---

## Tool Details

### fitbit_get_user

**What it does**: Gets current authenticated user profile.

**When to use**: Verify authentication, get user info.

**Arguments**: None

**Example LLM prompt**: "Get my Fitbit profile"

---

### fitbit_list_activities

**What it does**: Lists activities for the user.

**When to use**: View activity history.

**Arguments**:
- `date` (optional): Specific date (YYYY-MM-DD)
- `period` (optional): Period (1d, 7d, 30d)

**Example LLM prompt**: "List my activities for the last week"

---

### fitbit_get_activity

**What it does**: Gets details of a specific activity.

**When to use**: View activity details.

**Arguments**:
- `id` (required): Activity ID
- `date` (optional): Date

**Example LLM prompt**: "Get activity details for today"

---

### fitbit_list_sleep

**What it does**: Lists sleep records.

**When to use**: View sleep history.

**Arguments**:
- `date` (optional): Specific date
- `period` (optional): Period (1d, 7d, 30d)

**Example LLM prompt**: "List my sleep records for the last week"

---

### fitbit_get_sleep

**What it does**: Gets sleep details for a specific date.

**When to use**: View detailed sleep data.

**Arguments**:
- `date` (required): Date (YYYY-MM-DD)

**Example LLM prompt**: "Get my sleep data for last night"

---

### fitbit_list_body

**What it does**: Lists body measurements.

**When to use**: View weight/body history.

**Arguments**:
- `date` (optional): Specific date
- `period` (optional): Period (1d, 7d, 30d)

**Example LLM prompt**: "List my body measurements for the last month"

---

### fitbit_get_body

**What it does**: Gets body measurements for a specific date.

**When to use**: View specific day's body data.

**Arguments**:
- `date` (required): Date (YYYY-MM-DD)

**Example LLM prompt**: "Get my body measurements for today"

---

### fitbit_list_heart

**What it does**: Lists heart rate data.

**When to use**: View heart rate trends.

**Arguments**:
- `date` (optional): Specific date
- `period` (optional): Period (1d, 7d, 30d)

**Example LLM prompt**: "List my heart rate data for the last week"

---

### fitbit_list_food

**What it does**: Lists food logs.

**When to use**: View nutrition history.

**Arguments**:
- `date` (optional): Date

**Example LLM prompt**: "List my food logs for today"

---

### fitbit_get_goals

**What it does**: Gets user goals.

**When to use**: View fitness goals.

**Arguments**:
- `period` (optional): Period (daily, weekly)

**Example LLM prompt**: "Get my current fitness goals"

---

## Fitbit API Notes

- **User ID**: Use `-` for current authenticated user
- **Activities**: Steps, distance, calories, active minutes
- **Sleep**: Sleep stages, duration, efficiency
- **Body**: Weight, BMI, fat percentage
- **Heart**: Resting heart rate, heart rate zones
- **Food**: Nutrition logging
- **Goals**: Daily and weekly targets
