# AWS IAM Tools

Provider: `aws-iam` | Engine: `nango` | Auth: BASIC via Nango (Access Key ID + Secret Access Key)

## Overview

These tools wrap the AWS IAM API. They allow AI agents to manage users, groups, roles, and policies in AWS Identity and Access Management. AWS IAM provides fine-grained access control to AWS resources.

## Authentication

**Nango BASIC Auth**:
- User provides AWS Access Key ID (username) and Secret Access Key (password)
- Token stored in Nango, accessed via `connection_ref`
- Region configuration required

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `aws_iam_list_users` | List IAM users | GET | / |
| `aws_iam_get_user` | Get user details | GET | / |
| `aws_iam_list_groups` | List IAM groups | GET | / |
| `aws_iam_list_attached_policies` | List attached policies | GET | / |
| `aws_iam_list_roles` | List IAM roles | GET | / |
| `aws_iam_get_role` | Get role details | GET | / |
| `aws_iam_list_access_keys` | List access keys | GET | / |
| `aws_iam_list_policies` | List managed policies | GET | / |
| `aws_iam_get_policy` | Get policy details | GET | / |
| `aws_iam_list_account_aliases` | List account aliases | GET | / |

---

## Tool Details

### aws_iam_list_users

**What it does**: Lists all IAM users in the AWS account.

**When to use**: Browse users, find specific users for policy management.

**Arguments**:
- `PathPrefix` (optional): Filter by path prefix
- `MaxItems` (optional): Max items to return (default 20)

**Example LLM prompt**: "List all IAM users in our account"

---

### aws_iam_get_user

**What it does**: Gets detailed information about a specific IAM user.

**When to use**: Check user permissions, attached policies, and access keys.

**Arguments**:
- `UserName` (required): IAM user name

**Example LLM prompt**: "Get details for user john.doe"

---

### aws_iam_list_groups

**What it does**: Lists all IAM groups in the account.

**When to use**: View group structure, manage user memberships.

**Arguments**:
- `MaxItems` (optional): Max items to return (default 20)

**Example LLM prompt**: "List all IAM groups"

---

### aws_iam_list_attached_policies

**What it does**: Lists managed policies attached to a user, group, or role.

**When to use**: Check effective permissions, audit access.

**Arguments**:
- `EntityName` (required): User, group, or role name

**Example LLM prompt**: "List policies attached to the admin group"

---

### aws_iam_list_roles

**What it does**: Lists all IAM roles in the account.

**When to use**: Browse roles, find roles for service access.

**Arguments**:
- `PathPrefix` (optional): Filter by path prefix
- `MaxItems` (optional): Max items to return (default 20)

**Example LLM prompt**: "List all IAM roles"

---

### aws_iam_get_role

**What it does**: Gets detailed information about a specific role including trust policy.

**When to use**: Check role permissions, understand service access.

**Arguments**:
- `RoleName` (required): IAM role name

**Example LLM prompt**: "Get details for the Lambda execution role"

---

### aws_iam_list_access_keys

**What it does**: Lists access keys for a specific IAM user.

**When to use**: Audit access keys, check key age and status.

**Arguments**:
- `UserName` (required): IAM user name

**Example LLM prompt**: "List access keys for user john.doe"

---

### aws_iam_list_policies

**What it does**: Lists all managed policies in the account.

**When to use**: Browse available policies, find specific permissions.

**Arguments**:
- `Scope` (optional): Policy scope (Local, AWS, All)
- `MaxItems` (optional): Max items to return (default 20)

**Example LLM prompt**: "List all managed policies"

---

### aws_iam_get_policy

**What it does**: Gets details of a managed policy including the default version.

**When to use**: Review policy permissions, understand access scope.

**Arguments**:
- `PolicyArn` (required): Policy ARN

**Example LLM prompt**: "Get details for policy arn:aws:iam::123456789012:policy/MyPolicy"

---

### aws_iam_list_account_aliases

**What it does**: Lists account aliases associated with the AWS account.

**When to use**: Identify which AWS account you are working with.

**Arguments**: None required

**Example LLM prompt**: "List our AWS account aliases"

---

## AWS IAM API Notes

- **ARN Format**: Policy and role ARNs follow `arn:aws:iam::account-id:resource-type/resource-name`
- **Path Prefixes**: Use paths to organize users, roles, and policies
- **Access Keys**: Include access key ID and creation date; secret key is not returned
- **Policy Versions**: Managed policies can have multiple versions; one is default
- **Region**: IAM is a global service; region parameter may be ignored
