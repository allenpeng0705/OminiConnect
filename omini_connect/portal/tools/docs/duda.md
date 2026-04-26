# Duda Tools

Provider: `duda` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Duda API. They allow AI agents to manage websites, pages, widgets, and users in a Duda account. Duda is a white-label website builder platform for agencies and SaaS companies.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Duda
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `sites:read`, `sites:write`, `pages:read`, `pages:write`, `widgets:read`, `widgets:write`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `duda_list_sites` | List all sites | GET | /api/sites |
| `duda_get_site` | Get site details | GET | /api/sites/{site_name} |
| `duda_create_site` | Create a new site | POST | /api/sites |
| `duda_list_pages` | List pages in a site | GET | /api/sites/{site_name}/pages |
| `duda_get_page` | Get page details | GET | /api/sites/{site_name}/pages/{page_uuid} |
| `duda_create_page` | Create a new page | POST | /api/sites/{site_name}/pages |
| `duda_list_widgets` | List widgets in a site | GET | /api/sites/{site_name}/widgets |
| `duda_get_widget` | Get widget details | GET | /api/sites/{site_name}/widgets/{widget_uuid} |
| `duda_list_users` | List users in a site | GET | /api/sites/{site_name}/users |
| `duda_get_user` | Get user details | GET | /api/sites/{site_name}/users/{user_id} |

---

## Tool Details

### duda_list_sites

**What it does**: Lists all websites associated with your Duda account.

**When to use**: View all sites, find sites by pagination.

**Arguments**:
- `limit` (optional): Maximum number of sites to return (default 20, max 100)
- `offset` (optional): Number of sites to skip for pagination

**Example LLM prompt**: "List all sites in my Duda account"

---

### duda_get_site

**What it does**: Gets detailed information about a specific website by site name or ID.

**When to use**: Check site settings, view site configuration.

**Arguments**:
- `site_name` (required): Unique identifier of the site

**Example LLM prompt**: "Get details for site mysite"

---

### duda_create_site

**What it does**: Creates a new website in your Duda account with specified template and settings.

**When to use**: Add new client sites, create sites from templates.

**Arguments**:
- `template_id` (required): Template ID to use for the new site
- `site_title` (required): Title for the new site
- `domain` (optional): Custom domain for the site

**Example LLM prompt**: "Create a new site called 'Acme Corp' using template tpl_123"

---

### duda_list_pages

**What it does**: Lists all pages within a specific Duda website.

**When to use**: Browse site structure, find specific pages.

**Arguments**:
- `site_name` (required): Unique identifier of the site

**Example LLM prompt**: "List all pages in site mysite"

---

### duda_get_page

**What it does**: Gets detailed information about a specific page including content and settings.

**When to use**: Get page details, check page configuration.

**Arguments**:
- `site_name` (required): Unique identifier of the site
- `page_uuid` (required): Unique identifier of the page

**Example LLM prompt**: "Get page details for page abc-123 in site mysite"

---

### duda_create_page

**What it does**: Creates a new page within a Duda website.

**When to use**: Add new pages to a site.

**Arguments**:
- `site_name` (required): Unique identifier of the site
- `page_title` (required): Title for the new page
- `page_type` (optional): Type of page to create (e.g., template, blank)

**Example LLM prompt**: "Create a new page called 'Contact' in site mysite"

---

### duda_list_widgets

**What it does**: Lists all widgets available in a Duda website.

**When to use**: Browse widgets, understand site component structure.

**Arguments**:
- `site_name` (required): Unique identifier of the site

**Example LLM prompt**: "List all widgets in site mysite"

---

### duda_get_widget

**What it does**: Gets detailed configuration and content of a specific widget.

**When to use**: Get widget settings, understand widget behavior.

**Arguments**:
- `site_name` (required): Unique identifier of the site
- `widget_uuid` (required): Unique identifier of the widget

**Example LLM prompt**: "Get widget details for widget abc-123 in site mysite"

---

### duda_list_users

**What it does**: Lists all team members and collaborators with access to a Duda site.

**When to use**: View site collaborators, check team access.

**Arguments**:
- `site_name` (required): Unique identifier of the site

**Example LLM prompt**: "List all users with access to site mysite"

---

### duda_get_user

**What it does**: Gets detailed information about a specific team member or collaborator.

**When to use**: Check user permissions, get user profile.

**Arguments**:
- `site_name` (required): Unique identifier of the site
- `user_id` (required): Unique identifier of the user

**Example LLM prompt**: "Get details for user abc-123 in site mysite"

---

## Duda API Notes

- **Site Names**: Duda uses site_name as the unique identifier (not UUIDs)
- **Page/Widget UUIDs**: Pages and widgets use UUID format
- **Rate Limits**: 120 requests/minute standard, HTTP 429 on exceed
- **Templates**: Pre-designed layouts available for new sites
- **Users**: Team members and external collaborators with granular permissions
- **Base URL**: `https://api.duda.co`