# Recruiterflow Tools

Provider: `recruiterflow` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Recruiterflow is an applicant tracking system (ATS) for recruitment agencies. These tools allow AI agents to manage jobs, candidates, interviews, tasks, and hiring pipelines.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Recruiterflow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `jobs:read`, `candidates:read`, `candidates:write`, `interviews:read`, `interviews:write`, `tasks:read`, `tasks:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `recruiterflow_list_jobs` | List all job openings | GET | /jobs |
| `recruiterflow_get_job` | Get job details | GET | /jobs/{jobId} |
| `recruiterflow_list_candidates` | List all candidates | GET | /candidates |
| `recruiterflow_get_candidate` | Get candidate details | GET | /candidates/{candidateId} |
| `recruiterflow_add_candidate` | Add a candidate to a job | POST | /jobs/{jobId}/candidates |
| `recruiterflow_list_interviews` | List all interviews | GET | /interviews |
| `recruiterflow_schedule_interview` | Schedule an interview | POST | /interviews |
| `recruiterflow_list_tasks` | List all tasks | GET | /tasks |
| `recruiterflow_create_task` | Create a task | POST | /tasks |
| `recruiterflow_get_pipeline` | Get pipeline for a job | GET | /jobs/{jobId}/pipeline |

---

## Tool Details

### recruiterflow_list_jobs

**What it does**: Returns a list of all job openings.

**When to use**: View open positions, track job status.

**Arguments**:
- `limit` (optional): Number of jobs (default 50)
- `status` (optional): Filter by status (open, closed, draft)

**Example LLM prompt**: "List all open jobs"

---

### recruiterflow_get_job

**What it does**: Gets details of a specific job.

**When to use**: Get job description, requirements, and status.

**Arguments**:
- `jobId` (required): The job ID

**Example LLM prompt**: "Get details for job jb_abc123"

---

### recruiterflow_list_candidates

**What it does**: Returns a list of candidates.

**When to use**: Browse candidates, filter by job.

**Arguments**:
- `limit` (optional): Number of candidates (default 50)
- `jobId` (optional): Filter by job

**Example LLM prompt**: "List all candidates for job jb_abc123"

---

### recruiterflow_get_candidate

**What it does**: Gets details of a specific candidate.

**When to use**: View candidate profile and history.

**Arguments**:
- `candidateId` (required): The candidate ID

**Example LLM prompt**: "Get details for candidate cnd_xyz789"

---

### recruiterflow_add_candidate

**What it does**: Adds a candidate to a job's hiring pipeline.

**When to use**: Start tracking a candidate for a specific role.

**Arguments**:
- `jobId` (required): The job ID
- `candidateId` (required): The candidate ID
- `stage` (optional): Initial pipeline stage

**Example LLM prompt**: "Add candidate cnd_123 to job jb_456"

---

### recruiterflow_list_interviews

**What it does**: Returns a list of all interviews.

**When to use**: View interview schedule.

**Arguments**:
- `limit` (optional): Number of interviews (default 50)

**Example LLM prompt**: "List all upcoming interviews"

---

### recruiterflow_schedule_interview

**What it does**: Schedules a new interview.

**When to use**: Book interview time with candidate.

**Arguments**:
- `candidateId` (required): Candidate ID
- `jobId` (required): Job ID
- `scheduledTime` (required): Interview time (ISO 8601)
- `interviewType` (optional): Type (phone, video, in-person)

**Example LLM prompt**: "Schedule a video interview with candidate cnd_123 for job jb_456"

---

### recruiterflow_list_tasks

**What it does**: Returns a list of all tasks.

**When to use**: View pending to-dos and follow-ups.

**Arguments**:
- `limit` (optional): Number of tasks (default 50)
- `completed` (optional): Filter by completion status

**Example LLM prompt**: "List all incomplete tasks"

---

### recruiterflow_create_task

**What it does**: Creates a new task.

**When to use**: Create reminders and follow-ups.

**Arguments**:
- `title` (required): Task title
- `description` (optional): Task description
- `candidateId` (optional): Related candidate
- `dueDate` (optional): Due date (ISO 8601)

**Example LLM prompt**: "Create a task to follow up with candidate cnd_123"

---

### recruiterflow_get_pipeline

**What it does**: Gets the hiring pipeline stages for a job.

**When to use**: See candidate progression through hiring stages.

**Arguments**:
- `jobId` (required): The job ID

**Example LLM prompt**: "Get pipeline for job jb_abc123"

---

## Recruiterflow Notes

- Jobs represent open positions being recruited for
- Candidates are tracked through pipeline stages
- Pipelines show hiring progress from application to hire
- Tasks help track follow-ups and action items
