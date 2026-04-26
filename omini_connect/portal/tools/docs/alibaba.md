# Alibaba Cloud Tools

## Overview

[Alibaba Cloud](https://www.alibabacloud.com) is the cloud computing division of Alibaba Group, offering a comprehensive suite of cloud services including computing, storage, databases, networking, and security.

## Authentication

Alibaba Cloud uses AccessKey/SecretKey authentication with RAM (Resource Access Management) for API access. The following scopes are supported:

| Scope | Description |
|-------|-------------|
| `ecs` | Elastic Compute Service - cloud virtual machines |
| `slb` | Server Load Balancer - traffic distribution |
| `rds` | Relational Database Service - managed databases |
| `oss` | Object Storage Service - scalable storage |
| `sms` | DirectMail - SMS messaging service |

## Provider Configuration

```yaml
provider: alibaba
auth_mode: ACCESS_KEY
base_url: https://ecs.aliyuncs.com  # varies by service
```

## Tools

### ECS (Elastic Compute) Tools

#### alibaba_list_ecs_instances

List all Elastic Compute Service (ECS) instances in Alibaba Cloud.

**Endpoint:** `GET /ecs/v1/instances`

**Scopes:** `ecs`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region_id | string | Yes | Alibaba Cloud region ID (e.g., cn-hangzhou, us-west-1). |
| status | string | No | Instance status: Running, Stopped, etc. |
| page_size | integer | No | Number of instances per page (max 100). |
| page_number | integer | No | Page number for pagination. |

#### alibaba_get_ecs_instance

Get details of a specific ECS instance by ID.

**Endpoint:** `GET /ecs/v1/instances/{instance_id}`

**Scopes:** `ecs`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| instance_id | string | Yes | The ECS instance ID. |
| region_id | string | Yes | Alibaba Cloud region ID. |

#### alibaba_create_ecs_instance

Create a new Elastic Compute Service (ECS) instance in Alibaba Cloud.

**Endpoint:** `POST /ecs/v1/instances`

**Scopes:** `ecs`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region_id | string | Yes | Alibaba Cloud region ID. |
| image_id | string | Yes | OS image ID to use for the instance. |
| instance_type | string | Yes | Instance type (e.g., ecs.n4.small). |
| security_group_id | string | No | Security group ID to attach. |
| vswitch_id | string | No | VSwitch ID for VPC network. |
| instance_name | string | No | Display name for the instance. |

### SLB (Server Load Balancer) Tools

#### alibaba_list_slbs

List all Server Load Balancer (SLB) instances in Alibaba Cloud.

**Endpoint:** `GET /slb/v1/loadbalancers`

**Scopes:** `slb`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region_id | string | Yes | Alibaba Cloud region ID. |
| page_size | integer | No | Number of results per page (max 100). |
| page_number | integer | No | Page number for pagination. |

#### alibaba_get_slb

Get details of a specific Server Load Balancer by ID.

**Endpoint:** `GET /slb/v1/loadbalancers/{loadbalancer_id}`

**Scopes:** `slb`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| loadbalancer_id | string | Yes | The SLB instance ID. |
| region_id | string | Yes | Alibaba Cloud region ID. |

### RDS (Relational Database) Tools

#### alibaba_list_rds_instances

List all Relational Database Service (RDS) instances in Alibaba Cloud.

**Endpoint:** `GET /rds/v1/instances`

**Scopes:** `rds`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region_id | string | Yes | Alibaba Cloud region ID. |
| engine | string | No | Database engine: MySQL, PostgreSQL, SQLServer, MariaDB. |
| status | string | No | Instance status: Running, Stopped, etc. |
| page_size | integer | No | Number of results per page (max 100). |
| page_number | integer | No | Page number for pagination. |

#### alibaba_get_rds_instance

Get details of a specific RDS database instance by ID.

**Endpoint:** `GET /rds/v1/instances/{instance_id}`

**Scopes:** `rds`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| instance_id | string | Yes | The RDS instance ID. |
| region_id | string | Yes | Alibaba Cloud region ID. |

### OSS (Object Storage) Tools

#### alibaba_list_oss_buckets

List all Object Storage Service (OSS) buckets in Alibaba Cloud.

**Endpoint:** `GET /oss/v1/buckets`

**Scopes:** `oss`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region_id | string | Yes | Alibaba Cloud region ID. |

#### alibaba_get_oss_bucket

Get details and list objects in an OSS bucket.

**Endpoint:** `GET /oss/v1/buckets/{bucket_name}`

**Scopes:** `oss`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| bucket_name | string | Yes | Name of the OSS bucket. |
| region_id | string | Yes | Alibaba Cloud region ID. |
| max_keys | integer | No | Maximum number of objects to return (max 1000). |
| marker | string | No | Object name to start listing from for pagination. |

### SMS (DirectMail) Tools

#### alibaba_send_sms

Send an SMS message using Alibaba Cloud DirectMail.

**Endpoint:** `POST /dysms/v1/regions/{region_id}/sms`

**Scopes:** `sms`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region_id | string | Yes | Alibaba Cloud region ID. |
| phone_numbers | string | Yes | Recipient phone number(s), comma-separated for multiple. |
| sign_name | string | Yes | SMS signature name (approved by Alibaba Cloud). |
| template_code | string | Yes | SMS template code. |
| template_param | string | No | JSON string of template parameter key-value pairs. |
