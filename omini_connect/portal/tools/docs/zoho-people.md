# Zoho People Tools

Provider: `zoho-people` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho People is an HR management platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho People
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_people_list_employees` | List all employees | GET | /api/v1/employees |
| `zoho_people_get_employee` | Get employee details | GET | /api/v1/employees/{id} |
| `zoho_people_create_employee` | Create a new employee | POST | /api/v1/employees |
| `zoho_people_update_employee` | Update employee details | PUT | /api/v1/employees/{id} |
| `zoho_people_list_attendance` | List attendance records | GET | /api/v1/attendance |
| `zoho_people_record_attendance` | Record attendance entry | POST | /api/v1/attendance |
| `zoho_people_list_leaves` | List all leave requests | GET | /api/v1/leaves |
| `zoho_people_create_leave` | Create a leave request | POST | /api/v1/leaves |
| `zoho_people_list_departments` | List all departments | GET | /api/v1/departments |
| `zoho_people_list_shifts` | List all shifts | GET | /api/v1/shifts |

---

## Tool Details

### zoho_people_list_employees

**What it does**: List all employees

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_get_employee

**What it does**: Get employee details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_create_employee

**What it does**: Create a new employee

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_update_employee

**What it does**: Update employee details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_list_attendance

**What it does**: List attendance records

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_record_attendance

**What it does**: Record attendance entry

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_list_leaves

**What it does**: List all leave requests

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_create_leave

**What it does**: Create a leave request

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_list_departments

**What it does**: List all departments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_people_list_shifts

**What it does**: List all shifts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://people.zoho.com`
- Docs: https://nango.dev/docs/integrations/all/zoho-people
