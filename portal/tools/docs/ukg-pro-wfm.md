# UKG Pro WFM Tools

Provider: `ukg-pro-wfm` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

UKG Pro Workforce Management module. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with UKG Pro WFM
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ukg_list_shifts` | List scheduled shifts | GET | /wfm/v1/shifts |
| `ukg_create_shift` | Create a new shift | POST | /wfm/v1/shifts |
| `ukg_list_time_entries` | List time entries | GET | /wfm/v1/time-entries |
| `ukg_create_time_entry` | Create a time entry | POST | /wfm/v1/time-entries |
| `ukg_list_schedules` | List work schedules | GET | /wfm/v1/schedules |
| `ukg_get_schedule` | Get schedule details | GET | /wfm/v1/schedules/{id} |
| `ukg_list_availability` | List employee availability | GET | /wfm/v1/availability |
| `ukg_list_approvals` | List pending approvals | GET | /wfm/v1/approvals |
| `ukg_approve_request` | Approve a request | POST | /wfm/v1/approvals/{id}/approve |
| `ukg_reject_request` | Reject a request | POST | /wfm/v1/approvals/{id}/reject |

---

## Tool Details

### ukg_list_shifts

**What it does**: List scheduled shifts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_create_shift

**What it does**: Create a new shift

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_list_time_entries

**What it does**: List time entries

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_create_time_entry

**What it does**: Create a time entry

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_list_schedules

**What it does**: List work schedules

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_get_schedule

**What it does**: Get schedule details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_list_availability

**What it does**: List employee availability

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_list_approvals

**What it does**: List pending approvals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_approve_request

**What it does**: Approve a request

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_reject_request

**What it does**: Reject a request

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.hostname}/api`
- Docs: https://nango.dev/docs/integrations/all/ukg-pro-wfm
