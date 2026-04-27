# Google Cloud Storage Tools

Provider: `google-cloud-storage` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Cloud Storage API. They allow AI agents to manage buckets, objects, and ACLs. **Requires Google OAuth2 with Cloud Storage permissions.**

## Authentication

**Nango OAUTH2 (Cloud Storage)**:
- User authenticates via OAuth2 with Cloud Storage scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://storage.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_cloud_storage_list_buckets` | List buckets | GET | /storage/v1/b |
| `google_cloud_storage_get_bucket` | Get bucket details | GET | /storage/v1/b/{bucket} |
| `google_cloud_storage_list_objects` | List objects | GET | /storage/v1/b/{bucket}/o |
| `google_cloud_storage_get_object` | Get object details | GET | /storage/v1/b/{bucket}/o/{object} |
| `google_cloud_storage_insert_object` | Upload object | POST | /upload/storage/v1/b/{bucket}/o |
| `google_cloud_storage_copy_object` | Copy object | POST | /storage/v1/b/{sourceBucket}/o/{sourceObject}/copyTo/b/{destBucket}/o/{destObject} |
| `google_cloud_storage_delete_object` | Delete object | DELETE | /storage/v1/b/{bucket}/o/{object} |
| `google_cloud_storage_get_acl` | Get ACL for object | GET | /storage/v1/b/{bucket}/o/{object}/acl |
| `google_cloud_storage_insert_acl` | Add ACL entry | POST | /storage/v1/b/{bucket}/o/{object}/acl |
| `google_cloud_storage_list_default_acl` | List default ACL | GET | /storage/v1/b/{bucket}/defaultObjectAcl |

---

## Tool Details

### google_cloud_storage_list_buckets

**What it does**: Lists all buckets in the project.

**When to use**: Browse available storage buckets.

**Arguments**:
- `project` (required): Google Cloud project ID

**Example LLM prompt**: "List all buckets in my project"

---

### google_cloud_storage_get_bucket

**What it does**: Gets detailed information about a bucket.

**When to use**: Get bucket metadata and settings.

**Arguments**:
- `bucket` (required): Bucket name

**Example LLM prompt**: "Get details for bucket my-bucket"

---

### google_cloud_storage_list_objects

**What it does**: Lists objects in a bucket.

**When to use**: Browse files in a bucket.

**Arguments**:
- `bucket` (required): Bucket name
- `prefix` (optional): Object name prefix filter

**Example LLM prompt**: "List objects in bucket my-bucket with prefix 'folder/'"

---

### google_cloud_storage_get_object

**What it does**: Gets detailed information about an object.

**When to use**: Get object metadata.

**Arguments**:
- `bucket` (required): Bucket name
- `object` (required): Object name (URL-encoded)

**Example LLM prompt**: "Get details for object my-file.txt in bucket my-bucket"

---

### google_cloud_storage_insert_object

**What it does**: Uploads an object to a bucket.

**When to use**: Upload files to Cloud Storage.

**Arguments**:
- `bucket` (required): Bucket name
- `name` (required): Object name
- `content` (optional): Object content (base64-encoded)

**Example LLM prompt**: "Upload file my-file.txt to bucket my-bucket"

---

### google_cloud_storage_copy_object

**What it does**: Copies an object to a new location.

**When to use**: Duplicate files in storage.

**Arguments**:
- `sourceBucket` (required): Source bucket name
- `sourceObject` (required): Source object name
- `destBucket` (required): Destination bucket name
- `destObject` (required): Destination object name

**Example LLM prompt**: "Copy object file.txt from bucket-a to bucket-b"

---

### google_cloud_storage_delete_object

**What it does**: Deletes an object from a bucket.

**When to use**: Remove files from storage.

**Arguments**:
- `bucket` (required): Bucket name
- `object` (required): Object name (URL-encoded)

**Example LLM prompt**: "Delete object my-file.txt from bucket my-bucket"

---

### google_cloud_storage_get_acl

**What it does**: Gets ACL for an object.

**When to use**: Check who has access to an object.

**Arguments**:
- `bucket` (required): Bucket name
- `object` (required): Object name

**Example LLM prompt**: "Get ACL for object my-file.txt in bucket my-bucket"

---

### google_cloud_storage_insert_acl

**What it does**: Adds an ACL entry for an object.

**When to use**: Grant access to objects.

**Arguments**:
- `bucket` (required): Bucket name
- `object` (required): Object name
- `entity` (required): ACL entity (e.g., user:email@example.com)
- `role` (required): ACL role (READER, WRITER, OWNER)

**Example LLM prompt**: "Grant OWNER access to user@example.com for my-file.txt"

---

### google_cloud_storage_list_default_acl

**What it does**: Lists default ACL for a bucket.

**When to use**: See default access settings for new objects.

**Arguments**:
- `bucket` (required): Bucket name

**Example LLM prompt**: "List default ACL for bucket my-bucket"

---

## Cloud Storage API Notes

- **Bucket names**: Globally unique names
- **Object names**: URL-encoded path within bucket
- **ACL entities**: user:, group:, allUsers, allAuthenticatedUsers
- **Roles**: READER, WRITER, OWNER
- **Default ACL**: Applied to new objects in bucket
