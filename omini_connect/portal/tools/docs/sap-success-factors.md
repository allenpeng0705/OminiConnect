# Sap-success-factors Tools

Provider: `sap-success-factors` | Engine: `nango` | Auth: OAuth via Nango

## Overview

SAP SuccessFactors is an HR management system for employee data, time off, and performance. These tools allow AI agents to manage employees, departments, positions, time off, and performance reviews.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SAP SuccessFactors
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `employees:read`, `employees:write`, `departments:read`, `positions:read`, `timeOff:read`, `timeOff:write`, `performance:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap-success-factors_list_employees` | List all employees | GET | /v1/employees |
| `sap-success-factors_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `sap-success-factors_list_departments` | List all departments | GET | /v1/departments |
| `sap-success-factors_get_department` | Get department details | GET | /v1/departments/{departmentId} |
| `sap-success-factors_list_positions` | List all positions | GET | /v1/positions |
| `sap-success-factors_get_position` | Get position details | GET | /v1/positions/{positionId} |
| `sap-success-factors_list_time_off` | List time off records | GET | /v1/timeOff |
| `sap-success-factors_approve_time_off` | Approve time off request | POST | /v1/timeOff/{timeOffId}/approve |
| `sap-success-factors_list_performance` | List performance reviews | GET | /v1/performance |
| `sap-success-factors_get_employee_profile` | Get employee profile | GET | /v1/employees/{employeeId}/profile |

---

## Tool Details

### sap-success-factors_list_employees

**What it does**: Returns a list of all employees.

**When to use**: View employee directory.

**Arguments**:
- `limit` (optional): Number of employees (default 50)
- `department` (optional): Filter by department

**Example LLM prompt**: "List all employees"

---

### sap-success-factors_get_employee

**What it does**: Gets details of a specific employee.

**When to use**: Get employee information.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get details for employee emp_abc123"

---

### sap-success-factors_list_departments

**What it does**: Returns a list of all departments.

**When to use**: View organizational structure.

**Arguments**:
- `limit` (optional): Number of departments (default 50)

**Example LLM prompt**: "List all departments"

---

### sap-success-factors_get_department

**What it does**: Gets details of a specific department.

**When to use**: Get department information.

**Arguments**:
- `departmentId` (required): The department ID

**Example LLM prompt**: "Get details for department dept_xyz789"

---

### sap-success-factors_list_positions

**What it does**: Returns a list of all positions.

**When to use**: View job positions.

**Arguments**:
- `limit` (optional): Number of positions (default 50)
- `department` (optional): Filter by department

**Example LLM prompt**: "List all positions in Engineering"

---

### sap-success-factors_get_position

**What it does**: Gets details of a specific position.

**When to use**: Get position information.

**Arguments**:
- `positionId` (required): The position ID

**Example LLM prompt**: "Get details for position pos_abc123"

---

### sap-success-factors_list_time_off

**What it does**: Returns a list of all time off records.

**When to use**: View time off requests.

**Arguments**:
- `limit` (optional): Number of records (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending time off requests"

---

### sap-success-factors_approve_time_off

**What it does**: Approves a time off request.

**When to use**: Process time off approval.

**Arguments**:
- `timeOffId` (required): The time off ID to approve

**Example LLM prompt**: "Approve time off request to_123"

---

### sap-success-factors_list_performance

**What it does**: Returns a list of all performance reviews.

**When to use**: View performance evaluations.

**Arguments**:
- `limit` (optional): Number of reviews (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending performance reviews"

---

### sap-success-factors_get_employee_profile

**What it does**: Gets employee profile information.

**When to use**: View detailed employee profile.

**Arguments**:
- `employeeId` (required): The employee ID

**Example LLM prompt**: "Get profile for employee emp_abc123"

---

## SAP SuccessFactors Notes

- Employees are HR records
- Departments organize the company
- Positions define job roles
- Time off tracks leave requests
- Performance manages evaluations
