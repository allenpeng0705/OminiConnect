# Hibob Service User Tools

Provider: `hibob-service-user` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Hibob API. They allow AI agents to manage employees, departments, job titles, and named lists. Hibob is a human resources platform for SMBs.

## Authentication

**Nango Basic Auth**:
- User provides Hibob ID and token via Nango Connect
- Uses Hibob ID as username and token as password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.hibob.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `hibob_list_employees` | List employees | GET | /v1/employees |
| `hibob_get_employee` | Get employee details | GET | /v1/employees/{id} |
| `hibob_list_departments` | List departments | GET | /v1/company/departments |
| `hibob_get_department` | Get department details | GET | /v1/company/departments/{id} |
| `hibob_list_jobs` | List job titles | GET | /v1/company/job-titles |
| `hibob_get_job` | Get job title details | GET | /v1/company/job-titles/{id} |
| `hibob_list_named_lists` | List named lists | GET | /v1/company/named-lists |
| `hibob_get_named_list` | Get named list details | GET | /v1/company/named-lists/{id} |
| `hibob_list_anniversaries` | List anniversaries | GET | /v1/employees/anniversaries |
| `hibob_list_birthdays` | List birthdays | GET | /v1/employees/birthdays |

---

## Tool Details

### hibob_list_employees

**What it does**: Lists all employees in Hibob.

**When to use**: Browse employee directory.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all employees"

---

### hibob_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: View employee profile and details.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get employee with ID abc123"

---

### hibob_list_departments

**What it does**: Lists all departments in Hibob.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### hibob_get_department

**What it does**: Gets detailed information about a specific department.

**When to use**: View department members and settings.

**Arguments**:
- `id` (required): Department ID

**Example LLM prompt**: "Get department with ID eng123"

---

### hibob_list_jobs

**What it does**: Lists all job titles in Hibob.

**When to use**: Browse job titles.

**Arguments**: None

**Example LLM prompt**: "List all job titles"

---

### hibob_get_job

**What it does**: Gets detailed information about a specific job title.

**When to use**: View job title and employees.

**Arguments**:
- `id` (required): Job title ID

**Example LLM prompt**: "Get job title with ID mgr456"

---

### hibob_list_named_lists

**What it does**: Lists all named lists in Hibob.

**When to use**: Browse predefined lists.

**Arguments**: None

**Example LLM prompt**: "List all named lists"

---

### hibob_get_named_list

**What it does**: Gets detailed information about a specific named list.

**When to use**: View list values.

**Arguments**:
- `id` (required): Named list ID

**Example LLM prompt**: "Get named list with ID lst789"

---

### hibob_list_anniversaries

**What it does**: Lists upcoming work anniversaries.

**When to use**: Track employee milestones.

**Arguments**:
- `months_ahead` (optional): Months ahead to look (default 1)

**Example LLM prompt**: "List anniversaries for the next month"

---

### hibob_list_birthdays

**What it does**: Lists upcoming employee birthdays.

**When to use**: Track upcoming birthdays.

**Arguments**:
- `months_ahead` (optional): Months ahead to look (default 1)

**Example LLM prompt**: "List birthdays for the next month"

---

## Hibob API Notes

- **API Base URL**: https://api.hibob.com
- **Auth Mode**: Basic Auth with ID and token
- **Employees**: HR records for company employees
- **Departments**: Organizational units
- **Job Titles**: Employee positions
- **Named Lists**: Predefined dropdown lists
- **Anniversaries**: Work anniversary tracking
- **Birthdays**: Employee birthday tracking
