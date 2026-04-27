# Jobvite Tools

Provider: `jobvite` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Jobvite API. They allow AI agents to manage candidates, jobs, offers, departments, and locations. **Requires Jobvite API key and secret key.**

## Authentication

**API Key via Nango**:
- User provides API key and secret key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.jobvite.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jobvite_list_candidates` | List candidates | GET | /v2/candidate |
| `jobvite_get_candidate` | Get candidate details | GET | /v2/candidate/{candidate_id} |
| `jobvite_list_jobs` | List job postings | GET | /v2/job |
| `jobvite_get_job` | Get job details | GET | /v2/job/{job_id} |
| `jobvite_list_offers` | List offers | GET | /v2/offer |
| `jobvite_get_offer` | Get offer details | GET | /v2/offer/{offer_id} |
| `jobvite_list_departments` | List departments | GET | /v2/department |
| `jobvite_list_locations` | List locations | GET | /v2/location |
| `jobvite_create_candidate` | Create a candidate | POST | /v2/candidate |
| `jobvite_create_job` | Create a job | POST | /v2/job |

---

## Tool Details

### jobvite_list_candidates

**What it does**: Lists all candidates in Jobvite.

**When to use**: Find candidates, search candidate database.

**Arguments**:
- `status` (optional): Candidate status filter
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all candidates in Jobvite"

---

### jobvite_get_candidate

**What it does**: Gets details for a specific candidate.

**When to use**: Get candidate information, review candidate profile.

**Arguments**:
- `candidate_id` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate abc123"

---

### jobvite_list_jobs

**What it does**: Lists all job postings.

**When to use**: View open positions, find job details.

**Arguments**:
- `status` (optional): Job status (open, closed, all - default: open)
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all open jobs in Jobvite"

---

### jobvite_get_job

**What it does**: Gets details for a specific job posting.

**When to use**: Get job information, view job requirements.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job xyz789"

---

### jobvite_list_offers

**What it does**: Lists all offers.

**When to use**: View offers, track hiring decisions.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all offers in Jobvite"

---

### jobvite_get_offer

**What it does**: Gets details for a specific offer.

**When to use**: Get offer information.

**Arguments**:
- `offer_id` (required): Offer ID

**Example LLM prompt**: "Get details for offer o1"

---

### jobvite_list_departments

**What it does**: Lists all departments.

**When to use**: View departments, organize jobs.

**Arguments**: None

**Example LLM prompt**: "List all departments in Jobvite"

---

### jobvite_list_locations

**What it does**: Lists all locations.

**When to use**: View locations, organize jobs.

**Arguments**: None

**Example LLM prompt**: "List all locations in Jobvite"

---

### jobvite_create_candidate

**What it does**: Creates a new candidate.

**When to use**: Add candidates to Jobvite.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address

**Example LLM prompt**: "Create a candidate named John Doe with email john@example.com"

---

### jobvite_create_job

**What it does**: Creates a new job posting.

**When to use**: Post new jobs.

**Arguments**:
- `title` (required): Job title
- `department_id` (optional): Department ID
- `location_id` (optional): Location ID
- `description` (optional): Job description

**Example LLM prompt**: "Create a job titled Software Engineer"

---

## Jobvite API Notes

- **IDs**: String IDs for candidates, jobs, offers
- **Authentication**: Uses both API key and secret key headers
- **Status filtering**: Filter candidates and jobs by status
