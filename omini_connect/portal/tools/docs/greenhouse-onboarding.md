# Greenhouse Onboarding Tools

Provider: `greenhouse-onboarding` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the Greenhouse Onboarding API. They allow AI agents to manage employees in the onboarding process, including tasks, forms, groups, and documents. Greenhouse Onboarding handles new hire onboarding workflows.

## Authentication

**Nango Basic Auth**:
- User provides access key via Nango Connect
- Uses access key as username with empty password
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://onboarding-api.greenhouse.io

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `greenhouse_onboarding_list_employees` | List onboarding employees | GET | /employees |
| `greenhouse_onboarding_get_employee` | Get onboarding employee | GET | /employees/{id} |
| `greenhouse_onboarding_list_tasks` | List onboarding tasks | GET | /tasks |
| `greenhouse_onboarding_get_task` | Get task details | GET | /tasks/{id} |
| `greenhouse_onboarding_list_forms` | List onboarding forms | GET | /forms |
| `greenhouse_onboarding_get_form` | Get form details | GET | /forms/{id} |
| `greenhouse_onboarding_list_groups` | List onboarding groups | GET | /groups |
| `greenhouse_onboarding_get_group` | Get group details | GET | /groups/{id} |
| `greenhouse_onboarding_list_documents` | List documents | GET | /documents |
| `greenhouse_onboarding_get_document` | Get document details | GET | /documents/{id} |

---

## Tool Details

### greenhouse_onboarding_list_employees

**What it does**: Lists all employees in the onboarding process.

**When to use**: View all employees being onboarded.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all onboarding employees"

---

### greenhouse_onboarding_get_employee

**What it does**: Gets detailed information about an onboarding employee.

**When to use**: View employee's onboarding progress and tasks.

**Arguments**:
- `id` (required): Employee ID

**Example LLM prompt**: "Get onboarding employee with ID 123"

---

### greenhouse_onboarding_list_tasks

**What it does**: Lists all onboarding tasks.

**When to use**: View all onboarding tasks.

**Arguments**:
- `employee_id` (optional): Filter by employee ID

**Example LLM prompt**: "List all onboarding tasks"

---

### greenhouse_onboarding_get_task

**What it does**: Gets detailed information about an onboarding task.

**When to use**: View task details and assignee.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get task with ID 456"

---

### greenhouse_onboarding_list_forms

**What it does**: Lists all onboarding forms.

**When to use**: View available onboarding forms.

**Arguments**: None

**Example LLM prompt**: "List all onboarding forms"

---

### greenhouse_onboarding_get_form

**What it does**: Gets detailed information about an onboarding form.

**When to use**: View form fields and requirements.

**Arguments**:
- `id` (required): Form ID

**Example LLM prompt**: "Get form with ID 789"

---

### greenhouse_onboarding_list_groups

**What it does**: Lists all onboarding groups.

**When to use**: View onboarding group structure.

**Arguments**: None

**Example LLM prompt**: "List all onboarding groups"

---

### greenhouse_onboarding_get_group

**What it does**: Gets detailed information about an onboarding group.

**When to use**: View group members and assigned tasks.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get group with ID 101"

---

### greenhouse_onboarding_list_documents

**What it does**: Lists all onboarding documents.

**When to use**: View available onboarding documents.

**Arguments**: None

**Example LLM prompt**: "List all onboarding documents"

---

### greenhouse_onboarding_get_document

**What it does**: Gets detailed information about an onboarding document.

**When to use**: View document content.

**Arguments**:
- `id` (required): Document ID

**Example LLM prompt**: "Get document with ID 202"

---

## Greenhouse Onboarding API Notes

- **Base URL**: https://onboarding-api.greenhouse.io
- **Auth Mode**: Basic Auth with access key as username
- **Employees**: New hires in onboarding process
- **Tasks**: Onboarding tasks assigned to employees
- **Forms**: Digital forms for new hire completion
- **Groups**: Cohorts of employees onboarding together
- **Documents**: Onboarding materials and resources
