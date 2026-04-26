# Phabricator Tools

Provider: `phabricator` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Phabricator Conduit API. They allow AI agents to manage projects, tasks (Maniphest), repositories (Diffusion), and users. Phabricator is an open-source software development platform by Meta that provides code review, repository hosting, and project management.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Phabricator
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `project`, `maniphest`, `diffusion`, `user`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `phabricator_list_projects` | List projects | POST | /project.query |
| `phabricator_get_project` | Get project details | POST | /project.query |
| `phabricator_create_project` | Create a project | POST | /project.create |
| `phabricator_list_tasks` | List tasks | POST | /maniphest.query |
| `phabricator_get_task` | Get task details | POST | /maniphest.query |
| `phabricator_create_task` | Create a task | POST | /maniphest.create |
| `phabricator_list_repositories` | List repositories | POST | /diffusion.repository.search |
| `phabricator_get_repository` | Get repository details | POST | /diffusion.repository.search |
| `phabricator_list_users` | List users | POST | /user.search |
| `phabricator_get_user` | Get user details | POST | /user.search |

---

## Tool Details

### phabricator_list_projects

**What it does**: Lists projects in your Phabricator instance. Projects are containers for tasks, repositories, and team members to organize work.

**When to use**: Find projects, see available workspaces, browse project directory, understand team structure.

**Arguments**:
- `query` (optional): Search projects by name
- `status` (optional): Filter by status (`active`, `archived`) - default `active`
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all active projects in Phabricator to find the Platform team project"

---

### phabricator_get_project

**What it does**: Gets detailed information about a specific project including its name, description, members, and associated repositories and tasks.

**When to use**: Understand project structure, check project status, find related tasks and repositories.

**Arguments**:
- `project_ids` (required): Project ID(s) (array)

**Example LLM prompt**: "Get details for project PHID-PROJ-123 to see its members and recent tasks"

---

### phabricator_create_project

**What it does**: Creates a new project in Phabricator. Projects organize work, track progress, and group team members together.

**When to use**: Initialize a new team workspace, create a project for a new initiative, set up tracking for a new product.

**Arguments**:
- `name` (required): Project name
- `description` (optional): Project description
- `icon` (optional): Project icon (default `project`)
- `members` (optional): Array of user PHIDs to add as members

**Example LLM prompt**: "Create a new project called 'Mobile App v2' with a smartphone icon and add alice and bob as members"

---

### phabricator_list_tasks

**What it does**: Lists tasks (Maniphest tickets) in your Phabricator instance. Tasks track work items, bugs, and features with status and priority.

**When to use**: Find work items, track tasks, see assigned work, find tasks by project or assignee.

**Arguments**:
- `query` (optional): Search tasks by title
- `project_ids` (optional): Filter by project PHIDs
- `status` (optional): Filter by status (`open`, `resolved`, `closed`)
- `priority` (optional): Filter by priority PHIDs (array)
- `assignees` (optional): Filter by assignee PHIDs (array)
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all open high-priority tasks assigned to me in the Mobile App project"

---

### phabricator_get_task

**What it does**: Gets detailed information about a specific task including title, description, status, priority, assignees, and comments.

**When to use**: Review task details, understand requirements, check task history, see subtasks and relationships.

**Arguments**:
- `task_ids` (required): Task ID(s) (array, format like `T123`)

**Example LLM prompt**: "Get details for task T456 to see the full description and current assignees"

---

### phabricator_create_task

**What it does**: Creates a new task (Maniphest ticket) in Phabricator. Tasks represent work items that can be assigned, prioritized, and tracked.

**When to use**: Log bugs, create work items, capture tasks from conversations, file feature requests.

**Arguments**:
- `title` (required): Task title
- `description` (optional): Task description/notes
- `priority` (optional): Priority (`unbreakable`, `needs Triage`, `high`, `normal`, `low`, `wishlist`) - default `normal`
- `project_ids` (optional): Project PHIDs to assign the task to
- `assignee` (optional): User PHID to assign the task to

**Example LLM prompt**: "Create a task with title 'Fix login validation bug' in the Mobile App project with high priority"

---

### phabricator_list_repositories

**What it does**: Lists repositories (Diffusion) in your Phabricator instance. Repositories host version control for projects.

**When to use**: Find repositories, browse available repos, check repository list for a project.

**Arguments**:
- `query` (optional): Search repositories by name
- `status` (optional): Filter by status (`active`, `inactive`)
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all active repositories to find the main backend repository"

---

### phabricator_get_repository

**What it does**: Gets detailed information about a specific repository including name, remote URL, tracking configuration, and recent commits.

**When to use**: Check repository configuration, find repository details, understand repository setup and branches.

**Arguments**:
- `repository_phids` (required): Repository PHID(s) (array)

**Example LLM prompt**: "Get details for repository phid-repo-123 to see the remote URL and default branch"

---

### phabricator_list_users

**What it does**: Lists users in your Phabricator instance. Users are members who can be assigned tasks, own repositories, and comment on work.

**When to use**: Find team members, get user information, see available users for assignment, check user status.

**Arguments**:
- `query` (optional): Search users by name or username
- `status` (optional): Filter by status (`active`, `disabled`)
- `limit` (optional): Max results (default 20, max 100)
- `offset` (optional): Offset for pagination

**Example LLM prompt**: "List all active users to find the username for alice@example.com"

---

### phabricator_get_user

**What it does**: Gets detailed information about a specific user including their username, real name, avatar, roles, and current activities.

**When to use**: Check user profile, find user roles, see what projects a user works on, verify user exists.

**Arguments**:
- `user_phids` (required): User PHID(s) (array)

**Example LLM prompt**: "Get details for user PHID-USER-789 to see their username and current assignments"

---

## Phabricator API Notes

- **Task IDs**: Format is `T` followed by numbers (e.g., `T123`)
- **PHIDs**: Phabricator uses PHIDs (Phabricator IDs) as stable object identifiers across all object types
- **Status values**: Tasks can be `open`, `resolved`, `closed`
- **Priority levels**: `unbreakable` (urgent), `needs Triage`, `high`, `normal`, `low`, `wishlist` (lowest)
- **Conduit API**: All API calls use POST to `/api/*` endpoints with JSON request bodies
- **Rate Limits**: Phabricator API has rate limits; use batching when possible
- **Diffusion**: Phabricator's repository hosting component is called Diffusion and uses Git and Mercurial
- **Maniphest**: Phabricator's task management system for tracking work items
- **Projects**: Can contain multiple repositories, tasks, and members