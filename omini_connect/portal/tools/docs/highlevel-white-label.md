# HighLevel White Label Tools

Provider: `highlevel-white-label` | Engine: `nango` | Auth: OAuth2 via Nango (LeadConnector)

## Overview

These tools wrap the HighLevel White Label (LeadConnector) API. They allow AI agents to manage contacts, opportunities, tasks, appointments, and campaigns. LeadConnector is the white-label version of HighLevel for agencies.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with LeadConnector
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://marketplace.leadconnectorhq.com/oauth/chooselocation
- Token URL: https://services.leadconnectorhq.com/oauth/token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `highlevel_list_contacts` | List contacts | GET | /contacts |
| `highlevel_get_contact` | Get contact details | GET | /contacts/{id} |
| `highlevel_list_opportunities` | List opportunities | GET | /opportunities |
| `highlevel_get_opportunity` | Get opportunity details | GET | /opportunities/{id} |
| `highlevel_list_tasks` | List tasks | GET | /tasks |
| `highlevel_get_task` | Get task details | GET | /tasks/{id} |
| `highlevel_list_appointments` | List appointments | GET | /appointments |
| `highlevel_get_appointment` | Get appointment details | GET | /appointments/{id} |
| `highlevel_list_campaigns` | List campaigns | GET | /campaigns |
| `highlevel_list_forms` | List forms | GET | /forms |

---

## Tool Details

### highlevel_list_contacts

**What it does**: Lists all contacts in HighLevel White Label.

**When to use**: Browse contact directory.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts"

---

### highlevel_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: View contact profile and pipeline status.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact with ID abc123"

---

### highlevel_list_opportunities

**What it does**: Lists all opportunities in HighLevel White Label.

**When to use**: Browse sales pipeline.

**Arguments**:
- `pipeline_id` (optional): Filter by pipeline ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all opportunities"

---

### highlevel_get_opportunity

**What it does**: Gets detailed information about a specific opportunity.

**When to use**: View opportunity details and value.

**Arguments**:
- `id` (required): Opportunity ID

**Example LLM prompt**: "Get opportunity with ID xyz789"

---

### highlevel_list_tasks

**What it does**: Lists all tasks in HighLevel White Label.

**When to use**: Browse task list.

**Arguments**:
- `contact_id` (optional): Filter by contact ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all tasks for contact abc123"

---

### highlevel_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: View task details and due date.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get task with ID tsk456"

---

### highlevel_list_appointments

**What it does**: Lists all appointments in HighLevel White Label.

**When to use**: Browse appointment calendar.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List appointments for next week"

---

### highlevel_get_appointment

**What it does**: Gets detailed information about a specific appointment.

**When to use**: View appointment details.

**Arguments**:
- `id` (required): Appointment ID

**Example LLM prompt**: "Get appointment with ID apt789"

---

### highlevel_list_campaigns

**What it does**: Lists all campaigns in HighLevel White Label.

**When to use**: Browse marketing campaigns.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### highlevel_list_forms

**What it does**: Lists all forms in HighLevel White Label.

**When to use**: Browse lead capture forms.

**Arguments**: None

**Example LLM prompt**: "List all forms"

---

## HighLevel White Label API Notes

- **API Base URL**: https://services.leadconnectorhq.com
- **Auth Mode**: OAuth2
- **Location**: Agency manages multiple client locations
- **Contacts**: Customer and lead records
- **Opportunities**: Sales pipeline deals
- **Tasks**: To-do items and reminders
- **Appointments**: Scheduled meetings and calls
- **Campaigns**: Marketing automation campaigns
- **Forms**: Lead capture web forms
