# AWS Tools

Provider: `aws` | Engine: `nango` | Auth: API_KEY / OAuth (via Nango)

## Overview

These tools wrap the AWS API (EC2, S3, Lambda, IAM, RDS). They allow AI agents to interact with AWS resources on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `s3:*`, `ec2:DescribeInstances`, `lambda:*`, `iam:*`, `rds:DescribeDBInstances`

**Native API_KEY (engine=omini_connect_native)**:
- Access Key ID and Secret Access Key stored in `client_secret` field
- Direct AWS API passthrough with signature version 4

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `aws_list_ec2_instances` | List EC2 instances | GET | /ec2/instances |
| `aws_get_ec2_instance` | Get EC2 instance details | GET | /ec2/instances/{instance_id} |
| `aws_list_s3_buckets` | List S3 buckets | GET | / |
| `aws_list_s3_objects` | List objects in a bucket | GET | /{bucket} |
| `aws_get_s3_object` | Get S3 object metadata | GET | /{bucket}/{key} |
| `aws_list_lambda_functions` | List Lambda functions | GET | /lambda/functions |
| `aws_get_lambda_function` | Get Lambda function details | GET | /lambda/functions/{function_name} |
| `aws_list_iam_users` | List IAM users | GET | /iam/users |
| `aws_get_iam_user` | Get IAM user details | GET | /iam/users/{user_name} |
| `aws_list_rds_instances` | List RDS instances | GET | /rds/instances |

---

## Tool Details

### aws_list_ec2_instances

**What it does**: Lists EC2 instances with their IDs, types, states, and tags. Can be filtered by state or other criteria.

**When to use**: Check running instances, find instances by tag, or get an overview of compute resources.

**Arguments**:
- `filters` (optional): Key-value pairs to filter instances (e.g., instance-state-name: running)
- `max_results` (optional): Maximum instances to return (default 100)

**Example LLM prompt**: "List all running EC2 instances"

---

### aws_get_ec2_instance

**What it does**: Returns detailed information about a specific EC2 instance including state, type, security groups, and networking.

**When to use**: Get specific instance details, check instance health, or gather configuration info.

**Arguments**:
- `instance_id` (required): EC2 instance ID

**Example LLM prompt**: "What are the details of instance i-1234567890abcdef0?"

---

### aws_list_s3_buckets

**What it does**: Returns a list of all S3 buckets owned by the authenticated AWS account.

**When to use**: When you need to know what S3 buckets exist in the account before listing or accessing objects.

**Arguments**: None

**Example LLM prompt**: "What S3 buckets do I have in AWS?"

---

### aws_list_s3_objects

**What it does**: Lists objects in a specified S3 bucket with optional prefix filtering.

**When to use**: Browse files in a bucket, find objects by path prefix, or enumerate bucket contents.

**Arguments**:
- `bucket` (required): S3 bucket name
- `prefix` (optional): Filter objects by prefix
- `max_keys` (optional): Maximum objects to return (default 1000)
- `continuation_token` (optional): Pagination token

**Example LLM prompt**: "List all objects in my s3-bucket under the 'data/' prefix"

---

### aws_get_s3_object

**What it does**: Retrieves an object from S3 including its content, metadata, and version info.

**When to use**: Read files, fetch configuration data, or download content from S3.

**Arguments**:
- `bucket` (required): S3 bucket name
- `key` (required): Object key/path
- `version_id` (optional): Specific object version

**Example LLM prompt**: "Get the file at s3-bucket/models/config.json"

---

### aws_list_lambda_functions

**What it does**: Lists all Lambda functions with their configurations, runtimes, and memory settings.

**When to use**: Discover available Lambda functions, check function configurations, or find functions to invoke.

**Arguments**:
- `max_items` (optional): Maximum functions to return (default 100)
- `marker` (optional): Pagination token

**Example LLM prompt**: "What Lambda functions are deployed in my account?"

---

### aws_get_lambda_function

**What it does**: Returns configuration details of a Lambda function including runtime, handler, environment variables, and resource usage.

**When to use**: Check function settings, verify configuration, or understand function capabilities.

**Arguments**:
- `function_name` (required): Lambda function name or ARN

**Example LLM prompt**: "Show me the configuration of my process-data Lambda function"

---

### aws_list_iam_users

**What it does**: Lists all IAM users with their names, IDs, creation dates, and last activity timestamps.

**When to use**: Discover users in the account, audit access, or find users to check permissions.

**Arguments**:
- `max_items` (optional): Maximum users to return (default 100)
- `marker` (optional): Pagination token

**Example LLM prompt**: "List all IAM users in the account"

---

### aws_get_iam_user

**What it does**: Returns details about a specific IAM user including attached policies, group memberships, and access keys.

**When to use**: Audit user permissions, check access key status, or understand user capabilities.

**Arguments**:
- `user_name` (required): IAM username

**Example LLM prompt**: "Get details for the user 'deploy-bot'"

---

### aws_list_rds_instances

**What it does**: Lists all RDS database instances with their identifiers, engines, states, and configurations.

**When to use**: Discover database instances, check instance status, or find databases for connection.

**Arguments**:
- `region` (optional): AWS region
- `max_records` (optional): Maximum instances to return (default 100)

**Example LLM prompt**: "What RDS instances are running in us-east-1?"

---

## AWS API Reference

These tools use the AWS API. See official docs for full details:
- https://docs.aws.amazon.com/
- Rate limits: Vary by service and request type
- Authentication: AWS Signature Version 4
- All dates: ISO 8601 format