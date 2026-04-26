# Odoo (OAuth) Tools

Provider: `odoo` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Odoo REST API. They allow AI agents to manage contacts, projects, tasks, sales orders, and CRM leads. **Requires Odoo OAuth authentication.**

## Authentication

**OAuth2**:
- User authenticates via Nango Connect with Odoo
- Authorization URL: `https://{serverUrl}/restapi/1.0/common/oauth2/authorize`
- Token URL: `https://{serverUrl}/restapi/1.0/common/oauth2/access_token`
- Base URL: `https://{serverUrl}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `odoo_list_contacts` | List contacts | GET | /res.partner |
| `odoo_get_contact` | Get contact details | GET | /res.partner/{id} |
| `odoo_create_contact` | Create contact | POST | /res.partner |
| `odoo_update_contact` | Update contact | PUT | /res.partner/{id} |
| `odoo_list_tasks` | List tasks | GET | /project.task |
| `odoo_get_task` | Get task details | GET | /project.task/{id} |
| `odoo_create_task` | Create task | POST | /project.task |
| `odoo_list_projects` | List projects | GET | /project.project |
| `odoo_list_sales` | List sales orders | GET | /sale.order |
| `odoo_get_lead` | Get CRM lead | GET | /crm.lead/{id} |

---

## Tool Details

### odoo_list_contacts

**What it does**: Lists all contacts in Odoo.

**When to use**: Browse contact directory, find customers.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Items per page (default 20)

**Example LLM prompt**: "List all contacts with email domain @company.com"

---

### odoo_get_contact

**What it does**: Gets detailed information for a specific contact.

**When to use**: View contact details, communication history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact ID 123"

---

### odoo_create_contact

**What it does**: Creates a new contact in Odoo.

**When to use**: Add new customers, create leads.

**Arguments**:
- `name` (required): Contact name
- `email` (optional): Contact email
- `phone` (optional): Contact phone

**Example LLM prompt**: "Create a new contact for John Doe with email john@company.com"

---

### odoo_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact information, update status.

**Arguments**:
- `id` (required): Contact ID
- `name` (optional): Contact name
- `email` (optional): Contact email

**Example LLM prompt**: "Update contact 123 with new phone number"

---

### odoo_list_tasks

**What it does**: Lists all tasks in Odoo Project.

**When to use**: Browse tasks, track progress.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `stage_id` (optional): Filter by stage ID

**Example LLM prompt**: "List all tasks in project 5 that are in progress"

---

### odoo_get_task

**What it does**: Gets detailed information for a specific task.

**When to use**: View task details, check assignments.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task 456"

---

### odoo_create_task

**What it does**: Creates a new task in Odoo Project.

**When to use**: Create project tasks, assign work.

**Arguments**:
- `name` (required): Task name
- `project_id` (required): Project ID
- `description` (optional): Task description

**Example LLM prompt**: "Create a task called 'Review PR' in project 5"

---

### odoo_list_projects

**What it does**: Lists all projects in Odoo.

**When to use**: Browse projects, find work areas.

**Arguments**: None

**Example LLM prompt**: "List all active projects"

---

### odoo_list_sales

**What it does**: Lists all sales orders in Odoo.

**When to use**: Track sales pipeline, manage orders.

**Arguments**:
- `state` (optional): Filter by state (draft, sale, done)

**Example LLM prompt**: "List all confirmed sales orders"

---

### odoo_get_lead

**What it does**: Gets detailed information for a CRM lead.

**When to use**: View lead details, track opportunities.

**Arguments**:
- `id` (required): Lead ID

**Example LLM prompt**: "Get details for lead 789"

---

## Odoo Notes

- **Server URL**: Your Odoo instance domain
- **Model IDs**: Numeric IDs used for Odoo models
- **States**: Different states for different document types
- **Projects**: Tasks belong to projects in Odoo Project
- **Contacts**: Both customers and leads are stored as partners
