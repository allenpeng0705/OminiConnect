# Kintone Tools

Provider: `kintone` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Kintone API. They allow AI agents to manage records, apps, and comments. **Requires Kintone OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Kintone
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{subdomain}.kintone.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `kintone_list_records` | List records | GET | /k/v1/records.json |
| `kintone_get_record` | Get record details | GET | /k/v1/record.json |
| `kintone_create_record` | Create a record | POST | /k/v1/record.json |
| `kintone_update_record` | Update a record | PUT | /k/v1/record.json |
| `kintone_delete_record` | Delete a record | DELETE | /k/v1/records.json |
| `kintone_list_apps` | List apps | GET | /k/v1/apps.json |
| `kintone_get_app` | Get app details | GET | /k/v1/app.json |
| `kintone_list_comments` | List comments | GET | /k/v1/comments.json |
| `kintone_add_comment` | Add a comment | POST | /k/v1/comment.json |
| `kintone_get_form_fields` | Get form fields | GET | /k/v1/form.json |

---

## Tool Details

### kintone_list_records

**What it does**: Lists records in an app.

**When to use**: Find records, query database.

**Arguments**:
- `app` (required): App ID
- `query` (optional): Kintone query string
- `fields` (optional): Fields to return
- `limit` (optional): Max results (default 100, max 500)

**Example LLM prompt**: "List all records in app 1"

---

### kintone_get_record

**What it does**: Gets details for a specific record.

**When to use**: Get record information.

**Arguments**:
- `app` (required): App ID
- `id` (required): Record ID

**Example LLM prompt**: "Get record 123 from app 1"

---

### kintone_create_record

**What it does**: Creates a new record.

**When to use**: Add records to an app.

**Arguments**:
- `app` (required): App ID
- `record` (required): Record data

**Example LLM prompt**: "Create a new record in app 1"

---

### kintone_update_record

**What it does**: Updates an existing record.

**When to use**: Modify record data.

**Arguments**:
- `app` (required): App ID
- `id` (required): Record ID
- `record` (required): Record data

**Example LLM prompt**: "Update record 123 in app 1"

---

### kintone_delete_record

**What it does**: Deletes a record.

**When to use**: Remove records.

**Arguments**:
- `app` (required): App ID
- `ids` (required): Record IDs to delete

**Example LLM prompt**: "Delete records 123, 124 from app 1"

---

### kintone_list_apps

**What it does**: Lists all apps.

**When to use**: Find apps, view available databases.

**Arguments**:
- `space_id` (optional): Filter by space ID

**Example LLM prompt**: "List all apps in Kintone"

---

### kintone_get_app

**What it does**: Gets details for a specific app.

**When to use**: Get app information.

**Arguments**:
- `app` (required): App ID

**Example LLM prompt**: "Get details for app 1"

---

### kintone_list_comments

**What it does**: Lists comments on a record.

**When to use**: View discussion, read comments.

**Arguments**:
- `app` (required): App ID
- `record_id` (required): Record ID

**Example LLM prompt**: "List comments for record 123 in app 1"

---

### kintone_add_comment

**What it does**: Adds a comment to a record.

**When to use**: Add notes, respond to records.

**Arguments**:
- `app` (required): App ID
- `record_id` (required): Record ID
- `comment` (required): Comment object with text

**Example LLM prompt**: "Add a comment to record 123 in app 1"

---

### kintone_get_form_fields

**What it does**: Gets form fields for an app.

**When to use**: Understand app structure.

**Arguments**:
- `app` (required): App ID

**Example LLM prompt**: "Get form fields for app 1"

---

## Kintone API Notes

- **Apps**: Database applications with forms and fields
- **Records**: Data entries in an app
- **Query Language**: Kintone query syntax for filtering
- **Comments**: Discussion on records
