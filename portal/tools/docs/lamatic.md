# Lamatic Tools - AI Platform

Provider: `lamatic` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Lamatic AI platform API. They allow AI agents to manage AI models, deployments, endpoints, and inference. Lamatic is a low-code platform for building and deploying AI models.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Lamatic
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `models:read`, `models:write`, `deployments:read`, `deployments:write`, `endpoints:read`, `endpoints:write`, `inference:write`, `logs:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lamatic_list_models` | List available AI models | GET | /models |
| `lamatic_get_model` | Get model details | GET | /models/{model_id} |
| `lamatic_deploy_model` | Deploy a model | POST | /deployments |
| `lamatic_create_endpoint` | Create an API endpoint | POST | /endpoints |
| `lamatic_list_endpoints` | List endpoints | GET | /endpoints |
| `lamatic_get_endpoint` | Get endpoint details | GET | /endpoints/{endpoint_id} |
| `lamatic_invoke_endpoint` | Invoke endpoint for inference | POST | /endpoints/{endpoint_id}/invoke |
| `lamatic_list_deployments` | List deployments | GET | /deployments |
| `lamatic_get_deployment` | Get deployment details | GET | /deployments/{deployment_id} |
| `lamatic_get_inference_log` | Get inference log | GET | /inference-logs/{log_id} |

---

## Tool Details

### lamatic_list_models

**What it does**: Retrieves a list of all available AI models in the Lamatic platform.

**When to use**: Discover what models are available for deployment.

**Arguments**: None

**Example LLM prompt**: "What AI models are available in Lamatic?"

---

### lamatic_get_model

**What it does**: Gets detailed information about a specific model by ID.

**When to use**: Understand model capabilities before deployment.

**Arguments**:
- `model_id` (required): The unique identifier of the model

**Example LLM prompt**: "Get details for model abc123"

---

### lamatic_deploy_model

**What it does**: Deploys a model to make it available for inference.

**When to use**: Provision a model for production use.

**Arguments**:
- `model_id` (required): The model ID to deploy
- `instance_type` (optional): The instance type for deployment

**Example LLM prompt**: "Deploy model abc123 with gpu-large instance type"

---

### lamatic_create_endpoint

**What it does**: Creates a new API endpoint for accessing deployed models.

**When to use**: Set up an endpoint to serve inference requests.

**Arguments**:
- `name` (required): Name for the endpoint
- `deployment_id` (required): The deployment ID to attach

**Example LLM prompt**: "Create an endpoint called 'chatbot' attached to deployment def456"

---

### lamatic_list_endpoints

**What it does**: Retrieves a list of all API endpoints.

**When to use**: View all available inference endpoints.

**Arguments**: None

**Example LLM prompt**: "List all endpoints in Lamatic"

---

### lamatic_get_endpoint

**What it does**: Gets details for a specific endpoint by ID.

**When to use**: Check endpoint status and configuration.

**Arguments**:
- `endpoint_id` (required): The unique identifier of the endpoint

**Example LLM prompt**: "Get details for endpoint ghi789"

---

### lamatic_invoke_endpoint

**What it does**: Sends an inference request to a deployed endpoint.

**When to use**: Run inference on a deployed model.

**Arguments**:
- `endpoint_id` (required): The endpoint ID to invoke
- `input` (required): The input prompt or data for inference

**Example LLM prompt**: "Invoke endpoint ghi789 with input 'Hello, how are you?'"

---

### lamatic_list_deployments

**What it does**: Retrieves a list of all model deployments.

**When to use**: Monitor active deployments.

**Arguments**: None

**Example LLM prompt**: "List all deployments"

---

### lamatic_get_deployment

**What it does**: Gets details for a specific deployment by ID.

**When to use**: Check deployment status and health.

**Arguments**:
- `deployment_id` (required): The unique identifier of the deployment

**Example LLM prompt**: "Get details for deployment jkl012"

---

### lamatic_get_inference_log

**What it does**: Retrieves the inference log for a specific deployment or endpoint.

**When to use**: Debug inference issues, review request history.

**Arguments**:
- `log_id` (required): The unique identifier of the inference log

**Example LLM prompt**: "Get the inference log mno345"

---

## Lamatic API Notes

- **Models**: AI models that can be deployed for inference
- **Deployments**: Running instances of deployed models
- **Endpoints**: API endpoints for sending inference requests
- **Instance types**: Different compute configurations for deployments
- **Inference logs**: Historical record of inference requests and responses
