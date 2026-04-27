# Redshift Tools

Provider: `redshift` | Engine: `nango` | Auth: AWS Credentials (via Nango)

## Overview

These tools wrap the Amazon Redshift Data API. They allow AI agents to explore database structure, list schemas and tables, and execute SQL queries. Amazon Redshift is a managed data warehouse service.

## Authentication

**Nango AWS Credentials**:
- User provides AWS access key and secret via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `redshift_data`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `redshift_list_databases` | List databases in the cluster | GET | / |
| `redshift_get_database` | Get database details | GET | / |
| `redshift_list_schemas` | List schemas in a database | GET | / |
| `redshift_get_schema` | Get schema details | GET | / |
| `redshift_list_tables` | List tables in a schema | GET | / |
| `redshift_get_table` | Get table details and columns | GET | / |
| `redshift_execute_query` | Execute a SQL query | POST | / |
| `redshift_list_users` | List database users | GET | / |
| `redshift_get_user` | Get user details | GET | / |
| `redshift_get_query_results` | Get query execution results | GET | / |

---

## Tool Details

### redshift_list_databases

**What it does**: Lists all databases in the Redshift cluster. Returns database names and descriptions.

**When to use**: Discover available databases before exploring schemas.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier

**Example LLM prompt**: "List all databases in the data-warehouse cluster"

---

### redshift_get_database

**What it does**: Returns detailed information about a specific database including name, created date, and table count.

**When to use**: Verify database exists and get its metadata.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name

**Example LLM prompt**: "Get details for the analytics database in cluster data-warehouse"

---

### redshift_list_schemas

**What it does**: Lists all schemas in a Redshift database (e.g., public, information_schema).

**When to use**: Explore database structure and find specific schemas.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name

**Example LLM prompt**: "List all schemas in the analytics database"

---

### redshift_get_schema

**What it does**: Returns detailed information about a specific schema including name and created date.

**When to use**: Understand schema organization.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name
- `schemaName` (required): Schema name

**Example LLM prompt**: "Get details for the sales schema"

---

### redshift_list_tables

**What it does**: Lists all tables in a Redshift schema. Returns table names, types, and column counts.

**When to use**: Discover available tables for querying.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name
- `schemaName` (required): Schema name

**Example LLM prompt**: "List all tables in the sales schema"

---

### redshift_get_table

**What it does**: Returns detailed information about a specific table including column definitions, types, and distribution keys.

**When to use**: Understand table structure before writing queries.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name
- `schemaName` (required): Schema name
- `tableName` (required): Table name

**Example LLM prompt**: "Get details for the orders table in the sales schema"

---

### redshift_execute_query

**What it does**: Executes a SQL query against Redshift and returns results. Use for SELECT, INSERT, UPDATE, DELETE, or DDL.

**When to use**: Query data, create tables, or modify data.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name
- `sql` (required): SQL query string
- `withCredentials` (optional): Execute with user credentials (default false)

**Example LLM prompt**: "Run a query to get total orders by customer from the sales database"

---

### redshift_list_users

**What it does**: Lists all users in the Redshift cluster. Returns usernames, session defaults, and create status.

**When to use**: Audit database access.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name

**Example LLM prompt**: "List all users in the cluster"

---

### redshift_get_user

**What it does**: Returns detailed information about a specific database user including their permissions and session settings.

**When to use**: Check user permissions and settings.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name
- `userName` (required): User name

**Example LLM prompt**: "Get details for the analytics_user"

---

### redshift_get_query_results

**What it does**: Retrieves the results of a previously executed query by its ID.

**When to use**: Fetch results after running an asynchronous query.

**Arguments**:
- `clusterIdentifier` (required): Redshift cluster identifier
- `databaseName` (required): Database name
- `queryId` (required): Query execution ID

**Example LLM prompt**: "Get results for query abc123"

---

## Redshift API Notes

- **Cluster-first**: Most operations require the cluster identifier
- **Schema-qualified**: Tables and procedures are scoped to schemas
- **SQL execution**: Use `execute_query` for SELECT, INSERT, UPDATE, DELETE, and DDL
- **Distribution keys**: Tables may have distribution keys affecting query performance
- **Information schema**: `information_schema` contains metadata about all database objects
- **Async queries**: Long-running queries return a query ID for later retrieval
