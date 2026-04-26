# Drata Tools

Provider: `drata` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Drata API. They allow AI agents to manage devices, users, evidence, controls, and policies. Drata is a security and compliance automation platform.

## Authentication

**Nango API_KEY**:
- User provides their Drata API key via Nango
- Token stored in Nango, accessed via `connection_ref`
- Passed in headers as Bearer token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `drata_list_devices` | List devices | GET | /v1/devices |
| `drata_get_device` | Get device details | GET | /v1/devices/{device_id} |
| `drata_list_users` | List users | GET | /v1/users |
| `drata_get_user` | Get user details | GET | /v1/users/{user_id} |
| `drata_list_evidence` | List evidence | GET | /v1/evidence |
| `drata_get_evidence` | Get evidence details | GET | /v1/evidence/{evidence_id} |
| `drata_list_controls` | List controls | GET | /v1/controls |
| `drata_get_control` | Get control details | GET | /v1/controls/{control_id} |
| `drata_list_policies` | List policies | GET | /v1/policies |
| `drata_get_policy` | Get policy details | GET | /v1/policies/{policy_id} |

---

## Tool Details

### drata_list_devices

**What it does**: Lists all devices with optional status filtering.

**When to use**: View device inventory, check device compliance status, track monitored endpoints.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `status` (optional): Filter by healthy, needs_attention, or critical

**Example LLM prompt**: "List all devices with critical status"

---

### drata_get_device

**What it does**: Gets detailed device information.

**When to use**: Check device details, verify compliance status, review device configuration.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get details for device d-123"

---

### drata_list_users

**What it does**: Lists all users in the Drata workspace.

**When to use**: View user list, manage user access, track user compliance.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all Drata users"

---

### drata_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user details, verify user compliance status, review user permissions.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-456"

---

### drata_list_evidence

**What it does**: Lists all evidence items with optional control filtering.

**When to use**: Browse evidence, find evidence for controls, track compliance evidence.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `control_id` (optional): Filter by control ID

**Example LLM prompt**: "List all evidence for control c-789"

---

### drata_get_evidence

**What it does**: Gets detailed evidence information.

**When to use**: Review evidence details, check evidence status, verify evidence authenticity.

**Arguments**:
- `evidence_id` (required): Evidence ID

**Example LLM prompt**: "Get details for evidence e-101"

---

### drata_list_controls

**What it does**: Lists all controls with optional status filtering.

**When to use**: View compliance controls, track control status, find failing controls.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `status` (optional): Filter by passing, failing, or needs_attention

**Example LLM prompt**: "List all failing controls"

---

### drata_get_control

**What it does**: Gets detailed control information.

**When to use**: Review control details, check control requirements, understand control evidence.

**Arguments**:
- `control_id` (required): Control ID

**Example LLM prompt**: "Get details for control c-202"

---

### drata_list_policies

**What it does**: Lists all policies in the workspace.

**When to use**: Browse policies, find security policies, review policy documents.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all Drata policies"

---

### drata_get_policy

**What it does**: Gets detailed policy information.

**When to use**: Review policy details, check policy requirements, understand policy scope.

**Arguments**:
- `policy_id` (required): Policy ID

**Example LLM prompt**: "Get details for policy pol-303"

---

## Drata API Notes

- **Compliance Automation**: Platform for SOC2, ISO27001, and other compliance frameworks
- **Devices**: Endpoints monitored for compliance status
- **Users**: Team members tracked for compliance training and access
- **Evidence**: Documentation and data supporting compliance controls
- **Controls**: Security and compliance requirements being monitored
- **Policies**: Security and compliance policies and procedures
- **API Key**: Passed in headers as `Authorization: Bearer ${apiKey}`
