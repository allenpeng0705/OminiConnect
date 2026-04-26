# JazzHR Tools

Provider: `jazzhr` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the JazzHR API. They allow AI agents to manage candidates, job postings, users, and notes. **Requires JazzHR API key.**

## Authentication

**API Key via Nango**:
- User provides their JazzHR API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.resumatorapi.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jazzhr_list_candidates` | List candidates | GET | /v1/candidates |
| `jazzhr_get_candidate` | Get candidate details | GET | /v1/candidates/{candidate_id} |
| `jazzhr_list_jobs` | List job postings | GET | /v1/jobs |
| `jazzhr_get_job` | Get job details | GET | /v1/jobs/{job_id} |
| `jazzhr_list_users` | List users | GET | /v1/users |
| `jazzhr_get_user` | Get user details | GET | /v1/users/{user_id} |
| `jazzhr_list_notes` | List notes | GET | /v1/notes |
| `jazzhr_create_note` | Create a note | POST | /v1/notes |
| `jazzhr_list_activities` | List activities | GET | /v1/activities |
| `jazzhr_list_questionnaires` | List questionnaires | GET | /v1/questionnaires |

---

## Tool Details

### jazzhr_list_candidates

**What it does**: Lists all candidates in JazzHR.

**When to use**: Find candidates, search candidate database.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all candidates in JazzHR"

---

### jazzhr_get_candidate

**What it does**: Gets details for a specific candidate.

**When to use**: Get candidate information, review candidate profile.

**Arguments**:
- `candidate_id` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate 12345"

---

### jazzhr_list_jobs

**What it does**: Lists all job postings.

**When to use**: View open positions, find job details.

**Arguments**:
- `status` (optional): Job status (open, closed, all - default: open)
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all open jobs in JazzHR"

---

### jazzhr_get_job

**What it does**: Gets details for a specific job posting.

**When to use**: Get job information, view job requirements.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job 67890"

---

### jazzhr_list_users

**What it does**: Lists all users in the organization.

**When to use**: View team members, find user information.

**Arguments**: None

**Example LLM prompt**: "List all users in our JazzHR account"

---

### jazzhr_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information, view user profile.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user 111"

---

### jazzhr_list_notes

**What it does**: Lists notes for candidates or jobs.

**When to use**: View candidate notes, find job notes.

**Arguments**:
- `candidate_id` (optional): Filter by candidate ID
- `job_id` (optional): Filter by job ID

**Example LLM prompt**: "List all notes for candidate 12345"

---

### jazzhr_create_note

**What it does**: Creates a note on a candidate or job.

**When to use**: Add notes, document interactions.

**Arguments**:
- `candidate_id` (required): Candidate ID
- `body` (required): Note content
- `job_id` (optional): Job ID

**Example LLM prompt**: "Add a note to candidate 12345 saying they are a good fit"

---

### jazzhr_list_activities

**What it does**: Lists activities for candidates or jobs.

**When to use**: View activity history, track candidate progress.

**Arguments**:
- `candidate_id` (optional): Filter by candidate ID
- `job_id` (optional): Filter by job ID

**Example LLM prompt**: "List all activities for candidate 12345"

---

### jazzhr_list_questionnaires

**What it does**: Lists all questionnaires.

**When to use**: View available questionnaires, find questionnaire IDs.

**Arguments**: None

**Example LLM prompt**: "List all questionnaires available"

---

## JazzHR API Notes

- **Pagination**: Use page and per_page parameters
- **IDs**: Numeric IDs for candidates, jobs, users
- **Notes**: Associated with candidates or jobs
- **Activities**: Track all interactions with candidates
