# Callrail Tools

Provider: `callrail` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Callrail API. Callrail is a call tracking and analytics platform that helps businesses track phone calls, manage leads, and analyze marketing campaign effectiveness. **Requires Callrail API key.**

## Authentication

**Nango API_KEY**:
- User provides their Callrail API key
- Token passed via `Authorization: Token token=${apiKey}` header
- Base URL: `https://api.callrail.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `callrail_list_accounts` | List accounts | GET | /v3/accounts |
| `callrail_get_account` | Get account details | GET | /v3/accounts/{accountId} |
| `callrail_list_companies` | List companies | GET | /v3/accounts/{accountId}/companies |
| `callrail_get_company` | Get company details | GET | /v3/companies/{companyId} |
| `callrail_list_calls` | List calls | GET | /v3/companies/{companyId}/calls |
| `callrail_get_call` | Get call details | GET | /v3/calls/{callId} |
| `callrail_list_call_tags` | List call tags | GET | /v3/companies/{companyId}/tags |
| `callrail_create_call_tag` | Create a call tag | POST | /v3/companies/{companyId}/tags |
| `callrail_list_agents` | List agents | GET | /v3/companies/{companyId}/agents |
| `callrail_get_call_score` | Get call score | GET | /v3/calls/{callId}/score |

---

## Tool Details

### callrail_list_accounts

**What it does**: Lists all accounts associated with the API key.

**When to use**: Find account IDs for subsequent API calls.

**Arguments**: None

**Example LLM prompt**: "List my Callrail accounts"

---

### callrail_get_account

**What it does**: Gets details of a specific account.

**When to use**: View account settings and limits.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for Callrail account 123"

---

### callrail_list_companies

**What it does**: Lists all companies in an account.

**When to use**: Find company IDs for call tracking.

**Arguments**:
- `accountId` (required): Account ID
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List companies in my Callrail account"

---

### callrail_get_company

**What it does**: Gets details of a specific company.

**When to use**: View company settings and call flow.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get company 456 details"

---

### callrail_list_calls

**What it does**: Lists all calls for a company with optional date filtering.

**When to use**: Analyze call volume, find specific calls.

**Arguments**:
- `companyId` (required): Company ID
- `date_from` (optional): Start date (YYYY-MM-DD)
- `date_to` (optional): End date (YYYY-MM-DD)
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List calls for company 456 from the last week"

---

### callrail_get_call

**What it does**: Gets details of a specific call.

**When to use**: View call recording, transcript, or details.

**Arguments**:
- `callId` (required): Call ID

**Example LLM prompt**: "Get details for call 789"

---

### callrail_list_call_tags

**What it does**: Lists all tags used for calls in a company.

**When to use**: View existing tags for categorization.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "List tags for company 456"

---

### callrail_create_call_tag

**What it does**: Creates a new tag for categorizing calls.

**When to use**: Add custom tags for call tracking.

**Arguments**:
- `companyId` (required): Company ID
- `name` (required): Tag name
- `color` (optional): Tag color as hex code

**Example LLM prompt**: "Create a tag called 'sales-lead' for company 456"

---

### callrail_list_agents

**What it does**: Lists all agents in a company.

**When to use**: View team members who handle calls.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "List agents for company 456"

---

### callrail_get_call_score

**What it does**: Gets the conversation analysis score for a call.

**When to use**: Evaluate call quality and lead handling.

**Arguments**:
- `callId` (required): Call ID

**Example LLM prompt**: "Get the score for call 789"

---

## Callrail API Notes

- **API Key Format**: 32-character hex string
- **Company**: Main entity for call tracking and analytics
- **Calls**: Include source tracking, duration, recording, and transcript
- **Tags**: Custom categorization for calls and leads
