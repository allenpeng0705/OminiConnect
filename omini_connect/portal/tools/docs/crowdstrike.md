# CrowdStrike Tools

Provider: `crowdstrike` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the CrowdStrike Falcon API. They allow AI agents to manage hosts, vulnerabilities, incidents, detections, and threat intelligence. CrowdStrike is a cybersecurity platform providing endpoint protection and threat intelligence.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine authentication
- Token stored in Nango, accessed via `connection_ref`
- Base URL configured per connection (api.crowdstrike.com for production)

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `crowdstrike_list_hosts` | List hosts/devices | GET | /devices/entities/devices/v1 |
| `crowdstrike_get_host` | Get host details | GET | /devices/entities/devices/v1 |
| `crowdstrike_list_vulnerabilities` | List vulnerabilities | GET | /vulnerabilities/entities/vulnerabilities/v2 |
| `crowdstrike_get_vulnerability` | Get vulnerability details | GET | /vulnerabilities/entities/vulnerabilities/v2 |
| `crowdstrike_list_incidents` | List incidents | GET | /incidents/entities/incidents/v1 |
| `crowdstrike_get_incident` | Get incident details | GET | /incidents/entities/incidents/v1 |
| `crowdstrike_list_detection_summaries` | List detection summaries | GET | /detects/entities/summaries/GET/v1 |
| `crowdstrike_get_detection` | Get detection details | GET | /detects/entities/detects/v1 |
| `crowdstrike_list_actors` | List threat actors | GET | /intel/entities/actors/v1 |
| `crowdstrike_list_policies` | List prevention policies | GET | /policy/entities/policies/v1 |

---

## Tool Details

### crowdstrike_list_hosts

**What it does**: Lists all hosts (devices) enrolled in CrowdStrike Falcon.

**When to use**: View all monitored endpoints, check device inventory, find devices by status.

**Arguments**:
- `limit` (optional): Max results (default 50, max 5000)
- `offset` (optional): Pagination offset
- `filter` (optional): FQL filter string

**Example LLM prompt**: "List all hosts in the production environment"

---

### crowdstrike_get_host

**What it does**: Gets detailed host information including security posture and agent version.

**When to use**: Check specific endpoint status, verify agent installation, assess host risk.

**Arguments**:
- `ids` (required): Array of host IDs

**Example LLM prompt**: "Get details for host 12345"

---

### crowdstrike_list_vulnerabilities

**What it does**: Lists vulnerabilities detected across the endpoint fleet.

**When to use**: Prioritize remediation, track vulnerability trends, find exposed systems.

**Arguments**:
- `limit` (optional): Max results (default 50, max 500)
- `filter` (optional): FQL filter string

**Example LLM prompt**: "List critical vulnerabilities discovered this month"

---

### crowdstrike_get_vulnerability

**What it does**: Gets detailed vulnerability information with affected hosts list.

**When to use**: Understand vulnerability scope, prioritize fixes, get remediation guidance.

**Arguments**:
- `ids` (required): Array of vulnerability IDs

**Example LLM prompt**: "Get details for vulnerability CVE-2024-1234"

---

### crowdstrike_list_incidents

**What it does**: Lists security incidents with filtering by status and severity.

**When to use**: Track active threats, review incident response, prioritize security work.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `filter` (optional): FQL filter string

**Example LLM prompt**: "List all high severity incidents from the last week"

---

### crowdstrike_get_incident

**What it does**: Gets detailed incident with timeline, affected hosts, and indicators.

**When to use**: Investigate specific incidents, understand attack scope, gather forensic data.

**Arguments**:
- `ids` (required): Array of incident IDs

**Example LLM prompt**: "Get details for incident INC-7890"

---

### crowdstrike_list_detection_summaries

**What it does**: Lists detection summaries from the Falcon platform.

**When to use**: Review security alerts, track detection trends, find suspicious activity.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `filter` (optional): FQL filter string

**Example LLM prompt**: "List all detections from the last 24 hours"

---

### crowdstrike_get_detection

**What it does**: Gets detailed detection with threat intelligence and MITRE tactics.

**When to use**: Analyze specific detection, understand attacker technique, get remediation steps.

**Arguments**:
- `ids` (required): Array of detection IDs

**Example LLM prompt**: "Get details for detection det-456"

---

### crowdstrike_list_actors

**What it does**: Lists known threat actors with associated indicators and motives.

**When to use**: Research threat actors, understand adversary TTPs, threat hunting.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all threat actors associated with ransomware"

---

### crowdstrike_list_policies

**What it does**: Lists prevention policies for endpoint protection.

**When to use**: Review security policies, check policy assignments, verify protection rules.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `policy_type` (optional): Policy type - prevention or sensor_update

**Example LLM prompt**: "List all prevention policies"

---

## CrowdStrike API Notes

- **FQL Filters**: CrowdStrike uses Falcon Query Language for filtering
- **Client Credentials**: Uses OAuth2 machine-to-machine flow for service accounts
- **Base URL**: Configured per connection - api.crowdstrike.com for production
- **Hosts**: Endpoints monitored by CrowdStrike Falcon agent
- **Vulnerabilities**: Known vulnerabilities detected on endpoints
- **Incidents**: Security incidents combining multiple related detections
- **Detections**: Individual security alerts from behavioral engines
- **Threat Actors**: Named adversary groups with associated TTPs
