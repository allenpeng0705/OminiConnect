# Trello More Tools - Extended Trello Functionality

Provider: `trello_more` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Trello REST API with extended functionality. They allow AI agents to manage boards, lists, cards, and actions. This is an extended tool set beyond the base Trello integration.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Trello
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `trello_more_list_boards` | List all boards | GET | /members/me/boards |
| `trello_more_get_board` | Get board details | GET | /boards/{board_id} |
| `trello_more_create_board` | Create a new board | POST | /boards |
| `trello_more_list_lists` | List lists on a board | GET | /boards/{board_id}/lists |
| `trello_more_get_list` | Get list details | GET | /lists/{list_id} |
| `trello_more_create_list` | Create a new list | POST | /boards/{board_id}/lists |
| `trello_more_list_cards` | List cards on a board | GET | /boards/{board_id}/cards |
| `trello_more_get_card` | Get card details | GET | /cards/{card_id} |
| `trello_more_create_card` | Create a new card | POST | /cards |
| `trello_more_get_board_actions` | Get board actions | GET | /boards/{board_id}/actions |

---

## Tool Details

### trello_more_list_boards

**What it does**: Retrieves a list of all boards accessible to the authenticated user.

**When to use**: Find boards to work with, navigate workspace structure.

**Arguments**: None

**Example LLM prompt**: "List all my Trello boards"

---

### trello_more_get_board

**What it does**: Gets details for a specific board by ID.

**When to use**: View board settings, members, and configuration.

**Arguments**:
- `board_id` (required): The unique identifier of the board

**Example LLM prompt**: "Get details for board abc123"

---

### trello_more_create_board

**What it does**: Creates a new board with the given name and settings.

**When to use**: Set up new project boards.

**Arguments**:
- `name` (required): Name of the board
- `description` (optional): Optional description for the board

**Example LLM prompt**: "Create a board called 'Q2 Marketing Campaign'"

---

### trello_more_list_lists

**What it does**: Retrieves all lists on a specific board.

**When to use**: See what lists exist in a board, typically for workflow stages.

**Arguments**:
- `board_id` (required): The board ID to list lists from

**Example LLM prompt**: "List all lists on board abc123"

---

### trello_more_get_list

**What it does**: Gets details for a specific list by ID.

**When to use**: View list configuration and card count.

**Arguments**:
- `list_id` (required): The unique identifier of the list

**Example LLM prompt**: "Get details for list def456"

---

### trello_more_create_list

**What it does**: Creates a new list on a board.

**When to use**: Add new workflow stages or categories.

**Arguments**:
- `board_id` (required): The board ID to create the list on
- `name` (required): Name of the list

**Example LLM prompt**: "Create a list called 'In Review' on board abc123"

---

### trello_more_list_cards

**What it does**: Retrieves all cards on a board or list.

**When to use**: See all cards for sprint planning or backlog review.

**Arguments**:
- `board_id` (required): The board ID to list cards from

**Example LLM prompt**: "List all cards on board abc123"

---

### trello_more_get_card

**What it does**: Gets details for a specific card by ID.

**When to use**: View full card details including description, comments, and attachments.

**Arguments**:
- `card_id` (required): The unique identifier of the card

**Example LLM prompt**: "Get details for card ghi789"

---

### trello_more_create_card

**What it does**: Creates a new card in a list.

**When to use**: Add new tasks or items to a list.

**Arguments**:
- `name` (required): Name of the card
- `id_list` (required): The list ID to add the card to
- `desc` (optional): Optional description for the card

**Example LLM prompt**: "Create a card called 'Write blog post' in list def456"

---

### trello_more_get_board_actions

**What it does**: Retrieves recent actions performed on a board.

**When to use**: Review activity history, audit changes.

**Arguments**:
- `board_id` (required): The board ID to get actions from
- `filter` (optional): Optional filter for action types

**Example LLM prompt**: "Get recent actions for board abc123"

---

## Trello API Notes

- **Boards**: Top-level containers for projects, contain lists and cards
- **Lists**: Columns within a board representing workflow stages
- **Cards**: Individual items or tasks within lists
- **Actions**: Activity history tracking all changes to boards
- **Board ID format**: 24-character alphanumeric strings
