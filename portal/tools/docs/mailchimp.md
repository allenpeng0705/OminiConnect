# Mailchimp Tools

Provider: `mailchimp` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Mailchimp API. They allow AI agents to manage audiences (lists), members, campaigns, and automations. Mailchimp is a popular email marketing platform for businesses of all sizes.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Mailchimp
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `lists`, `campaigns`, `automation`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mailchimp_list_lists` | List all audiences | GET | /3.0/lists |
| `mailchimp_get_list` | Get audience details | GET | /3.0/lists/{list_id} |
| `mailchimp_create_list` | Create a new audience | POST | /3.0/lists |
| `mailchimp_list_members` | List members in an audience | GET | /3.0/lists/{list_id}/members |
| `mailchimp_get_member` | Get member details | GET | /3.0/lists/{list_id}/members/{subscriber_hash} |
| `mailchimp_add_member` | Add a member to an audience | POST | /3.0/lists/{list_id}/members |
| `mailchimp_list_campaigns` | List all campaigns | GET | /3.0/campaigns |
| `mailchimp_get_campaign` | Get campaign details | GET | /3.0/campaigns/{campaign_id} |
| `mailchimp_create_campaign` | Create a new campaign | POST | /3.0/campaigns |
| `mailchimp_list_automations` | List automations | GET | /3.0/automations |

---

## Tool Details

### mailchimp_list_lists

**What it does**: Gets all audiences (formerly called lists) in your Mailchimp account. Audiences contain your subscribers and are used for sending campaigns.

**When to use**: See available audiences, check audience sizes, or prepare to add members or send campaigns.

**Arguments**:
- `count` (optional): Number of results per page (max 100, default 10)
- `offset` (optional): Index offset for pagination
- `before_date_created` (optional): Filter lists created before this date
- `after_date_created` (optional): Filter lists created after this date

**Example LLM prompt**: "List all our Mailchimp audiences"

---

### mailchimp_get_list

**What it does**: Gets details about a specific audience including stats, settings, and campaign defaults.

**When to use**: Check audience details, view member count, or review campaign defaults before sending.

**Arguments**:
- `list_id` (required): Audience ID

**Example LLM prompt**: "Get details for audience abc123"

---

### mailchimp_create_list

**What it does**: Creates a new audience in your Mailchimp account with contact details and campaign defaults.

**When to use**: Set up new audiences for different customer segments or newsletter types.

**Arguments**:
- `name` (required): Audience name
- `contact` (optional): Company contact info (company, address1, city, state, zip, country)
- `permission_reminder` (optional): How contacts gave permission
- `campaign_defaults` (optional): Default values for campaigns (from_name, from_email, subject, language)

**Example LLM prompt**: "Create a new audience called 'Product Updates' with company info and default sender notifications@example.com"

---

### mailchimp_list_members

**What it does**: Gets all members of a specific audience with their subscriber status, engagement metrics, and tags.

**When to use**: View audience members, check subscriber status, or prepare bulk operations.

**Arguments**:
- `list_id` (required): Audience ID
- `status` (optional): Filter by status: subscribed, unsubscribed, cleaned, pending, transactional
- `count` (optional): Number of results per page (max 100, default 10)
- `offset` (optional): Index offset for pagination

**Example LLM prompt**: "List the first 50 subscribed members in audience abc123"

---

### mailchimp_get_member

**What it does**: Gets details about a specific audience member including contact info, status, tags, and activity history.

**When to use**: Look up individual subscriber details before updating, removing, or personalizing messages.

**Arguments**:
- `list_id` (required): Audience ID
- `subscriber_hash` (required): MD5 hash of lowercase email address (subscriber hash)

**Example LLM prompt**: "Get member details for user@example.com in audience abc123"

---

### mailchimp_add_member

**What it does**: Adds a new member to an audience. Can optionally send a confirmation email for double opt-in.

**When to use**: Sign up new subscribers, add contacts from sign-up forms, or import contacts.

**Arguments**:
- `list_id` (required): Audience ID
- `email_address` (required): Member email address
- `status` (optional): Subscription status: subscribed, pending, unsubscribed (default pending)
- `merge_fields` (optional): Additional merge field data
- `tags` (optional): Tags to apply to the member

**Example LLM prompt**: "Add member john@example.com to audience abc123 with first name John and tag 'newsletter'"

---

### mailchimp_list_campaigns

**What it does**: Gets all campaigns in your Mailchimp account including sent, scheduled, and draft campaigns.

**When to use**: Review campaign history, check campaign statuses, or prepare new campaigns.

**Arguments**:
- `status` (optional): Filter by status: save, paused, schedule, sending, sent
- `count` (optional): Number of results per page (default 10)
- `offset` (optional): Index offset for pagination

**Example LLM prompt**: "List all sent campaigns from the last month"

---

### mailchimp_get_campaign

**What it does**: Gets details about a specific campaign including content, recipients, send time, and tracking metrics.

**When to use**: Check campaign performance, review settings, or see why a campaign performed certain way.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign abc123"

---

### mailchimp_create_campaign

**What it does**: Creates a new campaign in Mailchimp with recipients, subject, from info, and content settings.

**When to use**: Set up newsletters, promotional emails, or any marketing campaign before sending.

**Arguments**:
- `type` (required): Campaign type: regular, plaintext, absplit, rss, variate (default regular)
- `recipients` (optional): Recipient settings (list_id, segment_opts)
- `settings` (optional): Campaign settings (subject_line, preview_text, title, from_name, reply_to)

**Example LLM prompt**: "Create a new regular campaign with subject 'Weekly Update' from 'My Company <hello@example.com>'"

---

### mailchimp_list_automations

**What it does**: Gets all automations in your Mailchimp account. Automations are triggered email sequences like welcome series or abandoned cart emails.

**When to use**: Browse existing automations, see what automated emails are running, or prepare to create new flows.

**Arguments**:
- `count` (optional): Number of results per page (default 10)
- `offset` (optional): Index offset for pagination
- `status` (optional): Filter by status: paused, unpaused, send

**Example LLM prompt**: "List all running automations in our Mailchimp account"

---

## Mailchimp API Notes

- **Audiences vs Lists**: Mailchimp rebranded "lists" to "audiences" but the API still uses `/lists`
- **Subscriber Hash**: MD5 hash of lowercase email used to identify members in API URLs
- **Double Opt-in**: When enabled, new subscribers receive a confirmation email before being added
- **Merge Fields**: Custom data fields like FNAME, LNAME for personalization
- **Status Values**: subscribed (active), unsubscribed (opted out), pending (awaiting confirmation), cleaned (removed due to bounces/complaints)
- **Automation Triggers**: Welcome series, birthday emails, abandoned cart, post-purchase, etc.
