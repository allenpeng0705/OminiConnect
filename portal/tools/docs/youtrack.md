# YouTrack

YouTrack is an agile issue tracking system by JetBrains that helps teams manage issues, projects, Agile boards, and time tracking.

## Authentication

OminiConnect uses OAuth2 to connect to YouTrack. During the authorization flow, you'll be asked to grant the following permissions:

| Scope | Description |
|-------|-------------|
| `read` | Read access to issue tracking data |
| `write` | Write access to issue tracking data |

## Configuration

When adding a YouTrack connection, you'll need to provide:

- **YouTrack URL**: Your YouTrack installation URL (e.g., `https://yourcompany.myjetbrains.com/youtrack`)
- **API Token**: Your YouTrack API token (found in your profile settings)

## Available Tools

### Issue Management

| Tool | Description |
|------|-------------|
| `youtrack_list_issues` | Retrieve a list of issues, optionally filtered by project or assignee |
| `youtrack_get_issue` | Retrieve details for a specific issue by its ID |
| `youtrack_create_issue` | Create a new issue in a project |
| `youtrack_update_issue` | Update an existing issue |

### Project Management

| Tool | Description |
|------|-------------|
| `youtrack_list_projects` | Retrieve a list of all projects |
| `youtrack_get_project` | Retrieve details for a specific project by its ID |

### Board Management

| Tool | Description |
|------|-------------|
| `youtrack_list_boards` | Retrieve a list of Agile boards |
| `youtrack_get_board` | Retrieve details for a specific Agile board |

### Time Tracking

| Tool | Description |
|------|-------------|
| `youtrack_list_time_entries` | Retrieve a list of time entries for an issue |
| `youtrack_get_time_entry` | Retrieve details for a specific time entry |

## Usage Example

```typescript
// List issues in a project
const issues = await client.tools.execute('youtrack_list_issues', {
  project: 'PROJ',
  state: 'Open'
});

// Get a specific issue
const issue = await client.tools.execute('youtrack_get_issue', {
  issue_id: 'PROJ-123'
});

// Create a new issue
const newIssue = await client.tools.execute('youtrack_create_issue', {
  project: 'PROJ',
  summary: 'Implement dark mode',
  description: 'Add a dark mode toggle to the settings page',
  type: 'Feature',
  priority: 'High'
});

// Update an issue
const updatedIssue = await client.tools.execute('youtrack_update_issue', {
  issue_id: 'PROJ-123',
  state: 'Done',
  assignee: 'john.doe'
});

// List Agile boards
const boards = await client.tools.execute('youtrack_list_boards', {
  project: 'PROJ'
});

// List time entries for an issue
const timeEntries = await client.tools.execute('youtrack_list_time_entries', {
  issue_id: 'PROJ-123'
});
```

## Rate Limits

YouTrack API rate limits vary by plan. Enterprise plans typically have higher limits than cloud plans. The tool execution will return appropriate errors if rate limits are exceeded.
