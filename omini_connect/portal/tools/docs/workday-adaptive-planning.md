# Workday Adaptive Planning Tools

Provider: `workday-adaptive-planning` | Engine: `nango` | Auth: Two-Step OAuth via Nango

## Overview

Workday Adaptive Planning is a cloud-based budgeting and planning solution. **Requires two-step oauth via nango.**

## Authentication

**Nango Two-Step OAuth**:
- User authenticates via two-step OAuth flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `workday_list_plans` | List all planning plans | GET | /api/v1/plans |
| `workday_get_plan` | Get plan details | GET | /api/v1/plans/{id} |
| `workday_list_instances` | List all planning instances | GET | /api/v1/instances |
| `workday_get_instance` | Get instance details | GET | /api/v1/instances/{id} |
| `workday_list_sheets` | List all planning sheets | GET | /api/v1/sheets |
| `workday_get_sheet` | Get sheet details | GET | /api/v1/sheets/{id} |
| `workday_get_data` | Get planning data | GET | /api/v1/data |
| `workday_set_data` | Set planning data | POST | /api/v1/data |
| `workday_list_dimensions` | List all dimensions | GET | /api/v1/dimensions |
| `workday_list_levels` | List all levels in the hierarchy | GET | /api/v1/levels |

---

## Tool Details

### workday_list_plans

**What it does**: List all planning plans

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_get_plan

**What it does**: Get plan details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_instances

**What it does**: List all planning instances

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_get_instance

**What it does**: Get instance details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_sheets

**What it does**: List all planning sheets

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_get_sheet

**What it does**: Get sheet details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_get_data

**What it does**: Get planning data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_set_data

**What it does**: Set planning data

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_dimensions

**What it does**: List all dimensions

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### workday_list_levels

**What it does**: List all levels in the hierarchy

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.adaptiveplanning.com`
- Docs: https://nango.dev/docs/integrations/all/workday-adaptive-planning
