# Canvas LMS Tools

Provider: `canvas-lms` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Canvas LMS API. Canvas is a learning management system (LMS) used by educational institutions for managing courses, assignments, and student progress. **Requires Canvas OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Canvas
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Configured per Canvas instance (`${connectionConfig.hostname}`)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `canvas_list_courses` | List courses | GET | /api/v1/courses |
| `canvas_get_course` | Get course details | GET | /api/v1/courses/{id} |
| `canvas_list_users` | List users | GET | /api/v1/accounts/{accountId}/users |
| `canvas_get_user` | Get user details | GET | /api/v1/users/{id} |
| `canvas_list_assignments` | List assignments | GET | /api/v1/courses/{courseId}/assignments |
| `canvas_get_assignment` | Get assignment details | GET | /api/v1/courses/{courseId}/assignments/{id} |
| `canvas_list_submissions` | List submissions | GET | /api/v1/courses/{courseId}/assignments/{assignmentId}/submissions |
| `canvas_create_assignment` | Create an assignment | POST | /api/v1/courses/{courseId}/assignments |
| `canvas_list_modules` | List modules | GET | /api/v1/courses/{courseId}/modules |
| `canvas_list_announcements` | List announcements | GET | /api/v1/courses/{courseId}/announcements |

---

## Tool Details

### canvas_list_courses

**What it does**: Lists all courses the user is enrolled in.

**When to use**: Find courses, browse available classes.

**Arguments**:
- `enrollment_type` (optional): Filter by enrollment type
- `state` (optional): Course state filter
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all my Canvas courses"

---

### canvas_get_course

**What it does**: Gets details of a specific course.

**When to use**: View course syllabus, students, or settings.

**Arguments**:
- `id` (required): Course ID

**Example LLM prompt**: "Get course 123 details"

---

### canvas_list_users

**What it does**: Lists all users in an account or course.

**When to use**: Find students, view class rosters.

**Arguments**:
- `accountId` (required): Account ID
- `search_term` (optional): Search by name or email
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List users in account 1"

---

### canvas_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile, enrollments, grades.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 456 details"

---

### canvas_list_assignments

**What it does**: Lists all assignments in a course.

**When to use**: View homework, find due dates.

**Arguments**:
- `courseId` (required): Course ID
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List assignments for course 123"

---

### canvas_get_assignment

**What it does**: Gets details of a specific assignment.

**When to use**: View assignment description, points, due date.

**Arguments**:
- `courseId` (required): Course ID
- `id` (required): Assignment ID

**Example LLM prompt**: "Get assignment 789 in course 123"

---

### canvas_list_submissions

**What it does**: Lists all submissions for an assignment.

**When to use**: View student work, check turn-in status.

**Arguments**:
- `courseId` (required): Course ID
- `assignmentId` (required): Assignment ID

**Example LLM prompt**: "List submissions for assignment 789"

---

### canvas_create_assignment

**What it does**: Creates a new assignment in a course.

**When to use**: Post new homework or exams.

**Arguments**:
- `courseId` (required): Course ID
- `name` (required): Assignment title
- `description` (optional): Assignment description
- `due_at` (optional): Due date (ISO 8601)
- `points_possible` (optional): Maximum points

**Example LLM prompt**: "Create an assignment called 'Week 5 Quiz' in course 123"

---

### canvas_list_modules

**What it does**: Lists all modules in a course.

**When to use**: View course structure, find learning materials.

**Arguments**:
- `courseId` (required): Course ID

**Example LLM prompt**: "List modules for course 123"

---

### canvas_list_announcements

**What it does**: Lists all announcements in a course.

**When to use**: View course updates, professor announcements.

**Arguments**:
- `courseId` (required): Course ID

**Example LLM prompt**: "List announcements for course 123"

---

## Canvas LMS Notes

- **Hostname**: Each Canvas instance has its own hostname (e.g., canvas.instructure.com)
- **Courses**: Classes containing modules, assignments, and students
- **Assignments**: Homework or exams with due dates and points
- **Submissions**: Student work submitted for an assignment
- **Modules**: Course content organized into sections
