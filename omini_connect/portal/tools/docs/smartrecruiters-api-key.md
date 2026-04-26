# Smartrecruiters Tools

Provider: `smartrecruiters-api-key` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Smartrecruiters API. They allow AI agents to interact with Smartrecruiters functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_job_postings` | List all job postings | GET | /job_postings |
| `get_job_posting` | Get job posting details | GET | /job_postings/{id} |
| `create_job_posting` | Create a job posting | POST | /job_postings |
| `list_candidates` | List all candidates | GET | /candidates |
| `get_candidate` | Get candidate details | GET | /candidates/{id} |
| `list_publications` | List job publications | GET | /publications |
| `list_offers` | List all offers | GET | /offers |
| `get_offer` | Get offer details | GET | /offers/{id} |
| `list_interviews` | List scheduled interviews | GET | /interviews |
| `get_requisition` | Get requisition details | GET | /requisitions/{id} |

---

## Tool Details

### list_job_postings

**What it does**: List all job postings

**When to use**: Use this tool when you need to list all job postings.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list job postings to..."

---

### get_job_posting

**What it does**: Get job posting details

**When to use**: Use this tool when you need to get job posting details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get job posting to..."

---

### create_job_posting

**What it does**: Create a job posting

**When to use**: Use this tool when you need to create a job posting.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create job posting to..."

---

### list_candidates

**What it does**: List all candidates

**When to use**: Use this tool when you need to list all candidates.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list candidates to..."

---

### get_candidate

**What it does**: Get candidate details

**When to use**: Use this tool when you need to get candidate details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get candidate to..."

---

### list_publications

**What it does**: List job publications

**When to use**: Use this tool when you need to list job publications.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list publications to..."

---

### list_offers

**What it does**: List all offers

**When to use**: Use this tool when you need to list all offers.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list offers to..."

---

### get_offer

**What it does**: Get offer details

**When to use**: Use this tool when you need to get offer details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get offer to..."

---

### list_interviews

**What it does**: List scheduled interviews

**When to use**: Use this tool when you need to list scheduled interviews.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list interviews to..."

---

### get_requisition

**What it does**: Get requisition details

**When to use**: Use this tool when you need to get requisition details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get requisition to..."

---

## Smartrecruiters API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://api.smartrecruiters.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
