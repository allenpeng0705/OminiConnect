# Databricks Tools

Provider: `databricks` | Engine: `nango` | Auth: OAuth / API_KEY (via Nango)

## Overview

These tools wrap the Databricks API. They allow AI agents to interact with Databricks workspaces, clusters, notebooks, jobs, and ML experiments on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `workspace_read`, `workspace_write`, `cluster_read`, `cluster_write`, `job_read`, `job_write`, `experiment_read`, `experiment_write`

**Native API_KEY (engine=omini_connect_native)**:
- Databricks API token stored in `client_secret` field
- Direct Databricks API passthrough

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `databricks_list_workspaces` | List workspaces | GET | /workspaces |
| `databricks_get_workspace` | Get workspace details | GET | /workspaces/{workspace_id} |
| `databricks_list_clusters` | List clusters | GET | /clusters |
| `databricks_get_cluster` | Get cluster details | GET | /clusters/{cluster_id} |
| `databricks_list_notebooks` | List notebooks in workspace | GET | /workspaces/{workspace_id}/notebooks |
| `databricks_get_notebook` | Get notebook content | GET | /workspaces/{workspace_id}/notebooks/{path} |
| `databricks_list_jobs` | List jobs | GET | /jobs |
| `databricks_get_job` | Get job details | GET | /jobs/{job_id} |
| `databricks_list_experiments` | List ML experiments | GET | /mlflow/experiments/list |
| `databricks_get_experiment` | Get experiment details | GET | /mlflow/experiments/{experiment_id} |

---

## Tool Details

### databricks_list_workspaces

**What it does**: Returns a list of all Databricks workspaces in the account with metadata including names and regions.

**When to use**: Discover available workspaces, find workspaces to explore, or understand the account structure.

**Arguments**: None

**Example LLM prompt**: "What Databricks workspaces are available?"

---

### databricks_get_workspace

**What it does**: Returns detailed information about a specific Databricks workspace including name, region, and storage configuration.

**When to use**: Get context about a workspace before exploring its clusters, jobs, or notebooks.

**Arguments**:
- `workspace_id` (required): Databricks workspace ID

**Example LLM prompt**: "Tell me about the ML Platform workspace"

---

### databricks_list_clusters

**What it does**: Lists all clusters in a Databricks workspace with their names, states, sizes, and Spark versions.

**When to use**: Discover available compute resources, check cluster availability, or find clusters for notebook execution.

**Arguments**:
- `workspace_id` (required): Databricks workspace ID

**Example LLM prompt**: "What clusters are running in the ML Platform workspace?"

---

### databricks_get_cluster

**What it does**: Returns detailed information about a specific Databricks cluster including state, configuration, and runtime.

**When to use**: Check cluster health, get cluster specifications, or verify cluster readiness.

**Arguments**:
- `cluster_id` (required): Databricks cluster ID

**Example LLM prompt**: "What are the specs of cluster 1234-567890-abc123?"

---

### databricks_list_notebooks

**What it does**: Lists all notebooks in a Databricks workspace or folder with their paths, names, and languages.

**When to use**: Discover available notebooks, browse workspace structure, or find notebooks to read.

**Arguments**:
- `workspace_id` (required): Databricks workspace ID
- `path` (optional): Folder path to list (default /)

**Example LLM prompt**: "What notebooks are in the /Production folder?"

---

### databricks_get_notebook

**What it does**: Returns the content of a Databricks notebook including all code cells and their outputs.

**When to use**: Read notebook code, understand analyses, or review existing work before modifying.

**Arguments**:
- `workspace_id` (required): Databricks workspace ID
- `path` (required): Notebook path

**Example LLM prompt**: "Show me the content of /Models/training-notebook"

---

### databricks_list_jobs

**What it does**: Lists all jobs in a Databricks workspace with their names, schedules, and last run statuses.

**When to use**: Discover available data pipelines, ML training jobs, or scheduled analytics.

**Arguments**:
- `workspace_id` (required): Databricks workspace ID

**Example LLM prompt**: "What jobs are defined in the ML Platform workspace?"

---

### databricks_get_job

**What it does**: Returns details about a specific Databricks job including settings, schedule, and run history.

**When to use**: Check job configuration, understand job triggers, or review job performance.

**Arguments**:
- `job_id` (required): Databricks job ID

**Example LLM prompt**: "Show me the details of job 456"

---

### databricks_list_experiments

**What it does**: Lists all MLflow experiments in a Databricks workspace with their names, IDs, and artifact locations.

**When to use**: Discover available machine learning experiments, find experiments for model tracking.

**Arguments**:
- `workspace_id` (required): Databricks workspace ID

**Example LLM prompt**: "What ML experiments exist in the ML Platform workspace?"

---

### databricks_get_experiment

**What it does**: Returns details about a specific MLflow experiment including metadata, runs, and artifact storage.

**When to use**: Understand experiment configuration, track ML runs, or analyze model performance.

**Arguments**:
- `experiment_id` (required): MLflow experiment ID

**Example LLM prompt**: "Get details of experiment 123"

---

## Databricks API Reference

These tools use the Databricks API. See official docs for full details:
- https://docs.databricks.com/en/dev-tools/api/
- Rate limits: Vary by endpoint and tier
- Authentication: Bearer token
- All dates: ISO 8601 format