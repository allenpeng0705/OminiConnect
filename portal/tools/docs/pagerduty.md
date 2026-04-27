# PagerDuty Tools

Provider: `pagerduty` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the PagerDuty REST API. They allow AI agents to manage incidents, users, services, and on-call schedules. PagerDuty is the leading incident management platform for on-call alerting and response.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with PagerDuty
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `incidents.read`, `incidents.write`, `users.read`, `services.read`, `schedules.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pagerduty_list_incidents` | List incidents | GET | /incidents |
| `pagerduty_get_incident` | Get incident details | GET | /incidents/{id} |
| `pagerduty_create_incident` | Create an incident | POST | /incidents |
| `pagerduty_manage_incident` | Update/resolve an incident | PUT | /incidents/{id} |
| `pagerduty_list_users` | List users | GET | /users |
| `pagerduty_get_user` | Get user details | GET | /users/{id} |
| `pagerduty_list_services` | List services | GET | /services |
| `pagerduty_get_service` | Get service details | GET | /services/{id} |
| `pagerduty_list_schedules` | List on-call schedules | GET | /schedules |
| `pagerduty_get_schedule` | Get schedule details | GET | /schedules/{id} |

---

## Tool Details

### pagerduty_list_incidents

**What it does**: Lists incidents in PagerDuty with filtering options for status, service, and date range.

**When to use**: Monitor active incidents, find triggered alerts, track incident trends over time.

**Arguments**:
- `status` (optional): Filter by status (`triggered`, `acknowledged`, `resolved`)
- `service_ids` (optional): Filter by service ID(s)
- `date_range` (optional): Date range filter (`today`, `week`, `month`)
- `sort_by` (optional): Sort field (`created_at`, `urgency`)
- `limit` (optional): Max results (default 25, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all triggered incidents from the last week in the payment service"

---

### pagerduty_get_incident

**What it does**: Gets detailed information about a specific incident including status, assigned responders, timeline, and service details.

**When to use**: Understand incident context, check who is assigned, review incident timeline.

**Arguments**:
- `incident_id` (required): Incident ID

**Example LLM prompt**: "Get details for incident P12345"

---

### pagerduty_create_incident

**What it does**: Creates a new incident, triggering alerts to on-call responders based on escalation policies.

**When to use**: Report issues that need immediate attention, create alerts from monitoring systems.

**Arguments**:
- `title` (required): Incident title
- `service_id` (required): Service ID to attach the incident to
- `urgency` (optional): Urgency level (`high`, `low`) - default `high`
- `body` (optional): Incident description/details
- `incident_key` (optional): Deduplication key to prevent duplicate incidents

**Example LLM prompt**: "Create an incident for the payment service with title 'Payment processing failures'"

---

### pagerduty_manage_incident

**What it does**: Updates an incident's status, urgency, or assignments. Use to acknowledge or resolve active incidents.

**When to use**: Acknowledge alerts, change urgency level, reassign responders, mark incidents as resolved.

**Arguments**:
- `incident_id` (required): Incident ID
- `status` (optional): New status (`acknowledged`, `resolved`)
- `urgency` (optional): Change urgency (`high`, `low`)
- `assignee` (optional): Array of assignee user objects

**Example LLM prompt**: "Acknowledge incident P12345 and assign to john@example.com"

---

### pagerduty_list_users

**What it does**: Lists all users in PagerDuty who can be assigned to incidents and are part of escalation policies.

**When to use**: Find team members, get user IDs, see who is available for escalation.

**Arguments**:
- `query` (optional): Search users by name or email
- `limit` (optional): Max results (default 25, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all users on the engineering on-call team"

---

### pagerduty_get_user

**What it does**: Gets detailed information about a specific user including contact methods, time zone, and current on-call status.

**When to use**: Check who is on-call, get user contact details, verify user availability.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user U98765 and check if they are currently on-call"

---

### pagerduty_list_services

**What it does**: Lists all services configured in PagerDuty. Services are applications or components that can create incidents.

**When to use**: See available services, find service IDs, understand your monitoring setup.

**Arguments**:
- `limit` (optional): Max results (default 25, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all services in PagerDuty to find the database monitoring service ID"

---

### pagerduty_get_service

**What it does**: Gets detailed information about a specific service including integration keys, escalation policy, and incident creation settings.

**When to use**: Check service configuration, find related incidents, review escalation routing for a service.

**Arguments**:
- `service_id` (required): Service ID

**Example LLM prompt**: "Get details for service ABC123 and see what escalation policy it uses"

---

### pagerduty_list_schedules

**What it does**: Lists all on-call schedules in PagerDuty. Schedules define who is on-call and when they receive incident notifications.

**When to use**: Find schedules, see on-call rotations, check who is currently on-call.

**Arguments**:
- `query` (optional): Search schedules by name
- `limit` (optional): Max results (default 25, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all on-call schedules for the platform team"

---

### pagerduty_get_schedule

**What it does**: Gets detailed information about a specific schedule including the escalation policy and upcoming on-call shifts.

**When to use**: Understand rotation timing, see upcoming on-call assignments, find who is on-call now.

**Arguments**:
- `schedule_id` (required): Schedule ID
- `since` (optional): Start time for schedule overview (ISO 8601)
- `until` (optional): End time for schedule overview (ISO 8601)

**Example LLM prompt**: "Get the on-call schedule for next week to see who is covering the weekend"

---

## PagerDuty API Notes

- **Incident IDs**: Format is typically `P` followed by numbers (e.g., `P12345`)
- **Status transitions**: Incidents flow from `triggered` -> `acknowledged` -> `resolved`
- **Urgency levels**: `high` urgency triggers immediate notifications; `low` urgency respects quiet hours
- **Services**: Each service has its own escalation policy and integration points
- **Rate Limits**: PagerDuty API has rate limits; use pagination and limit results when possible
- **On-call schedules**: Schedules support complex rotation patterns including handoff times