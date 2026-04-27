# Veeva Vault Tools

Provider: `veeva-vault` | Engine: `nango` | Auth: Two-Step OAuth via Nango

## Overview

Veeva Vault is a cloud-based content management platform for life sciences. **Requires two-step oauth via nango.**

## Authentication

**Nango Two-Step OAuth**:
- User authenticates via two-step OAuth flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `veeva_list_documents` | List all documents | GET | /documents |
| `veeva_get_document` | Get document details | GET | /documents/{id} |
| `veeva_create_document` | Create a new document | POST | /documents |
| `veeva_update_document` | Update a document | PUT | /documents/{id} |
| `veeva_list_objects` | List all custom objects | GET | /objects |
| `veeva_get_object` | Get object details | GET | /objects/{name}/{id} |
| `veeva_create_object` | Create a new object | POST | /objects/{name} |
| `veeva_list_packets` | List all packets for submission | GET | /packages |
| `veeva_submit_packet` | Submit a packet | POST | /packages/{id}/submit |
| `veeva_get_audit_trail` | Get audit trail for a document | GET | /documents/{id}/audit |

---

## Tool Details

### veeva_list_documents

**What it does**: List all documents

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_get_document

**What it does**: Get document details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_create_document

**What it does**: Create a new document

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_update_document

**What it does**: Update a document

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_list_objects

**What it does**: List all custom objects

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_get_object

**What it does**: Get object details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_create_object

**What it does**: Create a new object

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_list_packets

**What it does**: List all packets for submission

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_submit_packet

**What it does**: Submit a packet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### veeva_get_audit_trail

**What it does**: Get audit trail for a document

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.subdomain}.veevavault.com/api/${connectionConfig.version}`
- Docs: https://nango.dev/docs/integrations/all/veeva-vault
