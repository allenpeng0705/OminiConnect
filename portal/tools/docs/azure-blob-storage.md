# Azure Blob Storage Tools

Provider: `azure-blob-storage` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Azure Blob Storage REST API. They allow AI agents to manage containers and blobs for file storage. Azure Blob Storage is Microsoft's object storage solution for the cloud, optimized for storing massive amounts of unstructured data.

## Authentication

**Nango OAuth2**:
- User authenticates via OAuth2 with Azure Storage
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `https://storage.azure.com/.default`
- Requires storage account name configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `azure_blob_list_containers` | List blob containers | GET | /?comp=list |
| `azure_blob_list_blobs` | List blobs in container | GET | /{container}?restype=container&comp=list |
| `azure_blob_get_blob` | Get blob properties | GET | /{container}/{blob} |
| `azure_blob_download_blob` | Download blob content | GET | /{container}/{blob}?comp=blocklist |
| `azure_blob_upload_blob` | Upload blob content | PUT | /{container}/{blob}?comp=blocklist |
| `azure_blob_delete_blob` | Delete blob | DELETE | /{container}/{blob} |
| `azure_blob_create_container` | Create container | PUT | /{container}?restype=container |
| `azure_blob_delete_container` | Delete container | DELETE | /{container}?restype=container |
| `azure_blob_get_container_props` | Get container properties | HEAD | /{container}?restype=container |
| `azure_blob_copy_blob` | Copy blob | PUT | /{container}/{blob}?comp=copy |

---

## Tool Details

### azure_blob_list_containers

**What it does**: Lists all blob containers in the storage account.

**When to use**: Browse available containers, find data location.

**Arguments**:
- `maxresults` (optional): Max containers to return (default 20)
- `prefix` (optional): Filter containers by prefix

**Example LLM prompt**: "List all containers in my storage account"

---

### azure_blob_list_blobs

**What it does**: Lists all blobs in a specific container.

**When to use**: Browse files in a container, find specific blobs.

**Arguments**:
- `container` (required): Container name
- `prefix` (optional): Filter blobs by prefix
- `include` (optional): Include additional data (snapshots, metadata, uncommittedblobs)

**Example LLM prompt**: "List all blobs in the 'documents' container"

---

### azure_blob_get_blob

**What it does**: Gets properties and metadata of a specific blob.

**When to use**: Check blob size, type, creation date, or custom metadata.

**Arguments**:
- `container` (required): Container name
- `blob` (required): Blob name (URL-encoded)

**Example LLM prompt**: "Get properties for blob 'reports/q1-2024.pdf' in container 'data'"

---

### azure_blob_download_blob

**What it does**: Downloads the content of a blob.

**When to use**: Retrieve file content, read data.

**Arguments**:
- `container` (required): Container name
- `blob` (required): Blob name (URL-encoded)

**Example LLM prompt**: "Download the file 'backup/latest.tar.gz' from container 'backups'"

---

### azure_blob_upload_blob

**What it does**: Uploads or creates a blob with content.

**When to use**: Store files, backup data, save processed content.

**Arguments**:
- `container` (required): Container name
- `blob` (required): Blob name (URL-encoded)
- `content` (required): Blob content (base64-encoded)
- `contentType` (optional): MIME type (default: application/octet-stream)

**Example LLM prompt**: "Upload 'Hello World' as 'welcome.txt' to container 'public'"

---

### azure_blob_delete_blob

**What it does**: Deletes a specific blob.

**When to use**: Remove files, clean up storage.

**Arguments**:
- `container` (required): Container name
- `blob` (required): Blob name (URL-encoded)

**Example LLM prompt**: "Delete blob 'temp/file.tmp' from container 'staging'"

---

### azure_blob_create_container

**What it does**: Creates a new blob container.

**When to use**: Organize data, create new storage partitions.

**Arguments**:
- `container` (required): Container name
- `publicAccess` (optional): Public access level (blob, container, none)

**Example LLM prompt**: "Create a container called 'project-data'"

---

### azure_blob_delete_container

**What it does**: Deletes a container and all its blobs.

**When to use**: Remove entire storage partition, clean up.

**Arguments**:
- `container` (required): Container name

**Example LLM prompt**: "Delete container 'temp-uploads'"

---

### azure_blob_get_container_props

**What it does**: Gets properties of a container including metadata.

**When to use**: Check container settings, lease status.

**Arguments**:
- `container` (required): Container name

**Example LLM prompt**: "Get properties for container 'archive'"

---

### azure_blob_copy_blob

**What it does**: Copies a blob from source to destination.

**When to use**: Duplicate files, backup data, create archives.

**Arguments**:
- `container` (required): Container name
- `blob` (required): Destination blob name
- `source` (required): Source blob URL

**Example LLM prompt**: "Copy 'data/file.csv' to 'archive/file.csv' in container 'backups'"

---

## Azure Blob Storage API Notes

- **Container Names**: Must be lowercase, 3-63 characters, start with letter or number
- **Blob Names**: URL-encoded, case-sensitive, can contain forward slashes
- **Public Access**: Can be set at container or blob level for public content
- **Metadata**: Custom key-value pairs attached to blobs and containers
- **Block Blobs**: Optimized for large files, supports parallel upload
