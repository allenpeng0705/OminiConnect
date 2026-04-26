# Jobadder Tools

Provider: `jobadder` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Jobadder API. They allow AI agents to manage candidates, jobs, users, contacts, and placements. **Requires Jobadder OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Jobadder
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.jobadder.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jobadder_list_candidates` | List candidates | GET | /v2/candidates |
| `jobadder_get_candidate` | Get candidate details | GET | /v2/candidates/{candidate_id} |
| `jobadder_list_jobs` | List job postings | GET | /v2/jobs |
| `jobadder_get_job` | Get job details | GET | /v2/jobs/{job_id} |
| `jobadder_list_users` | List users | GET | /v2/users |
| `jobadder_get_user` | Get user details | GET | /v2/users/{user_id} |
| `jobadder_list_contacts` | List contacts | GET | /v2/contacts |
| `jobadder_get_contact` | Get contact details | GET | /v2/contacts/{contact_id} |
| `jobadder_list_placements` | List placements | GET | /v2/placements |
| `jobadder_get_placement` | Get placement details | GET | /v2/placements/{placement_id} |

---

## Tool Details

### jobadder_list_candidates

**What it does**: Lists all candidates in Jobadder.

**When to use**: Find candidates, search candidate database.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `pageSize` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all candidates in Jobadder"

---

### jobadder_get_candidate

**What it does**: Gets details for a specific candidate.

**When to use**: Get candidate information, review candidate profile.

**Arguments**:
- `candidate_id` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate 12345"

---

### jobadder_list_jobs

**What it does**: Lists all job postings.

**When to use**: View open positions, find job details.

**Arguments**:
- `status` (optional): Job status (active, closed, all - default: active)
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all active jobs in Jobadder"

---

### jobadder_get_job

**What it does**: Gets details for a specific job posting.

**When to use**: Get job information, view job requirements.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job 67890"

---

### jobadder_list_users

**What it does**: Lists all users in the organization.

**When to use**: View team members, find user information.

**Arguments**: None

**Example LLM prompt**: "List all users in our Jobadder account"

---

### jobadder_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information, view user profile.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user 111"

---

### jobadder_list_contacts

**What it does**: Lists all contacts.

**When to use**: View contacts, find contact information.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all contacts in Jobadder"

---

### jobadder_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: Get contact information.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 222"

---

### jobadder_list_placements

**What it does**: Lists all placements.

**When to use**: View placements, track hires.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all placements"

---

### jobadder_get_placement

**What it does**: Gets details for a specific placement.

**When to use**: Get placement information.

**Arguments**:
- `placement_id` (required): Placement ID

**Example LLM prompt**: "Get details for placement 333"

---

## Jobadder API Notes

- **IDs**: Numeric IDs for candidates, jobs, users, contacts, placements
- **Pagination**: Use page and pageSize parameters
- **Status filtering**: Filter jobs by status (active, closed, all)
