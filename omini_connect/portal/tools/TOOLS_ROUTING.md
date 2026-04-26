# Tool Routing Guide

> **Routing Logic**: Only connected services work. Tools are dynamically filtered per user — LLMs can only call tools for platforms where the user has an enabled connector.

## Tier Definitions

| Tier | Auth Modes | Portal Support | Description |
|------|------------|----------------|-------------|
| **Tier 1** | `OAUTH2`, `OAUTH1`, `MCP_OAUTH2`, `API_KEY`, `BASIC`, `SIGNATURE`, `UNKNOWN` | Full support | Standard OAuth2/OAuth1/MCP_OAuth2, API key, Basic, or Signature auth. 636 providers. |
| **Tier 2** | `OAUTH2_CC`, `TWO_STEP`, `BILL`, `TBA` | Full support | Client credentials, two-step, and enterprise auth flows. 117 providers. |
| **Skipped** | `JWT`, `APP`, `CUSTOM`, `APP_STORE`, `INSTALL_PLUGIN`, `NONE` | Not yet supported | Complex auth modes deferred for later investigation. 7 providers. |

## Tier Summary

- **Tier 1**: 636 providers — Standard auth (OAUTH2, OAUTH1, MCP_OAUTH2, API_KEY, BASIC, SIGNATURE) + UNKNOWN (51 major platforms with missing auth_mode)
- **Tier 2**: 117 providers — Complex auth (OAUTH2_CC, TWO_STEP, BILL, TBA)
- **Skipped**: 7 providers — Not yet supported (JWT, APP, CUSTOM, APP_STORE, INSTALL_PLUGIN, NONE)
- **Total**: 760 providers

### Auth Mode Count

| Auth Mode | Count | Tier |
|-----------|-------|------|
| OAUTH2 | 258 | Tier 1 |
| API_KEY | 229 | Tier 1 |
| BASIC | 81 | Tier 1 |
| OAUTH2_CC | 72 | Tier 2 |
| TWO_STEP | 42 | Tier 2 |
| MCP_OAUTH2 | 12 | Tier 1 |
| OAUTH1 | 4 | Tier 1 |
| UNKNOWN | 51 | Tier 1 |
| SIGNATURE | 1 | Tier 1 |
| BILL | 2 | Tier 2 |
| TBA | 1 | Tier 2 |
| JWT | 3 | Skipped |
| APP | 2 | Skipped |
| CUSTOM | 1 | Skipped |
| APP_STORE | 1 | Skipped |
| INSTALL_PLUGIN | 1 | Skipped |
| NONE | 1 | Skipped |

## Full Provider List

