# FullEnrich Tools

Provider: `fullenrich` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the FullEnrich API. They allow AI agents to enrich person and company data, look up domains, manage webhooks, and monitor credit usage. **Requires FullEnrich API key.**

## Authentication

**Nango API_KEY**:
- User provides their FullEnrich API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://app.fullenrich.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fullenrich_get_account` | Get account info | GET | /v2/account |
| `fullenrich_get_credits` | Get account credits | GET | /v2/account/credits |
| `fullenrich_enrich_person` | Enrich person data | POST | /v2/person/enrich |
| `fullenrich_enrich_company` | Enrich company data | POST | /v2/company/enrich |
| `fullenrich_lookup_domain` | Look up company by domain | GET | /v2/company/lookup |
| `fullenrich_list_enrichments` | List enrichment history | GET | /v2/enrichments |
| `fullenrich_get_enrichment` | Get enrichment result | GET | /v2/enrichments/{id} |
| `fullenrich_list_webhooks` | List webhooks | GET | /v2/webhooks |
| `fullenrich_create_webhook` | Create webhook | POST | /v2/webhooks |
| `fullenrich_delete_webhook` | Delete webhook | DELETE | /v2/webhooks/{id} |

---

## Tool Details

### fullenrich_get_account

**What it does**: Gets FullEnrich account information.

**When to use**: Check account status and settings.

**Arguments**: None

**Example LLM prompt**: "Get my FullEnrich account info"

---

### fullenrich_get_credits

**What it does**: Gets account credit balance and usage.

**When to use**: Monitor credit usage for data enrichment.

**Arguments**: None

**Example LLM prompt**: "How many credits do I have left?"

---

### fullenrich_enrich_person

**What it does**: Enriches person data with email or name/company.

**When to use**: Get detailed information about a person (title, company, social profiles).

**Arguments**:
- `email` (optional): Person's email address
- `first_name` (optional): Person's first name
- `last_name` (optional): Person's last name
- `domain` (optional): Company domain

**Example LLM prompt**: "Enrich the person with email john@company.com"

---

### fullenrich_enrich_company

**What it does**: Enriches company data with domain or company name.

**When to use**: Get detailed information about a company (size, revenue, industry).

**Arguments**:
- `domain` (optional): Company domain
- `name` (optional): Company name

**Example LLM prompt**: "Enrich the company with domain example.com"

---

### fullenrich_lookup_domain

**What it does**: Looks up company information by domain.

**When to use**: Quick company lookup using website domain.

**Arguments**:
- `domain` (required): Company domain

**Example LLM prompt**: "Look up company info for google.com"

---

### fullenrich_list_enrichments

**What it does**: Lists recent enrichment requests and results.

**When to use**: View enrichment history and results.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Results per page (default 20)

**Example LLM prompt**: "List my recent enrichments"

---

### fullenrich_get_enrichment

**What it does**: Gets a specific enrichment result by ID.

**When to use**: Get detailed result of a specific enrichment request.

**Arguments**:
- `id` (required): Enrichment ID

**Example LLM prompt**: "Get enrichment result for id abc123"

---

### fullenrich_list_webhooks

**What it does**: Lists all configured webhooks.

**When to use**: View existing webhook subscriptions.

**Arguments**: None

**Example LLM prompt**: "List all my webhooks"

---

### fullenrich_create_webhook

**What it does**: Creates a new webhook for enrichment events.

**When to use**: Set up real-time notifications for enrichment results.

**Arguments**:
- `url` (required): Webhook URL
- `events` (required): Event types to subscribe

**Example LLM prompt**: "Create a webhook for enrichment.completed events at https://myapp.com/webhook"

---

### fullenrich_delete_webhook

**What it does**: Deletes a webhook by ID.

**When to use**: Remove webhook subscription.

**Arguments**:
- `id` (required): Webhook ID

**Example LLM prompt**: "Delete webhook with id abc123"

---

## FullEnrich API Notes

- **Credit usage**: Each enrichment consumes credits
- **Async enrichment**: Some enrichments may be async
- **Webhook events**: Subscribe to enrichment.completed, enrichment.failed
- **Rate limits**: Check headers for rate limit info
