# Snowflake Tools

Provider: `snowflake` | Engine: `nango` | Auth: OAuth / API Key (via Nango)

## Overview

These tools wrap the Snowflake API. They allow AI agents to explore Snowflake databases, schemas, tables, stages, and pipes. Snowflake is a cloud data platform for analytics and data warehousing.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `account_readonly` or `account_admin`

**Native API_KEY (engine=omini_connect_native)**:
- Snowflake credentials stored in `client_secret` field
- Direct Snowflake API passthrough

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `snowflake_list_databases` | List databases in the account | GET | /databases |
| `snowflake_get_database` | Get database details | GET | /databases/{database} |
| `snowflake_list_schemas` | List schemas in a database | GET | /databases/{database}/schemas |
| `snowflake_get_schema` | Get schema details | GET | /databases/{database}/schemas/{schema} |
| `snowflake_list_tables` | List tables in a schema | GET | /databases/{database}/schemas/{schema}/tables |
| `snowflake_get_table` | Get table details and columns | GET | /databases/{database}/schemas/{schema}/tables/{table} |
| `snowflake_list_stages` | List stages in a schema | GET | /databases/{database}/schemas/{schema}/stages |
| `snowflake_get_stage` | Get stage details | GET | /databases/{database}/schemas/{schema}/stages/{stage} |
| `snowflake_list_pipes` | List pipes in a schema | GET | /databases/{database}/schemas/{schema}/pipes |
| `snowflake_get_pipe` | Get pipe details | GET | /databases/{database}/schemas/{schema}/pipes/{pipe} |

---

## Tool Details

### snowflake_list_databases

**What it does**: Returns a list of all databases in the Snowflake account with metadata including names and creation dates.

**When to use**: Discover available data sources, understand the data landscape, or find databases to explore.

**Arguments**:
- `like` (optional): Filter databases by name pattern

**Example LLM prompt**: "What databases are available in our Snowflake account?"

---

### snowflake_get_database

**What it does**: Returns details about a specific Snowflake database including name, created date, and comment.

**When to use**: Get context about a database before exploring its schemas and tables.

**Arguments**:
- `database` (required): Database name

**Example LLM prompt**: "Tell me about the ANALYTICS database"

---

### snowflake_list_schemas

**What it does**: Lists all schemas within a Snowflake database. Schemas organize tables and views.

**When to use**: Discover the structure of a database, find specific schema names, or plan table exploration.

**Arguments**:
- `database` (required): Database name
- `like` (optional): Filter schemas by name pattern

**Example LLM prompt**: "What schemas exist in the PRODUCTION database?"

---

### snowflake_get_schema

**What it does**: Returns details about a specific Snowflake schema including name and created date.

**When to use**: Get schema metadata before listing its tables or views.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name

**Example LLM prompt**: "What is the PUBLIC schema?"

---

### snowflake_list_tables

**What it does**: Lists all tables in a Snowflake schema with their names, types, and column counts.

**When to use**: Discover available data tables, find tables to query, or understand the data structure.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name
- `like` (optional): Filter tables by name pattern

**Example LLM prompt**: "List all tables in the SALES schema"

---

### snowflake_get_table

**What it does**: Returns detailed information about a Snowflake table including column definitions, types, primary keys, and row count.

**When to use**: Understand table structure before writing queries, see column names and data types.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name
- `table` (required): Table name

**Example LLM prompt**: "What are the columns in the CUSTOMERS table?"

---

### snowflake_list_stages

**What it does**: Lists all stages in a Snowflake schema with their names, types (internal/external), and storage locations.

**When to use**: Discover data loading destinations, find stages for data ingestion, or understand data pipelines.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name
- `like` (optional): Filter stages by name pattern

**Example LLM prompt**: "What stages exist in the DATA_LOAD schema?"

---

### snowflake_get_stage

**What it does**: Returns detailed information about a Snowflake stage including type, storage location, and file format options.

**When to use**: Understand stage configuration before setting up data loads.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name
- `stage` (required): Stage name

**Example LLM prompt**: "Get details for the S3_LOAD stage"

---

### snowflake_list_pipes

**What it does**: Lists all pipes in a Snowflake schema with their names, definitions, and status.

**When to use**: Discover data ingestion pipelines and their current status.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name
- `like` (optional): Filter pipes by name pattern

**Example LLM prompt**: "What pipes are defined in the DATA_LOAD schema?"

---

### snowflake_get_pipe

**What it does**: Returns detailed information about a Snowflake pipe including the COPY statement definition and status.

**When to use**: Understand pipe configuration and current ingestion status.

**Arguments**:
- `database` (required): Database name
- `schema` (required): Schema name
- `pipe` (required): Pipe name

**Example LLM prompt**: "Get details for the orders_ingest pipe"

---

## Snowflake API Reference

These tools use the Snowflake API. See official docs for full details:
- https://docs.snowflake.com/en/developer-guide/api/
- Rate limits: Vary by Snowflake edition and tier
- Query results: Limited to 1000 rows by default
- All dates: ISO 8601 format
- **Stages**: Internal (Snowflake-managed) or external (S3, Azure, GCS) storage for data loading
- **Pipes**: Continuous data ingestion objects that load data from stages into tables
