# Snowflake (JWT) Tools

Provider: `snowflake-jwt` | Engine: `nango` | Auth: JWT

## Overview

These tools wrap the Snowflake (JWT) API. They allow AI agents to interact with Snowflake (JWT) functionality. **Requires JWT authentication.**

## Authentication

**JWT Authentication**:
- Uses RSA key pair for JWT-based authentication
- Nango manages JWT signing and token refresh
- Scopes depend on the Snowflake account permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `query_data` | Execute a SQL query | POST | /api/v2/statements |
| `list_databases` | List all databases | GET | /api/v2/databases |
| `list_schemas` | List schemas in database | GET | /api/v2/databases/{db}/schemas |
| `list_tables` | List tables in schema | GET | /api/v2/databases/{db}/schemas/{schema}/tables |
| `get_query_status` | Check query execution status | GET | /api/v2/statements/{id} |
| `cancel_query` | Cancel a running query | POST | /api/v2/statements/{id}/cancel |
| `list_pipes` | List pipes in schema | GET | /api/v2/databases/{db}/schemas/{schema}/pipes |
| `list_streams` | List streams in schema | GET | /api/v2/databases/{db}/schemas/{schema}/streams |
| `list_tasks` | List tasks in schema | GET | /api/v2/databases/{db}/schemas/{schema}/tasks |
| `get_warehouse` | Get warehouse details | GET | /api/v2/warehouses/{name} |

---

## Tool Details

### query_data

**What it does**: Execute a SQL query

**When to use**: Use this tool when you need to execute a sql query.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use query data to..."

---

### list_databases

**What it does**: List all databases

**When to use**: Use this tool when you need to list all databases.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list databases to..."

---

### list_schemas

**What it does**: List schemas in database

**When to use**: Use this tool when you need to list schemas in database.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list schemas to..."

---

### list_tables

**What it does**: List tables in schema

**When to use**: Use this tool when you need to list tables in schema.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list tables to..."

---

### get_query_status

**What it does**: Check query execution status

**When to use**: Use this tool when you need to check query execution status.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get query status to..."

---

### cancel_query

**What it does**: Cancel a running query

**When to use**: Use this tool when you need to cancel a running query.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use cancel query to..."

---

### list_pipes

**What it does**: List pipes in schema

**When to use**: Use this tool when you need to list pipes in schema.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list pipes to..."

---

### list_streams

**What it does**: List streams in schema

**When to use**: Use this tool when you need to list streams in schema.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list streams to..."

---

### list_tasks

**What it does**: List tasks in schema

**When to use**: Use this tool when you need to list tasks in schema.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list tasks to..."

---

### get_warehouse

**What it does**: Get warehouse details

**When to use**: Use this tool when you need to get warehouse details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get warehouse to..."

---

## Snowflake (JWT) API Notes

- **Auth mode**: JWT
- **Base URL**: https://{subdomain}.snowflakecomputing.com
- **API prefix**: /api/v2
- **Rate limits**: Check provider documentation for specific limits
