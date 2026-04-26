# Smartlead.ai Tools

Provider: `smartlead-ai` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Smartlead.ai API. They allow AI agents to interact with Smartlead.ai functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_campaigns` | List all campaigns | GET | /v1/campaigns |
| `get_campaign` | Get campaign details | GET | /v1/campaigns/{id} |
| `list_leads` | List leads in campaign | GET | /v1/campaigns/{id}/leads |
| `add_lead` | Add lead to campaign | POST | /v1/campaigns/{id}/leads |
| `get_lead` | Get lead details | GET | /v1/leads/{id} |
| `list_sequences` | List email sequences | GET | /v1/sequences |
| `get_sequence` | Get sequence details | GET | /v1/sequences/{id} |
| `list_templates` | List email templates | GET | /v1/templates |
| `get_stats` | Get campaign stats | GET | /v1/campaigns/{id}/stats |
| `get_activity` | Get email activity | GET | /v1/campaigns/{id}/activity |

---

## Tool Details

### list_campaigns

**What it does**: List all campaigns

**When to use**: Use this tool when you need to list all campaigns.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list campaigns to..."

---

### get_campaign

**What it does**: Get campaign details

**When to use**: Use this tool when you need to get campaign details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get campaign to..."

---

### list_leads

**What it does**: List leads in campaign

**When to use**: Use this tool when you need to list leads in campaign.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list leads to..."

---

### add_lead

**What it does**: Add lead to campaign

**When to use**: Use this tool when you need to add lead to campaign.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use add lead to..."

---

### get_lead

**What it does**: Get lead details

**When to use**: Use this tool when you need to get lead details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get lead to..."

---

### list_sequences

**What it does**: List email sequences

**When to use**: Use this tool when you need to list email sequences.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list sequences to..."

---

### get_sequence

**What it does**: Get sequence details

**When to use**: Use this tool when you need to get sequence details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get sequence to..."

---

### list_templates

**What it does**: List email templates

**When to use**: Use this tool when you need to list email templates.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list templates to..."

---

### get_stats

**What it does**: Get campaign stats

**When to use**: Use this tool when you need to get campaign stats.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get stats to..."

---

### get_activity

**What it does**: Get email activity

**When to use**: Use this tool when you need to get email activity.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get activity to..."

---

## Smartlead.ai API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://server.smartlead.ai/api
- **API prefix**: /v1
- **Rate limits**: Check provider documentation for specific limits
