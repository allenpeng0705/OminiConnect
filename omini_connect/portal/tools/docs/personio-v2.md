# Personio v2 Tools

Provider: `personio-v2` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Personio API v2. They allow AI agents to manage employees, absences, attendances, contracts, and departments. Personio is an HR management platform. **Requires Personio v2 OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client credentials for token flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.personio.de/v2
- Uses form-encoded requests

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `personio_v2_list_employees` | List employees | GET | /v2/employees |
| `personio_v2_get_employee` | Get employee details | GET | /v2/employees/{employeeId} |
| `personio_v2_list_absences` | List absences | GET | /v2/absences |
| `personio_v2_get_absence` | Get absence details | GET | /v2/absences/{absenceId} |
| `personio_v2_list_attendances` | List attendances | GET | /v2/attendances |
| `personio_v2_get_attendance` | Get attendance details | GET | /v2/attendances/{attendanceId} |
| `personio_v2_list_contracts` | List contracts | GET | /v2/contracts |
| `personio_v2_get_contract` | Get contract details | GET | /v2/contracts/{contractId} |
| `personio_v2_list_departments` | List departments | GET | /v2/departments |
| `personio_v2_get_company` | Get company information | GET | /v2/company |

---

## Tool Details

### personio_v2_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees.

**Arguments**:
- `departmentId` (optional): Filter by department
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active employees"

---

### personio_v2_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, profile.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### personio_v2_list_absences

**What it does**: Lists all absences in the organization.

**When to use**: Browse absence records, track time off.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show absences for this month"

---

### personio_v2_get_absence

**What it does**: Gets detailed information about a specific absence.

**When to use**: Get absence details.

**Arguments**:
- `absenceId` (required): Absence ID

**Example LLM prompt**: "Get details for absence 67890"

---

### personio_v2_list_attendances

**What it does**: Lists all attendances in the organization.

**When to use**: Browse attendance records.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show attendances for last week"

---

### personio_v2_get_attendance

**What it does**: Gets detailed information about a specific attendance record.

**When to use**: Get attendance details.

**Arguments**:
- `attendanceId` (required): Attendance ID

**Example LLM prompt**: "Get details for attendance 11111"

---

### personio_v2_list_contracts

**What it does**: Lists all employment contracts.

**When to use**: Browse contract details.

**Arguments**: None

**Example LLM prompt**: "List all contracts"

---

### personio_v2_get_contract

**What it does**: Gets detailed information about a specific contract.

**When to use**: Get contract details.

**Arguments**:
- `contractId` (required): Contract ID

**Example LLM prompt**: "Get details for contract C-22222"

---

### personio_v2_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### personio_v2_get_company

**What it does**: Gets company/organization information.

**When to use**: Get company settings, configuration.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Personio v2 API Notes

- **OAuth2 Client Credentials**: Machine-to-machine authentication
- **Form-encoded**: Uses form encoding for requests
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
