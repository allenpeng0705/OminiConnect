# Google Search Console Tools

Provider: `google-search-console` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Search Console API. They allow AI agents to manage sites, sitemaps, search analytics, and URL inspection. **Requires Google OAuth2 with Search Console permissions.**

## Authentication

**Nango OAUTH2 (Google Search Console)**:
- User authenticates via OAuth2 with Webmasters scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://searchconsole.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_search_console_list_sites` | List sites | GET | /webmasters/v3/sites |
| `google_search_console_add_site` | Add site | PUT | /webmasters/v3/sites/{site_url} |
| `google_search_console_delete_site` | Remove site | DELETE | /webmasters/v3/sites/{site_url} |
| `google_search_console_get_site` | Get site details | GET | /webmasters/v3/sites/{site_url} |
| `google_search_console_list_sitemaps` | List sitemaps | GET | /webmasters/v3/sites/{site_url}/sitemaps |
| `google_search_console_submit_sitemap` | Submit sitemap | PUT | /webmasters/v3/sites/{site_url}/sitemaps |
| `google_search_console_search_analytics` | Search analytics | POST | /webmasters/v3/sites/{site_url}/searchAnalytics/query |
| `google_search_console_url_inspection` | URL inspection | POST | /v1/urlInspection/index |
| `google_search_console_list_urlanan` | List URL annotations | GET | /v1/urlAnnotations |
| `google_search_console_create_urlanan` | Create URL annotation | POST | /v1/urlAnnotations |

---

## Tool Details

### google_search_console_list_sites

**What it does**: Lists sites in Search Console.

**When to use**: See all verified sites.

**Arguments**: None

**Example LLM prompt**: "List all my sites in Search Console"

---

### google_search_console_add_site

**What it does**: Adds a site to Search Console.

**When to use**: Add new property for tracking.

**Arguments**:
- `site_url` (required): Site URL

**Example LLM prompt**: "Add https://example.com to Search Console"

---

### google_search_console_delete_site

**What it does**: Removes a site from Search Console.

**When to use**: Remove unwanted property.

**Arguments**:
- `site_url` (required): Site URL

**Example LLM prompt**: "Remove https://example.com from Search Console"

---

### google_search_console_get_site

**What it does**: Gets site details.

**When to use**: Get site permission level.

**Arguments**:
- `site_url` (required): Site URL

**Example LLM prompt**: "Get details for https://example.com"

---

### google_search_console_list_sitemaps

**What it does**: Lists sitemaps for a site.

**When to use**: See submitted sitemaps.

**Arguments**:
- `site_url` (required): Site URL

**Example LLM prompt**: "List sitemaps for https://example.com"

---

### google_search_console_submit_sitemap

**What it does**: Submits a sitemap to Search Console.

**When to use**: Tell Google about new pages.

**Arguments**:
- `site_url` (required): Site URL
- `feedpath` (required): Sitemap path

**Example LLM prompt**: "Submit sitemap.xml for https://example.com"

---

### google_search_console_search_analytics

**What it does**: Gets search analytics data.

**When to use**: See search performance.

**Arguments**:
- `site_url` (required): Site URL
- `start_date` (required): Start date (YYYY-MM-DD)
- `end_date` (required): End date (YYYY-MM-DD)
- `dimensions` (optional): Dimensions (query, page, country, device)

**Example LLM prompt**: "Get search analytics for https://example.com last 30 days"

---

### google_search_console_url_inspection

**What it does**: Inspects a URL in Search Console.

**When to use**: Check URL indexing status.

**Arguments**:
- `site_url` (required): Site URL
- `inspection_url` (required): URL to inspect

**Example LLM prompt**: "Inspect https://example.com/page1"

---

### google_search_console_list_urlanan

**What it does**: Lists URL annotations.

**When to use**: See custom annotations.

**Arguments**: None

**Example LLM prompt**: "List URL annotations"

---

### google_search_console_create_urlanan

**What it does**: Creates a URL annotation.

**When to use**: Add custom tracking.

**Arguments**:
- `url` (required): URL
- `annotation` (required): Annotation data

**Example LLM prompt**: "Create annotation for https://example.com"

---

## Search Console API Notes

- **Site URL**: Must be verified in Search Console
- **Sitemaps**: Help Google discover content
- **Search analytics**: Shows how people find your site
- **URL inspection**: Check live indexing status
- **Date format**: YYYY-MM-DD
