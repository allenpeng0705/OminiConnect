# Ring-central-sandbox Tools

Provider: `ring-central-sandbox` | Engine: `nango` | Auth: OAuth via Nango

## Overview

RingCentral Sandbox is a testing environment for the RingCentral business communications platform. These tools allow AI agents to test messaging, extensions, presence, call logs, and phone number functionality.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with RingCentral Sandbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `messages:read`, `sms:write`, `extensions:read`, `presence:read`, `presence:write`, `call-log:read`, `phone-number:read`

**Important**: This is the sandbox environment. All operations are test operations and do not affect production data.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ring-central-sandbox_list_messages` | List all messages | GET | /restapi/v1.0/account/~/extension/~/message-store |
| `ring-central-sandbox_get_message` | Get message details | GET | /restapi/v1.0/account/~/extension/~/message-store/{messageId} |
| `ring-central-sandbox_send_sms` | Send SMS message | POST | /restapi/v1.0/account/~/extension/~/sms |
| `ring-central-sandbox_list_extensions` | List extensions | GET | /restapi/v1.0/account/~/extension |
| `ring-central-sandbox_get_extension` | Get extension details | GET | /restapi/v1.0/account/~/extension/{extensionId} |
| `ring-central-sandbox_list_presence` | List presence status | GET | /restapi/v1.0/account/~/extension/~/presence |
| `ring-central-sandbox_update_presence` | Update presence | PUT | /restapi/v1.0/account/~/extension/~/presence |
| `ring-central-sandbox_list_call_logs` | List call logs | GET | /restapi/v1.0/account/~/extension/~/call-log |
| `ring-central-sandbox_get_call_log` | Get call log entry | GET | /restapi/v1.0/account/~/extension/~/call-log/{callLogId} |
| `ring-central-sandbox_list_phone_numbers` | List phone numbers | GET | /restapi/v1.0/account/~/phone-number |

---

## Tool Details

### ring-central-sandbox_list_messages

**What it does**: Returns a list of test messages.

**When to use**: Test message retrieval.

**Arguments**:
- `limit` (optional): Number of messages (default 50)
- `type` (optional): Message type (sms, fax, pager)

**Example LLM prompt**: "List all test messages"

---

### ring-central-sandbox_get_message

**What it does**: Gets details of a specific message.

**When to use**: Test message details retrieval.

**Arguments**:
- `messageId` (required): The message ID

**Example LLM prompt**: "Get details for message msg_test123"

---

### ring-central-sandbox_send_sms

**What it does**: Sends an SMS in sandbox.

**When to use**: Test SMS sending.

**Arguments**:
- `to` (required): Array of recipient phone numbers
- `text` (required): Message text
- `from` (optional): Sender phone number

**Example LLM prompt**: "Send test SMS 'Hello' to 555-1234"

---

### ring-central-sandbox_list_extensions

**What it does**: Returns test extensions.

**When to use**: Test extension listing.

**Arguments**:
- `limit` (optional): Number of extensions (default 100)

**Example LLM prompt**: "List all test extensions"

---

### ring-central-sandbox_get_extension

**What it does**: Gets test extension details.

**When to use**: Test extension retrieval.

**Arguments**:
- `extensionId` (required): The extension ID

**Example LLM prompt**: "Get details for extension 101"

---

### ring-central-sandbox_list_presence

**What it does**: Returns test presence status.

**When to use**: Test presence listing.

**Arguments**:
- `extensionId` (optional): Filter by extension

**Example LLM prompt**: "List test presence status"

---

### ring-central-sandbox_update_presence

**What it does**: Updates test presence status.

**When to use**: Test presence updates.

**Arguments**:
- `availability` (optional): Status
- `userStatus` (optional): User status

**Example LLM prompt**: "Set test status to Available"

---

### ring-central-sandbox_list_call_logs

**What it does**: Returns test call logs.

**When to use**: Test call log retrieval.

**Arguments**:
- `limit` (optional): Number of entries (default 50)
- `type` (optional): Call type (all, inbound, outbound)

**Example LLM prompt**: "List test call logs"

---

### ring-central-sandbox_get_call_log

**What it does**: Gets a test call log entry.

**When to use**: Test call log details retrieval.

**Arguments**:
- `callLogId` (required): The call log ID

**Example LLM prompt**: "Get details for test call log cl_test789"

---

### ring-central-sandbox_list_phone_numbers

**What it does**: Returns test phone numbers.

**When to use**: Test phone number listing.

**Arguments**:
- `limit` (optional): Number of phone numbers (default 100)

**Example LLM prompt**: "List test phone numbers"

---

## RingCentral Sandbox Notes

- This is a TEST environment - data does not affect production
- Sandbox extensions and numbers are for testing only
- Use for testing integrations and workflows
- May have limited functionality compared to production
