# Dialpad Tools

Provider: `dialpad` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Dialpad API. They allow AI agents to manage users, calls, departments, phone numbers, and voicemails. Dialpad is a cloud-based phone and video conferencing platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Dialpad
- Token stored in Nango, accessed via `connection_ref`
- Supports refresh token flow

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dialpad_list_users` | List users | GET | /users |
| `dialpad_get_user` | Get user details | GET | /users/{user_id} |
| `dialpad_list_calls` | List calls | GET | /calls |
| `dialpad_get_call` | Get call details | GET | /calls/{call_id} |
| `dialpad_list_departments` | List departments | GET | /departments |
| `dialpad_get_department` | Get department details | GET | /departments/{department_id} |
| `dialpad_list_numbers` | List phone numbers | GET | /numbers |
| `dialpad_get_number` | Get number details | GET | /numbers/{number_id} |
| `dialpad_list_voicemails` | List voicemails | GET | /voicemails |
| `dialpad_get_voicemail` | Get voicemail details | GET | /voicemails/{voicemail_id} |

---

## Tool Details

### dialpad_list_users

**What it does**: Lists all users in the Dialpad organization.

**When to use**: View company directory, find users, manage user accounts.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all Dialpad users"

---

### dialpad_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user details, verify user settings, get user contact info.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-123"

---

### dialpad_list_calls

**What it does**: Lists all calls with optional filtering.

**When to use**: View call history, find missed calls, track call metrics.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `direction` (optional): Filter by inbound or outbound
- `status` (optional): Filter by started, ended, or missed

**Example LLM prompt**: "List all missed calls from today"

---

### dialpad_get_call

**What it does**: Gets detailed call information.

**When to use**: Review call details, check call recording, verify call participants.

**Arguments**:
- `call_id` (required): Call ID

**Example LLM prompt**: "Get details for call c-456"

---

### dialpad_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: View department structure, find department contacts, manage routing.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all departments"

---

### dialpad_get_department

**What it does**: Gets detailed department information.

**When to use**: Check department settings, view department members, verify call routing.

**Arguments**:
- `department_id` (required): Department ID

**Example LLM prompt**: "Get details for department d-789"

---

### dialpad_list_numbers

**What it does**: Lists all phone numbers.

**When to use**: View available numbers, find number assignments, manage number routing.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all phone numbers"

---

### dialpad_get_number

**What it does**: Gets detailed phone number information.

**When to use**: Check number assignment, verify number settings, get number owner.

**Arguments**:
- `number_id` (required): Number ID

**Example LLM prompt**: "Get details for number n-101"

---

### dialpad_list_voicemails

**What it does**: Lists all voicemails with optional user filtering.

**When to use**: Check voicemails, find missed messages, track voicemail activity.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `user_id` (optional): Filter by user ID

**Example LLM prompt**: "List all voicemails for user u-123"

---

### dialpad_get_voicemail

**What it does**: Gets detailed voicemail information.

**When to use**: Review voicemail, get voicemail transcript, check voicemail metadata.

**Arguments**:
- `voicemail_id` (required): Voicemail ID

**Example LLM prompt**: "Get details for voicemail v-202"

---

## Dialpad API Notes

- **Users**: Organization members with phone/chat access
- **Calls**: Telephony calls with direction and status
- **Departments**: Call queues and organizational units for call routing
- **Numbers**: Phone numbers assigned to users or departments
- **Voicemails**: Audio messages left by callers
- **OAuth Flow**: Standard OAuth2 with refresh tokens
