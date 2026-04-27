# Salesforce-cdp Tools

Provider: `salesforce-cdp` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Salesforce Customer Data Platform (CDP) unifies customer data from multiple sources. These tools allow AI agents to manage data models, ingest jobs, connections, segments, and analytics.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Salesforce CDP
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `data_model:read`, `ingest:read`, `ingest:write`, `connections:read`, `connections:write`, `segments:read`, `exports:read`, `analytics:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `salesforce-cdp_list_data_models` | List data models | GET | /v1/dataModels |
| `salesforce-cdp_get_data_model` | Get data model details | GET | /v1/dataModels/{modelId} |
| `salesforce-cdp_list_ingest_jobs` | List ingest jobs | GET | /v1/ingestJobs |
| `salesforce-cdp_create_ingest_job` | Create ingest job | POST | /v1/ingestJobs |
| `salesforce-cdp_list_connections` | List connections | GET | /v1/connections |
| `salesforce-cdp_create_connection` | Create connection | POST | /v1/connections |
| `salesforce-cdp_list_segments` | List segments | GET | /v1/segments |
| `salesforce-cdp_get_segment` | Get segment details | GET | /v1/segments/{segmentId} |
| `salesforce-cdp_list_exports` | List export jobs | GET | /v1/exports |
| `salesforce-cdp_get_analytics` | Get analytics | GET | /v1/analytics |

---

## Tool Details

### salesforce-cdp_list_data_models

**What it does**: Returns a list of all data models.

**When to use**: Understand CDP data structure.

**Arguments**:
- `limit` (optional): Number of data models (default 50)

**Example LLM prompt**: "List all data models"

---

### salesforce-cdp_get_data_model

**What it does**: Gets details of a specific data model.

**When to use**: Get model schema and fields.

**Arguments**:
- `modelId` (required): The data model ID

**Example LLM prompt**: "Get details for data model dm_abc123"

---

### salesforce-cdp_list_ingest_jobs

**What it does**: Returns ingest jobs.

**When to use**: Monitor data loading.

**Arguments**:
- `limit` (optional): Number of jobs (default 50)

**Example LLM prompt**: "List all ingest jobs"

---

### salesforce-cdp_create_ingest_job

**What it does**: Creates a new ingest job.

**When to use**: Load data into CDP.

**Arguments**:
- `definitionId` (required): Ingest definition ID
- `records` (optional): Records to ingest

**Example LLM prompt**: "Create an ingest job with definition def_123"

---

### salesforce-cdp_list_connections

**What it does**: Returns all connections.

**When to use**: View data integrations.

**Arguments**:
- `limit` (optional): Number of connections (default 50)

**Example LLM prompt**: "List all connections"

---

### salesforce-cdp_create_connection

**What it does**: Creates a new connection.

**When to use**: Set up data integration.

**Arguments**:
- `name` (required): Connection name
- `type` (required): Connection type

**Example LLM prompt**: "Create a connection called 'My Data Source'"

---

### salesforce-cdp_list_segments

**What it does**: Returns all segments.

**When to use**: View audience segments.

**Arguments**:
- `limit` (optional): Number of segments (default 50)

**Example LLM prompt**: "List all segments"

---

### salesforce-cdp_get_segment

**What it does**: Gets segment details.

**When to use**: Get audience segment information.

**Arguments**:
- `segmentId` (required): The segment ID

**Example LLM prompt**: "Get details for segment seg_123"

---

### salesforce-cdp_list_exports

**What it does**: Returns export jobs.

**When to use**: View data exports.

**Arguments**:
- `limit` (optional): Number of exports (default 50)

**Example LLM prompt**: "List all export jobs"

---

### salesforce-cdp_get_analytics

**What it does**: Returns analytics and insights.

**When to use**: Get data insights.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get analytics for the last 30 days"

---

## Salesforce CDP Notes

- CDP unifies customer data from multiple sources
- Data models define unified customer profiles
- Ingest jobs bring data into the platform
- Segments are audiences for activation
- Analytics show data quality and insights
