# Quentn Tools

Provider: `quentn` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Quentn is a marketing automation platform that helps businesses manage contacts, campaigns, and forms. These tools allow AI agents to manage contacts, view campaigns, and handle tasks.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Quentn
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contacts:read`, `contacts:write`, `campaigns:read`, `forms:read`, `tasks:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `quentn_list_contacts` | List all contacts | GET | /contacts |
| `quentn_get_contact` | Get a specific contact | GET | /contacts/{id} |
| `quentn_create_contact` | Create a new contact | POST | /contacts |
| `quentn_update_contact` | Update an existing contact | PUT | /contacts/{id} |
| `quentn_delete_contact` | Delete a contact | DELETE | /contacts/{id} |
| `quentn_list_campaigns` | List all campaigns | GET | /campaigns |
| `quentn_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `quentn_list_forms` | List all forms | GET | /forms |
| `quentn_get_form` | Get form details | GET | /forms/{id} |
| `quentn_create_task` | Create a task | POST | /tasks |

---

## Tool Details

### quentn_list_contacts

**What it does**: Returns a paginated list of all contacts.

**When to use**: Find contacts, search your contact database.

**Arguments**:
- `limit` (optional): Number of contacts to return (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all contacts in Quentn"

---

### quentn_get_contact

**What it does**: Gets details of a specific contact by ID.

**When to use**: Get contact details, check contact information.

**Arguments**:
- `id` (required): The contact ID

**Example LLM prompt**: "Get details for contact ID 12345"

---

### quentn_create_contact

**What it does**: Creates a new contact with the provided information.

**When to use**: Add new leads or customers to your database.

**Arguments**:
- `email` (required): Email address
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a new contact for john@example.com"

---

### quentn_update_contact

**What it does**: Updates an existing contact's information.

**When to use**: Modify contact details.

**Arguments**:
- `id` (required): The contact ID
- `email` (optional): Updated email
- `firstName` (optional): Updated first name
- `lastName` (optional): Updated last name
- `phone` (optional): Updated phone

**Example LLM prompt**: "Update contact 12345 with phone number 555-1234"

---

### quentn_delete_contact

**What it does**: Deletes a contact from your account.

**When to use**: Remove unwanted contacts.

**Arguments**:
- `id` (required): The contact ID to delete

**Example LLM prompt**: "Delete contact 12345"

---

### quentn_list_campaigns

**What it does**: Returns a list of all email campaigns.

**When to use**: View your marketing campaigns.

**Arguments**:
- `limit` (optional): Number of campaigns to return
- `status` (optional): Filter by status (active, paused, draft)

**Example LLM prompt**: "List all active campaigns"

---

### quentn_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign performance, get campaign settings.

**Arguments**:
- `id` (required): The campaign ID

**Example LLM prompt**: "Get details for campaign 567"

---

### quentn_list_forms

**What it does**: Returns a list of all landing page forms.

**When to use**: View available lead capture forms.

**Arguments**:
- `limit` (optional): Number of forms to return

**Example LLM prompt**: "List all Quentn forms"

---

### quentn_get_form

**What it does**: Gets details of a specific form.

**When to use**: Get form configuration and fields.

**Arguments**:
- `id` (required): The form ID

**Example LLM prompt**: "Get details for form 789"

---

### quentn_create_task

**What it does**: Creates a new task associated with a contact.

**When to use**: Create follow-up tasks for sales or support.

**Arguments**:
- `title` (required): Task title
- `contactId` (optional): Associated contact ID
- `dueDate` (optional): Due date (ISO 8601)
- `priority` (optional): Priority level (low, normal, high)

**Example LLM prompt**: "Create a task to follow up with contact 12345"

---

## Quentn API Notes

- Contact IDs are integers in the API
- Campaign IDs are integers
- Tasks can be linked to contacts for follow-up tracking
