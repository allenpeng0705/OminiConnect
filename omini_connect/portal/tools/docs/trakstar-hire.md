# Trakstar Hire Tools

Provider: `trakstar-hire` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

Trakstar Hire is an applicant tracking system for recruiting. **Requires basic auth via nango.**

## Authentication

**Nango Basic Auth**:
- User provides username/password
- Credentials stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `trakstar_list_jobs` | List all job openings | GET | /api/v1/jobs |
| `trakstar_get_job` | Get job details | GET | /api/v1/jobs/{id} |
| `trakstar_list_candidates` | List all candidates | GET | /api/v1/candidates |
| `trakstar_get_candidate` | Get candidate details | GET | /api/v1/candidates/{id} |
| `trakstar_create_candidate` | Add a new candidate | POST | /api/v1/candidates |
| `trakstar_list_interviews` | List scheduled interviews | GET | /api/v1/interviews |
| `trakstar_create_interview` | Schedule an interview | POST | /api/v1/interviews |
| `trakstar_list_offices` | List all offices | GET | /api/v1/offices |
| `trakstar_list_departments` | List all departments | GET | /api/v1/departments |
| `trakstar_add_note` | Add a note to a candidate | POST | /api/v1/notes |

---

## Tool Details

### trakstar_list_jobs

**What it does**: List all job openings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_get_job

**What it does**: Get job details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_list_candidates

**What it does**: List all candidates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_get_candidate

**What it does**: Get candidate details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_create_candidate

**What it does**: Add a new candidate

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_list_interviews

**What it does**: List scheduled interviews

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_create_interview

**What it does**: Schedule an interview

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_list_offices

**What it does**: List all offices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_list_departments

**What it does**: List all departments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### trakstar_add_note

**What it does**: Add a note to a candidate

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.recruiterbox.com`
- Docs: https://nango.dev/docs/integrations/all/trakstar-hire
