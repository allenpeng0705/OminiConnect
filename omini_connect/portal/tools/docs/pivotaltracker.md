# Pivotal Tracker Tools

Provider: `pivotaltracker` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Pivotal Tracker API. They allow AI agents to manage projects, stories, tasks, iterations, and team members. Pivotal Tracker is an agile project management platform. **Requires Pivotal Tracker API Key authentication.**

## Authentication

**Nango API Key**:
- Uses X-TrackerToken header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://www.pivotaltracker.com/services/v5

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pivotaltracker_list_projects` | List projects | GET | /projects |
| `pivotaltracker_get_project` | Get project details | GET | /projects/{projectId} |
| `pivotaltracker_list_stories` | List stories | GET | /projects/{projectId}/stories |
| `pivotaltracker_get_story` | Get story details | GET | /projects/{projectId}/stories/{storyId} |
| `pivotaltracker_list_tasks` | List tasks | GET | /projects/{projectId}/stories/{storyId}/tasks |
| `pivotaltracker_get_task` | Get task details | GET | /projects/{projectId}/stories/{storyId}/tasks/{taskId} |
| `pivotaltracker_list_members` | List project members | GET | /projects/{projectId}/memberships |
| `pivotaltracker_get_iteration` | Get iteration details | GET | /projects/{projectId}/iterations/{iterationId} |
| `pivotaltracker_list_iterations` | List iterations | GET | /projects/{projectId}/iterations |
| `pivotaltracker_get_account` | Get account info | GET | /accounts |

---

## Tool Details

### pivotaltracker_list_projects

**What it does**: Lists all projects.

**When to use**: Browse available projects.

**Arguments**: None

**Example LLM prompt**: "List all my projects"

---

### pivotaltracker_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: Get project details, settings.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project 12345"

---

### pivotaltracker_list_stories

**What it does**: Lists all stories in a project.

**When to use**: Browse backlog, track progress.

**Arguments**:
- `projectId` (required): Project ID
- `storyType` (optional): Filter by type (feature, bug, chore, release)
- `state` (optional): Filter by state

**Example LLM prompt**: "List all features in project 12345"

---

### pivotaltracker_get_story

**What it does**: Gets detailed information about a specific story.

**When to use**: Get story details, comments.

**Arguments**:
- `projectId` (required): Project ID
- `storyId` (required): Story ID

**Example LLM prompt**: "Get details for story 67890"

---

### pivotaltracker_list_tasks

**What it does**: Lists all tasks in a story.

**When to use**: Browse tasks, track completion.

**Arguments**:
- `projectId` (required): Project ID
- `storyId` (required): Story ID

**Example LLM prompt**: "List tasks for story 67890"

---

### pivotaltracker_get_task

**What it does**: Gets detailed information about a specific task.

**When to use**: Get task details, status.

**Arguments**:
- `projectId` (required): Project ID
- `storyId` (required): Story ID
- `taskId` (required): Task ID

**Example LLM prompt**: "Get details for task 11111"

---

### pivotaltracker_list_members

**What it does**: Lists all members in a project.

**When to use**: View project team.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List members of project 12345"

---

### pivotaltracker_get_iteration

**What it does**: Gets detailed information about a specific iteration.

**When to use**: Get iteration stats, stories.

**Arguments**:
- `projectId` (required): Project ID
- `iterationId` (required): Iteration ID

**Example LLM prompt**: "Get details for iteration 22222"

---

### pivotaltracker_list_iterations

**What it does**: Lists all iterations in a project.

**When to use**: Browse sprint history.

**Arguments**:
- `projectId` (required): Project ID
- `limit` (optional): Number of iterations (default 10)

**Example LLM prompt**: "List recent iterations for project 12345"

---

### pivotaltracker_get_account

**What it does**: Gets account information.

**When to use**: Get account details.

**Arguments**: None

**Example LLM prompt**: "Get my account information"

---

## Pivotal Tracker API Notes

- **API Key**: Uses X-TrackerToken header
- **Agile Workflows**: Stories, tasks, iterations
- **Project-based**: All work organized in projects
