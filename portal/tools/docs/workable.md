# Workable Tools

Provider: `workable` | Engine: `nango` | Auth: API Key via Nango

## Overview

Workable is an applicant tracking system for recruiting. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Workable API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `workable_list_jobs` | List all job postings | GET | /spi/v3/jobs |
| `workable_get_job` | Get job details | GET | /spi/v3/jobs/{shortcode} |
| `workable_list_candidates` | List all candidates | GET | /spi/v3/candidates |
| `workable_get_candidate` | Get candidate details | GET | /spi/v3/candidates/{id} |
| `workable_create_candidate` | Create a new candidate | POST | /spi/v3/candidates |
| `workable_list_applications` | List job applications | GET | /spi/v3/applications |
| `workable_get_application` | Get application details | GET | /spi/v3/applications/{id} |
| `workable_list_offices` | List all offices | GET | /spi/v3/offices |
| `workable_list_departments` | List all departments | GET | /spi/v3/departments |
| `workable_list_interviews` | List scheduled interviews | GET | /spi/v3/interviews |

---

## Tool Details

### workable_list_jobs

**What it does**: List all job postings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_get_job

**What it does**: Get job details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_list_candidates

**What it does**: List all candidates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_get_candidate

**What it does**: Get candidate details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_create_candidate

**What it does**: Create a new candidate

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_list_applications

**What it does**: List job applications

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_get_application

**What it does**: Get application details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_list_offices

**What it does**: List all offices

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_list_departments

**What it does**: List all departments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workable_list_interviews

**What it does**: List scheduled interviews

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.subdomain}.workable.com`
- Docs: https://nango.dev/docs/integrations/all/workable
