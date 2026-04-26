# SurveyMonkey Tools

Provider: `survey-monkey` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the SurveyMonkey API. They allow AI agents to interact with SurveyMonkey functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_surveys` | List all surveys | GET | /v3/surveys |
| `get_survey` | Get survey details | GET | /v3/surveys/{id} |
| `list_collectors` | List survey collectors | GET | /v3/surveys/{id}/collectors |
| `get_collector` | Get collector details | GET | /v3/collectors/{id} |
| `list_responses` | List survey responses | GET | /v3/surveys/{id}/responses |
| `get_response` | Get response details | GET | /v3/responses/{id} |
| `list_pages` | List survey pages | GET | /v3/surveys/{id}/pages |
| `list_questions` | List survey questions | GET | /v3/surveys/{id}/questions |
| `get_benchmark` | Get survey benchmark | GET | /v3/surveys/{id}/benchmarks |
| `get_details` | Get survey details with questions | GET | /v3/surveys/{id}/details |

---

## Tool Details

### list_surveys

**What it does**: List all surveys

**When to use**: Use this tool when you need to list all surveys.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list surveys to..."

---

### get_survey

**What it does**: Get survey details

**When to use**: Use this tool when you need to get survey details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get survey to..."

---

### list_collectors

**What it does**: List survey collectors

**When to use**: Use this tool when you need to list survey collectors.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list collectors to..."

---

### get_collector

**What it does**: Get collector details

**When to use**: Use this tool when you need to get collector details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get collector to..."

---

### list_responses

**What it does**: List survey responses

**When to use**: Use this tool when you need to list survey responses.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list responses to..."

---

### get_response

**What it does**: Get response details

**When to use**: Use this tool when you need to get response details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get response to..."

---

### list_pages

**What it does**: List survey pages

**When to use**: Use this tool when you need to list survey pages.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list pages to..."

---

### list_questions

**What it does**: List survey questions

**When to use**: Use this tool when you need to list survey questions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list questions to..."

---

### get_benchmark

**What it does**: Get survey benchmark

**When to use**: Use this tool when you need to get survey benchmark.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get benchmark to..."

---

### get_details

**What it does**: Get survey details with questions

**When to use**: Use this tool when you need to get survey details with questions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get details to..."

---

## SurveyMonkey API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://api.surveymonkey.com
- **API prefix**: /v3
- **Rate limits**: Check provider documentation for specific limits
