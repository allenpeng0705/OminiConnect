# Mixpanel Tools

Provider: `mixpanel` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Mixpanel REST API. They allow AI agents to manage projects, track events, analyze funnels, query user profiles, and run custom JQL queries. Mixpanel is a product analytics platform that helps businesses understand user behavior through event tracking.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Mixpanel
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `events:read`, `track:write`, `users:read`, `funnels:read`, `analytics:read`, `cohorts:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mixpanel_list_events` | List events in your project | GET | /api/2.0/events/names |
| `mixpanel_get_event` | Get details about a specific event | GET | /api/2.0/events/properties |
| `mixpanel_track_event` | Track an event | POST | /import |
| `mixpanel_list_users` | List users matching criteria | GET | /api/2.0/engage |
| `mixpanel_get_user` | Get user profile details | GET | /api/2.0/engage/{distinct_id} |
| `mixpanel_get_funnel` | Get funnel analysis data | POST | /api/2.0/funnels |
| `mixpanel_get_retention` | Get retention analysis | GET | /api/2.0/retention |
| `mixpanel_get_cohort` | Get cohort details | GET | /api/2.0/cohorts/{cohort_id} |
| `mixpanel_get_engage_report` | Get engagement report | POST | /api/2.0/engage |
| `mixpanel_get_jql_query` | Execute JQL query | POST | /api/2.0/jql |

---

## Tool Details

### mixpanel_list_events

**What it does**: Lists all events in your Mixpanel project. Returns event names and their occurrence counts over time.

**When to use**: Discover available events, understand what is being tracked.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `limit` (optional): Max events to return (default 100)

**Example LLM prompt**: "List all events in my Mixpanel project"

---

### mixpanel_get_event

**What it does**: Gets detailed information about a specific event including property distributions. Use this to understand event structure and common property values.

**When to use**: Understand event schema, see what properties are available for analysis.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `event` (required): Name of the event to inspect
- `name` (optional): Property name to get details for

**Example LLM prompt**: "Get details for the 'Purchase' event"

---

### mixpanel_track_event

**What it does**: Tracks an event. Events are actions users take in your application. Include properties to add context and segment users.

**When to use**: Record user actions like purchases, page views, sign-ups.

**Arguments**:
- `event` (required): Event name
- `properties` (required): Event properties including distinct_id, token, time

**Example LLM prompt**: "Track a 'Item Added to Cart' event for user 123 with properties item: 'Widget', price: 29.99"

---

### mixpanel_list_users

**What it does**: Lists users matching specific criteria or profile properties. Use for audience segmentation and user research.

**When to use**: Find user segments, view user lists.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `distinct_id` (optional): User distinct ID to lookup
- `limit` (optional): Max users to return (default 100)
- `where` (optional): JQL filter for user properties

**Example LLM prompt**: "List users who have the 'premium' plan property"

---

### mixpanel_get_user

**What it does**: Gets a user profile with all associated properties and event history. Use this for user research and debugging user-specific issues.

**When to use**: Look up individual user details, view user journeys.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `distinct_id` (required): User's distinct ID

**Example LLM prompt**: "Get user profile for distinct_id user123"

---

### mixpanel_get_funnel

**What it does**: Gets funnel analysis data. Funnels show conversion rates between steps. Define steps by event names and time windows.

**When to use**: Analyze user drop-off, measure conversion optimization.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `from_date` (required): Start date (YYYY-MM-DD)
- `to_date` (required): End date (YYYY-MM-DD)
- `steps` (required): Array of funnel step objects with event name and time window
- `bucketed_by` (optional): Segment by property (e.g., browser, country)

**Example LLM prompt**: "Analyze the signup funnel from Page View to Sign Up to Purchase for the last 30 days"

---

### mixpanel_get_retention

**What it does**: Gets retention analysis. Shows how users return over time after their first action. Use to measure engagement and product stickiness.

**When to use**: Measure user engagement, understand product stickiness.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `from_date` (required): Start date (YYYY-MM-DD)
- `to_date` (required): End date (YYYY-MM-DD)
- `event` (required): Initial event to calculate retention from
- `retention_type` (optional): anything, first, repeat (default: anything)

**Example LLM prompt**: "Show retention for users who performed 'First Login' over the past 30 days"

---

### mixpanel_get_cohort

**What it does**: Gets details about a cohort including member count and properties. Cohorts are groups of users sharing common characteristics.

**When to use**: Understand cohort composition, export cohort data.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `cohort_id` (required): Cohort ID

**Example LLM prompt**: "Get details for cohort 12345"

---

### mixpanel_get_engage_report

**What it does**: Gets an engagement report with metrics like DAU, MAU, and session stats. Use for understanding user engagement trends over time.

**When to use**: Get engagement metrics, build custom reports.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `from_date` (required): Start date (YYYY-MM-DD)
- `to_date` (required): End date (YYYY-MM-DD)
- `analysis` (optional): Analysis type and parameters

**Example LLM prompt**: "Get engagement report for the last 7 days"

---

### mixpanel_get_jql_query

**What it does**: Executes a JQL (JavaScript Query Language) query to analyze event and user data. Use for custom analysis not covered by standard APIs.

**When to use**: Run custom analysis, complex queries across events and users.

**Arguments**:
- `project_id` (optional): Mixpanel project ID
- `script` (required): JQL script to execute
- `params` (optional): Parameters to pass to the JQL script

**Example LLM prompt**: "Run a JQL query to find all users who performed more than 10 events"

---

## Mixpanel API Notes

- **Distinct ID**: The unique identifier for each user (similar to Segment's userId)
- **Event properties**: Custom key-value pairs attached to each event
- **User properties**: Traits attached to each user profile
- **JQL**: Mixpanel's JSON query language for filtering users and events
- **Funnels**: Sequence of events to measure conversion
- **Cohorts**: Groups of users sharing common characteristics
- **Retention**: Shows how users return over time after initial action
- **Date format**: YYYY-MM-DD for all date parameters
