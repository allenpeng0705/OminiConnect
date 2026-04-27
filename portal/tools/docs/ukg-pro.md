# UKG Pro Tools

Provider: `ukg-pro` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

UKG Pro is a human resources and payroll management system. **Requires basic auth via nango.**

## Authentication

**Nango Basic Auth**:
- User provides username/password
- Credentials stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ukg_list_employees` | List all employees | GET | /api/v1/employees |
| `ukg_get_employee` | Get employee details | GET | /api/v1/employees/{id} |
| `ukg_list_payrolls` | List payroll runs | GET | /api/v1/payrolls |
| `ukg_get_payroll` | Get payroll details | GET | /api/v1/payrolls/{id} |
| `ukg_list_time_off` | List time off requests | GET | /api/v1/time-off |
| `ukg_create_time_off` | Create time off request | POST | /api/v1/time-off |
| `ukg_list_departments` | List all departments | GET | /api/v1/departments |
| `ukg_list_jobs` | List all job positions | GET | /api/v1/jobs |
| `ukg_get_company_info` | Get company information | GET | /api/v1/company |
| `ukg_list_benefits` | List employee benefits | GET | /api/v1/benefits |

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

### ukg_list_payrolls

**What it does**: List payroll runs

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_get_payroll

**What it does**: Get payroll details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_list_time_off

**What it does**: List time off requests

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### ukg_create_time_off

**What it does**: Create time off request

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

### ukg_list_jobs

**What it does**: List all job positions

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

### ukg_list_benefits

**What it does**: List employee benefits

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.hostname}`
- Docs: https://nango.dev/docs/integrations/all/ukg-pro
