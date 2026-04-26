# Gobi Tools

Provider: `gobi` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Gobi API. They allow AI agents to manage candidates, jobs, interviews, and offers. Gobi is an applicant tracking system (ATS) that helps recruiting teams streamline their hiring workflow.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Gobi
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `gobi.candidates.read`, `gobi.candidates.write`, `gobi.jobs.read`, `gobi.interviews.read`, `gobi.offers.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gobi_list_candidates` | List all candidates | GET | /candidates |
| `gobi_get_candidate` | Get candidate details | GET | /candidates/{candidateId} |
| `gobi_create_candidate` | Create a new candidate | POST | /candidates |
| `gobi_update_candidate` | Update a candidate | PUT | /candidates/{candidateId} |
| `gobi_list_jobs` | List all jobs | GET | /jobs |
| `gobi_get_job` | Get job details | GET | /jobs/{jobId} |
| `gobi_list_interviews` | List interviews | GET | /interviews |
| `gobi_get_interview` | Get interview details | GET | /interviews/{interviewId} |
| `gobi_list_offers` | List offers | GET | /offers |
| `gobi_get_offer` | Get offer details | GET | /offers/{offerId} |

---

## Tool Details

### gobi_list_candidates

**What it does**: Lists all candidates in Gobi with pagination.

**When to use**: Browse your candidate pool, review applicants for open positions.

**Arguments**:
- `limit` (optional): Max results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all candidates in Gobi"

---

### gobi_get_candidate

**What it does**: Gets detailed information about a specific candidate including contact info, resume, and application history.

**When to use**: Review candidate profile before scheduling interviews or making hiring decisions.

**Arguments**:
- `candidateId` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate abc123"

---

### gobi_create_candidate

**What it does**: Creates a new candidate in Gobi.

**When to use**: Add new applicants or build candidate profiles for future positions.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (required): Email address
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a candidate for Jane Doe with email jane@example.com"

---

### gobi_update_candidate

**What it does**: Updates an existing candidate's information.

**When to use**: Update contact details, modify candidate status, or add notes.

**Arguments**:
- `candidateId` (required): Candidate ID
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Update candidate abc123 with phone number 555-9876"

---

### gobi_list_jobs

**What it does**: Lists all jobs in Gobi with optional status filtering.

**When to use**: Browse open positions, find jobs to apply candidates to.

**Arguments**:
- `status` (optional): Filter by status (open, closed, draft)
- `limit` (optional): Max results

**Example LLM prompt**: "List all open jobs"

---

### gobi_get_job

**What it does**: Gets details about a specific job including description, requirements, and hiring team.

**When to use**: Review job details before sharing with candidates or creating job postings.

**Arguments**:
- `jobId` (required): Job ID

**Example LLM prompt**: "Get details for job xyz789"

---

### gobi_list_interviews

**What it does**: Lists all interviews with optional filtering by candidate, job, or status.

**When to use**: Track interview schedules, find upcoming interviews, review interview outcomes.

**Arguments**:
- `candidateId` (optional): Filter by candidate ID
- `jobId` (optional): Filter by job ID
- `status` (optional): Filter by status (scheduled, completed, cancelled)

**Example LLM prompt**: "List all scheduled interviews for job xyz789"

---

### gobi_get_interview

**What it does**: Gets details about a specific interview including schedule, participants, and feedback.

**When to use**: Review interview details before conducting or evaluating interviews.

**Arguments**:
- `interviewId` (required): Interview ID

**Example LLM prompt**: "Get details for interview int123"

---

### gobi_list_offers

**What it does**: Lists all offers with optional filtering by candidate or job.

**When to use**: Track offer status, find pending offers, review offer history.

**Arguments**:
- `candidateId` (optional): Filter by candidate ID
- `jobId` (optional): Filter by job ID
- `status` (optional): Filter by status (pending, accepted, rejected)

**Example LLM prompt**: "List all pending offers"

---

### gobi_get_offer

**What it does**: Gets details about a specific offer including salary, start date, and status.

**When to use**: Review offer details before extending to candidates.

**Arguments**:
- `offerId` (required): Offer ID

**Example LLM prompt**: "Get details for offer off456"

---

## Gobi API Notes

- **Candidates**: People applying for jobs in your organization
- **Jobs**: Open positions available for candidates
- **Interviews**: Scheduled meetings between candidates and hiring team
- **Offers**: Employment offers extended to candidates
- **Statuses**: Track progress through candidate pipeline
- **Pagination**: Use offset-based pagination for list endpoints
- **Filtering**: Combine filters to narrow down results
