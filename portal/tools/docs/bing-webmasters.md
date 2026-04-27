# Bing Webmasters Tools

Provider: `bing-webmasters` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Bing Webmasters API. They allow AI agents to manage site verification, submit URLs for indexing, view keyword statistics, and monitor SEO performance. Bing Webmasters provides insights into how Bing crawls and indexes your site.

## Authentication

**Nango OAuth2**:
- User authenticates via Bing Webmasters OAuth
- Token stored in Nango, accessed via `connection_ref`
- Scopes for webmaster access

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bing_get_site_info` | Get site info | GET | /api/sites/{siteUrl} |
| `bing_list_sites` | List verified sites | GET | /api/sites |
| `bing_add_url` | Submit a URL for indexing | POST | /api/url |
| `bing_get_crawl_info` | Get crawl information | GET | /api/crawlinfo/{siteUrl} |
| `bing_list_keywords` | List keywords | GET | /api/keywords/{siteUrl} |
| `bing_get_keyword_stats` | Get keyword statistics | GET | /api/keywords/{siteUrl}/stats |
| `bing_list_pages` | List indexed pages | GET | /api/pages/{siteUrl} |
| `bing_get_page_stats` | Get page statistics | GET | /api/pages/{siteUrl}/{pageUrl} |
| `bing_list_sitemaps` | List sitemaps | GET | /api/sitemaps/{siteUrl} |
| `bing_submit_sitemap` | Submit a sitemap | POST | /api/sitemaps/{siteUrl} |

---

## Tool Details

### bing_get_site_info

**What it does**: Gets information about a verified site.

**When to use**: Check site status, crawl settings.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)

**Example LLM prompt**: "Get info for https://example.com"

---

### bing_list_sites

**What it does**: Lists all sites verified in Bing Webmasters.

**When to use**: View all registered sites.

**Arguments**: None required

**Example LLM prompt**: "List all my verified sites"

---

### bing_add_url

**What it does**: Submits a URL for indexing in Bing.

**When to use**: Request indexing for new or updated pages.

**Arguments**:
- `siteUrl` (required): Site URL
- `url` (required): URL to submit

**Example LLM prompt**: "Submit URL https://example.com/new-page for indexing"

---

### bing_get_crawl_info

**What it does**: Gets crawl information for a site.

**When to use**: Check how Bing crawls your site.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)

**Example LLM prompt**: "Get crawl info for https://example.com"

---

### bing_list_keywords

**What it does**: Lists keywords for a site with traffic data.

**When to use**: View search queries driving traffic.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Keywords per page (default 20)

**Example LLM prompt**: "List keywords for https://example.com"

---

### bing_get_keyword_stats

**What it does**: Gets detailed statistics for a keyword.

**When to use**: Analyze keyword performance.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)
- `keyword` (required): Keyword to get stats for

**Example LLM prompt**: "Get stats for keyword 'coffee shop' on example.com"

---

### bing_list_pages

**What it does**: Lists indexed pages for a site.

**When to use**: Check which pages Bing has indexed.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Pages per page (default 20)

**Example LLM prompt**: "List indexed pages for https://example.com"

---

### bing_get_page_stats

**What it does**: Gets statistics for a specific page.

**When to use**: Check page-level SEO metrics.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)
- `pageUrl` (required): Page URL (URL-encoded)

**Example LLM prompt**: "Get stats for https://example.com/about"

---

### bing_list_sitemaps

**What it does**: Lists all sitemaps for a site.

**When to use**: View submitted sitemaps.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)

**Example LLM prompt**: "List sitemaps for https://example.com"

---

### bing_submit_sitemap

**What it does**: Submits a sitemap for a site.

**When to use**: Register new sitemaps, update search index.

**Arguments**:
- `siteUrl` (required): Site URL (URL-encoded)
- `sitemapUrl` (required): Sitemap URL

**Example LLM prompt**: "Submit sitemap https://example.com/sitemap.xml"

---

## Bing Webmasters API Notes

- **Site Verification**: Sites must be verified before using API
- **URL Encoding**: Site and page URLs must be URL-encoded
- **Rate Limits**: API has rate limiting; implement backoff
- **Indexing**: Submitting URL requests indexing but doesn't guarantee it
- **Keywords**: Data based on Bing's search results, not real-time
