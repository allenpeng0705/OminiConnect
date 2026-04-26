# Supabase (MCP OAuth) Tools

Provider: `supabase-mcp-oauth` | Engine: `nango` | Auth: MCP_OAUTH2

## Overview

These tools wrap the Supabase (MCP OAuth) API. They allow AI agents to interact with Supabase (MCP OAuth) functionality. **Requires MCP_OAUTH2 authentication.**

## Authentication

**MCP OAuth2 Authentication**:
- Special OAuth2 flow for MCP (Model Context Protocol)
- User authenticates via MCP OAuth2
- Token managed by Nango for MCP server access

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_projects` | List all projects | GET | /projects |
| `get_project` | Get project details | GET | /projects/{ref} |
| `query_table` | Query a database table | GET | /rest/v1/{table} |
| `insert_rows` | Insert rows into table | POST | /rest/v1/{table} |
| `update_rows` | Update rows in table | PATCH | /rest/v1/{table} |
| `delete_rows` | Delete rows from table | DELETE | /rest/v1/{table} |
| `list_functions` | List edge functions | GET | /functions/v1/functions |
| `invoke_function` | Invoke edge function | POST | /functions/v1/{name} |
| `get_storageBuckets` | List storage buckets | GET | /storage/v1/buckets |
| `upload_file` | Upload file to storage | POST | /storage/v1/buckets/{bucket}/objects |

---

## Tool Details

### list_projects

**What it does**: List all projects

**When to use**: Use this tool when you need to list all projects.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list projects to..."

---

### get_project

**What it does**: Get project details

**When to use**: Use this tool when you need to get project details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get project to..."

---

### query_table

**What it does**: Query a database table

**When to use**: Use this tool when you need to query a database table.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use query table to..."

---

### insert_rows

**What it does**: Insert rows into table

**When to use**: Use this tool when you need to insert rows into table.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use insert rows to..."

---

### update_rows

**What it does**: Update rows in table

**When to use**: Use this tool when you need to update rows in table.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use update rows to..."

---

### delete_rows

**What it does**: Delete rows from table

**When to use**: Use this tool when you need to delete rows from table.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use delete rows to..."

---

### list_functions

**What it does**: List edge functions

**When to use**: Use this tool when you need to list edge functions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list functions to..."

---

### invoke_function

**What it does**: Invoke edge function

**When to use**: Use this tool when you need to invoke edge function.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use invoke function to..."

---

### get_storageBuckets

**What it does**: List storage buckets

**When to use**: Use this tool when you need to list storage buckets.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get storage buckets to..."

---

### upload_file

**What it does**: Upload file to storage

**When to use**: Use this tool when you need to upload file to storage.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload file to..."

---

## Supabase (MCP OAuth) API Notes

- **Auth mode**: MCP_OAUTH2
- **Base URL**: https://mcp.supabase.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
