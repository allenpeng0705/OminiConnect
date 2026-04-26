# Yandex Tools

## Overview

[Yandex](https://yandex.com) is a Russian multinational technology company offering internet-related services including search, email, cloud storage, translation, and calendar.

## Authentication

Yandex uses OAuth 2.0 for authentication. The following scopes are supported:

| Scope | Description |
|-------|-------------|
| `cloud_api` | Access to Yandex Disk API |
| `translate` | Access to Yandex Translate API |
| `mail` | Access to Yandex Mail API |
| `calendar` | Access to Yandex Calendar API |

## Provider Configuration

```yaml
provider: yandex
auth_mode: OAUTH2
base_url: https://api.yandex.com
```

## Tools

### Disk Tools

#### yandex_list_disk_files

List all files and folders in Yandex Disk storage.

**Endpoint:** `GET /v1/disk/resources`

**Scopes:** `cloud_api`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| path | string | No | Path to the directory on Yandex Disk (e.g., /). Defaults to root. |
| limit | integer | No | Maximum number of items to return (1-100). |
| offset | integer | No | Number of items to skip for pagination. |

#### yandex_get_disk_file

Get metadata and download URL for a specific file on Yandex Disk.

**Endpoint:** `GET /v1/disk/resources`

**Scopes:** `cloud_api`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| path | string | Yes | Full path to the file on Yandex Disk (e.g., /photos/cat.jpg). |

#### yandex_upload_disk_file

Upload a file to Yandex Disk storage.

**Endpoint:** `PUT /v1/disk/resources`

**Scopes:** `cloud_api`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| path | string | Yes | Destination path on Yandex Disk (e.g., /photos/cat.jpg). |
| content | string | Yes | Base64-encoded file content or URL to upload from. |
| overwrite | boolean | No | Whether to overwrite existing file. Defaults to false. |

### Translate Tools

#### yandex_translate_text

Translate text between languages using Yandex Translate.

**Endpoint:** `GET /api/v1.5/tr.json/translate`

**Scopes:** `translate`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| text | string | Yes | Text to translate (max 10000 characters). |
| target_language | string | Yes | Target language code (e.g., en, ru, zh). Required if source not specified. |
| source_language | string | No | Source language code (e.g., en, auto for auto-detect). |
| format | string | No | Text format: plain or html. Defaults to plain. |

#### yandex_detect_language

Detect the language of the given text using Yandex Translate.

**Endpoint:** `GET /api/v1.5/tr.json/detect`

**Scopes:** `translate`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| text | string | Yes | Text to detect language for (max 10000 characters). |
| hint | string | No | Comma-separated list of language codes to prioritize in detection. |

### Mail Tools

#### yandex_list_mail_messages

List email messages from Yandex Mail inbox.

**Endpoint:** `GET /v2/messages`

**Scopes:** `mail`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| folder | string | No | Mail folder to list (inbox, sent, draft, spam, trash). Defaults to inbox. |
| limit | integer | No | Maximum number of messages to return (1-100). |
| offset | integer | No | Number of messages to skip for pagination. |

#### yandex_get_mail_message

Get a specific email message by ID from Yandex Mail.

**Endpoint:** `GET /v2/messages/{message_id}`

**Scopes:** `mail`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| message_id | string | Yes | Unique identifier of the email message. |

#### yandex_send_mail_message

Send an email message via Yandex Mail.

**Endpoint:** `POST /v2/messages/send`

**Scopes:** `mail`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| to | string | Yes | Recipient email address. |
| subject | string | Yes | Email subject line. |
| body | string | Yes | Email body content (plain text or HTML). |
| cc | string | No | CC recipient(s), comma-separated. |
| bcc | string | No | BCC recipient(s), comma-separated. |

### Calendar Tools

#### yandex_list_calendar_events

List events from Yandex Calendar.

**Endpoint:** `GET /calendar/v1/calendars/{calendar_id}/events`

**Scopes:** `calendar`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| calendar_id | string | No | Calendar ID. Use 'default' for primary calendar. |
| time_min | string | No | Start time in ISO 8601 format. |
| time_max | string | No | End time in ISO 8601 format. |
| limit | integer | No | Maximum number of events to return. |

#### yandex_get_calendar_event

Get a specific calendar event by ID from Yandex Calendar.

**Endpoint:** `GET /calendar/v1/calendars/{calendar_id}/events/{event_id}`

**Scopes:** `calendar`

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| calendar_id | string | Yes | Calendar ID. Use 'default' for primary calendar. |
| event_id | string | Yes | Unique identifier of the calendar event. |
