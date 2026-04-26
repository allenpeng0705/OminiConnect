# Google Maps Tools

Provider: `google-maps` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Google Maps API. They allow AI agents to geocode addresses, search for places, get directions, and more. **Requires Google Maps API key.**

## Authentication

**Nango API_KEY**:
- User provides their Google Maps API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://maps.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_maps_geocode` | Geocode address | GET | /maps/api/geocode/json |
| `google_maps_reverse_geocode` | Reverse geocode | GET | /maps/api/geocode/json |
| `google_maps_places_search` | Search places | GET | /maps/api/place/textsearch/json |
| `google_maps_place_details` | Get place details | GET | /maps/api/place/details/json |
| `google_maps_places_photo` | Get place photo | GET | /maps/api/place/photo |
| `google_maps_distance_matrix` | Get distance matrix | GET | /maps/api/distancematrix/json |
| `google_maps_directions` | Get directions | GET | /maps/api/directions/json |
| `google_maps_elevation` | Get elevation | GET | /maps/api/elevation/json |
| `google_maps_timezone` | Get timezone | GET | /maps/api/timezone/json |
| `google_maps_roads_nearest` | Get nearest roads | GET | /maps/api/roads/nearestRoads |

---

## Tool Details

### google_maps_geocode

**What it does**: Converts address to coordinates.

**When to use**: Get lat/lng for an address.

**Arguments**:
- `address` (required): Address to geocode

**Example LLM prompt**: "Geocode address '1600 Amphitheatre Parkway, Mountain View'"

---

### google_maps_reverse_geocode

**What it does**: Converts coordinates to address.

**When to use**: Get address from lat/lng.

**Arguments**:
- `latlng` (required): Latitude,longitude

**Example LLM prompt**: "Reverse geocode 37.422,-122.085"

---

### google_maps_places_search

**What it does**: Searches for places near a location.

**When to use**: Find businesses, landmarks.

**Arguments**:
- `query` (required): Search query
- `location` (optional): Latitude,longitude for proximity
- `radius` (optional): Radius in meters

**Example LLM prompt**: "Search for 'pizza' near 37.422,-122.085"

---

### google_maps_place_details

**What it does**: Gets detailed information about a place.

**When to use**: Get reviews, hours, phone.

**Arguments**:
- `place_id` (required): Place ID

**Example LLM prompt**: "Get details for place ChIJN1..."

---

### google_maps_places_photo

**What it does**: Gets photo reference for a place.

**When to use**: Get place photos.

**Arguments**:
- `photo_reference` (required): Photo reference
- `max_height` (optional): Max height

**Example LLM prompt**: "Get photo with reference abc123"

---

### google_maps_distance_matrix

**What it does**: Gets travel distances and times.

**When to use**: Calculate travel times between multiple points.

**Arguments**:
- `origins` (required): Origin addresses or lat,lng
- `destinations` (required): Destination addresses or lat,lng
- `mode` (optional): Travel mode (driving, walking, bicycling, transit)

**Example LLM prompt**: "Get driving distance from SF to LA"

---

### google_maps_directions

**What it does**: Gets directions between locations.

**When to use**: Get route planning.

**Arguments**:
- `origin` (required): Origin address or lat,lng
- `destination` (required): Destination address or lat,lng
- `mode` (optional): Travel mode

**Example LLM prompt**: "Get walking directions from home to office"

---

### google_maps_elevation

**What it does**: Gets elevation for coordinates.

**When to use**: Get altitude data.

**Arguments**:
- `locations` (required): Latitude,longitude pairs

**Example LLM prompt**: "Get elevation for 37.422,-122.085"

---

### google_maps_timezone

**What it does**: Gets timezone for a location.

**When to use**: Get local time information.

**Arguments**:
- `location` (required): Latitude,longitude
- `timestamp` (optional): Timestamp

**Example LLM prompt**: "Get timezone for 37.422,-122.085"

---

### google_maps_roads_nearest

**What it does**: Gets nearest road points to coordinates.

**When to use**: Snap GPS to roads.

**Arguments**:
- `points` (required): Latitude,longitude pairs

**Example LLM prompt**: "Find nearest road to 37.422,-122.085"

---

## Google Maps API Notes

- **API key**: Requires Google Maps API key with appropriate APIs enabled
- **Travel modes**: driving, walking, bicycling, transit
- **Place IDs**: Unique identifiers for places
- **Rate limits**: Subject to Google Maps API rate limits
- **Billing**: Each API has different billing
