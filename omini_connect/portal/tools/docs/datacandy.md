# DataCandy Tools

Provider: `datacandy` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the DataCandy API. They allow AI agents to manage customers, loyalty rewards, gift cards, and marketing campaigns. DataCandy is a customer loyalty and engagement platform.

## Authentication

**Nango BASIC**:
- User provides application_key and access_key via Nango connection config
- Tokens stored in Nango, accessed via `connection_ref`
- Query parameters passed with each request

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `datacandy_list_customers` | List customers | GET | /merchant |
| `datacandy_get_customer` | Get customer details | GET | /merchant/{customer_id} |
| `datacandy_create_customer` | Create a customer | POST | /merchant |
| `datacandy_update_customer` | Update customer details | PUT | /merchant/{customer_id} |
| `datacandy_list_loyalty_rewards` | List loyalty rewards | GET | /loyalty |
| `datacandy_get_loyalty_reward` | Get reward details | GET | /loyalty/{reward_id} |
| `datacandy_list_gift_cards` | List gift cards | GET | /giftcard |
| `datacandy_get_gift_card` | Get gift card details | GET | /giftcard/{card_id} |
| `datacandy_list_campaigns` | List campaigns | GET | /campaigns |
| `datacandy_get_campaign` | Get campaign details | GET | /campaigns/{campaign_id} |

---

## Tool Details

### datacandy_list_customers

**What it does**: Lists all customers with pagination.

**When to use**: Browse customer database, find customers, manage customer records.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all customers"

---

### datacandy_get_customer

**What it does**: Gets detailed customer information including contact and loyalty status.

**When to use**: Review customer profile, check loyalty points, verify customer data.

**Arguments**:
- `customer_id` (required): Customer ID

**Example LLM prompt**: "Get details for customer c-123"

---

### datacandy_create_customer

**What it does**: Creates a new customer record.

**When to use**: Add new customers, register loyalty members, create customer profiles.

**Arguments**:
- `email` (required): Customer email
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Create a new customer with email john@example.com"

---

### datacandy_update_customer

**What it does**: Updates customer information.

**When to use**: Modify customer details, update contact information, change preferences.

**Arguments**:
- `customer_id` (required): Customer ID
- `email` (optional): Customer email
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `phone` (optional): Phone number

**Example LLM prompt**: "Update customer c-123 with new phone number"

---

### datacandy_list_loyalty_rewards

**What it does**: Lists loyalty rewards and points, optionally filtered by customer.

**When to use**: Check loyalty balances, view reward history, track points.

**Arguments**:
- `customer_id` (optional): Filter by customer ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all loyalty rewards for customer c-123"

---

### datacandy_get_loyalty_reward

**What it does**: Gets detailed loyalty reward information.

**When to use**: Review specific reward details, check reward eligibility.

**Arguments**:
- `reward_id` (required): Reward ID

**Example LLM prompt**: "Get details for reward r-456"

---

### datacandy_list_gift_cards

**What it does**: Lists all gift cards.

**When to use**: View gift card inventory, check card balances, track gift card usage.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all gift cards"

---

### datacandy_get_gift_card

**What it does**: Gets detailed gift card information including balance.

**When to use**: Check gift card balance, verify card status, view transaction history.

**Arguments**:
- `card_id` (required): Gift Card ID

**Example LLM prompt**: "Get details for gift card gc-789"

---

### datacandy_list_campaigns

**What it does**: Lists all marketing campaigns.

**When to use**: Browse campaigns, track campaign performance, find active promotions.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all campaigns"

---

### datacandy_get_campaign

**What it does**: Gets detailed campaign information including metrics.

**When to use**: Review campaign performance, check campaign settings, analyze results.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign cmp-101"

---

## DataCandy API Notes

- **Customer Loyalty**: Platform for managing loyalty programs and customer engagement
- **Customers**: Customer profiles with contact and preference data
- **Loyalty Rewards**: Points and rewards system for customer engagement
- **Gift Cards**: Stored value cards for purchases
- **Marketing Campaigns**: Promotional and marketing initiatives
- **Authentication**: Uses application_key and access_key in query parameters
