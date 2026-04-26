# Accelo Tools

Provider: `accelo` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Accelo is a professional services automation platform that helps agencies and service businesses manage contacts, companies, matters (cases/projects), tasks, and activities. These tools wrap the Accelo REST API, enabling AI agents to track client work, manage project matters, and monitor team activities.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Accelo
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write` (based on permission level)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `accelo_list_contacts` | List contacts | GET | /api/contacts |
| `accelo_get_contact` | Get contact details | GET | /api/contacts/{id} |
| `accelo_create_contact` | Create a new contact | POST | /api/contacts |
| `accelo_list_companies` | List companies | GET | /api/companies |
| `accelo_get_company` | Get company details | GET | /api/companies/{id} |
| `accelo_list_matters` | List matters | GET | /api/matters |
| `accelo_get_matter` | Get matter details | GET | /api/matters/{id} |
| `accelo_list_tasks` | List tasks | GET | /api/tasks |
| `accelo_get_task` | Get task details | GET | /api/tasks/{id} |
| `accelo_list_activities` | List activities | GET | /api/activities |

---

## Tool Details

### accelo_list_contacts

**What it does**: Lists all contacts in Accelo with optional company filtering.

**When to use**: Browse contacts, find people at a specific company.

**Arguments**:
- `company_id` (optional): Filter by company ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all contacts"

---

### accelo_get_contact

**What it does**: Gets detailed information about a specific contact including email, phone, and company.

**When to use**: Get full contact details before outreach or follow-up.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### accelo_create_contact

**What it does**: Creates a new contact in Accelo.

**When to use**: Add new clients, capture contact information from conversations.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Associated company ID

**Example LLM prompt**: "Add a new contact named John Smith with email john@example.com"

---

### accelo_list_companies

**What it does**: Lists all companies in Accelo.

**When to use**: Browse client companies, find accounts to associate contacts or matters.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all companies"

---

### accelo_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: Review company details, see associated contacts and matters.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company 67890"

---

### accelo_list_matters

**What it does**: Lists all matters (cases, projects, or files) with optional status and company filtering.

**When to use**: Browse active projects, find matters for a specific client.

**Arguments**:
- `status` (optional): Filter by status (open, closed, pending)
- `company_id` (optional): Filter by company ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all open matters for company 67890"

---

### accelo_get_matter

**What it does**: Gets detailed information about a specific matter including status, assigned staff, and related tasks.

**When to use**: Review matter details, understand project scope and progress.

**Arguments**:
- `id` (required): Matter ID

**Example LLM prompt**: "Get details for matter 11111"

---

### accelo_list_tasks

**What it does**: Lists all tasks with optional status and matter filtering.

**When to use**: Review task queue, find tasks for a specific matter or project.

**Arguments**:
- `status` (optional): Filter by status (active, completed)
- `matter_id` (optional): Filter by matter ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all active tasks for matter 11111"

---

### accelo_get_task

**What it does**: Gets detailed information about a specific task including assignee, due date, and status.

**When to use**: Review task details before starting work or marking complete.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task 99999"

---

### accelo_list_activities

**What it does**: Lists all activities (notes, emails, calls, meetings) with optional filtering.

**When to use**: Review client interaction history, see recent communications on a matter.

**Arguments**:
- `object` (optional): Filter by object type (contact, company, matter)
- `object_id` (optional): Filter by object ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all activities for matter 11111"

---

## Accelo API Notes

- **IDs**: Accelo uses integer IDs for all objects
- **Contacts**: Individual people who may be associated with companies
- **Companies**: Client businesses that can have multiple contacts
- **Matters**: Cases, projects, or files - the core work tracking unit in Accelo
- **Tasks**: Action items that can be linked to matters or other objects
- **Activities**: Communication history including notes, emails, calls, and meetings
- **Professional Services**: Accelo is designed for agencies and service businesses
