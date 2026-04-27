# CompanyCam Tools

Provider: `companycam` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the CompanyCam API. CompanyCam is a visual project management platform for contractors and construction professionals. **Requires CompanyCam API token.**

## Authentication

**Nango API_KEY**:
- User provides their CompanyCam access token
- Token passed via `Authorization: Bearer` header
- Base URL: `https://api.companycam.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `companycam_list_projects` | List projects | GET | /v2/projects |
| `companycam_get_project` | Get project details | GET | /v2/projects/{id} |
| `companycam_create_project` | Create a project | POST | /v2/projects |
| `companycam_list_photos` | List photos | GET | /v2/projects/{projectId}/photos |
| `companycam_get_photo` | Get photo details | GET | /v2/photos/{id} |
| `companycam_upload_photo` | Upload a photo | POST | /v2/projects/{projectId}/photos |
| `companycam_list_notes` | List notes | GET | /v2/projects/{projectId}/notes |
| `companycam_create_note` | Create a note | POST | /v2/projects/{projectId}/notes |
| `companycam_list_members` | List project members | GET | /v2/projects/{projectId}/members |
| `companycam_get_company` | Get company info | GET | /v2/company |

---

## Tool Details

### companycam_list_projects

**What it does**: Lists all projects in your CompanyCam account.

**When to use**: Find projects to work with.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all CompanyCam projects"

---

### companycam_get_project

**What it does**: Gets details of a specific project.

**When to use**: View project overview and settings.

**Arguments**:
- `id` (required): Project ID

**Example LLM prompt**: "Get project 123 details"

---

### companycam_create_project

**What it does**: Creates a new project.

**When to use**: Start a new project.

**Arguments**:
- `name` (required): Project name
- `description` (optional): Project description

**Example LLM prompt**: "Create a project called 'Kitchen Remodel'"

---

### companycam_list_photos

**What it does**: Lists all photos in a project.

**When to use**: View project visual documentation.

**Arguments**:
- `projectId` (required): Project ID
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List photos for project 123"

---

### companycam_get_photo

**What it does**: Gets details of a specific photo.

**When to use**: View photo metadata.

**Arguments**:
- `id` (required): Photo ID

**Example LLM prompt**: "Get photo 456 details"

---

### companycam_upload_photo

**What it does**: Uploads a photo to a project.

**When to use**: Add visual documentation to a project.

**Arguments**:
- `projectId` (required): Project ID
- `url` (required): Photo URL
- `name` (optional): Photo name

**Example LLM prompt**: "Upload a photo to project 123"

---

### companycam_list_notes

**What it does**: Lists all notes in a project.

**When to use**: View project discussions.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List notes for project 123"

---

### companycam_create_note

**What it does**: Creates a new note in a project.

**When to use**: Add discussion to a project.

**Arguments**:
- `projectId` (required): Project ID
- `content` (required): Note content

**Example LLM prompt**: "Add a note to project 123"

---

### companycam_list_members

**What it does**: Lists all members of a project.

**When to use**: View project team.

**Arguments**:
- `projectId` (required): Project ID

**Example LLM prompt**: "List members of project 123"

---

### companycam_get_company

**What it does**: Gets company information.

**When to use**: View company settings.

**Arguments**: None

**Example LLM prompt**: "Get my CompanyCam company info"

---

## CompanyCam API Notes

- **Projects**: Visual project documentation spaces
- **Photos**: Site photos attached to projects
- **Notes**: Text discussions on projects
- **Members**: Team members with access to projects
