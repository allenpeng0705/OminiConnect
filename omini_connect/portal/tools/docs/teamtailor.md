# Teamtailor Tools

Provider: `teamtailor` | Engine: `nango` | Auth: API Key via Nango

## Overview

Teamtailor is an applicant tracking system (ATS) for recruiting. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Teamtailor API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `teamtailor_list_jobs` | List all job postings | GET | /jobs |
| `teamtailor_get_job` | Get details of a job posting | GET | /jobs/{job_id} |
| `teamtailor_list_candidates` | List all candidates | GET | /candidates |
| `teamtailor_get_candidate` | Get candidate details | GET | /candidates/{candidate_id} |
| `teamtailor_list_departments` | List all departments | GET | /departments |
| `teamtailor_list_offices` | List all offices | GET | /offices |
| `teamtailor_list_users` | List all users | GET | /users |
| `teamtailor_list_questions` | List all questions for job applications | GET | /questions |
| `teamtailor_list_interviews` | List all scheduled interviews | GET | /interviews |
| `teamtailor_create_candidate` | Create a new candidate | POST | /candidates |

---

## Tool Details

### teamtailor_list_jobs

**What it does**: List all job postings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_get_job

**What it does**: Get details of a job posting

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_list_candidates

**What it does**: List all candidates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_get_candidate

**What it does**: Get candidate details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_list_departments

**What it does**: List all departments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_list_offices

**What it does**: List all offices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_list_users

**What it does**: List all users

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_list_questions

**What it does**: List all questions for job applications

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_list_interviews

**What it does**: List all scheduled interviews

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### teamtailor_create_candidate

**What it does**: Create a new candidate

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.extension}.teamtailor.com`
- Docs: https://nango.dev/docs/integrations/all/teamtailor
