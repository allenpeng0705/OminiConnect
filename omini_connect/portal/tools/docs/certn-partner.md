# Certn Partner Tools

Provider: `certn-partner` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Certn Partner API. Certn Partner provides white-label background check services. **Requires Certn Partner OAuth2 Client Credentials.**

## Authentication

**Nango OAuth2_CC**:
- Uses Client Credentials flow for partner integration
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.certn.co`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `certn_partner_list_teams` | List teams | GET | /api/v2/teams |
| `certn_partner_get_team` | Get team details | GET | /api/v2/teams/{id} |
| `certn_partner_list_applicants` | List applicants | GET | /api/v2/teams/{teamId}/applicants |
| `certn_partner_get_applicant` | Get applicant details | GET | /api/v2/applicants/{id} |
| `certn_partner_create_applicant` | Create an applicant | POST | /api/v2/teams/{teamId}/applicants |
| `certn_partner_list_background_checks` | List background checks | GET | /api/v2/applicants/{applicantId}/background-checks |
| `certn_partner_get_background_check` | Get background check details | GET | /api/v2/background-checks/{id} |
| `certn_partner_list_reports` | List reports | GET | /api/v2/background-checks/{checkId}/reports |
| `certn_partner_get_report` | Get report details | GET | /api/v2/reports/{id} |
| `certn_partner_list_invites` | List invites | GET | /api/v2/teams/{teamId}/invites |

---

## Tool Details

### certn_partner_list_teams

**What it does**: Lists all teams in the partner account.

**When to use**: Find teams for managing applicants.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List my Certn Partner teams"

---

### certn_partner_get_team

**What it does**: Gets details of a specific team.

**When to use**: View team settings and members.

**Arguments**:
- `id` (required): Team ID

**Example LLM prompt**: "Get team 123 details"

---

### certn_partner_list_applicants

**What it does**: Lists all applicants in a team.

**When to use**: View candidates for screening.

**Arguments**:
- `teamId` (required): Team ID
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List applicants for team 123"

---

### certn_partner_get_applicant

**What it does**: Gets details of a specific applicant.

**When to use**: View applicant profile and screening status.

**Arguments**:
- `id` (required): Applicant ID

**Example LLM prompt**: "Get applicant 456 details"

---

### certn_partner_create_applicant

**What it does**: Creates a new applicant in a team.

**When to use**: Add candidates for screening.

**Arguments**:
- `teamId` (required): Team ID
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address

**Example LLM prompt**: "Create an applicant John Doe with email john@example.com"

---

### certn_partner_list_background_checks

**What it does**: Lists all background checks for an applicant.

**When to use**: Track screening progress.

**Arguments**:
- `applicantId` (required): Applicant ID

**Example LLM prompt**: "List background checks for applicant 456"

---

### certn_partner_get_background_check

**What it does**: Gets details of a specific background check.

**When to use**: View check status and details.

**Arguments**:
- `id` (required): Background check ID

**Example LLM prompt**: "Get background check 789 details"

---

### certn_partner_list_reports

**What it does**: Lists all reports for a background check.

**When to use**: View screening results.

**Arguments**:
- `checkId` (required): Background check ID

**Example LLM prompt**: "List reports for background check 789"

---

### certn_partner_get_report

**What it does**: Gets details of a specific report.

**When to use**: View detailed screening report.

**Arguments**:
- `id` (required): Report ID

**Example LLM prompt**: "Get report 101 details"

---

### certn_partner_list_invites

**What it does**: Lists all invites for a team.

**When to use**: Track pending applicant invitations.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "List invites for team 123"

---

## Certn Partner API Notes

- **Client Credentials**: Uses OAuth2 client credentials flow
- **Teams**: Organizational units for managing screening
- **Applicants**: Candidates undergoing background checks
- **Background Checks**: Screening requests for criminal, employment, etc.
- **Reports**: Final screening results with findings
