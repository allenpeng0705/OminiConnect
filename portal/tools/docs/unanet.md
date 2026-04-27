# Unanet Tools

Provider: `unanet` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

Unanet is a project management and resource planning platform. **Requires basic auth via nango.**

## Authentication

**Nango Basic Auth**:
- User provides username/password
- Credentials stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `unanet_list_projects` | List all projects | GET | /api/v1/projects |
| `unanet_get_project` | Get project details | GET | /api/v1/projects/{id} |
| `unanet_list_tasks` | List all tasks | GET | /api/v1/tasks |
| `unanet_get_task` | Get task details | GET | /api/v1/tasks/{id} |
| `unanet_list_resources` | List all resources/people | GET | /api/v1/resources |
| `unanet_get_resource` | Get resource details | GET | /api/v1/resources/{id} |
| `unanet_list_timesheets` | List timesheets | GET | /api/v1/timesheets |
| `unanet_submit_timesheet` | Submit a timesheet | POST | /api/v1/timesheets/{id}/submit |
| `unanet_list_clients` | List all clients | GET | /api/v1/clients |
| `unanet_get_client` | Get client details | GET | /api/v1/clients/{id} |

---

## Tool Details

### unanet_list_projects

**What it does**: List all projects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_get_project

**What it does**: Get project details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_list_tasks

**What it does**: List all tasks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_get_task

**What it does**: Get task details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_list_resources

**What it does**: List all resources/people

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_get_resource

**What it does**: Get resource details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_list_timesheets

**What it does**: List timesheets

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_submit_timesheet

**What it does**: Submit a timesheet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_list_clients

**What it does**: List all clients

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### unanet_get_client

**What it does**: Get client details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://compass.cosential.com`
- Docs: https://nango.dev/docs/integrations/all/unanet
