# Zoho Recruit Tools

Provider: `zoho-recruit` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Recruit is an applicant tracking system. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Recruit
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_recruit_list_candidates` | List all candidates | GET | /api/v2/candidates |
| `zoho_recruit_get_candidate` | Get candidate details | GET | /api/v2/candidates/{id} |
| `zoho_recruit_create_candidate` | Create a new candidate | POST | /api/v2/candidates |
| `zoho_recruit_update_candidate` | Update candidate details | PUT | /api/v2/candidates/{id} |
| `zoho_recruit_list_jobs` | List all job openings | GET | /api/v2/jobopenings |
| `zoho_recruit_get_job` | Get job details | GET | /api/v2/jobopenings/{id} |
| `zoho_recruit_create_job` | Create a new job opening | POST | /api/v2/jobopenings |
| `zoho_recruit_list_interviews` | List all interviews | GET | /api/v2/interviews |
| `zoho_recruit_create_interview` | Schedule an interview | POST | /api/v2/interviews |
| `zoho_recruit_list_clients` | List all clients | GET | /api/v2/clients |

---

## Tool Details

### zoho_recruit_list_candidates

**What it does**: List all candidates

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_get_candidate

**What it does**: Get candidate details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_create_candidate

**What it does**: Create a new candidate

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_update_candidate

**What it does**: Update candidate details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_list_jobs

**What it does**: List all job openings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_get_job

**What it does**: Get job details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_create_job

**What it does**: Create a new job opening

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_list_interviews

**What it does**: List all interviews

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_create_interview

**What it does**: Schedule an interview

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_recruit_list_clients

**What it does**: List all clients

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://recruit.zoho.${connectionConfig.extension}/recruit`
- Docs: https://nango.dev/docs/integrations/all/zoho-recruit