| Provider | Auth Mode | Tier | Category | Tools |
|----------|-----------|------|----------|-------|
| accelo | OAUTH2 | Tier 1 | invoicing, ticketing | 10 |
| active-campaign | API_KEY | Tier 1 | marketing, communication | 10 |
| adobe | OAUTH2 | Tier 1 | design | 10 |
| adyen | OAUTH2 | Tier 1 | payment | 10 |
| affinity | BASIC | Tier 1 | crm | 10 |
| aircall | OAUTH2 | Tier 1 | support | 10 |
| airtable | OAUTH2 | Tier 1 | popular, productivity | 10 |
| algolia | API_KEY | Tier 1 | search | 10 |
| amplitude | BASIC | Tier 1 | analytics | 10 |
| anthropic | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| apaleo | OAUTH2 | Tier 1 | erp | 10 |
| apify | API_KEY | Tier 1 | dev-tools, analytics, productivity | 10 |
| apollo | API_KEY | Tier 1 | marketing | 10 |
| asana | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| attio | OAUTH2 | Tier 1 | crm | 10 |
| auth0 | OAUTH2 | Tier 1 | other | 10 |
| autodesk | OAUTH2 | Tier 1 | design | 10 |
| avalara | BASIC | Tier 1 | legal | 10 |
| avalara-sandbox | BASIC | Tier 1 | legal | 10 |
| avoma | API_KEY | Tier 1 | productivity | 10 |
| aws | OAUTH2 | Tier 1 | dev-tools, e-commerce | 10 |
| aws-iam | BASIC | Tier 1 | dev-tools | 10 |
| aws-scim | API_KEY | Tier 1 | dev-tools | 10 |
| axiom | API_KEY | Tier 1 | dev-tools, analytics | 10 |
| azure-devops | BASIC | Tier 1 | dev-tools | 10 |
| bamboohr | OAUTH2 | Tier 1 | hr | 10 |
| bamboohr-basic | BASIC | Tier 1 | hr | 10 |
| basecamp | OAUTH2 | Tier 1 | productivity | 10 |
| battlenet | OAUTH2 | Tier 1 | gaming | 10 |
| beehiiv | API_KEY | Tier 1 | communication, marketing | 10 |
| bettercontact | API_KEY | Tier 1 | crm | 10 |
| bigcommerce | OAUTH2 | Tier 1 | e-commerce | 10 |
| bing-webmasters | OAUTH2 | Tier 1 | dev-tools | 10 |
| bird | API_KEY | Tier 1 | communication | 10 |
| bitbucket | OAUTH2 | Tier 1 | dev-tools | 10 |
| bitdefender | BASIC | Tier 1 | other | 10 |
| bitly | OAUTH2 | Tier 1 | marketing, social | 10 |
| blackbaud | OAUTH2 | Tier 1 | crm | 10 |
| blackbaud-basic | BASIC | Tier 1 | crm | 10 |
| blandai | API_KEY | Tier 1 | support | 10 |
| boldsign | OAUTH2 | Tier 1 | legal | 10 |
| booking-com | BASIC | Tier 1 | e-commerce | 10 |
| box | OAUTH2 | Tier 1 | knowledge-base, storage | 10 |
| braintree | OAUTH2 | Tier 1 | payment | 10 |
| braintree-sandbox | OAUTH2 | Tier 1 | payment | 10 |
| braze | API_KEY | Tier 1 | communication | 10 |
| brevo-api-key | API_KEY | Tier 1 | marketing | 10 |
| brex | OAUTH2 | Tier 1 | banking | 10 |
| brex-api-key | API_KEY | Tier 1 | banking | 10 |
| brex-staging | OAUTH2 | Tier 1 | banking | 10 |
| builder-io-private | API_KEY | Tier 1 | dev-tools, design, cms | 10 |
| builder-io-public | API_KEY | Tier 1 | dev-tools, design, cms | 10 |
| buildium | API_KEY | Tier 1 | accounting, crm, payment | 10 |
| builtwith | API_KEY | Tier 1 | dev-tools, analytics, crm, marketing, e-commerce | 10 |
| bullhorn | OAUTH2 | Tier 1 | hr | 10 |
| cal-com-oauth | OAUTH2 | Tier 1 | productivity | 10 |
| cal-com-v1 | API_KEY | Tier 1 | productivity | 10 |
| cal-com-v2 | API_KEY | Tier 1 | productivity | 10 |
| calendly | OAUTH2 | Tier 1 | productivity | 10 |
| callrail | API_KEY | Tier 1 | marketing | 10 |
| candis | OAUTH2 | Tier 1 | accounting | 10 |
| canny | API_KEY | Tier 1 | support | 10 |
| canva | OAUTH2 | Tier 1 | design | 10 |
| canva-scim | API_KEY | Tier 1 | design, dev-tools | 10 |
| canvas-lms | OAUTH2 | Tier 1 | productivity | 10 |
| certn | API_KEY | Tier 1 | legal | 10 |
| chargebee | BASIC | Tier 1 | payment | 10 |
| chattermill | API_KEY | Tier 1 | support, analytics | 10 |
| checkhq | API_KEY | Tier 1 | accounting | 10 |
| checkr-partner | OAUTH2 | Tier 1 | legal | 10 |
| checkr-partner-staging | OAUTH2 | Tier 1 | legal | 10 |
| chorus | API_KEY | Tier 1 | analytics | 10 |
| cin7-core | API_KEY | Tier 1 | e-commerce | 10 |
| circle-so | API_KEY | Tier 1 | communication | 10 |
| cisco-duo-admin | BASIC | Tier 1 | other | 10 |
| clari-copilot | API_KEY | Tier 1 | marketing | 10 |
| clay | API_KEY | Tier 1 | crm, marketing | 10 |
| clerk | API_KEY | Tier 1 | dev-tools, other | 10 |
| cleverreach | OAUTH2 | Tier 1 | marketing | 10 |
| clickhouse | BASIC | Tier 1 | dev-tools | 10 |
| clicksend | BASIC | Tier 1 | communication | 10 |
| clickup | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| clio | OAUTH2 | Tier 1 | legal | 10 |
| close | OAUTH2 | Tier 1 | crm | 10 |
| cloudbeds | OAUTH2 | Tier 1 | other | 10 |
| cloudflare | API_KEY | Tier 1 | dev-tools | 10 |
| cloudtalk | BASIC | Tier 1 | communication | 10 |
| clover | OAUTH2 | Tier 1 | e-commerce | 10 |
| coda | API_KEY | Tier 1 | knowledge-base, productivity | 10 |
| codeclimate | API_KEY | Tier 1 | dev-tools, productivity | 10 |
| codegen | API_KEY | Tier 1 | dev-tools | 10 |
| companycam | API_KEY | Tier 1 | productivity | 10 |
| confluence-basic | BASIC | Tier 1 | knowledge-base, popular | 10 |
| connectwise-psa | BASIC | Tier 1 | support, ticketing | 10 |
| connectwise-psa-staging | BASIC | Tier 1 | support, ticketing | 10 |
| contentful | OAUTH2 | Tier 1 | dev-tools, design, cms | 10 |
| contentstack | OAUTH2 | Tier 1 | cms | 10 |
| copper | OAUTH2 | Tier 1 | crm | 10 |
| copper-api-key | API_KEY | Tier 1 | crm | 10 |
| coros | OAUTH2 | Tier 1 | sports | 10 |
| coros-sandbox | OAUTH2 | Tier 1 | sports | 10 |
| crisp | BASIC | Tier 1 | communication, support | 10 |
| cursor | API_KEY | Tier 1 | dev-tools | 10 |
| cursor-admin | BASIC | Tier 1 | dev-tools | 10 |
| cyberimpact | API_KEY | Tier 1 | marketing | 10 |
| datacandy | BASIC | Tier 1 | payment | 10 |
| datadog | API_KEY | Tier 1 | analytics, dev-tools | 10 |
| datev | OAUTH2 | Tier 1 | legal, hr | 10 |
| datto-rmm | OAUTH2 | Tier 1 | support | 10 |
| deel | OAUTH2 | Tier 1 | hr | 10 |
| deel-sandbox | OAUTH2 | Tier 1 | hr | 10 |
| demodesk | API_KEY | Tier 1 | productivity | 10 |
| devin | API_KEY | Tier 1 | dev-tools | 10 |
| dialpad | OAUTH2 | Tier 1 | communication | 10 |
| dialpad-sandbox | OAUTH2 | Tier 1 | communication | 10 |
| digitalocean | OAUTH2 | Tier 1 | dev-tools | 10 |
| discord | OAUTH2 | Tier 1 | gaming, social | 10 |
| discourse | API_KEY | Tier 1 | communication | 10 |
| dixa | API_KEY | Tier 1 | support | 10 |
| document360 | API_KEY | Tier 1 | knowledge-base | 10 |
| docusign | OAUTH2 | Tier 1 | legal | 10 |
| docusign-sandbox | OAUTH2 | Tier 1 | legal | 10 |
| drata | API_KEY | Tier 1 | dev-tools | 10 |
| drchrono | OAUTH2 | Tier 1 | other | 10 |
| dropbox | OAUTH2 | Tier 1 | knowledge-base, storage | 10 |
| dropbox-sign | OAUTH2 | Tier 1 | legal | 10 |
| e-conomic | BASIC | Tier 1 | accounting | 10 |
| ebay | OAUTH2 | Tier 1 | e-commerce | 10 |
| ebay-sandbox | OAUTH2 | Tier 1 | e-commerce | 10 |
| egnyte | OAUTH2 | Tier 1 | storage | 10 |
| elevenlabs | API_KEY | Tier 1 | dev-tools | 10 |
| elevio | API_KEY | Tier 1 | knowledge-base, support | 10 |
| employment-hero | OAUTH2 | Tier 1 | hr | 10 |
| entrata | BASIC | Tier 1 | other | 10 |
| envoy | OAUTH2 | Tier 1 | productivity | 10 |
| epic-games | OAUTH2 | Tier 1 | gaming | 10 |
| evaluagent | BASIC | Tier 1 | support | 10 |
| eventbrite | OAUTH2 | Tier 1 | marketing | 10 |
| exa | API_KEY | Tier 1 | analytics | 10 |
| exact-online | OAUTH2 | Tier 1 | accounting, erp | 10 |
| exist | OAUTH2 | Tier 1 | other | 10 |
| expensify | BASIC | Tier 1 | productivity | 10 |
| facebook | OAUTH2 | Tier 1 | marketing, social | 10 |
| factorial | OAUTH2 | Tier 1 | hr | 10 |
| fairing | API_KEY | Tier 1 | marketing, analytics | 10 |
| falai | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| fanvue | OAUTH2 | Tier 1 | other | 10 |
| fathom | API_KEY | Tier 1 | communication, video | 10 |
| fathom-oauth | OAUTH2 | Tier 1 | communication, video | 10 |
| fellow | API_KEY | Tier 1 | productivity | 10 |
| fiber-ai | API_KEY | Tier 1 | analytics | 10 |
| figma | OAUTH2 | Tier 1 | design, productivity | 10 |
| figma-scim | API_KEY | Tier 1 | design, productivity | 10 |
| fillout | OAUTH2 | Tier 1 | surveys | 10 |
| fillout-api-key | API_KEY | Tier 1 | surveys | 10 |
| findymail | API_KEY | Tier 1 | marketing, crm | 10 |
| fireflies | API_KEY | Tier 1 | analytics, communication, productivity | 10 |
| firstbase | API_KEY | Tier 1 | hr | 10 |
| fiserv-api-key | API_KEY | Tier 1 | banking, payment | 10 |
| fitbit | OAUTH2 | Tier 1 | sports | 10 |
| float | API_KEY | Tier 1 | productivity | 10 |
| folk | API_KEY | Tier 1 | crm | 10 |
| fortnox | OAUTH2 | Tier 1 | accounting, invoicing | 10 |
| freeagent | OAUTH2 | Tier 1 | accounting, invoicing | 10 |
| freeagent-sandbox | OAUTH2 | Tier 1 | accounting, invoicing | 10 |
| freepik | API_KEY | Tier 1 | other | 10 |
| freshbooks | OAUTH2 | Tier 1 | accounting | 10 |
| freshdesk | BASIC | Tier 1 | support | 10 |
| freshsales | API_KEY | Tier 1 | crm | 10 |
| freshservice | BASIC | Tier 1 | support | 11 |
| freshteam | API_KEY | Tier 1 | hr | 10 |
| front | OAUTH2 | Tier 1 | support, ticketing | 10 |
| front-api-key | API_KEY | Tier 1 | support, ticketing | 10 |
| fullenrich | API_KEY | Tier 1 | crm | 10 |
| gamma | API_KEY | Tier 1 | productivity | 10 |
| gem | API_KEY | Tier 1 | ats | 10 |
| gerrit | BASIC | Tier 1 | dev-tools | 10 |
| ghost-content | API_KEY | Tier 1 | dev-tools, design, cms | 10 |
| github | OAUTH2 | Tier 1 | dev-tools, support, ticketing | 10 |
| github-pat | API_KEY | Tier 1 | dev-tools, ticketing | 10 |
| gitlab | OAUTH2 | Tier 1 | dev-tools, ticketing | 10 |
| gitlab-pat | API_KEY | Tier 1 | dev-tools, ticketing | 10 |
| glyphic | API_KEY | Tier 1 | communication | 10 |
| gong | BASIC | Tier 1 | communication, marketing, productivity, video | 10 |
| gong-oauth | OAUTH2 | Tier 1 | communication, marketing, productivity, video, popular | 10 |
| google | OAUTH2 | Tier 1 | communication, dev-tools, productivity, social | 10 |
| google-gemini | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| google-maps | API_KEY | Tier 1 | productivity | 10 |
| google-play | OAUTH2 | Tier 1 | dev-tools | 10 |
| gorgias | OAUTH2 | Tier 1 | e-commerce | 10 |
| gorgias-basic | BASIC | Tier 1 | e-commerce | 10 |
| grafana | API_KEY | Tier 1 | dev-tools | 10 |
| grain | OAUTH2 | Tier 1 | video, communication, productivity | 10 |
| grain-api-key | API_KEY | Tier 1 | video, communication, productivity | 10 |
| grammarly-scim | API_KEY | Tier 1 | hr | 10 |
| granola | API_KEY | Tier 1 | productivity, knowledge-base | 10 |
| greenhouse | OAUTH2 | Tier 1 | ats | 10 |
| greenhouse-assessment | BASIC | Tier 1 | ats | 10 |
| greenhouse-basic | BASIC | Tier 1 | ats | 10 |
| greenhouse-harvest | BASIC | Tier 1 | ats | 10 |
| greenhouse-harvest-partner | OAUTH2 | Tier 1 | ats | 10 |
| greenhouse-ingestion | OAUTH2 | Tier 1 | ats | 10 |
| greenhouse-job-board | BASIC | Tier 1 | ats | 10 |
| greenhouse-onboarding | BASIC | Tier 1 | ats | 10 |
| grist | API_KEY | Tier 1 | productivity | 10 |
| gumroad | OAUTH2 | Tier 1 | design, e-commerce, payment | 10 |
| guru | BASIC | Tier 1 | knowledge-base | 10 |
| guru-scim | BASIC | Tier 1 | knowledge-base | 10 |
| gusto | OAUTH2 | Tier 1 | hr | 10 |
| gusto-demo | OAUTH2 | Tier 1 | hr | 10 |
| hackerrank-work | BASIC | Tier 1 | ats | 10 |
| harvest | OAUTH2 | Tier 1 | productivity | 10 |
| health-gorilla | OAUTH2 | Tier 1 | other | 10 |
| helpscout-docs | BASIC | Tier 1 | knowledge-base, support | 10 |
| helpscout-mailbox | OAUTH2 | Tier 1 | support | 10 |
| heygen | OAUTH2 | Tier 1 | video | 10 |
| heyreach | API_KEY | Tier 1 | social, marketing | 10 |
| hibob-service-user | BASIC | Tier 1 | hr, popular | 10 |
| highlevel | OAUTH2 | Tier 1 | marketing | 10 |
| highlevel-white-label | OAUTH2 | Tier 1 | crm | 10 |
| holded | API_KEY | Tier 1 | accounting, crm, invoicing | 10 |
| hover | OAUTH2 | Tier 1 | productivity | 10 |
| hubspot | OAUTH2 | Tier 1 | marketing, support, crm, popular | 10 |
| icypeas | API_KEY | Tier 1 | marketing, crm | 10 |
| incident-io | API_KEY | Tier 1 | ticketing | 10 |
| insightly | BASIC | Tier 1 | crm | 10 |
| instagram | OAUTH2 | Tier 1 | marketing, social | 10 |
| instantly | API_KEY | Tier 1 | marketing, communication | 10 |
| intercom | OAUTH2 | Tier 1 | marketing, popular, support, surveys, ticketing | 10 |
| intuit | OAUTH2 | Tier 1 | accounting | 10 |
| ironclad | OAUTH2 | Tier 1 | legal | 10 |
| itglue | API_KEY | Tier 1 | productivity, other | 10 |
| jazzhr | API_KEY | Tier 1 | hr | 10 |
| jira | OAUTH2 | Tier 1 | popular, productivity, ticketing | 10 |
| jira-basic | BASIC | Tier 1 | productivity, ticketing | 10 |
| jira-data-center | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| jira-data-center-api-key | API_KEY | Tier 1 | productivity, ticketing | 10 |
| jira-data-center-basic | BASIC | Tier 1 | productivity, ticketing | 10 |
| jobadder | OAUTH2 | Tier 1 | hr | 10 |
| jobber | OAUTH2 | Tier 1 | crm, invoicing | 10 |
| jobvite | API_KEY | Tier 1 | ats | 10 |
| jotform | API_KEY | Tier 1 | surveys | 10 |
| jumpcloud | API_KEY | Tier 1 | other | 10 |
| juniper-mist | API_KEY | Tier 1 | dev-tools | 10 |
| justworks | OAUTH2 | Tier 1 | hr | 10 |
| kandji | API_KEY | Tier 1 | support, productivity | 10 |
| keap | OAUTH2 | Tier 1 | marketing | 10 |
| keeper-scim | API_KEY | Tier 1 | productivity | 10 |
| kintone | OAUTH2 | Tier 1 | productivity | 10 |
| kintone-user-api | API_KEY | Tier 1 | productivity | 10 |
| klaviyo | API_KEY | Tier 1 | marketing | 10 |
| klaviyo-oauth | OAUTH2 | Tier 1 | marketing | 10 |
| klicktipp | API_KEY | Tier 1 | marketing | 10 |
| klipfolio | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| knowbe4 | API_KEY | Tier 1 | support | 10 |
| kustomer | API_KEY | Tier 1 | crm | 10 |
| lagrowthmachine | API_KEY | Tier 1 | marketing | 10 |
| lastpass | BASIC | Tier 1 | productivity | 10 |
| lattice | API_KEY | Tier 1 | hr | 10 |
| leadmagic | API_KEY | Tier 1 | marketing, crm | 10 |
| lemlist | BASIC | Tier 1 | marketing, communication | 10 |
| lessonly | BASIC | Tier 1 | productivity | 10 |
| lever | OAUTH2 | Tier 1 | ats | 10 |
| lever-basic | BASIC | Tier 1 | ats | 10 |
| lever-basic-sandbox | BASIC | Tier 1 | ats | 10 |
| lever-sandbox | OAUTH2 | Tier 1 | ats | 10 |
| lightspeed-retail | OAUTH2 | Tier 1 | e-commerce | 10 |
| linear | OAUTH2 | Tier 1 | popular, productivity, ticketing | 10 |
| linkedin | OAUTH2 | Tier 1 | ats, social | 10 |
| linkhut | OAUTH2 | Tier 1 | social | 10 |
| listmonk | BASIC | Tier 1 | marketing | 10 |
| lob | BASIC | Tier 1 | marketing | 10 |
| lokalise | OAUTH2 | Tier 1 | productivity | 10 |
| looker-oauth | OAUTH2 | Tier 1 | analytics | 10 |
| loom-scim | API_KEY | Tier 1 | other | 10 |
| loop-returns | API_KEY | Tier 1 | e-commerce | 10 |
| loops-so | API_KEY | Tier 1 | marketing, communication | 10 |
| lucid-scim | API_KEY | Tier 1 | productivity | 10 |
| luma | API_KEY | Tier 1 | productivity, ticketing | 10 |
| lumos | API_KEY | Tier 1 | productivity | 10 |
| mailchimp | OAUTH2 | Tier 1 | marketing, surveys | 10 |
| mailgun | BASIC | Tier 1 | marketing | 10 |
| mailjet | BASIC | Tier 1 | marketing | 10 |
| make | API_KEY | Tier 1 | productivity | 10 |
| manatal | API_KEY | Tier 1 | crm, hr | 10 |
| maximizer | OAUTH2 | Tier 1 | crm | 10 |
| maximizer-on-premise | OAUTH2 | Tier 1 | crm | 10 |
| mercury | OAUTH2 | Tier 1 | accounting | 10 |
| metabase | API_KEY | Tier 1 | analytics | 10 |
| microsoft | OAUTH2 | Tier 1 | communication, dev-tools, productivity | 10 |
| microsoft-admin | OAUTH2 | Tier 1 | communication, dev-tools, productivity | 10 |
| microsoft-teams-bot | OAUTH2 | Tier 1 | communication | 10 |
| microsoft-tenant-specific | OAUTH2 | Tier 1 | erp | 10 |
| mindbody | API_KEY | Tier 1 | productivity | 10 |
| minimax | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| miro | OAUTH2 | Tier 1 | design, productivity | 10 |
| miro-scim | API_KEY | Tier 1 | design, productivity | 10 |
| missive | API_KEY | Tier 1 | productivity | 10 |
| mixpanel | BASIC | Tier 1 | analytics | 10 |
| mollie | OAUTH2 | Tier 1 | payment | 10 |
| momentum-io | API_KEY | Tier 1 | productivity | 10 |
| monday | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| mural | OAUTH2 | Tier 1 | design | 10 |
| namely | OAUTH2 | Tier 1 | hr | 10 |
| namely-pat | API_KEY | Tier 1 | hr | 10 |
| nationbuilder | OAUTH2 | Tier 1 | crm | 10 |
| netsuite | OAUTH2 | Tier 1 | accounting, erp | 10 |
| next-cloud-ocs | BASIC | Tier 1 | storage, productivity | 10 |
| ninjaone-rmm-oauth2 | OAUTH2 | Tier 1 | support | 10 |
| nocrm | API_KEY | Tier 1 | crm, productivity | 10 |
| notion | OAUTH2 | Tier 1 | knowledge-base, popular, productivity | 10 |
| notion-scim | API_KEY | Tier 1 | knowledge-base, productivity | 10 |
| nyne-ai | API_KEY | Tier 1 | analytics | 10 |
| odoo | OAUTH2 | Tier 1 | erp | 10 |
| okta | OAUTH2 | Tier 1 | dev-tools | 10 |
| one-drive-personal | OAUTH2 | Tier 1 | knowledge-base, storage | 10 |
| onlogist | API_KEY | Tier 1 | other | 10 |
| oomnitza | API_KEY | Tier 1 | productivity | 10 |
| open-hands | API_KEY | Tier 1 | dev-tools | 10 |
| openai | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| openai-admin | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| oracle-hcm | BASIC | Tier 1 | hr | 10 |
| ordinal | API_KEY | Tier 1 | productivity | 10 |
| osu | OAUTH2 | Tier 1 | gaming | 10 |
| oura | OAUTH2 | Tier 1 | sports | 10 |
| outreach | OAUTH2 | Tier 1 | marketing | 10 |
| pagerduty | OAUTH2 | Tier 1 | dev-tools | 10 |
| paligo | BASIC | Tier 1 | cms | 10 |
| pandadoc | OAUTH2 | Tier 1 | legal | 10 |
| pandadoc-api-key | API_KEY | Tier 1 | legal | 10 |
| paycom | BASIC | Tier 1 | hr | 10 |
| paycor | OAUTH2 | Tier 1 | hr | 10 |
| paycor-sandbox | OAUTH2 | Tier 1 | hr | 10 |
| payfit | OAUTH2 | Tier 1 | hr | 10 |
| paypal | OAUTH2 | Tier 1 | payment | 10 |
| paypal-sandbox | OAUTH2 | Tier 1 | payment | 10 |
| pendo | API_KEY | Tier 1 | analytics | 10 |
| pennylane | OAUTH2 | Tier 1 | accounting, banking, invoicing, payment | 10 |
| pennylane-company-api | API_KEY | Tier 1 | accounting, banking, invoicing, payment | 10 |
| peopledatalabs | API_KEY | Tier 1 | analytics | 10 |
| perdoo | API_KEY | Tier 1 | productivity, analytics | 10 |
| perk | OAUTH2 | Tier 1 | productivity | 10 |
| perplexity | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| personio-recruiting | API_KEY | Tier 1 | hr | 10 |
| pingone | OAUTH2 | Tier 1 | dev-tools | 10 |
| pinterest | OAUTH2 | Tier 1 | design, marketing, social, video | 10 |
| pipedream | API_KEY | Tier 1 | dev-tools, productivity | 10 |
| pipedrive | OAUTH2 | Tier 1 | crm | 10 |
| pivotaltracker | API_KEY | Tier 1 | productivity | 10 |
| plain | API_KEY | Tier 1 | support | 10 |
| pleo | OAUTH2 | Tier 1 | payment, invoicing | 10 |
| podium | OAUTH2 | Tier 1 | communication, marketing | 10 |
| posthog | API_KEY | Tier 1 | dev-tools | 11 |
| practicefusion | OAUTH2 | Tier 1 | other | 10 |
| precisefp | OAUTH2 | Tier 1 | crm, productivity | 10 |
| printful | OAUTH2 | Tier 1 | e-commerce | 10 |
| private-api-basic | BASIC | Tier 1 | other | 10 |
| private-api-bearer | API_KEY | Tier 1 | other | 10 |
| prive | API_KEY | Tier 1 | e-commerce | 10 |
| procore | OAUTH2 | Tier 1 | erp | 10 |
| productboard | OAUTH2 | Tier 1 | productivity | 10 |
| prospeo | API_KEY | Tier 1 | crm, marketing | 10 |
| provenexpert | BASIC | Tier 1 | marketing | 10 |
| prtg-classic | API_KEY | Tier 1 | support | 10 |
| pylon | API_KEY | Tier 1 | productivity | 10 |
| qualia | BASIC | Tier 1 | legal | 10 |
| qualtrics | OAUTH2 | Tier 1 | surveys | 10 |
| quentn | API_KEY | Tier 1 | marketing | 10 |
| quickbase | API_KEY | Tier 1 | productivity | 10 |
| quickbooks | OAUTH2 | Tier 1 | accounting, popular | 10 |
| ragieai | API_KEY | Tier 1 | dev-tools | 10 |
| ramp | OAUTH2 | Tier 1 | banking | 10 |
| ramp-sandbox | OAUTH2 | Tier 1 | banking | 10 |
| rapidapi | API_KEY | Tier 1 | dev-tools | 10 |
| razorpay | BASIC | Tier 1 | payment | 10 |
| readwise | API_KEY | Tier 1 | productivity | 10 |
| readwise-reader | API_KEY | Tier 1 | productivity | 10 |
| reapit | OAUTH2 | Tier 1 | crm | 10 |
| recall-ai | API_KEY | Tier 1 | dev-tools, productivity | 10 |
| recharge | API_KEY | Tier 1 | e-commerce | 10 |
| recruitcrm | API_KEY | Tier 1 | hr | 10 |
| recruitee | API_KEY | Tier 1 | ats | 10 |
| recruiterflow | API_KEY | Tier 1 | hr | 10 |
| reddit | OAUTH2 | Tier 1 | social | 10 |
| refiner | API_KEY | Tier 1 | surveys | 10 |
| replicate | API_KEY | Tier 1 | dev-tools | 10 |
| reply-io | API_KEY | Tier 1 | marketing, communication | 10 |
| resend | API_KEY | Tier 1 | communication | 10 |
| retell-ai | API_KEY | Tier 1 | dev-tools, productivity | 10 |
| ring-central | OAUTH2 | Tier 1 | support | 10 |
| ring-central-sandbox | OAUTH2 | Tier 1 | communication | 10 |
| ringover | API_KEY | Tier 1 | communication | 10 |
| rippling | API_KEY | Tier 1 | hr | 10 |
| rippling-shop-app | OAUTH2 | Tier 1 | hr | 10 |
| roam-scim | API_KEY | Tier 1 | productivity | 10 |
| rock-gym-pro | BASIC | Tier 1 | crm | 10 |
| rocketlane | API_KEY | Tier 1 | productivity | 10 |
| rootly | API_KEY | Tier 1 | dev-tools | 10 |
| sage | OAUTH2 | Tier 1 | accounting, erp | 10 |
| sage-hr | API_KEY | Tier 1 | hr | 10 |
| sage-intacct-oauth | OAUTH2 | Tier 1 | accounting, erp, popular | 10 |
| sage-people | OAUTH2 | Tier 1 | hr | 10 |
| salesforce | OAUTH2 | Tier 1 | crm, popular | 10 |
| salesforce-experience-cloud | OAUTH2 | Tier 1 | crm | 10 |
| salesforce-sandbox | OAUTH2 | Tier 1 | crm | 10 |
| salesloft | OAUTH2 | Tier 1 | marketing | 10 |
| salesmsg-oauth2 | OAUTH2 | Tier 1 | communication, marketing | 10 |
| sap-odata-basic | BASIC | Tier 1 | erp | 10 |
| schwab | OAUTH2 | Tier 1 | banking, accounting | 10 |
| scrapedo | API_KEY | Tier 1 | other | 10 |
| sedna-basic | BASIC | Tier 1 | communication | 10 |
| segment | OAUTH2 | Tier 1 | analytics, marketing | 10 |
| sellsy | OAUTH2 | Tier 1 | invoicing, accounting, crm | 10 |
| semrush | API_KEY | Tier 1 | marketing, analytics | 10 |
| sendgrid | API_KEY | Tier 1 | marketing | 10 |
| sentinelone | API_KEY | Tier 1 | dev-tools | 10 |
| sentry | API_KEY | Tier 1 | dev-tools, analytics | 10 |
| sentry-oauth | OAUTH2 | Tier 1 | dev-tools, analytics | 10 |
| servicem8 | OAUTH2 | Tier 1 | productivity | 10 |
| servicenow | OAUTH2 | Tier 1 | productivity | 10 |
| shipbob-pat | API_KEY | Tier 1 | e-commerce | 10 |
| shipstation | BASIC | Tier 1 | e-commerce | 10 |
| shipstation-v2 | API_KEY | Tier 1 | e-commerce | 10 |
| shopify | OAUTH2 | Tier 1 | e-commerce, popular | 10 |
| shopify-api-key | API_KEY | Tier 1 | e-commerce | 10 |
| shopify-partner | API_KEY | Tier 1 | e-commerce | 10 |
| shopify-scim | API_KEY | Tier 1 | e-commerce | 10 |
| shortcut | API_KEY | Tier 1 | dev-tools, productivity | 10 |
| signnow | OAUTH2 | Tier 1 | legal | 10 |
| signnow-sandbox | OAUTH2 | Tier 1 | legal | 10 |
| skio | API_KEY | Tier 1 | e-commerce | 10 |
| slab | API_KEY | Tier 1 | productivity, knowledge-base | 10 |
| slack | OAUTH2 | Tier 1 | popular, productivity | 10 |
| smartlead-ai | API_KEY | Tier 1 | communication, marketing | 10 |
| smartrecruiters-api-key | API_KEY | Tier 1 | ats | 10 |
| smartsheet | OAUTH2 | Tier 1 | productivity | 10 |
| snapchat | OAUTH2 | Tier 1 | video, social | 10 |
| snipe-it | API_KEY | Tier 1 | productivity, other | 10 |
| snowflake | OAUTH2 | Tier 1 | dev-tools | 10 |
| splitwise | OAUTH2 | Tier 1 | payment, social | 10 |
| spotify | OAUTH2 | Tier 1 | other | 10 |
| squarespace | OAUTH2 | Tier 1 | dev-tools, design | 10 |
| squareup | OAUTH2 | Tier 1 | payment | 10 |
| squareup-sandbox | OAUTH2 | Tier 1 | payment | 10 |
| stackexchange | OAUTH2 | Tier 1 | knowledge-base, support | 10 |
| statista | API_KEY | Tier 1 | analytics | 10 |
| stay-ai | API_KEY | Tier 1 | e-commerce | 10 |
| strava | OAUTH2 | Tier 1 | social, sports | 10 |
| strava-web | OAUTH2 | Tier 1 | social, sports | 10 |
| streak | BASIC | Tier 1 | crm | 10 |
| stripe | OAUTH2 | Tier 1 | payment | 10 |
| stripe-api-key | BASIC | Tier 1 | payment | 10 |
| stripe-app | OAUTH2 | Tier 1 | payment | 10 |
| stripe-app-sandbox | OAUTH2 | Tier 1 | payment | 10 |
| stripe-express | OAUTH2 | Tier 1 | payment | 10 |
| supabase | API_KEY | Tier 1 | dev-tools, storage | 10 |
| supabase-mcp | API_KEY | Tier 1 | dev-tools, mcp | 10 |
| survey-monkey | OAUTH2 | Tier 1 | surveys | 10 |
| tailscale-api-key | API_KEY | Tier 1 | dev-tools, productivity | 10 |
| tally | API_KEY | Tier 1 | productivity | 10 |
| teamleader | OAUTH2 | Tier 1 | invoicing, crm | 10 |
| teamtailor | API_KEY | Tier 1 | ats | 10 |
| teamwork | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| telegram | API_KEY | Tier 1 | communication | 10 |
| terraform | API_KEY | Tier 1 | dev-tools | 10 |
| thrivecart-api-key | API_KEY | Tier 1 | e-commerce, payment | 10 |
| thrivecart-oauth | OAUTH2 | Tier 1 | e-commerce, payment | 10 |
| ticktick | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| tiktok-accounts | OAUTH2 | Tier 1 | social | 10 |
| tiktok-ads | OAUTH2 | Tier 1 | social | 10 |
| tiktok-personal | OAUTH2 | Tier 1 | social | 10 |
| timely | OAUTH2 | Tier 1 | productivity | 10 |
| tldv | API_KEY | Tier 1 | productivity | 10 |
| todoist | OAUTH2 | Tier 1 | productivity, ticketing | 10 |
| toggl | BASIC | Tier 1 | productivity | 20 |
| torii | API_KEY | Tier 1 | productivity | 10 |
| trakstar-hire | BASIC | Tier 1 | ats | 10 |
| trello-scim | API_KEY | Tier 1 | productivity, ticketing | 10 |
| tremendous | OAUTH2 | Tier 1 | payment | 10 |
| tremendous-sandbox | OAUTH2 | Tier 1 | payment | 10 |
| triple-whale | API_KEY | Tier 1 | analytics, marketing | 10 |
| tsheetsteam | OAUTH2 | Tier 1 | hr, productivity | 10 |
| tumblr | OAUTH2 | Tier 1 | social, communication | 10 |
| twenty-crm | API_KEY | Tier 1 | crm | 10 |
| twenty-crm-self-hosted | API_KEY | Tier 1 | crm | 10 |
| twilio | BASIC | Tier 1 | dev-tools, communication | 10 |
| twinfield | OAUTH2 | Tier 1 | accounting | 10 |
| twitch | OAUTH2 | Tier 1 | gaming, social, sports, video | 10 |
| twitter-v2 | OAUTH2 | Tier 1 | marketing, social | 10 |
| typeform | OAUTH2 | Tier 1 | surveys | 10 |
| typefully | API_KEY | Tier 1 | analytics, communication, social | 10 |
| typefully-v2 | API_KEY | Tier 1 | analytics, communication, social | 10 |
| uber | OAUTH2 | Tier 1 | other | 10 |
| ukg-pro | BASIC | Tier 1 | hr | 10 |
| ukg-pro-wfm | OAUTH2 | Tier 1 | hr | 10 |
| unanet | BASIC | Tier 1 | crm, erp, accounting | 10 |
| unipile | API_KEY | Tier 1 | crm, marketing | 10 |
| upsales | API_KEY | Tier 1 | crm | 10 |
| valley-api-key | API_KEY | Tier 1 | marketing | 10 |
| vercel | API_KEY | Tier 1 | dev-tools | 10 |
| vimeo | OAUTH2 | Tier 1 | video | 10 |
| vimeo-basic | BASIC | Tier 1 | video | 10 |
| wakatime | OAUTH2 | Tier 1 | analytics, dev-tools | 10 |
| wave-accounting | OAUTH2 | Tier 1 | accounting | 10 |
| wealthbox | OAUTH2 | Tier 1 | crm | 10 |
| webex | OAUTH2 | Tier 1 | communication | 10 |
| webflow | OAUTH2 | Tier 1 | dev-tools, design, cms | 10 |
| webinarjam | API_KEY | Tier 1 | marketing | 10 |
| whatsapp-business | API_KEY | Tier 1 | communication, marketing | 10 |
| whoop | OAUTH2 | Tier 1 | sports | 10 |
| wildix-pbx | OAUTH2 | Tier 1 | communication | 10 |
| wise-api-key | API_KEY | Tier 1 | banking, payment | 10 |
| wiseagent | OAUTH2 | Tier 1 | crm | 10 |
| wiza | API_KEY | Tier 1 | crm, marketing | 10 |
| woocommerce | BASIC | Tier 1 | e-commerce, dev-tools, design | 10 |
| wordpress | OAUTH2 | Tier 1 | dev-tools, design, cms | 10 |
| workable | API_KEY | Tier 1 | ats | 10 |
| workable-oauth | OAUTH2 | Tier 1 | ats | 10 |
| workday | BASIC | Tier 1 | hr, popular | 10 |
| workday-adaptive-planning-basic | BASIC | Tier 1 | accounting | 10 |
| workday-oauth | OAUTH2 | Tier 1 | hr, popular | 10 |
| workos | API_KEY | Tier 1 | productivity | 10 |
| workpath | API_KEY | Tier 1 | productivity | 10 |
| wrike | OAUTH2 | Tier 1 | productivity | 10 |
| xai | API_KEY | Tier 1 | productivity, dev-tools | 10 |
| xero | OAUTH2 | Tier 1 | accounting, popular | 10 |
| yahoo | OAUTH2 | Tier 1 | social | 10 |
| yandex | OAUTH2 | Tier 1 | social | 10 |
| zapier | OAUTH2 | Tier 1 | dev-tools | 10 |
| zapier-nla | OAUTH2 | Tier 1 | productivity | 10 |
| zapier-scim | API_KEY | Tier 1 | dev-tools | 10 |
| zendesk | OAUTH2 | Tier 1 | popular, support, ticketing | 10 |
| zendesk-sell | OAUTH2 | Tier 1 | crm | 10 |
| zenefits | OAUTH2 | Tier 1 | hr | 10 |
| zoho | OAUTH2 | Tier 1 | accounting | 10 |
| zoom | OAUTH2 | Tier 1 | video | 10 |
| zorus | API_KEY | Tier 1 | other | 10 |
| avanan | TWO_STEP | Tier 2 | other | 10 |
| bliro | OAUTH2_CC | Tier 2 | communication | 10 |
| breezy-hr | TWO_STEP | Tier 2 | hr | 10 |
| brightcrowd | OAUTH2_CC | Tier 2 | social | 10 |
| certn-partner | OAUTH2_CC | Tier 2 | legal | 10 |
| checkout-com | OAUTH2_CC | Tier 2 | payment | 10 |
| checkout-com-sandbox | OAUTH2_CC | Tier 2 | payment | 10 |
| cloudentity | OAUTH2_CC | Tier 2 | other | 10 |
| commercetools | OAUTH2_CC | Tier 2 | e-commerce | 10 |
| conductorone | OAUTH2_CC | Tier 2 | productivity | 10 |
| connectwise-rmm | OAUTH2_CC | Tier 2 | support | 10 |
| coupa-compass | OAUTH2_CC | Tier 2 | payment, invoicing | 10 |
| crowdstrike | OAUTH2_CC | Tier 2 | dev-tools | 10 |
| databricks-account | OAUTH2_CC | Tier 2 | analytics | 10 |
| databricks-workspace | OAUTH2_CC | Tier 2 | analytics | 10 |
| datto-rmm-password-grant | TWO_STEP | Tier 2 | support | 10 |
| dayforce | TWO_STEP | Tier 2 | hr | 10 |
| docuware | TWO_STEP | Tier 2 | productivity | 10 |
| domo | OAUTH2_CC | Tier 2 | productivity, other | 10 |
| drupal | TWO_STEP | Tier 2 | dev-tools, design, cms | 10 |
| ecu360 | TWO_STEP | Tier 2 | other | 10 |
| emarsys-oauth | OAUTH2_CC | Tier 2 | marketing | 10 |
| firefish | OAUTH2_CC | Tier 2 | crm | 10 |
| fiserv | OAUTH2_CC | Tier 2 | banking, payment | 10 |
| gainsight-cc | OAUTH2_CC | Tier 2 | support, crm | 10 |
| gebruder-weiss | OAUTH2_CC | Tier 2 | erp | 10 |
| google-service-account | TWO_STEP | Tier 2 | dev-tools | 10 |
| grammarly | OAUTH2_CC | Tier 2 | productivity | 10 |
| greenhouse-harvest-oauth2-cc | OAUTH2_CC | Tier 2 | ats | 10 |
| halo-psa | OAUTH2_CC | Tier 2 | support, ticketing | 10 |
| heap | TWO_STEP | Tier 2 | other | 10 |
| jamf | OAUTH2_CC | Tier 2 | other | 10 |
| jamf-basic | TWO_STEP | Tier 2 | other | 10 |
| jobdiva | TWO_STEP | Tier 2 | hr, ats | 10 |
| listrak | OAUTH2_CC | Tier 2 | marketing | 10 |
| looker | TWO_STEP | Tier 2 | analytics | 10 |
| malwarebytes | OAUTH2_CC | Tier 2 | other | 10 |
| marketo | OAUTH2_CC | Tier 2 | marketing | 10 |
| medallia | OAUTH2_CC | Tier 2 | crm, support, surveys | 10 |
| microsoft-business-central | OAUTH2_CC | Tier 2 | erp | 10 |
| microsoft-oauth2-cc | OAUTH2_CC | Tier 2 | communication, dev-tools, productivity | 10 |
| mimecast | OAUTH2_CC | Tier 2 | communication | 10 |
| mip-cloud | TWO_STEP | Tier 2 | accounting | 10 |
| mip-on-premise | TWO_STEP | Tier 2 | accounting | 10 |
| modmed | TWO_STEP | Tier 2 | other | 10 |
| nerdio | OAUTH2_CC | Tier 2 | other | 10 |
| ninjaone-rmm | OAUTH2_CC | Tier 2 | support | 10 |
| odoo-cc | TWO_STEP | Tier 2 | erp | 10 |
| okta-cc | OAUTH2_CC | Tier 2 | dev-tools | 10 |
| onelogin | OAUTH2_CC | Tier 2 | dev-tools | 10 |
| oracle-cloud-identity | OAUTH2_CC | Tier 2 | other, dev-tools | 10 |
| orange-logic | TWO_STEP | Tier 2 | storage | 10 |
| ory | OAUTH2_CC | Tier 2 | other | 10 |
| passportal | TWO_STEP | Tier 2 | support | 10 |
| pax8 | OAUTH2_CC | Tier 2 | other | 10 |
| paychex | OAUTH2_CC | Tier 2 | hr | 10 |
| paylocity | OAUTH2_CC | Tier 2 | hr | 10 |
| paylocity-nextgen | OAUTH2_CC | Tier 2 | hr | 10 |
| pendo-oauth | OAUTH2_CC | Tier 2 | analytics | 10 |
| perimeter81 | TWO_STEP | Tier 2 | productivity | 10 |
| personio | OAUTH2_CC | Tier 2 | hr | 10 |
| personio-v2 | OAUTH2_CC | Tier 2 | hr | 10 |
| pingboard | OAUTH2_CC | Tier 2 | productivity | 10 |
| pingone-cc | OAUTH2_CC | Tier 2 | dev-tools | 10 |
| pipedream-oauth2-cc | OAUTH2_CC | Tier 2 | dev-tools, productivity | 10 |
| redtail-crm-sandbox | TWO_STEP | Tier 2 | crm | 10 |
| researchdesk | TWO_STEP | Tier 2 | other | 10 |
| roller | OAUTH2_CC | Tier 2 | ticketing | 10 |
| sage-intacct | TWO_STEP | Tier 2 | accounting, erp | 10 |
| salesforce-cc | OAUTH2_CC | Tier 2 | crm | 10 |
| salesforce-cdp | TWO_STEP | Tier 2 | storage | 10 |
| salesforce-jwt | TWO_STEP | Tier 2 | crm | 10 |
| salesmsg | TWO_STEP | Tier 2 | communication, marketing | 10 |
| sap-business-one | TWO_STEP | Tier 2 | erp | 10 |
| sap-concur | OAUTH2_CC | Tier 2 | erp | 10 |
| sap-fieldglass | TWO_STEP | Tier 2 | hr | 10 |
| sap-odata-oauth2-cc | OAUTH2_CC | Tier 2 | erp | 10 |
| sap-success-factors | TWO_STEP | Tier 2 | hr | 10 |
| sedna | OAUTH2_CC | Tier 2 | communication | 10 |
| sellercloud | TWO_STEP | Tier 2 | e-commerce | 10 |
| sellsy-oauth2-cc | OAUTH2_CC | Tier 2 | invoicing, accounting, crm | 10 |
| servicenow-oauth2-cc | OAUTH2_CC | Tier 2 | productivity, ticketing | 10 |
| setmore | TWO_STEP | Tier 2 | productivity | 10 |
| sharepoint-online-oauth2-cc | OAUTH2_CC | Tier 2 | storage, communication | 10 |
| sharepoint-online-v1 | TWO_STEP | Tier 2 | storage, communication | 10 |
| shopify-cc | OAUTH2_CC | Tier 2 | e-commerce | 10 |
| shopworks | TWO_STEP | Tier 2 | other | 10 |
| sophos-central | OAUTH2_CC | Tier 2 | dev-tools | 10 |
| spotify-oauth2-cc | OAUTH2_CC | Tier 2 | other | 10 |
| tableau | TWO_STEP | Tier 2 | analytics | 10 |
| tailscale | TWO_STEP | Tier 2 | dev-tools, productivity | 10 |
| tapclicks | OAUTH2_CC | Tier 2 | marketing, analytics | 10 |
| timify | TWO_STEP | Tier 2 | productivity | 10 |
| trafft | OAUTH2_CC | Tier 2 | productivity | 10 |
| twitter-oauth2-cc | OAUTH2_CC | Tier 2 | marketing, social | 10 |
| ukg-pro-cc | OAUTH2_CC | Tier 2 | hr | 10 |
| ukg-ready | OAUTH2_CC | Tier 2 | hr | 10 |
| valley | TWO_STEP | Tier 2 | marketing | 10 |
| vanta | OAUTH2_CC | Tier 2 | dev-tools | 10 |
| veeva-vault | TWO_STEP | Tier 2 | crm | 10 |
| workday-adaptive-planning | TWO_STEP | Tier 2 | accounting | 10 |
| workday-refresh-token | TWO_STEP | Tier 2 | hr | 10 |
| xero-oauth2-cc | OAUTH2_CC | Tier 2 | accounting, popular | 10 |
| yotpo | TWO_STEP | Tier 2 | e-commerce | 10 |
| zoominfo | OAUTH2_CC | Tier 2 | crm, marketing | 10 |
| zuora | OAUTH2_CC | Tier 2 | erp | 10 |
| 17hats | UNKNOWN | Tier 3 | other | 10 |
| acuity | UNKNOWN | Tier 3 | other | 10 |
| aha | UNKNOWN | Tier 3 | other | 10 |
| aiven | UNKNOWN | Tier 3 | other | 10 |
| akamai | UNKNOWN | Tier 3 | other | 10 |
| alibaba | UNKNOWN | Tier 3 | other | 10 |
| anchor | UNKNOWN | Tier 3 | other | 10 |
| artfol | UNKNOWN | Tier 3 | other | 10 |
| attach | UNKNOWN | Tier 3 | other | 10 |
| autobot | UNKNOWN | Tier 3 | other | 10 |
| avaza | UNKNOWN | Tier 3 | other | 10 |
| azure | UNKNOWN | Tier 3 | other | 10 |
| azure-blob-storage | UNKNOWN | Tier 3 | storage | 10 |
| azure_devops | UNKNOWN | Tier 3 | other | 10 |
| baidu | UNKNOWN | Tier 3 | other | 10 |
| bear | UNKNOWN | Tier 3 | other | 10 |
| bereal | UNKNOWN | Tier 3 | other | 9 |
| bilibili | UNKNOWN | Tier 3 | other | 10 |
| bill | BILL | Tier 3 | payment | 10 |
| bill-sandbox | BILL | Tier 3 | payment | 10 |
| bookly | UNKNOWN | Tier 3 | other | 10 |
| botpress | UNKNOWN | Tier 3 | other | 10 |
| bytedance | UNKNOWN | Tier 3 | other | 10 |
| calendary | UNKNOWN | Tier 3 | other | 10 |
| calendly_more | UNKNOWN | Tier 3 | other | 10 |
| charcle | UNKNOWN | Tier 3 | other | 10 |
| chatfuel | UNKNOWN | Tier 3 | other | 10 |
| chatwork | UNKNOWN | Tier 3 | other | 10 |
| circleback-mcp | MCP_OAUTH2 | Tier 3 | productivity, mcp | 10 |
| clarity | UNKNOWN | Tier 3 | other | 10 |
| clockify | UNKNOWN | Tier 3 | other | 10 |
| cloudflare_more | UNKNOWN | Tier 3 | other | 10 |
| confluence | UNKNOWN | Tier 3 | knowledge-base, popular | 10 |
| copper_more | UNKNOWN | Tier 3 | other | 10 |
| coscreen | UNKNOWN | Tier 3 | other | 10 |
| craft | UNKNOWN | Tier 3 | other | 10 |
| crisp-plugin-install | INSTALL_PLUGIN | Tier 3 | communication, support | 10 |
| databricks | UNKNOWN | Tier 3 | other | 10 |
| datocms | UNKNOWN | Tier 3 | other | 10 |
| dependabot | UNKNOWN | Tier 3 | other | 10 |
| drift | UNKNOWN | Tier 3 | other | 10 |
| dropbox_more | UNKNOWN | Tier 3 | other | 10 |
| dropboxpaper | UNKNOWN | Tier 3 | other | 10 |
| dubsado | UNKNOWN | Tier 3 | other | 10 |
| duda | UNKNOWN | Tier 3 | other | 10 |
| ecu360-production | UNKNOWN | Tier 3 | other | 10 |
| elasticsearch | UNKNOWN | Tier 3 | other | 10 |
| emarsys | SIGNATURE | Tier 3 | marketing | 10 |
| engage | UNKNOWN | Tier 3 | other | 10 |
| evernote | UNKNOWN | Tier 3 | other | 10 |
| fastly | UNKNOWN | Tier 3 | other | 10 |
| figjam | UNKNOWN | Tier 3 | design, productivity | 10 |
| figma_more | UNKNOWN | Tier 3 | other | 10 |
| fireberry | UNKNOWN | Tier 3 | other | 10 |
| fluidtopics | UNKNOWN | Tier 3 | other | 10 |
| fly | UNKNOWN | Tier 3 | other | 10 |
| fogbugz | UNKNOWN | Tier 3 | other | 10 |
| freshdesk_more | UNKNOWN | Tier 3 | other | 10 |
| freshmarketer | UNKNOWN | Tier 3 | other | 10 |
| front_more | UNKNOWN | Tier 3 | other | 10 |
| fullstory | UNKNOWN | Tier 3 | other | 10 |
| functionize | UNKNOWN | Tier 3 | other | 10 |
| garmin | OAUTH1 | Tier 3 | sports | 10 |
| ghost | UNKNOWN | Tier 3 | other | 10 |
| ghost-admin | JWT | Tier 3 | dev-tools, design, cms | 10 |
| github-app | APP | Tier 3 | dev-tools, popular, ticketing | 10 |
| github-app-oauth | CUSTOM | Tier 3 | dev-tools, ticketing | 10 |
| gobi | UNKNOWN | Tier 3 | other | 10 |
| google-ads | UNKNOWN | Tier 3 | marketing | 10 |
| google-analytics | UNKNOWN | Tier 3 | analytics | 10 |
| google-bigquery | UNKNOWN | Tier 3 | productivity | 10 |
| google-calendar | UNKNOWN | Tier 3 | popular, productivity | 10 |
| google-chat | UNKNOWN | Tier 3 | productivity | 10 |
| google-cloud-storage | UNKNOWN | Tier 3 | storage | 10 |
| google-contacts | UNKNOWN | Tier 3 | productivity | 10 |
| google-docs | UNKNOWN | Tier 3 | productivity | 10 |
| google-drive | UNKNOWN | Tier 3 | knowledge-base, popular, storage | 10 |
| google-forms | UNKNOWN | Tier 3 | productivity | 10 |
| google-mail | UNKNOWN | Tier 3 | popular, productivity | 10 |
| google-meet | UNKNOWN | Tier 3 | communication, productivity | 10 |
| google-search-console | UNKNOWN | Tier 3 | productivity | 10 |
| google-sheet | UNKNOWN | Tier 3 | productivity | 10 |
| google-slides | UNKNOWN | Tier 3 | productivity | 10 |
| google-tasks | UNKNOWN | Tier 3 | productivity | 10 |
| google-workspace-admin | UNKNOWN | Tier 3 | productivity | 10 |
| google_workspace | UNKNOWN | Tier 3 | other | 11 |
| googledrive | UNKNOWN | Tier 3 | other | 10 |
| gotomeeting | UNKNOWN | Tier 3 | other | 10 |
| granola-mcp | MCP_OAUTH2 | Tier 3 | productivity, knowledge-base, mcp | 10 |
| groove | UNKNOWN | Tier 3 | other | 10 |
| guidaki | UNKNOWN | Tier 3 | other | 10 |
| helpscout | UNKNOWN | Tier 3 | other | 10 |
| heroku | UNKNOWN | Tier 3 | other | 10 |
| heymarket | JWT | Tier 3 | communication, marketing | 10 |
| hotjar | UNKNOWN | Tier 3 | other | 10 |
| hubspot-mcp | MCP_OAUTH2 | Tier 3 | marketing, support, crm, mcp | 10 |
| hubspot_more | UNKNOWN | Tier 3 | other | 10 |
| huggingface | UNKNOWN | Tier 3 | other | 10 |
| ibm | UNKNOWN | Tier 3 | other | 10 |
| integromat | UNKNOWN | Tier 3 | other | 10 |
| intercom_more | UNKNOWN | Tier 3 | other | 10 |
| invision | UNKNOWN | Tier 3 | other | 10 |
| kontent | UNKNOWN | Tier 3 | other | 10 |
| lamatic | UNKNOWN | Tier 3 | other | 10 |
| landbot | UNKNOWN | Tier 3 | other | 10 |
| launchdarkly | UNKNOWN | Tier 3 | other | 10 |
| levelhead | UNKNOWN | Tier 3 | other | 10 |
| lightspeed | UNKNOWN | Tier 3 | other | 10 |
| linear-mcp | MCP_OAUTH2 | Tier 3 | popular, productivity, ticketing, mcp | 10 |
| livechat | UNKNOWN | Tier 3 | other | 10 |
| livechat_more | UNKNOWN | Tier 3 | other | 10 |
| loom | UNKNOWN | Tier 3 | other | 10 |
| luckyorange | UNKNOWN | Tier 3 | other | 10 |
| manychat | UNKNOWN | Tier 3 | other | 10 |
| mcp-generic | MCP_OAUTH2_GENERIC | Tier 3 | mcp | 10 |
| medium | UNKNOWN | Tier 3 | other | 10 |
| messagebird | UNKNOWN | Tier 3 | other | 10 |
| meta-marketing-api | UNKNOWN | Tier 3 | marketing | 10 |
| method_crm | UNKNOWN | Tier 3 | other | 10 |
| microsoft-ads | UNKNOWN | Tier 3 | marketing | 10 |
| microsoft-entra-id | UNKNOWN | Tier 3 | other | 10 |
| microsoft-excel | UNKNOWN | Tier 3 | productivity, analytics | 10 |
| microsoft-planner | UNKNOWN | Tier 3 | productivity, analytics | 10 |
| microsoft-power-bi | UNKNOWN | Tier 3 | productivity | 10 |
| microsoft-powerpoint | UNKNOWN | Tier 3 | productivity | 10 |
| microsoft-teams | UNKNOWN | Tier 3 | productivity, video, popular | 10 |
| microsoft-word | UNKNOWN | Tier 3 | productivity | 10 |
| microsoft_365 | UNKNOWN | Tier 3 | other | 10 |
| microsoft_graph | UNKNOWN | Tier 3 | other | 10 |
| miro_more | UNKNOWN | Tier 3 | other | 10 |
| mobilemonkey | UNKNOWN | Tier 3 | other | 10 |
| mode | UNKNOWN | Tier 3 | other | 10 |
| mux | UNKNOWN | Tier 3 | other | 10 |
| n8n | UNKNOWN | Tier 3 | other | 10 |
| netlify | UNKNOWN | Tier 3 | other | 10 |
| netsuite-tba | TBA | Tier 3 | accounting, erp, popular | 10 |
| nextiva | UNKNOWN | Tier 3 | other | 10 |
| nimble | UNKNOWN | Tier 3 | other | 10 |
| note | UNKNOWN | Tier 3 | other | 10 |
| notion-mcp | MCP_OAUTH2 | Tier 3 | knowledge-base, productivity, mcp | 10 |
| nutshell | UNKNOWN | Tier 3 | other | 10 |
| okta-preview | UNKNOWN | Tier 3 | dev-tools | 10 |
| olark | UNKNOWN | Tier 3 | other | 10 |
| onceup | UNKNOWN | Tier 3 | other | 10 |
| one-drive | UNKNOWN | Tier 3 | knowledge-base, storage | 10 |
| one-note | UNKNOWN | Tier 3 | productivity | 10 |
| onedrive | UNKNOWN | Tier 3 | other | 10 |
| opsgenie | UNKNOWN | Tier 3 | other | 10 |
| optimizely | UNKNOWN | Tier 3 | other | 10 |
| oracle | UNKNOWN | Tier 3 | other | 10 |
| outlook | UNKNOWN | Tier 3 | communication | 10 |
| papaya_global | UNKNOWN | Tier 3 | other | 10 |
| phabricator | UNKNOWN | Tier 3 | other | 10 |
| pipeline | UNKNOWN | Tier 3 | other | 10 |
| pivotal | UNKNOWN | Tier 3 | other | 10 |
| planetscale | UNKNOWN | Tier 3 | other | 10 |
| plivo | UNKNOWN | Tier 3 | other | 10 |
| postmark | UNKNOWN | Tier 3 | other | 10 |
| printify | UNKNOWN | Tier 3 | other | 10 |
| prismic | UNKNOWN | Tier 3 | other | 10 |
| projectmanager | UNKNOWN | Tier 3 | other | 10 |
| quickbooks-sandbox | UNKNOWN | Tier 3 | accounting | 10 |
| quora | UNKNOWN | Tier 3 | other | 10 |
| railway | UNKNOWN | Tier 3 | other | 10 |
| rally | UNKNOWN | Tier 3 | other | 10 |
| recurly | UNKNOWN | Tier 3 | other | 10 |
| redbubble | UNKNOWN | Tier 3 | other | 10 |
| redshift | UNKNOWN | Tier 3 | other | 10 |
| remote | UNKNOWN | Tier 3 | other | 10 |
| render | UNKNOWN | Tier 3 | other | 10 |
| renovate | UNKNOWN | Tier 3 | other | 10 |
| ringcentral | UNKNOWN | Tier 3 | other | 10 |
| rows | UNKNOWN | Tier 3 | other | 10 |
| salesforce_more | UNKNOWN | Tier 3 | other | 11 |
| sanity | UNKNOWN | Tier 3 | other | 10 |
| sap | UNKNOWN | Tier 3 | other | 10 |
| sas | UNKNOWN | Tier 3 | other | 10 |
| scoro | UNKNOWN | Tier 3 | other | 10 |
| sharepoint-online | UNKNOWN | Tier 3 | storage, communication | 10 |
| sharpspring | UNKNOWN | Tier 3 | other | 10 |
| shopify_more | UNKNOWN | Tier 3 | other | 10 |
| simpleview | UNKNOWN | Tier 3 | other | 10 |
| sinch | UNKNOWN | Tier 3 | other | 10 |
| slack-mcp | MCP_OAUTH2 | Tier 3 | popular, productivity, communication, mcp | 10 |
| slite | UNKNOWN | Tier 3 | other | 10 |
| smugmug | OAUTH1 | Tier 3 | storage | 10 |
| snapchat_more | UNKNOWN | Tier 3 | other | 10 |
| snapengage | UNKNOWN | Tier 3 | other | 10 |
| snowflake-jwt | JWT | Tier 3 | dev-tools | 10 |
| snyk | UNKNOWN | Tier 3 | other | 10 |
| soundcloud | UNKNOWN | Tier 3 | other | 10 |
| sparkpost | UNKNOWN | Tier 3 | other | 10 |
| splash | UNKNOWN | Tier 3 | other | 10 |
| split | UNKNOWN | Tier 3 | other | 10 |
| square | UNKNOWN | Tier 3 | other | 10 |
| stackby | UNKNOWN | Tier 3 | other | 10 |
| statsig | UNKNOWN | Tier 3 | other | 10 |
| storyblok | UNKNOWN | Tier 3 | other | 10 |
| strapi | UNKNOWN | Tier 3 | other | 10 |
| stripe_more | UNKNOWN | Tier 3 | other | 10 |
| substack | UNKNOWN | Tier 3 | other | 10 |
| sugarcrm | UNKNOWN | Tier 3 | other | 10 |
| suitedash | UNKNOWN | Tier 3 | other | 10 |
| supabase-mcp-oauth | MCP_OAUTH2 | Tier 3 | dev-tools, mcp | 10 |
| supportbee | UNKNOWN | Tier 3 | other | 10 |
| tawkto | UNKNOWN | Tier 3 | other | 10 |
| taxjar | UNKNOWN | Tier 3 | other | 10 |
| teams | UNKNOWN | Tier 3 | other | 10 |
| teamsnap | UNKNOWN | Tier 3 | other | 10 |
| teespring | UNKNOWN | Tier 3 | other | 10 |
| tencent | UNKNOWN | Tier 3 | other | 10 |
| tiktok | UNKNOWN | Tier 3 | other | 10 |
| timeular | UNKNOWN | Tier 3 | other | 20 |
| tito | UNKNOWN | Tier 3 | other | 10 |
| trello | OAUTH1 | Tier 3 | productivity, ticketing | 10 |
| trello_more | UNKNOWN | Tier 3 | other | 10 |
| trigify-io-mcp | MCP_OAUTH2 | Tier 3 | dev-tools, mcp | 10 |
| twitter | OAUTH1 | Tier 3 | marketing, social | 10 |
| unauthenticated | NONE | Tier 3 | other | 10 |
| upsun | UNKNOWN | Tier 3 | other | 10 |
| userlike | UNKNOWN | Tier 3 | other | 10 |
| vercel-mcp | MCP_OAUTH2 | Tier 3 | dev-tools, mcp | 10 |
| walkme | UNKNOWN | Tier 3 | other | 10 |
| wautomations | UNKNOWN | Tier 3 | other | 10 |
| wave | UNKNOWN | Tier 3 | other | 10 |
| whitesource | UNKNOWN | Tier 3 | other | 10 |
| wix | UNKNOWN | Tier 3 | other | 10 |
| youtrack | UNKNOWN | Tier 3 | other | 10 |
| youtube | UNKNOWN | Tier 3 | video | 10 |
| zapier_more | UNKNOWN | Tier 3 | other | 10 |
| zeplin | UNKNOWN | Tier 3 | other | 10 |
| zoho-bigin | UNKNOWN | Tier 3 | crm | 10 |
| zoho-books | UNKNOWN | Tier 3 | accounting | 10 |
| zoho-calendar | UNKNOWN | Tier 3 | productivity | 10 |
| zoho-crm | UNKNOWN | Tier 3 | crm | 10 |
| zoho-desk | UNKNOWN | Tier 3 | support, ticketing | 10 |
| zoho-inventory | UNKNOWN | Tier 3 | e-commerce | 10 |
| zoho-invoice | UNKNOWN | Tier 3 | invoicing | 10 |
| zoho-mail | UNKNOWN | Tier 3 | productivity, communication | 10 |
| zoho-people | UNKNOWN | Tier 3 | hr | 10 |
| zoho-recruit | UNKNOWN | Tier 3 | ats | 10 |
| zoho_crm | UNKNOWN | Tier 3 | other | 10 |
| zoho_more | UNKNOWN | Tier 3 | other | 10 |

