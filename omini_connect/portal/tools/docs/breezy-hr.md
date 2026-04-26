# Breezy HR Tools

Provider: `breezy-hr` | Engine: `nango` | Auth: TWO_STEP via Nango (email + password)

## Overview

These tools wrap the Breezy HR API. They allow AI agents to manage job positions, candidates, interviews, and team members for applicant tracking and recruiting. Breezy HR is a modern HR platform for hiring and talent management.

## Authentication

**Nango TWO_STEP Auth**:
- User provides email and password
- Nango obtains session token via login API
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `breezy_list_positions` | List positions | GET | /v3/positions |
| `breezy_get_position` | Get position details | GET | /v3/positions/{positionId} |
| `breezy_list_candidates` | List candidates | GET | /v3/candidates |
| `breezy_get_candidate` | Get candidate details | GET | /v3/candidates/{candidateId} |
| `breezy_list_interviews` | List interviews | GET | /v3/interviews |
| `breezy_get_interview` | Get interview details | GET | /v3/interviews/{interviewId} |
| `breezy_list_members` | List team members | GET | /v3/members |
| `breezy_get_member` | Get member details | GET | /v3/members/{memberId} |
| `breezy_list_offer_docs` | List offer documents | GET | /v3/offers |
| `breezy_get_offer` | Get offer details | GET | /v3/offers/{offerId} |

---

## Tool Details

### breezy_list_positions

**What it does**: Lists all job positions in Breezy HR.

**When to use**: Browse open jobs, find positions to apply to.

**Arguments**:
- `status` (optional): Filter by status (open, closed, draft)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open positions"

---

### breezy_get_position

**What it does**: Gets details for a specific position.

**When to use**: View position requirements, description.

**Arguments**:
- `positionId` (required): Position ID

**Example LLM prompt**: "Get details for position P-123"

---

### breezy_list_candidates

**What it does**: Lists candidates for a position or all candidates.

**When to use**: View applicant pipeline, track candidates.

**Arguments**:
- `positionId` (optional): Filter by position
- `stage` (optional): Filter by hiring stage
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all candidates for position P-123"

---

### breezy_get_candidate

**What it does**: Gets details for a specific candidate.

**When to use**: Review candidate profile, resume, notes.

**Arguments**:
- `candidateId` (required): Candidate ID

**Example LLM prompt**: "Get details for candidate C-456"

---

### breezy_list_interviews

**What it does**: Lists scheduled interviews.

**When to use**: View interview schedule, manage calendar.

**Arguments**:
- `positionId` (optional): Filter by position
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all interviews for this week"

---

### breezy_get_interview

**What it does**: Gets details for a specific interview.

**When to use**: View interview details, participants.

**Arguments**:
- `interviewId` (required): Interview ID

**Example LLM prompt**: "Get details for interview I-789"

---

### breezy_list_members

**What it does**: Lists team members in Breezy HR.

**When to use**: View hiring team, assign reviewers.

**Arguments**: None required

**Example LLM prompt**: "List all team members"

---

### breezy_get_member

**What it does**: Gets details for a specific team member.

**When to use**: Check member role, permissions.

**Arguments**:
- `memberId` (required): Member ID

**Example LLM prompt**: "Get details for member M-100"

---

### breezy_list_offer_docs

**What it does**: Lists offer documents for candidates.

**When to use**: Track offers, manage hiring process.

**Arguments**:
- `candidateId` (optional): Filter by candidate
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all pending offers"

---

### breezy_get_offer

**What it does**: Gets details for a specific offer.

**When to use**: View offer terms, status.

**Arguments**:
- `offerId` (required): Offer ID

**Example LLM prompt**: "Get details for offer O-200"

---

## Breezy HR API Notes

- **Positions**: Job postings with requirements and description
- **Candidates**: Applicants in the hiring pipeline
- **Stages**: Hiring pipeline stages (applied, screening, interview, offer, hired)
- **Interviews**: Scheduled interviews with candidates
- **Team Members**: HR team users with different roles
