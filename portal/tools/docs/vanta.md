# Vanta Tools

Provider: `vanta` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

Vanta is a compliance automation platform for SOC 2, ISO 27001, and other frameworks. **Requires oauth2 client credentials via nango.**

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for app-level access
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `vanta_list_compliance_frameworks` | List all compliance frameworks | GET | /v1/compliance-frameworks |
| `vanta_get_framework` | Get framework details | GET | /v1/compliance-frameworks/{id} |
| `vanta_list_controls` | List all controls | GET | /v1/controls |
| `vanta_get_control` | Get control details | GET | /v1/controls/{id} |
| `vanta_list_users` | List all users | GET | /v1/users |
| `vanta_invite_user` | Invite a new user | POST | /v1/users/invite |
| `vanta_list_vendors` | List all vendors | GET | /v1/vendors |
| `vanta_add_vendor` | Add a new vendor | POST | /v1/vendors |
| `vanta_list_evidence` | List all evidence items | GET | /v1/evidence |
| `vanta_upload_evidence` | Upload evidence | POST | /v1/evidence/upload |

---

## Tool Details

### vanta_list_compliance_frameworks

**What it does**: List all compliance frameworks

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_get_framework

**What it does**: Get framework details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_list_controls

**What it does**: List all controls

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_get_control

**What it does**: Get control details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_list_users

**What it does**: List all users

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_invite_user

**What it does**: Invite a new user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_list_vendors

**What it does**: List all vendors

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_add_vendor

**What it does**: Add a new vendor

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_list_evidence

**What it does**: List all evidence items

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vanta_upload_evidence

**What it does**: Upload evidence

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.vanta.com`
- Docs: https://nango.dev/docs/integrations/all/vanta
