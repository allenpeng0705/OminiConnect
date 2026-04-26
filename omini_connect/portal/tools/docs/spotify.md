# Spotify Tools

Provider: `spotify` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Spotify Web API. They allow AI agents to manage playlists, search for tracks and artists, and access album and listening data on Spotify.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Spotify
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `playlist-read-private`, `playlist-read-collaborative`, `playlist-modify-public`, `playlist-modify-private`, `user-follow-read`, `user-read-private`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `spotify_list_playlists` | List user's playlists | GET | /me/playlists |
| `spotify_get_playlist` | Get playlist details | GET | /playlists/{playlist_id} |
| `spotify_create_playlist` | Create a new playlist | POST | /users/{user_id}/playlists |
| `spotify_list_tracks` | List tracks in a playlist | GET | /playlists/{playlist_id}/tracks |
| `spotify_get_track` | Get track details | GET | /tracks/{track_id} |
| `spotify_search_tracks` | Search for tracks by keyword | GET | /search |
| `spotify_list_artists` | List artists | GET | /me/following |
| `spotify_get_artist` | Get artist details | GET | /artists/{artist_id} |
| `spotify_list_albums` | List albums | GET | /artists/{artist_id}/albums |
| `spotify_get_album` | Get album details | GET | /albums/{album_id} |

---

## Tool Details

### spotify_list_playlists

**What it does**: Returns a paginated list of playlists owned or followed by the authenticated user.

**When to use**: Browse user's playlists, find specific collections, or manage playlist organization.

**Arguments**:
- `limit` (optional): Max playlists (default 50, max 50)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all my Spotify playlists"

---

### spotify_get_playlist

**What it does**: Gets detailed information about a specific playlist including name, description, cover image, and owner.

**When to use**: Get playlist metadata before adding tracks or sharing.

**Arguments**:
- `playlist_id` (required): Playlist ID

**Example LLM prompt**: "Get details for playlist 1234567890abcdef"

---

### spotify_create_playlist

**What it does**: Creates a new playlist for the authenticated user with name and optional description and visibility settings.

**When to use**: Create new playlists for organization, sharing, or recommendations.

**Arguments**:
- `user_id` (required): User ID (use 'me' for authenticated user)
- `name` (required): Playlist name
- `description` (optional): Playlist description
- `public` (optional): Make playlist public (default false)

**Example LLM prompt**: "Create a new playlist called 'My Favorites' for me"

---

### spotify_list_tracks

**What it does**: Lists all tracks in a playlist with full details including name, artists, album, and duration.

**When to use**: View playlist contents, analyze track selection, or prepare to modify playlist.

**Arguments**:
- `playlist_id` (required): Playlist ID
- `limit` (optional): Max tracks (default 100, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "Show all tracks in my 'Workout' playlist"

---

### spotify_get_track

**What it does**: Gets detailed information about a track including name, artists, album, duration, popularity, and preview URL.

**When to use**: Get track metadata before adding to playlist or analyzing.

**Arguments**:
- `track_id` (required): Track ID

**Example LLM prompt**: "Get details for track 3hN7iZ9LZkIggJOxhTQJ8b"

---

### spotify_search_tracks

**What it does**: Searches for tracks by keyword, returning matching results with artist and album info.

**When to use**: Find specific songs, discover similar music, or add tracks by name.

**Arguments**:
- `q` (required): Search query
- `type` (optional): Result type filter (default track)
- `limit` (optional): Max results (default 20, max 50)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "Search for tracks named 'Bohemian Rhapsody'"

---

### spotify_list_artists

**What it does**: Lists artists followed by the authenticated user with images, genres, and follower counts.

**When to use**: View followed artists, analyze music preferences, or find similar artists.

**Arguments**:
- `type` (optional): Type of items (default artist)
- `limit` (optional): Max artists (default 20, max 50)
- `after` (optional): Cursor for pagination (artist ID)

**Example LLM prompt**: "List all artists I follow on Spotify"

---

### spotify_get_artist

**What it does**: Gets detailed artist information including name, images, genres, and follower count.

**When to use**: Get artist details, view popularity metrics, or check genres.

**Arguments**:
- `artist_id` (required): Artist ID

**Example LLM prompt**: "Get details for artist 0YC192cP3KPCRWx8zr8MfZ"

---

### spotify_list_albums

**What it does**: Lists albums from an artist's discography with metadata, cover art, and track counts.

**When to use**: Browse artist's albums, find discography details.

**Arguments**:
- `artist_id` (required): Artist ID
- `limit` (optional): Max albums (default 20, max 50)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all albums by artist 0YC192cP3KPCRWx8zr8MfZ"

---

### spotify_get_album

**What it does**: Gets detailed album information including name, artist, cover art, release date, and complete track listing.

**When to use**: Get album metadata, view track list, or analyze album content.

**Arguments**:
- `album_id` (required): Album ID

**Example LLM prompt**: "Get details for album 4aawyAB9vmqN3uQ7FjRGTy"

---

## Spotify API Notes

- **Rate Limits**: 100 requests per 30 seconds for most endpoints
- **Pagination**: Uses offset-based pagination with limit parameter
- **IDs**: Spotify uses base-62 encoded IDs for tracks, artists, albums, and playlists
- **Scopes**: Some endpoints require specific OAuth scopes for access
- **Track Previews**: Not all tracks have preview URLs available