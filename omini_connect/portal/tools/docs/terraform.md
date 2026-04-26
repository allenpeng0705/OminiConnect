# Terraform Cloud Tools

Provider: `terraform` | Engine: `nango` | Auth: API Key via Nango

## Overview

Terraform Cloud is an infrastructure as code platform for provisioning and management. **Requires api key via nango.**

## Authentication

**Nango API Key**:
- User provides their Terraform Cloud API key
- Key stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `terraform_list_workspaces` | List all workspaces in Terraform Cloud | GET | /api/v2/workspaces |
| `terraform_get_workspace` | Get details of a workspace | GET | /api/v2/workspaces/{workspace_id} |
| `terraform_list_runs` | List runs in a workspace | GET | /api/v2/workspaces/{workspace_id}/runs |
| `terraform_create_run` | Create a new run in a workspace | POST | /api/v2/runs |
| `terraform_get_run` | Get details of a run | GET | /api/v2/runs/{run_id} |
| `terraform_apply_run` | Apply a pending run | POST | /api/v2/runs/{run_id}/actions/apply |
| `terraform_discard_run` | Discard a run | POST | /api/v2/runs/{run_id}/actions/discard |
| `terraform_list_state_versions` | List state versions for a workspace | GET | /api/v2/workspaces/{workspace_id}/state-versions |
| `terraform_get_current_state` | Get the current state of a workspace | GET | /api/v2/workspaces/{workspace_id}/current-state-version |
| `terraform_list_organizations` | List all organizations | GET | /api/v2/organizations |

---

## Tool Details

### terraform_list_workspaces

**What it does**: List all workspaces in Terraform Cloud

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_get_workspace

**What it does**: Get details of a workspace

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_list_runs

**What it does**: List runs in a workspace

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_create_run

**What it does**: Create a new run in a workspace

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_get_run

**What it does**: Get details of a run

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_apply_run

**What it does**: Apply a pending run

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_discard_run

**What it does**: Discard a run

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_list_state_versions

**What it does**: List state versions for a workspace

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_get_current_state

**What it does**: Get the current state of a workspace

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### terraform_list_organizations

**What it does**: List all organizations

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://app.terraform.io`
- Docs: https://nango.dev/docs/integrations/all/terraform
