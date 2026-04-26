# BambooHR (Basic Auth) Tools

Provider: `bamboohr-basic` | Engine: `nango` | Auth: BASIC via Nango (API Key)

## Overview

These tools wrap the BambooHR API. They allow AI agents to manage employees, view organizational structure, and access HR data. BambooHR is a cloud-based HR management system for small and medium businesses.

## Authentication

**Nango BASIC Auth**:
- User provides BambooHR API key as username
- Password is fixed (dummy value)
- Token stored in Nango, accessed via `connection_ref`
- Requires subdomain configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bamboohr_get_employee` | Get employee details | GET | /v1/employees/{id} |
| `bamboohr_list_employees` | List employees | GET | /v1/employees |
| `bamboohr_get_job_info` | Get job information | GET | /v1/employees/{id}/jobInfo |
| `bamboohr_get_time_off` | Get time off requests | GET | /v1/employees/{id}/timeOffRequests |
| `bamboohr_list_departments` | List departments | GET | /v1/departments |
| `bamboohr_list_locations` | List locations | GET | /v1/locations |
| `bamboohr_get_benefits` | Get employee benefits | GET | /v1/employees/{id}/benefits |
| `bamboohr_get_compensation` | Get compensation info | GET | /v1/employees/{id}/compensation |
| `bamboohr_list_metadata_fields` | List custom fields | GET | /v1/meta/fields |
| `bamboohr_get_reports` | Get report data | GET | /v1/reports/{reportId} |

---

## Tool Details

### bamboohr_get_employee

**What it does**: Gets details for a specific employee.

**When to use**: Look up employee information, contact details.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get details for employee ID 123"

---

### bamboohr_list_employees

**What it does**: Lists all employees in the company.

**When to use**: Browse employee directory, find team members.

**Arguments**:
- `sort` (optional): Sort by field (lastName, firstName, hireDate, etc.)
- `fields` (optional): Comma-separated fields to include

**Example LLM prompt**: "List all employees"

---

### bamboohr_get_job_info

**What it does**: Gets job information for an employee.

**When to use**: Check title, department, manager, location.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get job info for employee 123"

---

### bamboohr_get_time_off

**What it does**: Gets time off requests and balances for an employee.

**When to use**: Check PTO balance, review time off history.

**Arguments**:
- `id` (required): Employee ID
- `start` (optional): Start date filter (YYYY-MM-DD)
- `end` (optional): End date filter (YYYY-MM-DD)

**Example LLM prompt**: "Get time off requests for employee 123 this month"

---

### bamboohr_list_departments

**What it does**: Lists all departments in the company.

**When to use**: View organizational structure.

**Arguments**: None required

**Example LLM prompt**: "List all departments"

---

### bamboohr_list_locations

**What it does**: Lists all locations in the company.

**When to use**: View office locations, remote work setup.

**Arguments**: None required

**Example LLM prompt**: "List all locations"

---

### bamboohr_get_benefits

**What it does**: Gets benefits information for an employee.

**When to use**: Review health insurance, retirement plans.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get benefits for employee 123"

---

### bamboohr_get_compensation

**What it does**: Gets compensation information for an employee.

**When to use**: Check salary, pay history, bonuses.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get compensation for employee 123"

---

### bamboohr_list_metadata_fields

**What it does**: Lists all available employee fields.

**When to use**: Discover available data, understand schema.

**Arguments**: None required

**Example LLM prompt**: "List all available employee fields"

---

### bamboohr_get_reports

**What it does**: Gets data from a saved BambooHR report.

**When to use**: Generate reports, export HR data.

**Arguments**:
- `reportId` (required): Report ID or name

**Example LLM prompt**: "Get report 'Employee Headcount'"

---

## BambooHR API Notes

- **API Key**: 40-character hexadecimal string, found in BambooHR settings
- **Subdomain**: Your company's BambooHR subdomain (e.g., acme.bamboohr.com)
- **Employee ID**: Unique numeric identifier for each employee
- **Custom Fields**: Companies can create custom employee fields
- **Reports**: Pre-built or custom reports generated in BambooHR
