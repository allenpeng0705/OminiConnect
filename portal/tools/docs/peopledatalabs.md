# People Data Labs Tools

Provider: `peopledatalabs` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the People Data Labs API. They allow AI agents to search and retrieve person and company data, including professional profiles, contact information, and company details. **Requires People Data Labs API Key authentication.**

## Authentication

**Nango API Key**:
- API key passed as query parameter
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.peopledatalabs.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `peopledatalabs_search_persons` | Search for persons | GET | /v5/person/search |
| `peopledatalabs_get_person` | Get person by ID | GET | /v5/person |
| `peopledatalabs_lookup_email` | Lookup email | GET | /v5/person/lookup |
| `peopledatalabs_enrich_person` | Enrich person data | GET | /v5/person/enrich |
| `peopledatalabs_search_companies` | Search for companies | GET | /v5/company/search |
| `peopledatalabs_get_company` | Get company by ID | GET | /v5/company |
| `peopledatalabs_enrich_company` | Enrich company data | GET | /v5/company/enrich |
| `peopledatalabs_get_school` | Get school by ID | GET | /v5/school |
| `peopledatalabs_get_location` | Get location by ID | GET | /v5/location |
| `peopledatalabs_get_api_usage` | Get API usage stats | GET | /v5/api_usage |

---

## Tool Details

### peopledatalabs_search_persons

**What it does**: Searches for persons matching criteria.

**When to use**: Find people by name, title, company, skills, location.

**Arguments**:
- `query` (required): Search query string
- `title` (optional): Filter by job title
- `company` (optional): Filter by company name
- `location` (optional): Filter by location
- `skills` (optional): Filter by skills (comma-separated)

**Example LLM prompt**: "Find software engineers at Google in San Francisco"

---

### peopledatalabs_get_person

**What it does**: Gets person by their unique identifier.

**When to use**: Retrieve detailed person profile by ID.

**Arguments**:
- `personId` (required): Person ID (various ID types supported)

**Example LLM prompt**: "Get person profile for ID 12345"

---

### peopledatalabs_lookup_email

**What it does**: Looks up person by email address.

**When to use**: Find person profile from email address.

**Arguments**:
- `email` (required): Email address

**Example LLM prompt**: "Look up email john@company.com"

---

### peopledatalabs_enrich_person

**What it does**: Enriches person data with additional fields.

**When to use**: Get comprehensive person data from minimal input.

**Arguments**:
- `email` (optional): Email address
- `phone` (optional): Phone number
- `company` (optional): Company name

**Example LLM prompt**: "Enrich data for person at john@company.com"

---

### peopledatalabs_search_companies

**What it does**: Searches for companies matching criteria.

**When to use**: Find companies by name, location, industry.

**Arguments**:
- `query` (required): Search query string
- `name` (optional): Filter by company name
- `location` (optional): Filter by location

**Example LLM prompt**: "Find tech companies in San Francisco"

---

### peopledatalabs_get_company

**What it does**: Gets company by ID.

**When to use**: Retrieve detailed company profile.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get company profile for 99999"

---

### peopledatalabs_enrich_company

**What it does**: Enriches company data with additional fields.

**When to use**: Get comprehensive company data from minimal input.

**Arguments**:
- `domain` (optional): Company domain
- `name` (optional): Company name

**Example LLM prompt**: "Enrich company data for example.com"

---

### peopledatalabs_get_school

**What it does**: Gets school by ID.

**When to use**: Retrieve school/education institution details.

**Arguments**:
- `schoolId` (required): School ID

**Example LLM prompt**: "Get school details for 88888"

---

### peopledatalabs_get_location

**What it does**: Gets location by ID.

**When to use**: Retrieve location/geography details.

**Arguments**:
- `locationId` (required): Location ID

**Example LLM prompt**: "Get location details for 77777"

---

### peopledatalabs_get_api_usage

**What it does**: Gets API usage statistics.

**When to use**: Check API rate limits, usage.

**Arguments**: None

**Example LLM prompt**: "What's our API usage this month?"

---

## People Data Labs API Notes

- **API Key**: Passed as query parameter `api_key`
- **Rate limits**: Check API usage endpoint for limits
- **Data fields**: Comprehensive person and company data available
