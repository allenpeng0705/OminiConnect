# Greenhouse Assessment Tools

Provider: `greenhouse-assessment` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Greenhouse Assessment API. They allow AI agents to manage candidates, applications, jobs, and offers. Greenhouse is an applicant tracking system (ATS) for recruiting.

## Authentication

**Nango Basic Auth**:
- User provides access key via Nango Connect
- Uses API key as username with empty password
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_list_tests` | List available tests | GET | /list_tests |
| `greenhouse_get_test` | Get test details | GET | /get_test |
| `greenhouse_list_candidates` | List candidates | GET | /candidates |
| `greenhouse_get_candidate` | Get candidate details | GET | /candidates/{id} |
| `greenhouse_list_applications` | List applications | GET | /applications |
| `greenhouse_get_application` | Get application details | GET | /applications/{id} |
| `greenhouse_list_jobs` | List jobs | GET | /jobs |
| `greenhouse_get_job` | Get job details | GET | /jobs/{id} |
| `greenhouse_list_offers` | List offers | GET | /offers |
| `greenhouse_get_offer` | Get offer details | GET | /offers/{id} |

---

## Tool Details

### greenhouse_list_tests

**What it does**: Lists all available assessment tests.

**When to use**: View available assessments for candidates.

**Arguments**: None

**Example LLM prompt**: "List all available tests"

---

### greenhouse_get_test

**What it does**: Gets detailed information about a specific test.

**When to use**: View test structure and questions.

**Arguments**:
- `test_id` (required): Test ID

**Example LLM prompt**: "Get test with ID abc123"

---

### greenhouse_list_candidates

**What it does**: Lists all candidates in Greenhouse.

**When to use**: Browse candidate pool, find specific candidates.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all candidates"

---

### greenhouse_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: View candidate profile and application history.

**Arguments**:
- `id` (required): Candidate ID

**Example LLM prompt**: "Get candidate with ID 12345"

---

### greenhouse_list_applications

**What it does**: Lists all job applications.

**When to use**: View applications by candidate or job.

**Arguments**:
- `candidate_id` (optional): Filter by candidate ID
- `job_id` (optional): Filter by job ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all applications for job 123"

---

### greenhouse_get_application

**What it does**: Gets detailed information about a specific application.

**When to use**: View application details and scorecards.

**Arguments**:
- `id` (required): Application ID

**Example LLM prompt**: "Get application with ID 456"

---

### greenhouse_list_jobs

**What it does**: Lists all open jobs.

**When to use**: Browse job openings, find specific positions.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all open jobs"

---

### greenhouse_get_job

**What it does**: Gets detailed information about a specific job.

**When to use**: View job details and hiring pipeline.

**Arguments**:
- `id` (required): Job ID

**Example LLM prompt**: "Get job with ID 789"

---

### greenhouse_list_offers

**What it does**: Lists all job offers.

**When to use**: View pending and extended offers.

**Arguments**:
- `application_id` (optional): Filter by application ID

**Example LLM prompt**: "List all offers"

---

### greenhouse_get_offer

**What it does**: Gets detailed information about a specific offer.

**When to use**: View offer terms and approval status.

**Arguments**:
- `id` (required): Offer ID

**Example LLM prompt**: "Get offer with ID 111"

---

## Greenhouse Assessment API Notes

- **Base URL**: Configured via connection config (baseUrl)
- **Auth Mode**: Basic Auth with API key as username
- **Candidates**: Core person entity in the recruiting system
- **Applications**: Link between candidates and jobs
- **Jobs**: Open positions being hired for
- **Offers**: Employment offers extended to candidates
