# Azure Tools

Provider: `azure` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Azure Resource Manager API. They allow AI agents to manage Azure resources including virtual machines, storage accounts, resource groups, and blob containers. Azure is Microsoft's cloud computing platform offering compute, storage, networking, and container services.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Microsoft Azure
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `https://management.azure.com/user_impersonation`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `azure_list_resource_groups` | List Azure resource groups | GET | /subscriptions/{subscriptionId}/resourcegroups |
| `azure_get_resource_group` | Get resource group details | GET | /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName} |
| `azure_create_resource_group` | Create a new resource group | PUT | /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName} |
| `azure_list_resources` | List resources in subscription | GET | /subscriptions/{subscriptionId}/resources |
| `azure_get_resource` | Get resource details | GET | /subscriptions/.../providers/{resourceProviderNamespace}/{resourceType}/{resourceName} |
| `azure_list_storage_accounts` | List storage accounts | GET | /subscriptions/{subscriptionId}/providers/Microsoft.Storage/storageaccounts |
| `azure_get_storage_account` | Get storage account details | GET | /subscriptions/.../Microsoft.Storage/storageAccounts/{storageAccountName} |
| `azure_list_virtual_machines` | List virtual machines | GET | /subscriptions/.../Microsoft.Compute/virtualMachines |
| `azure_get_virtual_machine` | Get virtual machine details | GET | /subscriptions/.../Microsoft.Compute/virtualMachines/{vmName} |
| `azure_list_containers` | List blob containers | GET | /subscriptions/.../storageAccounts/{storageAccountName}/blobServices/default/containers |

---

## Tool Details

### azure_list_resource_groups

**What it does**: Lists all Azure resource groups in the subscription. Resource groups are containers that hold related resources for an Azure solution.

**When to use**: Organize and discover resources, find resource groups for deployment, understand your Azure infrastructure layout.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `filter` (optional): OData filter expression
- `top` (optional): Number of results to return

**Example LLM prompt**: "List all resource groups in my production subscription"

---

### azure_get_resource_group

**What it does**: Gets details about a specific Azure resource group including its resources and tags.

**When to use**: Understand resource group structure, check deployment status, see what resources are grouped together.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Resource group name

**Example LLM prompt**: "Get details about the 'eastus-web' resource group"

---

### azure_create_resource_group

**What it does**: Creates a new Azure resource group to organize resources and manage permissions.

**When to use**: Set up new infrastructure projects, create environments (dev/staging/prod), organize resources by team or application.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Desired resource group name
- `location` (required): Azure region (e.g., eastus, westus2)
- `tags` (optional): Resource tags as key-value pairs

**Example LLM prompt**: "Create a new resource group called 'ai-projects' in the eastus region"

---

### azure_list_resources

**What it does**: Lists all resources in a subscription or resource group. Use this to discover what resources exist in your Azure environment.

**When to use**: Audit infrastructure, find specific resource types, discover deployed services.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (optional): Filter by resource group name
- `filter` (optional): OData filter expression
- `top` (optional): Number of results to return

**Example LLM prompt**: "List all Azure resources in my subscription"

---

### azure_get_resource

**What it does**: Gets details about a specific Azure resource including provisioning state, tags, and metadata.

**When to use**: Inspect resource configuration, get resource metadata, understand resource status.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Resource group name
- `resourceProviderNamespace` (required): Resource provider namespace (e.g., Microsoft.Compute)
- `resourceType` (required): Resource type (e.g., virtualMachines)
- `resourceName` (required): Resource name
- `apiVersion` (optional): API version of the resource provider

**Example LLM prompt**: "Get details about the 'myapp-prod' virtual machine in the 'web-rg' resource group"

---

### azure_list_storage_accounts

**What it does**: Lists all Azure storage accounts in a subscription or resource group. Storage accounts contain blob, file, table, and queue data.

**When to use**: Discover storage resources, find storage for data operations, audit storage costs.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (optional): Filter by resource group name
- `filter` (optional): OData filter expression

**Example LLM prompt**: "List all storage accounts in my subscription"

---

### azure_get_storage_account

**What it does**: Gets details about a specific Azure storage account including keys, endpoints, and properties.

**When to use**: Get storage connection strings, understand storage configuration, find blob/container endpoints.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Resource group name
- `storageAccountName` (required): Storage account name

**Example LLM prompt**: "Get details about my storage account 'appdata001'"

---

### azure_list_virtual_machines

**What it does**: Lists all virtual machines in a subscription or resource group. Returns VM sizes, statuses, and basic configuration.

**When to use**: Inventory compute resources, find VMs for management, check VM statuses.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Resource group name
- `filter` (optional): OData filter expression
- `expand` (optional): Expand response with additional details (e.g., instanceView)

**Example LLM prompt**: "List all virtual machines in the 'prod-rg' resource group"

---

### azure_get_virtual_machine

**What it does**: Gets detailed information about an Azure virtual machine including status, sizes, and configuration.

**When to use**: Inspect VM configuration, check VM status, get power state and instance details.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Resource group name
- `vmName` (required): Virtual machine name
- `expand` (optional): Expand response with additional details (e.g., instanceView)

**Example LLM prompt**: "Get details about the 'web-server-01' virtual machine"

---

### azure_list_containers

**What it does**: Lists blob containers in an Azure storage account. Containers organize blobs similar to folders in a file system.

**When to use**: Discover blob containers, find containers for file operations, understand blob storage structure.

**Arguments**:
- `subscriptionId` (required): Azure subscription ID
- `resourceGroupName` (required): Resource group name
- `storageAccountName` (required): Storage account name
- `filter` (optional): OData filter expression
- `maxresults` (optional): Maximum number of results to return

**Example LLM prompt**: "List all containers in the 'appstorage001' storage account"

---

## Azure API Notes

- **Resource hierarchy**: Subscription > Resource Group > Resource
- **Resource ID format**: `/subscriptions/{sub}/resourceGroups/{rg}/providers/{provider}/{type}/{name}`
- **Provider namespaces**: Microsoft.Compute (VMs), Microsoft.Web (App Services), Microsoft.Storage (Storage), Microsoft.Sql (SQL), Microsoft.ContainerRegistry (ACR)
- **ARM templates**: For complex deployments, consider using ARM templates instead of individual API calls
- **Regions**: Check Azure documentation for available regions and services per region
- **Blob containers**: Part of Azure Storage; use for storing unstructured data like images, logs, backups
