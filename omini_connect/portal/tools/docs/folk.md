# Folk Tools

Provider: `folk` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Folk API. They allow AI agents to manage contacts, deals, companies, and tasks. Folk is a CRM platform for managing relationships and deals.

## Authentication

**Nango API_KEY**:
- User provides their Folk API key via Nango Connect
- Key is passed in the Authorization header as Bearer token
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `folk_list_contacts` | List contacts | GET | /v1/contacts |
| `folk_get_contact` | Get contact details | GET | /v1/contacts/{id} |
| `folk_create_contact` | Create a contact | POST | /v1/contacts |
| `folk_update_contact` | Update contact details | PUT | /v1/contacts/{id} |
| `folk_list_deals` | List deals | GET | /v1/deals |
| `folk_get_deal` | Get deal details | GET | /v1/deals/{id} |
| `folk_list_companies` | List companies | GET | /v1/companies |
| `folk_get_company` | Get company details | GET | /v1/companies/{id} |
| `folk_list_tasks` | List tasks | GET | /v1/tasks |
| `folk_get_task` | Get task details | GET | /v1/tasks/{id} |

---

## Tool Details

### folk_list_contacts

**What it does**: Lists all contacts.

**When to use**: Browse contact database.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all contacts"

---

### folk_get_contact

**What it does**: Gets details of a specific contact.

**When to use**: View contact information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get contact abc123"

---

### folk_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new people to the CRM.

**Arguments**:
- `name` (required): Contact name
- `email` (optional): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a contact for John Doe"

---

### folk_update_contact

**What it does**: Updates contact details.

**When to use**: Modify contact information.

**Arguments**:
- `id` (required): Contact ID
- `name` (optional): Contact name
- `email` (optional): Email address

**Example LLM prompt**: "Update contact abc123 with new email"

---

### folk_list_deals

**What it does**: Lists all deals.

**When to use**: Browse deal pipeline.

**Arguments**:
- `stage` (optional): Filter by stage

**Example LLM prompt**: "List all deals"

---

### folk_get_deal

**What it does**: Gets details of a specific deal.

**When to use**: View deal information.

**Arguments**:
- `id` (required): Deal ID

**Example LLM prompt**: "Get deal xyz789"

---

### folk_list_companies

**What it does**: Lists all companies.

**When to use**: Browse company database.

**Arguments**: None

**Example LLM prompt**: "List all companies"

---

### folk_get_company

**What it does**: Gets details of a specific company.

**When to use**: View company information.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get company def456"

---

### folk_list_tasks

**What it does**: Lists all tasks.

**When to use**: Browse task list.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all tasks"

---

### folk_get_task

**What it does**: Gets details of a specific task.

**When to use**: View task details.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get task ghi789"

---

## Folk API Notes

- **Contacts**: Individual people in the CRM
- **Companies**: Organizations linked to contacts
- **Deals**: Sales opportunities
- **Tasks**: Action items and follow-ups
- **API Key Format**: Key starts with `FOLK` prefix
- **Rate Limiting**: Check retry-after header
