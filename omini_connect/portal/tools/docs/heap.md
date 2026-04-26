# Heap Tools

Provider: `heap` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Heap REST API. They allow AI agents to manage events, track user actions, analyze funnels and sessions, list reports, and manage properties. Heap is an analytics platform that automatically captures user interactions on web and mobile apps.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Heap
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `heap_full_access` - Full access to Heap data

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `heap_list_events` | List events in your application | GET | /api/v1/events |
| `heap_get_event` | Get event details and properties | GET | /api/v1/events/{event_id} |
| `heap_track_event` | Track an event | POST | /api/v1/track |
| `heap_list_users` | List users matching criteria | GET | /api/v1/users |
| `heap_get_user` | Get user profile details | GET | /api/v1/users/{user_id} |
| `heap_get_funnel` | Get funnel analysis data | GET | /api/v1/funnels |
| `heap_get_session` | Get session data | GET | /api/v1/sessions/{session_id} |
| `heap_list_reports` | List saved reports | GET | /api/v1/reports |
| `heap_get_report` | Get report details | GET | /api/v1/reports/{report_id} |
| `heap_list_properties` | List custom properties | GET | /api/v1/properties |

---

## Tool Details

### heap_list_events

**What it does**: Lists all events in your Heap application. Returns event names, types, and metadata.

**When to use**: Discover available events, understand what is being tracked.

**Arguments**:
- `app_id` (optional): Heap application ID
- `limit` (optional): Max events to return (default 100)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all events in my Heap app"

---

### heap_get_event

**What it does**: Gets details about a specific event including property analysis and occurrence counts. Use this to understand event structure and verify tracking.

**When to use**: Understand event schema, check event properties.

**Arguments**:
- `app_id` (optional): Heap application ID
- `event_id` (required): Event ID or name

**Example LLM prompt**: "Get details for event 'page_view'"

---

### heap_track_event

**What it does**: Tracks an event. Events are actions users take in your application. Heap automatically captures many events; use this for custom events.

**When to use**: Record custom events beyond automatic capture.

**Arguments**:
- `app_id` (required): Heap application ID
- `user_id` (optional): User identifier
- `event` (required): Event name
- `properties` (optional): Custom event properties
- `timestamp` (optional): ISO 8601 timestamp

**Example LLM prompt**: "Track a 'Purchase' event for user 123 with properties amount: 99.99"

---

### heap_list_users

**What it does**: Lists users matching specific criteria or properties. Use for audience segmentation and user research.

**When to use**: Find user segments, view user lists.

**Arguments**:
- `app_id` (optional): Heap application ID
- `limit` (optional): Max users to return (default 100)
- `page` (optional): Page number (default 1)
- `filter` (optional): Filter expression for users

**Example LLM prompt**: "List users who signed up in the last 30 days"

---

### heap_get_user

**What it does**: Gets a user profile with all associated properties and event history. Use for user research and debugging user-specific issues.

**When to use**: Look up individual user details, view user journeys.

**Arguments**:
- `app_id` (optional): Heap application ID
- `user_id` (required): User ID

**Example LLM prompt**: "Get user profile for user123"

---

### heap_get_funnel

**What it does**: Gets funnel analysis data. Funnels show conversion rates between sequential steps. Define steps by event names and analyze drop-off.

**When to use**: Analyze user drop-off, measure conversion optimization.

**Arguments**:
- `app_id` (optional): Heap application ID
- `steps` (required): Array of funnel step objects with event names
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Analyze funnel from 'page_view' to 'signup' to 'purchase'"

---

### heap_get_session

**What it does**: Gets session data for a user or list of sessions. Sessions group events by user activity within a time window.

**When to use**: Analyze user session behavior, view session details.

**Arguments**:
- `app_id` (optional): Heap application ID
- `session_id` (required): Session ID

**Example LLM prompt**: "Get session details for session sess_123"

---

### heap_list_reports

**What it does**: Lists saved reports in your Heap application. Reports contain saved analysis configurations for quick access.

**When to use**: Discover saved analyses, find report IDs.

**Arguments**:
- `app_id` (optional): Heap application ID
- `limit` (optional): Max reports to return (default 50)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all saved reports"

---

### heap_get_report

**What it does**: Gets details of a specific report including configuration and results. Use this to retrieve saved analysis data.

**When to use**: View saved report results, export analysis data.

**Arguments**:
- `app_id` (optional): Heap application ID
- `report_id` (required): Report ID

**Example LLM prompt**: "Get report details for report_123"

---

### heap_list_properties

**What it does**: Lists all custom properties defined in your Heap application. Properties are attributes that can be used for segmentation and filtering.

**When to use**: Discover available properties, understand user and event attributes.

**Arguments**:
- `app_id` (optional): Heap application ID
- `type` (optional): Property type: user or event
- `limit` (optional): Max properties to return (default 100)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all user properties"

---

## Heap API Notes

- **Events**: Actions users take in your application
- **Sessions**: Group of events from a single user activity period
- **User properties**: Traits attached to each user profile
- **Event properties**: Attributes attached to specific events
- **Reports**: Saved analysis configurations
- **Automatic capture**: Heap automatically captures many events without explicit tracking calls
- **Date format**: YYYY-MM-DD for date parameters
