# Open Hands Tools

Provider: `open-hands` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Open Hands AI agent API. They allow AI agents to manage conversations, tasks, agents, and workflows. **Requires Open Hands API key authentication.**

## Authentication

**API Key**:
- User provides Open Hands API key
- Key passed via `Authorization: Bearer` header
- Base URL: `https://app.all-hands.dev`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `open_hands_list_conversations` | List conversations | GET | /api/conversations |
| `open_hands_get_conversation` | Get conversation details | GET | /api/conversations/{id} |
| `open_hands_send_message` | Send message | POST | /api/conversations/{id}/messages |
| `open_hands_create_task` | Create task | POST | /api/tasks |
| `open_hands_list_tasks` | List tasks | GET | /api/tasks |
| `open_hands_get_task` | Get task details | GET | /api/tasks/{id} |
| `open_hands_update_task` | Update task | PUT | /api/tasks/{id} |
| `open_hands_list_agents` | List agents | GET | /api/agents |
| `open_hands_get_status` | Get system status | GET | /api/status |
| `open_hands_get_workflow` | Get workflow | GET | /api/workflows/{id} |

---

## Tool Details

### open_hands_list_conversations

**What it does**: Lists all conversations in Open Hands.

**When to use**: Browse conversations, find active chats.

**Arguments**:
- `limit` (optional): Number of conversations (default 20)

**Example LLM prompt**: "List all recent conversations"

---

### open_hands_get_conversation

**What it does**: Gets detailed information for a specific conversation.

**When to use**: View conversation history, messages.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Get details for conversation ABC123"

---

### open_hands_send_message

**What it does**: Sends a message in a conversation.

**When to use**: Interact with AI agent, continue conversation.

**Arguments**:
- `id` (required): Conversation ID
- `content` (required): Message content

**Example LLM prompt**: "Send message 'Hello' in conversation ABC123"

---

### open_hands_create_task

**What it does**: Creates a new task in Open Hands.

**When to use**: Create tasks, assign work.

**Arguments**:
- `title` (required): Task title
- `description` (optional): Task description

**Example LLM prompt**: "Create a task called 'Review PR'"

---

### open_hands_list_tasks

**What it does**: Lists all tasks in Open Hands.

**When to use**: Browse tasks, track work.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending tasks"

---

### open_hands_get_task

**What it does**: Gets detailed information for a specific task.

**When to use**: View task details, assignments.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task 123"

---

### open_hands_update_task

**What it does**: Updates an existing task.

**When to use**: Modify task, change status.

**Arguments**:
- `id` (required): Task ID
- `status` (optional): Task status

**Example LLM prompt**: "Update task 123 status to completed"

---

### open_hands_list_agents

**What it does**: Lists all available agents.

**When to use**: Browse agents, find capabilities.

**Arguments**: None

**Example LLM prompt**: "List all available agents"

---

### open_hands_get_status

**What it does**: Gets system status and health.

**When to use**: Check system health, verify connectivity.

**Arguments**: None

**Example LLM prompt**: "Get system status"

---

### open_hands_get_workflow

**What it does**: Gets workflow configuration.

**When to use**: View workflow steps, automation config.

**Arguments**:
- `id` (required): Workflow ID

**Example LLM prompt**: "Get workflow details for workflow 456"

---

## Open Hands Notes

- **Conversations**: Interactive chat sessions with AI agents
- **Tasks**: Work items and assignments
- **Agents**: AI agents with different capabilities
- **Workflows**: Automated multi-step processes
- **API Key format**: 32 character alphanumeric string
