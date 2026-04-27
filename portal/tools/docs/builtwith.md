# BuiltWith Tools

Provider: `builtwith` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the BuiltWith API. They allow AI agents to lookup website technologies, analyze tech stacks, and get market intelligence. BuiltWith is a technology intelligence platform that shows what websites are built with.

## Authentication

**Nango API_KEY**:
- User provides BuiltWith API key
- Token stored in Nango, accessed via `connection_ref`
- API key passed as query parameter

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `builtwith_lookup_domain` | Lookup domain | GET | /v2/lookup.json |
| `builtwith_get_trends` | Get technology trends | GET | /v2/trends.json |
| `builtwith_list_domains` | List domains using technology | GET | /v2/domain-list.json |
| `builtwith_get_domain_info` | Get detailed domain info | GET | /v2/domain-info.json |
| `builtwith_list_technologies` | List technologies used by domain | GET | /v2/technologies.json |
| `builtwith_get_technology_info` | Get technology details | GET | /v2/technology.json |
| `builtwith_list_categories` | List technology categories | GET | /v2/categories.json |
| `builtwith_get_category_info` | Get category details | GET | /v2/category.json |
| `builtwith_search_domains` | Search domains by technology | GET | /v2/search.json |
| `builtwith_get_usage_stats` | Get usage statistics | GET | /v2/usage-stats.json |

---

## Tool Details

### builtwith_lookup_domain

**What it does**: Looks up a domain to get technology usage information.

**When to use**: Discover what technologies a website uses.

**Arguments**:
- `domain` (required): Domain to lookup (e.g., example.com)

**Example LLM prompt**: "What technologies does example.com use"

---

### builtwith_get_trends

**What it does**: Gets technology trends data.

**When to use**: Analyze market trends, popular technologies.

**Arguments**:
- `category` (optional): Technology category
- `date` (optional): Date for trends (YYYY-MM)

**Example LLM prompt**: "Show me trends for JavaScript frameworks"

---

### builtwith_list_domains

**What it does**: Lists domains using a specific technology.

**When to use**: Find websites using a specific tool.

**Arguments**:
- `technology` (required): Technology name
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List domains using React"

---

### builtwith_get_domain_info

**What it does**: Gets detailed information about a domain.

**When to use**: Deep dive into a website's tech stack.

**Arguments**:
- `domain` (required): Domain name

**Example LLM prompt**: "Get detailed info for example.com"

---

### builtwith_list_technologies

**What it does**: Lists all technologies used by a domain.

**When to use**: View complete technology profile.

**Arguments**:
- `domain` (required): Domain to check

**Example LLM prompt**: "List all technologies on shopify.com"

---

### builtwith_get_technology_info

**What it does**: Gets details about a specific technology.

**When to use**: Learn about a technology, market share.

**Arguments**:
- `technology` (required): Technology name or ID

**Example LLM prompt**: "Get info about Shopify"

---

### builtwith_list_categories

**What it does**: Lists all technology categories.

**When to use**: Browse technology categories.

**Arguments**: None required

**Example LLM prompt**: "List all technology categories"

---

### builtwith_get_category_info

**What it does**: Gets information about a technology category.

**When to use**: Explore category details, related tech.

**Arguments**:
- `category` (required): Category name or ID

**Example LLM prompt**: "Get info about Ecommerce category"

---

### builtwith_search_domains

**What it does**: Searches domains by technology criteria.

**When to use**: Find websites matching criteria.

**Arguments**:
- `query` (required): Search query
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "Search for sites using WordPress and WooCommerce"

---

### builtwith_get_usage_stats

**What it does**: Gets usage statistics for technologies.

**When to use**: Check technology popularity, market share.

**Arguments**:
- `technology` (required): Technology name

**Example LLM prompt**: "Get usage stats for Shopify"

---

## BuiltWith API Notes

- **Domain Lookup**: Returns all detected technologies
- **Technology Categories**: Ecommerce, CMS, Analytics, etc.
- **Trends**: Historical adoption data
- **Usage Stats**: Market share information
- **API Limits**: Rate limiting applies; check plan limits
