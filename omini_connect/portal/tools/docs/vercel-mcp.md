# Vercel MCP Tools

Provider: `vercel-mcp` | Engine: `nango` | Auth: MCP OAuth2 via Nango

## Overview

Vercel MCP server for deployment and infrastructure management. **Requires mcp oauth2 via nango.**

## Authentication

**MCP OAuth2**:
- Uses MCP OAuth2 flow
- Token stored in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `vercel_list_projects` | List all projects | GET | /v1/projects |
| `vercel_get_project` | Get project details | GET | /v1/projects/{id} |
| `vercel_list_deployments` | List all deployments | GET | /v1/deployments |
| `vercel_get_deployment` | Get deployment details | GET | /v1/deployments/{id} |
| `vercel_create_deployment` | Create a new deployment | POST | /v1/deployments |
| `vercel_delete_deployment` | Delete a deployment | DELETE | /v1/deployments/{id} |
| `vercel_list_domains` | List all domains | GET | /v1/domains |
| `vercel_add_domain` | Add a new domain | POST | /v1/domains |
| `vercel_list_secrets` | List all environment secrets | GET | /v1/secrets |
| `vercel_create_secret` | Create an environment secret | POST | /v1/secrets |

---

## Tool Details

### vercel_list_projects

**What it does**: List all projects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_get_project

**What it does**: Get project details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_list_deployments

**What it does**: List all deployments

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_get_deployment

**What it does**: Get deployment details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_create_deployment

**What it does**: Create a new deployment

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_delete_deployment

**What it does**: Delete a deployment

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_list_domains

**What it does**: List all domains

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_add_domain

**What it does**: Add a new domain

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_list_secrets

**What it does**: List all environment secrets

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vercel_create_secret

**What it does**: Create an environment secret

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://mcp.vercel.com`
- Docs: https://nango.dev/docs/integrations/all/vercel-mcp
