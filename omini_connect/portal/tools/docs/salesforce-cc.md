# Salesforce-cc Tools

Provider: `salesforce-cc` | Engine: `nango` | Auth: Client Credentials OAuth via Nango

## Overview

Salesforce Customer Data Platform (CDP) with Client Credentials authentication. These tools allow AI agents to manage data models, ingest jobs, connections, segments, and data operations.

## Authentication

**Nango OAuth Client Credentials**:
- Uses Client ID and Client Secret to authenticate
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `data_model:read`, `ingest:read`, `ingest:write`, `connections:read`, `connections:write`, `exports:read`, `segments:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesforce-cc_list_data_models` | List data models | GET | /v1/dataModels |
| `salesforce-cc_get_data_model` | Get data model details | GET | /v1/dataModels/{modelId} |
| `salesforce-cc_list_ingest_definitions` | List ingest definitions | GET | /v1/ingestDefinitions |
| `salesforce-cc_create_ingest_job` | Create ingest job | POST | /v1/ingestJobs |
| `salesforce-cc_list_ingest_jobs` | List ingest jobs | GET | /v1/ingestJobs |
| `salesforce-cc_get_ingest_job_status` | Get job status | GET | /v1/ingestJobs/{jobId} |
| `salesforce-cc_list_connections` | List connections | GET | /v1/connections |
| `salesforce-cc_create_connection` | Create connection | POST | /v1/connections |
| `salesforce-cc_list_exports` | List export jobs | GET | /v1/exports |
| `salesforce-cc_get_segment` | Get segment details | GET | /v1/segments/{segmentId} |

---

## Tool Details

### salesforce-cc_list_data_models

**What it does**: Returns a list of all data models.

**When to use**: Understand CDP data structure.

**Arguments**:
- `limit` (optional): Number of data models (default 50)

**Example LLM prompt**: "List all data models"

---

### salesforce-cc_get_data_model

**What it does**: Gets details of a specific data model.

**When to use**: Get model schema and fields.

**Arguments**:
- `modelId` (required): The data model ID

**Example LLM prompt**: "Get details for data model dm_abc123"

---

### salesforce-cc_list_ingest_definitions

**What it does**: Returns ingest definitions.

**When to use**: View data ingestion configurations.

**Arguments**:
- `limit` (optional): Number of definitions (default 50)

**Example LLM prompt**: "List all ingest definitions"

---

### salesforce-cc_create_ingest_job

**What it does**: Creates a new ingest job.

**When to use**: Load data into CDP.

**Arguments**:
- `ingestDefinitionId` (required): Ingest definition ID
- `data` (optional): Data records to ingest

**Example LLM prompt**: "Create an ingest job with definition ing_123"

---

### salesforce-cc_list_ingest_jobs

**What it does**: Returns ingest jobs.

**When to use**: Monitor data loading.

**Arguments**:
- `limit` (optional): Number of jobs (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all running ingest jobs"

---

### salesforce-cc_get_ingest_job_status

**What it does**: Gets status of an ingest job.

**When to use**: Check job completion.

**Arguments**:
- `jobId` (required): The job ID

**Example LLM prompt**: "Get status for job ij_xyz789"

---

### salesforce-cc_list_connections

**What it does**: Returns all connections.

**When to use**: View data integrations.

**Arguments**:
- `limit` (optional): Number of connections (default 50)

**Example LLM prompt**: "List all connections"

---

### salesforce-cc_create_connection

**What it does**: Creates a new connection.

**When to use**: Set up data integration.

**Arguments**:
- `name` (required): Connection name
- `type` (required): Connection type

**Example LLM prompt**: "Create a connection called 'My Data Source'"

---

### salesforce-cc_list_exports

**What it does**: Returns export jobs.

**When to use**: View data exports.

**Arguments**:
- `limit` (optional): Number of exports (default 50)

**Example LLM prompt**: "List all export jobs"

---

### salesforce-cc_get_segment

**What it does**: Gets segment details.

**When to use**: Get audience segment information.

**Arguments**:
- `segmentId` (required): The segment ID

**Example LLM prompt**: "Get details for segment seg_123"

---

## Salesforce CDP Notes

- CDP unifies customer data from multiple sources
- Data models define how data is structured
- Ingest jobs bring data into the platform
- Connections link external data sources
- Segments are audience groups for activation
