# ProvenExpert Tools

Provider: `provenexpert` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

These tools wrap the ProvenExpert API. They allow AI agents to manage ratings, reviews, profile, competitors, statistics, and badges. ProvenExpert is a reputation management platform. **Requires ProvenExpert Basic Authentication (API ID).**

## Authentication

**Nango Basic Auth**:
- Uses API ID as username (no password typically needed)
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://www.provenexpert.com/api/

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `provenexpert_get_rating_summary` | Get rating summary | POST | /v1/rating/summary/get |
| `provenexpert_list_ratings` | List ratings | POST | /v1/rating/get |
| `provenexpert_get_rating` | Get rating details | POST | /v1/rating/get |
| `provenexpert_list_reviews` | List reviews | POST | /v1/reviews/get |
| `provenexpert_get_review` | Get review details | POST | /v1/reviews/get |
| `provenexpert_get_profile` | Get profile info | POST | /v1/profile/get |
| `provenexpert_list_competitors` | List competitors | POST | /v1/competitor/get |
| `provenexpert_get_statistics` | Get statistics | POST | /v1/statistics/get |
| `provenexpert_list_badges` | List badges | POST | /v1/badges/get |
| `provenexpert_get_seals` | Get seals info | POST | /v1/seals/get |

---

## Tool Details

### provenexpert_get_rating_summary

**What it does**: Gets overall rating summary.

**When to use**: Get quick overview of ratings.

**Arguments**: None

**Example LLM prompt**: "Get our rating summary"

---

### provenexpert_list_ratings

**What it does**: Lists all ratings.

**When to use**: Browse rating history.

**Arguments**:
- `startDate` (optional): Start date filter
- `endDate` (optional): End date filter

**Example LLM prompt**: "List ratings from this month"

---

### provenexpert_get_rating

**What it does**: Gets detailed information about a specific rating.

**When to use**: Get rating breakdown.

**Arguments**:
- `ratingId` (required): Rating ID

**Example LLM prompt**: "Get details for rating 12345"

---

### provenexpert_list_reviews

**What it does**: Lists all reviews.

**When to use**: Browse customer reviews.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all reviews"

---

### provenexpert_get_review

**What it does**: Gets detailed information about a specific review.

**When to use**: Get review content.

**Arguments**:
- `reviewId` (required): Review ID

**Example LLM prompt**: "Get details for review 67890"

---

### provenexpert_get_profile

**What it does**: Gets profile information.

**When to use**: Get company profile.

**Arguments**: None

**Example LLM prompt**: "Get our profile"

---

### provenexpert_list_competitors

**What it does**: Lists competitor information.

**When to use**: Market analysis.

**Arguments**: None

**Example LLM prompt**: "Who are our competitors?"

---

### provenexpert_get_statistics

**What it does**: Gets statistics information.

**When to use**: Analytics overview.

**Arguments**: None

**Example LLM prompt**: "Get our statistics"

---

### provenexpert_list_badges

**What it does**: Lists all badges.

**When to use**: View earned badges.

**Arguments**: None

**Example LLM prompt**: "What badges have we earned?"

---

### provenexpert_get_seals

**What it does**: Gets seal information for embedding.

**When to use**: Get seal codes for website.

**Arguments**: None

**Example LLM prompt**: "Get seal codes for our website"

---

## ProvenExpert API Notes

- **Basic Auth**: Uses API ID as username
- **Reputation Management**: Ratings and reviews
- **Rate limits**: Apply to API calls
