# NationBuilder Tools

Provider: `nationbuilder` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the NationBuilder CRM API. They allow AI agents to manage people, donations, events, petitions, surveys, and email broadcasts. **Requires NationBuilder OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with NationBuilder
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{accountId}.nationbuilder.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `nationbuilder_list_people` | List people/contacts | GET | /people |
| `nationbuilder_get_person` | Get person details | GET | /people/{id} |
| `nationbuilder_list_donations` | List donations | GET | /donations |
| `nationbuilder_list_events` | List events | GET | /events |
| `nationbuilder_list_petitions` | List petitions | GET | /petitions |
| `nationbuilder_list_surveys` | List surveys | GET | /surveys |
| `nationbuilder_list_pages` | List website pages | GET | /pages |
| `nationbuilder_list_tags` | List tags | GET | /tags |
| `nationbuilder_list_lists` | List saved lists | GET | /lists |
| `nationbuilder_list_broadcasts` | List broadcasts | GET | /broadcasts |

---

## Tool Details

### nationbuilder_list_people

**What it does**: Lists all people and contacts in the CRM.

**When to use**: Browse contacts, find people by tag, search constituency.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Items per page (default 25, max 100)
- `tag` (optional): Filter by tag slug

**Example LLM prompt**: "List all people with the 'volunteer' tag"

---

### nationbuilder_get_person

**What it does**: Gets detailed information for a specific person.

**When to use**: View contact details, donation history, event attendance.

**Arguments**:
- `id` (required): Person ID

**Example LLM prompt**: "Get details for person ID 12345"

---

### nationbuilder_list_donations

**What it does**: Lists all donations in the system.

**When to use**: Track fundraising, view donor history.

**Arguments**:
- `person_id` (optional): Filter by person ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all donations over $100 this month"

---

### nationbuilder_list_events

**What it does**: Lists all events and gatherings.

**When to use**: Browse events, find upcoming gatherings.

**Arguments**:
- `page` (optional): Page number
- `slug` (optional): Filter by event slug

**Example LLM prompt**: "List all upcoming events in my area"

---

### nationbuilder_list_petitions

**What it does**: Lists all petitions in the system.

**When to use**: View petition campaigns, signature counts.

**Arguments**: None

**Example LLM prompt**: "List all active petitions"

---

### nationbuilder_list_surveys

**What it does**: Lists all surveys and their responses.

**When to use**: Review survey data, analyze responses.

**Arguments**: None

**Example LLM prompt**: "List all surveys with their response counts"

---

### nationbuilder_list_pages

**What it does**: Lists all website pages and content.

**When to use**: Browse website content, find specific pages.

**Arguments**:
- `type` (optional): Filter by page type

**Example LLM prompt**: "List all press release pages"

---

### nationbuilder_list_tags

**What it does**: Lists all tags used to categorize people.

**When to use**: Understand tagging structure, find tag categories.

**Arguments**: None

**Example LLM prompt**: "List all tags used in the CRM"

---

### nationbuilder_list_lists

**What it does**: Lists all saved lists of people.

**When to use**: View segmentation lists, find specific cohorts.

**Arguments**: None

**Example LLM prompt**: "List all saved lists"

---

### nationbuilder_list_broadcasts

**What it does**: Lists all email broadcasts and campaigns.

**When to use**: Track email campaigns, view send history.

**Arguments**:
- `status` (optional): Filter by status (draft, scheduled, sent)

**Example LLM prompt**: "List all broadcasts sent this month"

---

## NationBuilder API Notes

- **Account ID**: Used as subdomain prefix (e.g., `acme.nationbuilder.com`)
- **Pagination**: Use page and per_page parameters for large result sets
- **Tags**: String slugs used for filtering and segmentation
- **Rate limits**: Implement backoff for heavy API usage
