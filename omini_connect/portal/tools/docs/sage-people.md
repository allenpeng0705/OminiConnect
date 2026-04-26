# Sage-people Tools

Provider: `sage-people` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Sage People is an HR management platform focused on people operations. These tools allow AI agents to manage people records, leave requests, compensation, benefits, and organization structure.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Sage People
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `people:read`, `people:write`, `leave:read`, `leave:write`, `compensation:read`, `benefits:read`, `org:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sage-people_list_people` | List all people | GET | /v1/people |
| `sage-people_get_person` | Get person details | GET | /v1/people/{personId} |
| `sage-people_create_person` | Create a person | POST | /v1/people |
| `sage-people_list_leave_balances` | List leave balances | GET | /v1/leave-balances |
| `sage-people_list_leave_requests` | List leave requests | GET | /v1/leave-requests |
| `sage-people_create_leave_request` | Create leave request | POST | /v1/leave-requests |
| `sage-people_list_compensation` | List compensation records | GET | /v1/compensation |
| `sage-people_list_benefits` | List benefit plans | GET | /v1/benefits |
| `sage-people_get_org_chart` | Get organization chart | GET | /v1/org-chart |
| `sage-people_list_employees_by_role` | List employees by role | GET | /v1/people/by-role |

---

## Tool Details

### sage-people_list_people

**What it does**: Returns a list of all people.

**When to use**: View employee roster.

**Arguments**:
- `limit` (optional): Number of people (default 50)
- `departmentId` (optional): Filter by department

**Example LLM prompt**: "List all people in Engineering"

---

### sage-people_get_person

**What it does**: Gets details of a specific person.

**When to use**: Get employee information.

**Arguments**:
- `personId` (required): The person ID

**Example LLM prompt**: "Get details for person ppl_abc123"

---

### sage-people_create_person

**What it does**: Creates a new person record.

**When to use**: Add new employees.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (required): Email address
- `departmentId` (optional): Department ID

**Example LLM prompt**: "Create a person for John Smith with email john@company.com"

---

### sage-people_list_leave_balances

**What it does**: Returns leave balances.

**When to use**: Check available PTO.

**Arguments**:
- `personId` (optional): Filter by person

**Example LLM prompt**: "List leave balances for person ppl_abc123"

---

### sage-people_list_leave_requests

**What it does**: Returns leave requests.

**When to use**: View time off requests.

**Arguments**:
- `limit` (optional): Number of requests (default 50)
- `status` (optional): Filter by status (pending, approved, denied)

**Example LLM prompt**: "List all pending leave requests"

---

### sage-people_create_leave_request

**What it does**: Creates a leave request.

**When to use**: Submit time off request.

**Arguments**:
- `personId` (required): Person ID
- `leaveType` (required): Type of leave
- `startDate` (required): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Create a vacation request for person ppl_123"

---

### sage-people_list_compensation

**What it does**: Returns compensation records.

**When to use**: View salary information.

**Arguments**:
- `personId` (optional): Filter by person

**Example LLM prompt**: "List compensation for person ppl_abc123"

---

### sage-people_list_benefits

**What it does**: Returns benefit plans.

**When to use**: View available benefits.

**Arguments**:
- `limit` (optional): Number of plans (default 50)

**Example LLM prompt**: "List all benefit plans"

---

### sage-people_get_org_chart

**What it does**: Returns the organization chart.

**When to use**: View reporting structure.

**Arguments**: None

**Example LLM prompt**: "Get the organization chart"

---

### sage-people_list_employees_by_role

**What it does**: Returns employees with a specific role.

**When to use**: Find people in a role.

**Arguments**:
- `roleId` (required): Role ID

**Example LLM prompt**: "List all Engineering Managers"

---

## Sage People Notes

- People records contain all employee information
- Leave balances track available PTO
- Leave requests track time off workflow
- Compensation includes salary and bonuses
- Benefits are health, dental, etc.
- Org chart shows reporting structure
