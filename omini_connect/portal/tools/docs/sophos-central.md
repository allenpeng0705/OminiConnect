# Sophos Central Tools

Provider: `sophos-central` | Engine: `nango` | Auth: OAUTH2_CC

## Overview

These tools wrap the Sophos Central API. They allow AI agents to interact with Sophos Central functionality. **Requires OAUTH2_CC authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine auth
- Nango manages token refresh automatically
- Scopes depend on application permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_alerts` | Get security alerts | GET | /alerts |
| `get_endpoints` | List all endpoints | GET | /endpoints |
| `get_endpoint` | Get endpoint details | GET | /endpoints/{id} |
| `get_tenant_info` | Get tenant information | GET | /tenants |
| `list_users` | List directory users | GET | /directory/users |
| `list_groups` | List directory groups | GET | /directory/groups |
| `get_web_logs` | Get web filtering logs | GET | /weblogs |
| `get_scan_results` | Get scan results | GET | /scans/results |
| `get_firewall_rules` | List firewall rules | GET | /firewalls/rules |
| `get_lockdowns` | List endpoint lockdowns | GET | /lockdowns |

---

## Tool Details

### get_alerts

**What it does**: Get security alerts

**When to use**: Use this tool when you need to get security alerts.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get alerts to..."

---

### get_endpoints

**What it does**: List all endpoints

**When to use**: Use this tool when you need to list all endpoints.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get endpoints to..."

---

### get_endpoint

**What it does**: Get endpoint details

**When to use**: Use this tool when you need to get endpoint details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get endpoint to..."

---

### get_tenant_info

**What it does**: Get tenant information

**When to use**: Use this tool when you need to get tenant information.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get tenant info to..."

---

### list_users

**What it does**: List directory users

**When to use**: Use this tool when you need to list directory users.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list users to..."

---

### list_groups

**What it does**: List directory groups

**When to use**: Use this tool when you need to list directory groups.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list groups to..."

---

### get_web_logs

**What it does**: Get web filtering logs

**When to use**: Use this tool when you need to get web filtering logs.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get web logs to..."

---

### get_scan_results

**What it does**: Get scan results

**When to use**: Use this tool when you need to get scan results.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get scan results to..."

---

### get_firewall_rules

**What it does**: List firewall rules

**When to use**: Use this tool when you need to list firewall rules.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get firewall rules to..."

---

### get_lockdowns

**What it does**: List endpoint lockdowns

**When to use**: Use this tool when you need to list endpoint lockdowns.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get lockdowns to..."

---

## Sophos Central API Notes

- **Auth mode**: OAUTH2_CC
- **Base URL**: {dataRegion}
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
