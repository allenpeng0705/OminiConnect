# Sage-hr Tools

Provider: `sage-hr` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Sage HR is a human resources management platform. These tools allow AI agents to manage employees, departments, leave requests, attendance, and company information.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Sage HR
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `employees:read`, `employees:write`, `departments:read`, `leave:read`, `leave:write`, `attendance:read`, `attendance:write`, `companies:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sage-hr_list_employees` | List all employees | GET | /v1/employees |
| `sage-hr_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `sage-hr_create_employee` | Create an employee | POST | /v1/employees |
| `sage-hr_list_departments` | List departments | GET | /v1/departments |
| `sage-hr_list_leave_requests` | List leave requests | GET | /v1/leave-requests |
| `sage-hr_approve_leave` | Approve leave request | POST | /v1/leave-requests/{requestId}/approve |
| `sage-hr_list_attendance` | List attendance records | GET | /v1/attendance |
| `sage-hr_record_attendance` | Record attendance | POST | /v1/attendance |
| `sage-hr_list_companies` | List companies | GET | /v1/companies |
| `sage-hr_get_company_info` | Get company info | GET | /v1/companies/{companyId} |

---

## Tool Details

### sage-hr_list_employees

**What it does**: Returns a list of all employees.

**When to use**: View employee roster.

**Arguments**:
- `limit` (optional): Number of employees (default 50)
- `departmentId` (optional): Filter by department

**Example LLM prompt**: "List all employees in Engineering"

---

### sage-hr_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Get employee information and role.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get details for employee emp_abc123"

---

### sage-hr_create_employee

**What it does**: Creates a new employee.

**When to use**: Onboard a new team member.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (required): Work email
- `departmentId` (optional): Department ID

**Example LLM prompt**: "Create an employee for John Smith with email john@company.com"

---

### sage-hr_list_departments

**What it does**: Returns a list of all departments.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### sage-hr_list_leave_requests

**What it does**: Returns a list of leave requests.

**When to use**: Review pending time off.

**Arguments**:
- `limit` (optional): Number of requests (default 50)
- `status` (optional): Filter by status (pending, approved, denied)

**Example LLM prompt**: "List all pending leave requests"

---

### sage-hr_approve_leave

**What it does**: Approves a leave request.

**When to use**: Accept an employee's time off.

**Arguments**:
- `requestId` (required): The request ID

**Example LLM prompt**: "Approve leave request lr_xyz789"

---

### sage-hr_list_attendance

**What it does**: Returns attendance records.

**When to use**: Track employee attendance.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `date` (optional): Filter by date

**Example LLM prompt**: "List attendance for today"

---

### sage-hr_record_attendance

**What it does**: Records attendance for an employee.

**When to use**: Check in employees.

**Arguments**:
- `employeeId` (required): Employee ID
- `date` (required): Attendance date
- `status` (required): Status (present, absent, late)

**Example LLM prompt**: "Record attendance for emp_123 as present on 2024-01-15"

---

### sage-hr_list_companies

**What it does**: Returns a list of companies.

**When to use**: View company structure.

**Arguments**: None

**Example LLM prompt**: "List all companies"

---

### sage-hr_get_company_info

**What it does**: Gets company information.

**When to use**: Get company details and settings.

**Arguments**:
- `companyId` (required): The company ID

**Example LLM prompt**: "Get info for company comp_123"

---

## Sage HR Notes

- Employees belong to departments
- Leave requests track time off
- Attendance records daily presence
- Companies contain departments and employees
