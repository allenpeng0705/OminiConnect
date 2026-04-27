# Bullhorn CRM Tools

Provider: `bullhorn` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Bullhorn REST API for the recruiting CRM. They allow AI agents to manage candidates, job orders, placements, and sales opportunities.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `Candidates`, `JobOrders`, `Placements`, `Opportunities`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bullhorn_list_candidates` | List candidates in the recruiting CRM | GET | /candidate |
| `bullhorn_get_candidate` | Get details of a specific candidate | GET | /candidate/{id} |
| `bullhorn_create_candidate` | Create a new candidate | POST | /candidate |
| `bullhorn_list_jobs` | List job orders | GET | /jobOrder |
| `bullhorn_get_job` | Get details of a specific job | GET | /jobOrder/{id} |
| `bullhorn_create_job` | Create a new job order | POST | /jobOrder |
| `bullhorn_list_placements` | List placements (hires) | GET | /placement |
| `bullhorn_get_placement` | Get details of a specific placement | GET | /placement/{id} |
| `bullhorn_list_opportunities` | List sales opportunities | GET | /opportunity |
| `bullhorn_get_opportunity` | Get details of a specific opportunity | GET | /opportunity/{id} |

---

## Tool Details

### bullhorn_list_candidates

**What it does**: Returns a paginated list of candidates. Filter by status, skills, or date added.

**When to use**: Find candidates matching a role, check candidate pipeline, or research candidate backgrounds.

**Arguments**:
- `count` (optional): Number of results (default 20, max 200)
- `start` (optional): Offset for pagination
- `where` (optional): Bullhorn WHERE clause for filtering
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Show me all candidates added this week"

---

### bullhorn_get_candidate

**What it does**: Get full details of a specific candidate including contact info, work history, and skills.

**When to use**: Review a specific candidate before an interview or placement decision.

**Arguments**:
- `id` (required): Candidate ID
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Get details for candidate ID 12345"

---

### bullhorn_create_candidate

**What it does**: Create a new candidate record with contact info and initial details.

**When to use**: Add a new candidate discovered from sourcing, referral, or application.

**Arguments**:
- `firstName` (required): First name
- `lastName` (required): Last name
- `email` (optional): Email address
- `phone` (optional): Phone number
- `status` (optional): Candidate status (default: New)
- `notes` (optional): Initial notes

**Example LLM prompt**: "Create a new candidate named John Smith with email john@example.com"

---

### bullhorn_list_jobs

**What it does**: Returns a paginated list of job orders. Filter by status, title, or client.

**When to use**: Find open positions, check job pipeline, or match candidates to roles.

**Arguments**:
- `count` (optional): Number of results
- `start` (optional): Offset for pagination
- `where` (optional): Bullhorn WHERE clause
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "List all open job orders in the engineering department"

---

### bullhorn_get_job

**What it does**: Get full details of a specific job order including description, requirements, and client info.

**When to use**: Understand job requirements before sourcing candidates.

**Arguments**:
- `id` (required): Job order ID
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Get details for job order 5432"

---

### bullhorn_create_job

**What it does**: Create a new job order (open position) for a client.

**When to use**: Open a new req for a hiring manager request.

**Arguments**:
- `title` (required): Job title
- `clientCorporation` (required): Client corporation ID
- `description` (optional): Job description and requirements
- `status` (optional): Job status (default: Open)
- `address` (optional): Location object with city, state, countryID

**Example LLM prompt**: "Create a new job order titled Senior Engineer for client 100"

---

### bullhorn_list_placements

**What it does**: Returns a paginated list of placements (hired candidates). Filter by job, candidate, or date.

**When to use**: Track recent hires, report on placement success, or manage onboarding.

**Arguments**:
- `count` (optional): Number of results
- `start` (optional): Offset for pagination
- `where` (optional): Bullhorn WHERE clause
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Show me all placements made this month"

---

### bullhorn_get_placement

**What it does**: Get full details of a specific placement including candidate, job, start date, and status.

**When to use**: Review a specific placement for onboarding or billing purposes.

**Arguments**:
- `id` (required): Placement ID
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Get details for placement 789"

---

### bullhorn_list_opportunities

**What it does**: Returns a paginated list of sales opportunities. Filter by status, owner, or value.

**When to use**: Track pipeline deals, review sales forecasts, or monitor client engagements.

**Arguments**:
- `count` (optional): Number of results
- `start` (optional): Offset for pagination
- `where` (optional): Bullhorn WHERE clause
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Show me all open opportunities over $50k"

---

### bullhorn_get_opportunity

**What it does**: Get full details of a specific sales opportunity including company, value, stage, and notes.

**When to use**: Review a specific deal before a client call or to update status.

**Arguments**:
- `id` (required): Opportunity ID
- `fields` (optional): Comma-separated field list

**Example LLM prompt**: "Get details for opportunity 456"

---

## Bullhorn API Reference

These tools use the Bullhorn REST API. See official docs for full details:
- https://developer.bullhorn.com
- Rate limits: Follow your Bullhorn plan limits
- Pagination: Use `count` and `start` parameters
- All dates: ISO 8601 format
