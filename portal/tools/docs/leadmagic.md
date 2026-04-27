# LeadMagic Tools

Provider: `leadmagic` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the LeadMagic API. They allow AI agents to manage enrichments, lookups, companies, and webhooks. **Requires LeadMagic API key.**

## Authentication

**API Key via Nango**:
- User provides their LeadMagic API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.leadmagic.io`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `leadmagic_list_enrichments` | List enrichments | GET | /v1/enrichments |
| `leadmagic_get_enrichment` | Get enrichment details | GET | /v1/enrichments/{enrichment_id} |
| `leadmagic_list_credits` | Get credit balance | GET | /v1/credits |
| `leadmagic_list_lookups` | List lookups | GET | /v1/lookups |
| `leadmagic_create_lookup` | Create a lookup | POST | /v1/lookups |
| `leadmagic_list_domains` | List domains | GET | /v1/domains |
| `leadmagic_get_domain` | Get domain details | GET | /v1/domains/{domain_id} |
| `leadmagic_list_companies` | List companies | GET | /v1/companies |
| `leadmagic_get_company` | Get company details | GET | /v1/companies/{company_id} |
| `leadmagic_list_webhooks` | List webhooks | GET | /v1/webhooks |

---

## Tool Details

### leadmagic_list_enrichments

**What it does**: Lists all enrichments.

**When to use**: View enrichment history.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all enrichments in LeadMagic"

---

### leadmagic_get_enrichment

**What it does**: Gets details for a specific enrichment.

**When to use**: Get enrichment information.

**Arguments**:
- `enrichment_id` (required): Enrichment ID

**Example LLM prompt**: "Get details for enrichment abc123"

---

### leadmagic_list_credits

**What it does**: Gets the current credit balance.

**When to use**: Check available credits.

**Arguments**: None

**Example LLM prompt**: "Get my credit balance in LeadMagic"

---

### leadmagic_list_lookups

**What it does**: Lists all lookups.

**When to use**: View lookup history.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all lookups in LeadMagic"

---

### leadmagic_create_lookup

**What it does**: Creates a new lookup.

**When to use**: Enrich a new email.

**Arguments**:
- `email` (required): Email to look up

**Example LLM prompt**: "Look up john@example.com"

---

### leadmagic_list_domains

**What it does**: Lists all domains.

**When to use**: View tracked domains.

**Arguments**: None

**Example LLM prompt**: "List all domains in LeadMagic"

---

### leadmagic_get_domain

**What it does**: Gets details for a specific domain.

**When to use**: Get domain information.

**Arguments**:
- `domain_id` (required): Domain ID

**Example LLM prompt**: "Get details for domain d1"

---

### leadmagic_list_companies

**What it does**: Lists all companies.

**When to use**: View company data.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all companies in LeadMagic"

---

### leadmagic_get_company

**What it does**: Gets details for a specific company.

**When to use**: Get company information.

**Arguments**:
- `company_id` (required): Company ID

**Example LLM prompt**: "Get details for company c1"

---

### leadmagic_list_webhooks

**What it does**: Lists all webhooks.

**When to use**: View webhook configurations.

**Arguments**: None

**Example LLM prompt**: "List all webhooks in LeadMagic"

---

## LeadMagic API Notes

- **Lead Enrichment**: Data enrichment for leads and contacts
- **Lookups**: Query email addresses for enriched data
- **Companies**: Company information and data
- **Credits**: Usage tracking and billing
- **Webhooks**: Real-time notifications
