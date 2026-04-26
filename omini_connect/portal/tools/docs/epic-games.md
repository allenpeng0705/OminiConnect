# Epic Games Tools

Provider: `epic-games` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Epic Games API. They allow AI agents to access game catalog, player profiles, achievements, leaderboards, and tournaments. Epic Games is a major gaming platform and developer.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Epic Games
- Token stored in Nango, accessed via `connection_ref`
- Authorization header used for token transmission

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `epic_games_get_catalog` | Get catalog items | GET | /api/v1/catalog |
| `epic_games_list_offers` | List available offers | GET | /api/v1/offers |
| `epic_games_get_offer` | Get offer details | GET | /api/v1/offers/{id} |
| `epic_games_get_player` | Get player info | GET | /api/v1/player |
| `epic_games_list_entitlements` | List player entitlements | GET | /api/v1/entitlements |
| `epic_games_get_achievements` | Get player achievements | GET | /api/v1/achievements |
| `epic_games_list_leaderboards` | List leaderboards | GET | /api/v1/leaderboards |
| `epic_games_get_leaderboard` | Get leaderboard entries | GET | /api/v1/leaderboards/{id} |
| `epic_games_list_news` | List news items | GET | /api/v1/news |
| `epic_games_get_tournament` | Get tournament details | GET | /api/v1/tournaments/{id} |

---

## Tool Details

### epic_games_get_catalog

**What it does**: Gets the game catalog with available items.

**When to use**: Browse available games, find product IDs.

**Arguments**:
- `country` (optional): Country code for localization (default US)

**Example LLM prompt**: "Get the Epic Games catalog for US"

---

### epic_games_list_offers

**What it does**: Lists available offers in the store.

**When to use**: Find current deals, check prices.

**Arguments**:
- `country` (optional): Country code (default US)

**Example LLM prompt**: "List current offers in the Epic store"

---

### epic_games_get_offer

**What it does**: Gets details of a specific offer.

**When to use**: Get offer pricing, check availability.

**Arguments**:
- `id` (required): Offer ID

**Example LLM prompt**: "Get details for offer abc123"

---

### epic_games_get_player

**What it does**: Gets player profile information.

**When to use**: Get current user info, verify authentication.

**Arguments**: None

**Example LLM prompt**: "Get my Epic Games profile"

---

### epic_games_list_entitlements

**What it does**: Lists player entitlements and owned items.

**When to use**: Check owned games, verify purchases.

**Arguments**:
- `epic_id` (optional): Player Epic ID

**Example LLM prompt**: "List my owned games"

---

### epic_games_get_achievements

**What it does**: Gets player achievements and progress.

**When to use**: Check gaming achievements, track progress.

**Arguments**:
- `epic_id` (optional): Player Epic ID

**Example LLM prompt**: "Get my achievement progress"

---

### epic_games_list_leaderboards

**What it does**: Lists available leaderboards.

**When to use**: Find leaderboard IDs, see competitive modes.

**Arguments**: None

**Example LLM prompt**: "List available leaderboards"

---

### epic_games_get_leaderboard

**What it does**: Gets leaderboard entries and rankings.

**When to use**: Check rankings, compare scores.

**Arguments**:
- `id` (required): Leaderboard ID

**Example LLM prompt**: "Get the top 100 on the global leaderboard"

---

### epic_games_list_news

**What it does**: Lists news articles and game updates.

**When to use**: Stay updated on game news, find announcements.

**Arguments**:
- `country` (optional): Country code (default US)

**Example LLM prompt**: "List recent Epic Games news"

---

### epic_games_get_tournament

**What it does**: Gets tournament details and standings.

**When to use**: Check tournament info, view brackets.

**Arguments**:
- `id` (required): Tournament ID

**Example LLM prompt**: "Get details for tournament xyz789"

---

## Epic Games API Notes

- **Catalog**: Browse and search available games
- **Offers**: Current store deals and pricing
- **Entitlements**: Player-owned content and games
- **Achievements**: Unlockable milestones in games
- **Leaderboards**: Competitive rankings
- **Tournaments**: Competitive gaming events
