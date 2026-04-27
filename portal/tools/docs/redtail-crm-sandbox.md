# Redtail-Crm-Sandbox Tools

Provider: `redtail-crm-sandbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Redtail CRM Sandbox is a testing environment for Redtail's financial advisor CRM platform. These tools allow AI agents to test contact management, activities, tasks, events, and calendar functionality.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Redtail CRM Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contacts:read`, `contacts:write`, `activities:read`, `activities:write`, `tasks:read`, `tasks:write`, `events:read`, `events:write`, `calendar:read`

**Important**: This is the sandbox environment. All operations are test operations and do not affect production data.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `redtail-crm-sandbox_list_contacts` | List all contacts | GET | /contacts |
| `redtail-crm-sandbox_get_contact` | Get contact details | GET | /contacts/{contactId} |
| `redtail-crm-sandbox_create_contact` | Create a contact | POST | /contacts |
| `redtail-crm-sandbox_list_activities` | List all activities | GET | /activities |
| `redtail-crm-sandbox_create_activity` | Create an activity | POST | /activities |
| `redtail-crm-sandbox_list_tasks` | List all tasks | GET | /tasks |
| `redtail-crm-sandbox_create_task` | Create a task | POST | /tasks |
| `redtail-crm-sandbox_list_events` | List calendar events | GET | /events |
| `redtail-crm-sandbox_create_event` | Create an event | POST | /events |
| `redtail-crm-sandbox_get_calendar` | Get calendar view | GET | /calendar |

---

## Tool Details

### redtail-crm-sandbox_list_contacts

**What it does**: Returns a list of all contacts.

**When to use**: Browse contacts in the sandbox.

**Arguments**:
- `limit` (optional): Number of contacts (default 50)
- `search` (optional): Search term

**Example LLM prompt**: "List all test contacts"

---

### redtail-crm-sandbox_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: Get contact information.

**Arguments**:
- `contactId` (required): The contact ID

**Example LLM prompt**: "Get details for contact ct_abc123"

---

### redtail-crm-sandbox_create_contact

**What it does**: Creates a new contact in sandbox.

**When to use**: Test contact creation.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a test contact named 'John Doe'"

---

### redtail-crm-sandbox_list_activities

**What it does**: Returns a list of all activities.

**When to use**: View activity history.

**Arguments**:
- `limit` (optional): Number of activities (default 50)
- `contactId` (optional): Filter by contact

**Example LLM prompt**: "List all test activities"

---

### redtail-crm-sandbox_create_activity

**What it does**: Creates a new activity log entry.

**When to use**: Log a test activity.

**Arguments**:
- `contactId` (required): Contact ID
- `type` (required): Type (call, email, meeting, note)
- `subject` (required): Subject
- `description` (optional): Description
- `date` (optional): Activity date

**Example LLM prompt**: "Log a test call with contact ct_123"

---

### redtail-crm-sandbox_list_tasks

**What it does**: Returns a list of all tasks.

**When to use**: View task list.

**Arguments**:
- `limit` (optional): Number of tasks (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all test tasks"

---

### redtail-crm-sandbox_create_task

**What it does**: Creates a new task in sandbox.

**When to use**: Create a test task.

**Arguments**:
- `title` (required): Task title
- `description` (optional): Description
- `dueDate` (optional): Due date
- `priority` (optional): Priority (low, medium, high)

**Example LLM prompt**: "Create a test task 'Follow up with client'"

---

### redtail-crm-sandbox_list_events

**What it does**: Returns a list of calendar events.

**When to use**: View event schedule.

**Arguments**:
- `startDate` (optional): Start date filter
- `endDate` (optional): End date filter

**Example LLM prompt**: "List all events this month"

---

### redtail-crm-sandbox_create_event

**What it does**: Creates a new calendar event.

**When to use**: Schedule a test event.

**Arguments**:
- `title` (required): Event title
- `startTime` (required): Start time (ISO 8601)
- `endTime` (optional): End time
- `description` (optional): Description

**Example LLM prompt**: "Create a test meeting for tomorrow"

---

### redtail-crm-sandbox_get_calendar

**What it does**: Gets a calendar view of events and tasks.

**When to use**: View combined calendar.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get my test calendar for this week"

---

## Redtail CRM Sandbox Notes

- This is a TEST environment - data does not affect production
- Contact IDs are sandbox-specific
- Activities track interactions with contacts
- Calendar shows events and tasks combined
