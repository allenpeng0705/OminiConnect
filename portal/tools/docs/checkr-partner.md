# Checkr Partner Tools

Provider: `checkr-partner` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Checkr Partner API. Checkr is a background checking platform for employment and tenant screening. **Requires Checkr Partner OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Checkr Partner
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.checkr.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `checkr_partner_list_candidates` | List candidates | GET | /api/v1/candidates |
| `checkr_partner_get_candidate` | Get candidate details | GET | /api/v1/candidates/{id} |
| `checkr_partner_create_candidate` | Create a candidate | POST | /api/v1/candidates |
| `checkr_partner_list_background_checks` | List background checks | GET | /api/v1/background_checks |
| `checkr_partner_get_background_check` | Get background check details | GET | /api/v1/background_checks/{id} |
| `checkr_partner_create_background_check` | Create a background check | POST | /api/v1/background_checks |
| `checkr_partner_list_reports` | List reports | GET | /api/v1/reports |
| `checkr_partner_get_report` | Get report details | GET | /api/v1/reports/{id} |
| `checkr_partner_list_invitations` | List invitations | GET | /api/v1/invitations |
| `checkr_partner_create_invitation` | Create an invitation | POST | /api/v1/invitations |

---

## Tool Details

### checkr_partner_list_candidates

**What it does**: Lists all candidates in the account.

**When to use**: Find candidates for background checks.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my Checkr candidates"

---

### checkr_partner_get_candidate

**What it does**: Gets details of a specific candidate.

**When to use**: View candidate profile and status.

**Arguments**:
- `id` (required): Candidate ID

**Example LLM prompt**: "Get candidate 123 details"

---

### checkr_partner_create_candidate

**What it does**: Creates a new candidate.

**When to use**: Add a new person for background screening.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address
- `zipcode` (optional): Zip code for location

**Example LLM prompt**: "Create a candidate John Doe with email john@example.com"

---

### checkr_partner_list_background_checks

**What it does**: Lists all background checks for a candidate.

**When to use**: Track screening status.

**Arguments**:
- `candidate_id` (required): Candidate ID

**Example LLM prompt**: "List background checks for candidate 123"

---

### checkr_partner_get_background_check

**What it does**: Gets details of a specific background check.

**When to use**: View screening progress and status.

**Arguments**:
- `id` (required): Background check ID

**Example LLM prompt**: "Get background check 456 details"

---

### checkr_partner_create_background_check

**What it does**: Creates a new background check for a candidate.

**When to use**: Initiate screening process.

**Arguments**:
- `candidate_id` (required): Candidate ID
- `package` (required): Background check package type

**Example LLM prompt**: "Create a background check for candidate 123 with package employee_basic"

---

### checkr_partner_list_reports

**What it does**: Lists all reports for a background check.

**When to use**: View screening results.

**Arguments**:
- `background_check_id` (required): Background check ID

**Example LLM prompt**: "List reports for background check 456"

---

### checkr_partner_get_report

**What it does**: Gets details of a specific report.

**When to use**: View detailed screening report.

**Arguments**:
- `id` (required): Report ID

**Example LLM prompt**: "Get report 789 details"

---

### checkr_partner_list_invitations

**What it does**: Lists all invitations sent to candidates.

**When to use**: Track pending invitations.

**Arguments**:
- `candidate_id` (optional): Filter by candidate

**Example LLM prompt**: "List invitations for candidate 123"

---

### checkr_partner_create_invitation

**What it does**: Sends an invitation to a candidate.

**When to use**: Invite a candidate to complete their information.

**Arguments**:
- `candidate_id` (required): Candidate ID
- `package` (required): Background check package

**Example LLM prompt**: "Send an invitation to candidate 123"

---

## Checkr Partner API Notes

- **Candidates**: Individuals being screened
- **Background Checks**: Screening requests with specific packages
- **Packages**: Different screening levels (criminal, employment, etc.)
- **Reports**: Final screening results
- **Invitations**: Requests to candidates to provide information
