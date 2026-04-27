# Greenhouse Tools

Provider: `greenhouse` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Greenhouse Recruiting API. They allow AI agents to manage jobs, applications, offers, and users in the Greenhouse applicant tracking system. Greenhouse is a popular ATS for hiring teams at companies of all sizes.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Greenhouse
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `applications:read`, `applications:write`, `jobs:read`, `jobs:write`, `offers:read`, `offers:write`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_list_applications` | List all job applications | GET | /applications |
| `greenhouse_get_application` | Get application details | GET | /applications/{application_id} |
| `greenhouse_update_application` | Update application status/fields | PUT | /applications/{application_id} |
| `greenhouse_list_jobs` | List all open jobs | GET | /jobs |
| `greenhouse_get_job` | Get job details | GET | /jobs/{job_id} |
| `greenhouse_create_job` | Create a new job posting | POST | /jobs |
| `greenhouse_list_offers` | List offers for candidates | GET | /offers |
| `greenhouse_get_offer` | Get offer details | GET | /offers/{offer_id} |
| `greenhouse_create_offer` | Create an offer for candidate | POST | /offers |
| `greenhouse_list_users` | List users (hiring team, recruiters) | GET | /users |

---

## Tool Details

### greenhouse_list_applications

**What it does**: Lists all job applications with optional filters for status, job, or date range. Returns candidate info, job applied to, current stage, and application status.

**When to use**: Browse all applications, find candidates by status or job, track applicant flow.

**Arguments**:
- `job_id` (optional): Filter by job ID
- `status` (optional): Filter by status: active, rejected, hired, etc.
- `limit` (optional): Number of results (default 100, max 500)
- `page` (optional): Page number for pagination

**Example LLM prompt**: "List all active applications for the Engineering manager position"

---

### greenhouse_get_application

**What it does**: Gets detailed information about a specific application including candidate profile, interview scorecards, offer details, and activity history.

**When to use**: Review full application details before making a hiring decision, check interview feedback.

**Arguments**:
- `application_id` (required): Application ID

**Example LLM prompt**: "Get details for application 12345"

---

### greenhouse_update_application

**What it does**: Updates an application's status, assigned recruiter, or custom fields. Use this to move candidates through hiring stages or reject/hire candidates.

**When to use**: Advance candidates through stages, reassign recruiters, add notes.

**Arguments**:
- `application_id` (required): Application ID
- `status` (optional): New status: active, rejected, hired, etc.
- `recruiter_id` (optional): Assign a recruiter by user ID
- `notes` (optional): Add a note to the application

**Example LLM prompt**: "Move application 12345 to the hired stage and assign recruiter 678"

---

### greenhouse_list_jobs

**What it does**: Lists all jobs with optional filters for status, department, or office location. Returns job titles, descriptions, status, and hiring team info.

**When to use**: Browse open positions, find jobs by department, check hiring status.

**Arguments**:
- `status` (optional): Filter by status: open, closed, draft
- `department_id` (optional): Filter by department ID
- `office_id` (optional): Filter by office ID
- `limit` (optional): Number of results
- `page` (optional): Page number

**Example LLM prompt**: "List all open Engineering jobs"

---

### greenhouse_get_job

**What it does**: Gets detailed information about a specific job including description, requirements, hiring team, interview plan, and application statistics.

**When to use**: Read full job details, check requirements, view hiring pipeline progress.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job 123 to see the hiring plan"

---

### greenhouse_create_job

**What it does**: Creates a new job posting with title, description, department, and office. Jobs are created in draft status by default and must be published to accept applications.

**When to use**: Create new job openings, set up positions before publishing.

**Arguments**:
- `title` (required): Job title
- `departments` (optional): Array of department IDs
- `offices` (optional): Array of office IDs
- `description` (optional): Job description (HTML)
- `status` (optional): Job status: open, closed, draft (default: draft)

**Example LLM prompt**: "Create a new job posting for Senior Software Engineer in the Engineering department"

---

### greenhouse_list_offers

**What it does**: Lists all offers with optional filters for status, candidate, or job. Returns offer amounts, statuses, start dates, and associated applications.

**When to use**: Track offer status, review pending offers, check compensation packages.

**Arguments**:
- `application_id` (optional): Filter by application ID
- `status` (optional): Filter by status: pending, approved, declined, etc.
- `limit` (optional): Number of results
- `page` (optional): Page number

**Example LLM prompt**: "List all pending offers"

---

### greenhouse_get_offer

**What it does**: Gets detailed information about a specific offer including compensation details, start date, approval status, and offer letter content.

**When to use**: Review offer details, verify compensation terms before approval.

**Arguments**:
- `offer_id` (required): Offer ID

**Example LLM prompt**: "Get details for offer 99999"

---

### greenhouse_create_offer

**What it does**: Creates an offer for a candidate who has completed the hiring process. Includes salary, start date, and other compensation details.

**When to use**: Extend an offer to a candidate after they have completed interviews.

**Arguments**:
- `application_id` (required): Application ID
- `payload` (required): Offer details object containing salary, start_date, etc.

**Example LLM prompt**: "Create an offer for candidate 123 with salary 120000 starting March 1st"

---

### greenhouse_list_users

**What it does**: Lists all users in the organization including recruiters, hiring managers, and admins. Returns names, email addresses, roles, and permission levels.

**When to use**: Find team members, assign recruiters to jobs, get hiring manager contact info.

**Arguments**:
- `email` (optional): Filter by email address
- `name` (optional): Search by name
- `limit` (optional): Number of results
- `page` (optional): Page number

**Example LLM prompt**: "List all recruiters in the company"

---

## Greenhouse API Notes

- **Application ID**: Unique identifier for a candidate's application to a specific job
- **Job Status**: Draft jobs are not visible to candidates - publish to open applications
- **Offers**: Linked to applications - create offer after candidate accepts job
- **Users**: Include recruiters, hiring managers, interviewers, and admins with different permissions
- **Stages**: Hiring pipeline stages are configurable per job - check job's stage configuration
- **Applications**: Link candidates to jobs - a candidate can have multiple applications for different jobs
