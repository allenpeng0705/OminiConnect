# Azure DevOps Tools

Provider: `azure_devops` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Azure DevOps REST API. They allow AI agents to manage projects, work items, repositories, builds, and releases. Azure DevOps is Microsoft's DevOps platform offering version control, CI/CD, project tracking, and more.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Azure DevOps
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `vso.project`, `vso.work`, `vso.code`, `vso.build`, `vso.release`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `azure_devops_list_projects` | List projects for an organization | GET | /api/v4/projects |
| `azure_devops_get_project` | Get details of a specific project | GET | /api/v4/projects/{project_id} |
| `azure_devops_list_work_items` | List work items in a project | GET | /api/v4/projects/{project_id}/workitems |
| `azure_devops_get_work_item` | Get a specific work item | GET | /api/v4/projects/{project_id}/workitems/{work_item_id} |
| `azure_devops_create_work_item` | Create a new work item | POST | /api/v4/projects/{project_id}/workitems |
| `azure_devops_list_repos` | List repositories in a project | GET | /api/v4/projects/{project_id}/repos |
| `azure_devops_get_repo` | Get a specific repository | GET | /api/v4/projects/{project_id}/repos/{repo_id} |
| `azure_devops_list_builds` | List build definitions or runs | GET | /api/v4/projects/{project_id}/builds |
| `azure_devops_get_build` | Get a specific build | GET | /api/v4/projects/{project_id}/builds/{build_id} |
| `azure_devops_list_releases` | List release definitions or runs | GET | /api/v4/projects/{project_id}/releases |

---

## Tool Details

### azure_devops_list_projects

**What it does**: Lists all projects in an Azure DevOps organization with optional filtering by state.

**When to use**: Find available projects, browse organization structure, check project status.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `$top` (optional): Number of projects to return (default 100, max 1000)
- `$skip` (optional): Number of projects to skip for pagination (default 0)
- `state` (optional): Filter by state (`wellFormed`, `createPending`, `deleting`, `changed`, `healthy`, `unhealthy`)

**Example LLM prompt**: "List all projects in the mycompany organization"

---

### azure_devops_get_project

**What it does**: Gets detailed information about a specific project including name, description, visibility, source control type, and process template.

**When to use**: Understand project configuration, check settings, get project ID for further operations.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name

**Example LLM prompt**: "Get details for the Backend-API project"

---

### azure_devops_list_work_items

**What it does**: Lists work items in a project with optional filtering by type, state, assignee, or area path.

**When to use**: Find bugs, tasks, user stories; track sprint progress; find work assigned to a team member.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `work_item_type` (optional): Filter by type (e.g., `Task`, `Bug`, `User Story`)
- `state` (optional): Filter by state
- `assigned_to` (optional): Filter by assignee email or name
- `area_path` (optional): Filter by area path
- `$top` (optional): Number of work items (default 100, max 200)
- `$skip` (optional): Number to skip (default 0)

**Example LLM prompt**: "List all open bugs in the Frontend project assigned to john@example.com"

---

### azure_devops_get_work_item

**What it does**: Gets detailed information about a specific work item including title, description, state, assignees, tags, and history.

**When to use**: Read full work item context before updating, commenting, or changing state.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `work_item_id` (required): Work item ID

**Example LLM prompt**: "Get details for work item #1234 to see the full bug description"

---

### azure_devops_create_work_item

**What it does**: Creates a new work item in a project with title, type, and optional description, assignees, and tags.

**When to use**: Log bugs, create tasks, capture user stories from conversation.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `work_item_type` (required): Work item type (e.g., `Task`, `Bug`, `User Story`)
- `title` (required): Work item title
- `description` (optional): Work item description
- `assigned_to` (optional): User email or display name
- `tags` (optional): Comma-separated list of tags

**Example LLM prompt**: "Create a new bug in the Backend-API project titled 'Fix login timeout issue'"

---

### azure_devops_list_repos

**What it does**: Lists all repositories in a project including Git and TFVC repositories.

**When to use**: Find available repositories, browse project code assets, check repository structure.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `$top` (optional): Number of repos to return (default 100)
- `$skip` (optional): Number to skip (default 0)

**Example LLM prompt**: "List all repositories in the Frontend project"

---

### azure_devops_get_repo

**What it does**: Gets detailed information about a specific repository including name, URL, default branch, and remote URL.

**When to use**: Get repository details before cloning, check repository settings, find remote URL.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `repo_id` (required): Repository ID or name

**Example LLM prompt**: "Get details for the main-api repository"

---

### azure_devops_list_builds

**What it does**: Lists build definitions or recent build runs with optional filtering by branch, status, or result.

**When to use**: Check CI status, find failed builds, review build history for a branch.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `definition_id` (optional): Filter by build definition ID
- `branch` (optional): Filter by source branch
- `status` (optional): Filter by status (`inProgress`, `completed`, `cancelling`, `paused`, `notStarted`)
- `result` (optional): Filter by result (`succeeded`, `partiallySucceeded`, `failed`, `canceled`, `none`)
- `$top` (optional): Number of builds (default 100)
- `$skip` (optional): Number to skip (default 0)

**Example LLM prompt**: "List the last 10 builds for the main branch"

---

### azure_devops_get_build

**What it does**: Gets detailed information about a specific build including build number, status, result, timing, and source branch.

**When to use**: Check build details, see which commits were included, review build errors.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `build_id` (required): Build ID

**Example LLM prompt**: "Get details for build #567 to see why it failed"

---

### azure_devops_list_releases

**What it does**: Lists release definitions or recent release deployments with optional filtering by status.

**When to use**: Track deployments, find release history, check deployment status across environments.

**Arguments**:
- `organization` (required): Azure DevOps organization name
- `project_id` (required): Project ID or name
- `definition_id` (optional): Filter by release definition ID
- `status` (optional): Filter by status (`drafts`, `active`, `completed`, `abandoned`)
- `$top` (optional): Number of releases (default 100)
- `$skip` (optional): Number to skip (default 0)

**Example LLM prompt**: "List all active releases in the Backend-API project"

---

## Azure DevOps API Notes

- **Project ID**: Can be the project name (string) or GUID
- **Work Item IDs**: Numeric identifiers unique within an organization
- **Repository IDs**: Can be the repository name (string) or GUID
- **Build/Release IDs**: Numeric identifiers for specific runs
- **Area Paths**: Hierarchical paths for organizing work items (e.g., `ProjectName\TeamName\Area`)
- **Tags**: Simple string labels for work items; comma-separated when creating
- **Source Branches**: Format like `refs/heads/main` or just `main`
- **Rate Limits**: Azure DevOps has rate limits; use pagination for large result sets
- **TFVC**: Azure DevOps also supports TFVC version control (not covered by these tools)
