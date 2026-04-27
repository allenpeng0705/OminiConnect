# Zapier Tools

Provider: `zapier` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Zapier API. They allow AI agents to manage integrations (Zaps), list available actions and searches across apps, trigger actions, and monitor Zap status. Zapier connects thousands of apps to automate workflows.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zapier
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zapier_list_integrations` | List integrations/Zaps | GET | /zaps |
| `zapier_get_integration` | Get integration details | GET | /zaps/{zap_id} |
| `zapier_create_integration` | Create an integration | POST | /zaps |
| `zapier_list_actions` | List available actions | GET | /actions |
| `zapier_get_action` | Get action details | GET | /actions/{action_id} |
| `zapier_trigger_action` | Trigger an action | POST | /actions/trigger |
| `zapier_list_searches` | List available searches | GET | /searches |
| `zapier_get_search` | Get search details | GET | /searches/{search_id} |
| `zapier_create_zap` | Create a Zap | POST | /zaps |
| `zapier_get_zap_status` | Get Zap status | GET | /zaps/{zap_id}/status |

---

## Tool Details

### zapier_list_integrations

**What it does**: Lists all Zapier integrations (Zaps) in the connected account.

**When to use**: See all active automations, find Zaps by status.

**Arguments**:
- `status` (optional): Filter by `active`, `paused`, or `draft`
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all active Zaps"

---

### zapier_get_integration

**What it does**: Gets detailed information about a specific integration or Zap.

**When to use**: Understand Zap configuration, see trigger and action details.

**Arguments**:
- `zap_id` (required): Zap ID

**Example LLM prompt**: "Get details for Zap 12345"

---

### zapier_create_integration

**What it does**: Creates a new integration or Zap with trigger and action configuration.

**When to use**: Set up new automation workflows, connect apps.

**Arguments**:
- `name` (required): Integration/Zap name
- `trigger_app` (required): Trigger app name
- `trigger_action` (optional): Trigger action name
- `action_app` (required): Action app name
- `action_operation` (optional): Action operation name
- `mapping` (optional): Field mapping between trigger and action

**Example LLM prompt**: "Create a Zap called 'New Gmail to Slack' with Gmail trigger and Slack action"

---

### zapier_list_actions

**What it does**: Lists available Zapier actions across all supported apps.

**When to use**: Discover what actions are possible, filter by app or category.

**Arguments**:
- `app` (optional): Filter by app name
- `category` (optional): Filter by category
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all actions for Salesforce"

---

### zapier_get_action

**What it does**: Gets detailed information about a specific action including input fields.

**When to use**: Understand what data an action needs, see field definitions.

**Arguments**:
- `action_id` (required): Action ID

**Example LLM prompt**: "Get details for action 12345"

---

### zapier_trigger_action

**What it does**: Triggers a specific action directly without creating a full Zap.

**When to use**: Execute one-time actions, push data into apps immediately.

**Arguments**:
- `action_id` (optional): Action ID
- `app` (required): App name
- `action` (required): Action name
- `input_data` (optional): Input data for the action

**Example LLM prompt**: "Trigger a Slack message action with text 'Hello from AI agent'"

---

### zapier_list_searches

**What it does**: Lists available Zapier searches across all supported apps.

**When to use**: Discover search functionality, find records in connected apps.

**Arguments**:
- `app` (optional): Filter by app name
- `category` (optional): Filter by category
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all searches for HubSpot"

---

### zapier_get_search

**What it does**: Gets detailed information about a specific search including input fields.

**When to use**: Understand search parameters, see what can be searched.

**Arguments**:
- `search_id` (required): Search ID

**Example LLM prompt**: "Get details for search 67890"

---

### zapier_create_zap

**What it does**: Creates a new Zap with full trigger and action configuration.

**When to use**: Set up complete automations with multiple steps.

**Arguments**:
- `name` (required): Zap name
- `trigger` (required): Trigger configuration object with `app` and `event`
- `actions` (optional): Array of action configurations
- `zap_mode` (optional): Mode `live` or `test`

**Example LLM prompt**: "Create a Zap with a GitHub issue trigger and Jira create issue action"

---

### zapier_get_zap_status

**What it does**: Gets the current status of a Zap including last run details and history.

**When to use**: Check if Zap is working, see recent execution history.

**Arguments**:
- `zap_id` (required): Zap ID

**Example LLM prompt**: "Check the status of Zap 12345"

---

## Zapier API Notes

- **Zap structure**: Zaps consist of a trigger (when this happens) and one or more actions (then do this)
- **Supported apps**: Zapier supports 5,000+ apps including Salesforce, Slack, Jira, Gmail, etc.
- **Field mapping**: Actions accept data from trigger output via field mapping
- **Zap modes**: Zaps can run in `test` mode (for testing) or `live` mode (production)
- **Action triggering**: Direct action triggering bypasses Zap creation for one-time operations
- **Pagination**: Default page_size is 20, max is 100
