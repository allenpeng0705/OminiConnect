# TSheets Tools

Provider: `tsheetsteam` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

TSheets is a time tracking and scheduling solution. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TSheets
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tsheets_list_timesheets` | List all time entries | GET | /timesheets |
| `tsheets_get_timesheet` | Get timesheet entry details | GET | /timesheets/{id} |
| `tsheets_create_timesheet` | Create a new timesheet entry | POST | /timesheets |
| `tsheets_update_timesheet` | Update a timesheet entry | PUT | /timesheets/{id} |
| `tsheets_list_employees` | List all employees | GET | /employees |
| `tsheets_get_employee` | Get employee details | GET | /employees/{id} |
| `tsheets_list_jobs` | List all jobs/projects | GET | /jobs |
| `tsheets_get_job` | Get job details | GET | /jobs/{id} |
| `tsheets_list_schedules` | List scheduled shifts | GET | /schedules |
| `tsheets_create_schedule` | Create a scheduled shift | POST | /schedules |

---

## Tool Details

### tsheets_list_timesheets

**What it does**: List all time entries

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_get_timesheet

**What it does**: Get timesheet entry details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_create_timesheet

**What it does**: Create a new timesheet entry

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_update_timesheet

**What it does**: Update a timesheet entry

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_list_employees

**What it does**: List all employees

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_get_employee

**What it does**: Get employee details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_list_jobs

**What it does**: List all jobs/projects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_get_job

**What it does**: Get job details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_list_schedules

**What it does**: List scheduled shifts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tsheets_create_schedule

**What it does**: Create a scheduled shift

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://rest.tsheets.com/api/v1`
- Docs: https://nango.dev/docs/integrations/all/tsheetsteam
