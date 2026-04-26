# Prospeo Tools

Provider: `prospeo` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Prospeo API. They allow AI agents to search and enrich companies and contacts, manage campaigns. Prospeo is a B2B data enrichment platform. **Requires Prospeo API Key authentication.**

## Authentication

**Nango API Key**:
- Uses X-Key header for authentication
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.prospeo.io

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `prospeo_get_account_info` | Get account info | POST | /account-information |
| `prospeo_search_companies` | Search companies | POST | /company/search |
| `prospeo_get_company` | Get company details | POST | /company |
| `prospeo_search_contacts` | Search contacts | POST | /contact/search |
| `prospeo_get_contact` | Get contact details | POST | /contact |
| `prospeo_enrich_company` | Enrich company data | POST | /company/enrich |
| `prospeo_enrich_contact` | Enrich contact data | POST | /contact/enrich |
| `prospeo_get_domain_info` | Get domain info | POST | /domain |
| `prospeo_list_campaigns` | List campaigns | GET | /campaigns |
| `prospeo_get_campaign` | Get campaign details | GET | /campaigns/{campaignId} |

---

## Tool Details

### prospeo_get_account_info

**What it does**: Gets account information and usage stats.

**When to use**: Check credits, account status.

**Arguments**: None

**Example LLM prompt**: "Get my account information"

---

### prospeo_search_companies

**What it does**: Searches for companies by name, domain, or other criteria.

**When to use**: Find companies for outreach.

**Arguments**:
- `name` (optional): Company name
- `domain` (optional): Company domain
- `location` (optional): Location filter

**Example LLM prompt**: "Find tech companies in San Francisco"

---

### prospeo_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: Get company profile, details.

**Arguments**:
- `companyId` (required): Company ID or domain

**Example LLM prompt**: "Get details for example.com"

---

### prospeo_search_contacts

**What it does**: Searches for contacts by name, email, company, etc.

**When to use**: Find contacts for outreach.

**Arguments**:
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `company` (optional): Company name
- `email` (optional): Email address

**Example LLM prompt**: "Find software engineers at Google"

---

### prospeo_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: Get contact profile, data.

**Arguments**:
- `contactId` (required): Contact ID or email

**Example LLM prompt**: "Get contact info for john@example.com"

---

### prospeo_enrich_company

**What it does**: Enriches company data with additional information.

**When to use**: Get comprehensive company data.

**Arguments**:
- `domain` (optional): Company domain
- `name` (optional): Company name

**Example LLM prompt**: "Enrich data for example.com"

---

### prospeo_enrich_contact

**What it does**: Enriches contact data with additional information.

**When to use**: Get comprehensive contact data.

**Arguments**:
- `email` (required): Email address

**Example LLM prompt**: "Enrich contact for john@example.com"

---

### prospeo_get_domain_info

**What it does**: Gets information about a domain.

**When to use**: Domain research.

**Arguments**:
- `domain` (required): Domain name

**Example LLM prompt**: "Get info for example.com"

---

### prospeo_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: Browse campaigns.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### prospeo_get_campaign

**What it does**: Gets detailed information about a specific campaign.

**When to use**: Get campaign details, stats.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 12345"

---

## Prospeo API Notes

- **API Key**: Uses X-Key header for authentication
- **B2B Data**: Company and contact enrichment
- **Rate limits**: Check account for limits
