# Databricks Account Level Tools

Provider: `databricks-account` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Databricks Account API. They allow AI agents to manage workspaces, users, groups, clusters, and jobs at the account level. Databricks is a unified analytics platform built on Apache Spark.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine authentication
- Token stored in Nango, accessed via `connection_ref`
- Account ID configured per connection for multi-account access

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `databricks_account_list_workspaces` | List workspaces | GET | /2.0/accounts/{accountId}/workspaces |
| `databricks_account_get_workspace` | Get workspace details | GET | /2.0/accounts/{accountId}/workspaces/{workspaceId} |
| `databricks_account_list_users` | List account users | GET | /2.0/accounts/{accountId}/users |
| `databricks_account_get_user` | Get user details | GET | /2.0/accounts/{accountId}/users/{userId} |
| `databricks_account_list_groups` | List account groups | GET | /2.0/accounts/{accountId}/groups |
| `databricks_account_get_group` | Get group details | GET | /2.0/accounts/{accountId}/groups/{groupId} |
| `databricks_account_list_clusters` | List clusters | GET | /2.0/accounts/{accountId}/clusters |
| `databricks_account_get_cluster` | Get cluster details | GET | /2.0/accounts/{accountId}/clusters/{clusterId} |
| `databricks_account_list_jobs` | List jobs | GET | /2.0/accounts/{accountId}/jobs |
| `databricks_account_get_job` | Get job details | GET | /2.0/accounts/{accountId}/jobs/{jobId} |

---

## Tool Details

### databricks_account_list_workspaces

**What it does**: Lists all Databricks workspaces in the account.

**When to use**: View all deployed workspaces, find workspace by region or name.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all Databricks workspaces"

---

### databricks_account_get_workspace

**What it does**: Gets detailed workspace information including deployment and storage.

**When to use**: Check workspace configuration, verify workspace status, review workspace settings.

**Arguments**:
- `workspace_id` (required): Workspace ID

**Example LLM prompt**: "Get details for workspace ws-123"

---

### databricks_account_list_users

**What it does**: Lists all users in the Databricks account.

**When to use**: View account users, find users by email, manage user access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all users in the account"

---

### databricks_account_get_user

**What it does**: Gets detailed user information including groups and permissions.

**When to use**: Check user permissions, verify group membership, review user access.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-456"

---

### databricks_account_list_groups

**What it does**: Lists all groups in the account.

**When to use**: View group structure, find groups by name, manage permissions.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all groups"

---

### databricks_account_get_group

**What it does**: Gets detailed group information including member list.

**When to use**: Review group membership, check group permissions, manage team access.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group g-789"

---

### databricks_account_list_clusters

**What it does**: Lists all clusters across the account.

**When to use**: View cluster inventory, check cluster status, find idle clusters.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all clusters in the account"

---

### databricks_account_get_cluster

**What it does**: Gets detailed cluster configuration and current status.

**When to use**: Check cluster health, verify node configuration, review cluster logs.

**Arguments**:
- `cluster_id` (required): Cluster ID

**Example LLM prompt**: "Get details for cluster cl-101"

---

### databricks_account_list_jobs

**What it does**: Lists all jobs in the account.

**When to use**: View scheduled workflows, find jobs by name, track job status.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all jobs"

---

### databricks_account_get_job

**What it does**: Gets detailed job configuration including schedule and tasks.

**When to use**: Check job settings, review task dependencies, verify schedules.

**Arguments**:
- `job_id` (required): Job ID

**Example LLM prompt**: "Get details for job j-202"

---

## Databricks Account API Notes

- **Account Level**: Access resources across all workspaces in the account
- **Client Credentials**: Uses OAuth2 machine-to-machine flow for service principals
- **Account ID**: UUID format, required for all account-level API calls
- **Workspaces**: Individual Databricks deployments with isolated notebooks and clusters
- **Users/Groups**: Account-level IAM for access control across workspaces
- **Clusters**: Compute resources for running Spark jobs and notebooks
- **Jobs**: Scheduled or triggered workflows running notebooks or JARs
