# Functionize Tools

Provider: `functionize` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Functionize REST API. They allow AI agents to manage tests, test runs, results, suites, and environments — Functionize is an AI-powered end-to-end testing platform that uses machine learning to automate test creation, execution, and maintenance.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Functionize
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `tests:read`, `tests:write`, `tests:execute`, `runs:read`, `results:read`, `suites:read`, `environments:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `functionize_list_tests` | List tests | GET | /api/v1/tests |
| `functionize_get_test` | Get test details | GET | /api/v1/tests/{testId} |
| `functionize_create_test` | Create a test | POST | /api/v1/tests |
| `functionize_run_test` | Run a test | POST | /api/v1/tests/{testId}/run |
| `functionize_list_test_runs` | List test runs | GET | /api/v1/runs |
| `functionize_get_test_run` | Get test run details | GET | /api/v1/runs/{runId} |
| `functionize_list_results` | List results | GET | /api/v1/results |
| `functionize_get_result` | Get result details | GET | /api/v1/results/{resultId} |
| `functionize_list_suites` | List test suites | GET | /api/v1/suites |
| `functionize_list_environments` | List environments | GET | /api/v1/environments |

---

## Tool Details

### functionize_list_tests

**What it does**: Lists all tests with optional project, suite, and status filters.

**When to use**: Browse available tests, find tests in a specific project or suite.

**Arguments**:
- `projectId` (optional): Filter by project ID
- `suiteId` (optional): Filter by suite ID
- `status` (optional): Filter by status (Active, Disabled)

**Example LLM prompt**: "List all tests in project abc-123"

---

### functionize_get_test

**What it does**: Gets detailed test info — name, URL, steps, status, last run date.

**When to use**: Understand test scope before running or updating.

**Arguments**:
- `testId` (required): Test ID

**Example LLM prompt**: "Get details for test xyz-456"

---

### functionize_create_test

**What it does**: Creates a new test with specified name, URL, and optional steps.

**When to use**: Add new tests for web pages or user flows.

**Arguments**:
- `name` (required): Test name
- `url` (required): Target URL
- `steps` (optional): Array of test steps
- `projectId` (optional): Project ID
- `suiteId` (optional): Suite ID

**Example LLM prompt**: "Create a test called 'Homepage load test' for https://example.com"

---

### functionize_run_test

**What it does**: Triggers a test run on the specified environment.

**When to use**: Execute tests manually, run tests before deployment.

**Arguments**:
- `testId` (required): Test ID
- `environmentId` (optional): Environment ID to run against

**Example LLM prompt**: "Run test xyz-456 on staging environment"

---

### functionize_list_test_runs

**What it does**: Lists test runs with optional test, status, and date filters.

**When to use**: Check test execution history, find recent runs.

**Arguments**:
- `testId` (optional): Filter by test ID
- `status` (optional): Filter by run status
- `startDate` (optional): Start date filter (YYYY-MM-DD)
- `endDate` (optional): End date filter (YYYY-MM-DD)

**Example LLM prompt**: "List all test runs in the last week"

---

### functionize_get_test_run

**What it does**: Gets detailed test run info — test ID, start time, end time, status, environment.

**When to use**: Review test execution details and status.

**Arguments**:
- `runId` (required): Test run ID

**Example LLM prompt**: "Get details for test run abc-run-1"

---

### functionize_list_results

**What it does**: Lists test results with optional run, test, and status filters.

**When to use**: Check test pass/fail history, find recent failures.

**Arguments**:
- `runId` (optional): Filter by run ID
- `testId` (optional): Filter by test ID
- `status` (optional): Filter by result status (Passed, Failed)

**Example LLM prompt**: "List all failed results in the last week"

---

### functionize_get_result

**What it does**: Gets detailed result info — test ID, run date, status, error messages, screenshots.

**When to use**: Investigate test failures, see what went wrong.

**Arguments**:
- `resultId` (required): Result ID

**Example LLM prompt**: "Get result details for result abc-789"

---

### functionize_list_suites

**What it does**: Lists test suites with optional project filter.

**When to use**: Browse test suites, organize tests by functional area.

**Arguments**:
- `projectId` (optional): Filter by project ID

**Example LLM prompt**: "List all test suites in project abc-123"

---

### functionize_list_environments

**What it does**: Lists all test environments configured in Functionize.

**When to use**: See available environments for test execution.

**Arguments**: None

**Example LLM prompt**: "What environments are configured for testing"

---

## Functionize API Notes

- **IDs**: Functionize uses string IDs for all objects
- **Tests**: Automated test scenarios that visit URLs and verify functionality
- **Test Runs**: Execution records of tests with start/end times and status
- **Results**: Individual step results within a test run with pass/fail status
- **Suites**: Groups of tests that can be run together
- **Environments**: Target environments (dev, staging, prod) for test execution
- **AI Features**: Functionize uses AI to automatically maintain tests when application UI changes
- **API Version**: Uses `/api/v1/` prefix for all endpoints