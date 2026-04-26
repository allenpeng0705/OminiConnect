# Wiza Tools

Provider: `wiza` | Engine: `nango` | Auth: API Key via Nango

## Overview

Wiza is a sales intelligence platform for enriching B2B data. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Wiza API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wiza_list_people` | List all people/contacts | GET | /v1/people |
| `wiza_get_person` | Get person details | GET | /v1/people/{id} |
| `wiza_enrich_person` | Enrich person data | POST | /v1/enrich |
| `wiza_list_companies` | List all companies | GET | /v1/companies |
| `wiza_get_company` | Get company details | GET | /v1/companies/{id} |
| `wiza_enrich_company` | Enrich company data | POST | /v1/companies/enrich |
| `wiza_search_people` | Search for people | GET | /v1/search/people |
| `wiza_lookup_email` | Lookup email by person data | POST | /v1/lookup |
| `wiza_list_verifications` | List email verifications | GET | /v1/verifications |
| `wiza_verify_email` | Verify an email address | POST | /v1/verify |

---

## Tool Details

### wiza_list_people

**What it does**: List all people/contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_get_person

**What it does**: Get person details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_enrich_person

**What it does**: Enrich person data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_list_companies

**What it does**: List all companies

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_get_company

**What it does**: Get company details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_enrich_company

**What it does**: Enrich company data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_search_people

**What it does**: Search for people

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_lookup_email

**What it does**: Lookup email by person data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_list_verifications

**What it does**: List email verifications

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wiza_verify_email

**What it does**: Verify an email address

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://wiza.co/api`
- Docs: https://nango.dev/docs/integrations/all/wiza
