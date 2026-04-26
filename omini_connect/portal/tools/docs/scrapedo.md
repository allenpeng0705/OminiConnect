# Scrape.do Tools

Provider: `scrapedo` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Scrape.do API. They allow AI agents to interact with Scrape.do functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `scrape_url` | Scrape content from a URL | POST | /scrape |
| `scrape_batch` | Scrape multiple URLs in batch | POST | /scrape/batch |
| `get_scraped_data` | Retrieve previously scraped data | GET | /data |
| `list_projects` | List all scraping projects | GET | /projects |
| `create_project` | Create a new scraping project | POST | /projects |
| `get_project` | Get project details | GET | /projects/{id} |
| `update_project` | Update project settings | PUT | /projects/{id} |
| `delete_project` | Delete a project | DELETE | /projects/{id} |
| `get_usage` | Get API usage statistics | GET | /usage |
| `get_status` | Check service status | GET | /status |

---

## Tool Details

### scrape_url

**What it does**: Scrape content from a URL

**When to use**: Use this tool when you need to scrape content from a url.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use scrape url to..."

---

### scrape_batch

**What it does**: Scrape multiple URLs in batch

**When to use**: Use this tool when you need to scrape multiple urls in batch.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use scrape batch to..."

---

### get_scraped_data

**What it does**: Retrieve previously scraped data

**When to use**: Use this tool when you need to retrieve previously scraped data.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get scraped data to..."

---

### list_projects

**What it does**: List all scraping projects

**When to use**: Use this tool when you need to list all scraping projects.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list projects to..."

---

### create_project

**What it does**: Create a new scraping project

**When to use**: Use this tool when you need to create a new scraping project.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create project to..."

---

### get_project

**What it does**: Get project details

**When to use**: Use this tool when you need to get project details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get project to..."

---

### update_project

**What it does**: Update project settings

**When to use**: Use this tool when you need to update project settings.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use update project to..."

---

### delete_project

**What it does**: Delete a project

**When to use**: Use this tool when you need to delete a project.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use delete project to..."

---

### get_usage

**What it does**: Get API usage statistics

**When to use**: Use this tool when you need to get api usage statistics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get usage to..."

---

### get_status

**What it does**: Check service status

**When to use**: Use this tool when you need to check service status.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get status to..."

---

## Scrape.do API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.scrape.do
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
