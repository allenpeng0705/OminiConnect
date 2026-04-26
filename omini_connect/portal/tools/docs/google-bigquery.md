# Google BigQuery Tools

Provider: `google-bigquery` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the BigQuery API. They allow AI agents to manage datasets, tables, run SQL queries, and work with ML models. **Requires Google OAuth2 with BigQuery permissions.**

## Authentication

**Nango OAUTH2 (BigQuery)**:
- User authenticates via OAuth2 with BigQuery scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://bigquery.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_bigquery_list_datasets` | List datasets | GET | /bigquery/v2/projects/{projectId}/datasets |
| `google_bigquery_get_dataset` | Get dataset details | GET | /bigquery/v2/projects/{projectId}/datasets/{datasetId} |
| `google_bigquery_list_tables` | List tables in dataset | GET | /bigquery/v2/projects/{projectId}/datasets/{datasetId}/tables |
| `google_bigquery_get_table` | Get table details | GET | /bigquery/v2/projects/{projectId}/datasets/{datasetId}/tables/{tableId} |
| `google_bigquery_query` | Run a SQL query | POST | /bigquery/v2/projects/{projectId}/queries |
| `google_bigquery_list_jobs` | List query jobs | GET | /bigquery/v2/projects/{projectId}/jobs |
| `google_bigquery_get_job` | Get job details | GET | /bigquery/v2/projects/{projectId}/jobs/{jobId} |
| `google_bigquery_insert_query` | Insert a query job | POST | /bigquery/v2/projects/{projectId}/jobs |
| `google_bigquery_list_routines` | List routines | GET | /bigquery/v2/projects/{projectId}/datasets/{datasetId}/routines |
| `google_bigquery_list_models` | List ML models | GET | /bigquery/v2/projects/{projectId}/datasets/{datasetId}/models |

---

## Tool Details

### google_bigquery_list_datasets

**What it does**: Lists all BigQuery datasets in the project.

**When to use**: Browse available datasets.

**Arguments**:
- `projectId` (required): Google Cloud project ID

**Example LLM prompt**: "List all datasets in my project"

---

### google_bigquery_get_dataset

**What it does**: Gets detailed information about a dataset.

**When to use**: Get dataset metadata and table count.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `datasetId` (required): Dataset ID

**Example LLM prompt**: "Get details for dataset my_dataset"

---

### google_bigquery_list_tables

**What it does**: Lists all tables in a dataset.

**When to use**: See tables available for querying.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `datasetId` (required): Dataset ID

**Example LLM prompt**: "List tables in dataset my_dataset"

---

### google_bigquery_get_table

**What it does**: Gets detailed information about a table.

**When to use**: Get table schema and metadata.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `datasetId` (required): Dataset ID
- `tableId` (required): Table ID

**Example LLM prompt**: "Get schema for table my_dataset.my_table"

---

### google_bigquery_query

**What it does**: Runs a SQL query on BigQuery data.

**When to use**: Query data for analysis.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `query` (required): SQL query
- `useLegacySql` (optional): Use legacy SQL (default: false)

**Example LLM prompt**: "Run SELECT * FROM my_dataset.my_table LIMIT 10"

---

### google_bigquery_list_jobs

**What it does**: Lists recent query jobs.

**When to use**: Track query history.

**Arguments**:
- `projectId` (required): Google Cloud project ID

**Example LLM prompt**: "List recent query jobs"

---

### google_bigquery_get_job

**What it does**: Gets details of a query job.

**When to use**: Check job status and results.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `jobId` (required): Job ID

**Example LLM prompt**: "Get status for job abc123"

---

### google_bigquery_insert_query

**What it does**: Inserts a query job to run asynchronously.

**When to use**: Start a long-running query.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `query` (required): SQL query
- `destinationTable` (optional): Destination table for results

**Example LLM prompt**: "Insert a query job to save results to my_dataset.results"

---

### google_bigquery_list_routines

**What it does**: Lists routines (stored procedures, functions) in a dataset.

**When to use**: Find reusable SQL routines.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `datasetId` (required): Dataset ID

**Example LLM prompt**: "List routines in dataset my_dataset"

---

### google_bigquery_list_models

**What it does**: Lists ML models in a dataset.

**When to use**: Find BigQuery ML models.

**Arguments**:
- `projectId` (required): Google Cloud project ID
- `datasetId` (required): Dataset ID

**Example LLM prompt**: "List models in dataset my_dataset"

---

## BigQuery API Notes

- **Project ID**: Google Cloud project identifier
- **Dataset ID**: Dataset within a project
- **Table ID**: Table within a dataset
- **SQL**: Standard SQL by default (useLegacySql=false)
- **Jobs**: Asynchronous query execution
