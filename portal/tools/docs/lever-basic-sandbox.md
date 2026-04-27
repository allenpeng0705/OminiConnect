# Lever Basic Auth Sandbox Tools

Provider: `lever-basic-sandbox` | Engine: `nango` | Auth: BASIC via Nango (Sandbox)

## Overview

These tools wrap the Lever Sandbox API. They allow AI agents to manage opportunities, offers, jobs, and hiring workflows in a testing environment. **Requires Lever Sandbox API key.**

## Authentication

**Basic Auth via Nango**:
- User provides Lever Sandbox API key (as username)
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.sandbox.lever.co`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lever_basic_sandbox_list_opportunities` | List opportunities | GET | /v1/opportunities |
| `lever_basic_sandbox_get_opportunity` | Get opportunity details | GET | /v1/opportunities/{opportunity_id} |
| `lever_basic_sandbox_list_offers` | List offers | GET | /v1/offers |
| `lever_basic_sandbox_get_offer` | Get offer details | GET | /v1/offers/{offer_id} |
| `lever_basic_sandbox_list_users` | List users | GET | /v1/users |
| `lever_basic_sandbox_get_user` | Get user details | GET | /v1/users/{user_id} |
| `lever_basic_sandbox_list_jobs` | List jobs | GET | /v1/jobs |
| `lever_basic_sandbox_get_job` | Get job details | GET | /v1/jobs/{job_id} |
| `lever_basic_sandbox_list_stages` | List stages | GET | /v1/stages |
| `lever_basic_sandbox_list_notes` | List notes | GET | /v1/notes |

---

## Tool Details

### lever_basic_sandbox_list_opportunities

**What it does**: Lists all opportunities (candidates) in Lever Sandbox.

**When to use**: Find candidates, view pipeline.

**Arguments**:
- `stage` (optional): Filter by stage
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all opportunities in Lever Sandbox"

---

### lever_basic_sandbox_get_opportunity

**What it does**: Gets details for a specific opportunity.

**When to use**: Get candidate information.

**Arguments**:
- `opportunity_id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity abc123"

---

### lever_basic_sandbox_list_offers

**What it does**: Lists all offers.

**When to use**: View offers, track hiring decisions.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all offers in Lever Sandbox"

---

### lever_basic_sandbox_get_offer

**What it does**: Gets details for a specific offer.

**When to use**: Get offer information.

**Arguments**:
- `offer_id` (required): Offer ID

**Example LLM prompt**: "Get details for offer xyz789"

---

### lever_basic_sandbox_list_users

**What it does**: Lists all users (team members).

**When to use**: View team, find recruiters.

**Arguments**: None

**Example LLM prompt**: "List all users in Lever Sandbox"

---

### lever_basic_sandbox_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u1"

---

### lever_basic_sandbox_list_jobs

**What it does**: Lists all jobs.

**When to use**: View open positions.

**Arguments**:
- `status` (optional): Job status (open, closed)

**Example LLM prompt**: "List all open jobs in Lever Sandbox"

---

### lever_basic_sandbox_get_job

**What it does**: Gets details for a specific job.

**When to use**: Get job information.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job j1"

---

### lever_basic_sandbox_list_stages

**What it does**: Lists all stages.

**When to use**: View hiring pipeline stages.

**Arguments**: None

**Example LLM prompt**: "List all stages in Lever Sandbox"

---

### lever_basic_sandbox_list_notes

**What it does**: Lists all notes.

**When to use**: View notes, find comments.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all notes in Lever Sandbox"

---

## Lever Sandbox API Notes

- **Sandbox Environment**: Testing version of Lever
- **Same functionality as production** but with test data
- **Opportunities**: Candidates in the pipeline
- **Offers**: Job offers extended to candidates
- **Jobs**: Open positions
- **Stages**: Pipeline stages for hiring
