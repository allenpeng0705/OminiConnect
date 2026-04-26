# Pipedream Client Credentials Tools

Provider: `pipedream-oauth2-cc` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Pipedream API. They allow AI agents to manage event sources, workflows, API keys, events, and app connections. Pipedream is an automation and integration platform. **Requires Pipedream OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client credentials for token flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.pipedream.com

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pipedream_cc_list_sources` | List event sources | GET | /v1/sources |
| `pipedream_cc_get_source` | Get source details | GET | /v1/sources/{sourceId} |
| `pipedream_cc_list_workflows` | List workflows | GET | /v1/workflows |
| `pipedream_cc_get_workflow` | Get workflow details | GET | /v1/workflows/{workflowId} |
| `pipedream_cc_list_keys` | List API keys | GET | /v1/keys |
| `pipedream_cc_get_user` | Get user info | GET | /v1/users/me |
| `pipedream_cc_list_events` | List events | GET | /v1/events |
| `pipedream_cc_get_event` | Get event details | GET | /v1/events/{eventId} |
| `pipedream_cc_list_connections` | List connections | GET | /v1/connections |
| `pipedream_cc_get_account` | Get account info | GET | /v1/account |

---

## Tool Details

### pipedream_cc_list_sources

**What it does**: Lists all event sources.

**When to use**: Browse event sources, check status.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active sources"

---

### pipedream_cc_get_source

**What it does**: Gets detailed information about a specific source.

**When to use**: Get source configuration, metrics.

**Arguments**:
- `sourceId` (required): Source ID

**Example LLM prompt**: "Get details for source 12345"

---

### pipedream_cc_list_workflows

**What it does**: Lists all workflows.

**When to use**: Browse automation workflows.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active workflows"

---

### pipedream_cc_get_workflow

**What it does**: Gets detailed information about a specific workflow.

**When to use**: Get workflow details, execution history.

**Arguments**:
- `workflowId` (required): Workflow ID

**Example LLM prompt**: "Get details for workflow 67890"

---

### pipedream_cc_list_keys

**What it does**: Lists all API keys.

**When to use**: Manage API access.

**Arguments**: None

**Example LLM prompt**: "List my API keys"

---

### pipedream_cc_get_user

**What it does**: Gets current user information.

**When to use**: Get user profile, subscription.

**Arguments**: None

**Example LLM prompt**: "Get my user information"

---

### pipedream_cc_list_events

**What it does**: Lists events from a source or workflow.

**When to use**: Browse event history.

**Arguments**:
- `sourceId` (optional): Filter by source
- `workflowId` (optional): Filter by workflow

**Example LLM prompt**: "List events for source 12345"

---

### pipedream_cc_get_event

**What it does**: Gets detailed information about a specific event.

**When to use**: Inspect event payload, metadata.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "Get details for event EVT-11111"

---

### pipedream_cc_list_connections

**What it does**: Lists all app connections.

**When to use**: Browse connected apps.

**Arguments**: None

**Example LLM prompt**: "What apps are connected?"

---

### pipedream_cc_get_account

**What it does**: Gets account information.

**When to use**: Get account settings, limits.

**Arguments**: None

**Example LLM prompt**: "Get my account information"

---

## Pipedream Client Credentials API Notes

- **OAuth2 Client Credentials**: Machine-to-machine authentication
- **Event-Driven**: Sources emit events
- **Workflows**: Automate processes with steps
