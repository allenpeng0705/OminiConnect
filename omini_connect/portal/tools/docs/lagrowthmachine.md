# La Growth Machine Tools

Provider: `lagrowthmachine` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the La Growth Machine API. They allow AI agents to manage sequences, contacts, tasks, and campaigns. **Requires La Growth Machine API key.**

## Authentication

**API Key via Nango**:
- User provides their La Growth Machine API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://apiv2.lagrowthmachine.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lagrowthmachine_list_sequences` | List sequences | GET | /sequences |
| `lagrowthmachine_get_sequence` | Get sequence details | GET | /sequences/{sequence_id} |
| `lagrowthmachine_list_contacts` | List contacts | GET | /contacts |
| `lagrowthmachine_get_contact` | Get contact details | GET | /contacts/{contact_id} |
| `lagrowthmachine_add_contact` | Add a contact | POST | /contacts |
| `lagrowthmachine_update_contact` | Update a contact | PUT | /contacts/{contact_id} |
| `lagrowthmachine_list_tasks` | List tasks | GET | /tasks |
| `lagrowthmachine_complete_task` | Complete a task | POST | /tasks/{task_id}/complete |
| `lagrowthmachine_list_campaigns` | List campaigns | GET | /campaigns |
| `lagrowthmachine_get_campaign` | Get campaign details | GET | /campaigns/{campaign_id} |

---

## Tool Details

### lagrowthmachine_list_sequences

**What it does**: Lists all sequences.

**When to use**: View sequences, find outreach flows.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all sequences in La Growth Machine"

---

### lagrowthmachine_get_sequence

**What it does**: Gets details for a specific sequence.

**When to use**: Get sequence information.

**Arguments**:
- `sequence_id` (required): Sequence ID

**Example LLM prompt**: "Get details for sequence abc123"

---

### lagrowthmachine_list_contacts

**What it does**: Lists all contacts.

**When to use**: View contacts, find leads.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all contacts in La Growth Machine"

---

### lagrowthmachine_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: Get contact information.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact xyz789"

---

### lagrowthmachine_add_contact

**What it does**: Adds a new contact.

**When to use**: Add leads to La Growth Machine.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Add a contact for john@example.com"

---

### lagrowthmachine_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact data.

**Arguments**:
- `contact_id` (required): Contact ID
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update contact xyz789 with new name"

---

### lagrowthmachine_list_tasks

**What it does**: Lists all tasks.

**When to use**: View tasks, track follow-ups.

**Arguments**:
- `contact_id` (optional): Filter by contact ID

**Example LLM prompt**: "List all tasks in La Growth Machine"

---

### lagrowthmachine_complete_task

**What it does**: Marks a task as complete.

**When to use**: Complete follow-ups.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Complete task t1"

---

### lagrowthmachine_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: View campaigns, find outreach programs.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all campaigns in La Growth Machine"

---

### lagrowthmachine_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Get campaign information.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign c1"

---

## La Growth Machine API Notes

- **Sales Engagement**: Multi-channel outreach platform
- **Sequences**: Automated outreach flows
- **Contacts**: Leads and prospects
- **Tasks**: Follow-up actions
- **Campaigns**: Grouped outreach programs
