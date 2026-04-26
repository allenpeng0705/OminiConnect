# SharePoint Online (v2) Tools

Provider: `sharepoint-online` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the SharePoint Online (v2) API. They allow AI agents to interact with SharePoint Online (v2) functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_sites` | List SharePoint sites | GET | /sites |
| `get_site` | Get site details | GET | /sites/{site_id} |
| `list_drives` | List document libraries | GET | /sites/{site_id}/drives |
| `list_files` | List files in drive | GET | /sites/{site_id}/drives/{drive_id}/root/children |
| `get_file` | Get file metadata | GET | /sites/{site_id}/drives/{drive_id}/items/{file_id} |
| `download_file` | Download file content | GET | /sites/{site_id}/drives/{drive_id}/items/{file_id}/content |
| `upload_file` | Upload a file | POST | /sites/{site_id}/drives/{drive_id}/root/children |
| `create_folder` | Create a folder | POST | /sites/{site_id}/drives/{drive_id}/items/{parent_id} |
| `delete_file` | Delete a file or folder | DELETE | /sites/{site_id}/drives/{drive_id}/items/{file_id} |
| `list_permissions` | List item permissions | GET | /sites/{site_id}/drives/{drive_id}/items/{file_id}/permissions |

---

## Tool Details

### list_sites

**What it does**: List SharePoint sites

**When to use**: Use this tool when you need to list sharepoint sites.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list sites to..."

---

### get_site

**What it does**: Get site details

**When to use**: Use this tool when you need to get site details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get site to..."

---

### list_drives

**What it does**: List document libraries

**When to use**: Use this tool when you need to list document libraries.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list drives to..."

---

### list_files

**What it does**: List files in drive

**When to use**: Use this tool when you need to list files in drive.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list files to..."

---

### get_file

**What it does**: Get file metadata

**When to use**: Use this tool when you need to get file metadata.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get file to..."

---

### download_file

**What it does**: Download file content

**When to use**: Use this tool when you need to download file content.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use download file to..."

---

### upload_file

**What it does**: Upload a file

**When to use**: Use this tool when you need to upload a file.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload file to..."

---

### create_folder

**What it does**: Create a folder

**When to use**: Use this tool when you need to create a folder.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create folder to..."

---

### delete_file

**What it does**: Delete a file or folder

**When to use**: Use this tool when you need to delete a file or folder.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use delete file to..."

---

### list_permissions

**What it does**: List item permissions

**When to use**: Use this tool when you need to list item permissions.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list permissions to..."

---

## SharePoint Online (v2) API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://graph.microsoft.com
- **API prefix**: /v1.0
- **Rate limits**: Check provider documentation for specific limits
