# Personio Recruiting Tools

Provider: `personio-recruiting` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Personio Recruiting API. They allow AI agents to manage candidates, job postings, applications, interviews, and offer documents. **Requires Personio Recruiting API Key authentication.**

## Authentication

**Nango API Key**:
- Uses Bearer token in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.personio.de/v1
- Requires companyId, partnerId, and appId in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `personio_recruiting_list_candidates` | List candidates | GET | /xml |
| `personio_recruiting_get_candidate` | Get candidate details | GET | /xml/{candidateId} |
| `personio_recruiting_list_jobs` | List job postings | GET | /xml/jobs |
| `personio_recruiting_get_job` | Get job details | GET | /xml/jobs/{jobId} |
| `personio_recruiting_list_applications` | List applications | GET | /xml/applications |
| `personio_recruiting_get_application` | Get application details | GET | /xml/applications/{applicationId} |
| `personio_recruiting_list_interviews` | List interviews | GET | /xml/interviews |
| `personio_recruiting_get_interview` | Get interview details | GET | /xml/interviews/{interviewId} |
| `personio_recruiting_list_offer_docs` | List offer documents | GET | /xml/offer-documents |
| `personio_recruiting_get_company` | Get company information | GET | /xml?language=en |

---

## Tool Details

### personio_recruiting_list_candidates

**What it does**: Lists all candidates in the recruiting system.

**When to use**: Browse candidate pool.

**Arguments**:
- `status` (optional): Filter by candidate status

**Example LLM prompt**: "List all active candidates"

---

### personio_recruiting_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: Get candidate profile, resume.

**Arguments**:
- `candidateId` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate 12345"

---

### personio_recruiting_list_jobs

**What it does**: Lists all job postings.

**When to use**: Browse open positions.

**Arguments**:
- `status` (optional): Filter by status (open, closed)

**Example LLM prompt**: "List all open jobs"

---

### personio_recruiting_get_job

**What it does**: Gets detailed information about a specific job posting.

**When to use**: Get job details, requirements.

**Arguments**:
- `jobId` (required): Job ID

**Example LLM prompt**: "Get details for job 67890"

---

### personio_recruiting_list_applications

**What it does**: Lists all job applications.

**When to use**: Browse application pipeline.

**Arguments**:
- `jobId` (optional): Filter by job ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List applications for job 67890"

---

### personio_recruiting_get_application

**What it does**: Gets detailed information about a specific application.

**When to use**: Get application details, status.

**Arguments**:
- `applicationId` (required): Application ID

**Example LLM prompt**: "Get details for application 11111"

---

### personio_recruiting_list_interviews

**What it does**: Lists all scheduled interviews.

**When to use**: View interview schedule.

**Arguments**:
- `applicationId` (optional): Filter by application
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show interviews for this week"

---

### personio_recruiting_get_interview

**What it does**: Gets detailed information about a specific interview.

**When to use**: Get interview details, feedback.

**Arguments**:
- `interviewId` (required): Interview ID

**Example LLM prompt**: "Get details for interview 22222"

---

### personio_recruiting_list_offer_docs

**What it does**: Lists all offer documents.

**When to use**: Browse offer letters.

**Arguments**:
- `applicationId` (optional): Filter by application

**Example LLM prompt**: "List all offer documents"

---

### personio_recruiting_get_company

**What it does**: Gets company information.

**When to use**: Get company settings.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

## Personio Recruiting API Notes

- **API Key**: Uses Bearer token for authentication
- **XML Format**: Uses XML for requests/responses
- **Rate limits**: Apply to API calls
