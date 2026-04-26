# ActiveCampaign

Marketing automation platform for managing contacts, deals, campaigns, and more.

## Provider Details

- **Provider Key**: `active-campaign`
- **Authentication**: OAuth2/API key
- **API Docs**: https://developers.activecampaign.com/

## Tool Registry

### Contacts

| Tool | Description |
|------|-------------|
| `active-campaign_list_contacts` | Retrieve a paginated list of all contacts |
| `active-campaign_get_contact` | Get details of a specific contact by ID |
| `active-campaign_create_contact` | Create a new contact |
| `active-campaign_update_contact` | Update an existing contact's information |
| `active-campaign_add_contact_to_tag` | Add a tag to a contact |

### Deals

| Tool | Description |
|------|-------------|
| `active-campaign_list_deals` | Retrieve a list of all deals |
| `active-campaign_get_deal` | Get details of a specific deal by ID |
| `active-campaign_create_deal` | Create a new deal |

### Campaigns

| Tool | Description |
|------|-------------|
| `active-campaign_list_campaigns` | Retrieve a list of all email campaigns |
| `active-campaign_get_campaign` | Get details of a specific campaign by ID |

## Scopes

The following scopes are used by ActiveCampaign tools:

- `contact_list` - Read contact list
- `contact_view` - View contact details
- `contact_add` - Create new contacts
- `contact_edit` - Update contacts
- `contact_tag` - Tag contacts
- `deal_list` - Read deal list
- `deal_view` - View deal details
- `deal_add` - Create deals
- `campaign_list` - Read campaign list
- `campaign_view` - View campaign details

## API Base URL

`https://{account}.api-us1.com/api/3`

## Notes

- ActiveCampaign uses a region-specific subdomain for their API (e.g., `api-us1.com`)
- Rate limits apply based on your ActiveCampaign plan
- Contact IDs are integers in the API