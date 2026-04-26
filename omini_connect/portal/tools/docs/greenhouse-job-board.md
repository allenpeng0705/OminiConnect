# Greenhouse Job Board Tools

Provider: `greenhouse-job-board` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Greenhouse Job Board API. They allow AI agents to browse jobs, departments, and offices on a Greenhouse job board. The Job Board API is public-facing for candidate recruitment.

## Authentication

**Nango Basic Auth**:
- User provides board token via Nango Connect
- Uses board token as username with empty password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://boards-api.greenhouse.io

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_list_board_jobs` | List board jobs | GET | /v1/boards/{boardToken}/jobs |
| `greenhouse_get_board_job` | Get board job details | GET | /v1/boards/{boardToken}/jobs/{jobId} |
| `greenhouse_list_job_questions` | List job questions | GET | /v1/boards/{boardToken}/jobs/{jobId}/questions |
| `greenhouse_get_job_question` | Get job question details | GET | /v1/boards/{boardToken}/questions/{questionId} |
| `greenhouse_list_departments` | List departments | GET | /v1/boards/{boardToken}/departments |
| `greenhouse_get_department` | Get department details | GET | /v1/boards/{boardToken}/departments/{departmentId} |
| `greenhouse_list_offices` | List offices | GET | /v1/boards/{boardToken}/offices |
| `greenhouse_get_office` | Get office details | GET | /v1/boards/{boardToken}/offices/{officeId} |
| `greenhouse_list_employees` | List employees | GET | /v1/boards/{boardToken}/employees |
| `greenhouse_get_employee` | Get employee details | GET | /v1/boards/{boardToken}/employees/{employeeId} |

---

## Tool Details

### greenhouse_list_board_jobs

**What it does**: Lists all jobs on the job board.

**When to use**: Browse available job openings.

**Arguments**:
- `boardToken` (required): Board token
- `content` (optional): Content format (`full` or `minimal`)

**Example LLM prompt**: "List all jobs on the board"

---

### greenhouse_get_board_job

**What it does**: Gets detailed information about a specific job.

**When to use**: View job description and requirements.

**Arguments**:
- `boardToken` (required): Board token
- `jobId` (required): Job ID

**Example LLM prompt**: "Get job 123 on the board"

---

### greenhouse_list_job_questions

**What it does**: Lists all questions for a job.

**When to use**: View application questions for a job.

**Arguments**:
- `boardToken` (required): Board token
- `jobId` (required): Job ID

**Example LLM prompt**: "List questions for job 123"

---

### greenhouse_get_job_question

**What it does**: Gets detailed information about a job question.

**When to use**: View question details and answer options.

**Arguments**:
- `boardToken` (required): Board token
- `questionId` (required): Question ID

**Example LLM prompt**: "Get question 456"

---

### greenhouse_list_departments

**What it does**: Lists all departments on the job board.

**When to use**: Browse departments with open positions.

**Arguments**:
- `boardToken` (required): Board token

**Example LLM prompt**: "List all departments"

---

### greenhouse_get_department

**What it does**: Gets detailed information about a department.

**When to use**: View department and its jobs.

**Arguments**:
- `boardToken` (required): Board token
- `departmentId` (required): Department ID

**Example LLM prompt**: "Get department 789"

---

### greenhouse_list_offices

**What it does**: Lists all offices on the job board.

**When to use**: Browse office locations.

**Arguments**:
- `boardToken` (required): Board token

**Example LLM prompt**: "List all offices"

---

### greenhouse_get_office

**What it does**: Gets detailed information about an office.

**When to use**: View office location and jobs.

**Arguments**:
- `boardToken` (required): Board token
- `officeId` (required): Office ID

**Example LLM prompt**: "Get office 111"

---

### greenhouse_list_employees

**What it does**: Lists all employees on the job board.

**When to use**: View employee profiles.

**Arguments**:
- `boardToken` (required): Board token

**Example LLM prompt**: "List all employees"

---

### greenhouse_get_employee

**What it does**: Gets detailed information about an employee.

**When to use**: View employee bio.

**Arguments**:
- `boardToken` (required): Board token
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get employee 222"

---

## Greenhouse Job Board API Notes

- **Base URL**: https://boards-api.greenhouse.io
- **Board Token**: Found in job board URL (e.g., `boards.company.com/{boardToken}`)
- **Content Format**: Use `minimal` for lighter response
- **Job Board**: Public-facing recruitment page
- **Departments**: Organize jobs by team/department
- **Offices**: Physical office locations
- **Employees**: Employee spotlight profiles
