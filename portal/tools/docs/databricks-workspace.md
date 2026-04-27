# Databricks Workspace Level Tools

Provider: `databricks-workspace` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Databricks Workspace API. They allow AI agents to manage notebooks, clusters, jobs, and MLflow models within a specific Databricks workspace.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine authentication
- Token stored in Nango, accessed via `connection_ref`
- Databricks instance configured per connection

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `databricks_workspace_list_notebooks` | List notebooks | GET | /2.0/workspace/list |
| `databricks_workspace_get_notebook` | Get notebook details | GET | /2.0/workspace/get |
| `databricks_workspace_create_notebook` | Create a notebook | POST | /2.0/workspace/mkdirs |
| `databricks_workspace_run_notebook` | Run a notebook | POST | /2.0/jobs/run-now |
| `databricks_workspace_list_clusters` | List workspace clusters | GET | /2.0/clusters/list |
| `databricks_workspace_get_cluster` | Get cluster details | GET | /2.0/clusters/get |
| `databricks_workspace_list_jobs` | List workspace jobs | GET | /2.0/jobs/list |
| `databricks_workspace_get_job` | Get job details | GET | /2.0/jobs/get |
| `databricks_workspace_list_models` | List MLflow models | GET | /2.0/mlflow/databricks/registered-models/list |
| `databricks_workspace_get_model` | Get model details | GET | /2.0/mlflow/databricks/registered-models/get |

---

## Tool Details

### databricks_workspace_list_notebooks

**What it does**: Lists all notebooks in the workspace folder structure.

**When to use**: Browse workspace files, navigate folder hierarchy, find specific notebooks.

**Arguments**:
- `path` (optional): Folder path (default: /)
- `limit` (optional): Max results (default 50, max 1000)

**Example LLM prompt**: "List all notebooks in the /Production folder"

---

### databricks_workspace_get_notebook

**What it does**: Gets notebook details including language and content.

**When to use**: Read notebook content, check notebook configuration, review code.

**Arguments**:
- `path` (required): Notebook path

**Example LLM prompt**: "Get the notebook at /Production/etl_pipeline"

---

### databricks_workspace_create_notebook

**What it does**: Creates a new notebook in the workspace.

**When to use**: Create new notebooks, set up new projects, add analysis files.

**Arguments**:
- `path` (required): Notebook path
- `language` (optional): Language - python, scala, r, or sql (default: python)

**Example LLM prompt**: "Create a new notebook at /Sandbox/test_notebook"

---

### databricks_workspace_run_notebook

**What it does**: Executes a notebook on a specified cluster.

**When to use**: Run ETL pipelines, execute analysis, trigger workflows.

**Arguments**:
- `notebook_task` (required): Notebook task with notebook_path
- `cluster_id` (optional): Cluster ID to run on

**Example LLM prompt**: "Run notebook /Production/etl_pipeline on cluster cl-123"

---

### databricks_workspace_list_clusters

**What it does**: Lists all clusters available in the workspace.

**When to use**: View compute resources, find available clusters, check cluster status.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all clusters"

---

### databricks_workspace_get_cluster

**What it does**: Gets detailed cluster configuration and status.

**When to use**: Check cluster health, verify configuration, monitor resource usage.

**Arguments**:
- `cluster_id` (required): Cluster ID

**Example LLM prompt**: "Get details for cluster cl-456"

---

### databricks_workspace_list_jobs

**What it does**: Lists all jobs in the workspace.

**When to use**: View scheduled workflows, find jobs by name, track job status.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all jobs in the workspace"

---

### databricks_workspace_get_job

**What it does**: Gets detailed job configuration including schedule and tasks.

**When to use**: Check job settings, review task dependencies, verify schedules.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job j-789"

---

### databricks_workspace_list_models

**What it does**: Lists all MLflow registered models in the workspace.

**When to use**: Browse model registry, find available models, track model versions.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all registered models"

---

### databricks_workspace_get_model

**What it does**: Gets detailed model information including all versions.

**When to use**: Check model metadata, review version history, get model stages.

**Arguments**:
- `name` (required): Model name

**Example LLM prompt**: "Get details for model production_classifier"

---

## Databricks Workspace API Notes

- **Workspace**: Isolated namespace for notebooks, clusters, and jobs
- **Notebooks**: Interactive documents with code, markdown, and visualizations
- **Clusters**: Compute resources - can be shared or job-specific
- **Jobs**: Scheduled or triggered workflows running notebooks or JARs
- **MLflow Models**: Model registry for versioning and deployment tracking
- **Instance Configuration**: Databricks instance URL (e.g., dbc-xxx.cloud.databricks.com)
