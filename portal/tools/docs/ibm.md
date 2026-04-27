# IBM Cloud Integration

IBM Cloud provides watson AI assistants, serverless cloud functions, Kubernetes containers, managed databases, and object storage.

## Authentication

IBM Cloud uses IAM (Identity and Access Management) for authentication. Configure the following:

- **API Key**: Your IBM Cloud IAM API key
- **Region**: Your IBM Cloud region (e.g., us-south, eu-gb, jp-tok)
- **Account ID**: Your IBM Cloud account identifier

## API Endpoints

- Watson Assistant: `https://api.us-south.assistant.watson.cloud.ibm.com`
- Cloud Functions: `https://cloud.ibm.com`
- VPC Containers: `https://containers.cloud.ibm.com`
- Databases: `https://api.{region}.databases.cloud.ibm.com`
- Object Storage: `https://s3.{region}.cloud-object-storage.cloud.ibm.com`

## Available Tools

### Watson Assistants

| Tool | Name | Description |
|------|------|-------------|
| `ibm_list_watson_assistants` | List Watson Assistants | Retrieve a list of all Watson Assistant instances |
| `ibm_get_watson_assistant` | Get Watson Assistant | Retrieve detailed information for a specific assistant |
| `ibm_create_watson_assistant` | Create Watson Assistant | Create a new Watson Assistant instance |

### Cloud Functions

| Tool | Name | Description |
|------|------|-------------|
| `ibm_list_cloud_functions` | List Cloud Functions | Retrieve a list of all Cloud Functions (actions) |
| `ibm_invoke_cloud_function` | Invoke Cloud Function | Invoke a specific Cloud Function with optional parameters |

### Containers

| Tool | Name | Description |
|------|------|-------------|
| `ibm_list_containers` | List Containers | Retrieve a list of all container clusters |
| `ibm_get_container` | Get Container | Retrieve detailed information for a specific container |

### Databases

| Tool | Name | Description |
|------|------|-------------|
| `ibm_list_databases` | List Databases | Retrieve a list of all databases |
| `ibm_get_database` | Get Database | Retrieve detailed information for a specific database |

### Object Storage

| Tool | Name | Description |
|------|------|-------------|
| `ibm_list_objects` | List Objects | Retrieve a list of all objects in a COS bucket |

## Rate Limits

- Watson Assistant: 100 requests per minute
- Cloud Functions: 1000 invocations per minute
- Databases: Varies by plan

## Error Handling

IBM Cloud API errors return standard HTTP status codes:

| Status Code | Meaning |
|-------------|---------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or expired IAM token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |

## Example Usage

### List Watson Assistants

```json
{
  "tool": "ibm_list_watson_assistants",
  "parameters": {
    "limit": 20
  }
}
```

### Invoke Cloud Function

```json
{
  "tool": "ibm_invoke_cloud_function",
  "parameters": {
    "action_name": "myNamespace/helloWorld",
    "params": {
      "name": "World"
    },
    "blocking": true
  }
}
```

### List Objects in COS Bucket

```json
{
  "tool": "ibm_list_objects",
  "parameters": {
    "bucket": "my-bucket",
    "prefix": "documents/"
  }
}
```
