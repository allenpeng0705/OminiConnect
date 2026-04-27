# Tencent Cloud Tools

## Overview

[Tencent Cloud](https://cloud.tencent.com) is the cloud computing platform of Tencent, offering a wide range of cloud services including computing, storage, databases, networking, and security.

## Authentication

Tencent Cloud uses SecretId/SecretKey authentication (API keys) for API access. The following scopes are supported:

| Scope | Description |
|-------|-------------|
| `cvm` | Cloud Virtual Machine - elastic compute |
| `cos` | Cloud Object Storage - scalable object storage |
| `cbs` | Cloud Block Storage - block storage volumes |
| `vpc` | Virtual Private Cloud - networking |
| `clb` | Cloud Load Balancer - traffic distribution |

## Provider Configuration

```yaml
provider: tencent
auth_mode: SECRET_KEY
base_url: https://cloud.tencent.com/api  # varies by service
```

## Tools

### CVM (Cloud Virtual Machine) Tools

#### tencent_list_cvm_instances

List all Cloud Virtual Machine (CVM) instances in Tencent Cloud.

**Endpoint:** `GET /cvm/v1/instances`

**Scopes:** `cvm`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region | string | Yes | Tencent Cloud region (e.g., ap-guangzhou, ap-singapore). |
| filters | string | No | Filter conditions as JSON string (e.g., {'instance-state-name': ['RUNNING']}). |
| limit | integer | No | Number of instances to return (max 100). |
| offset | integer | No | Offset for pagination. |

#### tencent_get_cvm_instance

Get details of a specific CVM instance by ID.

**Endpoint:** `GET /cvm/v1/instances/{instance_id}`

**Scopes:** `cvm`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| instance_id | string | Yes | The CVM instance ID. |
| region | string | Yes | Tencent Cloud region. |

### COS (Cloud Object Storage) Tools

#### tencent_list_cos_buckets

List all Cloud Object Storage (COS) buckets in Tencent Cloud.

**Endpoint:** `GET /cos/v1/buckets`

**Scopes:** `cos`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region | string | Yes | Tencent Cloud region. |

#### tencent_list_cos_objects

List objects in a COS bucket.

**Endpoint:** `GET /cos/v1/buckets/{bucket}/objects`

**Scopes:** `cos`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| bucket | string | Yes | COS bucket name (e.g., mybucket-1234567890). |
| region | string | Yes | Tencent Cloud region. |
| prefix | string | No | Object key prefix to filter results. |
| marker | string | No | Object key to start listing from for pagination. |
| max_keys | integer | No | Maximum number of objects to return (max 1000). |

#### tencent_upload_cos_object

Upload a file to COS bucket in Tencent Cloud.

**Endpoint:** `POST /cos/v1/buckets/{bucket}/objects`

**Scopes:** `cos`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| bucket | string | Yes | COS bucket name. |
| region | string | Yes | Tencent Cloud region. |
| key | string | Yes | Object key (file path in bucket). |
| body | string | Yes | Base64-encoded file content. |
| content_type | string | No | MIME type of the file (e.g., image/jpeg, application/pdf). |

### CBS (Cloud Block Storage) Tools

#### tencent_list_cbs_volumes

List all Cloud Block Storage (CBS) volumes in Tencent Cloud.

**Endpoint:** `GET /cbs/v1/volumes`

**Scopes:** `cbs`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region | string | Yes | Tencent Cloud region. |
| filters | string | No | Filter conditions as JSON string. |
| limit | integer | No | Number of volumes to return (max 100). |
| offset | integer | No | Offset for pagination. |

#### tencent_get_cbs_volume

Get details of a specific CBS volume by ID.

**Endpoint:** `GET /cbs/v1/volumes/{volume_id}`

**Scopes:** `cbs`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| volume_id | string | Yes | The CBS volume ID. |
| region | string | Yes | Tencent Cloud region. |

### VPC (Virtual Private Cloud) Tools

#### tencent_list_vpcs

List all Virtual Private Cloud (VPC) networks in Tencent Cloud.

**Endpoint:** `GET /vpc/v1/vpcs`

**Scopes:** `vpc`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region | string | Yes | Tencent Cloud region. |
| limit | integer | No | Number of VPCs to return (max 100). |
| offset | integer | No | Offset for pagination. |

#### tencent_get_vpc

Get details of a specific VPC by ID.

**Endpoint:** `GET /vpc/v1/vpcs/{vpc_id}`

**Scopes:** `vpc`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| vpc_id | string | Yes | The VPC ID. |
| region | string | Yes | Tencent Cloud region. |

### CLB (Cloud Load Balancer) Tools

#### tencent_list_load_balancers

List all CLB (Cloud Load Balancer) instances in Tencent Cloud.

**Endpoint:** `GET /clb/v1/loadbalancers`

**Scopes:** `clb`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| region | string | Yes | Tencent Cloud region. |
| filters | string | No | Filter conditions as JSON string. |
| limit | integer | No | Number of load balancers to return (max 100). |
| offset | integer | No | Offset for pagination. |
