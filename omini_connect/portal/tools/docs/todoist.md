# Todoist Tools

Provider: `todoist` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Todoist REST API. They allow AI agents to create, manage, and complete tasks; organize work into projects and sections; and apply labels for categorization. Todoist is a popular task management app that helps teams stay organized and productive.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Todoist
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `task:read`, `task:add`, `task:update`, `task:complete`, `project:read`, `project:add`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `todoist_list_tasks` | List tasks | GET | /rest/v2/tasks |
| `todoist_get_task` | Get task details | GET | /rest/v2/tasks/{task_id} |
| `todoist_create_task` | Create a task | POST | /rest/v2/tasks |
| `todoist_update_task` | Update a task | POST | /rest/v2/tasks/{task_id} |
| `todoist_close_task` | Close/complete a task | POST | /rest/v2/tasks/{task_id}/close |
| `todoist_list_projects` | List projects | GET | /rest/v2/projects |
| `todoist_get_project` | Get project details | GET | /rest/v2/projects/{project_id} |
| `todoist_create_project` | Create a project | POST | /rest/v2/projects |
| `todoist_list_sections` | List sections | GET | /rest/v2/sections |
| `todoist_get_section` | Get section details | GET | /rest/v2/sections/{section_id} |

---

## Tool Details

### todoist_list_tasks

**What it does**: Lists all tasks with optional filters for project, section, assignee, labels, or due date.

**When to use**: Find tasks by project, see tasks due today, filter by assignee or labels.

**Arguments**:
- `project_id` (optional): Filter by project ID
- `section_id` (optional): Filter by section ID
- `assignee` (optional): Filter by assignee ID
- `labels` (optional): Filter by labels (array)
- `due_date` (optional): Filter by due date (YYYY-MM-DD)
- `filter` (optional): Todoist filter string (e.g. 'today | overdue')

**Example LLM prompt**: "Find all tasks due today in the Work project"

---

### todoist_get_task

**What it does**: Gets detailed information about a task including description, due date, priority, and labels.

**When to use**: Read full task details before updating or completing.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Get the details of task 123456789"

---

### todoist_create_task

**What it does**: Creates a new task in a project or inbox.

**When to use**: Log tasks, capture to-dos, add items to project boards.

**Arguments**:
- `content` (required): Task title/content
- `description` (optional): Task description
- `project_id` (optional): Project ID (defaults to inbox)
- `section_id` (optional): Section ID to add task to
- `parent_id` (optional): Parent task ID (for subtasks)
- `order` (optional): Position in project
- `priority` (optional): Priority 1-4 (1=normal, 4=urgent)
- `due_string` (optional): Natural language due date (e.g. 'tomorrow at 3pm')
- `due_date` (optional): Due date (YYYY-MM-DD)
- `due_datetime` (optional): Due datetime (RFC3339)
- `labels` (optional): Array of label names

**Example LLM prompt**: "Create a task to review the Q4 budget due tomorrow with priority 4"

---

### todoist_update_task

**What it does**: Updates a task's content, description, due date, priority, or labels.

**When to use**: Change task details, reschedule, or relabel tasks.

**Arguments**:
- `task_id` (required): Task ID
- `content` (optional): New task title
- `description` (optional): New description
- `priority` (optional): Priority 1-4 (1=normal, 4=urgent)
- `due_string` (optional): Natural language due date
- `due_date` (optional): Due date (YYYY-MM-DD)
- `due_datetime` (optional): Due datetime (RFC3339)
- `labels` (optional): Array of label names

**Example LLM prompt**: "Update task 123456789 to be due next Friday and set priority to high"

---

### todoist_close_task

**What it does**: Closes (completes) a task.

**When to use**: Mark items as done, clear completed tasks from lists.

**Arguments**:
- `task_id` (required): Task ID

**Example LLM prompt**: "Complete the task to buy office supplies"

---

### todoist_list_projects

**What it does**: Lists all projects the user has access to.

**When to use**: See available projects, choose where to create tasks.

**Arguments**: None

**Example LLM prompt**: "List all my Todoist projects"

---

### todoist_get_project

**What it does**: Gets detailed information about a project.

**When to use**: Understand project details before creating tasks.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project 123456789"

---

### todoist_create_project

**What it does**: Creates a new project.

**When to use**: Set up new project categories, organize related tasks.

**Arguments**:
- `name` (required): Project name
- `parent_id` (optional): Parent project ID for subprojects
- `color` (optional): Project color (e.g. 'blue', 'red')
- `is_favorite` (optional): Mark as favorite

**Example LLM prompt**: "Create a new project called Marketing Campaigns with blue color"

---

### todoist_list_sections

**What it does**: Lists all sections in a project.

**When to use**: See sections within a project for organizing tasks.

**Arguments**:
- `project_id` (optional): Filter by project ID

**Example LLM prompt**: "List all sections in the Work project"

---

### todoist_get_section

**What it does**: Gets detailed information about a section.

**When to use**: Understand section details before adding tasks.

**Arguments**:
- `section_id` (required): Section ID

**Example LLM prompt**: "Get details for section 987654321"

---

## Todoist API Notes

- **Due dates**: Can be natural language like "tomorrow", "next Monday" or RFC3339 format
- **Priority levels**: 1 = No priority, 2 = Low, 3 = Medium, 4 = Urgent
- **Projects vs Sections**: Projects contain sections which contain tasks
- **Inbox**: Default project for tasks without a project assignment
- **Labels**: Global across all projects, not tied to specific projects
- **Filters**: Todoist has a powerful filter syntax like "today | overdue & @work"
