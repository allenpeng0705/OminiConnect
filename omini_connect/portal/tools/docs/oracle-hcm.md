# Oracle Fusion Cloud (HCM) Tools

Provider: `oracle-hcm` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Oracle Fusion Cloud HCM REST API. They allow AI agents to manage workers, departments, locations, absences, salaries, and performance records. **Requires Oracle HCM Basic authentication.**

## Authentication

**Basic Auth**:
- User provides Oracle HCM username and password
- Credentials passed via Nango
- Base URL: `https://{restServerUrl}`
- Content-Type: `application/vnd.oracle.adf.resourceitem+json`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `oracle_list_workers` | List workers | GET | /hcm/v1/workers |
| `oracle_get_worker` | Get worker details | GET | /hcm/v1/workers/{personId} |
| `oracle_list_departments` | List departments | GET | /hcm/v1/departments |
| `oracle_list_jobs` | List jobs | GET | /hcm/v1/jobs |
| `oracle_list_locations` | List locations | GET | /hcm/v1/locations |
| `oracle_list_absences` | List absences | GET | /hcm/v1/absences |
| `oracle_list_salaries` | List salary data | GET | /hcm/v1/salaries |
| `oracle_list_performance` | List performance records | GET | /hcm/v1/performance |
| `oracle_list_leave_requests` | List leave requests | GET | /hcm/v1/leaveRequests |
| `oracle_get_person` | Get person details | GET | /hcm/v1/persons/{personId} |

---

## Tool Details

### oracle_list_workers

**What it does**: Lists all workers in Oracle HCM.

**When to use**: Browse employee directory, find workers.

**Arguments**:
- `limit` (optional): Number of workers (default 20)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all workers in the engineering department"

---

### oracle_get_worker

**What it does**: Gets detailed information for a specific worker.

**When to use**: View worker profile, assignment, manager.

**Arguments**:
- `personId` (required): Person ID

**Example LLM prompt**: "Get details for worker P12345"

---

### oracle_list_departments

**What it does**: Lists all departments in Oracle HCM.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### oracle_list_jobs

**What it does**: Lists all jobs in Oracle HCM.

**When to use**: View job definitions, understand roles.

**Arguments**: None

**Example LLM prompt**: "List all jobs in the system"

---

### oracle_list_locations

**What it does**: Lists all locations in Oracle HCM.

**When to use**: View workplace locations, offices.

**Arguments**: None

**Example LLM prompt**: "List all locations"

---

### oracle_list_absences

**What it does**: Lists all absence records in Oracle HCM.

**When to use**: Track absences, view attendance.

**Arguments**:
- `personId` (optional): Filter by person ID

**Example LLM prompt**: "List all absences for worker P12345"

---

### oracle_list_salaries

**What it does**: Lists salary data in Oracle HCM.

**When to use**: View compensation, salary history.

**Arguments**:
- `personId` (optional): Filter by person ID

**Example LLM prompt**: "List salary data for worker P12345"

---

### oracle_list_performance

**What it does**: Lists performance records in Oracle HCM.

**When to use**: View performance reviews, ratings.

**Arguments**:
- `personId` (optional): Filter by person ID

**Example LLM prompt**: "List performance records for worker P12345"

---

### oracle_list_leave_requests

**What it does**: Lists leave requests in Oracle HCM.

**When to use**: Track leave requests, approve time off.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending leave requests"

---

### oracle_get_person

**What it does**: Gets person details from Oracle HCM.

**When to use**: View personal information, contact details.

**Arguments**:
- `personId` (required): Person ID

**Example LLM prompt**: "Get person details for P12345"

---

## Oracle HCM Notes

- **REST Server URL**: Oracle Fusion REST server endpoint
- **Content-Type**: Uses Oracle ADF resourceitem format
- **Workers**: Employees and non-workers in HCM
- **Person IDs**: Unique identifiers for persons
- **Absences**: Tracks time off and leave
