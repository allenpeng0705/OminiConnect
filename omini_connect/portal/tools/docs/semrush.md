# Semrush Tools

Provider: `semrush` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Semrush API. They allow AI agents to interact with Semrush functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_projects` | List all Semrush projects | GET | /management/v1/projects |
| `get_domain_analytics` | Get domain analytics data | GET | /reports/v2/domain_analytics |
| `get_keyword_analytics` | Get keyword analytics | GET | /reports/v2/keyword_analytics |
| `get_backlinks` | Get backlink data | GET | /reports/v2/backlinks |
| `get_traffic_analytics` | Get traffic analytics | GET | /reports/v2/traffic_analytics |
| `get_position_tracking` | Get position tracking data | GET | /reports/v2/position_tracking |
| `get_analytics_overview` | Get analytics overview | GET | /reports/v2/analytics_overview |
| `get_competitors` | Get competitor analysis | GET | /reports/v2/competitors |
| `get_link_build` | Get link building opportunities | GET | /reports/v2/link_build |
| `get_social_media` | Get social media analytics | GET | /reports/v2/social_media |

---

## Tool Details

### get_projects

**What it does**: List all Semrush projects

**When to use**: Use this tool when you need to list all semrush projects.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get projects to..."

---

### get_domain_analytics

**What it does**: Get domain analytics data

**When to use**: Use this tool when you need to get domain analytics data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get domain analytics to..."

---

### get_keyword_analytics

**What it does**: Get keyword analytics

**When to use**: Use this tool when you need to get keyword analytics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get keyword analytics to..."

---

### get_backlinks

**What it does**: Get backlink data

**When to use**: Use this tool when you need to get backlink data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get backlinks to..."

---

### get_traffic_analytics

**What it does**: Get traffic analytics

**When to use**: Use this tool when you need to get traffic analytics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get traffic analytics to..."

---

### get_position_tracking

**What it does**: Get position tracking data

**When to use**: Use this tool when you need to get position tracking data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get position tracking to..."

---

### get_analytics_overview

**What it does**: Get analytics overview

**When to use**: Use this tool when you need to get analytics overview.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get analytics overview to..."

---

### get_competitors

**What it does**: Get competitor analysis

**When to use**: Use this tool when you need to get competitor analysis.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get competitors to..."

---

### get_link_build

**What it does**: Get link building opportunities

**When to use**: Use this tool when you need to get link building opportunities.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get link building to..."

---

### get_social_media

**What it does**: Get social media analytics

**When to use**: Use this tool when you need to get social media analytics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get social media to..."

---

## Semrush API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.semrush.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
