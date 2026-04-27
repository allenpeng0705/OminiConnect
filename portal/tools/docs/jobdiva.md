# JobDiva Tools

Provider: `jobdiva` | Engine: `nango` | Auth: TWO_STEP via Nango

## Overview

These tools wrap the JobDiva API. They allow AI agents to manage candidates, jobs, contacts, submissions, and notes. **Requires JobDiva credentials (Client ID + username + password).**

## Authentication

**Two-Step Auth via Nango**:
- User provides Client ID, username, and password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.jobdiva.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jobdiva_list_candidates` | List candidates | GET | /apiv2/candidates |
| `jobdiva_get_candidate` | Get candidate details | GET | /apiv2/candidates/{candidate_id} |
| `jobdiva_list_jobs` | List job postings | GET | /apiv2/jobs |
| `jobdiva_get_job` | Get job details | GET | /apiv2/jobs/{job_id} |
| `jobdiva_list_contacts` | List contacts | GET | /apiv2/contacts |
| `jobdiva_get_contact` | Get contact details | GET | /apiv2/contacts/{contact_id} |
| `jobdiva_list_submissions` | List submissions | GET | /apiv2/submissions |
| `jobdiva_get_submission` | Get submission details | GET | /apiv2/submissions/{submission_id} |
| `jobdiva_list_notes` | List notes | GET | /apiv2/notes |
| `jobdiva_create_note` | Create a note | POST | /apiv2/notes |

---

## Tool Details

### jobdiva_list_candidates

**What it does**: Lists all candidates in JobDiva.

**When to use**: Find candidates, search candidate database.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all candidates in JobDiva"

---

### jobdiva_get_candidate

**What it does**: Gets details for a specific candidate.

**When to use**: Get candidate information, review candidate profile.

**Arguments**:
- `candidate_id` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate 12345"

---

### jobdiva_list_jobs

**What it does**: Lists all job postings.

**When to use**: View open positions, find job details.

**Arguments**:
- `status` (optional): Job status (open, closed, all - default: open)
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all open jobs in JobDiva"

---

### jobdiva_get_job

**What it does**: Gets details for a specific job posting.

**When to use**: Get job information, view job requirements.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job 67890"

---

### jobdiva_list_contacts

**What it does**: Lists all contacts.

**When to use**: View contacts, find contact information.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all contacts in JobDiva"

---

### jobdiva_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: Get contact information.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 222"

---

### jobdiva_list_submissions

**What it does**: Lists all candidate submissions.

**When to use**: View submissions, track candidates for jobs.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all submissions in JobDiva"

---

### jobdiva_get_submission

**What it does**: Gets details for a specific submission.

**When to use**: Get submission information.

**Arguments**:
- `submission_id` (required): Submission ID

**Example LLM prompt**: "Get details for submission 333"

---

### jobdiva_list_notes

**What it does**: Lists notes for candidates or jobs.

**When to use**: View notes, find related information.

**Arguments**:
- `candidate_id` (optional): Filter by candidate ID
- `job_id` (optional): Filter by job ID

**Example LLM prompt**: "List notes for candidate 12345"

---

### jobdiva_create_note

**What it does**: Creates a note on a candidate or job.

**When to use**: Add notes, document interactions.

**Arguments**:
- `content` (required): Note content
- `candidate_id` (optional): Candidate ID
- `job_id` (optional): Job ID

**Example LLM prompt**: "Add a note to candidate 12345 saying they are a good fit"

---

## JobDiva API Notes

- **IDs**: Numeric IDs for candidates, jobs, contacts, submissions
- **Pagination**: Use page and page_size parameters
- **Notes**: Can be associated with candidates or jobs
- **Submissions**: Track candidates submitted to jobs
