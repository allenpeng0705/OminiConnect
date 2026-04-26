# Tally Tools

Provider: `tally` | Engine: `nango` | Auth: API Key via Nango

## Overview

Tally is the simplest way to create beautiful forms for data collection. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Tally API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tally_list_forms` | List all forms in your Tally account | GET | /forms |
| `tally_get_form` | Get details of a specific form | GET | /forms/{form_id} |
| `tally_get_form_schema` | Get the schema/fields of a form | GET | /forms/{form_id}/schema |
| `tally_list_submissions` | List all submissions for a form | GET | /forms/{form_id}/submissions |
| `tally_get_submission` | Get a specific submission | GET | /forms/{form_id}/submissions/{submission_id} |
| `tally_create_form` | Create a new form | POST | /forms |
| `tally_update_form` | Update an existing form | PUT | /forms/{form_id} |
| `tally_delete_form` | Delete a form | DELETE | /forms/{form_id} |
| `tally_calculate_form` | Calculate form metrics | GET | /forms/{form_id}/calculate |
| `tally_list_webhooks` | List webhooks for a form | GET | /forms/{form_id}/webhooks |

---

## Tool Details

### tally_list_forms

**What it does**: List all forms in your Tally account

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_get_form

**What it does**: Get details of a specific form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_get_form_schema

**What it does**: Get the schema/fields of a form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_list_submissions

**What it does**: List all submissions for a form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_get_submission

**What it does**: Get a specific submission

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_create_form

**What it does**: Create a new form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_update_form

**What it does**: Update an existing form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_delete_form

**What it does**: Delete a form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_calculate_form

**What it does**: Calculate form metrics

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tally_list_webhooks

**What it does**: List webhooks for a form

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.tally.so`
- Docs: https://nango.dev/docs/api-integrations/tally
