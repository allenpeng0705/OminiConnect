# Battle.net Tools

Provider: `battlenet` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Battle.net API. They allow AI agents to access game profiles, statistics, and achievements across Blizzard games including World of Warcraft, Diablo 3, StarCraft 2, and Overwatch.

## Authentication

**Nango OAuth2**:
- User authenticates via Battle.net OAuth
- Token stored in Nango, accessed via `connection_ref`
- Requires domain extension and API domain configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `battlenet_get_account` | Get Battle.net account info | GET | /account/user |
| `battlenet_list_games` | List available games | GET | /account/user/games |
| `battlenet_get_game_stats` | Get game statistics | GET | /profile/{region}/games/{gameId} |
| `battlenet_get_profile` | Get user profile | GET | /profile/{region}/player/{battletag} |
| `battlenet_list_sc2_profile` | Get StarCraft 2 profile | GET | /sc2/profile/{region}/{realm}/{profileId} |
| `battlenet_get_wow_character` | Get WoW character | GET | /wow/character/{realm}/{name} |
| `battlenet_list_wow_achievements` | List WoW achievements | GET | /wow/character/{realm}/{name}/achievements |
| `battlenet_get_d3_profile` | Get Diablo 3 profile | GET | /d3/profile/{account} |
| `battlenet_get_overwatch_profile` | Get Overwatch profile | GET | /overwatch/profile/{region}/{platform}/{tag} |
| `battlenet_list_news` | List game news | GET | /data/wow/news |

---

## Tool Details

### battlenet_get_account

**What it does**: Gets Battle.net account information.

**When to use**: Verify account, get linked games.

**Arguments**: None required

**Example LLM prompt**: "Get my Battle.net account info"

---

### battlenet_list_games

**What it does**: Lists all games linked to the Battle.net account.

**When to use**: See available games, find game IDs.

**Arguments**:
- `highlightCategory` (optional): Filter by category

**Example LLM prompt**: "List games on my Battle.net account"

---

### battlenet_get_game_stats

**What it does**: Gets statistics for a specific game.

**When to use**: View detailed game statistics.

**Arguments**:
- `region` (required): Region (us, eu, kr, tw, cn)
- `gameId` (required): Game ID

**Example LLM prompt**: "Get stats for game 1 in region us"

---

### battlenet_get_profile

**What it does**: Gets Battle.net user profile.

**When to use**: View player profile information.

**Arguments**:
- `region` (required): Region (us, eu, kr, tw, cn)
- `battletag` (required): BattleTag (e.g., User#1234)

**Example LLM prompt**: "Get profile for User#1234 in region us"

---

### battlenet_list_sc2_profile

**What it does**: Gets StarCraft 2 profile and statistics.

**When to use**: View SC2 rankings, campaign progress.

**Arguments**:
- `region` (required): Region ID (1=US, 2=EU, 3=KR)
- `realm` (required): Realm ID
- `profileId` (required): Profile ID

**Example LLM prompt**: "Get SC2 profile for region 1, realm 1, profile 123"

---

### battlenet_get_wow_character

**What it does**: Gets World of Warcraft character details.

**When to use**: View character装备, achievements, stats.

**Arguments**:
- `realm` (required): Realm slug
- `name` (required): Character name
- `fields` (optional): Additional fields (stats, items, achievements, etc.)

**Example LLM prompt**: "Get WoW character Thrall on realm Stormrage"

---

### battlenet_list_wow_achievements

**What it does**: Lists achievements for a WoW character.

**When to use**: Check character achievements, progress.

**Arguments**:
- `realm` (required): Realm slug
- `name` (required): Character name

**Example LLM prompt**: "List achievements for character Gandalf on realm Blackrock"

---

### battlenet_get_d3_profile

**What it does**: Gets Diablo 3 profile and seasonal data.

**When to use**: View D3 heroes, paragon level, seasonal progress.

**Arguments**:
- `account` (required): BattleTag (without #)

**Example LLM prompt**: "Get Diablo 3 profile for User123"

---

### battlenet_get_overwatch_profile

**What it does**: Gets Overwatch player profile and stats.

**When to use**: View competitive rank, quick play stats.

**Arguments**:
- `region` (required): Region (us, eu, kr, global)
- `platform` (required): Platform (pc, console)
- `tag` (required): BattleTag (without #)

**Example LLM prompt**: "Get Overwatch profile for User#1234 on pc, us region"

---

### battlenet_list_news

**What it does**: Lists latest game news and announcements.

**When to use**: Stay updated on game patches, events.

**Arguments**:
- `region` (optional): Region (default: us)
- `locale` (optional): Locale (default: en_US)

**Example LLM prompt**: "List latest WoW news"

---

## Battle.net API Notes

- **Regions**: US (us), EU (eu), Korea (kr), Taiwan (tw), China (cn)
- **BattleTag**: Format: Name#1234, used to identify players
- **Game IDs**: Each Blizzard game has a unique ID
- **Realms**: WoW servers, each with a unique slug
- **Rate Limits**: API has rate limiting; implement backoff for heavy use