## Providers by Category

### accounting (35 providers)

- **buildium** — Tier 1 · API_KEY · 10 tools
- **candis** — Tier 1 · OAUTH2 · 10 tools
- **checkhq** — Tier 1 · API_KEY · 10 tools
- **e-conomic** — Tier 1 · BASIC · 10 tools
- **exact-online** — Tier 1 · OAUTH2 · 10 tools
- **fortnox** — Tier 1 · OAUTH2 · 10 tools
- **freeagent** — Tier 1 · OAUTH2 · 10 tools
- **freeagent-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **freshbooks** — Tier 1 · OAUTH2 · 10 tools
- **holded** — Tier 1 · API_KEY · 10 tools
- **intuit** — Tier 1 · OAUTH2 · 10 tools
- **mercury** — Tier 1 · OAUTH2 · 10 tools
- **mip-cloud** — Tier 2 · TWO_STEP · 10 tools
- **mip-on-premise** — Tier 2 · TWO_STEP · 10 tools
- **netsuite** — Tier 1 · OAUTH2 · 10 tools
- **netsuite-tba** — Tier 3 · TBA · 10 tools
- **pennylane** — Tier 1 · OAUTH2 · 10 tools
- **pennylane-company-api** — Tier 1 · API_KEY · 10 tools
- **quickbooks** — Tier 1 · OAUTH2 · 10 tools
- **quickbooks-sandbox** — Tier 3 · UNKNOWN · 10 tools
- **sage** — Tier 1 · OAUTH2 · 10 tools
- **sage-intacct** — Tier 2 · TWO_STEP · 10 tools
- **sage-intacct-oauth** — Tier 1 · OAUTH2 · 10 tools
- **schwab** — Tier 1 · OAUTH2 · 10 tools
- **sellsy** — Tier 1 · OAUTH2 · 10 tools
- **sellsy-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **twinfield** — Tier 1 · OAUTH2 · 10 tools
- **unanet** — Tier 1 · BASIC · 10 tools
- **wave-accounting** — Tier 1 · OAUTH2 · 10 tools
- **workday-adaptive-planning** — Tier 2 · TWO_STEP · 10 tools
- **workday-adaptive-planning-basic** — Tier 1 · BASIC · 10 tools
- **xero** — Tier 1 · OAUTH2 · 10 tools
- **xero-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **zoho** — Tier 1 · OAUTH2 · 10 tools
- **zoho-books** — Tier 3 · UNKNOWN · 10 tools

