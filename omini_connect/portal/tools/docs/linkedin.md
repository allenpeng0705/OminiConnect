# LinkedIn Tools

Provider: `linkedin` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the LinkedIn API. They allow AI agents to interact with LinkedIn profiles, connections, companies, jobs, and content on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `profile`, `network`, `email`, etc.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `linkedin_get_profile` | Get authenticated user's profile | GET | /me |
| `linkedin_list_connections` | List first-degree connections | GET | /connections |
| `linkedin_get_company` | Get company page details | GET | /companies/{id} |
| `linkedin_list_posts` | List posts from a profile | GET | /ugcPosts |
| `linkedin_create_post` | Create a text post | POST | /ugcPosts |
| `linkedin_get_job` | Get job details | GET | /jobs/{id} |
| `linkedin_list_jobs` | Search for jobs | GET | /jobSearch |
| `linkedin_search_people` | Search for LinkedIn users | GET | /people |
| `linkedin_get_user_updates` | Get profile updates and activities | GET | /networkUpdates |
| `linkedin_share_post` | Share an article or link | POST | /ugcPosts |

---

## Tool Details

### linkedin_get_profile

**What it does**: Returns the authenticated user's LinkedIn profile including name, headline, summary, and public profile URL.

**When to use**: Get context about the current user before personalizing actions or gathering profile information.

**Arguments**: None required

**Example LLM prompt**: "Get my LinkedIn profile"

---

### linkedin_list_connections

**What it does**: Lists the authenticated user's first-degree connections with name, headline, and profile URL.

**When to use**: Find connections to reach out to, or check if someone is already connected.

**Arguments**:
- `count` (optional): Number of connections to return (default 10, max 500)
- `start` (optional): Start offset for pagination

**Example LLM prompt**: "Show me my first 20 LinkedIn connections"

---

### linkedin_get_company

**What it does**: Get details about a LinkedIn company page including name, description, logo, and employee count.

**When to use**: Research a company before applying, networking, or posting as an organization.

**Arguments**:
- `id` (required): Company ID or universal name

**Example LLM prompt**: "Get details for the Acme Corp company page on LinkedIn"

---

### linkedin_list_posts

**What it does**: Lists posts from the authenticated user's profile or an organization. Use this to see recent activity or content posted.

**When to use**: Review past posts before creating new content, or check an organization's activity.

**Arguments**:
- `author` (optional): URN of the author (person or organization)
- `count` (optional): Number of posts to return (default 10, max 50)
- `start` (optional): Start offset for pagination

**Example LLM prompt**: "List the last 10 posts on my LinkedIn profile"

---

### linkedin_create_post

**What it does**: Creates a new text post on LinkedIn from the authenticated user's profile.

**When to use**: Share thoughts, updates, announcements, or professional insights.

**Arguments**:
- `author` (required): URN of the author (person or organization)
- `text` (required): Post text content
- `visibility` (optional): PUBLIC or CONNECTIONS (default: PUBLIC)

**Example LLM prompt**: "Create a LinkedIn post saying I'm excited to announce our new product launch"

---

### linkedin_get_job

**What it does**: Get details about a specific LinkedIn job posting including title, company, description, and requirements.

**When to use**: Research a job posting in detail before applying or sharing with someone.

**Arguments**:
- `id` (required): Job posting ID

**Example LLM prompt**: "Get details for LinkedIn job ID 123456"

---

### linkedin_list_jobs

**What it does**: Search for jobs on LinkedIn. Filter by keywords, location, company, or job function.

**When to use**: Find relevant job opportunities based on search criteria.

**Arguments**:
- `keywords` (optional): Search keywords
- `location` (optional): Location filter
- `company` (optional): Company name filter
- `count` (optional): Number of results (default 10, max 100)
- `start` (optional): Start offset

**Example LLM prompt**: "Find software engineering jobs in San Francisco at tech companies"

---

### linkedin_search_people

**What it does**: Search for LinkedIn users by keywords, location, industry, or company.

**When to use**: Find potential connections at a specific company, recruiters, or experts in a field.

**Arguments**:
- `keywords` (optional): Search keywords (name, title, company)
- `location` (optional): Location filter
- `industry` (optional): Industry code
- `company` (optional): Company name
- `count` (optional): Number of results (default 10, max 100)

**Example LLM prompt**: "Find product managers at Google in the Bay Area"

---

### linkedin_get_user_updates

**What it does**: Get profile updates and activities for the authenticated user or their connections.

**When to use**: Stay informed about network activity or track engagement on shared content.

**Arguments**:
- `type` (optional): Update type (STATUS, CONNECTION, etc.)
- `count` (optional): Number of updates (default 10, max 50)
- `start` (optional): Start offset

**Example LLM prompt**: "Show me recent updates from my LinkedIn network"

---

### linkedin_share_post

**What it does**: Share an article or link on LinkedIn with a commentary text.

**When to use**: Share external content, blog posts, articles, or news with your network.

**Arguments**:
- `author` (required): URN of the author
- `text` (optional): Commentary text
- `article_url` (required): URL of the article or link to share
- `title` (optional): Title of the shared content
- `visibility` (optional): PUBLIC or CONNECTIONS (default: PUBLIC)

**Example LLM prompt**: "Share this article about AI trends on LinkedIn with a comment about its implications"

---

## LinkedIn API Reference

These tools use the LinkedIn API. See official docs for full details:
- https://learn.microsoft.com/en-us/linkedin/
- Rate limits: Vary by API endpoint and access level
- Pagination: Use `count` and `start` parameters
- All dates: ISO 8601 format

## Notes

- LinkedIn API access requires approval for certain scopes
- Some endpoints require a LinkedIn Company Page or organization setup
- Job search requires specific permissions
