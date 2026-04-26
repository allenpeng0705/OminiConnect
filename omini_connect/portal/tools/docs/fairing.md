# Fairing Tools

Provider: `fairing` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fairing API. They allow AI agents to create and manage post-purchase surveys, track referrals, and analyze customer data. Fairing is a marketing attribution platform.

## Authentication

**Nango API_KEY**:
- User provides their Fairing Secret Token via Nango Connect
- Key is passed in the Authorization header
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fairing_list_questions` | List questions | GET | /questions |
| `fairing_get_question` | Get question details | GET | /questions/{id} |
| `fairing_list_surveys` | List surveys | GET | /surveys |
| `fairing_get_survey` | Get survey details | GET | /surveys/{id} |
| `fairing_list_responses` | List responses | GET | /responses |
| `fairing_get_response` | Get response details | GET | /responses/{id} |
| `fairing_list_referrals` | List referrals | GET | /referrals |
| `fairing_get_stats` | Get survey statistics | GET | /stats |
| `fairing_list_sources` | List traffic sources | GET | /sources |
| `fairing_get_overlay` | Get overlay settings | GET | /overlay |

---

## Tool Details

### fairing_list_questions

**What it does**: Lists all survey questions.

**When to use**: Browse questions, find question IDs.

**Arguments**:
- `survey_id` (optional): Filter by survey

**Example LLM prompt**: "List all survey questions"

---

### fairing_get_question

**What it does**: Gets details of a specific question.

**When to use**: View question text and settings.

**Arguments**:
- `id` (required): Question ID

**Example LLM prompt**: "Get details for question abc123"

---

### fairing_list_surveys

**What it does**: Lists all surveys.

**When to use**: Browse available surveys.

**Arguments**: None

**Example LLM prompt**: "List all surveys"

---

### fairing_get_survey

**What it does**: Gets details of a specific survey.

**When to use**: View survey configuration.

**Arguments**:
- `id` (required): Survey ID

**Example LLM prompt**: "Get details for survey xyz789"

---

### fairing_list_responses

**What it does**: Lists all survey responses.

**When to use**: Browse collected data, find specific responses.

**Arguments**:
- `survey_id` (optional): Filter by survey

**Example LLM prompt**: "List all responses for survey xyz789"

---

### fairing_get_response

**What it does**: Gets details of a specific response.

**When to use**: View individual response data.

**Arguments**:
- `id` (required): Response ID

**Example LLM prompt**: "Get response def456"

---

### fairing_list_referrals

**What it does**: Lists referral data.

**When to use**: Track marketing attribution, see referral sources.

**Arguments**:
- `date_from` (optional): Start date
- `date_to` (optional): End date

**Example LLM prompt**: "List referrals from the last month"

---

### fairing_get_stats

**What it does**: Gets survey statistics and metrics.

**When to use**: Analyze survey performance.

**Arguments**:
- `survey_id` (optional): Survey ID

**Example LLM prompt**: "Get stats for survey xyz789"

---

### fairing_list_sources

**What it does**: Lists traffic sources for responses.

**When to use**: Understand where responses come from.

**Arguments**: None

**Example LLM prompt**: "List all traffic sources"

---

### fairing_get_overlay

**What it does**: Gets survey overlay settings.

**When to use**: Check display configuration.

**Arguments**:
- `survey_id` (optional): Survey ID

**Example LLM prompt**: "Get overlay settings"

---

## Fairing API Notes

- **Surveys**: Post-purchase survey campaigns
- **Questions**: Survey questions with various types
- **Responses**: Customer answers to surveys
- **Referrals**: Marketing attribution data
- **Sources**: Traffic source tracking
- **Overlay**: Survey display settings
