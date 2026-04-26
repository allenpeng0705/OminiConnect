# Zenefits Tools

Provider: `zenefits` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zenefits is an HR and benefits management platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zenefits
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zenefits_list_employees` | List all employees | GET | /core/employees |
| `zenefits_get_employee` | Get employee details | GET | /core/employees/{id} |
| `zenefits_list_departments` | List all departments | GET | /core/departments |
| `zenefits_list_locations` | List all locations | GET | /core/locations |
| `zenefits_list_jobs` | List all job positions | GET | /core/jobs |
| `zenefits_list_time_off` | List time off requests | GET | /time_off/requests |
| `zenefits_create_time_off` | Create time off request | POST | /time_off/requests |
| `zenefits_list_beneficiaries` | List all beneficiaries | GET | /people/beneficiaries |
| `zenefits_list_payrolls` | List payroll runs | GET | /payroll/payrolls |
| `zenefits_list_approvals` | List pending approvals | GET | /approvals/requests |

---

## Tool Details

### zenefits_list_employees

**What it does**: List all employees

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_get_employee

**What it does**: Get employee details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_departments

**What it does**: List all departments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_locations

**What it does**: List all locations

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_jobs

**What it does**: List all job positions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_time_off

**What it does**: List time off requests

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_create_time_off

**What it does**: Create time off request

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_beneficiaries

**What it does**: List all beneficiaries

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_payrolls

**What it does**: List payroll runs

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zenefits_list_approvals

**What it does**: List pending approvals

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.zenefits.com`
- Docs: https://nango.dev/docs/integrations/all/zenefits
