# Manatal Tools

Provider: `manatal` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Manatal HR API. They allow AI agents to manage candidates, jobs, departments, and offices in your ATS. **Requires Manatal API token.**

## Authentication

**Nango API_KEY**:
- User provides Manatal API token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `authorization: Token ${apiKey}`
- Base URL: `https://api.manatal.com/open/v3`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `manatal_list_candidates` | List all candidates | GET | /users |
| `manatal_get_candidate` | Get candidate details | GET | /users/{userId} |
| `manatal_create_candidate` | Create a candidate | POST | /users |
| `manatal_update_candidate` | Update a candidate | PUT | /users/{userId} |
| `manatal_list_jobs` | List all job postings | GET | /jobs |
| `manatal_get_job` | Get job details | GET | /jobs/{jobId} |
| `manatal_create_job` | Create a job posting | POST | /jobs |
| `manatal_list_departments` | List departments | GET | /departments |
| `manatal_get_department` | Get department details | GET | /departments/{departmentId} |
| `manatal_list_offices` | List offices | GET | /offices |

---

## Tool Details

### manatal_list_candidates

**What it does**: Lists all candidates in Manatal ATS.

**When to use**: Browse candidates, filter by stage or job.

**Arguments**:
- `stage_id` (optional): Filter by stage ID
- `job_id` (optional): Filter by job ID
- `search` (optional): Search query
- `page_size` (optional): Results per page (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all candidates in interview stage"

---

### manatal_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: Review candidate profile, check application status.

**Arguments**:
- `userId` (required): Candidate user ID

**Example LLM prompt**: "Get details for candidate 12345"

---

### manatal_create_candidate

**What it does**: Creates a new candidate in Manatal.

**When to use**: Add new applicants, sync candidates from career pages.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address
- `phone` (optional): Phone number
- `job_id` (optional): Associated job ID

**Example LLM prompt**: "Create a candidate for John Doe with email john@example.com"

---

### manatal_update_candidate

**What it does**: Updates an existing candidate.

**When to use**: Update candidate information, move to different stage.

**Arguments**:
- `userId` (required): Candidate user ID
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Update candidate 12345 with new phone number"

---

### manatal_list_jobs

**What it does**: Lists all job postings in Manatal.

**When to use**: Browse open positions, find job IDs.

**Arguments**:
- `status` (optional): Filter by status (open, closed, draft)
- `department_id` (optional): Filter by department ID
- `office_id` (optional): Filter by office ID
- `page_size` (optional): Results per page (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open jobs in Engineering department"

---

### manatal_get_job

**What it does**: Gets details of a specific job posting.

**When to use**: Review job details, check job requirements.

**Arguments**:
- `jobId` (required): Job ID

**Example LLM prompt**: "Get details for job 12345"

---

### manatal_create_job

**What it does**: Creates a new job posting in Manatal.

**When to use**: Post new job openings.

**Arguments**:
- `title` (required): Job title
- `description` (required): Job description
- `department_id` (optional): Department ID
- `office_id` (optional): Office ID
- `status` (optional): Job status (open, closed, draft) (default open)

**Example LLM prompt**: "Create a job posting for Senior Software Engineer"

---

### manatal_list_departments

**What it does**: Lists all departments in Manatal.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### manatal_get_department

**What it does**: Gets details of a specific department.

**When to use**: Check department info, find hiring managers.

**Arguments**:
- `departmentId` (required): Department ID

**Example LLM prompt**: "Get details for department 12345"

---

### manatal_list_offices

**What it does**: Lists all offices in Manatal.

**When to use**: View office locations.

**Arguments**: None

**Example LLM prompt**: "List all offices"

---

## Manatal Notes

- **Candidates**: Job applicants in the ATS
- **Jobs**: Active job postings
- **Departments**: Organizational units
- **Offices**: Physical office locations
- **Rate limits**: Implement backoff for bulk operations
