# Wealthbox Tools

Provider: `wealthbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Wealthbox CRM API. They allow AI agents to manage contacts, activities, projects, tasks, and events for financial advisory practices. Wealthbox is designed for financial advisors and wealth management firms to track client relationships and projects.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wealthbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contacts:r`, `contacts:w`, `activities:r`, `activities:w`, `projects:r`, `projects:w`, `tasks:r`, `tasks:w`, `events:r`, `events:w`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wealthbox_list_contacts` | List all contacts | GET | /contacts |
| `wealthbox_get_contact` | Get contact details | GET | /contacts/{id} |
| `wealthbox_create_contact` | Create a new contact | POST | /contacts |
| `wealthbox_list_activities` | List all activities | GET | /activities |
| `wealthbox_get_activity` | Get activity details | GET | /activities/{id} |
| `wealthbox_list_projects` | List all projects | GET | /projects |
| `wealthbox_get_project` | Get project details | GET | /projects/{id} |
| `wealthbox_list_tasks` | List all tasks | GET | /tasks |
| `wealthbox_get_task` | Get task details | GET | /tasks/{id} |
| `wealthbox_list_events` | List all events | GET | /events |

---

## Tool Details

### wealthbox_list_contacts

**What it does**: Retrieves all contacts from Wealthbox with optional filtering by name, email, tag, or company.

**When to use**: Get an overview of all clients and prospects in the CRM.

**Arguments**:
- `search` (optional): Search by name, email, or company
- `tag` (optional): Filter by tag
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all my contacts at tech companies"

---

### wealthbox_get_contact

**What it does**: Gets detailed information about a specific contact including all contact details and related data.

**When to use**: Get full contact details before a meeting or call.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact ID 12345"

---

### wealthbox_create_contact

**What it does**: Creates a new contact in Wealthbox with name, email, phone, company, and tags.

**When to use**: Add a new client or prospect to the CRM.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company` (optional): Company name
- `tags` (optional): Tags to assign

**Example LLM prompt**: "Add a new contact for Sarah Johnson at TechCorp with email sarah@techcorp.com"

---

### wealthbox_list_activities

**What it does**: Retrieves all activities with optional filtering by type, contact, or date range.

**When to use**: Review recent client interactions and touchpoints.

**Arguments**:
- `type` (optional): Filter by activity type (call, email, meeting, task)
- `contact_id` (optional): Filter by contact ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all calls I had this week"

---

### wealthbox_get_activity

**What it does**: Gets detailed information about a specific activity including notes and outcome.

**When to use**: Review details of a past interaction before a follow-up.

**Arguments**:
- `id` (required): Activity ID

**Example LLM prompt**: "Get details for activity ID 67890"

---

### wealthbox_list_projects

**What it does**: Retrieves all projects with optional filtering by status or client.

**When to use**: Overview of all active client projects and engagements.

**Arguments**:
- `status` (optional): Filter by status (active, completed, on-hold)
- `client_id` (optional): Filter by client ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all active projects for my top clients"

---

### wealthbox_get_project

**What it does**: Gets detailed information about a specific project including timeline and deliverables.

**When to use**: Review project status and details before client meetings.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project ID 11111"

---

### wealthbox_list_tasks

**What it does**: Retrieves all tasks with optional filtering by status, assignee, or due date.

**When to use**: Track pending action items and deadlines.

**Arguments**:
- `status` (optional): Filter by status (open, completed)
- `assignee_id` (optional): Filter by assignee ID
- `due_date_from` (optional): Due date from (YYYY-MM-DD)
- `due_date_to` (optional): Due date to (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all open tasks due this week"

---

### wealthbox_get_task

**What it does**: Gets detailed information about a specific task including description and assignee.

**When to use**: Review task details before working on it or updating status.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task ID 22222"

---

### wealthbox_list_events

**What it does**: Retrieves all calendar events with optional filtering by calendar or date range.

**When to use**: Check upcoming meetings and appointments.

**Arguments**:
- `calendar_id` (optional): Filter by calendar ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all events for tomorrow"

---

## Wealthbox API Notes

- **Financial Advisor Focus**: Wealthbox is designed for financial advisors, so contacts often include client portfolio and account information
- **Activity Types**: Log various interactions including calls, emails, meetings, and tasks
- **Project Workflow**: Track client projects from initiation through completion
- **Task Management**: Assign tasks to team members with due dates and priorities
- **Calendar Integration**: Sync events with popular calendar systems
- **Pagination**: Default per_page is 50, adjust based on your needs