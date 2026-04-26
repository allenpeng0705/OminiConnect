# Procore Tools

Provider: `procore` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Procore API. They allow AI agents to manage projects, drawings, RFIs, submittals, and observations. Procore is a construction management platform. **Requires Procore OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Procore
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.procore.com
- Requires companyId in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `procore_list_projects` | List projects | GET | /v1/projects |
| `procore_get_project` | Get project details | GET | /v1/projects/{projectId} |
| `procore_list_drawings` | List drawings | GET | /v1/projects/{projectId}/drawings |
| `procore_get_drawing` | Get drawing details | GET | /v1/projects/{projectId}/drawings/{drawingId} |
| `procore_list_rfis` | List RFIs | GET | /v1/projects/{projectId}/rfis |
| `procore_get_rfi` | Get RFI details | GET | /v1/projects/{projectId}/rfis/{rfiId} |
| `procore_list_submittals` | List submittals | GET | /v1/projects/{projectId}/submittals |
| `procore_get_submittal` | Get submittal details | GET | /v1/projects/{projectId}/submittals/{submittalId} |
| `procore_list_observations` | List observations | GET | /v1/projects/{projectId}/observations |
| `procore_get_company` | Get company info | GET | /v1/company |

---

## Tool Details

### procore_list_projects

**What it does**: Lists all projects in the organization.

**When to use**: Browse project portfolio.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active projects"

---

### procore_get_project

**What it does**: Gets detailed information about a specific project.

**When to use**: Get project details, overview.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "Get details for project 12345"

---

### procore_list_drawings

**What it does**: Lists all drawings in a project.

**When to use**: Browse project drawings.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List drawings for project 12345"

---

### procore_get_drawing

**What it does**: Gets detailed information about a specific drawing.

**When to use**: Get drawing details, revisions.

**Arguments**:
- `projectId` (required): Project ID
- `drawingId` (required): Drawing ID

**Example LLM prompt**: "Get details for drawing 67890"

---

### procore_list_rfis

**What it does**: Lists all RFIs (Requests for Information) in a project.

**When to use**: Browse RFI log.

**Arguments**:
- `projectId` (required): Project ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List open RFIs for project 12345"

---

### procore_get_rfi

**What it does**: Gets detailed information about a specific RFI.

**When to use**: Get RFI details, responses.

**Arguments**:
- `projectId` (required): Project ID
- `rfiId` (required): RFI ID

**Example LLM prompt**: "Get details for RFI 11111"

---

### procore_list_submittals

**What it does**: Lists all submittals in a project.

**When to use**: Browse submittal log.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List submittals for project 12345"

---

### procore_get_submittal

**What it does**: Gets detailed information about a specific submittal.

**When to use**: Get submittal details, status.

**Arguments**:
- `projectId` (required): Project ID
- `submittalId` (required): Submittal ID

**Example LLM prompt**: "Get details for submittal 22222"

---

### procore_list_observations

**What it does**: Lists all observations in a project.

**When to use**: Browse observation log.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List observations for project 12345"

---

### procore_get_company

**What it does**: Gets company information.

**When to use**: Get company settings.

**Arguments**: None

**Example LLM prompt**: "Get company information"

---

## Procore API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Construction Management**: Projects, drawings, RFIs, submittals
- **Rate limits**: Apply to API calls — check X-Rate-Limit-Reset header
