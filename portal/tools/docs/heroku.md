# Heroku Tools

Provider: `heroku` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Heroku Platform API. They allow AI agents to interact with apps, dynos, releases, and config vars on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `heroku_list_apps` | List all apps | GET | /apps |
| `heroku_get_app` | Get details of a specific app | GET | /apps/{app_id} |
| `heroku_create_app` | Create a new app | POST | /apps |
| `heroku_delete_app` | Delete an app | DELETE | /apps/{app_id} |
| `heroku_list_dynos` | List dynos for an app | GET | /apps/{app_id}/dynos |
| `heroku_restart_dyno` | Restart a dyno | POST | /apps/{app_id}/dynos/{dyno_id}/actions/restart |
| `heroku_list_releases` | List releases for an app | GET | /apps/{app_id}/releases |
| `heroku_get_release` | Get a specific release | GET | /apps/{app_id}/releases/{release_id} |
| `heroku_list_config_vars` | List config vars for an app | GET | /apps/{app_id}/config-vars |
| `heroku_get_config_var` | Get a specific config var | GET | /apps/{app_id}/config-vars/{key} |

---

## Tool Details

### heroku_list_apps

**What it does**: Returns a list of all apps for the authenticated user with details including name, region, stack, and creation date.

**When to use**: List all apps to find a specific one or see overall app inventory.

**Arguments**:
- `team_id` (optional): Filter by team/owner ID
- `limit` (optional): Maximum number of apps to return (default 20)
- `page` (optional): Page number for pagination (default 1)

**Example LLM prompt**: "List all my Heroku apps"

---

### heroku_get_app

**What it does**: Gets details of a specific app including region, stack, dynos, and add-ons.

**When to use**: Get app details, check configuration, or view resource usage.

**Arguments**:
- `app_id` (required): App name or ID

**Example LLM prompt**: "Show me details for my production-app"

---

### heroku_create_app

**What it does**: Creates a new app with specified name, region, and optional stack configuration.

**When to use**: Create a new app, initialize a project, or set up a staging environment.

**Arguments**:
- `name` (optional): App name (auto-generated if not provided)
- `region` (optional): Region: us or eu (default us)
- `stack` (optional): Stack: heroku-20, heroku-22
- `team` (optional): Team to assign the app to

**Example LLM prompt**: "Create a new app called my-app in the eu region"

---

### heroku_delete_app

**What it does**: Deletes an existing app and all its data. This action is irreversible.

**When to use**: Remove an app that is no longer needed. WARNING: This permanently destroys all dynos, data, and add-ons.

**Arguments**:
- `app_id` (required): App name or ID

**Example LLM prompt**: "Delete the my-app Heroku app"

---

### heroku_list_dynos

**What it does**: Lists all dynos running for a specific app with type, state, and uptime information.

**When to use**: Check app status, view running processes, or monitor resource usage.

**Arguments**:
- `app_id` (required): App name or ID

**Example LLM prompt**: "List all dynos running for my production-app"

---

### heroku_restart_dyno

**What it does**: Restarts a specific dyno or all dynos of an app.

**When to use**: Refresh an app after deployment, troubleshoot unresponsive dynos, or reset state.

**Arguments**:
- `app_id` (required): App name or ID
- `dyno_id` (required): Dyno name (e.g., web.1) or 'all' to restart all dynos

**Example LLM prompt**: "Restart the web.1 dyno on my production-app"

---

### heroku_list_releases

**What it does**: Lists all releases for an app, including the version number, deploy date, and description.

**When to use**: View deployment history, find a previous working version, or check who deployed.

**Arguments**:
- `app_id` (required): App name or ID

**Example LLM prompt**: "List all releases for my-app"

---

### heroku_get_release

**What it does**: Gets details of a specific release including the slug, created date, and description.

**When to use**: Get details about a particular release, see what code was deployed.

**Arguments**:
- `app_id` (required): App name or ID
- `release_id` (required): Release version (e.g., v1, v2) or release ID

**Example LLM prompt**: "Show me details for v3 of my-app"

---

### heroku_list_config_vars

**What it does**: Lists all config vars (environment variables) for an app as key-value pairs.

**When to use**: View environment configuration, check database URLs, or verify settings.

**Arguments**:
- `app_id` (required): App name or ID

**Example LLM prompt**: "List all config vars for my app"

---

### heroku_get_config_var

**What it does**: Gets a specific config var value for an app.

**When to use**: Get a single environment variable value, verify a specific setting.

**Arguments**:
- `app_id` (required): App name or ID
- `key` (required): Config var key

**Example LLM prompt**: "Get the DATABASE_URL config var"

---

## Heroku API Reference

These tools use the Heroku Platform API. See official docs for full details:
- https://devcenter.heroku.com/articles/platform-api-reference
- Rate limits: Varies by endpoint
- Pagination: Use `page` parameter
- Authentication: Bearer token via Nango