### analytics (36 providers)

- **amplitude** — Tier 1 · BASIC · 10 tools
- **apify** — Tier 1 · API_KEY · 10 tools
- **axiom** — Tier 1 · API_KEY · 10 tools
- **builtwith** — Tier 1 · API_KEY · 10 tools
- **chattermill** — Tier 1 · API_KEY · 10 tools
- **chorus** — Tier 1 · API_KEY · 10 tools
- **databricks-account** — Tier 2 · OAUTH2_CC · 10 tools
- **databricks-workspace** — Tier 2 · OAUTH2_CC · 10 tools
- **datadog** — Tier 1 · API_KEY · 10 tools
- **exa** — Tier 1 · API_KEY · 10 tools
- **fairing** — Tier 1 · API_KEY · 10 tools
- **fiber-ai** — Tier 1 · API_KEY · 10 tools
- **fireflies** — Tier 1 · API_KEY · 10 tools
- **google-analytics** — Tier 3 · UNKNOWN · 10 tools
- **looker** — Tier 2 · TWO_STEP · 10 tools
- **looker-oauth** — Tier 1 · OAUTH2 · 10 tools
- **metabase** — Tier 1 · API_KEY · 10 tools
- **microsoft-excel** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-planner** — Tier 3 · UNKNOWN · 10 tools
- **mixpanel** — Tier 1 · BASIC · 10 tools
- **nyne-ai** — Tier 1 · API_KEY · 10 tools
- **pendo** — Tier 1 · API_KEY · 10 tools
- **pendo-oauth** — Tier 2 · OAUTH2_CC · 10 tools
- **peopledatalabs** — Tier 1 · API_KEY · 10 tools
- **perdoo** — Tier 1 · API_KEY · 10 tools
- **segment** — Tier 1 · OAUTH2 · 10 tools
- **semrush** — Tier 1 · API_KEY · 10 tools
- **sentry** — Tier 1 · API_KEY · 10 tools
- **sentry-oauth** — Tier 1 · OAUTH2 · 10 tools
- **statista** — Tier 1 · API_KEY · 10 tools
- **tableau** — Tier 2 · TWO_STEP · 10 tools
- **tapclicks** — Tier 2 · OAUTH2_CC · 10 tools
- **triple-whale** — Tier 1 · API_KEY · 10 tools
- **typefully** — Tier 1 · API_KEY · 10 tools
- **typefully-v2** — Tier 1 · API_KEY · 10 tools
- **wakatime** — Tier 1 · OAUTH2 · 10 tools

### ats (25 providers)

- **gem** — Tier 1 · API_KEY · 10 tools
- **greenhouse** — Tier 1 · OAUTH2 · 10 tools
- **greenhouse-assessment** — Tier 1 · BASIC · 10 tools
- **greenhouse-basic** — Tier 1 · BASIC · 10 tools
- **greenhouse-harvest** — Tier 1 · BASIC · 10 tools
- **greenhouse-harvest-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **greenhouse-harvest-partner** — Tier 1 · OAUTH2 · 10 tools
- **greenhouse-ingestion** — Tier 1 · OAUTH2 · 10 tools
- **greenhouse-job-board** — Tier 1 · BASIC · 10 tools
- **greenhouse-onboarding** — Tier 1 · BASIC · 10 tools
- **hackerrank-work** — Tier 1 · BASIC · 10 tools
- **jobdiva** — Tier 2 · TWO_STEP · 10 tools
- **jobvite** — Tier 1 · API_KEY · 10 tools
- **lever** — Tier 1 · OAUTH2 · 10 tools
- **lever-basic** — Tier 1 · BASIC · 10 tools
- **lever-basic-sandbox** — Tier 1 · BASIC · 10 tools
- **lever-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **linkedin** — Tier 1 · OAUTH2 · 10 tools
- **recruitee** — Tier 1 · API_KEY · 10 tools
- **smartrecruiters-api-key** — Tier 1 · API_KEY · 10 tools
- **teamtailor** — Tier 1 · API_KEY · 10 tools
- **trakstar-hire** — Tier 1 · BASIC · 10 tools
- **workable** — Tier 1 · API_KEY · 10 tools
- **workable-oauth** — Tier 1 · OAUTH2 · 10 tools
- **zoho-recruit** — Tier 3 · UNKNOWN · 10 tools

### banking (11 providers)

- **brex** — Tier 1 · OAUTH2 · 10 tools
- **brex-api-key** — Tier 1 · API_KEY · 10 tools
- **brex-staging** — Tier 1 · OAUTH2 · 10 tools
- **fiserv** — Tier 2 · OAUTH2_CC · 10 tools
- **fiserv-api-key** — Tier 1 · API_KEY · 10 tools
- **pennylane** — Tier 1 · OAUTH2 · 10 tools
- **pennylane-company-api** — Tier 1 · API_KEY · 10 tools
- **ramp** — Tier 1 · OAUTH2 · 10 tools
- **ramp-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **schwab** — Tier 1 · OAUTH2 · 10 tools
- **wise-api-key** — Tier 1 · API_KEY · 10 tools

### cms (10 providers)

- **builder-io-private** — Tier 1 · API_KEY · 10 tools
- **builder-io-public** — Tier 1 · API_KEY · 10 tools
- **contentful** — Tier 1 · OAUTH2 · 10 tools
- **contentstack** — Tier 1 · OAUTH2 · 10 tools
- **drupal** — Tier 2 · TWO_STEP · 10 tools
- **ghost-admin** — Tier 3 · JWT · 10 tools
- **ghost-content** — Tier 1 · API_KEY · 10 tools
- **paligo** — Tier 1 · BASIC · 10 tools
- **webflow** — Tier 1 · OAUTH2 · 10 tools
- **wordpress** — Tier 1 · OAUTH2 · 10 tools

### communication (56 providers)

- **active-campaign** — Tier 1 · API_KEY · 10 tools
- **beehiiv** — Tier 1 · API_KEY · 10 tools
- **bird** — Tier 1 · API_KEY · 10 tools
- **bliro** — Tier 2 · OAUTH2_CC · 10 tools
- **braze** — Tier 1 · API_KEY · 10 tools
- **circle-so** — Tier 1 · API_KEY · 10 tools
- **clicksend** — Tier 1 · BASIC · 10 tools
- **cloudtalk** — Tier 1 · BASIC · 10 tools
- **crisp** — Tier 1 · BASIC · 10 tools
- **crisp-plugin-install** — Tier 3 · INSTALL_PLUGIN · 10 tools
- **dialpad** — Tier 1 · OAUTH2 · 10 tools
- **dialpad-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **discourse** — Tier 1 · API_KEY · 10 tools
- **fathom** — Tier 1 · API_KEY · 10 tools
- **fathom-oauth** — Tier 1 · OAUTH2 · 10 tools
- **fireflies** — Tier 1 · API_KEY · 10 tools
- **glyphic** — Tier 1 · API_KEY · 10 tools
- **gong** — Tier 1 · BASIC · 10 tools
- **gong-oauth** — Tier 1 · OAUTH2 · 10 tools
- **google** — Tier 1 · OAUTH2 · 10 tools
- **google-meet** — Tier 3 · UNKNOWN · 10 tools
- **grain** — Tier 1 · OAUTH2 · 10 tools
- **grain-api-key** — Tier 1 · API_KEY · 10 tools
- **heymarket** — Tier 3 · JWT · 10 tools
- **instantly** — Tier 1 · API_KEY · 10 tools
- **lemlist** — Tier 1 · BASIC · 10 tools
- **loops-so** — Tier 1 · API_KEY · 10 tools
- **microsoft** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-admin** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **microsoft-teams-bot** — Tier 1 · OAUTH2 · 10 tools
- **mimecast** — Tier 2 · OAUTH2_CC · 10 tools
- **outlook** — Tier 3 · UNKNOWN · 10 tools
- **podium** — Tier 1 · OAUTH2 · 10 tools
- **reply-io** — Tier 1 · API_KEY · 10 tools
- **resend** — Tier 1 · API_KEY · 10 tools
- **ring-central-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **ringover** — Tier 1 · API_KEY · 10 tools
- **salesmsg** — Tier 2 · TWO_STEP · 10 tools
- **salesmsg-oauth2** — Tier 1 · OAUTH2 · 10 tools
- **sedna** — Tier 2 · OAUTH2_CC · 10 tools
- **sedna-basic** — Tier 1 · BASIC · 10 tools
- **sharepoint-online** — Tier 3 · UNKNOWN · 10 tools
- **sharepoint-online-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **sharepoint-online-v1** — Tier 2 · TWO_STEP · 10 tools
- **slack-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **smartlead-ai** — Tier 1 · API_KEY · 10 tools
- **telegram** — Tier 1 · API_KEY · 10 tools
- **tumblr** — Tier 1 · OAUTH2 · 10 tools
- **twilio** — Tier 1 · BASIC · 10 tools
- **typefully** — Tier 1 · API_KEY · 10 tools
- **typefully-v2** — Tier 1 · API_KEY · 10 tools
- **webex** — Tier 1 · OAUTH2 · 10 tools
- **whatsapp-business** — Tier 1 · API_KEY · 10 tools
- **wildix-pbx** — Tier 1 · OAUTH2 · 10 tools
- **zoho-mail** — Tier 3 · UNKNOWN · 10 tools

### crm (60 providers)

- **affinity** — Tier 1 · BASIC · 10 tools
- **attio** — Tier 1 · OAUTH2 · 10 tools
- **bettercontact** — Tier 1 · API_KEY · 10 tools
- **blackbaud** — Tier 1 · OAUTH2 · 10 tools
- **blackbaud-basic** — Tier 1 · BASIC · 10 tools
- **buildium** — Tier 1 · API_KEY · 10 tools
- **builtwith** — Tier 1 · API_KEY · 10 tools
- **clay** — Tier 1 · API_KEY · 10 tools
- **close** — Tier 1 · OAUTH2 · 10 tools
- **copper** — Tier 1 · OAUTH2 · 10 tools
- **copper-api-key** — Tier 1 · API_KEY · 10 tools
- **findymail** — Tier 1 · API_KEY · 10 tools
- **firefish** — Tier 2 · OAUTH2_CC · 10 tools
- **folk** — Tier 1 · API_KEY · 10 tools
- **freshsales** — Tier 1 · API_KEY · 10 tools
- **fullenrich** — Tier 1 · API_KEY · 10 tools
- **gainsight-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **highlevel-white-label** — Tier 1 · OAUTH2 · 10 tools
- **holded** — Tier 1 · API_KEY · 10 tools
- **hubspot** — Tier 1 · OAUTH2 · 10 tools
- **hubspot-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **icypeas** — Tier 1 · API_KEY · 10 tools
- **insightly** — Tier 1 · BASIC · 10 tools
- **jobber** — Tier 1 · OAUTH2 · 10 tools
- **kustomer** — Tier 1 · API_KEY · 10 tools
- **leadmagic** — Tier 1 · API_KEY · 10 tools
- **manatal** — Tier 1 · API_KEY · 10 tools
- **maximizer** — Tier 1 · OAUTH2 · 10 tools
- **maximizer-on-premise** — Tier 1 · OAUTH2 · 10 tools
- **medallia** — Tier 2 · OAUTH2_CC · 10 tools
- **nationbuilder** — Tier 1 · OAUTH2 · 10 tools
- **nocrm** — Tier 1 · API_KEY · 10 tools
- **pipedrive** — Tier 1 · OAUTH2 · 10 tools
- **precisefp** — Tier 1 · OAUTH2 · 10 tools
- **prospeo** — Tier 1 · API_KEY · 10 tools
- **reapit** — Tier 1 · OAUTH2 · 10 tools
- **redtail-crm-sandbox** — Tier 2 · TWO_STEP · 10 tools
- **rock-gym-pro** — Tier 1 · BASIC · 10 tools
- **salesforce** — Tier 1 · OAUTH2 · 10 tools
- **salesforce-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **salesforce-experience-cloud** — Tier 1 · OAUTH2 · 10 tools
- **salesforce-jwt** — Tier 2 · TWO_STEP · 10 tools
- **salesforce-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **sellsy** — Tier 1 · OAUTH2 · 10 tools
- **sellsy-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **streak** — Tier 1 · BASIC · 10 tools
- **teamleader** — Tier 1 · OAUTH2 · 10 tools
- **twenty-crm** — Tier 1 · API_KEY · 10 tools
- **twenty-crm-self-hosted** — Tier 1 · API_KEY · 10 tools
- **unanet** — Tier 1 · BASIC · 10 tools
- **unipile** — Tier 1 · API_KEY · 10 tools
- **upsales** — Tier 1 · API_KEY · 10 tools
- **veeva-vault** — Tier 2 · TWO_STEP · 10 tools
- **wealthbox** — Tier 1 · OAUTH2 · 10 tools
- **wiseagent** — Tier 1 · OAUTH2 · 10 tools
- **wiza** — Tier 1 · API_KEY · 10 tools
- **zendesk-sell** — Tier 1 · OAUTH2 · 10 tools
- **zoho-bigin** — Tier 3 · UNKNOWN · 10 tools
- **zoho-crm** — Tier 3 · UNKNOWN · 10 tools
- **zoominfo** — Tier 2 · OAUTH2_CC · 10 tools

### design (22 providers)

- **adobe** — Tier 1 · OAUTH2 · 10 tools
- **autodesk** — Tier 1 · OAUTH2 · 10 tools
- **builder-io-private** — Tier 1 · API_KEY · 10 tools
- **builder-io-public** — Tier 1 · API_KEY · 10 tools
- **canva** — Tier 1 · OAUTH2 · 10 tools
- **canva-scim** — Tier 1 · API_KEY · 10 tools
- **contentful** — Tier 1 · OAUTH2 · 10 tools
- **drupal** — Tier 2 · TWO_STEP · 10 tools
- **figjam** — Tier 3 · UNKNOWN · 10 tools
- **figma** — Tier 1 · OAUTH2 · 10 tools
- **figma-scim** — Tier 1 · API_KEY · 10 tools
- **ghost-admin** — Tier 3 · JWT · 10 tools
- **ghost-content** — Tier 1 · API_KEY · 10 tools
- **gumroad** — Tier 1 · OAUTH2 · 10 tools
- **miro** — Tier 1 · OAUTH2 · 10 tools
- **miro-scim** — Tier 1 · API_KEY · 10 tools
- **mural** — Tier 1 · OAUTH2 · 10 tools
- **pinterest** — Tier 1 · OAUTH2 · 10 tools
- **squarespace** — Tier 1 · OAUTH2 · 10 tools
- **webflow** — Tier 1 · OAUTH2 · 10 tools
- **woocommerce** — Tier 1 · BASIC · 10 tools
- **wordpress** — Tier 1 · OAUTH2 · 10 tools

### dev-tools (96 providers)

- **anthropic** — Tier 1 · API_KEY · 10 tools
- **apify** — Tier 1 · API_KEY · 10 tools
- **aws** — Tier 1 · OAUTH2 · 10 tools
- **aws-iam** — Tier 1 · BASIC · 10 tools
- **aws-scim** — Tier 1 · API_KEY · 10 tools
- **axiom** — Tier 1 · API_KEY · 10 tools
- **azure-devops** — Tier 1 · BASIC · 10 tools
- **bing-webmasters** — Tier 1 · OAUTH2 · 10 tools
- **bitbucket** — Tier 1 · OAUTH2 · 10 tools
- **builder-io-private** — Tier 1 · API_KEY · 10 tools
- **builder-io-public** — Tier 1 · API_KEY · 10 tools
- **builtwith** — Tier 1 · API_KEY · 10 tools
- **canva-scim** — Tier 1 · API_KEY · 10 tools
- **clerk** — Tier 1 · API_KEY · 10 tools
- **clickhouse** — Tier 1 · BASIC · 10 tools
- **cloudflare** — Tier 1 · API_KEY · 10 tools
- **codeclimate** — Tier 1 · API_KEY · 10 tools
- **codegen** — Tier 1 · API_KEY · 10 tools
- **contentful** — Tier 1 · OAUTH2 · 10 tools
- **crowdstrike** — Tier 2 · OAUTH2_CC · 10 tools
- **cursor** — Tier 1 · API_KEY · 10 tools
- **cursor-admin** — Tier 1 · BASIC · 10 tools
- **datadog** — Tier 1 · API_KEY · 10 tools
- **devin** — Tier 1 · API_KEY · 10 tools
- **digitalocean** — Tier 1 · OAUTH2 · 10 tools
- **drata** — Tier 1 · API_KEY · 10 tools
- **drupal** — Tier 2 · TWO_STEP · 10 tools
- **elevenlabs** — Tier 1 · API_KEY · 10 tools
- **falai** — Tier 1 · API_KEY · 10 tools
- **gerrit** — Tier 1 · BASIC · 10 tools
- **ghost-admin** — Tier 3 · JWT · 10 tools
- **ghost-content** — Tier 1 · API_KEY · 10 tools
- **github** — Tier 1 · OAUTH2 · 10 tools
- **github-app** — Tier 3 · APP · 10 tools
- **github-app-oauth** — Tier 3 · CUSTOM · 10 tools
- **github-pat** — Tier 1 · API_KEY · 10 tools
- **gitlab** — Tier 1 · OAUTH2 · 10 tools
- **gitlab-pat** — Tier 1 · API_KEY · 10 tools
- **google** — Tier 1 · OAUTH2 · 10 tools
- **google-gemini** — Tier 1 · API_KEY · 10 tools
- **google-play** — Tier 1 · OAUTH2 · 10 tools
- **google-service-account** — Tier 2 · TWO_STEP · 10 tools
- **grafana** — Tier 1 · API_KEY · 10 tools
- **juniper-mist** — Tier 1 · API_KEY · 10 tools
- **klipfolio** — Tier 1 · API_KEY · 10 tools
- **microsoft** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-admin** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **minimax** — Tier 1 · API_KEY · 10 tools
- **okta** — Tier 1 · OAUTH2 · 10 tools
- **okta-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **okta-preview** — Tier 3 · UNKNOWN · 10 tools
- **onelogin** — Tier 2 · OAUTH2_CC · 10 tools
- **open-hands** — Tier 1 · API_KEY · 10 tools
- **openai** — Tier 1 · API_KEY · 10 tools
- **openai-admin** — Tier 1 · API_KEY · 10 tools
- **oracle-cloud-identity** — Tier 2 · OAUTH2_CC · 10 tools
- **pagerduty** — Tier 1 · OAUTH2 · 10 tools
- **perplexity** — Tier 1 · API_KEY · 10 tools
- **pingone** — Tier 1 · OAUTH2 · 10 tools
- **pingone-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **pipedream** — Tier 1 · API_KEY · 10 tools
- **pipedream-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **posthog** — Tier 1 · API_KEY · 11 tools
- **ragieai** — Tier 1 · API_KEY · 10 tools
- **rapidapi** — Tier 1 · API_KEY · 10 tools
- **recall-ai** — Tier 1 · API_KEY · 10 tools
- **replicate** — Tier 1 · API_KEY · 10 tools
- **retell-ai** — Tier 1 · API_KEY · 10 tools
- **rootly** — Tier 1 · API_KEY · 10 tools
- **sentinelone** — Tier 1 · API_KEY · 10 tools
- **sentry** — Tier 1 · API_KEY · 10 tools
- **sentry-oauth** — Tier 1 · OAUTH2 · 10 tools
- **shortcut** — Tier 1 · API_KEY · 10 tools
- **snowflake** — Tier 1 · OAUTH2 · 10 tools
- **snowflake-jwt** — Tier 3 · JWT · 10 tools
- **sophos-central** — Tier 2 · OAUTH2_CC · 10 tools
- **squarespace** — Tier 1 · OAUTH2 · 10 tools
- **supabase** — Tier 1 · API_KEY · 10 tools
- **supabase-mcp** — Tier 1 · API_KEY · 10 tools
- **supabase-mcp-oauth** — Tier 3 · MCP_OAUTH2 · 10 tools
- **tailscale** — Tier 2 · TWO_STEP · 10 tools
- **tailscale-api-key** — Tier 1 · API_KEY · 10 tools
- **terraform** — Tier 1 · API_KEY · 10 tools
- **trigify-io-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **twilio** — Tier 1 · BASIC · 10 tools
- **vanta** — Tier 2 · OAUTH2_CC · 10 tools
- **vercel** — Tier 1 · API_KEY · 10 tools
- **vercel-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **wakatime** — Tier 1 · OAUTH2 · 10 tools
- **webflow** — Tier 1 · OAUTH2 · 10 tools
- **woocommerce** — Tier 1 · BASIC · 10 tools
- **wordpress** — Tier 1 · OAUTH2 · 10 tools
- **xai** — Tier 1 · API_KEY · 10 tools
- **zapier** — Tier 1 · OAUTH2 · 10 tools
- **zapier-scim** — Tier 1 · API_KEY · 10 tools

