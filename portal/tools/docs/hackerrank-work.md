# HackerRank Work Tools

Provider: `hackerrank-work` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the HackerRank for Work API. They allow AI agents to manage candidates, tests, jobs, and interviews. HackerRank for Work is a technical hiring platform.

## Authentication

**Nango Basic Auth**:
- User provides API key via Nango Connect
- Uses API key as username with empty password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://www.hackerrank.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `hackerrank_list_candidates` | List candidates | GET | /x/api/v3/users |
| `hackerrank_get_candidate` | Get candidate details | GET | /x/api/v3/users/{id} |
| `hackerrank_list_tests` | List tests | GET | /x/api/v3/tests |
| `hackerrank_get_test` | Get test details | GET | /x/api/v3/tests/{id} |
| `hackerrank_list_jobs` | List jobs | GET | /x/api/v3/jobs |
| `hackerrank_get_job` | Get job details | GET | /x/api/v3/jobs/{id} |
| `hackerrank_list_interviews` | List interviews | GET | /x/api/v3/interviews |
| `hackerrank_get_interview` | Get interview details | GET | /x/api/v3/interviews/{id} |
| `hackerrank_list_users` | List users | GET | /x/api/v3/teams |
| `hackerrank_get_user` | Get user details | GET | /x/api/v3/teams/{id} |

---

## Tool Details

### hackerrank_list_candidates

**What it does**: Lists all candidates in HackerRank for Work.

**When to use**: Browse candidate pool.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `offset` (optional): Offset (default 0)

**Example LLM prompt**: "List all candidates"

---

### hackerrank_get_candidate

**What it does**: Gets detailed information about a specific candidate.

**When to use**: View candidate assessments and history.

**Arguments**:
- `id` (required): Candidate ID

**Example LLM prompt**: "Get candidate with ID 123"

---

### hackerrank_list_tests

**What it does**: Lists all tests in HackerRank for Work.

**When to use**: Browse available tests.

**Arguments**: None

**Example LLM prompt**: "List all tests"

---

### hackerrank_get_test

**What it does**: Gets detailed information about a specific test.

**When to use**: View test structure and questions.

**Arguments**:
- `id` (required): Test ID

**Example LLM prompt**: "Get test with ID 456"

---

### hackerrank_list_jobs

**What it does**: Lists all jobs in HackerRank for Work.

**When to use**: Browse job openings.

**Arguments**: None

**Example LLM prompt**: "List all jobs"

---

### hackerrank_get_job

**What it does**: Gets detailed information about a specific job.

**When to use**: View job details and associated tests.

**Arguments**:
- `id` (required): Job ID

**Example LLM prompt**: "Get job with ID 789"

---

### hackerrank_list_interviews

**What it does**: Lists all interviews in HackerRank for Work.

**When to use**: View scheduled interviews.

**Arguments**: None

**Example LLM prompt**: "List all interviews"

---

### hackerrank_get_interview

**What it does**: Gets detailed information about a specific interview.

**When to use**: View interview details and feedback.

**Arguments**:
- `id` (required): Interview ID

**Example LLM prompt**: "Get interview with ID 101"

---

### hackerrank_list_users

**What it does**: Lists all users in HackerRank for Work.

**When to use**: View team members.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### hackerrank_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: View user profile and permissions.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user with ID 202"

---

## HackerRank Work API Notes

- **API Base URL**: https://www.hackerrank.com
- **Auth Mode**: Basic Auth with API key as username
- **Candidates**: Job candidates with assessments
- **Tests**: Technical coding assessments
- **Jobs**: Job openings with associated tests
- **Interviews**: Scheduled technical interviews
- **Users**: Team members with access to the platform
