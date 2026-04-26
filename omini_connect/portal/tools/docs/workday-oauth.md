# Workday (OAuth) Tools

Provider: `workday-oauth` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Workday is an enterprise cloud software for HR and finance. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Workday (OAuth)
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `workday_list_workers` | List all workers | GET | /v1/workers |
| `workday_get_worker` | Get worker details | GET | /v1/workers/{id} |
| `workday_list_jobs` | List all jobs | GET | /v1/jobs |
| `workday_list_organizations` | List all organizations | GET | /v1/organizations |
| `workday_list_positions` | List all positions | GET | /v1/positions |
| `workday_list_time_entries` | List time entries | GET | /v1/time-entries |
| `workday_submit_time_entry` | Submit a time entry | POST | /v1/time-entries |
| `workday_list_approvals` | List pending approvals | GET | /v1/approvals |
| `workday_approve_request` | Approve a request | POST | /v1/approvals/{id}/approve |
| `workday_get_business_title` | Get business title details | GET | /v1/business-titles |

---

## Tool Details

### workday_list_workers

**What it does**: List all workers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_get_worker

**What it does**: Get worker details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_jobs

**What it does**: List all jobs

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_organizations

**What it does**: List all organizations

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_positions

**What it does**: List all positions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_time_entries

**What it does**: List time entries

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_submit_time_entry

**What it does**: Submit a time entry

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_approvals

**What it does**: List pending approvals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_approve_request

**What it does**: Approve a request

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_get_business_title

**What it does**: Get business title details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.tokenDomain}/ccx/api`
- Docs: https://nango.dev/docs/integrations/all/workday-oauth
