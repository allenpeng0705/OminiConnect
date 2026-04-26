# Maximizer Tools

Provider: `maximizer` | Engine: `nango` | Auth: OAUTH2 via Nango

## Overview

These tools wrap the Maximizer CRM Cloud API. They allow AI agents to manage contacts, companies, opportunities, and activities. **Requires Maximizer OAuth2.**

## Authentication

**Nango OAUTH2**:
- User authenticates via Nango Connect with Maximizer
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.region}.maximizercrmlive.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `maximizer_list_contacts` | List all contacts | GET | /api/v1/contacts |
| `maximizer_get_contact` | Get contact details | GET | /api/v1/contacts/{id} |
| `maximizer_create_contact` | Create a contact | POST | /api/v1/contacts |
| `maximizer_update_contact` | Update a contact | PUT | /api/v1/contacts/{id} |
| `maximizer_list_companies` | List all companies | GET | /api/v1/companies |
| `maximizer_get_company` | Get company details | GET | /api/v1/companies/{id} |
| `maximizer_list_opportunities` | List opportunities | GET | /api/v1/opportunities |
| `maximizer_get_opportunity` | Get opportunity details | GET | /api/v1/opportunities/{id} |
| `maximizer_list_activities` | List activities | GET | /api/v1/activities |
| `maximizer_create_activity` | Create an activity | POST | /api/v1/activities |

---

## Tool Details

### maximizer_list_contacts

**What it does**: Lists all contacts in Maximizer CRM.

**When to use**: Browse contacts, find specific people.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)
- `company_id` (optional): Filter by company ID
- `search` (optional): Search query
- `page_size` (optional): Results per page (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all active contacts at company 123"

---

### maximizer_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact details, view contact history.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### maximizer_create_contact

**What it does**: Creates a new contact in Maximizer.

**When to use**: Add new leads or customers.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company_id` (optional): Company ID

**Example LLM prompt**: "Create a contact for John Doe"

---

### maximizer_update_contact

**What it does**: Updates an existing contact.

**When to use**: Update contact information, change status.

**Arguments**:
- `id` (required): Contact ID
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Update contact 12345 with new email"

---

### maximizer_list_companies

**What it does**: Lists all companies in Maximizer CRM.

**When to use**: Browse companies, find organization.

**Arguments**:
- `status` (optional): Filter by status
- `search` (optional): Search query
- `page_size` (optional): Results per page (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all companies in Maximizer"

---

### maximizer_get_company

**What it does**: Gets details of a specific company.

**When to use**: Get company details, view related contacts.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get details for company 12345"

---

### maximizer_list_opportunities

**What it does**: Lists all opportunities in Maximizer CRM.

**When to use**: Track sales pipeline, find deals.

**Arguments**:
- `stage` (optional): Filter by stage
- `owner_id` (optional): Filter by owner ID
- `company_id` (optional): Filter by company ID
- `page_size` (optional): Results per page (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all opportunities in negotiation stage"

---

### maximizer_get_opportunity

**What it does**: Gets details of a specific opportunity.

**When to use**: Review deal details, check opportunity status.

**Arguments**:
- `id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity 12345"

---

### maximizer_list_activities

**What it does**: Lists all activities in Maximizer CRM.

**When to use**: View activity history, track interactions.

**Arguments**:
- `type` (optional): Filter by activity type
- `contact_id` (optional): Filter by contact ID
- `from_date` (optional): From date (ISO 8601)
- `to_date` (optional): To date (ISO 8601)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all calls with contact 12345"

---

### maximizer_create_activity

**What it does**: Creates a new activity in Maximizer.

**When to use**: Log calls, meetings, or tasks.

**Arguments**:
- `type` (required): Activity type
- `subject` (required): Activity subject
- `contact_id` (optional): Related contact ID
- `date` (optional): Activity date (ISO 8601)
- `description` (optional): Activity description

**Example LLM prompt**: "Log a call with contact 12345 about pricing"

---

## Maximizer Notes

- **Contacts**: Individual people in CRM
- **Companies**: Organizations that contain contacts
- **Opportunities**: Sales deals in pipeline
- **Activities**: Calls, meetings, tasks, notes
- **Region config**: Required for Maximizer Cloud
