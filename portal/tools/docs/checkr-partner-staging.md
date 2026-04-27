# Checkr Partner (Staging) Tools

Provider: `checkr-partner-staging` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Checkr Partner Staging API. The staging environment allows testing background check integrations without processing real data. **Requires Checkr Partner Staging OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Checkr Partner Staging
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.checkr-staging.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `checkr_partner_staging_list_candidates` | List candidates | GET | /api/v1/candidates |
| `checkr_partner_staging_get_candidate` | Get candidate details | GET | /api/v1/candidates/{id} |
| `checkr_partner_staging_create_candidate` | Create a candidate | POST | /api/v1/candidates |
| `checkr_partner_staging_list_background_checks` | List background checks | GET | /api/v1/background_checks |
| `checkr_partner_staging_get_background_check` | Get background check details | GET | /api/v1/background_checks/{id} |
| `checkr_partner_staging_create_background_check` | Create a background check | POST | /api/v1/background_checks |
| `checkr_partner_staging_list_reports` | List reports | GET | /api/v1/reports |
| `checkr_partner_staging_get_report` | Get report details | GET | /api/v1/reports/{id} |
| `checkr_partner_staging_list_invitations` | List invitations | GET | /api/v1/invitations |
| `checkr_partner_staging_create_invitation` | Create an invitation | POST | /api/v1/invitations |

---

## Tool Details

### checkr_partner_staging_list_candidates

**What it does**: Lists all candidates in the staging account.

**When to use**: Test candidate listing.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List test candidates in staging"

---

### checkr_partner_staging_get_candidate

**What it does**: Gets details of a specific candidate.

**When to use**: Test candidate retrieval.

**Arguments**:
- `id` (required): Candidate ID

**Example LLM prompt**: "Get candidate 123 details"

---

### checkr_partner_staging_create_candidate

**What it does**: Creates a new candidate.

**When to use**: Test candidate creation.

**Arguments**:
- `first_name` (required): First name
- `last_name` (required): Last name
- `email` (required): Email address
- `zipcode` (optional): Zip code

**Example LLM prompt**: "Create a test candidate John Doe"

---

### checkr_partner_staging_list_background_checks

**What it does**: Lists all background checks for a candidate.

**When to use**: Test check listing.

**Arguments**:
- `candidate_id` (required): Candidate ID

**Example LLM prompt**: "List background checks for candidate 123"

---

### checkr_partner_staging_get_background_check

**What it does**: Gets details of a specific background check.

**When to use**: Test check retrieval.

**Arguments**:
- `id` (required): Background check ID

**Example LLM prompt**: "Get background check 456 details"

---

### checkr_partner_staging_create_background_check

**What it does**: Creates a new background check for a candidate.

**When to use**: Test check creation flow.

**Arguments**:
- `candidate_id` (required): Candidate ID
- `package` (required): Background check package

**Example LLM prompt**: "Create a test background check for candidate 123"

---

### checkr_partner_staging_list_reports

**What it does**: Lists all reports for a background check.

**When to use**: Test report listing.

**Arguments**:
- `background_check_id` (required): Background check ID

**Example LLM prompt**: "List reports for background check 456"

---

### checkr_partner_staging_get_report

**What it does**: Gets details of a specific report.

**When to use**: Test report retrieval.

**Arguments**:
- `id` (required): Report ID

**Example LLM prompt**: "Get report 789 details"

---

### checkr_partner_staging_list_invitations

**What it does**: Lists all invitations.

**When to use**: Test invitation listing.

**Arguments**:
- `candidate_id` (optional): Filter by candidate

**Example LLM prompt**: "List test invitations"

---

### checkr_partner_staging_create_invitation

**What it does**: Sends an invitation to a candidate.

**When to use**: Test invitation flow.

**Arguments**:
- `candidate_id` (required): Candidate ID
- `package` (required): Background check package

**Example LLM prompt**: "Send a test invitation to candidate 123"

---

## Checkr Partner Staging API Notes

- **Staging Environment**: Isolated test environment
- **No Real Data**: All data is simulated
- **Same API Structure**: Mirror of production API
- **OAuth Flow**: Same as production authentication
