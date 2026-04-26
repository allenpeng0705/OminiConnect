# Gem Tools

Provider: `gem` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Gem ATS API. They allow AI agents to manage candidates, jobs, interviews, offers, sources, and analytics for recruiting. **Requires Gem API key.**

## Authentication

**Nango API_KEY**:
- User provides their Gem API key and app secret
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.gem.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gem_list_candidates` | List candidates | GET | /v0/users |
| `gem_get_candidate` | Get candidate details | GET | /v0/users/{user_id} |
| `gem_create_candidate` | Create a candidate | POST | /v0/users |
| `gem_update_candidate` | Update a candidate | PUT | /v0/users/{user_id} |
| `gem_list_jobs` | List jobs | GET | /v0/jobs |
| `gem_get_job` | Get job details | GET | /v0/jobs/{job_id} |
| `gem_list_interviews` | List interviews | GET | /v0/interviews |
| `gem_list_offers` | List offers | GET | /v0/offers |
| `gem_list_sources` | List candidate sources | GET | /v0/sources |
| `gem_list_analytics` | List analytics data | GET | /v0/analytics |

---

## Tool Details

### gem_list_candidates

**What it does**: Lists all candidates in the ATS.

**When to use**: Browse candidate pipeline, filter by job or stage.

**Arguments**:
- `job_id` (optional): Filter by job ID
- `stage` (optional): Filter by stage
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all candidates in the engineering pipeline"

---

### gem_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: Get candidate profile, resume, activity.

**Arguments**:
- `user_id` (required): Candidate user ID

**Example LLM prompt**: "Get details for candidate abc123"

---

### gem_create_candidate

**What it does**: Creates a new candidate in the ATS.

**When to use**: Add candidates to the recruiting pipeline.

**Arguments**:
- `name` (required): Candidate name
- `email` (required): Candidate email
- `job_id` (optional): Associated job ID

**Example LLM prompt**: "Create a new candidate named John Doe with email john@email.com"

---

### gem_update_candidate

**What it does**: Updates a candidate's information.

**When to use**: Move candidates between stages, update details.

**Arguments**:
- `user_id` (required): Candidate user ID
- `name` (optional): New name
- `stage` (optional): New stage

**Example LLM prompt**: "Move candidate abc123 to interview stage"

---

### gem_list_jobs

**What it does**: Lists all jobs in the ATS.

**When to use**: Browse open positions.

**Arguments**:
- `status` (optional): Filter by status (open, closed)

**Example LLM prompt**: "List all open jobs"

---

### gem_get_job

**What it does**: Gets detailed information about a specific job.

**When to use**: Get job details, requirements, candidate count.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job engineering-001"

---

### gem_list_interviews

**What it does**: Lists all scheduled interviews.

**When to use**: Track interview schedule.

**Arguments**:
- `job_id` (optional): Filter by job ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all interviews for this week"

---

### gem_list_offers

**What it does**: Lists all offers in the system.

**When to use**: Track offer status and history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending offers"

---

### gem_list_sources

**What it does**: Lists all candidate sources (referrals, job boards, etc).

**When to use**: Analyze where candidates come from.

**Arguments**: None

**Example LLM prompt**: "List all candidate sources"

---

### gem_list_analytics

**What it does**: Lists analytics and metrics data.

**When to use**: Get recruiting metrics and reports.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show recruiting analytics for Q1"

---

## Gem API Notes

- **ATS focus**: Applicant Tracking System for recruiting
- **Candidate stages**: Application, Screening, Interview, Offer, Hired
- **Sources**: Referral, Job Board, LinkedIn, etc.
- **Analytics**: Time-to-hire, source effectiveness
