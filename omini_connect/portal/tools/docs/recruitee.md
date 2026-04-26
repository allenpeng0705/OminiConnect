# Recruitee Tools

Provider: `recruitee` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Recruitee is a recruitment platform for managing hiring pipelines. These tools allow AI agents to manage offers, candidates, positions, departments, and interviews.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Recruitee
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `offers:read`, `offers:write`, `candidates:read`, `departments:read`, `positions:read`, `interviews:read`, `interviews:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `recruitee_list_offers` | List all offers | GET | /offers |
| `recruitee_get_offer` | Get offer details | GET | /offers/{offerId} |
| `recruitee_create_offer` | Create an offer | POST | /offers |
| `recruitee_list_candidates` | List candidates | GET | /candidates |
| `recruitee_get_candidate` | Get candidate details | GET | /candidates/{candidateId} |
| `recruitee_list_departments` | List departments | GET | /departments |
| `recruitee_list_positions` | List positions | GET | /positions |
| `recruitee_get_position` | Get position details | GET | /positions/{positionId} |
| `recruitee_list_interviews` | List interviews | GET | /interviews |
| `recruitee_create_interview` | Schedule interview | POST | /interviews |

---

## Tool Details

### recruitee_list_offers

**What it does**: Returns a list of all job offers.

**When to use**: Track offer status across candidates.

**Arguments**:
- `limit` (optional): Number of offers (default 50)
- `status` (optional): Filter by status (pending, accepted, declined)

**Example LLM prompt**: "List all pending offers"

---

### recruitee_get_offer

**What it does**: Gets details of a specific offer.

**When to use**: Check offer details, salary, and start date.

**Arguments**:
- `offerId` (required): The offer ID

**Example LLM prompt**: "Get details for offer off_abc123"

---

### recruitee_create_offer

**What it does**: Creates a new job offer.

**When to use**: Extend an offer to a candidate.

**Arguments**:
- `candidateId` (required): Candidate ID
- `positionId` (required): Position ID
- `salary` (optional): Salary amount
- `startDate` (optional): Proposed start date

**Example LLM prompt**: "Create an offer for candidate cnd_123 for position pos_456"

---

### recruitee_list_candidates

**What it does**: Returns a list of all candidates.

**When to use**: Browse candidate pool.

**Arguments**:
- `limit` (optional): Number of candidates (default 50)

**Example LLM prompt**: "List all candidates"

---

### recruitee_get_candidate

**What it does**: Gets details of a specific candidate.

**When to use**: View candidate profile and application.

**Arguments**:
- `candidateId` (required): The candidate ID

**Example LLM prompt**: "Get details for candidate cnd_xyz789"

---

### recruitee_list_departments

**What it does**: Returns a list of all departments.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### recruitee_list_positions

**What it does**: Returns a list of all positions.

**When to use**: View open roles and hiring status.

**Arguments**:
- `limit` (optional): Number of positions (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all open positions"

---

### recruitee_get_position

**What it does**: Gets details of a specific position.

**When to use**: Get position requirements and details.

**Arguments**:
- `positionId` (required): The position ID

**Example LLM prompt**: "Get details for position pos_456"

---

### recruitee_list_interviews

**What it does**: Returns a list of all interviews.

**When to use**: View interview schedule.

**Arguments**:
- `limit` (optional): Number of interviews (default 50)

**Example LLM prompt**: "List all upcoming interviews"

---

### recruitee_create_interview

**What it does**: Schedules a new interview.

**When to use**: Book interview time with candidate.

**Arguments**:
- `candidateId` (required): Candidate ID
- `positionId` (required): Position ID
- `scheduledTime` (required): Interview time
- `interviewType` (optional): Type (phone, video, onsite)

**Example LLM prompt**: "Schedule a video interview with candidate cnd_123 for position pos_456"

---

## Recruitee Notes

- Offers link candidates to positions with compensation details
- Candidates are tracked through the hiring pipeline
- Positions represent open roles within departments
- Interviews are scheduled between candidates and positions
