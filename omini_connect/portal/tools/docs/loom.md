# Loom Tools

Provider: `loom` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Loom API. They allow AI agents to manage video recordings, workspaces, folders, and comments. Loom is a popular video messaging platform for async communication and documentation.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Loom
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `videos:r`, `videos:w`, `videos:d`, `workspaces:r`, `folders:r`, `comments:w`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `loom_list_videos` | List all videos | GET | /videos |
| `loom_get_video` | Get video details | GET | /videos/{videoId} |
| `loom_create_video` | Create a new video | POST | /videos |
| `loom_delete_video` | Delete a video | DELETE | /videos/{videoId} |
| `loom_list_workspaces` | List workspaces | GET | /workspaces |
| `loom_get_workspace` | Get workspace details | GET | /workspaces/{workspaceId} |
| `loom_list_folders` | List folders | GET | /folders |
| `loom_get_folder` | Get folder details | GET | /folders/{folderId} |
| `loom_add_comment` | Add comment to video | POST | /videos/{videoId}/comments |
| `loom_update_thumbnail` | Update video thumbnail | PATCH | /videos/{videoId}/thumbnail |

---

## Tool Details

### loom_list_videos

**What it does**: Lists all videos for the authenticated user. Filter by workspace, folder, or search term.

**When to use**: Find videos, browse workspace content, search for specific recordings.

**Arguments**:
- `workspace_id` (optional): Filter by workspace ID
- `folder_id` (optional): Filter by folder ID
- `search` (optional): Search by title or description
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Show me all videos from the Design workspace"

---

### loom_get_video

**What it does**: Gets detailed information about a specific video including duration, view count, thumbnail, and sharing settings.

**When to use**: Get video metadata before sharing or downloading.

**Arguments**:
- `videoId` (required): Video ID

**Example LLM prompt**: "What is the duration of video abc-123?"

---

### loom_create_video

**What it does**: Creates a new video entry with title and metadata. For actual video upload, use the upload endpoint.

**When to use**: Create a video record before uploading the actual video file.

**Arguments**:
- `title` (required): Video title
- `description` (optional): Video description
- `workspace_id` (optional): Workspace ID
- `folder_id` (optional): Folder ID
- `url` (optional): Video URL

**Example LLM prompt**: "Create a new video titled 'Product Demo Q4' in the Marketing workspace"

---

### loom_delete_video

**What it does**: Deletes a video permanently. This action cannot be undone.

**When to use**: Remove unwanted or outdated video recordings.

**Arguments**:
- `videoId` (required): Video ID

**Example LLM prompt**: "Delete the video with ID abc-123"

---

### loom_list_workspaces

**What it does**: Lists all workspaces available to the authenticated user. Workspaces contain folders and videos.

**When to use**: Find available workspaces, see organization structure.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "What workspaces do I have access to?"

---

### loom_get_workspace

**What it does**: Gets detailed information about a specific workspace including member list and settings.

**When to use**: Get workspace details, understand team structure.

**Arguments**:
- `workspaceId` (required): Workspace ID

**Example LLM prompt**: "How many members are in the Design workspace?"

---

### loom_list_folders

**What it does**: Lists all folders in a workspace or parent folder. Folders organize videos within a workspace.

**When to use**: Browse workspace organization, find folders for video storage.

**Arguments**:
- `workspace_id` (optional): Workspace ID
- `parent_folder_id` (optional): Parent folder ID for nested folders
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "Show me all folders in the Marketing workspace"

---

### loom_get_folder

**What it does**: Gets detailed information about a specific folder including contents and subfolder listing.

**When to use**: Browse folder contents, see what videos are stored in a folder.

**Arguments**:
- `folderId` (required): Folder ID

**Example LLM prompt**: "What videos are in the Q4 Demos folder?"

---

### loom_add_comment

**What it does**: Adds a comment to a video at a specific timestamp. Comments can be text or time-linked.

**When to use**: Provide feedback on a video, ask questions at specific moments.

**Arguments**:
- `videoId` (required): Video ID
- `text` (required): Comment text
- `timestamp` (optional): Video timestamp in seconds for time-linked comments

**Example LLM prompt**: "Add a comment on video abc-123 at the 2-minute mark saying 'great point about the workflow'"

---

### loom_update_thumbnail

**What it does**: Updates the thumbnail image for a video. Can specify a timestamp to capture frame or upload custom image.

**When to use**: Improve video preview, set a specific frame as thumbnail.

**Arguments**:
- `videoId` (required): Video ID
- `timestamp` (optional): Capture frame at this timestamp (seconds)
- `image_url` (optional): URL of custom thumbnail image

**Example LLM prompt**: "Update the thumbnail for video abc-123 to show the frame at 30 seconds"

---

## Loom API Notes

- **Video IDs**: Loom uses alphanumeric IDs (e.g., `abc123def456`) for video identification
- **Workspace IDs**: Alphanumeric identifiers for workspaces
- **Folder IDs**: Alphanumeric identifiers for folders
- **Share URLs**: Each video has a share URL (e.g., `https://www.loom.com/share/abc123`)
- **Thumbnails**: Videos have automatically generated thumbnails, but can be customized
- **Timestamps**: Video timestamps are in seconds for time-linked comments and thumbnail capture
- **Permissions**: Videos can have different sharing permissions (public, private, team-only)
- **Workspaces**: Top-level organizational unit containing folders and videos
- **Folders**: Hierarchical organization within workspaces for grouping videos