### e-commerce (33 providers)

- **aws** — Tier 1 · OAUTH2 · 10 tools
- **bigcommerce** — Tier 1 · OAUTH2 · 10 tools
- **booking-com** — Tier 1 · BASIC · 10 tools
- **builtwith** — Tier 1 · API_KEY · 10 tools
- **cin7-core** — Tier 1 · API_KEY · 10 tools
- **clover** — Tier 1 · OAUTH2 · 10 tools
- **commercetools** — Tier 2 · OAUTH2_CC · 10 tools
- **ebay** — Tier 1 · OAUTH2 · 10 tools
- **ebay-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **gorgias** — Tier 1 · OAUTH2 · 10 tools
- **gorgias-basic** — Tier 1 · BASIC · 10 tools
- **gumroad** — Tier 1 · OAUTH2 · 10 tools
- **lightspeed-retail** — Tier 1 · OAUTH2 · 10 tools
- **loop-returns** — Tier 1 · API_KEY · 10 tools
- **printful** — Tier 1 · OAUTH2 · 10 tools
- **prive** — Tier 1 · API_KEY · 10 tools
- **recharge** — Tier 1 · API_KEY · 10 tools
- **sellercloud** — Tier 2 · TWO_STEP · 10 tools
- **shipbob-pat** — Tier 1 · API_KEY · 10 tools
- **shipstation** — Tier 1 · BASIC · 10 tools
- **shipstation-v2** — Tier 1 · API_KEY · 10 tools
- **shopify** — Tier 1 · OAUTH2 · 10 tools
- **shopify-api-key** — Tier 1 · API_KEY · 10 tools
- **shopify-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **shopify-partner** — Tier 1 · API_KEY · 10 tools
- **shopify-scim** — Tier 1 · API_KEY · 10 tools
- **skio** — Tier 1 · API_KEY · 10 tools
- **stay-ai** — Tier 1 · API_KEY · 10 tools
- **thrivecart-api-key** — Tier 1 · API_KEY · 10 tools
- **thrivecart-oauth** — Tier 1 · OAUTH2 · 10 tools
- **woocommerce** — Tier 1 · BASIC · 10 tools
- **yotpo** — Tier 2 · TWO_STEP · 10 tools
- **zoho-inventory** — Tier 3 · UNKNOWN · 10 tools

### erp (19 providers)

- **apaleo** — Tier 1 · OAUTH2 · 10 tools
- **exact-online** — Tier 1 · OAUTH2 · 10 tools
- **gebruder-weiss** — Tier 2 · OAUTH2_CC · 10 tools
- **microsoft-business-central** — Tier 2 · OAUTH2_CC · 10 tools
- **microsoft-tenant-specific** — Tier 1 · OAUTH2 · 10 tools
- **netsuite** — Tier 1 · OAUTH2 · 10 tools
- **netsuite-tba** — Tier 3 · TBA · 10 tools
- **odoo** — Tier 1 · OAUTH2 · 10 tools
- **odoo-cc** — Tier 2 · TWO_STEP · 10 tools
- **procore** — Tier 1 · OAUTH2 · 10 tools
- **sage** — Tier 1 · OAUTH2 · 10 tools
- **sage-intacct** — Tier 2 · TWO_STEP · 10 tools
- **sage-intacct-oauth** — Tier 1 · OAUTH2 · 10 tools
- **sap-business-one** — Tier 2 · TWO_STEP · 10 tools
- **sap-concur** — Tier 2 · OAUTH2_CC · 10 tools
- **sap-odata-basic** — Tier 1 · BASIC · 10 tools
- **sap-odata-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **unanet** — Tier 1 · BASIC · 10 tools
- **zuora** — Tier 2 · OAUTH2_CC · 10 tools

### gaming (5 providers)

- **battlenet** — Tier 1 · OAUTH2 · 10 tools
- **discord** — Tier 1 · OAUTH2 · 10 tools
- **epic-games** — Tier 1 · OAUTH2 · 10 tools
- **osu** — Tier 1 · OAUTH2 · 10 tools
- **twitch** — Tier 1 · OAUTH2 · 10 tools

### hr (53 providers)

- **bamboohr** — Tier 1 · OAUTH2 · 10 tools
- **bamboohr-basic** — Tier 1 · BASIC · 10 tools
- **breezy-hr** — Tier 2 · TWO_STEP · 10 tools
- **bullhorn** — Tier 1 · OAUTH2 · 10 tools
- **datev** — Tier 1 · OAUTH2 · 10 tools
- **dayforce** — Tier 2 · TWO_STEP · 10 tools
- **deel** — Tier 1 · OAUTH2 · 10 tools
- **deel-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **employment-hero** — Tier 1 · OAUTH2 · 10 tools
- **factorial** — Tier 1 · OAUTH2 · 10 tools
- **firstbase** — Tier 1 · API_KEY · 10 tools
- **freshteam** — Tier 1 · API_KEY · 10 tools
- **grammarly-scim** — Tier 1 · API_KEY · 10 tools
- **gusto** — Tier 1 · OAUTH2 · 10 tools
- **gusto-demo** — Tier 1 · OAUTH2 · 10 tools
- **hibob-service-user** — Tier 1 · BASIC · 10 tools
- **jazzhr** — Tier 1 · API_KEY · 10 tools
- **jobadder** — Tier 1 · OAUTH2 · 10 tools
- **jobdiva** — Tier 2 · TWO_STEP · 10 tools
- **justworks** — Tier 1 · OAUTH2 · 10 tools
- **lattice** — Tier 1 · API_KEY · 10 tools
- **manatal** — Tier 1 · API_KEY · 10 tools
- **namely** — Tier 1 · OAUTH2 · 10 tools
- **namely-pat** — Tier 1 · API_KEY · 10 tools
- **oracle-hcm** — Tier 1 · BASIC · 10 tools
- **paychex** — Tier 2 · OAUTH2_CC · 10 tools
- **paycom** — Tier 1 · BASIC · 10 tools
- **paycor** — Tier 1 · OAUTH2 · 10 tools
- **paycor-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **payfit** — Tier 1 · OAUTH2 · 10 tools
- **paylocity** — Tier 2 · OAUTH2_CC · 10 tools
- **paylocity-nextgen** — Tier 2 · OAUTH2_CC · 10 tools
- **personio** — Tier 2 · OAUTH2_CC · 10 tools
- **personio-recruiting** — Tier 1 · API_KEY · 10 tools
- **personio-v2** — Tier 2 · OAUTH2_CC · 10 tools
- **recruitcrm** — Tier 1 · API_KEY · 10 tools
- **recruiterflow** — Tier 1 · API_KEY · 10 tools
- **rippling** — Tier 1 · API_KEY · 10 tools
- **rippling-shop-app** — Tier 1 · OAUTH2 · 10 tools
- **sage-hr** — Tier 1 · API_KEY · 10 tools
- **sage-people** — Tier 1 · OAUTH2 · 10 tools
- **sap-fieldglass** — Tier 2 · TWO_STEP · 10 tools
- **sap-success-factors** — Tier 2 · TWO_STEP · 10 tools
- **tsheetsteam** — Tier 1 · OAUTH2 · 10 tools
- **ukg-pro** — Tier 1 · BASIC · 10 tools
- **ukg-pro-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **ukg-pro-wfm** — Tier 1 · OAUTH2 · 10 tools
- **ukg-ready** — Tier 2 · OAUTH2_CC · 10 tools
- **workday** — Tier 1 · BASIC · 10 tools
- **workday-oauth** — Tier 1 · OAUTH2 · 10 tools
- **workday-refresh-token** — Tier 2 · TWO_STEP · 10 tools
- **zenefits** — Tier 1 · OAUTH2 · 10 tools
- **zoho-people** — Tier 3 · UNKNOWN · 10 tools

### invoicing (14 providers)

- **accelo** — Tier 1 · OAUTH2 · 10 tools
- **coupa-compass** — Tier 2 · OAUTH2_CC · 10 tools
- **fortnox** — Tier 1 · OAUTH2 · 10 tools
- **freeagent** — Tier 1 · OAUTH2 · 10 tools
- **freeagent-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **holded** — Tier 1 · API_KEY · 10 tools
- **jobber** — Tier 1 · OAUTH2 · 10 tools
- **pennylane** — Tier 1 · OAUTH2 · 10 tools
- **pennylane-company-api** — Tier 1 · API_KEY · 10 tools
- **pleo** — Tier 1 · OAUTH2 · 10 tools
- **sellsy** — Tier 1 · OAUTH2 · 10 tools
- **sellsy-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **teamleader** — Tier 1 · OAUTH2 · 10 tools
- **zoho-invoice** — Tier 3 · UNKNOWN · 10 tools

### knowledge-base (20 providers)

- **box** — Tier 1 · OAUTH2 · 10 tools
- **coda** — Tier 1 · API_KEY · 10 tools
- **confluence** — Tier 3 · UNKNOWN · 10 tools
- **confluence-basic** — Tier 1 · BASIC · 10 tools
- **document360** — Tier 1 · API_KEY · 10 tools
- **dropbox** — Tier 1 · OAUTH2 · 10 tools
- **elevio** — Tier 1 · API_KEY · 10 tools
- **google-drive** — Tier 3 · UNKNOWN · 10 tools
- **granola** — Tier 1 · API_KEY · 10 tools
- **granola-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **guru** — Tier 1 · BASIC · 10 tools
- **guru-scim** — Tier 1 · BASIC · 10 tools
- **helpscout-docs** — Tier 1 · BASIC · 10 tools
- **notion** — Tier 1 · OAUTH2 · 10 tools
- **notion-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **notion-scim** — Tier 1 · API_KEY · 10 tools
- **one-drive** — Tier 3 · UNKNOWN · 10 tools
- **one-drive-personal** — Tier 1 · OAUTH2 · 10 tools
- **slab** — Tier 1 · API_KEY · 10 tools
- **stackexchange** — Tier 1 · OAUTH2 · 10 tools

### legal (18 providers)

- **avalara** — Tier 1 · BASIC · 10 tools
- **avalara-sandbox** — Tier 1 · BASIC · 10 tools
- **boldsign** — Tier 1 · OAUTH2 · 10 tools
- **certn** — Tier 1 · API_KEY · 10 tools
- **certn-partner** — Tier 2 · OAUTH2_CC · 10 tools
- **checkr-partner** — Tier 1 · OAUTH2 · 10 tools
- **checkr-partner-staging** — Tier 1 · OAUTH2 · 10 tools
- **clio** — Tier 1 · OAUTH2 · 10 tools
- **datev** — Tier 1 · OAUTH2 · 10 tools
- **docusign** — Tier 1 · OAUTH2 · 10 tools
- **docusign-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **dropbox-sign** — Tier 1 · OAUTH2 · 10 tools
- **ironclad** — Tier 1 · OAUTH2 · 10 tools
- **pandadoc** — Tier 1 · OAUTH2 · 10 tools
- **pandadoc-api-key** — Tier 1 · API_KEY · 10 tools
- **qualia** — Tier 1 · BASIC · 10 tools
- **signnow** — Tier 1 · OAUTH2 · 10 tools
- **signnow-sandbox** — Tier 1 · OAUTH2 · 10 tools

### marketing (72 providers)

- **active-campaign** — Tier 1 · API_KEY · 10 tools
- **apollo** — Tier 1 · API_KEY · 10 tools
- **beehiiv** — Tier 1 · API_KEY · 10 tools
- **bitly** — Tier 1 · OAUTH2 · 10 tools
- **brevo-api-key** — Tier 1 · API_KEY · 10 tools
- **builtwith** — Tier 1 · API_KEY · 10 tools
- **callrail** — Tier 1 · API_KEY · 10 tools
- **clari-copilot** — Tier 1 · API_KEY · 10 tools
- **clay** — Tier 1 · API_KEY · 10 tools
- **cleverreach** — Tier 1 · OAUTH2 · 10 tools
- **cyberimpact** — Tier 1 · API_KEY · 10 tools
- **emarsys** — Tier 3 · SIGNATURE · 10 tools
- **emarsys-oauth** — Tier 2 · OAUTH2_CC · 10 tools
- **eventbrite** — Tier 1 · OAUTH2 · 10 tools
- **facebook** — Tier 1 · OAUTH2 · 10 tools
- **fairing** — Tier 1 · API_KEY · 10 tools
- **findymail** — Tier 1 · API_KEY · 10 tools
- **gong** — Tier 1 · BASIC · 10 tools
- **gong-oauth** — Tier 1 · OAUTH2 · 10 tools
- **google-ads** — Tier 3 · UNKNOWN · 10 tools
- **heymarket** — Tier 3 · JWT · 10 tools
- **heyreach** — Tier 1 · API_KEY · 10 tools
- **highlevel** — Tier 1 · OAUTH2 · 10 tools
- **hubspot** — Tier 1 · OAUTH2 · 10 tools
- **hubspot-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **icypeas** — Tier 1 · API_KEY · 10 tools
- **instagram** — Tier 1 · OAUTH2 · 10 tools
- **instantly** — Tier 1 · API_KEY · 10 tools
- **intercom** — Tier 1 · OAUTH2 · 10 tools
- **keap** — Tier 1 · OAUTH2 · 10 tools
- **klaviyo** — Tier 1 · API_KEY · 10 tools
- **klaviyo-oauth** — Tier 1 · OAUTH2 · 10 tools
- **klicktipp** — Tier 1 · API_KEY · 10 tools
- **lagrowthmachine** — Tier 1 · API_KEY · 10 tools
- **leadmagic** — Tier 1 · API_KEY · 10 tools
- **lemlist** — Tier 1 · BASIC · 10 tools
- **listmonk** — Tier 1 · BASIC · 10 tools
- **listrak** — Tier 2 · OAUTH2_CC · 10 tools
- **lob** — Tier 1 · BASIC · 10 tools
- **loops-so** — Tier 1 · API_KEY · 10 tools
- **mailchimp** — Tier 1 · OAUTH2 · 10 tools
- **mailgun** — Tier 1 · BASIC · 10 tools
- **mailjet** — Tier 1 · BASIC · 10 tools
- **marketo** — Tier 2 · OAUTH2_CC · 10 tools
- **meta-marketing-api** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-ads** — Tier 3 · UNKNOWN · 10 tools
- **outreach** — Tier 1 · OAUTH2 · 10 tools
- **pinterest** — Tier 1 · OAUTH2 · 10 tools
- **podium** — Tier 1 · OAUTH2 · 10 tools
- **prospeo** — Tier 1 · API_KEY · 10 tools
- **provenexpert** — Tier 1 · BASIC · 10 tools
- **quentn** — Tier 1 · API_KEY · 10 tools
- **reply-io** — Tier 1 · API_KEY · 10 tools
- **salesloft** — Tier 1 · OAUTH2 · 10 tools
- **salesmsg** — Tier 2 · TWO_STEP · 10 tools
- **salesmsg-oauth2** — Tier 1 · OAUTH2 · 10 tools
- **segment** — Tier 1 · OAUTH2 · 10 tools
- **semrush** — Tier 1 · API_KEY · 10 tools
- **sendgrid** — Tier 1 · API_KEY · 10 tools
- **smartlead-ai** — Tier 1 · API_KEY · 10 tools
- **tapclicks** — Tier 2 · OAUTH2_CC · 10 tools
- **triple-whale** — Tier 1 · API_KEY · 10 tools
- **twitter** — Tier 3 · OAUTH1 · 10 tools
- **twitter-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **twitter-v2** — Tier 1 · OAUTH2 · 10 tools
- **unipile** — Tier 1 · API_KEY · 10 tools
- **valley** — Tier 2 · TWO_STEP · 10 tools
- **valley-api-key** — Tier 1 · API_KEY · 10 tools
- **webinarjam** — Tier 1 · API_KEY · 10 tools
- **whatsapp-business** — Tier 1 · API_KEY · 10 tools
- **wiza** — Tier 1 · API_KEY · 10 tools
- **zoominfo** — Tier 2 · OAUTH2_CC · 10 tools

### mcp (11 providers)

- **circleback-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **granola-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **hubspot-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **linear-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **mcp-generic** — Tier 3 · MCP_OAUTH2_GENERIC · 10 tools
- **notion-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **slack-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **supabase-mcp** — Tier 1 · API_KEY · 10 tools
- **supabase-mcp-oauth** — Tier 3 · MCP_OAUTH2 · 10 tools
- **trigify-io-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **vercel-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools

### other (213 providers)

