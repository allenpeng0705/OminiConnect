# Google Service Account Tools

Provider: `google-service-account` | Engine: `nango` | Auth: TWO_STEP (OAuth2 with RSA) via Nango

## Overview

These tools wrap the Google Cloud API using service account authentication. They allow AI agents to manage projects, services, resources, quotas, and billing. **Requires Google Service Account credentials.**

## Authentication

**Nango TWO_STEP (Service Account)**:
- Uses RSA private key to generate JWT tokens
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://cloudresourcemanager.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gcp_list_projects` | List projects | GET | /v1/projects |
| `gcp_get_project` | Get project details | GET | /v1/projects/{project_id} |
| `gcp_list_services` | List enabled services | GET | /v1/projects/{project_id}/services |
| `gcp_enable_service` | Enable a service | POST | /v1/projects/{project_id}/services/{service_name} |
| `gcp_list_resources` | List resources | GET | /v1/{parent}/resources |
| `gcp_get_quota` | Get quota information | GET | /v1/{project}/locations/{location}/quotas |
| `gcp_list_regions` | List regions | GET | /v1/projects/{project_id}/regions |
| `gcp_list_zones` | List zones | GET | /v1/projects/{project_id}/zones |
| `gcp_get_billing` | Get billing info | GET | /v1/projects/{project_id}/billingInfo |
| `gcp_list_operations` | List operations | GET | /v1/{name}/operations |

---

## Tool Details

### gcp_list_projects

**What it does**: Lists Google Cloud projects.

**When to use**: Browse available projects.

**Arguments**: None

**Example LLM prompt**: "List all my GCP projects"

---

### gcp_get_project

**What it does**: Gets details about a project.

**When to use**: Get project info and settings.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project my-project-123"

---

### gcp_list_services

**What it does**: Lists enabled services for a project.

**When to use**: See what APIs are enabled.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "List enabled services for project my-project-123"

---

### gcp_enable_service

**What it does**: Enables a service for a project.

**When to use**: Enable an API.

**Arguments**:
- `project_id` (required): Project ID
- `service_name` (required): Service name

**Example LLM prompt**: "Enable BigQuery API for project my-project-123"

---

### gcp_list_resources

**What it does**: Lists cloud resources.

**When to use**: Browse resources in a project.

**Arguments**:
- `parent` (required): Parent resource path

**Example LLM prompt**: "List resources in project my-project-123"

---

### gcp_get_quota

**What it does**: Gets quota information for a service.

**When to use**: Check usage limits.

**Arguments**:
- `project` (required): Project ID
- `location` (required): Location

**Example LLM prompt**: "Get quota for us-central1"

---

### gcp_list_regions

**What it does**: Lists available regions.

**When to use**: See where you can deploy.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "List regions for project my-project-123"

---

### gcp_list_zones

**What it does**: Lists available zones.

**When to use**: See where you can deploy.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "List zones for project my-project-123"

---

### gcp_get_billing

**What it does**: Gets billing information for a project.

**When to use**: Check billing status.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get billing info for project my-project-123"

---

### gcp_list_operations

**What it does**: Lists long-running operations.

**When to use**: Track async operations.

**Arguments**:
- `name` (required): Parent resource name

**Example LLM prompt**: "List operations for project my-project-123"

---

## Google Service Account API Notes

- **Service Account**: Uses RSA key pair for authentication
- **Delegation**: Can impersonate users with delegated access
- **Scopes**: Control what resources can be accessed
- **Projects**: Top-level containers for resources
- **Billing**: Must be linked to use paid services
