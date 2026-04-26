# Insightly Tools

Provider: `insightly` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Insightly is a CRM platform with strong features for managing contacts, organizations, opportunities, and projects. These tools wrap the Insightly REST API (v2.2), enabling AI agents to access and manage CRM data for sales automation, project tracking, and customer relationship management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Insightly
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write` (based on permission level)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `insightly_list_contacts` | List contacts | GET | /api/Contacts |
| `insightly_get_contact` | Get contact details | GET | /api/Contacts/{id} |
| `insightly_create_contact` | Create a new contact | POST | /api/Contacts |
| `insightly_list_organizations` | List organizations | GET | /api/Organizations |
| `insightly_get_organization` | Get organization details | GET | /api/Organizations/{id} |
| `insightly_list_opportunities` | List opportunities | GET | /api/Opportunities |
| `insightly_get_opportunity` | Get opportunity details | GET | /api/Opportunities/{id} |
| `insightly_list_projects` | List projects | GET | /api/Projects |
| `insightly_get_project` | Get project details | GET | /api/Projects/{id} |
| `insightly_list_tasks` | List tasks | GET | /api/Tasks |

---

## Tool Details

### insightly_list_contacts

**What it does**: Lists all contacts in the Insightly CRM with optional email filtering.

**When to use**: Browse contacts, search for specific people by email.

**Arguments**:
- `email` (optional): Filter by email address
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all contacts in the CRM"

---

### insightly_get_contact

**What it does**: Gets detailed information about a specific contact including address, social links, and organization.

**When to use**: Get full contact details before outreach or follow-up.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### insightly_create_contact

**What it does**: Creates a new contact in Insightly CRM.

**When to use**: Add new leads, capture contact information from conversations.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `organization_id` (optional): Associated organization ID

**Example LLM prompt**: "Add a new contact named Jane Doe with email jane@example.com"

---

### insightly_list_organizations

**What it does**: Lists all organizations (companies) in the Insightly CRM.

**When to use**: Browse companies, find accounts for associating contacts or opportunities.

**Arguments**:
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all organizations"

---

### insightly_get_organization

**What it does**: Gets detailed information about a specific organization including industry and address.

**When to use**: Review organization details, understand company context for deals.

**Arguments**:
- `id` (required): Organization ID

**Example LLM prompt**: "Get details for organization 67890"

---

### insightly_list_opportunities

**What it does**: Lists all opportunities (deals) with optional pipeline stage filtering.

**When to use**: Review sales pipeline, check opportunity progress.

**Arguments**:
- `stage_id` (optional): Filter by pipeline stage ID
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all opportunities in the pipeline"

---

### insightly_get_opportunity

**What it does**: Gets detailed information about a specific opportunity including value, stage, and probability.

**When to use**: Review deal details, understand sales progress.

**Arguments**:
- `id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity 11111"

---

### insightly_list_projects

**What it does**: Lists all projects in Insightly with optional status filtering.

**When to use**: Browse active projects, find projects for tracking or updates.

**Arguments**:
- `status` (optional): Filter by status (active, completed)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all active projects"

---

### insightly_get_project

**What it does**: Gets detailed information about a specific project including milestones and tasks.

**When to use**: Review project status, understand project scope and timeline.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get details for project 22222"

---

### insightly_list_tasks

**What it does**: Lists all tasks in Insightly CRM with optional status filtering.

**When to use**: Review pending tasks, find overdue items, plan daily work.

**Arguments**:
- `status` (optional): Filter by status (not started, in progress, completed)
- `limit` (optional): Max results (default 50)

**Example LLM prompt**: "List all incomplete tasks"

---

## Insightly API Notes

- **IDs**: Insightly uses integer IDs for all objects
- **Contacts**: Individual people associated with organizations
- **Organizations**: Companies/businesses that can have multiple contacts
- **Opportunities**: Sales deals with values, stages, and probabilities
- **Projects**: Long-term initiatives that can span multiple organizations
- **Tasks**: Action items that can be linked to any CRM object
