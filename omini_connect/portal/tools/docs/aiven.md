# Aiven Tools

Provider: `aiven` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Aiven API. They allow AI agents to manage projects, services (databases, Kafka, Redis, Elasticsearch), Kafka topics, access control lists, and connection pools. Aiven is a managed cloud data platform offering fully managed open-source databases and messaging systems.

## Authentication

**Nango API_KEY**:
- User authenticates via Nango Connect with Aiven
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `api_key`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `aiven_list_projects` | List Aiven projects | GET | /v1/projects |
| `aiven_get_project` | Get project details | GET | /v1/projects/{projectName} |
| `aiven_list_services` | List services in project | GET | /v1/projects/{projectName}/services |
| `aiven_get_service` | Get service details | GET | /v1/projects/{projectName}/services/{serviceName} |
| `aiven_list_topics` | List Kafka topics | GET | /v1/projects/{projectName}/services/{serviceName}/topics |
| `aiven_get_topic` | Get Kafka topic details | GET | /v1/projects/{projectName}/services/{serviceName}/topics/{topicName} |
| `aiven_list_access_list` | List ACL rules for topic | GET | /v1/projects/{projectName}/services/{serviceName}/topics/{topicName}/acl |
| `aiven_create_access_list` | Create ACL rule | POST | /v1/projects/{projectName}/services/{serviceName}/topics/{topicName}/acl |
| `aiven_list_connection_pools` | List connection pools | GET | /v1/projects/{projectName}/services/{serviceName}/connectionpools |
| `aiven_get_connection_pool` | Get connection pool details | GET | /v1/projects/{projectName}/services/{serviceName}/connectionpools/{poolName} |

---

## Tool Details

### aiven_list_projects

**What it does**: Lists all Aiven projects the authenticated user has access to. Projects organize services, users, and billing.

**When to use**: Discover available projects, choose which project to work with, list all your Aiven organizations.

**Arguments**:
- `max_results` (optional): Max number of results (default 25)

**Example LLM prompt**: "List all my Aiven projects"

---

### aiven_get_project

**What it does**: Gets details about a specific Aiven project including services, users, and billing info.

**When to use**: Check project status, understand project configuration, view project resources.

**Arguments**:
- `projectName` (required): Aiven project name

**Example LLM prompt**: "Get details about my 'production' project"

---

### aiven_list_services

**What it does**: Lists all managed services in an Aiven project. Services include databases (PostgreSQL, MySQL), Kafka, Redis, Elasticsearch, and more.

**When to use**: Discover services, inventory managed databases, find specific service types.

**Arguments**:
- `projectName` (required): Aiven project name

**Example LLM prompt**: "List all services in my 'production' project"

---

### aiven_get_service

**What it does**: Gets detailed information about a specific Aiven service including connection info and metrics.

**When to use**: Inspect service configuration, get connection details, check service health.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Service name

**Example LLM prompt**: "Get details about my 'pg-primary' PostgreSQL service"

---

### aiven_list_topics

**What it does**: Lists all Kafka topics in an Aiven project service. Topics are categories for organizing Kafka messages.

**When to use**: Discover Kafka topics, understand message streams, find topics for consumption or production.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Kafka service name

**Example LLM prompt**: "List all Kafka topics in my 'kafka-prod' service"

---

### aiven_get_topic

**What it does**: Gets details about a specific Kafka topic including partitions, retention, and cleanup policies.

**When to use**: Understand topic configuration, check retention settings, inspect topic structure.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Kafka service name
- `topicName` (required): Topic name

**Example LLM prompt**: "Get details about the 'user-events' Kafka topic"

---

### aiven_list_access_list

**What it does**: Lists all ACL (Access Control List) rules for a Kafka topic. ACLs control which users and applications can read or write to topics.

**When to use**: Audit topic permissions, see who has access to topics, review security configuration.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Kafka service name
- `topicName` (required): Topic name

**Example LLM prompt**: "List all ACL rules for the 'user-events' topic"

---

### aiven_create_access_list

**What it does**: Creates a new ACL rule for a Kafka topic to grant read or write access.

**When to use**: Grant topic permissions, enable applications to produce or consume messages, manage access control.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Kafka service name
- `topicName` (required): Topic name
- `permission` (required): Permission type: read or write
- `username` (required): Username to grant access

**Example LLM prompt**: "Grant 'app-service' read access to the 'user-events' topic"

---

### aiven_list_connection_pools

**What it does**: Lists all connection pools in an Aiven project service. Connection pools allow multiple applications to share database connections efficiently.

**When to use**: Discover connection pools, understand pool configuration, find pools for applications.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Database service name

**Example LLM prompt**: "List all connection pools in my 'pg-primary' PostgreSQL service"

---

### aiven_get_connection_pool

**What it does**: Gets details about a specific connection pool including pool size and target database.

**When to use**: Inspect pool configuration, check pool size and utilization, understand routing.

**Arguments**:
- `projectName` (required): Aiven project name
- `serviceName` (required): Database service name
- `poolName` (required): Connection pool name

**Example LLM prompt**: "Get details about the 'web-app-pool' connection pool"

---

## Aiven API Notes

- **Service types**: PostgreSQL, MySQL, Kafka, Redis, Elasticsearch, OpenSearch, Grafana, InfluxDB, and more
- **Project names**: Must be unique within an Aiven organization
- **Connection pools**: Used with PostgreSQL and MySQL services to multiplex connections
- **Kafka ACLs**: Control produce and consume permissions per topic per user
- **Billing**: Project-based billing, track costs per service
- **Regions**: Multiple cloud providers and regions available (AWS, GCP, Azure, UpCloud)
