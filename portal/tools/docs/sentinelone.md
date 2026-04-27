# SentinelOne Tools

Provider: `sentinelone` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the SentinelOne API. They allow AI agents to interact with SentinelOne functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_sites` | List all sites | GET | /sites |
| `get_agents` | List all agents | GET | /agents |
| `get_threats` | List detected threats | GET | /threats |
| `get_activities` | List activities | GET | /activities |
| `get_applications` | List installed applications | GET | /applications |
| `get_networks` | List network information | GET | /networks |
| `get_policies` | List security policies | GET | /policies |
| `get_exclusions` | List exclusions | GET | /exclusions |
| `get_dashboard` | Get dashboard summary | GET | /dashboard |
| `get_settings` | Get system settings | GET | /settings |

---

## Tool Details

### get_sites

**What it does**: List all sites

**When to use**: Use this tool when you need to list all sites.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get sites to..."

---

### get_agents

**What it does**: List all agents

**When to use**: Use this tool when you need to list all agents.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get agents to..."

---

### get_threats

**What it does**: List detected threats

**When to use**: Use this tool when you need to list detected threats.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get threats to..."

---

### get_activities

**What it does**: List activities

**When to use**: Use this tool when you need to list activities.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get activities to..."

---

### get_applications

**What it does**: List installed applications

**When to use**: Use this tool when you need to list installed applications.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get applications to..."

---

### get_networks

**What it does**: List network information

**When to use**: Use this tool when you need to list network information.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get networks to..."

---

### get_policies

**What it does**: List security policies

**When to use**: Use this tool when you need to list security policies.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get policies to..."

---

### get_exclusions

**What it does**: List exclusions

**When to use**: Use this tool when you need to list exclusions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get exclusions to..."

---

### get_dashboard

**What it does**: Get dashboard summary

**When to use**: Use this tool when you need to get dashboard summary.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get dashboard to..."

---

### get_settings

**What it does**: Get system settings

**When to use**: Use this tool when you need to get system settings.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get settings to..."

---

## SentinelOne API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://{hostname}
- **API prefix**: /web/api/v2.1
- **Rate limits**: Check provider documentation for specific limits
