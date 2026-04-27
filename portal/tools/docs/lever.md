# Lever Tools

Provider: `lever` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Lever API. They allow AI agents to manage opportunities, offers, users, and feedback in the Lever applicant tracking system. Lever is a modern ATS focused on candidate experience and collaborative hiring.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Lever
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `opportunities:read`, `opportunities:write`, `offers:read`, `offers:write`, `users:read`, `feedback:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lever_list_opportunities` | List candidate opportunities | GET | /opportunities |
| `lever_get_opportunity` | Get opportunity details | GET | /opportunities/{opportunity_id} |
| `lever_create_opportunity` | Create new opportunity | POST | /opportunities |
| `lever_update_opportunity` | Update opportunity fields | PUT | /opportunities/{opportunity_id} |
| `lever_list_offers` | List offers for opportunities | GET | /offers |
| `lever_get_offer` | Get offer details | GET | /offers/{offer_id} |
| `lever_create_offer` | Create an offer | POST | /offers |
| `lever_list_users` | List team members | GET | /users |
| `lever_get_user` | Get user profile | GET | /users/{user_id} |
| `lever_list_feedback` | List interview feedback | GET | /feedback |

---

## Tool Details

### lever_list_opportunities

**What it does**: Lists all candidate opportunities with optional filters for stage, status, or tags. Returns candidate info, current stage, owner, and timeline.

**When to use**: Browse pipeline, find candidates by stage, track hiring progress.

**Arguments**:
- `stage` (optional): Filter by stage name
- `status` (optional): Filter by status: active, archived, rejected
- `tags` (optional): Filter by tags (array)
- `limit` (optional): Number of results (default 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all opportunities in the Interview stage for the Engineering team"

---

### lever_get_opportunity

**What it does**: Gets detailed information about a specific opportunity including candidate profile, interview history, stage progression, offers, and all associated notes.

**When to use**: Review full candidate journey, check interview feedback and stage progression.

**Arguments**:
- `opportunity_id` (required): Opportunity ID

**Example LLM prompt**: "Get details for opportunity abc-123 to see the candidate's interview feedback"

---

### lever_create_opportunity

**What it does**: Creates a new opportunity for a candidate including name, email, stage, and owner. This is the first step in tracking a candidate through the hiring process.

**When to use**: Add new candidates to the pipeline, create records from job applications.

**Arguments**:
- `name` (required): Candidate name
- `emails` (optional): Array of email addresses
- `phone` (optional): Phone number
- `stage` (optional): Initial stage name (default: Applied)
- `owner` (optional): Owner user ID
- `tags` (optional): Array of tag names

**Example LLM prompt**: "Create a new opportunity for Sarah Chen who applied for the Product Manager role"

---

### lever_update_opportunity

**What it does**: Updates opportunity fields including stage, status, owner, or add notes and tags. Use this to move candidates through hiring stages or archive rejected candidates.

**When to use**: Move candidates through pipeline, update contact information, reassign to different recruiter.

**Arguments**:
- `opportunity_id` (required): Opportunity ID
- `stage` (optional): Move to a different stage
- `status` (optional): Update status: active, archived
- `owner` (optional): Assign to a different owner
- `tags` (optional): Update tags
- `notes` (optional): Add a note

**Example LLM prompt**: "Update opportunity abc-123 to move to the Offer stage"

---

### lever_list_offers

**What it does**: Lists all offers with optional filters for status, candidate, or date range. Returns offer amounts, statuses, conditions, and associated opportunities.

**When to use**: Track offer status, review pending offers, check compensation packages.

**Arguments**:
- `opportunity_id` (optional): Filter by opportunity ID
- `status` (optional): Filter by status: draft, pending, approved, declined, etc.
- `limit` (optional): Number of results
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all pending offers that need approval"

---

### lever_get_offer

**What it does**: Gets detailed information about a specific offer including compensation details, start date, approval chain, and status history.

**When to use**: Review offer details, verify compensation terms before sending to candidate.

**Arguments**:
- `offer_id` (required): Offer ID

**Example LLM prompt**: "Get details for offer def-456 to review the compensation package"

---

### lever_create_offer

**What it does**: Creates an offer for an opportunity including salary, equity, start date, and details. Offers go through an approval workflow before being sent to candidates.

**When to use**: Extend an offer to a candidate after they have completed interviews.

**Arguments**:
- `opportunity_id` (required): Opportunity ID
- `fields` (required): Offer fields object containing compensation, start_date, etc.

**Example LLM prompt**: "Create an offer for opportunity abc-123 with salary 130000 and March 15 start date"

---

### lever_list_users

**What it does**: Lists all team members including recruiters, hiring managers, and interviewers. Returns names, email addresses, titles, and team assignments.

**When to use**: Find team members, assign owners to opportunities, get interviewer contact info.

**Arguments**:
- `email` (optional): Filter by email
- `name` (optional): Search by name
- `limit` (optional): Number of results
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all hiring managers on the Engineering team"

---

### lever_get_user

**What it does**: Gets detailed information about a specific user including profile, role, teams they belong to, and their scheduling visibility.

**When to use**: Get user profile details, check interviewer availability and permissions.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user john@company.com to see their interview schedule"

---

### lever_list_feedback

**What it does**: Lists interview feedback for an opportunity including scores, comments, and interviewer info. Use this to review how candidates performed in their interviews.

**When to use**: Review interview feedback, summarize candidate performance for hiring decisions.

**Arguments**:
- `opportunity_id` (required): Opportunity ID
- `limit` (optional): Number of results
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "Get all feedback for opportunity abc-123 to see how the candidate did in interviews"

---

## Lever API Notes

- **Opportunity IDs**: Unique identifiers for candidate-job pairings - a candidate can have multiple opportunities for different jobs
- **Stages**: Configurable per organization - stages shown are your configured pipeline stages
- **Offers**: Linked to opportunities, contain compensation details and go through approval workflow
- **Feedback**: Interview feedback includes scores and comments from interviewers
- **Archives**: Candidates can be archived - archived opportunities are hidden by default
- **Users**: Include recruiters, hiring managers, interviewers with different roles and permissions