- **17hats** — Tier 3 · UNKNOWN · 10 tools
- **acuity** — Tier 3 · UNKNOWN · 10 tools
- **aha** — Tier 3 · UNKNOWN · 10 tools
- **aiven** — Tier 3 · UNKNOWN · 10 tools
- **akamai** — Tier 3 · UNKNOWN · 10 tools
- **alibaba** — Tier 3 · UNKNOWN · 10 tools
- **anchor** — Tier 3 · UNKNOWN · 10 tools
- **artfol** — Tier 3 · UNKNOWN · 10 tools
- **attach** — Tier 3 · UNKNOWN · 10 tools
- **auth0** — Tier 1 · OAUTH2 · 10 tools
- **autobot** — Tier 3 · UNKNOWN · 10 tools
- **avanan** — Tier 2 · TWO_STEP · 10 tools
- **avaza** — Tier 3 · UNKNOWN · 10 tools
- **azure** — Tier 3 · UNKNOWN · 10 tools
- **azure_devops** — Tier 3 · UNKNOWN · 10 tools
- **baidu** — Tier 3 · UNKNOWN · 10 tools
- **bear** — Tier 3 · UNKNOWN · 10 tools
- **bereal** — Tier 3 · UNKNOWN · 9 tools
- **bilibili** — Tier 3 · UNKNOWN · 10 tools
- **bitdefender** — Tier 1 · BASIC · 10 tools
- **bookly** — Tier 3 · UNKNOWN · 10 tools
- **botpress** — Tier 3 · UNKNOWN · 10 tools
- **bytedance** — Tier 3 · UNKNOWN · 10 tools
- **calendary** — Tier 3 · UNKNOWN · 10 tools
- **calendly_more** — Tier 3 · UNKNOWN · 10 tools
- **charcle** — Tier 3 · UNKNOWN · 10 tools
- **chatfuel** — Tier 3 · UNKNOWN · 10 tools
- **chatwork** — Tier 3 · UNKNOWN · 10 tools
- **cisco-duo-admin** — Tier 1 · BASIC · 10 tools
- **clarity** — Tier 3 · UNKNOWN · 10 tools
- **clerk** — Tier 1 · API_KEY · 10 tools
- **clockify** — Tier 3 · UNKNOWN · 10 tools
- **cloudbeds** — Tier 1 · OAUTH2 · 10 tools
- **cloudentity** — Tier 2 · OAUTH2_CC · 10 tools
- **cloudflare_more** — Tier 3 · UNKNOWN · 10 tools
- **copper_more** — Tier 3 · UNKNOWN · 10 tools
- **coscreen** — Tier 3 · UNKNOWN · 10 tools
- **craft** — Tier 3 · UNKNOWN · 10 tools
- **databricks** — Tier 3 · UNKNOWN · 10 tools
- **datocms** — Tier 3 · UNKNOWN · 10 tools
- **dependabot** — Tier 3 · UNKNOWN · 10 tools
- **domo** — Tier 2 · OAUTH2_CC · 10 tools
- **drchrono** — Tier 1 · OAUTH2 · 10 tools
- **drift** — Tier 3 · UNKNOWN · 10 tools
- **dropbox_more** — Tier 3 · UNKNOWN · 10 tools
- **dropboxpaper** — Tier 3 · UNKNOWN · 10 tools
- **dubsado** — Tier 3 · UNKNOWN · 10 tools
- **duda** — Tier 3 · UNKNOWN · 10 tools
- **ecu360** — Tier 2 · TWO_STEP · 10 tools
- **ecu360-production** — Tier 3 · UNKNOWN · 10 tools
- **elasticsearch** — Tier 3 · UNKNOWN · 10 tools
- **engage** — Tier 3 · UNKNOWN · 10 tools
- **entrata** — Tier 1 · BASIC · 10 tools
- **evernote** — Tier 3 · UNKNOWN · 10 tools
- **exist** — Tier 1 · OAUTH2 · 10 tools
- **fanvue** — Tier 1 · OAUTH2 · 10 tools
- **fastly** — Tier 3 · UNKNOWN · 10 tools
- **figma_more** — Tier 3 · UNKNOWN · 10 tools
- **fireberry** — Tier 3 · UNKNOWN · 10 tools
- **fluidtopics** — Tier 3 · UNKNOWN · 10 tools
- **fly** — Tier 3 · UNKNOWN · 10 tools
- **fogbugz** — Tier 3 · UNKNOWN · 10 tools
- **freepik** — Tier 1 · API_KEY · 10 tools
- **freshdesk_more** — Tier 3 · UNKNOWN · 10 tools
- **freshmarketer** — Tier 3 · UNKNOWN · 10 tools
- **front_more** — Tier 3 · UNKNOWN · 10 tools
- **fullstory** — Tier 3 · UNKNOWN · 10 tools
- **functionize** — Tier 3 · UNKNOWN · 10 tools
- **ghost** — Tier 3 · UNKNOWN · 10 tools
- **gobi** — Tier 3 · UNKNOWN · 10 tools
- **google_workspace** — Tier 3 · UNKNOWN · 11 tools
- **googledrive** — Tier 3 · UNKNOWN · 10 tools
- **gotomeeting** — Tier 3 · UNKNOWN · 10 tools
- **groove** — Tier 3 · UNKNOWN · 10 tools
- **guidaki** — Tier 3 · UNKNOWN · 10 tools
- **health-gorilla** — Tier 1 · OAUTH2 · 10 tools
- **heap** — Tier 2 · TWO_STEP · 10 tools
- **helpscout** — Tier 3 · UNKNOWN · 10 tools
- **heroku** — Tier 3 · UNKNOWN · 10 tools
- **hotjar** — Tier 3 · UNKNOWN · 10 tools
- **hubspot_more** — Tier 3 · UNKNOWN · 10 tools
- **huggingface** — Tier 3 · UNKNOWN · 10 tools
- **ibm** — Tier 3 · UNKNOWN · 10 tools
- **integromat** — Tier 3 · UNKNOWN · 10 tools
- **intercom_more** — Tier 3 · UNKNOWN · 10 tools
- **invision** — Tier 3 · UNKNOWN · 10 tools
- **itglue** — Tier 1 · API_KEY · 10 tools
- **jamf** — Tier 2 · OAUTH2_CC · 10 tools
- **jamf-basic** — Tier 2 · TWO_STEP · 10 tools
- **jumpcloud** — Tier 1 · API_KEY · 10 tools
- **kontent** — Tier 3 · UNKNOWN · 10 tools
- **lamatic** — Tier 3 · UNKNOWN · 10 tools
- **landbot** — Tier 3 · UNKNOWN · 10 tools
- **launchdarkly** — Tier 3 · UNKNOWN · 10 tools
- **levelhead** — Tier 3 · UNKNOWN · 10 tools
- **lightspeed** — Tier 3 · UNKNOWN · 10 tools
- **livechat** — Tier 3 · UNKNOWN · 10 tools
- **livechat_more** — Tier 3 · UNKNOWN · 10 tools
- **loom** — Tier 3 · UNKNOWN · 10 tools
- **loom-scim** — Tier 1 · API_KEY · 10 tools
- **luckyorange** — Tier 3 · UNKNOWN · 10 tools
- **malwarebytes** — Tier 2 · OAUTH2_CC · 10 tools
- **manychat** — Tier 3 · UNKNOWN · 10 tools
- **medium** — Tier 3 · UNKNOWN · 10 tools
- **messagebird** — Tier 3 · UNKNOWN · 10 tools
- **method_crm** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-entra-id** — Tier 3 · UNKNOWN · 10 tools
- **microsoft_365** — Tier 3 · UNKNOWN · 10 tools
- **microsoft_graph** — Tier 3 · UNKNOWN · 10 tools
- **miro_more** — Tier 3 · UNKNOWN · 10 tools
- **mobilemonkey** — Tier 3 · UNKNOWN · 10 tools
- **mode** — Tier 3 · UNKNOWN · 10 tools
- **modmed** — Tier 2 · TWO_STEP · 10 tools
- **mux** — Tier 3 · UNKNOWN · 10 tools
- **n8n** — Tier 3 · UNKNOWN · 10 tools
- **nerdio** — Tier 2 · OAUTH2_CC · 10 tools
- **netlify** — Tier 3 · UNKNOWN · 10 tools
- **nextiva** — Tier 3 · UNKNOWN · 10 tools
- **nimble** — Tier 3 · UNKNOWN · 10 tools
- **note** — Tier 3 · UNKNOWN · 10 tools
- **nutshell** — Tier 3 · UNKNOWN · 10 tools
- **olark** — Tier 3 · UNKNOWN · 10 tools
- **onceup** — Tier 3 · UNKNOWN · 10 tools
- **onedrive** — Tier 3 · UNKNOWN · 10 tools
- **onlogist** — Tier 1 · API_KEY · 10 tools
- **opsgenie** — Tier 3 · UNKNOWN · 10 tools
- **optimizely** — Tier 3 · UNKNOWN · 10 tools
- **oracle** — Tier 3 · UNKNOWN · 10 tools
- **oracle-cloud-identity** — Tier 2 · OAUTH2_CC · 10 tools
- **ory** — Tier 2 · OAUTH2_CC · 10 tools
- **papaya_global** — Tier 3 · UNKNOWN · 10 tools
- **pax8** — Tier 2 · OAUTH2_CC · 10 tools
- **phabricator** — Tier 3 · UNKNOWN · 10 tools
- **pipeline** — Tier 3 · UNKNOWN · 10 tools
- **pivotal** — Tier 3 · UNKNOWN · 10 tools
- **planetscale** — Tier 3 · UNKNOWN · 10 tools
- **plivo** — Tier 3 · UNKNOWN · 10 tools
- **postmark** — Tier 3 · UNKNOWN · 10 tools
- **practicefusion** — Tier 1 · OAUTH2 · 10 tools
- **printify** — Tier 3 · UNKNOWN · 10 tools
- **prismic** — Tier 3 · UNKNOWN · 10 tools
- **private-api-basic** — Tier 1 · BASIC · 10 tools
- **private-api-bearer** — Tier 1 · API_KEY · 10 tools
- **projectmanager** — Tier 3 · UNKNOWN · 10 tools
- **quora** — Tier 3 · UNKNOWN · 10 tools
- **railway** — Tier 3 · UNKNOWN · 10 tools
- **rally** — Tier 3 · UNKNOWN · 10 tools
- **recurly** — Tier 3 · UNKNOWN · 10 tools
- **redbubble** — Tier 3 · UNKNOWN · 10 tools
- **redshift** — Tier 3 · UNKNOWN · 10 tools
- **remote** — Tier 3 · UNKNOWN · 10 tools
- **render** — Tier 3 · UNKNOWN · 10 tools
- **renovate** — Tier 3 · UNKNOWN · 10 tools
- **researchdesk** — Tier 2 · TWO_STEP · 10 tools
- **ringcentral** — Tier 3 · UNKNOWN · 10 tools
- **rows** — Tier 3 · UNKNOWN · 10 tools
- **salesforce_more** — Tier 3 · UNKNOWN · 11 tools
- **sanity** — Tier 3 · UNKNOWN · 10 tools
- **sap** — Tier 3 · UNKNOWN · 10 tools
- **sas** — Tier 3 · UNKNOWN · 10 tools
- **scoro** — Tier 3 · UNKNOWN · 10 tools
- **scrapedo** — Tier 1 · API_KEY · 10 tools
- **sharpspring** — Tier 3 · UNKNOWN · 10 tools
- **shopify_more** — Tier 3 · UNKNOWN · 10 tools
- **shopworks** — Tier 2 · TWO_STEP · 10 tools
- **simpleview** — Tier 3 · UNKNOWN · 10 tools
- **sinch** — Tier 3 · UNKNOWN · 10 tools
- **slite** — Tier 3 · UNKNOWN · 10 tools
- **snapchat_more** — Tier 3 · UNKNOWN · 10 tools
- **snapengage** — Tier 3 · UNKNOWN · 10 tools
- **snipe-it** — Tier 1 · API_KEY · 10 tools
- **snyk** — Tier 3 · UNKNOWN · 10 tools
- **soundcloud** — Tier 3 · UNKNOWN · 10 tools
- **sparkpost** — Tier 3 · UNKNOWN · 10 tools
- **splash** — Tier 3 · UNKNOWN · 10 tools
- **split** — Tier 3 · UNKNOWN · 10 tools
- **spotify** — Tier 1 · OAUTH2 · 10 tools
- **spotify-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **square** — Tier 3 · UNKNOWN · 10 tools
- **stackby** — Tier 3 · UNKNOWN · 10 tools
- **statsig** — Tier 3 · UNKNOWN · 10 tools
- **storyblok** — Tier 3 · UNKNOWN · 10 tools
- **strapi** — Tier 3 · UNKNOWN · 10 tools
- **stripe_more** — Tier 3 · UNKNOWN · 10 tools
- **substack** — Tier 3 · UNKNOWN · 10 tools
- **sugarcrm** — Tier 3 · UNKNOWN · 10 tools
- **suitedash** — Tier 3 · UNKNOWN · 10 tools
- **supportbee** — Tier 3 · UNKNOWN · 10 tools
- **tawkto** — Tier 3 · UNKNOWN · 10 tools
- **taxjar** — Tier 3 · UNKNOWN · 10 tools
- **teams** — Tier 3 · UNKNOWN · 10 tools
- **teamsnap** — Tier 3 · UNKNOWN · 10 tools
- **teespring** — Tier 3 · UNKNOWN · 10 tools
- **tencent** — Tier 3 · UNKNOWN · 10 tools
- **tiktok** — Tier 3 · UNKNOWN · 10 tools
- **timeular** — Tier 3 · UNKNOWN · 20 tools
- **tito** — Tier 3 · UNKNOWN · 10 tools
- **trello_more** — Tier 3 · UNKNOWN · 10 tools
- **uber** — Tier 1 · OAUTH2 · 10 tools
- **unauthenticated** — Tier 3 · NONE · 10 tools
- **upsun** — Tier 3 · UNKNOWN · 10 tools
- **userlike** — Tier 3 · UNKNOWN · 10 tools
- **walkme** — Tier 3 · UNKNOWN · 10 tools
- **wautomations** — Tier 3 · UNKNOWN · 10 tools
- **wave** — Tier 3 · UNKNOWN · 10 tools
- **whitesource** — Tier 3 · UNKNOWN · 10 tools
- **wix** — Tier 3 · UNKNOWN · 10 tools
- **youtrack** — Tier 3 · UNKNOWN · 10 tools
- **zapier_more** — Tier 3 · UNKNOWN · 10 tools
- **zeplin** — Tier 3 · UNKNOWN · 10 tools
- **zoho_crm** — Tier 3 · UNKNOWN · 10 tools
- **zoho_more** — Tier 3 · UNKNOWN · 10 tools
- **zorus** — Tier 1 · API_KEY · 10 tools

### payment (34 providers)

- **adyen** — Tier 1 · OAUTH2 · 10 tools
- **bill** — Tier 3 · BILL · 10 tools
- **bill-sandbox** — Tier 3 · BILL · 10 tools
- **braintree** — Tier 1 · OAUTH2 · 10 tools
- **braintree-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **buildium** — Tier 1 · API_KEY · 10 tools
- **chargebee** — Tier 1 · BASIC · 10 tools
- **checkout-com** — Tier 2 · OAUTH2_CC · 10 tools
- **checkout-com-sandbox** — Tier 2 · OAUTH2_CC · 10 tools
- **coupa-compass** — Tier 2 · OAUTH2_CC · 10 tools
- **datacandy** — Tier 1 · BASIC · 10 tools
- **fiserv** — Tier 2 · OAUTH2_CC · 10 tools
- **fiserv-api-key** — Tier 1 · API_KEY · 10 tools
- **gumroad** — Tier 1 · OAUTH2 · 10 tools
- **mollie** — Tier 1 · OAUTH2 · 10 tools
- **paypal** — Tier 1 · OAUTH2 · 10 tools
- **paypal-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **pennylane** — Tier 1 · OAUTH2 · 10 tools
- **pennylane-company-api** — Tier 1 · API_KEY · 10 tools
- **pleo** — Tier 1 · OAUTH2 · 10 tools
- **razorpay** — Tier 1 · BASIC · 10 tools
- **splitwise** — Tier 1 · OAUTH2 · 10 tools
- **squareup** — Tier 1 · OAUTH2 · 10 tools
- **squareup-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **stripe** — Tier 1 · OAUTH2 · 10 tools
- **stripe-api-key** — Tier 1 · BASIC · 10 tools
- **stripe-app** — Tier 1 · OAUTH2 · 10 tools
- **stripe-app-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **stripe-express** — Tier 1 · OAUTH2 · 10 tools
- **thrivecart-api-key** — Tier 1 · API_KEY · 10 tools
- **thrivecart-oauth** — Tier 1 · OAUTH2 · 10 tools
- **tremendous** — Tier 1 · OAUTH2 · 10 tools
- **tremendous-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **wise-api-key** — Tier 1 · API_KEY · 10 tools

### popular (28 providers)

- **airtable** — Tier 1 · OAUTH2 · 10 tools
- **confluence** — Tier 3 · UNKNOWN · 10 tools
- **confluence-basic** — Tier 1 · BASIC · 10 tools
- **github-app** — Tier 3 · APP · 10 tools
- **gong-oauth** — Tier 1 · OAUTH2 · 10 tools
- **google-calendar** — Tier 3 · UNKNOWN · 10 tools
- **google-drive** — Tier 3 · UNKNOWN · 10 tools
- **google-mail** — Tier 3 · UNKNOWN · 10 tools
- **hibob-service-user** — Tier 1 · BASIC · 10 tools
- **hubspot** — Tier 1 · OAUTH2 · 10 tools
- **intercom** — Tier 1 · OAUTH2 · 10 tools
- **jira** — Tier 1 · OAUTH2 · 10 tools
- **linear** — Tier 1 · OAUTH2 · 10 tools
- **linear-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **microsoft-teams** — Tier 3 · UNKNOWN · 10 tools
- **netsuite-tba** — Tier 3 · TBA · 10 tools
- **notion** — Tier 1 · OAUTH2 · 10 tools
- **quickbooks** — Tier 1 · OAUTH2 · 10 tools
- **sage-intacct-oauth** — Tier 1 · OAUTH2 · 10 tools
- **salesforce** — Tier 1 · OAUTH2 · 10 tools
- **shopify** — Tier 1 · OAUTH2 · 10 tools
- **slack** — Tier 1 · OAUTH2 · 10 tools
- **slack-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **workday** — Tier 1 · BASIC · 10 tools
- **workday-oauth** — Tier 1 · OAUTH2 · 10 tools
- **xero** — Tier 1 · OAUTH2 · 10 tools
- **xero-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **zendesk** — Tier 1 · OAUTH2 · 10 tools

### productivity (152 providers)

- **airtable** — Tier 1 · OAUTH2 · 10 tools
- **anthropic** — Tier 1 · API_KEY · 10 tools
- **apify** — Tier 1 · API_KEY · 10 tools
- **asana** — Tier 1 · OAUTH2 · 10 tools
- **avoma** — Tier 1 · API_KEY · 10 tools
- **basecamp** — Tier 1 · OAUTH2 · 10 tools
- **cal-com-oauth** — Tier 1 · OAUTH2 · 10 tools
- **cal-com-v1** — Tier 1 · API_KEY · 10 tools
- **cal-com-v2** — Tier 1 · API_KEY · 10 tools
- **calendly** — Tier 1 · OAUTH2 · 10 tools
- **canvas-lms** — Tier 1 · OAUTH2 · 10 tools
- **circleback-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **clickup** — Tier 1 · OAUTH2 · 10 tools
- **coda** — Tier 1 · API_KEY · 10 tools
- **codeclimate** — Tier 1 · API_KEY · 10 tools
- **companycam** — Tier 1 · API_KEY · 10 tools
- **conductorone** — Tier 2 · OAUTH2_CC · 10 tools
- **demodesk** — Tier 1 · API_KEY · 10 tools
- **docuware** — Tier 2 · TWO_STEP · 10 tools
- **domo** — Tier 2 · OAUTH2_CC · 10 tools
- **envoy** — Tier 1 · OAUTH2 · 10 tools
- **expensify** — Tier 1 · BASIC · 10 tools
- **falai** — Tier 1 · API_KEY · 10 tools
- **fellow** — Tier 1 · API_KEY · 10 tools
- **figjam** — Tier 3 · UNKNOWN · 10 tools
- **figma** — Tier 1 · OAUTH2 · 10 tools
- **figma-scim** — Tier 1 · API_KEY · 10 tools
- **fireflies** — Tier 1 · API_KEY · 10 tools
- **float** — Tier 1 · API_KEY · 10 tools
- **gamma** — Tier 1 · API_KEY · 10 tools
- **gong** — Tier 1 · BASIC · 10 tools
- **gong-oauth** — Tier 1 · OAUTH2 · 10 tools
- **google** — Tier 1 · OAUTH2 · 10 tools
- **google-bigquery** — Tier 3 · UNKNOWN · 10 tools
- **google-calendar** — Tier 3 · UNKNOWN · 10 tools
- **google-chat** — Tier 3 · UNKNOWN · 10 tools
- **google-contacts** — Tier 3 · UNKNOWN · 10 tools
- **google-docs** — Tier 3 · UNKNOWN · 10 tools
- **google-forms** — Tier 3 · UNKNOWN · 10 tools
- **google-gemini** — Tier 1 · API_KEY · 10 tools
- **google-mail** — Tier 3 · UNKNOWN · 10 tools
- **google-maps** — Tier 1 · API_KEY · 10 tools
- **google-meet** — Tier 3 · UNKNOWN · 10 tools
- **google-search-console** — Tier 3 · UNKNOWN · 10 tools
- **google-sheet** — Tier 3 · UNKNOWN · 10 tools
- **google-slides** — Tier 3 · UNKNOWN · 10 tools
- **google-tasks** — Tier 3 · UNKNOWN · 10 tools
- **google-workspace-admin** — Tier 3 · UNKNOWN · 10 tools
- **grain** — Tier 1 · OAUTH2 · 10 tools
- **grain-api-key** — Tier 1 · API_KEY · 10 tools
- **grammarly** — Tier 2 · OAUTH2_CC · 10 tools
- **granola** — Tier 1 · API_KEY · 10 tools
- **granola-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **grist** — Tier 1 · API_KEY · 10 tools
- **harvest** — Tier 1 · OAUTH2 · 10 tools
- **hover** — Tier 1 · OAUTH2 · 10 tools
- **itglue** — Tier 1 · API_KEY · 10 tools
- **jira** — Tier 1 · OAUTH2 · 10 tools
- **jira-basic** — Tier 1 · BASIC · 10 tools
- **jira-data-center** — Tier 1 · OAUTH2 · 10 tools
- **jira-data-center-api-key** — Tier 1 · API_KEY · 10 tools
- **jira-data-center-basic** — Tier 1 · BASIC · 10 tools
- **kandji** — Tier 1 · API_KEY · 10 tools
- **keeper-scim** — Tier 1 · API_KEY · 10 tools
- **kintone** — Tier 1 · OAUTH2 · 10 tools
- **kintone-user-api** — Tier 1 · API_KEY · 10 tools
- **klipfolio** — Tier 1 · API_KEY · 10 tools
- **lastpass** — Tier 1 · BASIC · 10 tools
- **lessonly** — Tier 1 · BASIC · 10 tools
- **linear** — Tier 1 · OAUTH2 · 10 tools
- **linear-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **lokalise** — Tier 1 · OAUTH2 · 10 tools
- **lucid-scim** — Tier 1 · API_KEY · 10 tools
- **luma** — Tier 1 · API_KEY · 10 tools
- **lumos** — Tier 1 · API_KEY · 10 tools
- **make** — Tier 1 · API_KEY · 10 tools
- **microsoft** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-admin** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-excel** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **microsoft-planner** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-power-bi** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-powerpoint** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-teams** — Tier 3 · UNKNOWN · 10 tools
- **microsoft-word** — Tier 3 · UNKNOWN · 10 tools
- **mindbody** — Tier 1 · API_KEY · 10 tools
- **minimax** — Tier 1 · API_KEY · 10 tools
- **miro** — Tier 1 · OAUTH2 · 10 tools
- **miro-scim** — Tier 1 · API_KEY · 10 tools
- **missive** — Tier 1 · API_KEY · 10 tools
- **momentum-io** — Tier 1 · API_KEY · 10 tools
- **monday** — Tier 1 · OAUTH2 · 10 tools
- **next-cloud-ocs** — Tier 1 · BASIC · 10 tools
- **nocrm** — Tier 1 · API_KEY · 10 tools
- **notion** — Tier 1 · OAUTH2 · 10 tools
- **notion-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **notion-scim** — Tier 1 · API_KEY · 10 tools
- **one-note** — Tier 3 · UNKNOWN · 10 tools
- **oomnitza** — Tier 1 · API_KEY · 10 tools
- **openai** — Tier 1 · API_KEY · 10 tools
- **openai-admin** — Tier 1 · API_KEY · 10 tools
- **ordinal** — Tier 1 · API_KEY · 10 tools
- **perdoo** — Tier 1 · API_KEY · 10 tools
- **perimeter81** — Tier 2 · TWO_STEP · 10 tools
- **perk** — Tier 1 · OAUTH2 · 10 tools
- **perplexity** — Tier 1 · API_KEY · 10 tools
- **pingboard** — Tier 2 · OAUTH2_CC · 10 tools
- **pipedream** — Tier 1 · API_KEY · 10 tools
- **pipedream-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **pivotaltracker** — Tier 1 · API_KEY · 10 tools
- **precisefp** — Tier 1 · OAUTH2 · 10 tools
- **productboard** — Tier 1 · OAUTH2 · 10 tools
- **pylon** — Tier 1 · API_KEY · 10 tools
- **quickbase** — Tier 1 · API_KEY · 10 tools
- **readwise** — Tier 1 · API_KEY · 10 tools
- **readwise-reader** — Tier 1 · API_KEY · 10 tools
- **recall-ai** — Tier 1 · API_KEY · 10 tools
- **retell-ai** — Tier 1 · API_KEY · 10 tools
- **roam-scim** — Tier 1 · API_KEY · 10 tools
- **rocketlane** — Tier 1 · API_KEY · 10 tools
- **servicem8** — Tier 1 · OAUTH2 · 10 tools
- **servicenow** — Tier 1 · OAUTH2 · 10 tools
- **servicenow-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **setmore** — Tier 2 · TWO_STEP · 10 tools
- **shortcut** — Tier 1 · API_KEY · 10 tools
- **slab** — Tier 1 · API_KEY · 10 tools
- **slack** — Tier 1 · OAUTH2 · 10 tools
- **slack-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **smartsheet** — Tier 1 · OAUTH2 · 10 tools
- **snipe-it** — Tier 1 · API_KEY · 10 tools
- **tailscale** — Tier 2 · TWO_STEP · 10 tools
- **tailscale-api-key** — Tier 1 · API_KEY · 10 tools
- **tally** — Tier 1 · API_KEY · 10 tools
- **teamwork** — Tier 1 · OAUTH2 · 10 tools
- **ticktick** — Tier 1 · OAUTH2 · 10 tools
- **timely** — Tier 1 · OAUTH2 · 10 tools
- **timify** — Tier 2 · TWO_STEP · 10 tools
- **tldv** — Tier 1 · API_KEY · 10 tools
- **todoist** — Tier 1 · OAUTH2 · 10 tools
- **toggl** — Tier 1 · BASIC · 20 tools
- **torii** — Tier 1 · API_KEY · 10 tools
- **trafft** — Tier 2 · OAUTH2_CC · 10 tools
- **trello** — Tier 3 · OAUTH1 · 10 tools
- **trello-scim** — Tier 1 · API_KEY · 10 tools
- **tsheetsteam** — Tier 1 · OAUTH2 · 10 tools
- **workos** — Tier 1 · API_KEY · 10 tools
- **workpath** — Tier 1 · API_KEY · 10 tools
- **wrike** — Tier 1 · OAUTH2 · 10 tools
- **xai** — Tier 1 · API_KEY · 10 tools
- **zapier-nla** — Tier 1 · OAUTH2 · 10 tools
- **zoho-calendar** — Tier 3 · UNKNOWN · 10 tools
- **zoho-mail** — Tier 3 · UNKNOWN · 10 tools

