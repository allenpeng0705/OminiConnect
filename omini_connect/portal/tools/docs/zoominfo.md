# ZoomInfo Tools

Provider: `zoominfo` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

ZoomInfo is a B2B database and sales intelligence platform. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoominfo_search_contacts` | Search for contacts | POST | /contact/search |
| `zoominfo_get_contact` | Get contact details | GET | /contact/{id} |
| `zoominfo_search_companies` | Search for companies | POST | /company/search |
| `zoominfo_get_company` | Get company details | GET | /company/{id} |
| `zoominfo_enrich_contact` | Enrich contact data | POST | /enrich/contact |
| `zoominfo_enrich_company` | Enrich company data | POST | /enrich/company |
| `zoominfo_get_company_by_domain` | Get company by website domain | GET | /company/domain/{domain} |
| `zoominfo_list_technologies` | List technologies used by company | GET | /company/{id}/technologies |
| `zoominfo_get_job_change` | Get job change information | GET | /jobchange/{id} |
| `zoominfo_list_workflows` | List available workflows | GET | /workflows |

---

## Tool Details

### zoominfo_search_contacts

**What it does**: Search for contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_search_companies

**What it does**: Search for companies

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_get_company

**What it does**: Get company details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_enrich_contact

**What it does**: Enrich contact data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_enrich_company

**What it does**: Enrich company data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_get_company_by_domain

**What it does**: Get company by website domain

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_list_technologies

**What it does**: List technologies used by company

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_get_job_change

**What it does**: Get job change information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoominfo_list_workflows

**What it does**: List available workflows

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.zoominfo.com`
- Docs: https://nango.dev/docs/integrations/all/zoominfo
