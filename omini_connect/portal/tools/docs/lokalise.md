# Lokalise Tools

Provider: `lokalise` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Lokalise API. They allow AI agents to manage projects, keys, translations, and localization workflows. **Requires Lokalise OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Lokalise
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.lokalise.com/api2`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lokalise_list_projects` | List projects | GET | /api2/projects |
| `lokalise_get_project` | Get project details | GET | /api2/projects/{project_id} |
| `lokalise_list_keys` | List keys | GET | /api2/projects/{project_id}/keys |
| `lokalise_get_key` | Get key details | GET | /api2/projects/{project_id}/keys/{key_id} |
| `lokalise_create_key` | Create a key | POST | /api2/projects/{project_id}/keys |
| `lokalise_list_translations` | List translations | GET | /api2/projects/{project_id}/keys/{key_id}/translations |
| `lokalise_list_languages` | List languages | GET | /api2/projects/{project_id}/languages |
| `lokalise_get_language` | Get language details | GET | /api2/projects/{project_id}/languages/{language_id} |
| `lokalise_list_tasks` | List tasks | GET | /api2/projects/{project_id}/tasks |
| `lokalise_get_task` | Get task details | GET | /api2/projects/{project_id}/tasks/{task_id} |

---

## Tool Details

### lokalise_list_projects

**What it does**: Lists all projects.

**When to use**: Find projects, view workspaces.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all projects in Lokalise"

---

### lokalise_get_project

**What it does**: Gets details for a specific project.

**When to use**: Get project information.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project abc123"

---

### lokalise_list_keys

**What it does**: Lists all keys in a project.

**When to use**: View translation keys.

**Arguments**:
- `project_id` (required): Project ID
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all keys in project abc123"

---

### lokalise_get_key

**What it does**: Gets details for a specific key.

**When to use**: Get key information.

**Arguments**:
- `project_id` (required): Project ID
- `key_id` (required): Key ID

**Example LLM prompt**: "Get details for key 123 in project abc123"

---

### lokalise_create_key

**What it does**: Creates a new key.

**When to use**: Add translation keys.

**Arguments**:
- `project_id` (required): Project ID
- `key_name` (required): Key name
- `description` (optional): Key description

**Example LLM prompt**: "Create a key named 'greeting' in project abc123"

---

### lokalise_list_translations

**What it does**: Lists all translations for a key.

**When to use**: View translations for a key.

**Arguments**:
- `project_id` (required): Project ID
- `key_id` (required): Key ID

**Example LLM prompt**: "List translations for key 123 in project abc123"

---

### lokalise_list_languages

**What it does**: Lists all project languages.

**When to use**: View supported languages.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "List languages in project abc123"

---

### lokalise_get_language

**What it does**: Gets details for a specific language.

**When to use**: Get language information.

**Arguments**:
- `project_id` (required): Project ID
- `language_id` (required): Language ID

**Example LLM prompt**: "Get details for language 1 in project abc123"

---

### lokalise_list_tasks

**What it does**: Lists all tasks in a project.

**When to use**: View translation tasks.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "List tasks in project abc123"

---

### lokalise_get_task

**What it does**: Gets details for a specific task.

**When to use**: Get task information.

**Arguments**:
- `project_id` (required): Project ID
- `task_id` (required): Task ID

**Example LLM prompt**: "Get details for task t1 in project abc123"

---

## Lokalise API Notes

- **Localization**: Translation management platform
- **Projects**: Translation projects
- **Keys**: Translation keys (e.g., "greeting", "button.submit")
- **Translations**: Translations for each key
- **Languages**: Supported languages
- **Tasks**: Translation workflow tasks
