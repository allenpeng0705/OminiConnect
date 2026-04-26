# Greenhouse Ingestion Tools

Provider: `greenhouse-ingestion` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Greenhouse Ingestion API. They allow AI agents to ingest candidates, jobs, applications, and offers into Greenhouse. The Ingestion API enables automated data synchronization.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Greenhouse
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://api.greenhouse.io/oauth/authorize
- Token URL: https://api.greenhouse.io/oauth/token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_ingest_candidate` | Ingest a candidate | POST | /ingestion/candidates |
| `greenhouse_ingest_job` | Ingest a job | POST | /ingestion/jobs |
| `greenhouse_ingest_application` | Ingest an application | POST | /ingestion/applications |
| `greenhouse_ingest_offer` | Ingest an offer | POST | /ingestion/offers |
| `greenhouse_ingest_department` | Ingest a department | POST | /ingestion/departments |
| `greenhouse_ingest_office` | Ingest an office | POST | /ingestion/offices |
| `greenhouse_ingest_hiring_team` | Ingest a hiring team | POST | /ingestion/hiring_teams |
| `greenhouse_list_candidates` | List candidates | GET | /candidates |
| `greenhouse_list_jobs` | List jobs | GET | /jobs |
| `greenhouse_list_applications` | List applications | GET | /applications |

---

## Tool Details

### greenhouse_ingest_candidate

**What it does**: Ingests a candidate into Greenhouse.

**When to use**: Sync candidate data from external systems.

**Arguments**:
- `email` (required): Candidate email
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `phone` (optional): Phone number
- `company` (optional): Current company
- `title` (optional): Current title

**Example LLM prompt**: "Ingest a candidate for john@example.com"

---

### greenhouse_ingest_job

**What it does**: Ingests a job into Greenhouse.

**When to use**: Sync job data from external systems.

**Arguments**:
- `title` (required): Job title
- `description` (optional): Job description
- `department` (optional): Department name
- `office` (optional): Office location

**Example LLM prompt**: "Ingest a job for 'Software Engineer'"

---

### greenhouse_ingest_application

**What it does**: Ingests an application into Greenhouse.

**When to use**: Link candidates to jobs.

**Arguments**:
- `candidate_email` (required): Candidate email
- `job_title` (required): Job title
- `status` (optional): Application status

**Example LLM prompt**: "Ingest an application for john@example.com for 'Software Engineer'"

---

### greenhouse_ingest_offer

**What it does**: Ingests an offer into Greenhouse.

**When to use**: Record offer data.

**Arguments**:
- `candidate_email` (required): Candidate email
- `job_title` (required): Job title
- `salary` (optional): Base salary
- `start_date` (optional): Proposed start date

**Example LLM prompt**: "Ingest an offer for john@example.com"

---

### greenhouse_ingest_department

**What it does**: Ingests a department into Greenhouse.

**When to use**: Sync department structure.

**Arguments**:
- `name` (required): Department name
- `parent_name` (optional): Parent department name

**Example LLM prompt**: "Ingest a department called 'Engineering'"

---

### greenhouse_ingest_office

**What it does**: Ingests an office into Greenhouse.

**When to use**: Sync office locations.

**Arguments**:
- `name` (required): Office name
- `location` (optional): Office location
- `department` (optional): Department name

**Example LLM prompt**: "Ingest an office called 'New York'"

---

### greenhouse_ingest_hiring_team

**What it does**: Ingests a hiring team into Greenhouse.

**When to use**: Add team members to jobs.

**Arguments**:
- `job_title` (required): Job title
- `user_email` (required): User email
- `role` (optional): Hiring team role

**Example LLM prompt**: "Ingest hiring team member jane@example.com for 'Software Engineer'"

---

### greenhouse_list_candidates

**What it does**: Lists all candidates in Greenhouse.

**When to use**: Browse existing candidates.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all candidates"

---

### greenhouse_list_jobs

**What it does**: Lists all jobs in Greenhouse.

**When to use**: Browse existing jobs.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all jobs"

---

### greenhouse_list_applications

**What it does**: Lists all applications in Greenhouse.

**When to use**: Browse existing applications.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all applications"

---

## Greenhouse Ingestion API Notes

- **Base URL**: https://api.greenhouse.io
- **Auth Mode**: OAuth2
- **Ingestion**: Bulk data import from external systems
- **Matching**: Uses email and title for entity matching
- **Updates**: Can update existing records
