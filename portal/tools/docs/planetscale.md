# PlanetScale Tools

Provider: `planetscale` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the PlanetScale API. They allow AI agents to manage databases, branches, tables, and execute queries. PlanetScale is a MySQL-compatible serverless database platform offering branching, schema changes, and serverless connections.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with PlanetScale
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `database:read`, `database:write`, `branch:read`, `branch:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `planetscale_list_databases` | List databases | GET | /v1/databases |
| `planetscale_get_database` | Get database details | GET | /v1/databases/{database} |
| `planetscale_create_database` | Create a new database | POST | /v1/databases |
| `planetscale_list_branches` | List database branches | GET | /v1/databases/{database}/branches |
| `planetscale_get_branch` | Get branch details | GET | /v1/databases/{database}/branches/{branch} |
| `planetscale_create_branch` | Create a new branch | POST | /v1/databases/{database}/branches |
| `planetscale_list_tables` | List tables in a branch | GET | /v1/databases/{database}/branches/{branch}/tables |
| `planetscale_get_table` | Get table schema | GET | /v1/databases/{database}/branches/{branch}/tables/{table} |
| `planetscale_execute_query` | Execute a query | POST | /v1/databases/{database}/branches/{branch}/queries |
| `planetscale_get_query_result` | Get query result | GET | /v1/databases/{database}/queries/{queryId} |

---

## Tool Details

### planetscale_list_databases

**What it does**: Lists all PlanetScale databases in the organization. Databases are MySQL-compatible schemas managed by PlanetScale.

**When to use**: Discover available databases, inventory database resources, find databases for operations.

**Arguments**:
- `org` (required): Organization name

**Example LLM prompt**: "List all databases in my organization"

---

### planetscale_get_database

**What it does**: Gets details about a specific PlanetScale database including region, schema, and branch info.

**When to use**: Check database status, understand database configuration, view branch structure.

**Arguments**:
- `database` (required): Database name

**Example LLM prompt**: "Get details about the 'myapp-prod' database"

---

### planetscale_create_database

**What it does**: Creates a new PlanetScale database with an initial main branch. Specify the region for optimal latency.

**When to use**: Provision new databases, create project databases, set up new environments.

**Arguments**:
- `name` (required): Desired database name (lowercase, alphanumeric, hyphens)
- `region` (required): Region slug (e.g., us-east-1, eu-central-1)
- `notes` (optional): Optional notes about the database

**Example LLM prompt**: "Create a new database called 'myapp-dev' in the us-east-1 region"

---

### planetscale_list_branches

**What it does**: Lists all branches for a PlanetScale database. Branches are isolated copies of the database schema for development and testing.

**When to use**: Discover branches, understand branch structure, find branches for development.

**Arguments**:
- `database` (required): Database name

**Example LLM prompt**: "List all branches for the 'myapp-prod' database"

---

### planetscale_get_branch

**What it does**: Gets details about a specific database branch including schema, row count, and status.

**When to use**: Inspect branch state, check branch schema, verify branch readiness.

**Arguments**:
- `database` (required): Database name
- `branch` (required): Branch name

**Example LLM prompt**: "Get details about the 'feature-user-auth' branch"

---

### planetscale_create_branch

**What it does**: Creates a new branch from the main database or an existing branch. Branches can be used for development, testing, or creating schema changes.

**When to use**: Create development branches, set up testing environments, prepare schema changes.

**Arguments**:
- `database` (required): Database name
- `name` (required): New branch name
- `from_branch` (optional): Source branch to create from (default: main)

**Example LLM prompt**: "Create a new branch called 'feature-payments' from main"

---

### planetscale_list_tables

**What it does**: Lists all tables in a specific branch's schema. Returns table names, row counts, and column information.

**When to use**: Discover tables, understand schema structure, find tables for queries.

**Arguments**:
- `database` (required): Database name
- `branch` (required): Branch name

**Example LLM prompt**: "List all tables in the 'main' branch"

---

### planetscale_get_table

**What it does**: Gets the schema and column definitions of a specific table in a branch.

**When to use**: Understand table structure, find column types, plan queries and migrations.

**Arguments**:
- `database` (required): Database name
- `branch` (required): Branch name
- `table` (required): Table name

**Example LLM prompt**: "Get the schema of the 'users' table in the main branch"

---

### planetscale_execute_query

**What it does**: Executes a SQL query against a database branch using the query API. Supports SELECT, INSERT, UPDATE, and DELETE statements.

**When to use**: Query data, modify records, perform data analysis, test SQL changes.

**Arguments**:
- `database` (required): Database name
- `branch` (required): Branch name
- `query` (required): SQL query to execute
- `options` (optional): Query options including timeout

**Example LLM prompt**: "Execute a SELECT query to find all users created in the last 30 days"

---

### planetscale_get_query_result

**What it does**: Retrieves the results of a previously executed query by its ID. Use after Execute Query to get the actual data returned.

**When to use**: Fetch query results, retrieve data from executed queries, get query output.

**Arguments**:
- `database` (required): Database name
- `queryId` (required): Query ID returned from execute query

**Example LLM prompt**: "Get the results of query 'abc123'"

---

## PlanetScale API Notes

- **Database name**: Must be lowercase, alphanumeric with hyphens
- **Branches**: PlanetScale uses branching for schema changes, similar to Git
- **Main branch**: The production branch, protected and immutable directly
- **Schema changes**: Use deploy requests to merge branch changes to main
- **Serverless**: PlanetScale is MySQL-compatible without managing servers
- **Regions**: Choose regions for latency optimization (us-east-1, eu-central-1, etc.)
- **Query API**: Non-blocking queries via async query execution model
