# Greenhouse Harvest Partner Tools

Provider: `greenhouse-harvest-partner` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Greenhouse Harvest Partner API. They allow AI agents to manage candidates, jobs, applications, and offers using OAuth2 authorization code flow.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Greenhouse
- Uses authorization code flow with PKCE disabled
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://auth.greenhouse.io/authorize
- Token URL: https://auth.greenhouse.io/token
- Scope separator: space

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_list_candidates` | List candidates | GET | /v1/candidates |
| `greenhouse_get_candidate` | Get candidate details | GET | /v1/candidates/{id} |
| `greenhouse_list_jobs` | List jobs | GET | /v1/jobs |
| `greenhouse_get_job` | Get job details | GET | /v1/jobs/{id} |
| `greenhouse_list_applications` | List applications | GET | /v1/applications |
| `greenhouse_get_application` | Get application details | GET | /v1/applications/{id} |
| `greenhouse_list_offers` | List offers | GET | /v1/offers |
| `greenhouse_create_offer` | Create an offer | POST | /v1/offers |
| `greenhouse_list_rejection_reasons` | List rejection reasons | GET | /v1/rejection_reasons |
| `greenhouse_list_departments` | List departments | GET | /v1/departments |

---

## Tool Details

### greenhouse_list_candidates

**What it does**: Lists all candidates in Greenhouse Harvest.

**When to use**: Browse candidate pool, search for specific candidates.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all candidates"

---

### greenhouse_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: View candidate profile, application history, and tags.

**Arguments**:
- `id` (required): Candidate ID

**Example LLM prompt**: "Get candidate with ID 12345"

---

### greenhouse_list_jobs

**What it does**: Lists all jobs in Greenhouse Harvest.

**When to use**: Browse job openings, find specific positions.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all open jobs"

---

### greenhouse_get_job

**What it does**: Gets detailed information about a specific job.

**When to use**: View job description, hiring team, and pipeline.

**Arguments**:
- `id` (required): Job ID

**Example LLM prompt**: "Get job with ID 789"

---

### greenhouse_list_applications

**What it does**: Lists all applications in Greenhouse Harvest.

**When to use**: View applications by job or candidate.

**Arguments**:
- `job_id` (optional): Filter by job ID
- `candidate_id` (optional): Filter by candidate ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all applications for job 789"

---

### greenhouse_get_application

**What it does**: Gets detailed information about a specific application.

**When to use**: View application details, stage, and scorecards.

**Arguments**:
- `id` (required): Application ID

**Example LLM prompt**: "Get application with ID 456"

---

### greenhouse_list_offers

**What it does**: Lists all offers in Greenhouse Harvest.

**When to use**: View pending and extended offers.

**Arguments**: None

**Example LLM prompt**: "List all offers"

---

### greenhouse_create_offer

**What it does**: Creates a new offer for an application.

**When to use**: Extend an offer to a candidate.

**Arguments**:
- `application_id` (required): Application ID
- `job_id` (optional): Job ID
- `candidate_id` (optional): Candidate ID

**Example LLM prompt**: "Create an offer for application 456"

---

### greenhouse_list_rejection_reasons

**What it does**: Lists all rejection reasons configured in Greenhouse.

**When to use**: View available reasons for rejecting candidates.

**Arguments**: None

**Example LLM prompt**: "List all rejection reasons"

---

### greenhouse_list_departments

**What it does**: Lists all departments in Greenhouse Harvest.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

## Greenhouse Harvest Partner API Notes

- **Base URL**: https://harvest.greenhouse.io
- **Auth Mode**: OAuth2 Authorization Code
- **Scope Format**: `harvest:{resource}:{action}`
- **Scope Separator**: space
- **Scopes**: Must be requested during OAuth app registration
- **Refresh Tokens**: Supported for token renewal
- **Rate Limits**: Check x-ratelimit-reset header for backoff
