# Refiner Tools

Provider: `refiner` | Engine: `nango` | Auth: OAuth via Nango

## Overview

Refiner is a customer feedback and NPS survey platform. These tools allow AI agents to manage surveys, collect responses, analyze NPS scores, and target specific audience segments.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Refiner
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `surveys:read`, `surveys:write`, `responses:read`, `segments:read`, `analytics:read`, `targets:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `refiner_list_surveys` | List all surveys | GET | /surveys |
| `refiner_get_survey` | Get survey details | GET | /surveys/{surveyId} |
| `refiner_list_responses` | List survey responses | GET | /surveys/{surveyId}/responses |
| `refiner_get_response` | Get response details | GET | /responses/{responseId} |
| `refiner_create_survey` | Create a survey | POST | /surveys |
| `refiner_list_segments` | List all segments | GET | /segments |
| `refiner_get_segment` | Get segment details | GET | /segments/{segmentId} |
| `refiner_list_nps_scores` | List NPS scores | GET | /analytics/nps |
| `refiner_create_target` | Create a target profile | POST | /targets |
| `refiner_list_analytics` | Get survey analytics | GET | /analytics |

---

## Tool Details

### refiner_list_surveys

**What it does**: Returns a list of all surveys.

**When to use**: View your feedback surveys.

**Arguments**:
- `limit` (optional): Number of surveys (default 50)
- `status` (optional): Filter by status (active, paused, closed)

**Example LLM prompt**: "List all active surveys"

---

### refiner_get_survey

**What it does**: Gets details of a specific survey.

**When to use**: Get survey questions and configuration.

**Arguments**:
- `surveyId` (required): The survey ID

**Example LLM prompt**: "Get details for survey srv_abc123"

---

### refiner_list_responses

**What it does**: Returns a list of survey responses.

**When to use**: View feedback data collected.

**Arguments**:
- `surveyId` (required): The survey ID
- `limit` (optional): Number of responses (default 100)

**Example LLM prompt**: "List all responses for survey srv_abc123"

---

### refiner_get_response

**What it does**: Gets details of a specific response.

**When to use**: View individual feedback submission.

**Arguments**:
- `responseId` (required): The response ID

**Example LLM prompt**: "Get details for response rsp_xyz789"

---

### refiner_create_survey

**What it does**: Creates a new customer feedback survey.

**When to use**: Deploy new NPS or CSAT surveys.

**Arguments**:
- `name` (required): Survey name
- `type` (required): Survey type (nps, csat, ces)
- `questions` (optional): Array of questions

**Example LLM prompt**: "Create an NPS survey called 'Customer Satisfaction'"

---

### refiner_list_segments

**What it does**: Returns a list of all audience segments.

**When to use**: View targeting segments.

**Arguments**:
- `limit` (optional): Number of segments (default 50)

**Example LLM prompt**: "List all segments"

---

### refiner_get_segment

**What it does**: Gets details of a specific segment.

**When to use**: See segment criteria and size.

**Arguments**:
- `segmentId` (required): The segment ID

**Example LLM prompt**: "Get details for segment seg_123"

---

### refiner_list_nps_scores

**What it does**: Returns NPS scores and trends.

**When to use**: Track customer loyalty over time.

**Arguments**:
- `surveyId` (optional): Survey ID
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get NPS trend for the last 30 days"

---

### refiner_create_target

**What it does**: Creates a target profile for survey distribution.

**When to use**: Set up survey targeting rules.

**Arguments**:
- `name` (required): Target name
- `segmentId` (required): Segment ID
- `surveyId` (required): Survey ID

**Example LLM prompt**: "Create a target to survey segment 'Premium Users'"

---

### refiner_list_analytics

**What it does**: Returns survey analytics and metrics.

**When to use**: Get response rates and completion stats.

**Arguments**:
- `surveyId` (optional): Survey ID
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Get analytics for survey srv_abc123"

---

## Refiner Notes

- Survey types: NPS (Net Promoter Score), CSAT (Customer Satisfaction), CES (Customer Effort Score)
- Segments allow targeting specific user groups
- NPS scores range from 0-10 (Promoters 9-10, Passives 7-8, Detractors 0-6)
- Analytics show response rates and trends
