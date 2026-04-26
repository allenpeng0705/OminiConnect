# Baidu Tools

## Overview

[Baidu](https://baidu.com) is a Chinese multinational technology company specializing in internet-related services, AI, and search engine technology. It offers APIs for translation, maps, cloud storage (Pan), and face recognition.

## Authentication

Baidu APIs typically use API keys for authentication. The following scopes are supported:

| Scope | Description |
|-------|-------------|
| `translate` | Access to Baidu Translate API |
| `map` | Access to Baidu Maps API |
| `pan` | Access to Baidu Pan (Net Disk) API |
| `face` | Access to Baidu Face Recognition API |
| `search` | Access to Baidu Search API |

## Provider Configuration

```yaml
provider: baidu
auth_mode: API_KEY
base_url: https://api.baidu.com
```

## Tools

### Translate Tools

#### baidu_translate_text

Translate text between languages using Baidu Translate API.

**Endpoint:** `GET /api/trans/v2/lingo`

**Scopes:** `translate`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| text | string | Yes | Text to translate (max 6000 characters). |
| from | string | No | Source language code (e.g., en, zh, auto). Defaults to auto. |
| to | string | Yes | Target language code (e.g., en, zh). |

#### baidu_detect_language

Detect the language of the given text using Baidu Translate.

**Endpoint:** `GET /api/trans/v2/lingo`

**Scopes:** `translate`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| text | string | Yes | Text to detect language for (max 6000 characters). |

### Map Tools

#### baidu_get_map_place

Get detailed information about a specific place using Baidu Maps.

**Endpoint:** `GET /place/v2/detail`

**Scopes:** `map`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| uid | string | Yes | Unique place ID from Baidu Maps. |
| coord_type | string | No | Coordinate system: wgs84ll, gcj02ll, bd09ll. Defaults to gcj02ll. |

#### baidu_search_places

Search for places, businesses, or landmarks using Baidu Maps.

**Endpoint:** `GET /place/v2/search`

**Scopes:** `map`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| query | string | Yes | Search query term (e.g., restaurant, hotel). |
| region | string | Yes | City name or province to scope the search. |
| coord_type | string | No | Coordinate system for results: wgs84ll, gcj02ll, bd09ll. |
| radius | integer | No | Search radius in meters (max 50000). |
| page_size | integer | No | Number of results per page (max 20). |
| page_num | integer | No | Page number for pagination. |

### Pan (Cloud Storage) Tools

#### baidu_list_pan_files

List files in Baidu Pan (Net Disk) storage.

**Endpoint:** `GET /api/v1/pan_api/file/list`

**Scopes:** `pan`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| path | string | No | Directory path on Baidu Pan. Defaults to /. |
| order | string | No | Sort order: name, size, time. Defaults to time. |
| desc | boolean | No | Descending order. Defaults to true. |
| page_size | integer | No | Number of items per page (max 100). |
| page_index | integer | No | Page number (0-based). |

#### baidu_get_pan_file

Get metadata and download link for a specific file in Baidu Pan.

**Endpoint:** `GET /api/v1/pan_api/file/info`

**Scopes:** `pan`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| path | string | Yes | Full path to the file on Baidu Pan. |

#### baidu_upload_pan_file

Upload a file to Baidu Pan storage.

**Endpoint:** `POST /api/v1/pan_api/file/upload`

**Scopes:** `pan`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| path | string | Yes | Destination path on Baidu Pan. |
| content | string | Yes | Base64-encoded file content. |
| overwrite | boolean | No | Whether to overwrite if file exists. Defaults to false. |

### Face Recognition Tools

#### baidu_detect_face

Detect faces in an image and return attributes using Baidu Face API.

**Endpoint:** `POST /api/v1/face/v3/detect`

**Scopes:** `face`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| image | string | Yes | Base64-encoded image data (max 10MB). |
| image_type | string | No | Image type: BASE64, URL, FACE_TOKEN. Defaults to BASE64. |
| face_field | string | No | Face attributes to return: age, beauty, expression, face_shape, gender, glasses, landmark, race, quality. |

#### baidu_compare_faces

Compare two faces and return similarity score using Baidu Face API.

**Endpoint:** `POST /api/v1/face/v3/match`

**Scopes:** `face`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| image1 | string | Yes | First Base64-encoded image data. |
| image2 | string | Yes | Second Base64-encoded image data. |
| image_type | string | No | Image type: BASE64, URL, FACE_TOKEN. Defaults to BASE64. |

### Search Tools

#### baidu_search_web

Search the web using Baidu Search engine.

**Endpoint:** `GET /api/search/v1/compound`

**Scopes:** `search`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| query | string | Yes | Search query text. |
| page_num | integer | No | Page number (0-based). Defaults to 0. |
| page_size | integer | No | Number of results per page (max 50). |