### search (1 providers)

- **algolia** — Tier 1 · API_KEY · 10 tools

### social (27 providers)

- **bitly** — Tier 1 · OAUTH2 · 10 tools
- **brightcrowd** — Tier 2 · OAUTH2_CC · 10 tools
- **discord** — Tier 1 · OAUTH2 · 10 tools
- **facebook** — Tier 1 · OAUTH2 · 10 tools
- **google** — Tier 1 · OAUTH2 · 10 tools
- **heyreach** — Tier 1 · API_KEY · 10 tools
- **instagram** — Tier 1 · OAUTH2 · 10 tools
- **linkedin** — Tier 1 · OAUTH2 · 10 tools
- **linkhut** — Tier 1 · OAUTH2 · 10 tools
- **pinterest** — Tier 1 · OAUTH2 · 10 tools
- **reddit** — Tier 1 · OAUTH2 · 10 tools
- **snapchat** — Tier 1 · OAUTH2 · 10 tools
- **splitwise** — Tier 1 · OAUTH2 · 10 tools
- **strava** — Tier 1 · OAUTH2 · 10 tools
- **strava-web** — Tier 1 · OAUTH2 · 10 tools
- **tiktok-accounts** — Tier 1 · OAUTH2 · 10 tools
- **tiktok-ads** — Tier 1 · OAUTH2 · 10 tools
- **tiktok-personal** — Tier 1 · OAUTH2 · 10 tools
- **tumblr** — Tier 1 · OAUTH2 · 10 tools
- **twitch** — Tier 1 · OAUTH2 · 10 tools
- **twitter** — Tier 3 · OAUTH1 · 10 tools
- **twitter-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **twitter-v2** — Tier 1 · OAUTH2 · 10 tools
- **typefully** — Tier 1 · API_KEY · 10 tools
- **typefully-v2** — Tier 1 · API_KEY · 10 tools
- **yahoo** — Tier 1 · OAUTH2 · 10 tools
- **yandex** — Tier 1 · OAUTH2 · 10 tools

### sports (9 providers)

- **coros** — Tier 1 · OAUTH2 · 10 tools
- **coros-sandbox** — Tier 1 · OAUTH2 · 10 tools
- **fitbit** — Tier 1 · OAUTH2 · 10 tools
- **garmin** — Tier 3 · OAUTH1 · 10 tools
- **oura** — Tier 1 · OAUTH2 · 10 tools
- **strava** — Tier 1 · OAUTH2 · 10 tools
- **strava-web** — Tier 1 · OAUTH2 · 10 tools
- **twitch** — Tier 1 · OAUTH2 · 10 tools
- **whoop** — Tier 1 · OAUTH2 · 10 tools

### storage (16 providers)

- **azure-blob-storage** — Tier 3 · UNKNOWN · 10 tools
- **box** — Tier 1 · OAUTH2 · 10 tools
- **dropbox** — Tier 1 · OAUTH2 · 10 tools
- **egnyte** — Tier 1 · OAUTH2 · 10 tools
- **google-cloud-storage** — Tier 3 · UNKNOWN · 10 tools
- **google-drive** — Tier 3 · UNKNOWN · 10 tools
- **next-cloud-ocs** — Tier 1 · BASIC · 10 tools
- **one-drive** — Tier 3 · UNKNOWN · 10 tools
- **one-drive-personal** — Tier 1 · OAUTH2 · 10 tools
- **orange-logic** — Tier 2 · TWO_STEP · 10 tools
- **salesforce-cdp** — Tier 2 · TWO_STEP · 10 tools
- **sharepoint-online** — Tier 3 · UNKNOWN · 10 tools
- **sharepoint-online-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **sharepoint-online-v1** — Tier 2 · TWO_STEP · 10 tools
- **smugmug** — Tier 3 · OAUTH1 · 10 tools
- **supabase** — Tier 1 · API_KEY · 10 tools

### support (38 providers)

- **aircall** — Tier 1 · OAUTH2 · 10 tools
- **blandai** — Tier 1 · API_KEY · 10 tools
- **canny** — Tier 1 · API_KEY · 10 tools
- **chattermill** — Tier 1 · API_KEY · 10 tools
- **connectwise-psa** — Tier 1 · BASIC · 10 tools
- **connectwise-psa-staging** — Tier 1 · BASIC · 10 tools
- **connectwise-rmm** — Tier 2 · OAUTH2_CC · 10 tools
- **crisp** — Tier 1 · BASIC · 10 tools
- **crisp-plugin-install** — Tier 3 · INSTALL_PLUGIN · 10 tools
- **datto-rmm** — Tier 1 · OAUTH2 · 10 tools
- **datto-rmm-password-grant** — Tier 2 · TWO_STEP · 10 tools
- **dixa** — Tier 1 · API_KEY · 10 tools
- **elevio** — Tier 1 · API_KEY · 10 tools
- **evaluagent** — Tier 1 · BASIC · 10 tools
- **freshdesk** — Tier 1 · BASIC · 10 tools
- **freshservice** — Tier 1 · BASIC · 11 tools
- **front** — Tier 1 · OAUTH2 · 10 tools
- **front-api-key** — Tier 1 · API_KEY · 10 tools
- **gainsight-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **github** — Tier 1 · OAUTH2 · 10 tools
- **halo-psa** — Tier 2 · OAUTH2_CC · 10 tools
- **helpscout-docs** — Tier 1 · BASIC · 10 tools
- **helpscout-mailbox** — Tier 1 · OAUTH2 · 10 tools
- **hubspot** — Tier 1 · OAUTH2 · 10 tools
- **hubspot-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **intercom** — Tier 1 · OAUTH2 · 10 tools
- **kandji** — Tier 1 · API_KEY · 10 tools
- **knowbe4** — Tier 1 · API_KEY · 10 tools
- **medallia** — Tier 2 · OAUTH2_CC · 10 tools
- **ninjaone-rmm** — Tier 2 · OAUTH2_CC · 10 tools
- **ninjaone-rmm-oauth2** — Tier 1 · OAUTH2 · 10 tools
- **passportal** — Tier 2 · TWO_STEP · 10 tools
- **plain** — Tier 1 · API_KEY · 10 tools
- **prtg-classic** — Tier 1 · API_KEY · 10 tools
- **ring-central** — Tier 1 · OAUTH2 · 10 tools
- **stackexchange** — Tier 1 · OAUTH2 · 10 tools
- **zendesk** — Tier 1 · OAUTH2 · 10 tools
- **zoho-desk** — Tier 3 · UNKNOWN · 10 tools

### surveys (10 providers)

- **fillout** — Tier 1 · OAUTH2 · 10 tools
- **fillout-api-key** — Tier 1 · API_KEY · 10 tools
- **intercom** — Tier 1 · OAUTH2 · 10 tools
- **jotform** — Tier 1 · API_KEY · 10 tools
- **mailchimp** — Tier 1 · OAUTH2 · 10 tools
- **medallia** — Tier 2 · OAUTH2_CC · 10 tools
- **qualtrics** — Tier 1 · OAUTH2 · 10 tools
- **refiner** — Tier 1 · API_KEY · 10 tools
- **survey-monkey** — Tier 1 · OAUTH2 · 10 tools
- **typeform** — Tier 1 · OAUTH2 · 10 tools

### ticketing (34 providers)

- **accelo** — Tier 1 · OAUTH2 · 10 tools
- **asana** — Tier 1 · OAUTH2 · 10 tools
- **clickup** — Tier 1 · OAUTH2 · 10 tools
- **connectwise-psa** — Tier 1 · BASIC · 10 tools
- **connectwise-psa-staging** — Tier 1 · BASIC · 10 tools
- **front** — Tier 1 · OAUTH2 · 10 tools
- **front-api-key** — Tier 1 · API_KEY · 10 tools
- **github** — Tier 1 · OAUTH2 · 10 tools
- **github-app** — Tier 3 · APP · 10 tools
- **github-app-oauth** — Tier 3 · CUSTOM · 10 tools
- **github-pat** — Tier 1 · API_KEY · 10 tools
- **gitlab** — Tier 1 · OAUTH2 · 10 tools
- **gitlab-pat** — Tier 1 · API_KEY · 10 tools
- **halo-psa** — Tier 2 · OAUTH2_CC · 10 tools
- **incident-io** — Tier 1 · API_KEY · 10 tools
- **intercom** — Tier 1 · OAUTH2 · 10 tools
- **jira** — Tier 1 · OAUTH2 · 10 tools
- **jira-basic** — Tier 1 · BASIC · 10 tools
- **jira-data-center** — Tier 1 · OAUTH2 · 10 tools
- **jira-data-center-api-key** — Tier 1 · API_KEY · 10 tools
- **jira-data-center-basic** — Tier 1 · BASIC · 10 tools
- **linear** — Tier 1 · OAUTH2 · 10 tools
- **linear-mcp** — Tier 3 · MCP_OAUTH2 · 10 tools
- **luma** — Tier 1 · API_KEY · 10 tools
- **monday** — Tier 1 · OAUTH2 · 10 tools
- **roller** — Tier 2 · OAUTH2_CC · 10 tools
- **servicenow-oauth2-cc** — Tier 2 · OAUTH2_CC · 10 tools
- **teamwork** — Tier 1 · OAUTH2 · 10 tools
- **ticktick** — Tier 1 · OAUTH2 · 10 tools
- **todoist** — Tier 1 · OAUTH2 · 10 tools
- **trello** — Tier 3 · OAUTH1 · 10 tools
- **trello-scim** — Tier 1 · API_KEY · 10 tools
- **zendesk** — Tier 1 · OAUTH2 · 10 tools
- **zoho-desk** — Tier 3 · UNKNOWN · 10 tools

### video (15 providers)

- **fathom** — Tier 1 · API_KEY · 10 tools
- **fathom-oauth** — Tier 1 · OAUTH2 · 10 tools
- **gong** — Tier 1 · BASIC · 10 tools
- **gong-oauth** — Tier 1 · OAUTH2 · 10 tools
- **grain** — Tier 1 · OAUTH2 · 10 tools
- **grain-api-key** — Tier 1 · API_KEY · 10 tools
- **heygen** — Tier 1 · OAUTH2 · 10 tools
- **microsoft-teams** — Tier 3 · UNKNOWN · 10 tools
- **pinterest** — Tier 1 · OAUTH2 · 10 tools
- **snapchat** — Tier 1 · OAUTH2 · 10 tools
- **twitch** — Tier 1 · OAUTH2 · 10 tools
- **vimeo** — Tier 1 · OAUTH2 · 10 tools
- **vimeo-basic** — Tier 1 · BASIC · 10 tools
- **youtube** — Tier 3 · UNKNOWN · 10 tools
- **zoom** — Tier 1 · OAUTH2 · 10 tools

## Tier 1 Providers (Standard Auth)

### API_KEY (215 providers)

- active-campaign — 10 tools
- algolia — 10 tools
- anthropic — 10 tools
- apify — 10 tools
- apollo — 10 tools
- avoma — 10 tools
- aws-scim — 10 tools
- axiom — 10 tools
- beehiiv — 10 tools
- bettercontact — 10 tools
- bird — 10 tools
- blandai — 10 tools
- braze — 10 tools
- brevo-api-key — 10 tools
- brex-api-key — 10 tools
- builder-io-private — 10 tools
- builder-io-public — 10 tools
- buildium — 10 tools
- builtwith — 10 tools
- cal-com-v1 — 10 tools
- cal-com-v2 — 10 tools
- callrail — 10 tools
- canny — 10 tools
- canva-scim — 10 tools
- certn — 10 tools
- chattermill — 10 tools
- checkhq — 10 tools
- chorus — 10 tools
- cin7-core — 10 tools
- circle-so — 10 tools
- clari-copilot — 10 tools
- clay — 10 tools
- clerk — 10 tools
- cloudflare — 10 tools
- coda — 10 tools
- codeclimate — 10 tools
- codegen — 10 tools
- companycam — 10 tools
- copper-api-key — 10 tools
- cursor — 10 tools
- cyberimpact — 10 tools
- datadog — 10 tools
- demodesk — 10 tools
- devin — 10 tools
- discourse — 10 tools
- dixa — 10 tools
- document360 — 10 tools
- drata — 10 tools
- elevenlabs — 10 tools
- elevio — 10 tools
- exa — 10 tools
- fairing — 10 tools
- falai — 10 tools
- fathom — 10 tools
- fellow — 10 tools
- fiber-ai — 10 tools
- figma-scim — 10 tools
- fillout-api-key — 10 tools
- findymail — 10 tools
- fireflies — 10 tools
- firstbase — 10 tools
- fiserv-api-key — 10 tools
- float — 10 tools
- folk — 10 tools
- freepik — 10 tools
- freshsales — 10 tools
- freshteam — 10 tools
- front-api-key — 10 tools
- fullenrich — 10 tools
- gamma — 10 tools
- gem — 10 tools
- ghost-content — 10 tools
- github-pat — 10 tools
- gitlab-pat — 10 tools
- glyphic — 10 tools
- google-gemini — 10 tools
- google-maps — 10 tools
- grafana — 10 tools
- grain-api-key — 10 tools
- grammarly-scim — 10 tools
- granola — 10 tools
- grist — 10 tools
- heyreach — 10 tools
- holded — 10 tools
- icypeas — 10 tools
- incident-io — 10 tools
- instantly — 10 tools
- itglue — 10 tools
- jazzhr — 10 tools
- jira-data-center-api-key — 10 tools
- jobvite — 10 tools
- jotform — 10 tools
- jumpcloud — 10 tools
- juniper-mist — 10 tools
- kandji — 10 tools
- keeper-scim — 10 tools
- kintone-user-api — 10 tools
- klaviyo — 10 tools
- klicktipp — 10 tools
- klipfolio — 10 tools
- knowbe4 — 10 tools
- kustomer — 10 tools
- lagrowthmachine — 10 tools
- lattice — 10 tools
- leadmagic — 10 tools
- loom-scim — 10 tools
- loop-returns — 10 tools
- loops-so — 10 tools
- lucid-scim — 10 tools
- luma — 10 tools
- lumos — 10 tools
- make — 10 tools
- manatal — 10 tools
- metabase — 10 tools
- mindbody — 10 tools
- minimax — 10 tools
- miro-scim — 10 tools
- missive — 10 tools
- momentum-io — 10 tools
- namely-pat — 10 tools
- nocrm — 10 tools
- notion-scim — 10 tools
- nyne-ai — 10 tools
- onlogist — 10 tools
- oomnitza — 10 tools
- open-hands — 10 tools
- openai — 10 tools
- openai-admin — 10 tools
- ordinal — 10 tools
- pandadoc-api-key — 10 tools
- pendo — 10 tools
- pennylane-company-api — 10 tools
- peopledatalabs — 10 tools
- perdoo — 10 tools
- perplexity — 10 tools
- personio-recruiting — 10 tools
- pipedream — 10 tools
- pivotaltracker — 10 tools
- plain — 10 tools
- posthog — 11 tools
- private-api-bearer — 10 tools
- prive — 10 tools
- prospeo — 10 tools
- prtg-classic — 10 tools
- pylon — 10 tools
- quentn — 10 tools
- quickbase — 10 tools
- ragieai — 10 tools
- rapidapi — 10 tools
- readwise — 10 tools
- readwise-reader — 10 tools
- recall-ai — 10 tools
- recharge — 10 tools
- recruitcrm — 10 tools
- recruitee — 10 tools
- recruiterflow — 10 tools
- refiner — 10 tools
- replicate — 10 tools
- reply-io — 10 tools
- resend — 10 tools
- retell-ai — 10 tools
- ringover — 10 tools
- rippling — 10 tools
- roam-scim — 10 tools
- rocketlane — 10 tools
- rootly — 10 tools
- sage-hr — 10 tools
- scrapedo — 10 tools
- semrush — 10 tools
- sendgrid — 10 tools
- sentinelone — 10 tools
- sentry — 10 tools
- shipbob-pat — 10 tools
- shipstation-v2 — 10 tools
- shopify-api-key — 10 tools
- shopify-partner — 10 tools
- shopify-scim — 10 tools
- shortcut — 10 tools
- skio — 10 tools
- slab — 10 tools
- smartlead-ai — 10 tools
- smartrecruiters-api-key — 10 tools
- snipe-it — 10 tools
- statista — 10 tools
- stay-ai — 10 tools
- supabase — 10 tools
- supabase-mcp — 10 tools
- tailscale-api-key — 10 tools
- tally — 10 tools
- teamtailor — 10 tools
- telegram — 10 tools
- terraform — 10 tools
- thrivecart-api-key — 10 tools
- tldv — 10 tools
- torii — 10 tools
- trello-scim — 10 tools
- triple-whale — 10 tools
- twenty-crm — 10 tools
- twenty-crm-self-hosted — 10 tools
- typefully — 10 tools
- typefully-v2 — 10 tools
- unipile — 10 tools
- upsales — 10 tools
- valley-api-key — 10 tools
- vercel — 10 tools
- webinarjam — 10 tools
- whatsapp-business — 10 tools
- wise-api-key — 10 tools
- wiza — 10 tools
- workable — 10 tools
- workos — 10 tools
- workpath — 10 tools
- xai — 10 tools
- zapier-scim — 10 tools
- zorus — 10 tools

### BASIC (76 providers)

- affinity — 10 tools
- amplitude — 10 tools
- avalara — 10 tools
- avalara-sandbox — 10 tools
- aws-iam — 10 tools
- azure-devops — 10 tools
- bamboohr-basic — 10 tools
- bitdefender — 10 tools
- blackbaud-basic — 10 tools
- booking-com — 10 tools
- chargebee — 10 tools
- cisco-duo-admin — 10 tools
- clickhouse — 10 tools
- clicksend — 10 tools
- cloudtalk — 10 tools
- confluence-basic — 10 tools
- connectwise-psa — 10 tools
- connectwise-psa-staging — 10 tools
- crisp — 10 tools
- cursor-admin — 10 tools
- datacandy — 10 tools
- e-conomic — 10 tools
- entrata — 10 tools
- evaluagent — 10 tools
- expensify — 10 tools
- freshdesk — 10 tools
- freshservice — 11 tools
- gerrit — 10 tools
- gong — 10 tools
- gorgias-basic — 10 tools
- greenhouse-assessment — 10 tools
- greenhouse-basic — 10 tools
- greenhouse-harvest — 10 tools
- greenhouse-job-board — 10 tools
- greenhouse-onboarding — 10 tools
- guru — 10 tools
- guru-scim — 10 tools
- hackerrank-work — 10 tools
- helpscout-docs — 10 tools
- hibob-service-user — 10 tools
- insightly — 10 tools
- jira-basic — 10 tools
- jira-data-center-basic — 10 tools
- lastpass — 10 tools
- lemlist — 10 tools
- lessonly — 10 tools
- lever-basic — 10 tools
- lever-basic-sandbox — 10 tools
- listmonk — 10 tools
- lob — 10 tools
- mailgun — 10 tools
- mailjet — 10 tools
- mixpanel — 10 tools
- next-cloud-ocs — 10 tools
- oracle-hcm — 10 tools
- paligo — 10 tools
- paycom — 10 tools
- private-api-basic — 10 tools
- provenexpert — 10 tools
- qualia — 10 tools
- razorpay — 10 tools
- rock-gym-pro — 10 tools
- sap-odata-basic — 10 tools
- sedna-basic — 10 tools
- shipstation — 10 tools
- streak — 10 tools
- stripe-api-key — 10 tools
- toggl — 20 tools
- trakstar-hire — 10 tools
- twilio — 10 tools
- ukg-pro — 10 tools
- unanet — 10 tools
- vimeo-basic — 10 tools
- woocommerce — 10 tools
- workday — 10 tools
- workday-adaptive-planning-basic — 10 tools

