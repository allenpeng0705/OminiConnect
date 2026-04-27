# Dialpad Sandbox Tools

Provider: `dialpad-sandbox` | Engine: `nango` | Auth: OAuth via Nango (Sandbox)

## Overview

These tools wrap the Dialpad Sandbox API for testing purposes. They allow AI agents to manage users, calls, departments, phone numbers, and voicemails in a sandbox environment.

## Authentication

**Nango OAuth (Sandbox)**:
- User authenticates via Nango Connect with Dialpad Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Uses sandbox environment (sandbox.dialpad.com)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dialpad_sandbox_list_users` | List users | GET | /users |
| `dialpad_sandbox_get_user` | Get user details | GET | /users/{user_id} |
| `dialpad_sandbox_list_calls` | List calls | GET | /calls |
| `dialpad_sandbox_get_call` | Get call details | GET | /calls/{call_id} |
| `dialpad_sandbox_list_departments` | List departments | GET | /departments |
| `dialpad_sandbox_get_department` | Get department details | GET | /departments/{department_id} |
| `dialpad_sandbox_list_numbers` | List phone numbers | GET | /numbers |
| `dialpad_sandbox_get_number` | Get number details | GET | /numbers/{number_id} |
| `dialpad_sandbox_list_voicemails` | List voicemails | GET | /voicemails |
| `dialpad_sandbox_get_voicemail` | Get voicemail details | GET | /voicemails/{voicemail_id} |

---

## Tool Details

### dialpad_sandbox_list_users

**What it does**: Lists all users in the sandbox environment.

**When to use**: Test user listing, develop against mock data.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all sandbox users"

---

### dialpad_sandbox_get_user

**What it does**: Gets detailed user information in sandbox.

**When to use**: Test user retrieval, verify data structure.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-123"

---

### dialpad_sandbox_list_calls

**What it does**: Lists all calls in sandbox.

**When to use**: Test call listing, simulate call flows.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `direction` (optional): Filter by inbound or outbound
- `status` (optional): Filter by started, ended, or missed

**Example LLM prompt**: "List all sandbox calls"

---

### dialpad_sandbox_get_call

**What it does**: Gets detailed call information in sandbox.

**When to use**: Test call retrieval, verify call data.

**Arguments**:
- `call_id` (required): Call ID

**Example LLM prompt**: "Get details for call c-456"

---

### dialpad_sandbox_list_departments

**What it does**: Lists all departments in sandbox.

**When to use**: Test department listing, develop routing logic.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all sandbox departments"

---

### dialpad_sandbox_get_department

**What it does**: Gets detailed department information in sandbox.

**When to use**: Test department retrieval, verify department data.

**Arguments**:
- `department_id` (required): Department ID

**Example LLM prompt**: "Get details for department d-789"

---

### dialpad_sandbox_list_numbers

**What it does**: Lists all phone numbers in sandbox.

**When to use**: Test number listing, verify number data.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all sandbox numbers"

---

### dialpad_sandbox_get_number

**What it does**: Gets detailed phone number information in sandbox.

**When to use**: Test number retrieval, verify number settings.

**Arguments**:
- `number_id` (required): Number ID

**Example LLM prompt**: "Get details for number n-101"

---

### dialpad_sandbox_list_voicemails

**What it does**: Lists all voicemails in sandbox.

**When to use**: Test voicemail listing, simulate voicemail data.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `user_id` (optional): Filter by user ID

**Example LLM prompt**: "List all sandbox voicemails"

---

### dialpad_sandbox_get_voicemail

**What it does**: Gets detailed voicemail information in sandbox.

**When to use**: Test voicemail retrieval, verify voicemail data.

**Arguments**:
- `voicemail_id` (required): Voicemail ID

**Example LLM prompt**: "Get details for voicemail v-202"

---

## Dialpad Sandbox API Notes

- **Sandbox Only**: This environment is for testing and development
- **Same API Structure**: Identical endpoints to production Dialpad API
- **Sandbox URLs**: Uses sandbox.dialpad.com instead of dialpad.com
- **Mock Data**: Contains synthetic or test data
- **Users**: Test user accounts for development
- **Calls**: Simulated call data for testing
- **Departments**: Test department structures
- **Numbers**: Test phone numbers
- **Voicemails**: Simulated voicemail data
