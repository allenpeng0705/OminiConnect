# EvaluAgent Tools

Provider: `evaluagent` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the EvaluAgent API. They allow AI agents to manage quality assurance audits, view conversations, and track agent performance. EvaluAgent is a quality management platform for call centers and customer service teams.

## Authentication

**Nango BASIC**:
- User provides region and credentials (Access Key ID + Secret Key) via Nango Connect
- Region determines API endpoint: `https://{region}.evaluagent.com`
- Credentials are base64 encoded for Basic auth

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `evaluagent_list_audits` | List audits | GET | /v1/audits |
| `evaluagent_get_audit` | Get audit details | GET | /v1/audits/{id} |
| `evaluagent_list_conversations` | List conversations | GET | /v1/conversations |
| `evaluagent_get_conversation` | Get conversation details | GET | /v1/conversations/{id} |
| `evaluagent_list_agents` | List agents | GET | /v1/agents |
| `evaluagent_get_agent` | Get agent details | GET | /v1/agents/{id} |
| `evaluagent_list_teams` | List teams | GET | /v1/teams |
| `evaluagent_get_quality_score` | Get quality score | GET | /v1/quality-scores |
| `evaluagent_list_calibrations` | List calibrations | GET | /v1/calibrations |
| `evaluagent_get_calibration` | Get calibration details | GET | /v1/calibrations/{id} |

---

## Tool Details

### evaluagent_list_audits

**What it does**: Lists all quality assurance audits with pagination.

**When to use**: Review audit history, find specific audits.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Results per page (default 20)

**Example LLM prompt**: "List all recent audits"

---

### evaluagent_get_audit

**What it does**: Gets detailed information about a specific audit.

**When to use**: Review audit results, check scoring.

**Arguments**:
- `id` (required): Audit ID

**Example LLM prompt**: "Get details for audit abc123"

---

### evaluagent_list_conversations

**What it does**: Lists customer service conversations with filtering.

**When to use**: Review call logs, find specific interactions.

**Arguments**:
- `agent_id` (optional): Filter by agent
- `team_id` (optional): Filter by team
- `page` (optional): Page number

**Example LLM prompt**: "List conversations for agent 456"

---

### evaluagent_get_conversation

**What it does**: Gets details of a specific conversation.

**When to use**: Review call details, listen to recordings.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation def789"

---

### evaluagent_list_agents

**What it does**: Lists all agents in the organization.

**When to use**: Browse agent roster, find agent IDs.

**Arguments**:
- `team_id` (optional): Filter by team

**Example LLM prompt**: "List all agents in team support"

---

### evaluagent_get_agent

**What it does**: Gets details of a specific agent.

**When to use**: Get agent info, check agent performance.

**Arguments**:
- `id` (required): Agent ID

**Example LLM prompt**: "Get details for agent ghi123"

---

### evaluagent_list_teams

**What it does**: Lists all teams in the organization.

**When to use**: View organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all teams"

---

### evaluagent_get_quality_score

**What it does**: Gets quality score for an agent or team.

**When to use**: Track performance, identify coaching needs.

**Arguments**:
- `agent_id` (optional): Agent ID
- `team_id` (optional): Team ID

**Example LLM prompt**: "Get quality score for agent jkl456"

---

### evaluagent_list_calibrations

**What it does**: Lists calibration sessions with pagination.

**When to use**: Review calibration history, track consistency.

**Arguments**:
- `page` (optional): Page number

**Example LLM prompt**: "List recent calibrations"

---

### evaluagent_get_calibration

**What it does**: Gets details of a specific calibration session.

**When to use**: Review calibration results, check scoring consistency.

**Arguments**:
- `id` (required): Calibration ID

**Example LLM prompt**: "Get details for calibration mno789"

---

## EvaluAgent API Notes

- **Regions**: Different deployments per customer (eu, us, etc.)
- **Agents**: Customer service representatives being evaluated
- **Audits**: Quality assurance evaluations of conversations
- **Calibrations**: Sessions to ensure scoring consistency
- **Quality Scores**: Performance metrics for agents and teams
