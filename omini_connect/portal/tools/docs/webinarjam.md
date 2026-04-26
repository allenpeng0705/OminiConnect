# WebinarJam Tools

Provider: `webinarjam` | Engine: `nango` | Auth: API Key via Nango

## Overview

WebinarJam is a webinar hosting platform. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their WebinarJam API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `webinarjam_list_webinars` | List all webinars | GET | /webinars |
| `webinarjam_get_webinar` | Get webinar details | GET | /webinars/{id} |
| `webinarjam_list_registrants` | List all registrants for a webinar | GET | /registrants/{webinar_id} |
| `webinarjam_register` | Register a participant | POST | /register |
| `webinarjam_list_attendees` | List attendees for a webinar | GET | /attendees/{webinar_id} |
| `webinarjam_send_broadcast` | Send a broadcast message | POST | /broadcast |
| `webinarjam_get_dashboard` | Get webinar dashboard data | GET | /dashboard/{webinar_id} |
| `webinarjam_list_recordings` | List webinar recordings | GET | /recordings/{webinar_id} |
| `webinarjam_get_poll_results` | Get poll results for a webinar | GET | /polls/{webinar_id} |
| `webinarjam_get_replay` | Get replay URL for a webinar | GET | /replay/{webinar_id} |

---

## Tool Details

### webinarjam_list_webinars

**What it does**: List all webinars

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_get_webinar

**What it does**: Get webinar details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_list_registrants

**What it does**: List all registrants for a webinar

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_register

**What it does**: Register a participant

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_list_attendees

**What it does**: List attendees for a webinar

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_send_broadcast

**What it does**: Send a broadcast message

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_get_dashboard

**What it does**: Get webinar dashboard data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_list_recordings

**What it does**: List webinar recordings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_get_poll_results

**What it does**: Get poll results for a webinar

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### webinarjam_get_replay

**What it does**: Get replay URL for a webinar

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.webinarjam.com/webinarjam`
- Docs: https://nango.dev/docs/integrations/all/webinarjam
