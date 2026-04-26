# OpsGenie Tools

Provider: `opsgenie` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the OpsGenie REST API. They allow AI agents to manage alerts, users, on-call schedules, and escalation policies. OpsGenie is an incident management and on-call scheduling platform that integrates with monitoring tools.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with OpsGenie
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `alerts:read`, `alerts:write`, `users:read`, `schedules:read`, `policies:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `opsgenie_list_alerts` | List alerts | GET | /v2/alerts |
| `opsgenie_get_alert` | Get alert details | GET | /v2/alerts/{id} |
| `opsgenie_create_alert` | Create an alert | POST | /v2/alerts |
| `opsgenie_close_alert` | Close an alert | POST | /v2/alerts/{id}/close |
| `opsgenie_list_users` | List users | GET | /v2/users |
| `opsgenie_get_user` | Get user details | GET | /v2/users/{id} |
| `opsgenie_list_schedules` | List on-call schedules | GET | /v2/schedules |
| `opsgenie_get_schedule` | Get schedule details | GET | /v2/schedules/{id} |
| `opsgenie_list_policies` | List escalation policies | GET | /v2/policies |
| `opsgenie_get_policy` | Get policy details | GET | /v2/policies/{id} |

---

## Tool Details

### opsgenie_list_alerts

**What it does**: Lists alerts in OpsGenie with filtering options for status, priority, and search query.

**When to use**: Monitor active alerts, track alert trends, find alerts by priority level or tags.

**Arguments**:
- `query` (optional): Search query to filter alerts
- `status` (optional): Filter by status (`open`, `acknowledged`, `closed`)
- `priority` (optional): Filter by priority (`P1`, `P2`, `P3`, `P4`, `P5`)
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all open P1 and P2 alerts in the production namespace"

---

### opsgenie_get_alert

**What it does**: Gets detailed information about a specific alert including title, description, priority, status, and timeline of actions taken.

**When to use**: Understand alert context, check alert status, review response actions taken.

**Arguments**:
- `alert_id` (required): Alert ID

**Example LLM prompt**: "Get details for alert A123456 and show me the timeline of actions"

---

### opsgenie_create_alert

**What it does**: Creates a new alert in OpsGenie. Alerts notify designated responders based on escalation rules.

**When to use**: Report issues, create alerts from automated monitoring, escalate incidents that need attention.

**Arguments**:
- `message` (required): Alert message/title
- `description` (optional): Detailed description of the alert
- `priority` (optional): Priority (`P1` highest to `P5` lowest) - default `P3`
- `tags` (optional): Tags to categorize the alert
- `entity` (optional): Entity/source that caused the alert

**Example LLM prompt**: "Create a P1 alert with message 'Database connection failures in us-east-1' and tag it with 'database' and 'production'"

---

### opsgenie_close_alert

**What it does**: Closes an existing alert. Closed alerts are resolved and no longer require attention.

**When to use**: Resolve incidents that have been fixed, close false positives, mark alerts as handled.

**Arguments**:
- `alert_id` (required): Alert ID
- `note` (optional): Note explaining why the alert is being closed

**Example LLM prompt**: "Close alert A123456 with note 'Database restored after failover completion'"

---

### opsgenie_list_users

**What it does**: Lists all users in OpsGenie who can be assigned to alerts and are part of escalation policies.

**When to use**: Find team members, get contact information, see who is available for on-call rotation.

**Arguments**:
- `query` (optional): Search users by name or email
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all users in OpsGenie who have 'on-call' in their role"

---

### opsgenie_get_user

**What it does**: Gets detailed information about a specific user including contact information, role, and current on-call responsibilities.

**When to use**: Check who is on-call, get user contact details, verify user escalation settings.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user john.smith and check if they are currently on-call"

---

### opsgenie_list_schedules

**What it does**: Lists all on-call schedules in OpsGenie. Schedules define rotation of on-call responders and are used for alert routing.

**When to use**: Find schedules, see on-call rotation patterns, check available schedules for a team.

**Arguments**:
- `query` (optional): Search schedules by name
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all on-call schedules for the platform engineering team"

---

### opsgenie_get_schedule

**What it does**: Gets detailed information about a specific schedule including participants, rotation type, and upcoming on-call shifts.

**When to use**: Check schedule details, understand rotation timing, see who is currently and next on-call.

**Arguments**:
- `schedule_id` (required): Schedule ID
- `timeline` (optional): Timeline parameters for upcoming shifts

**Example LLM prompt**: "Get the primary on-call schedule to see who is on-call this weekend"

---

### opsgenie_list_policies

**What it does**: Lists all escalation policies in OpsGenie. Policies define the chain of notification steps when alerts are created or escalated.

**When to use**: See available policies, understand escalation routing rules, find policies for a team.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all escalation policies that notify the DBA team"

---

### opsgenie_get_policy

**What it does**: Gets detailed information about a specific escalation policy including the rules, recipients, and notification steps for each escalation level.

**When to use**: Understand who gets notified and when, review escalation chain for a service, check notification rules.

**Arguments**:
- `policy_id` (required): Policy ID

**Example LLM prompt**: "Get details for escalation policy EP12345 to understand the notification sequence"

---

## OpsGenie API Notes

- **Priority levels**: P1 is highest (critical), P5 is lowest. P1 and P2 typically trigger immediate notifications.
- **Alert status**: Alerts move from `open` -> `acknowledged` -> `closed`
- **Close behavior**: Closing an alert marks it as resolved; it remains visible for historical tracking
- **Escalation policies**: Define who gets notified, in what order, and after how long
- **Schedules**: Rotation schedules determine who gets called based on time-based rules
- **Rate Limits**: OpsGenie API has rate limits; use pagination and limit results when possible
- **Integration**: OpsGenie integrates with many monitoring tools, PagerDuty, and other incident management systems