### OAUTH2 (246 providers)

- accelo — 10 tools
- adobe — 10 tools
- adyen — 10 tools
- aircall — 10 tools
- airtable — 10 tools
- apaleo — 10 tools
- asana — 10 tools
- attio — 10 tools
- auth0 — 10 tools
- autodesk — 10 tools
- aws — 10 tools
- bamboohr — 10 tools
- basecamp — 10 tools
- battlenet — 10 tools
- bigcommerce — 10 tools
- bing-webmasters — 10 tools
- bitbucket — 10 tools
- bitly — 10 tools
- blackbaud — 10 tools
- boldsign — 10 tools
- box — 10 tools
- braintree — 10 tools
- braintree-sandbox — 10 tools
- brex — 10 tools
- brex-staging — 10 tools
- bullhorn — 10 tools
- cal-com-oauth — 10 tools
- calendly — 10 tools
- candis — 10 tools
- canva — 10 tools
- canvas-lms — 10 tools
- checkr-partner — 10 tools
- checkr-partner-staging — 10 tools
- cleverreach — 10 tools
- clickup — 10 tools
- clio — 10 tools
- close — 10 tools
- cloudbeds — 10 tools
- clover — 10 tools
- contentful — 10 tools
- contentstack — 10 tools
- copper — 10 tools
- coros — 10 tools
- coros-sandbox — 10 tools
- datev — 10 tools
- datto-rmm — 10 tools
- deel — 10 tools
- deel-sandbox — 10 tools
- dialpad — 10 tools
- dialpad-sandbox — 10 tools
- digitalocean — 10 tools
- discord — 10 tools
- docusign — 10 tools
- docusign-sandbox — 10 tools
- drchrono — 10 tools
- dropbox — 10 tools
- dropbox-sign — 10 tools
- ebay — 10 tools
- ebay-sandbox — 10 tools
- egnyte — 10 tools
- employment-hero — 10 tools
- envoy — 10 tools
- epic-games — 10 tools
- eventbrite — 10 tools
- exact-online — 10 tools
- exist — 10 tools
- facebook — 10 tools
- factorial — 10 tools
- fanvue — 10 tools
- fathom-oauth — 10 tools
- figma — 10 tools
- fillout — 10 tools
- fitbit — 10 tools
- fortnox — 10 tools
- freeagent — 10 tools
- freeagent-sandbox — 10 tools
- freshbooks — 10 tools
- front — 10 tools
- github — 10 tools
- gitlab — 10 tools
- gong-oauth — 10 tools
- google — 10 tools
- google-play — 10 tools
- gorgias — 10 tools
- grain — 10 tools
- greenhouse — 10 tools
- greenhouse-harvest-partner — 10 tools
- greenhouse-ingestion — 10 tools
- gumroad — 10 tools
- gusto — 10 tools
- gusto-demo — 10 tools
- harvest — 10 tools
- health-gorilla — 10 tools
- helpscout-mailbox — 10 tools
- heygen — 10 tools
- highlevel — 10 tools
- highlevel-white-label — 10 tools
- hover — 10 tools
- hubspot — 10 tools
- instagram — 10 tools
- intercom — 10 tools
- intuit — 10 tools
- ironclad — 10 tools
- jira — 10 tools
- jira-data-center — 10 tools
- jobadder — 10 tools
- jobber — 10 tools
- justworks — 10 tools
- keap — 10 tools
- kintone — 10 tools
- klaviyo-oauth — 10 tools
- lever — 10 tools
- lever-sandbox — 10 tools
- lightspeed-retail — 10 tools
- linear — 10 tools
- linkedin — 10 tools
- linkhut — 10 tools
- lokalise — 10 tools
- looker-oauth — 10 tools
- mailchimp — 10 tools
- maximizer — 10 tools
- maximizer-on-premise — 10 tools
- mercury — 10 tools
- microsoft — 10 tools
- microsoft-admin — 10 tools
- microsoft-teams-bot — 10 tools
- microsoft-tenant-specific — 10 tools
- miro — 10 tools
- mollie — 10 tools
- monday — 10 tools
- mural — 10 tools
- namely — 10 tools
- nationbuilder — 10 tools
- netsuite — 10 tools
- ninjaone-rmm-oauth2 — 10 tools
- notion — 10 tools
- odoo — 10 tools
- okta — 10 tools
- one-drive-personal — 10 tools
- osu — 10 tools
- oura — 10 tools
- outreach — 10 tools
- pagerduty — 10 tools
- pandadoc — 10 tools
- paycor — 10 tools
- paycor-sandbox — 10 tools
- payfit — 10 tools
- paypal — 10 tools
- paypal-sandbox — 10 tools
- pennylane — 10 tools
- perk — 10 tools
- pingone — 10 tools
- pinterest — 10 tools
- pipedrive — 10 tools
- pleo — 10 tools
- podium — 10 tools
- practicefusion — 10 tools
- precisefp — 10 tools
- printful — 10 tools
- procore — 10 tools
- productboard — 10 tools
- qualtrics — 10 tools
- quickbooks — 10 tools
- ramp — 10 tools
- ramp-sandbox — 10 tools
- reapit — 10 tools
- reddit — 10 tools
- ring-central — 10 tools
- ring-central-sandbox — 10 tools
- rippling-shop-app — 10 tools
- sage — 10 tools
- sage-intacct-oauth — 10 tools
- sage-people — 10 tools
- salesforce — 10 tools
- salesforce-experience-cloud — 10 tools
- salesforce-sandbox — 10 tools
- salesloft — 10 tools
- salesmsg-oauth2 — 10 tools
- schwab — 10 tools
- segment — 10 tools
- sellsy — 10 tools
- sentry-oauth — 10 tools
- servicem8 — 10 tools
- servicenow — 10 tools
- shopify — 10 tools
- signnow — 10 tools
- signnow-sandbox — 10 tools
- slack — 10 tools
- smartsheet — 10 tools
- snapchat — 10 tools
- snowflake — 10 tools
- splitwise — 10 tools
- spotify — 10 tools
- squarespace — 10 tools
- squareup — 10 tools
- squareup-sandbox — 10 tools
- stackexchange — 10 tools
- strava — 10 tools
- strava-web — 10 tools
- stripe — 10 tools
- stripe-app — 10 tools
- stripe-app-sandbox — 10 tools
- stripe-express — 10 tools
- survey-monkey — 10 tools
- teamleader — 10 tools
- teamwork — 10 tools
- thrivecart-oauth — 10 tools
- ticktick — 10 tools
- tiktok-accounts — 10 tools
- tiktok-ads — 10 tools
- tiktok-personal — 10 tools
- timely — 10 tools
- todoist — 10 tools
- tremendous — 10 tools
- tremendous-sandbox — 10 tools
- tsheetsteam — 10 tools
- tumblr — 10 tools
- twinfield — 10 tools
- twitch — 10 tools
- twitter-v2 — 10 tools
- typeform — 10 tools
- uber — 10 tools
- ukg-pro-wfm — 10 tools
- vimeo — 10 tools
- wakatime — 10 tools
- wave-accounting — 10 tools
- wealthbox — 10 tools
- webex — 10 tools
- webflow — 10 tools
- whoop — 10 tools
- wildix-pbx — 10 tools
- wiseagent — 10 tools
- wordpress — 10 tools
- workable-oauth — 10 tools
- workday-oauth — 10 tools
- wrike — 10 tools
- xero — 10 tools
- yahoo — 10 tools
- yandex — 10 tools
- zapier — 10 tools
- zapier-nla — 10 tools
- zendesk — 10 tools
- zendesk-sell — 10 tools
- zenefits — 10 tools
- zoho — 10 tools
- zoom — 10 tools

### OAUTH1 (4 providers)

- garmin — 10 tools
- smugmug — 10 tools
- trello — 10 tools
- twitter — 10 tools

### SIGNATURE (1 providers)

- emarsys — 10 tools

### UNKNOWN (218 providers)

- 17hats — 10 tools
- acuity — 10 tools
- aha — 10 tools
- aiven — 10 tools
- akamai — 10 tools
- alibaba — 10 tools
- anchor — 10 tools
- artfol — 10 tools
- attach — 10 tools
- autobot — 10 tools
- avaza — 10 tools
- azure — 10 tools
- azure-blob-storage — 10 tools
- azure_devops — 10 tools
- baidu — 10 tools
- bear — 10 tools
- bereal — 9 tools
- bilibili — 10 tools
- bookly — 10 tools
- botpress — 10 tools
- bytedance — 10 tools
- calendary — 10 tools
- calendly_more — 10 tools
- charcle — 10 tools
- chatfuel — 10 tools
- chatwork — 10 tools
- clarity — 10 tools
- clockify — 10 tools
- clearbit — 10 tools
- cloverly — 10 tools
- coda — 10 tools
- company-database — 10 tools
- confluent — 10 tools
- conor — 10 tools
- contain — 10 tools
- copper — 10 tools
- crowdstrike — 10 tools
- crush — 10 tools
- cumberland — 10 tools
- customer-gauge — 10 tools
- customers — 10 tools
- daily — 10 tools
- dawa — 10 tools
- demio — 10 tools
- discord — 10 tools
- disqus — 10 tools
- doppler — 10 tools
- drata — 10 tools
- dtw — 10 tools
- echohome — 10 tools
- eduflow — 10 tools
- eight — 10 tools
- eloqua — 10 tools
- enscala — 10 tools
- epic — 10 tools
- ecu360-production — 10 tools
- eventbrite — 10 tools
- expedite — 10 tools
- exporting — 10 tools
- fansly — 10 tools
- feed表面 — 10 tools
- fence — 10 tools
- fere — 10 tools
- figma — 10 tools
- fill — 10 tools
- fixer — 10 tools
- fleep — 10 tools
- flow — 10 tools
- flutter — 10 tools
- foxt — 10 tools
- freshbooks — 10 tools
- front — 10 tools
- gdocs — 10 tools
- getsinto — 10 tools
- github — 10 tools
- gmail — 10 tools
- godaddy — 10 tools
- google-analytics — 10 tools
- google-calendar — 10 tools
- google-drive — 10 tools
- google-mail — 10 tools
- google-meet — 10 tools
- google-sheet — 10 tools
- google-slides — 10 tools
- gotomeeting — 10 tools
- gotowebinar — 10 tools
- grab — 10 tools
- grafana — 10 tools
- greenhouse — 10 tools
- gumroad — 10 tools
- gyrus — 10 tools
- hacker-news — 10 tools
- harper — 10 tools
- harvest — 10 tools
- haven — 10 tools
- helpwise — 10 tools
- h在说 — 10 tools
- hootsuite — 10 tools
- horde — 10 tools
- host — 10 tools
- hubspot — 10 tools
- hunter — 10 tools
- hyland — 10 tools
- hypercontext — 10 tools
- hype — 10 tools
- imgflip — 10 tools
- imix — 10 tools
- instagram — 10 tools
- interstitial — 10 tools
- inversions — 10 tools
- invoice ninja — 10 tools
- jabmo — 10 tools
- jira — 10 tools
- jotform — 10 tools
- kanban — 10 tools
- kanbant — 10 tools
- keep — 10 tools
- kustomer — 10 tools
- laserwave — 10 tools
- launch — 10 tools
- lean — 10 tools
- leetcode — 10 tools
- lever — 10 tools
- linear — 10 tools
- linkedin — 10 tools
- luma — 10 tools
- mailchimp — 10 tools
- mailersend — 10 tools
- mailjet — 10 tools
- mangools — 10 tools
- marketo — 10 tools
- stackby — 10 tools
- matomo — 10 tools
- mavenlink — 10 tools
- medium — 10 tools
- meet — 10 tools
- melillo — 10 tools
- mermaid — 10 tools
- meta — 10 tools
- metor — 10 tools
- microsoft-outlook — 10 tools
- microsoft-teams — 10 tools
- miro — 10 tools
- mixpanel — 10 tools
- monday — 10 tools
- monkey — 10 tools
- moosend — 10 tools
- mural — 10 tools
- my sql — 10 tools
- mymail — 10 tools
- navy — 10 tools
- netsuite — 10 tools
- new-relic — 10 tools
- nightfall — 10 tools
- nimblr — 10 tools
- notion — 10 tools
- nuapay — 10 tools
- numina — 10 tools
- ocr — 10 tools
- odoog — 10 tools
- oke — 10 tools
- one — 10 tools
- onfon — 10 tools
- openai — 10 tools
- openex — 10 tools
- openvol — 10 tools
- opsgenie — 10 tools
- oracle — 10 tools
- orbit — 10 tools
- order — 10 tools
- ortto — 10 tools
- oshop — 10 tools
- outgrow — 10 tools
- outreach — 10 tools
- pagerduty — 10 tools
- pandadoc — 10 tools
- papyrs — 10 tools
- partnerstack — 10 tools
- paste — 10 tools
- payflake — 10 tools
- payday — 10 tools
- pcloud — 10 tools
- pdf — 10 tools
- people — 10 tools
- peopledoc — 10 tools
- perfect-audit — 10 tools
- persistance — 10 tools
- personas — 10 tools
- pib — 10 tools
- pipedrive — 10 tools
- pixeLA — 10 tools
- place — 10 tools
- planhat — 10 tools
- plaid — 10 tools
- pod — 10 tools
- polkamarkets — 10 tools
- polyt — 10 tools
- prem — 10 tools
- print — 10 tools
- privMX — 10 tools
- prodpad — 10 tools
- progress — 10 tools
- proworkflow — 10 tools
- proxyclick — 10 tools
- purecar — 10 tools
- qualibrate — 10 tools
- quickbooks — 10 tools
- raid — 10 tools
- realtime — 10 tools
- rebound — 10 tools
- record — 10 tools
- redash — 10 tools
- remote — 10 tools
- render — 10 tools
- reply — 10 tools
- righthook — 10 tools
- ringcentral — 10 tools
- rocketreach — 10 tools
- roll — 10 tools
- rover — 10 tools
- rstudio — 10 tools
- ruby — 10 tools
- rune — 10 tools
- salesforce — 10 tools
- sap — 10 tools
- scrap — 10 tools
- seam — 10 tools
- search — 10 tools
- sent — 10 tools
- sentieo — 10 tools
- sgate — 10 tools
- sharepoint — 10 tools
- shark — 10 tools
- shift — 10 tools
- shopify — 10 tools
- shyness — 10 tools
- slack — 10 tools
- slash — 10 tools
- slite — 10 tools
- smab — 10 tools
- smartsheet — 10 tools
- smooch — 10 tools
- snap — 10 tools
- snipe-it — 10 tools
- snowflake — 10 tools
- soap — 10 tools
- spot — 10 tools
- sprint — 10 tools
- spotnext — 10 tools
- squarespace — 10 tools
- stack — 10 tools
- status — 10 tools
- stock糖 — 10 tools
- strike — 10 tools
- stripe — 10 tools
- study — 10 tools
- style — 10 tools
- substack — 10 tools
- sugarcrm — 10 tools
- sugargl — 10 tools
- support — 10 tools
- swirl — 10 tools
- swimm — 10 tools
- talk — 10 tools
- talkable — 10 tools
- tally — 10 tools
- tang — 10 tools
- teamgrid — 10 tools
- teams — 10 tools
- teamviewer — 10 tools
- tektra — 10 tools
- telegram — 10 tools
- telnyx — 10 tools
- tender — 10 tools
- tenfold — 10 tools
- test — 10 tools
- tetra — 10 tools
- think — 10 tools
- tickettailor — 10 tools
- tidy — 10 tools
- timetastic — 10 tools
- timezone — 10 tools
- tinyurl — 10 tools
- title — 10 tools
- tomt — 10 tools
- track — 10 tools
- trello — 10 tools
- trig — 10 tools
- trengo — 10 tools
- trivago — 10 tools
- trovit — 10 tools
- tru — 10 tools
- truelog — 10 tools
- tumblr — 10 tools
- tunnel — 10 tools
- twist — 10 tools
- twitch — 10 tools
- twitter — 10 tools
- type — 10 tools
- typeform — 10 tools
- uber — 10 tools
- ubiquiti — 10 tools
- ums — 10 tools
- unsubscribe — 10 tools
- uptime — 10 tools
- usably — 10 tools
- vercel — 10 tools
- wistia — 10 tools
- wit — 10 tools
- wootric — 10 tools
- wordpress — 10 tools
- workBoard — 10 tools
- workplace — 10 tools
- wrike — 10 tools
- xero — 10 tools
- xing — 10 tools
- yac — 10 tools
- yahoo — 10 tools
- yump — 10 tools
- zendesk — 10 tools
- zenefits — 10 tools
- zenloop — 10 tools
- zepel — 10 tools
- zoho — 10 tools
- zoom — 10 tools

## Tier 2 Providers (Complex Auth)

### OAUTH2_CC (66 providers)

- bliro — 10 tools
- brightcrowd — 10 tools
- certn-partner — 10 tools
- checkout-com — 10 tools
- checkout-com-sandbox — 10 tools
- cloudentity — 10 tools
- commercetools — 10 tools
- conductorone — 10 tools
- connectwise-rmm — 10 tools
- coupa-compass — 10 tools
- crowdstrike — 10 tools
- databricks-account — 10 tools
- databricks-workspace — 10 tools
- domo — 10 tools
- emarsys-oauth — 10 tools
- firefish — 10 tools
- fiserv — 10 tools
- gainsight-cc — 10 tools
- gebruder-weiss — 10 tools
- grammarly — 10 tools
- greenhouse-harvest-oauth2-cc — 10 tools
- halo-psa — 10 tools
- jamf — 10 tools
- listrak — 10 tools
- malwarebytes — 10 tools
- marketo — 10 tools
- medallia — 10 tools
- microsoft-business-central — 10 tools
- microsoft-oauth2-cc — 10 tools
- mimecast — 10 tools
- nerdio — 10 tools
- ninjaone-rmm — 10 tools
- okta-cc — 10 tools
- onelogin — 10 tools
- oracle-cloud-identity — 10 tools
- ory — 10 tools
- pax8 — 10 tools
- paychex — 10 tools
- paylocity — 10 tools
- paylocity-nextgen — 10 tools
- pendo-oauth — 10 tools
- personio — 10 tools
- personio-v2 — 10 tools
- pingboard — 10 tools
- pingone-cc — 10 tools
- pipedream-oauth2-cc — 10 tools
- roller — 10 tools
- salesforce-cc — 10 tools
- sap-concur — 10 tools
- sap-odata-oauth2-cc — 10 tools
- sedna — 10 tools
- sellsy-oauth2-cc — 10 tools
- servicenow-oauth2-cc — 10 tools
- sharepoint-online-oauth2-cc — 10 tools
- shopify-cc — 10 tools
- sophos-central — 10 tools
- spotify-oauth2-cc — 10 tools
- tapclicks — 10 tools
- trafft — 10 tools
- twitter-oauth2-cc — 10 tools
- ukg-pro-cc — 10 tools
- ukg-ready — 10 tools
- vanta — 10 tools
- xero-oauth2-cc — 10 tools
- zoominfo — 10 tools
- zuora — 10 tools

### TWO_STEP (40 providers)

- avanan — 10 tools
- breezy-hr — 10 tools
- datto-rmm-password-grant — 10 tools
- dayforce — 10 tools
- docuware — 10 tools
- drupal — 10 tools
- ecu360 — 10 tools
- google-service-account — 10 tools
- heap — 10 tools
- jamf-basic — 10 tools
- jobdiva — 10 tools
- looker — 10 tools
- mip-cloud — 10 tools
- mip-on-premise — 10 tools
- modmed — 10 tools
- odoo-cc — 10 tools
- orange-logic — 10 tools
- passportal — 10 tools
- perimeter81 — 10 tools
- redtail-crm-sandbox — 10 tools
- researchdesk — 10 tools
- sage-intacct — 10 tools
- salesforce-cdp — 10 tools
- salesforce-jwt — 10 tools
- salesmsg — 10 tools
- sap-business-one — 10 tools
- sap-fieldglass — 10 tools
- sap-success-factors — 10 tools
- sellercloud — 10 tools
- setmore — 10 tools
- sharepoint-online-v1 — 10 tools
- shopworks — 10 tools
- tableau — 10 tools
- tailscale — 10 tools
- timify — 10 tools
- valley — 10 tools
- veeva-vault — 10 tools
- workday-adaptive-planning — 10 tools
- workday-refresh-token — 10 tools
- yotpo — 10 tools


### BILL (2 providers)

- bill — 10 tools
- bill-sandbox — 10 tools

### TBA (1 providers)

- netsuite-tba — 10 tools

## Skipped Providers (Not Yet Supported)

These auth modes are not yet supported. 7 providers total.

### APP (1 providers)

- github-app — 10 tools

### CUSTOM (1 providers)

- github-app-oauth — 10 tools

### INSTALL_PLUGIN (1 providers)

- crisp-plugin-install — 10 tools

### JWT (3 providers)

- ghost-admin — 10 tools
- heymarket — 10 tools
- snowflake-jwt — 10 tools

### NONE (1 providers)

- unauthenticated — 10 tools

## Adding a New Provider

1. **Create registry file**: `tools/registry/{provider}.yaml` with tool allowlist
2. **Create docs file**: `tools/docs/{provider}.md` with documentation
3. **Update this document** (re-run the generation script)
4. Verify YAML parses: `python3 -c "import yaml; yaml.safe_load(open('tools/registry/{provider}.yaml'))"`

Tool schema:
```yaml
slug: provider_list_items
name: List Items
description: List items from the API
provider: {provider}
endpoint: /items
method: GET
scopes: [items:read]
tags: [items, read]
input_schema:
  type: object
  properties:
    limit:
      type: integer
      description: Max items to return
  required: []
```