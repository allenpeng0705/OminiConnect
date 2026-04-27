# Greenhouse Basic Tools

Provider: `greenhouse-basic` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Greenhouse API. They allow AI agents to manage candidates, jobs, applications, interviews, and offers. Greenhouse is an applicant tracking system (ATS) for recruiting.

## Authentication

**Nango Basic Auth**:
- User provides API key and resource domain via Nango Connect
- Uses API key as username with empty password
- Token stored in Nango, accessed via `connection_ref`
- Resource configured via connection config (e.g., `api.greenhouse.io`)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_list_candidates` | List candidates | GET | /candidates |
| `greenhouse_get_candidate` | Get candidate details | GET | /candidates/{id} |
| `greenhouse_list_jobs` | List jobs | GET | /jobs |
| `greenhouse_get_job` | Get job details | GET | /jobs/{id} |
| `greenhouse_list_applications` | List applications | GET | /applications |
| `greenhouse_get_application` | Get application details | GET | /applications/{id} |
| `greenhouse_list_interviews` | List interviews | GET | /scheduled_interviews |
| `greenhouse_get_interview` | Get interview details | GET | /scheduled_interviews/{id} |
| `greenhouse_list_offers` | List offers | GET | /offers |
| `greenhouse_list_scorecards` | List scorecards | GET | /scorecards |

---

## Tool Details

### greenhouse_list_candidates

**What it does**: Lists all candidates in Greenhouse.

**When to use**: Browse candidate pool, search for specific candidates.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all candidates"

---

### greenhouse_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: View candidate profile, application history, and activity.

**Arguments**:
- `id` (required): Candidate ID

**Example LLM prompt**: "Get candidate with ID 12345"

---

### greenhouse_list_jobs

**What it does**: Lists all jobs in Greenhouse.

**When to use**: Browse job openings, find specific positions.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all open jobs"

---

### greenhouse_get_job

**What it does**: Gets detailed information about a specific job.

**When to use**: View job description, hiring team, and pipeline stages.

**Arguments**:
- `id` (required): Job ID

**Example LLM prompt**: "Get job with ID 789"

---

### greenhouse_list_applications

**What it does**: Lists all applications in Greenhouse.

**When to use**: View applications by job or candidate.

**Arguments**:
- `job_id` (optional): Filter by job ID
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

### greenhouse_list_interviews

**What it does**: Lists all scheduled interviews.

**When to use**: View upcoming interviews, check schedule.

**Arguments**:
- `job_id` (optional): Filter by job ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all interviews for job 789"

---

### greenhouse_get_interview

**What it does**: Gets detailed information about a specific interview.

**When to use**: View interview details and feedback.

**Arguments**:
- `id` (required): Interview ID

**Example LLM prompt**: "Get interview with ID 111"

---

### greenhouse_list_offers

**What it does**: Lists all job offers in Greenhouse.

**When to use**: View pending and extended offers.

**Arguments**: None

**Example LLM prompt**: "List all offers"

---

### greenhouse_list_scorecards

**What it does**: Lists all scorecards for applications.

**When to use**: View interviewer feedback and ratings.

**Arguments**:
- `application_id` (optional): Filter by application ID

**Example LLM prompt**: "List scorecards for application 456"

---

## Greenhouse API Notes

- **Base URL**: https://{resource}.greenhouse.io
- **Resource**: Configured via connection config (e.g., `api`)
- **Auth Mode**: Basic Auth with API key as username
- **Pagination**: Uses link header for pagination
- **Candidates**: Core person entity in the recruiting system
- **Applications**: Link between candidates and jobs
- **Interviews**: Scheduled interviews with scorecards
- **Offers**: Employment offers extended to candidates
