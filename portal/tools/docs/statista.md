# Statista Tools

Provider: `statista` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Statista API. They allow AI agents to interact with Statista functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_statistics` | List available statistics | GET | /v1/statistics |
| `get_statistic` | Get statistic details | GET | /v1/statistics/{id} |
| `search_statistics` | Search for statistics | GET | /v1/statistics/search |
| `list_databases` | List available databases | GET | /v1/databases |
| `list_topics` | List topics | GET | /v1/topics |
| `get_topic` | Get topic details | GET | /v1/topics/{id} |
| `list_countries` | List available countries | GET | /v1/countries |
| `list_charts` | List chart types | GET | /v1/charts |
| `get_forecast` | Get forecast data | GET | /v1/statistics/{id}/forecast |
| `get_export` | Export statistic data | GET | /v1/statistics/{id}/export |

---

## Tool Details

### list_statistics

**What it does**: List available statistics

**When to use**: Use this tool when you need to list available statistics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list statistics to..."

---

### get_statistic

**What it does**: Get statistic details

**When to use**: Use this tool when you need to get statistic details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get statistic to..."

---

### search_statistics

**What it does**: Search for statistics

**When to use**: Use this tool when you need to search for statistics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use search statistics to..."

---

### list_databases

**What it does**: List available databases

**When to use**: Use this tool when you need to list available databases.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list databases to..."

---

### list_topics

**What it does**: List topics

**When to use**: Use this tool when you need to list topics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list topics to..."

---

### get_topic

**What it does**: Get topic details

**When to use**: Use this tool when you need to get topic details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get topic to..."

---

### list_countries

**What it does**: List available countries

**When to use**: Use this tool when you need to list available countries.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list countries to..."

---

### list_charts

**What it does**: List chart types

**When to use**: Use this tool when you need to list chart types.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list charts to..."

---

### get_forecast

**What it does**: Get forecast data

**When to use**: Use this tool when you need to get forecast data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get forecast to..."

---

### get_export

**What it does**: Export statistic data

**When to use**: Use this tool when you need to export statistic data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get export to..."

---

## Statista API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.statista.ai
- **API prefix**: /v1
- **Rate limits**: Check provider documentation for specific limits
