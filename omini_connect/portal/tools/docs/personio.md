# Personio Tools

Provider: `personio` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Personio API (v1). They allow AI agents to manage employees, absences, attendances, contracts, and departments. Personio is an HR management platform. **Requires Personio OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client credentials for token flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.personio.de/v1
- Requires partnerId and appId in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `personio_list_employees` | List employees | GET | /v1/employees |
| `personio_get_employee` | Get employee details | GET | /v1/employees/{employeeId} |
| `personio_list_absences` | List absences | GET | /v1/absences |
| `personio_get_absence` | Get absence details | GET | /v1/absences/{absenceId} |
| `personio_list_attendances` | List attendances | GET | /v1/attendances |
| `personio_get_attendance` | Get attendance details | GET | /v1/attendances/{attendanceId} |
| `personio_list_contracts` | List contracts | GET | /v1/contracts |
| `personio_get_contract` | Get contract details | GET | /v1/contracts/{contractId} |
| `personio_list_departments` | List departments | GET | /v1/departments |
| `personio_get_company` | Get company information | GET | /v1/company |

---

## Tool Details

### personio_list_employees

**What it does**: Lists all employees in the organization.

**When to use**: Browse employee directory, find employees.

**Arguments**:
- `departmentId` (optional): Filter by department
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active employees"

---

### personio_get_employee

**What it does**: Gets detailed information about a specific employee.

**When to use**: Get employee details, profile.

**Arguments**:
- `employeeId` (required): Employee ID

**Example LLM prompt**: "Get details for employee 12345"

---

### personio_list_absences

**What it does**: Lists all absences in the organization.

**When to use**: Browse absence records, track time off.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show absences for this month"

---

### personio_get_absence

**What it does**: Gets detailed information about a specific absence.

**When to use**: Get absence details.

**Arguments**:
- `absenceId` (required): Absence ID

**Example LLM prompt**: "Get details for absence 67890"

---

### personio_list_attendances

**What it does**: Lists all attendances in the organization.

**When to use**: Browse attendance records.

**Arguments**:
- `employeeId` (optional): Filter by employee
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show attendances for last week"

---

### personio_get_attendance

**What it does**: Gets detailed information about a specific attendance record.

**When to use**: Get attendance details.

**Arguments**:
- `attendanceId` (required): Attendance ID

**Example LLM prompt**: "Get details for attendance 11111"

---

### personio_list_contracts

**What it does**: Lists all employment contracts.

**When to use**: Browse contract details.

**Arguments**: None

**Example LLM prompt**: "List all contracts"

---

### personio_get_contract

**What it does**: Gets detailed information about a specific contract.

**When to use**: Get contract details.

**Arguments**:
- `contractId` (required): Contract ID

**Example LLM prompt**: "Get details for contract C-22222"

---

### personio_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### personio_get_company

**What it does**: Gets company/organization information.

**When to use**: Get company settings, configuration.

**Arguments**: None

**Example LLM prompt**: "Get our company information"

---

## Personio API Notes

- **OAuth2 Client Credentials**: Machine-to-machine authentication
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
