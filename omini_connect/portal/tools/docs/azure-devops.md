# Azure DevOps Tools

Provider: `azure-devops` | Engine: `nango` | Auth: BASIC via Nango (Personal Access Token)

## Overview

These tools wrap the Azure DevOps REST API. They allow AI agents to manage projects, pipelines, work items, and repositories. Azure DevOps provides source control, CI/CD, project tracking, and more for software development teams.

## Authentication

**Nango BASIC Auth**:
- User provides Personal Access Token (PAT) as password
- Organization URL required (e.g., dev.azure.com/myorg or myorg.visualstudio.com)
- Token stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `azure_devops_list_projects` | List projects | GET | /_apis/projects |
| `azure_devops_get_project` | Get project details | GET | /_apis/projects/{projectId} |
| `azure_devops_list_pipelines` | List pipelines | GET | /{project}/_apis/pipelines |
| `azure_devops_get_pipeline` | Get pipeline details | GET | /{project}/_apis/pipelines/{pipelineId} |
| `azure_devops_list_runs` | List pipeline runs | GET | /{project}/_apis/pipelines/{pipelineId}/runs |
| `azure_devops_get_run` | Get run details | GET | /{project}/_apis/pipelines/{pipelineId}/runs/{runId} |
| `azure_devops_list_work_items` | List work items | GET | /{project}/_apis/wit/workitems |
| `azure_devops_get_work_item` | Get work item details | GET | /{project}/_apis/wit/workitems/{id} |
| `azure_devops_list_repos` | List repositories | GET | /{project}/_apis/git/repositories |
| `azure_devops_list_builds` | List builds | GET | /{project}/_apis/build/builds |

---

## Tool Details

### azure_devops_list_projects

**What it does**: Lists all projects in the Azure DevOps organization.

**When to use**: Browse available projects, select target for operations.

**Arguments**:
- `top` (optional): Max projects to return (default 20)
- `skip` (optional): Number to skip for pagination (default 0)

**Example LLM prompt**: "List all projects in my Azure DevOps organization"

---

### azure_devops_get_project

**What it does**: Gets details for a specific project.

**When to use**: Check project settings, visibility, or process template.

**Arguments**:
- `projectId` (required): Project ID or name

**Example LLM prompt**: "Get details for project 'MyProject'"

---

### azure_devops_list_pipelines

**What it does**: Lists all pipelines in a project.

**When to use**: Browse CI/CD pipelines, find specific pipeline.

**Arguments**:
- `project` (required): Project name or ID
- `top` (optional): Max pipelines to return (default 20)

**Example LLM prompt**: "List all pipelines in project 'Backend'"

---

### azure_devops_get_pipeline

**What it does**: Gets details for a specific pipeline.

**When to use**: Check pipeline configuration, triggers, or stages.

**Arguments**:
- `project` (required): Project name or ID
- `pipelineId` (required): Pipeline ID

**Example LLM prompt**: "Get details for pipeline ID 42"

---

### azure_devops_list_runs

**What it does**: Lists all runs (executions) for a pipeline.

**When to use**: View pipeline execution history, check status.

**Arguments**:
- `project` (required): Project name or ID
- `pipelineId` (required): Pipeline ID
- `top` (optional): Max runs to return (default 20)

**Example LLM prompt**: "List recent runs for pipeline 42"

---

### azure_devops_get_run

**What it does**: Gets details for a specific pipeline run.

**When to use**: Check run status, logs, or results.

**Arguments**:
- `project` (required): Project name or ID
- `pipelineId` (required): Pipeline ID
- `runId` (required): Run ID

**Example LLM prompt**: "Get details for run 100"

---

### azure_devops_list_work_items

**What it does**: Lists work items (bugs, tasks, features) in a project.

**When to use**: Track work, find tasks, manage backlog.

**Arguments**:
- `project` (required): Project name or ID
- `type` (optional): Work item type (Bug, Task, Feature, etc.)
- `state` (optional): Filter by state
- `top` (optional): Max items to return (default 20)

**Example LLM prompt**: "List all open bugs in project 'MyProject'"

---

### azure_devops_get_work_item

**What it does**: Gets details for a specific work item.

**When to use**: View task details, update work items.

**Arguments**:
- `project` (required): Project name or ID
- `id` (required): Work item ID

**Example LLM prompt**: "Get details for work item 123"

---

### azure_devops_list_repos

**What it does**: Lists all Git repositories in a project.

**When to use**: Browse source code repositories.

**Arguments**:
- `project` (required): Project name or ID

**Example LLM prompt**: "List all repositories in project 'MyProject'"

---

### azure_devops_list_builds

**What it does**: Lists build definitions and recent builds.

**When to use**: Monitor CI builds, check build status.

**Arguments**:
- `project` (required): Project name or ID
- `definitionId` (optional): Filter by build definition
- `top` (optional): Max builds to return (default 20)

**Example LLM prompt**: "List recent builds for project 'Backend'"

---

## Azure DevOps API Notes

- **Project**: The top-level organization unit in Azure DevOps
- **Pipelines**: Can be YAML-based or classic build pipelines
- **Work Items**: Include bugs, tasks, features, user stories, and more
- **Repositories**: Git-based source control
- **Builds**: CI/CD build definitions and execution history
