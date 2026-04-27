# BambooHR Tools

Provider: `bamboohr` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the BambooHR API. They allow AI agents to manage employees, jobs, time off requests, and benefits in the BambooHR HRIS system. BambooHR is a popular HR platform for small and medium businesses.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with BambooHR
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `employees:read`, `employees:write`, `jobs:read`, `time_off:read`, `time_off:write`, `benefits:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bamboohr_list_employees` | List all employees | GET | /employees |
| `bamboohr_get_employee` | Get employee details | GET | /employees/{employee_id} |
| `bamboohr_create_employee` | Create new employee | POST | /employees |
| `bamboohr_list_jobs` | List job openings | GET | /jobs |
| `bamboohr_get_job` | Get job posting details | GET | /jobs/{job_id} |
| `bamboohr_list_time_off` | List time off requests | GET | /time_off/requests |
| `bamboohr_get_time_off_request` | Get time off request details | GET | /time_off/requests/{request_id} |
| `bamboohr_create_time_off_request` | Submit time off request | POST | /time_off/requests |
| `bamboohr_list_benefits` | List benefit enrollment | GET | /benefits |
| `bamboohr_get_benefit` | Get benefit details | GET | /benefits/{benefit_id} |

---

## Tool Details

### bamboohr_list_employees

**What it does**: Lists all employees with optional filters for status, department, or location. Returns employee names, titles, departments, and contact information.

**When to use**: Browse employee directory, find employees by department, check org chart.

**Arguments**:
- `status` (optional): Filter by status: A (active), I (inactive)
- `department` (optional): Filter by department ID
- `location` (optional): Filter by location
- `fields` (optional): Comma-separated list of fields to return

**Example LLM prompt**: "List all active employees in the Engineering department"

---

### bamboohr_get_employee

**What it does**: Gets detailed employee information including personal info, job details, compensation, manager, and employment history.

**When to use**: Get full employee profile, review employment details and reporting structure.

**Arguments**:
- `employee_id` (required): Employee ID
- `fields` (optional): Comma-separated list of fields to return

**Example LLM prompt**: "Get details for employee 123 to see their manager and department"

---

### bamboohr_create_employee

**What it does**: Creates a new employee record with name, job info, start date, and personal details. New employees are added to the directory and can be assigned to departments.

**When to use**: Add new hires to the system, onboard employees.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `workEmail` (optional): Work email address
- `department` (optional): Department ID
- `hireDate` (optional): Hire date (YYYY-MM-DD)
- `employmentHistoryStatus` (optional): Employment type: FullTime, PartTime, Contractor (default: FullTime)

**Example LLM prompt**: "Create a new employee for Jane Smith starting March 1st in the Product department"

---

### bamboohr_list_jobs

**What it does**: Lists all job openings with optional filters for status or department. Returns job titles, descriptions, locations, and employment status.

**When to use**: Browse open positions, find jobs by department, check hiring status.

**Arguments**:
- `status` (optional): Filter by status: open, closed
- `department` (optional): Filter by department ID

**Example LLM prompt**: "List all open jobs in the Sales department"

---

### bamboohr_get_job

**What it does**: Gets detailed information about a specific job posting including full description, requirements, location, department, and salary range if available.

**When to use**: Read full job details, check requirements, share with candidates.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job 456 to see the full description and requirements"

---

### bamboohr_list_time_off

**What it does**: Lists time off requests with optional filters for employee, status, or date range. Returns request details, approval status, and time-off balances.

**When to use**: Review pending requests, track PTO usage, check employee availability.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `status` (optional): Filter by status: approved, denied, pending, canceled
- `start` (optional): Start date filter (YYYY-MM-DD)
- `end` (optional): End date filter (YYYY-MM-DD)
- `type` (optional): Time off type: Vacation, Sick, Personal, etc.

**Example LLM prompt**: "List all pending time off requests for next week"

---

### bamboohr_get_time_off_request

**What it does**: Gets detailed information about a specific time off request including dates, type, notes, approval history, and current status.

**When to use**: Review specific time off request details before approving or denying.

**Arguments**:
- `request_id` (required): Time off request ID

**Example LLM prompt**: "Get details for time off request 789 to see who needs to approve it"

---

### bamboohr_create_time_off_request

**What it does**: Submits a new time off request for an employee with start date, end date, and type. Request will be sent for approval to the employee's manager.

**When to use**: Submit PTO requests, request sick leave, schedule personal time.

**Arguments**:
- `employeeId` (required): Employee ID
- `start` (required): Start date YYYY-MM-DD
- `end` (required): End date YYYY-MM-DD
- `type` (required): Time off type: Vacation, Sick, Personal
- `notes` (optional): Optional notes for the request

**Example LLM prompt**: "Submit a time off request for employee 123 for March 15-20 vacation"

---

### bamboohr_list_benefits

**What it does**: Lists benefit enrollments for an employee or all employees with optional filters. Returns benefit plans, coverage levels, and enrollment status.

**When to use**: Review benefit enrollments, check employee coverage, list available plans.

**Arguments**:
- `employee_id` (optional): Filter by employee ID
- `benefit_type` (optional): Filter by benefit type: health, dental, vision, 401k, etc.

**Example LLM prompt**: "List all health benefits for employee 123"

---

### bamboohr_get_benefit

**What it does**: Gets detailed information about a specific benefit enrollment including coverage details, costs, provider information, and enrollment dates.

**When to use**: Review benefit details, check coverage levels and costs.

**Arguments**:
- `benefit_id` (required): Benefit enrollment ID

**Example LLM prompt**: "Get details for benefit 999 to see the coverage and monthly cost"

---

## BambooHR API Notes

- **Employee Status**: Use "A" for active, "I" for inactive employees
- **Time Off Types**: Common types include Vacation, Sick, Personal, Bereavement, Jury Duty
- **Approval Workflow**: Time off requests require manager approval before status changes to approved
- **Benefits**: Include health insurance, dental, vision, retirement (401k), and other company offerings
- **Departments**: Numeric IDs used for filtering and assignment
- **Field Selection**: Use `fields` parameter to specify which employee fields to return in responses
