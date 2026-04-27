# Rippling Tools

Provider: `rippling` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Rippling is an all-in-one HR, IT, and operations platform. These tools allow AI agents to manage employees, departments, time off, devices, and company information.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Rippling
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `employees:read`, `employees:write`, `departments:read`, `jobs:read`, `time-off:read`, `time-off:write`, `devices:read`, `apps:read`, `company:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `rippling_list_employees` | List all employees | GET | /v1/employees |
| `rippling_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `rippling_create_employee` | Create an employee | POST | /v1/employees |
| `rippling_list_departments` | List departments | GET | /v1/departments |
| `rippling_list_jobs` | List jobs | GET | /v1/jobs |
| `rippling_list_time_off` | List time off requests | GET | /v1/time-off |
| `rippling_approve_time_off` | Approve time off request | POST | /v1/time-off/{requestId}/approve |
| `rippling_list_devices` | List devices | GET | /v1/devices |
| `rippling_list_apps` | List apps | GET | /v1/apps |
| `rippling_get_company_info` | Get company info | GET | /v1/company |

---

## Tool Details

### rippling_list_employees

**What it does**: Returns a list of all employees.

**When to use**: View employee roster, find employees.

**Arguments**:
- `limit` (optional): Number of employees (default 50)
- `departmentId` (optional): Filter by department

**Example LLM prompt**: "List all employees in Engineering"

---

### rippling_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Get employee information, title, department.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get details for employee emp_abc123"

---

### rippling_create_employee

**What it does**: Creates a new employee record.

**When to use**: Onboard a new team member.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `workEmail` (required): Work email
- `departmentId` (optional): Department ID

**Example LLM prompt**: "Create an employee for John Doe with email john@company.com"

---

### rippling_list_departments

**What it does**: Returns a list of all departments.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### rippling_list_jobs

**What it does**: Returns a list of all jobs.

**When to use**: View job positions in the company.

**Arguments**:
- `limit` (optional): Number of jobs (default 50)

**Example LLM prompt**: "List all open jobs"

---

### rippling_list_time_off

**What it does**: Returns a list of time off requests.

**When to use**: Review pending time off requests.

**Arguments**:
- `limit` (optional): Number of requests (default 50)
- `status` (optional): Filter by status (pending, approved, denied)

**Example LLM prompt**: "List all pending time off requests"

---

### rippling_approve_time_off

**What it does**: Approves a time off request.

**When to use**: Accept a employee's time off request.

**Arguments**:
- `requestId` (required): The request ID

**Example LLM prompt**: "Approve time off request req_xyz789"

---

### rippling_list_devices

**What it does**: Returns a list of managed devices.

**When to use**: View company devices and their status.

**Arguments**:
- `limit` (optional): Number of devices (default 50)

**Example LLM prompt**: "List all company devices"

---

### rippling_list_apps

**What it does**: Returns a list of integrated apps.

**When to use**: View available software integrations.

**Arguments**:
- `limit` (optional): Number of apps (default 50)

**Example LLM prompt**: "List all integrated apps"

---

### rippling_get_company_info

**What it does**: Returns company information.

**When to use**: Get company details and settings.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

## Rippling Notes

- Rippling unifies HR, IT, and operations in one platform
- Employees belong to departments and have job titles
- Time off requests need approval from managers
- Devices are managed endpoints in the organization
