# Marketo Tools

Provider: `marketo` | Engine: `nango` | Auth: OAUTH2_CC via Nango (Client Credentials)

## Overview

These tools wrap the Marketo REST API. They allow AI agents to manage leads, campaigns, programs, and marketing activities. **Requires Marketo Client Credentials.**

## Authentication

**Nango OAUTH2_CC (Client Credentials)**:
- Uses client_id and client_secret via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.endpointURL}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `marketo_list_leads` | List leads | GET | /rest/v1/leads.json |
| `marketo_get_lead` | Get lead details | GET | /rest/v1/leads/{id}.json |
| `marketo_create_lead` | Create a lead | POST | /rest/v1/leads.json |
| `marketo_update_lead` | Update a lead | POST | /rest/v1/leads.json |
| `marketo_list_campaigns` | List marketing campaigns | GET | /rest/v1/campaigns.json |
| `marketo_get_campaign` | Get campaign details | GET | /rest/v1/campaigns/{id}.json |
| `marketo_list_activities` | List activities | GET | /rest/v1/activities.json |
| `marketo_list_programs` | List programs | GET | /rest/v1/programs.json |
| `marketo_get_program` | Get program details | GET | /rest/v1/programs/{id}.json |
| `marketo_list_lists` | List static lists | GET | /rest/v1/lists.json |

---

## Tool Details

### marketo_list_leads

**What it does**: Lists leads in Marketo.

**When to use**: Find leads, filter by segment or source.

**Arguments**:
- `filter_type` (optional): Field to filter by (email, id, etc.)
- `filter_values` (optional): Comma-separated filter values
- `fields` (optional): Fields to return
- `page` (optional): Page number (default 1)
- `maxReturn` (optional): Max results (default 20)

**Example LLM prompt**: "List leads with email containing @company.com"

---

### marketo_get_lead

**What it does**: Gets detailed information about a specific lead.

**When to use**: Get lead details, check lead score.

**Arguments**:
- `id` (required): Lead ID
- `fields` (optional): Fields to return

**Example LLM prompt**: "Get details for lead 12345"

---

### marketo_create_lead

**What it does**: Creates a new lead in Marketo.

**When to use**: Add new leads from web forms or imports.

**Arguments**:
- `email` (required): Email address
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `company` (optional): Company name

**Example LLM prompt**: "Create a lead for john@example.com"

---

### marketo_update_lead

**What it does**: Updates an existing lead in Marketo.

**When to use**: Update lead information, change lead score.

**Arguments**:
- `id` (required): Lead ID
- `email` (optional): Email address
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `company` (optional): Company name

**Example LLM prompt**: "Update lead 12345 with new company"

---

### marketo_list_campaigns

**What it does**: Lists all marketing campaigns in Marketo.

**When to use**: Browse campaigns, check campaign status.

**Arguments**:
- `status` (optional): Filter by status (on, off)
- `page` (optional): Page number (default 1)
- `maxReturn` (optional): Max results (default 20)

**Example LLM prompt**: "List all active campaigns"

---

### marketo_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign settings, analyze campaign performance.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 12345"

---

### marketo_list_activities

**What it does**: Lists marketing activities in Marketo.

**When to use**: Track lead activities, analyze behavior.

**Arguments**:
- `activityTypeIds` (optional): Comma-separated activity type IDs
- `startDate` (optional): Start date (ISO 8601)
- `endDate` (optional): End date (ISO 8601)
- `page` (optional): Page number (default 1)
- `maxReturn` (optional): Max results (default 20)

**Example LLM prompt**: "List all website visits from the last week"

---

### marketo_list_programs

**What it does**: Lists all programs in Marketo.

**When to use**: Browse programs, manage nurture tracks.

**Arguments**:
- `status` (optional): Filter by status
- `type` (optional): Filter by type
- `page` (optional): Page number (default 1)
- `maxReturn` (optional): Max results (default 20)

**Example LLM prompt**: "List all email nurture programs"

---

### marketo_get_program

**What it does**: Gets details of a specific program.

**When to use**: Check program members, analyze program effectiveness.

**Arguments**:
- `id` (required): Program ID

**Example LLM prompt**: "Get details for program 12345"

---

### marketo_list_lists

**What it does**: Lists all static lists in Marketo.

**When to use**: Manage target lists, find list members.

**Arguments**:
- `page` (optional): Page number (default 1)
- `maxReturn` (optional): Max results (default 20)

**Example LLM prompt**: "List all static lists"

---

## Marketo Notes

- **Leads**: Marketing leads with activities and scores
- **Campaigns**: Marketing campaigns (email, digital, events)
- **Programs**: Nurture tracks and engagement programs
- **Activities**: Behavioral tracking events
- **Rate limits**: Marketo has strict API rate limits
