# UKG Ready Tools

Provider: `ukg-ready` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

UKG Ready is a workforce management solution for small businesses. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ukg_list_employees` | List all employees | GET | /api/v1/employees |
| `ukg_get_employee` | Get employee details | GET | /api/v1/employees/{id} |
| `ukg_list_shifts` | List scheduled shifts | GET | /api/v1/shifts |
| `ukg_create_shift` | Create a new shift | POST | /api/v1/shifts |
| `ukg_list_time_entries` | List time entries | GET | /api/v1/time-entries |
| `ukg_list_departments` | List all departments | GET | /api/v1/departments |
| `ukg_list_positions` | List all positions | GET | /api/v1/positions |
| `ukg_get_company_info` | Get company information | GET | /api/v1/company |
| `ukg_list_approvals` | List pending approvals | GET | /api/v1/approvals |
| `ukg_process_payroll` | Process payroll | POST | /api/v1/payroll/process |

---

## Tool Details

### ukg_list_employees

**What it does**: List all employees

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_get_employee

**What it does**: Get employee details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

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

### ukg_list_departments

**What it does**: List all departments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_list_positions

**What it does**: List all positions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_get_company_info

**What it does**: Get company information

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

### ukg_process_payroll

**What it does**: Process payroll

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.hostname}`
- Docs: https://nango.dev/docs/integrations/all/ukg-ready
