# Lessonly Tools

Provider: `lessonly` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Lessonly API. They allow AI agents to manage users, lessons, assignments, and learning paths. **Requires Lessonly subdomain and API key.**

## Authentication

**Basic Auth via Nango**:
- User provides subdomain and API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.lessonly.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lessonly_list_users` | List users | GET | /api/users |
| `lessonly_get_user` | Get user details | GET | /api/users/{user_id} |
| `lessonly_list_lessons` | List lessons | GET | /api/lessons |
| `lessonly_get_lesson` | Get lesson details | GET | /api/lessons/{lesson_id} |
| `lessonly_list_assignments` | List assignments | GET | /api/assignments |
| `lessonly_get_assignment` | Get assignment details | GET | /api/assignments/{assignment_id} |
| `lessonly_list_paths` | List learning paths | GET | /api/paths |
| `lessonly_get_path` | Get learning path details | GET | /api/paths/{path_id} |
| `lessonly_list_groups` | List groups | GET | /api/groups |
| `lessonly_get_group` | Get group details | GET | /api/groups/{group_id} |

---

## Tool Details

### lessonly_list_users

**What it does**: Lists all users in Lessonly.

**When to use**: Find users, view team members.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all users in Lessonly"

---

### lessonly_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### lessonly_list_lessons

**What it does**: Lists all lessons.

**When to use**: View training content.

**Arguments**: None

**Example LLM prompt**: "List all lessons in Lessonly"

---

### lessonly_get_lesson

**What it does**: Gets details for a specific lesson.

**When to use**: Get lesson information.

**Arguments**:
- `lesson_id` (required): Lesson ID

**Example LLM prompt**: "Get details for lesson 67890"

---

### lessonly_list_assignments

**What it does**: Lists all assignments.

**When to use**: View training assignments.

**Arguments**:
- `user_id` (optional): Filter by user ID

**Example LLM prompt**: "List all assignments in Lessonly"

---

### lessonly_get_assignment

**What it does**: Gets details for a specific assignment.

**When to use**: Get assignment information.

**Arguments**:
- `assignment_id` (required): Assignment ID

**Example LLM prompt**: "Get details for assignment a1"

---

### lessonly_list_paths

**What it does**: Lists all learning paths.

**When to use**: View training programs.

**Arguments**: None

**Example LLM prompt**: "List all learning paths in Lessonly"

---

### lessonly_get_path

**What it does**: Gets details for a specific learning path.

**When to use**: Get learning path information.

**Arguments**:
- `path_id` (required): Path ID

**Example LLM prompt**: "Get details for path p1"

---

### lessonly_list_groups

**What it does**: Lists all groups.

**When to use**: View groups, organize users.

**Arguments**: None

**Example LLM prompt**: "List all groups in Lessonly"

---

### lessonly_get_group

**What it does**: Gets details for a specific group.

**When to use**: Get group information.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group g1"

---

## Lessonly API Notes

- **Training Platform**: Learning and development platform
- **Users**: Employees and learners
- **Lessons**: Training content and modules
- **Assignments**: Training assignments to users
- **Learning Paths**: Curated sequences of lessons
