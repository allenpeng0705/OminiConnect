# Amplitude Tools

Provider: `amplitude` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Amplitude REST API. They allow AI agents to manage events, track user actions, analyze funnels, study retention, and manage cohorts. Amplitude is an analytics platform that helps product teams understand user behavior.

## Authentication

**Nango OAuth/API Key**:
- User authenticates via Nango Connect with Amplitude
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `amplitude_api_key` - Read/write access to Amplitude data

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `amplitude_list_events` | List events in your project | POST | /api/events/list |
| `amplitude_get_event` | Get event details and properties | GET | /api/events/details |
| `amplitude_track_event` | Track an event | POST | /api/track |
| `amplitude_list_users` | List users matching criteria | POST | /api/users/list |
| `amplitude_get_user` | Get user profile details | GET | /api/users/details |
| `amplitude_get_funnel` | Get funnel analysis data | POST | /api/funnels |
| `amplitude_get_retention` | Get retention analysis | GET | /api/retention |
| `amplitude_get_session` | Get session data | GET | /api/sessions |
| `amplitude_list_cohorts` | List cohorts in your project | GET | /api/cohorts |
| `amplitude_get_cohort` | Get cohort details | GET | /api/cohorts/{cohort_id} |

---

## Tool Details

### amplitude_list_events

**What it does**: Lists events in your Amplitude project with pagination. Returns event names, estimated volumes, and metadata.

**When to use**: Discover available events, understand what is being tracked.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `start` (optional): Starting index for pagination (default 0)
- `limit` (optional): Max events to return (default 100)

**Example LLM prompt**: "List all events in my Amplitude project"

---

### amplitude_get_event

**What it does**: Gets details about a specific event including its properties and daily counts. Use this to understand event structure and popularity.

**When to use**: Understand event schema, see what properties are available.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `event_id` (required): Event ID

**Example LLM prompt**: "Get details for event evt_123"

---

### amplitude_track_event

**What it does**: Tracks an event. Events are actions users take in your application. Include user ID, device ID, and properties for rich analytics.

**When to use**: Record user actions like purchases, page views, sign-ups.

**Arguments**:
- `event` (required): Event name
- `user_id` (optional): User identifier
- `device_id` (optional): Device identifier
- `time` (optional): ISO 8601 timestamp
- `event_properties` (optional): Properties associated with the event
- `user_properties` (optional): Properties to set on the user

**Example LLM prompt**: "Track a 'Purchase' event for user 123 with properties amount: 99.99, product: 'Widget'"

---

### amplitude_list_users

**What it does**: Lists users matching specific criteria or retrieves user profiles. Use for audience segmentation and user research.

**When to use**: Find user segments, view user lists.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `start` (optional): Starting index (default 0)
- `limit` (optional): Max users to return (default 100)
- `user_id` (optional): Filter by specific user ID

**Example LLM prompt**: "List users in my Amplitude project"

---

### amplitude_get_user

**What it does**: Gets a user profile with all associated properties and event streams. Use for user research and debugging user-specific issues.

**When to use**: Look up individual user details, view user journeys.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `user_id` (required): User ID

**Example LLM prompt**: "Get user profile for user123"

---

### amplitude_get_funnel

**What it does**: Gets funnel analysis data. Funnels show conversion rates between sequential steps. Define steps by event names and calculate drop-off.

**When to use**: Analyze user drop-off, measure conversion optimization.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `start` (required): Start date (YYYY-MM-DD)
- `end` (required): End date (YYYY-MM-DD)
- `steps` (required): Array of funnel step objects with event names

**Example LLM prompt**: "Analyze funnel from 'PageView' to 'SignUp' to 'Purchase' for April 2026"

---

### amplitude_get_retention

**What it does**: Gets retention analysis. Shows how users return over time after performing an initial action. Use to measure engagement and product stickiness.

**When to use**: Measure user engagement, understand product stickiness.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `start` (required): Start date (YYYY-MM-DD)
- `end` (required): End date (YYYY-MM-DD)
- `initial_event` (required): Initial event to calculate retention from
- `returning_event` (optional): Event to check for returning users

**Example LLM prompt**: "Show retention for users who performed 'First Login' over the past 30 days"

---

### amplitude_get_session

**What it does**: Gets session data including session length, events per session, and session counts. Use to understand user engagement patterns.

**When to use**: Analyze user session behavior, understand engagement patterns.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `start` (required): Start date (YYYY-MM-DD)
- `end` (required): End date (YYYY-MM-DD)
- `user_id` (optional): Filter by specific user ID

**Example LLM prompt**: "Get session data for the last 7 days"

---

### amplitude_list_cohorts

**What it does**: Lists all cohorts in your Amplitude project. Cohorts are groups of users sharing common characteristics or behaviors.

**When to use**: Discover available cohorts, find cohort IDs.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `limit` (optional): Max cohorts to return (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all cohorts in my project"

---

### amplitude_get_cohort

**What it does**: Gets details about a specific cohort including size, properties, and creation date. Use this to understand cohort composition and behavior.

**When to use**: Understand cohort details, export cohort member data.

**Arguments**:
- `project_id` (optional): Amplitude project ID
- `cohort_id` (required): Cohort ID

**Example LLM prompt**: "Get details for cohort 12345"

---

## Amplitude API Notes

- **Event**: An action a user takes in your application
- **User ID**: The unique identifier for each user
- **Device ID**: Identifier for user's device
- **Event properties**: Key-value pairs describing the event
- **User properties**: Traits attached to each user profile
- **Sessions**: Group of events from a single user activity period
- **Cohorts**: Groups of users sharing common characteristics
- **Funnels**: Sequence of events measuring conversion
- **Retention**: Shows user return rates over time
- **Date format**: YYYY-MM-DD for all date parameters
