# Sentry Tools

Provider: `sentry` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Sentry REST API. They allow AI agents to manage projects, issues, events, releases, and teams. Sentry is a leading error tracking and performance monitoring platform for applications.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Sentry
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `project:read`, `project:write`, `event:read`, `release:read`, `team:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sentry_list_projects` | List all projects | GET | /api/0/projects/ |
| `sentry_get_project` | Get project details | GET | /api/0/projects/{organizationSlug}/{projectSlug}/ |
| `sentry_create_project` | Create a new project | POST | /api/0/organizations/{organizationSlug}/projects/ |
| `sentry_list_issues` | List issues | GET | /api/0/issues/ |
| `sentry_get_issue` | Get issue details | GET | /api/0/issues/{issueId}/ |
| `sentry_list_events` | List events | GET | /api/0/projects/{organizationSlug}/{projectSlug}/events/ |
| `sentry_get_event` | Get event details | GET | /api/0/events/{eventId}/ |
| `sentry_list_releases` | List releases | GET | /api/0/releases/ |
| `sentry_get_release` | Get release details | GET | /api/0/releases/{organizationSlug}/{version}/ |
| `sentry_list_teams` | List teams | GET | /api/0/teams/ |

---

## Tool Details

### sentry_list_projects

**What it does**: Lists all Sentry projects in the organization.

**When to use**: See all projects, find a project to report errors or view issues.

**Arguments**:
- `organizationSlug` (optional): Organization slug

**Example LLM prompt**: "List all Sentry projects"

---

### sentry_get_project

**What it does**: Gets detailed information about a specific Sentry project including stats and issue counts.

**When to use**: Understand project configuration, error rates, and latest releases.

**Arguments**:
- `organizationSlug` (required): Organization slug
- `projectSlug` (required): Project slug

**Example LLM prompt**: "Get details for the myorg/api project"

---

### sentry_create_project

**What it does**: Creates a new project in a Sentry organization.

**When to use**: Set up error tracking for a new application or service.

**Arguments**:
- `organizationSlug` (required): Organization slug
- `name` (required): Project name
- `slug` (optional): Project slug (URL-friendly name)
- `team` (optional): Team slug to assign the project to
- `platform` (optional): Platform (e.g., python, javascript, ruby)

**Example LLM prompt**: "Create a new project called 'my-service' for myorg with platform javascript"

---

### sentry_list_issues

**What it does**: Lists Sentry issues with optional filtering by project, status, or assigned user.

**When to use**: Find unhandled errors, see issues assigned to you, search for specific error types.

**Arguments**:
- `project` (optional): Project slugs to filter
- `status` (optional): Filter by status (`unresolved`, `resolved`, `ignored`)
- `assigned` (optional): Filter by assigned user (`me`, `none`, or user ID)
- `query` (optional): Search query

**Example LLM prompt**: "List all unresolved issues in the frontend project assigned to me"

---

### sentry_get_issue

**What it does**: Gets detailed information about a specific Sentry issue including tags, user reports, and activity.

**When to use**: Investigate an error, see stack trace, understand affected users and frequency.

**Arguments**:
- `issueId` (required): Issue ID

**Example LLM prompt**: "Get details for issue abc123"

---

### sentry_list_events

**What it does**: Lists events (error occurrences) for a Sentry project or issue.

**When to use**: See when errors occurred, find patterns in crashes, get stack traces.

**Arguments**:
- `organizationSlug` (required): Organization slug
- `projectSlug` (required): Project slug
- `query` (optional): Search query
- `maxEvents` (optional): Maximum number of events (default 100)

**Example LLM prompt**: "List recent events in the myorg/frontend project"

---

### sentry_get_event

**What it does**: Gets detailed information about a specific Sentry event including stack trace, context, and breadcrumbs.

**When to use**: Debug an error, see the exact line that failed, understand the request context.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "Get event def456 details"

---

### sentry_list_releases

**What it does**: Lists all releases in an organization or project.

**When to use**: View deployment history, see which releases have errors, track code deployments.

**Arguments**:
- `organizationSlug` (required): Organization slug
- `projectSlug` (optional): Project slug (to filter by project)
- `query` (optional): Search query

**Example LLM prompt**: "List all releases for myorg"

---

### sentry_get_release

**What it does**: Gets detailed information about a specific release including commit data and deploys.

**When to use**: See what commits were in a release, check deploy status, link errors to deployments.

**Arguments**:
- `organizationSlug` (required): Organization slug
- `version` (required): Release version (e.g., 1.0.0 or projected SHA)

**Example LLM prompt**: "Get details for release 1.2.3 of myorg"

---

### sentry_list_teams

**What it does**: Lists all Sentry teams in the organization.

**When to use**: See team structure, find team members for assignment.

**Arguments**:
- `organizationSlug` (optional): Organization slug

**Example LLM prompt**: "List all teams in the organization"

---

## Sentry API Notes

- **Organization slugs**: Found in your Sentry URL (e.g., `acme` in `sentry.io/organizations/acme/`)
- **Project slugs**: Found in project settings URL
- **Issue IDs**: Long alphanumeric strings (e.g., `abc123def456ghi789`)
- **Event IDs**: Different from issue IDs - each occurrence has its own event ID
- **Release versions**: Can be semantic versions (1.0.0) or commit SHAs
- **Breadcrumbs**: Sentry records a trail of events leading up to each error
