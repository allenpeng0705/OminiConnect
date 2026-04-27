# Nimble Tools

Provider: `nimble` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Nimble CRM API. They allow AI agents to manage contacts, tasks, and calendar events; and view analytics. Nimble is a social CRM that combines contact management with social media insights.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Nimble
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nimble_list_contacts` | List contacts | GET | /contacts |
| `nimble_get_contact` | Get contact details | GET | /contacts/{contact_id} |
| `nimble_create_contact` | Create a contact | POST | /contacts |
| `nimble_update_contact` | Update a contact | PUT | /contacts/{contact_id} |
| `nimble_list_tasks` | List tasks | GET | /tasks |
| `nimble_get_task` | Get task details | GET | /tasks/{task_id} |
| `nimble_create_task` | Create a task | POST | /tasks |
| `nimble_list_calendar` | List calendar events | GET | /calendar |
| `nimble_get_calendar_event` | Get calendar event details | GET | /calendar/{event_id} |
| `nimble_get_analytics` | Get analytics data | GET | /analytics |

---

## Tool Details

### nimble_list_contacts

**What it does**: Lists all contacts in Nimble with optional filters for search, tags, or company.

**When to use**: Find contacts, browse by tag, filter by company.

**Arguments**:
- `search` (optional): Search by name or email
- `tag` (optional): Filter by tag name
- `company_id` (optional): Filter by company ID
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all contacts with tag 'vip'"

---

### nimble_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Read full contact details including social profiles, tags, and company info.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### nimble_create_contact

**What it does**: Creates a new contact in Nimble.

**When to use**: Add new leads, customers, or business contacts.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Company ID
- `tags` (optional): Array of tag names

**Example LLM prompt**: "Create a contact for John Smith with email john@example.com"

---

### nimble_update_contact

**What it does**: Updates an existing contact's information.

**When to use**: Update contact details, change company association, modify tags.

**Arguments**:
- `contact_id` (required): Contact ID
- `first_name` (optional): New first name
- `last_name` (optional): New last name
- `email` (optional): New email
- `phone` (optional): New phone
- `company_id` (optional): New company ID
- `tags` (optional): New array of tag names

**Example LLM prompt**: "Update contact 12345 to add tag 'prospect'"

---

### nimble_list_tasks

**What it does**: Lists all tasks in Nimble with optional filters for status, assignee, and due date.

**When to use**: Find pending tasks, see tasks by due date, filter by assignee.

**Arguments**:
- `status` (optional): `pending` or `completed`
- `assignee_id` (optional): Filter by user ID
- `due_before` (optional): Due before date
- `due_after` (optional): Due after date
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all pending tasks due today"

---

### nimble_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: Read full task details including description and related contacts.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Get task details for 67890"

---

### nimble_create_task

**What it does**: Creates a new task in Nimble.

**When to use**: Schedule follow-ups, create reminders for contact activities.

**Arguments**:
- `name` (required): Task title
- `description` (optional): Task description
- `assignee_id` (optional): Assignee user ID
- `due_date` (optional): Due date (ISO 8601)
- `status` (optional): Initial status (`pending`, `completed`)

**Example LLM prompt**: "Create a task called 'Follow up with Acme' due tomorrow"

---

### nimble_list_calendar

**What it does**: Lists calendar events filtered by date range, user, or event type.

**When to use**: See upcoming meetings, calls, or events.

**Arguments**:
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)
- `user_id` (optional): Filter by user ID
- `type` (optional): `call`, `meeting`, `event`
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all meetings scheduled for next week"

---

### nimble_get_calendar_event

**What it does**: Gets detailed information about a specific calendar event.

**When to use**: Get full event details including attendees and location.

**Arguments**:
- `event_id` (required): Event ID

**Example LLM prompt**: "Get details for calendar event 11223"

---

### nimble_get_analytics

**What it does**: Gets analytics data including activity metrics, contact stats, and task completion rates.

**When to use**: Review CRM performance, track team productivity.

**Arguments**:
- `start_date` (optional): Start date (ISO 8601)
- `end_date` (optional): End date (ISO 8601)
- `metrics` (optional): Array of metric names to include

**Example LLM prompt**: "Get analytics for this month"

---

## Nimble API Notes

- **Social CRM**: Nimble automatically enriches contacts with social media profiles
- **Tags**: Contacts can have multiple tags for categorization
- **ISO 8601 dates**: All dates should be in ISO 8601 format
- **Pagination**: Default page_size is 20, max is 100
- **Company associations**: Contacts can be linked to companies for hierarchical organization
