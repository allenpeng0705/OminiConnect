# Google Play Tools

Provider: `google-play` | Engine: `nango` | Auth: OAUTH2 via Nango

## Overview

These tools wrap the Google Play Developer API. They allow AI agents to manage apps, reviews, subscriptions, orders, and refunds. **Requires Google OAuth2 with Play Developer permissions.**

## Authentication

**Nango OAUTH2 (Google Play)**:
- User authenticates via OAuth2 with Android Publisher scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://play.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_play_list_apps` | List apps | GET | /androidpublisher/v3/applications |
| `google_play_get_app` | Get app details | GET | /androidpublisher/v3/applications/{package_name} |
| `google_play_list_reviews` | List reviews | GET | /androidpublisher/v3/applications/{package_name}/reviews |
| `google_play_get_review` | Get review details | GET | /androidpublisher/v3/applications/{package_name}/reviews/{review_id} |
| `google_play_list_subscriptions` | List subscriptions | GET | /androidpublisher/v3/applications/{package_name}/subscriptions |
| `google_play_get_subscription` | Get subscription details | GET | /androidpublisher/v3/applications/{package_name}/subscriptions/{subscription_id} |
| `google_play_list_orders` | List orders | GET | /androidpublisher/v3/applications/{package_name}/orders |
| `google_play_get_order` | Get order details | GET | /androidpublisher/v3/applications/{package_name}/orders/{order_id} |
| `google_play_list_refunds` | List refunds | GET | /androidpublisher/v3/applications/{package_name}/refunds |
| `google_play_list_reviews_response` | List review responses | GET | /androidpublisher/v3/applications/{package_name}/reviews/{review_id}/responses |

---

## Tool Details

### google_play_list_apps

**What it does**: Lists apps in Google Play Console.

**When to use**: Browse your apps.

**Arguments**: None

**Example LLM prompt**: "List all my apps in Play Console"

---

### google_play_get_app

**What it does**: Gets details about an app.

**When to use**: Get app info and stats.

**Arguments**:
- `package_name` (required): App package name

**Example LLM prompt**: "Get details for com.example.myapp"

---

### google_play_list_reviews

**What it does**: Lists reviews for an app.

**When to use**: Monitor user feedback.

**Arguments**:
- `package_name` (required): App package name

**Example LLM prompt**: "List reviews for com.example.myapp"

---

### google_play_get_review

**What it does**: Gets details about a review.

**When to use**: Read full review content.

**Arguments**:
- `package_name` (required): App package name
- `review_id` (required): Review ID

**Example LLM prompt**: "Get review abc123 for com.example.myapp"

---

### google_play_list_subscriptions

**What it does**: Lists subscriptions for an app.

**When to use**: View subscription products.

**Arguments**:
- `package_name` (required): App package name

**Example LLM prompt**: "List subscriptions for com.example.myapp"

---

### google_play_get_subscription

**What it does**: Gets subscription details.

**When to use**: Get subscription settings.

**Arguments**:
- `package_name` (required): App package name
- `subscription_id` (required): Subscription ID

**Example LLM prompt**: "Get subscription basic for com.example.myapp"

---

### google_play_list_orders

**What it does**: Lists orders for an app.

**When to use**: View purchase history.

**Arguments**:
- `package_name` (required): App package name

**Example LLM prompt**: "List orders for com.example.myapp"

---

### google_play_get_order

**What it does**: Gets order details.

**When to use**: Get purchase info.

**Arguments**:
- `package_name` (required): App package name
- `order_id` (required): Order ID

**Example LLM prompt**: "Get order GPA.1234-5678-9012"

---

### google_play_list_refunds

**What it does**: Lists refunds for an app.

**When to use**: Track refund requests.

**Arguments**:
- `package_name` (required): App package name

**Example LLM prompt**: "List refunds for com.example.myapp"

---

### google_play_list_reviews_response

**What it does**: Lists responses to reviews.

**When to use**: See replied reviews.

**Arguments**:
- `package_name` (required): App package name
- `review_id` (required): Review ID

**Example LLM prompt**: "List responses for review abc123"

---

## Google Play API Notes

- **Package name**: Unique app identifier (e.g., com.example.myapp)
- **Review ID**: Unique identifier for reviews
- **Subscriptions**: In-app subscription products
- **Orders**: Purchase order IDs
- **Refunds**: Requested refunds
