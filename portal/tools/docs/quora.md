# Quora Tools

Provider: `quora` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Quora API. They allow AI agents to browse topics, ask and answer questions, search content, and manage user information. Quora is a platform for knowledge sharing with topics spanning technology, business, science, and more.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Quora
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `quora_list_topics` | List topics | GET | /topics |
| `quora_get_topic` | Get topic details | GET | /topics/{topic_id} |
| `quora_list_questions` | List questions | GET | /questions |
| `quora_get_question` | Get question details | GET | /questions/{question_id} |
| `quora_create_question` | Create a new question | POST | /questions |
| `quora_list_answers` | List answers for a question | GET | /questions/{question_id}/answers |
| `quora_get_answer` | Get answer details | GET | /answers/{answer_id} |
| `quora_create_answer` | Create an answer | POST | /questions/{question_id}/answers |
| `quora_get_user_info` | Get user profile | GET | /users/{user_id} |
| `quora_search_questions` | Search questions | GET | /search/questions |

---

## Tool Details

### quora_list_topics

**What it does**: Retrieves a list of topics on Quora.

**When to use**: Find relevant topics for asking questions or researching a subject.

**Arguments**:
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "Find topics related to machine learning"

---

### quora_get_topic

**What it does**: Gets details about a specific topic by its ID or slug.

**When to use**: Understand a topic's scope before asking questions or following it.

**Arguments**:
- `topic_id` (required): The topic ID or slug

**Example LLM prompt**: "Get details for the artificial-intelligence topic"

---

### quora_list_questions

**What it does**: Retrieves a list of questions, optionally filtered by topic.

**When to use**: Browse questions on a topic, find unanswered questions, or research common concerns.

**Arguments**:
- `topic_id` (optional): Filter by topic ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List questions about python data science"

---

### quora_get_question

**What it does**: Gets details about a specific question by its ID.

**When to use**: Read full question details and view existing answers before contributing.

**Arguments**:
- `question_id` (required): The question ID

**Example LLM prompt**: "Get details for question abc123"

---

### quora_create_question

**What it does**: Creates a new question on Quora.

**When to use**: Ask questions to the Quora community for expert answers, research, or opinions.

**Arguments**:
- `text` (required): The question text
- `topic_ids` (optional): Topic IDs to attach to the question

**Example LLM prompt**: "Ask a question: 'What are the best practices for training neural networks?'"

---

### quora_list_answers

**What it does**: Retrieves answers for a specific question.

**When to use**: Read existing answers, compare perspectives, or find the best answer.

**Arguments**:
- `question_id` (required): The question ID
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "List all answers to question abc123"

---

### quora_get_answer

**What it does**: Gets details about a specific answer by its ID.

**When to use**: Read a specific answer in full detail.

**Arguments**:
- `answer_id` (required): The answer ID

**Example LLM prompt**: "Get the full content of answer xyz789"

---

### quora_create_answer

**What it does**: Creates an answer to a specific question.

**When to use**: Share knowledge, provide explanations, or help others with their questions.

**Arguments**:
- `question_id` (required): The question ID to answer
- `text` (required): The answer text

**Example LLM prompt**: "Answer question abc123 with a detailed explanation of how gradient descent works"

---

### quora_get_user_info

**What it does**: Gets profile information for a specific user.

**When to use**: Research an answer author's background or find experts to follow.

**Arguments**:
- `user_id` (required): The user ID

**Example LLM prompt**: "Get the profile of user xyz456"

---

### quora_search_questions

**What it does**: Searches for questions matching a query.

**When to use**: Find questions on a specific topic or search for existing answers.

**Arguments**:
- `q` (required): Search query string
- `page` (optional): Page number for pagination
- `limit` (optional): Number of results per page (default 20)

**Example LLM prompt**: "Search for questions about machine learning"

---

## Quora API Notes

- **Content Quality**: Quora values detailed, well-researched answers
- **Topics**: Questions should be tagged with relevant topics for visibility
- **Credentials**: Users can add credentials to establish expertise in specific topics
- **Search**: Use search_questions to find existing questions before creating new ones
