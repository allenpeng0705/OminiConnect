# Recruitcrm Tools

Provider: `recruitcrm` | Engine: `nango` | Auth: OAuth via Nango

## Overview

RecruitCRM is an applicant tracking and recruitment management system. These tools allow AI agents to manage candidates, jobs, clients, and interviews in your recruitment pipeline.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with RecruitCRM
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `candidates:read`, `candidates:write`, `jobs:read`, `jobs:write`, `clients:read`, `interviews:read`, `interviews:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `recruitcrm_list_candidates` | List all candidates | GET | /candidates |
| `recruitcrm_get_candidate` | Get candidate details | GET | /candidates/{candidateId} |
| `recruitcrm_create_candidate` | Create a candidate | POST | /candidates |
| `recruitcrm_list_jobs` | List all job openings | GET | /jobs |
| `recruitcrm_get_job` | Get job details | GET | /jobs/{jobId} |
| `recruitcrm_create_job` | Create a job opening | POST | /jobs |
| `recruitcrm_list_clients` | List all clients | GET | /clients |
| `recruitcrm_get_client` | Get client details | GET | /clients/{clientId} |
| `recruitcrm_list_interviews` | List interviews | GET | /interviews |
| `recruitcrm_create_interview` | Schedule an interview | POST | /interviews |

---

## Tool Details

### recruitcrm_list_candidates

**What it does**: Returns a list of all candidates in your pipeline.

**When to use**: Browse candidates, search by status.

**Arguments**:
- `limit` (optional): Number of candidates (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active candidates"

---

### recruitcrm_get_candidate

**What it does**: Gets details of a specific candidate.

**When to use**: View candidate profile, skills, and history.

**Arguments**:
- `candidateId` (required): The candidate ID

**Example LLM prompt**: "Get details for candidate cnd_abc123"

---

### recruitcrm_create_candidate

**What it does**: Creates a new candidate record.

**When to use**: Add new applicants to your pipeline.

**Arguments**:
- `name` (required): Candidate name
- `email` (required): Email address
- `phone` (optional): Phone number
- `currentPosition` (optional): Current job title
- `skills` (optional): Array of skills

**Example LLM prompt**: "Create a candidate for John Smith with skills Python and SQL"

---

### recruitcrm_list_jobs

**What it does**: Returns a list of all job openings.

**When to use**: View open positions, track hiring needs.

**Arguments**:
- `limit` (optional): Number of jobs (default 50)
- `status` (optional): Filter by status (open, closed, on-hold)

**Example LLM prompt**: "List all open jobs"

---

### recruitcrm_get_job

**What it does**: Gets details of a specific job.

**When to use**: Get job requirements, description, and status.

**Arguments**:
- `jobId` (required): The job ID

**Example LLM prompt**: "Get details for job jb_xyz789"

---

### recruitcrm_create_job

**What it does**: Creates a new job opening.

**When to use**: Post new positions for recruitment.

**Arguments**:
- `title` (required): Job title
- `description` (required): Job description
- `location` (optional): Job location
- `requirements` (optional): Array of requirements

**Example LLM prompt**: "Create a job for 'Senior Developer' with description 'We need a Python expert'"

---

### recruitcrm_list_clients

**What it does**: Returns a list of all clients.

**When to use**: View your client roster.

**Arguments**:
- `limit` (optional): Number of clients (default 50)

**Example LLM prompt**: "List all clients"

---

### recruitcrm_get_client

**What it does**: Gets details of a specific client.

**When to use**: Get client company information and contacts.

**Arguments**:
- `clientId` (required): The client ID

**Example LLM prompt**: "Get details for client cli_123"

---

### recruitcrm_list_interviews

**What it does**: Returns a list of scheduled interviews.

**When to use**: View interview schedule, track upcoming interviews.

**Arguments**:
- `limit` (optional): Number of interviews (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all upcoming interviews"

---

### recruitcrm_create_interview

**What it does**: Schedules a new interview.

**When to use**: Book candidate interviews.

**Arguments**:
- `candidateId` (required): Candidate ID
- `jobId` (required): Job ID
- `scheduledTime` (required): Interview time (ISO 8601)
- `interviewType` (optional): Type (phone, video, onsite)

**Example LLM prompt**: "Schedule a video interview with candidate cnd_123 for job jb_456"

---

## RecruitCRM Notes

- Candidates move through recruitment pipeline stages
- Jobs represent open positions at client companies
- Clients are companies you recruit for
- Interviews link candidates to jobs with scheduling info
