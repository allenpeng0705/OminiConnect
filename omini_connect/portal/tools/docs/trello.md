# Trello Tools

Provider: `trello` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Trello REST API. They allow AI agents to manage boards, lists, and cards; track project progress; and collaborate with teammates. Trello is a popular kanban-style project management tool.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Trello
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `trello_list_boards` | List all boards for the user | GET | /1/members/me/boards |
| `trello_get_board` | Get board details | GET | /1/boards/{boardId} |
| `trello_create_board` | Create a new board | POST | /1/boards |
| `trello_list_lists` | List lists on a board | GET | /1/boards/{boardId}/lists |
| `trello_get_list` | Get list details | GET | /1/lists/{listId} |
| `trello_create_list` | Create a new list | POST | /1/lists |
| `trello_list_cards` | List cards in a list or board | GET | /1/boards/{boardId}/cards |
| `trello_get_card` | Get card details | GET | /1/cards/{cardId} |
| `trello_create_card` | Create a new card | POST | /1/cards |
| `trello_add_member_to_card` | Add a member to a card | POST | /1/cards/{cardId}/members |

---

## Tool Details

### trello_list_boards

**What it does**: Lists all Trello boards the authenticated user is a member of.

**When to use**: Find available boards to work with, check board names and IDs.

**Arguments**:
- `filter` (optional): Filter boards (`open`, `closed`, `pinned`, `unpinned`). Default: `open`

**Example LLM prompt**: "List all my Trello boards"

---

### trello_get_board

**What it does**: Gets detailed information about a specific board including name, description, and visibility settings.

**When to use**: Get board context before listing lists or cards.

**Arguments**:
- `boardId` (required): Board ID
- `lists` (optional): Include lists (`open`, `closed`, `all`). Default: `open`
- `members` (optional): Include members (`none`, `normal`, `all`). Default: `none`

**Example LLM prompt**: "Get details for board abc123"

---

### trello_create_board

**What it does**: Creates a new Trello board with specified name, description, and visibility settings.

**When to use**: Create new project boards, set up team workspaces.

**Arguments**:
- `name` (required): Board name
- `desc` (optional): Board description
- `idOrganization` (optional): Organization ID to create board in
- `visibility` (optional): Visibility (`private`, `team`, `public`). Default: `private`
- `defaultLists` (optional): Add default lists (To Do, Doing, Done). Default: `true`

**Example LLM prompt**: "Create a new board called 'Sprint Planning' with default lists"

---

### trello_list_lists

**What it does**: Lists all lists (columns) on a board, such as "To Do", "In Progress", "Done".

**When to use**: Find available lists to add cards to or see workflow stages.

**Arguments**:
- `boardId` (required): Board ID
- `filter` (optional): Filter lists (`open`, `closed`, `all`). Default: `open`

**Example LLM prompt**: "List all lists on board abc123"

---

### trello_get_list

**What it does**: Gets details about a specific list including name, position, and card count.

**When to use**: Get list context or check if a list has cards.

**Arguments**:
- `listId` (required): List ID
- `cards` (optional): Include cards (`open`, `closed`, `all`). Default: `none`
- `card_fields` (optional): Card fields to include. Default: `name`

**Example LLM prompt**: "Get the details for list xyz456"

---

### trello_create_list

**What it does**: Creates a new list on a Trello board.

**When to use**: Add new workflow stages or columns to a board.

**Arguments**:
- `name` (required): List name
- `idBoard` (required): Board ID to add list to
- `pos` (optional): Position (`top`, `bottom`, or a number). Default: `bottom`

**Example LLM prompt**: "Create a new list called 'Review' on board abc123"

---

### trello_list_cards

**What it does**: Lists cards in a list or all cards on a board.

**When to use**: See all tasks in a column or entire board.

**Arguments**:
- `boardId` (optional): Board ID (use `listId` instead to filter by list)
- `listId` (optional): List ID to filter cards
- `filter` (optional): Filter cards (`open`, `closed`, `all`). Default: `open`

**Example LLM prompt**: "List all cards in the In Progress list"

---

### trello_get_card

**What it does**: Gets detailed information about a card including description, due date, labels, members, and activity.

**When to use**: Read full card details before updating or commenting.

**Arguments**:
- `cardId` (required): Card ID
- `fields` (optional): Fields to include. Default: `all`

**Example LLM prompt**: "Get card def789 details"

---

### trello_create_card

**What it does**: Creates a new card on a Trello list with specified details.

**When to use**: Add new tasks, capture work items, create follow-up actions.

**Arguments**:
- `name` (required): Card name/title
- `idList` (required): List ID to add card to
- `desc` (optional): Card description
- `due` (optional): Due date (ISO 8601 format)
- `labels` (optional): Comma-separated label names or IDs
- `idMembers` (optional): Comma-separated member IDs to assign

**Example LLM prompt**: "Create a card named 'Review PR' in the To Do list with due date tomorrow"

---

### trello_add_member_to_card

**What it does**: Adds a member to a Trello card to assign them.

**When to use**: Assign work to team members, delegate tasks.

**Arguments**:
- `cardId` (required): Card ID
- `value` (required): Member ID to add

**Example LLM prompt**: "Add member john@example.com to card def789"

---

## Trello API Notes

- **Board IDs**: Use the long ID format (24 characters) for board operations
- **List IDs**: Can be found by listing board lists or from board URL
- **Card IDs**: Also use 24-character ID format
- **Power-Ups**: Some Power-Ups add custom fields that may not be visible via API
- **Checklists**: Trello supports checklists within cards but this tool set doesn't include them yet
- **Attachments**: Cards support file attachments but this tool set doesn't include them yet
