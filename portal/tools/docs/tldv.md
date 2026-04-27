# TLDV Tools

Provider: `tldv` | Engine: `nango` | Auth: API Key via Nango

## Overview

TLDV is a meeting intelligence platform for recording and summarizing meetings. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their TLDV API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tldv_list_meetings` | List all recorded meetings | GET | /api/v1/meetings |
| `tldv_get_meeting` | Get meeting details with transcript | GET | /api/v1/meetings/{id} |
| `tldv_list_transcripts` | List meeting transcripts | GET | /api/v1/transcripts |
| `tldv_get_transcript` | Get transcript for a meeting | GET | /api/v1/meetings/{id}/transcript |
| `tldv_list_summaries` | List AI-generated summaries | GET | /api/v1/summaries |
| `tldv_get_summary` | Get summary for a meeting | GET | /api/v1/meetings/{id}/summary |
| `tldv_create_action_item` | Create an action item from meeting | POST | /api/v1/action-items |
| `tldv_list_action_items` | List all action items | GET | /api/v1/action-items |
| `tldv_share_meeting` | Share a meeting recording | POST | /api/v1/meetings/{id}/share |
| `tldv_delete_meeting` | Delete a meeting recording | DELETE | /api/v1/meetings/{id} |

---

## Tool Details

### tldv_list_meetings

**What it does**: List all recorded meetings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_get_meeting

**What it does**: Get meeting details with transcript

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_list_transcripts

**What it does**: List meeting transcripts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_get_transcript

**What it does**: Get transcript for a meeting

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_list_summaries

**What it does**: List AI-generated summaries

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_get_summary

**What it does**: Get summary for a meeting

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_create_action_item

**What it does**: Create an action item from meeting

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_list_action_items

**What it does**: List all action items

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_share_meeting

**What it does**: Share a meeting recording

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tldv_delete_meeting

**What it does**: Delete a meeting recording

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://pasta.tldv.io`
- Docs: https://nango.dev/docs/integrations/all/tldv
