import { useMemo, useState } from 'react';

// ============================================================================
// SCOPE SUGGESTIONS — fallback scope lists for OAuth/API providers
//
// Structure: Most Popular first, then grouped by function.
// Only providers that actually work with OAuth/API scopes are listed here.
// ============================================================================

const SCOPE_SUGGESTIONS: Record<string, string[]> = {

  // ==========================================================================
  // MOST POPULAR — Top 25 globally used APIs
  // ==========================================================================
  // Social / Professional
  github:       ['repo', 'read:user', 'user:email', 'repo:status', 'workflow'],
  linkedin:     ['openid', 'profile', 'email', 'w_member_social', 'rw_organization_admin', 'r_organization_social'],
  facebook:     ['email', 'public_profile', 'pages_read_engagement', 'pages_manage_posts', 'pages_manage_metadata', 'business_management'],
  x:            ['tweet.read', 'users.read', 'tweet.write', 'offline.access', 'follows.read'],

  // Productivity / Docs
  slack:        ['channels:read', 'channels:write', 'channels:history', 'chat:write', 'chat:write.public', 'users:read', 'team:read'],
  notion:       ['workspace_metadata', 'database:read', 'database:write', 'page:read', 'page:write', 'comment:read'],
  jira:         ['read:jira-work', 'write:jira-work', 'manage:jira-configuration', 'admin'],
  asana:        ['default', 'projects:read', 'projects:write', 'tasks:read', 'tasks:write', 'users:read'],
  trello:       ['read', 'write', 'account', 'boards:read', 'boards:write'],
  linear:       ['admin', 'issues:read', 'issues:write', 'teams:read', 'teams:write', 'users:read'],

  // CRM / Sales
  salesforce:   ['api', 'refresh_token', 'full'],
  hubspot:      ['crm.objects.contacts.read', 'crm.objects.contacts.write', 'crm.schemas.contacts.read', 'oauth'],
  zendesk:      ['read', 'write', 'manage'],
  intercom:     ['read', 'write', 'conversations:read', 'conversations:write'],

  // Marketing / Ads
  google_ads:   ['adwords.report', 'adwords.optimization'],
  mailchimp:    ['read', 'write', 'campaigns:read', 'campaigns:write'],
  klaviyo:      ['read', 'write'],

  // Storage / Files
  dropbox:      ['account_info.read', 'files.content.read', 'files.content.write', 'files.metadata.read', 'files.metadata.write'],
  box:          ['root:read', 'folder:read', 'folder:write', 'file:read', 'file:write', 'upload_files'],
  googledrive:  ['https://www.googleapis.com/auth/drive.readonly', 'https://www.googleapis.com/auth/drive.file'],

  // Commerce / Payments
  shopify:      ['read_products', 'write_products', 'read_orders', 'write_orders', 'read_fulfillments', 'write_fulfillments'],
  stripe:       ['read_only', 'write', 'express_console'],
  paypal:       ['openid', 'profile', 'email', 'https://uri.paypal.com/services/invoicing', 'https://uri.paypal.com/services/paypalattributes'],

  // Communication / Video
  zoom:         ['meeting:read', 'meeting:write', 'webinar:read', 'webinar:write', 'user:read', 'user:master'],
  twilio:       ['Flex', 'studio', 'conversations', 'voice', 'messaging'],
  sendgrid:     ['mail:read', 'mail:write', 'marketing:read', 'marketing:write', 'campaigns:read', 'campaigns:write'],

  // Analytics / Data
  segment:      ['read', 'write', 'delete'],
  mixpanel:     ['read', 'write'],
  amplitude:   ['access_logs', 'delete_users', 'export_data', 'ingest_data', 'manage_projects'],

  // Infrastructure / DevOps
  vercel:       ['read', 'write'],
  datadog:      ['metrics:read', 'metrics:write', 'events:read', 'events:write', 'logs:read', 'logs:write'],
  newrelic:     ['read', 'write'],
  pagerduty:   ['read', 'write'],

  // ==========================================================================
  // SOCIAL / CONTENT PLATFORMS
  // ==========================================================================
  youtube:      ['https://www.googleapis.com/auth/youtube', 'https://www.googleapis.com/auth/youtube.readonly', 'https://www.googleapis.com/auth/youtube.upload'],
  instagram:    ['user_profile', 'user_media'],
  reddit:       ['account', 'announcements', 'edit', 'flair', 'history', 'identity', 'livemanage', 'modconfig', 'modflair', 'read', 'submit', 'vote'],
  tiktok:       ['user.info.read', 'video.upload', 'video.publish'],
  tumblr:       ['basic', 'posts', 'blog', 'user'],
  vimeo:        ['edit', 'upload', 'view'],
  unsplash:     ['read', 'write'],
  shutterstock: ['read', 'write'],

  // ==========================================================================
  // DEVELOPER TOOLS — Git, APIs, Documentation, Monitoring
  // ==========================================================================
  gitlab:       ['read_api', 'write_repository', 'api', 'openid', 'profile', 'email'],
  bitbucket:    ['repository', 'pullrequest', 'account'],
  confluence:   ['read', 'write', 'admin'],
  youtrack:     ['read', 'write'],
  jira_data_center: ['read:jira-work', 'write:jira-work'],
  shortcut:     ['read', 'write'],
  clubhouse:    ['stories:read', 'stories:write'],
  height:       ['read', 'write'],
  codemagic:    ['read', 'write'],
  targetprocess: ['read', 'write'],
  sentry:       ['event:read', 'event:write', 'project:read', 'project:write', 'org:read', 'org:write'],
  grafana:      ['read', 'write'],
  splunk:       ['search', 'inputs'],
  cloudflare:   ['read', 'write'],
  digitalocean: ['read', 'write'],
  heroku:       ['read', 'write'],
  netlify:      ['read', 'write'],
  terraform:    ['read', 'write'],
  aws:          ['sts:assume_role'],

  // ==========================================================================
  // PRODUCTIVITY / DOCUMENTS — Notes, Docs, Wikis, Office
  // ==========================================================================
  airtable:     ['data.records:read', 'data.records:write', 'schema.bases:read'],
  evernote:     ['business_notes', 'notes', 'notes:read', 'notes:write'],
  onenote:      ['Notes.Read', 'Notes.ReadWrite'],
  google_docs:  ['https://www.googleapis.com/auth/documents', 'https://www.googleapis.com/auth/documents.readonly'],
  google_slides: ['https://www.googleapis.com/auth/presentations', 'https://www.googleapis.com/auth/presentations.readonly'],
  google_sheets: ['https://www.googleapis.com/auth/spreadsheets', 'https://www.googleapis.com/auth/spreadsheets.readonly'],
  'google-mail': ['https://www.googleapis.com/auth/gmail.readonly', 'https://www.googleapis.com/auth/gmail.modify'],
  gmail:        ['https://www.googleapis.com/auth/gmail.readonly', 'https://www.googleapis.com/auth/gmail.modify'],
  coda:         ['read', 'write'],
  confluent:    ['cloudkraken:read', 'cloudkraken:write'],
  quip:         ['read', 'write'],
  dropbox_paper: ['read', 'write'],

  // ==========================================================================
  // CRM / SALES
  // ==========================================================================
  pipedrive:    ['read', 'write'],
  close:        ['all.full_access', 'offline_access'],
  copper:       ['developer/v1/all'],
  agility:      ['read', 'write'],
  clari:        ['read', 'write'],
  salesloft:    ['account:read', 'accounts:delete', 'accounts:read', 'accounts:write'],
  outreach:     ['accounts.all', 'accounts.read', 'calls.all', 'calls.read'],
  gong:         ['read', 'write'],

  // ==========================================================================
  // PROJECT / TASK MANAGEMENT
  // ==========================================================================
  todoist:      ['task:read', 'task:write', 'project:read', 'project:write', 'label:read', 'label:write'],
  clickup:     ['tasks:read', 'tasks:write', 'lists:read', 'lists:write', 'teams:read'],
  basecamp:    ['project', 'person', 'todo', 'calendar', 'document'],
  wrike:       ['wsdrive:read', 'wsdrive:write', 'wsdrive:delete'],
  workday:     ['default', 'workday.externalsystems', 'workday.core'],
  bamboohr:    ['bamboohr_api', 'bamboohr_employees'],
  personio:    ['read', 'write'],
  greenhouse:  ['read', 'write'],
  lever:       ['read', 'write'],

  // ==========================================================================
  // COMMUNICATION / VIDEO
  // ==========================================================================
  webex:       ['spark:calls:read', 'spark:calls:write', 'spark:messages:read', 'spark:messages:write', 'spark:rooms:read', 'spark:rooms:write'],
  ringcentral: ['read', 'write'],
  teams:       ['User.Read', 'Calendars.Read', 'Calendars.ReadWrite', 'OnlineMeetings.ReadWrite'],

  // ==========================================================================
  // MARKETING / ADS
  // ==========================================================================
  tiktok_ads:   ['advertiser.read', 'advertiser.write'],
  twitter:      ['tweet.read', 'users.read', 'tweet.write', 'offline.access'],
  pinterest:    ['ads:read', 'ads:write', 'boards:read', 'pins:read', 'pins:write'],
  brevo:        ['sendTransactionalEmail', 'smtp:read', 'smtp:write', 'contacts:read', 'contacts:write'],
  customer_io:  ['api:all', 'campaigns:read', 'campaigns:write'],
  braze:        ['users.soft_delete', 'campaigns.read', 'campaigns.write', 'messages:read'],
  moosend:      ['user', 'campaigns', 'subscribers'],
  mailjet:      ['api_key', 'messages:read', 'messages:write', 'contacts:read'],
  marketo:      ['read', 'write', 'execute'],
  sharpspring:  ['read', 'write'],
  convertible:  ['read', 'write'],
  lemlist:      ['campaigns:read', 'campaigns:write', 'leads:read', 'leads:write'],
  closo:        ['read', 'write'],

  // ==========================================================================
  // STORAGE / FILE SHARING
  // ==========================================================================
  onedrive:    ['Files.Read', 'Files.ReadWrite', 'Sites.Read.All'],

  // ==========================================================================
  // COMMERCE / PAYMENTS / BILLING
  // ==========================================================================
  squareup:     ['MERCHANT_API_READ', 'PAYMENTS_READ', 'PAYMENTS_WRITE', 'ITEMS_READ', 'ITEMS_WRITE'],
  square:       ['MERCHANT_API_READ', 'PAYMENTS_READ', 'PAYMENTS_WRITE', 'ITEMS_READ', 'ITEMS_WRITE'],
  woocommerce:  ['read', 'write'],
  bigcommerce:  ['read_only', 'store_v2_products', 'store_v2_information'],
  commerce:     ['read', 'write'],
  bill:         ['read', 'write'],
  chargify:     ['read', 'write'],
  gumroad:      ['read', 'write'],
  vend:         ['read', 'write'],
  ebay:         ['read_catalog', 'read_inventory', 'write_inventory'],

  // ==========================================================================
  // ANALYTICS / MONITORING / DATA
  // ==========================================================================
  posthog:      ['read', 'write', 'analytics'],
  heap:         ['read', 'write'],
  simplex:      ['read', 'write'],
  saturn:       ['read', 'write'],
  statuspage:   ['read', 'write'],

  // ==========================================================================
  // HR / PAYROLL / ATS
  // ==========================================================================
  adp:          ['openid', 'profile', 'email', 'workforce-core'],
  paychex:      ['read', 'write'],
  paylocity:    ['read', 'write'],
  rippling:     ['read', 'write'],
  gusto:        ['read', 'write'],
  adp_workforce_now: ['read', 'write'],
  namely:       ['all', 'default'],

  // ==========================================================================
  // FINANCE / ACCOUNTING
  // ==========================================================================
  quickbooks:   ['com.intuit.quickbooks.accounting', 'com.intuit.quickbooks.payment', 'com.intuit.quickbooks.employees', 'com.intuit.quickbooks.billing'],
  xero:         ['openid', 'profile', 'email', 'accounting.transactions.read', 'accounting.transactions.write', 'accounting.contacts.read'],
  freshbooks:   ['read', 'write'],
  freeagent:    ['read', 'write'],
  zoho:         ['read', 'write', 'ZohoCRM.modules.ALL'],
  wave:         ['accounting', 'payments', 'receivables'],
  sage:         ['read', 'write'],
  chargebee:    ['read', 'write'],

  // ==========================================================================
  // DESIGN / COLLABORATION
  // ==========================================================================
  figma:        ['files:read', 'files:write', 'comments:read', 'comments:write'],
  figjam:       ['read', 'write'],
  miro:         ['boards:read', 'boards:write', 'teams:read'],
  invision:     ['read', 'write'],

  // ==========================================================================
  // DATA / ML / AI
  // ==========================================================================
  openai:       ['read', 'write'],
  anthropic:    ['read', 'write'],
  stability_ai: ['read', 'write'],
  replicate:    ['read', 'write'],
  clarifai:     ['read', 'write'],
  databricks:   ['read', 'write'],
  snowflake:    ['read', 'write'],
  bigquery:     ['https://www.googleapis.com/auth/bigquery', 'https://www.googleapis.com/auth/bigquery.readonly'],

  // ==========================================================================
  // OTHER — Scheduling, forms, misc
  // ==========================================================================
  calendly:     ['default'],
  typeform:     ['offline'],
  constant_contact: ['access_any_user', 'campaign_data', 'contact_data'],
  gravitee:     ['portal'],
  graviteeio:   ['portal'],
  activecampaign: ['api_key'],
  lytics:       ['read', 'write'],
  conversocial: ['read', 'write'],
  doctolib:     ['read', 'write'],
  front:        ['read', 'write'],
  pandadoc:     ['read', 'write'],
  volusion:     ['read', 'write'],
  zapier_nla:   ['read', 'write'],
  zuora:        ['read', 'write'],
};

function normalizeScopes(value: string): string[] {
  return value
    .split(/\s+/)
    .map((s) => s.trim())
    .filter(Boolean);
}

type Props = {
  value: string;
  providerKey?: string;
  /** Available scopes from the catalog for this provider. */
  availableScopes?: string[];
  onChange: (value: string) => void;
};

export default function ScopeSelector({ value, providerKey, availableScopes, onChange }: Props) {
  const [candidate, setCandidate] = useState('');
  const [custom, setCustom] = useState('');
  const [showCustom, setShowCustom] = useState(false);
  const selected = useMemo(() => normalizeScopes(value), [value]);
  const options = useMemo(() => {
    // Prefer catalog-provided scopes; fall back to hardcoded suggestions
    const catalogScopes = availableScopes || [];
    const key = (providerKey || '').trim().toLowerCase();
    const fallbackScopes = key ? (SCOPE_SUGGESTIONS[key] || []) : [];
    const base = catalogScopes.length > 0 ? catalogScopes : fallbackScopes;
    return base.filter((s) => !selected.includes(s));
  }, [providerKey, availableScopes, selected]);

  function write(next: string[]) {
    onChange(next.join(' '));
  }

  function addScope(scope: string) {
    const s = scope.trim();
    if (!s || selected.includes(s)) return;
    write([...selected, s]);
    setCandidate('');
    setCustom('');
  }

  function removeScope(scope: string) {
    write(selected.filter((s) => s !== scope));
  }

  return (
    <div>
      <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.5rem', alignItems: 'center' }}>
        <select
          value={candidate}
          onChange={(e) => setCandidate(e.target.value)}
          style={{ flex: 1, padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
        >
          <option value="">{options.length > 0 ? 'Select a scope…' : 'No preset scopes for this provider'}</option>
          {options.map((s) => (
            <option key={s} value={s}>{s}</option>
          ))}
        </select>
        <button
          type="button"
          onClick={() => addScope(candidate)}
          disabled={!candidate}
          style={{ padding: '0.45rem 0.8rem', border: '1px solid #cbd5e1', borderRadius: '4px', background: '#fff', cursor: 'pointer' }}
        >
          Add
        </button>
      </div>
      <div style={{ marginBottom: '0.5rem' }}>
        <button
          type="button"
          onClick={() => setShowCustom((v) => !v)}
          style={{ padding: '0.35rem 0.6rem', border: '1px dashed #cbd5e1', borderRadius: '4px', background: '#fff', cursor: 'pointer', fontSize: '0.75rem', color: '#475569' }}
        >
          {showCustom ? 'Hide custom scope input' : 'Need a custom scope?'}
        </button>
      </div>
      {showCustom && (
        <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.5rem' }}>
          <input
            type="text"
            value={custom}
            onChange={(e) => setCustom(e.target.value)}
            placeholder="Enter custom scope"
            style={{ flex: 1, padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
          />
          <button
            type="button"
            onClick={() => addScope(custom)}
            disabled={!custom.trim()}
            style={{ padding: '0.45rem 0.8rem', border: '1px solid #cbd5e1', borderRadius: '4px', background: '#fff', cursor: 'pointer' }}
          >
            Add custom
          </button>
        </div>
      )}
      {selected.length > 0 ? (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
          {selected.map((scope) => (
            <button
              key={scope}
              type="button"
              onClick={() => removeScope(scope)}
              style={{ padding: '0.2rem 0.5rem', borderRadius: '9999px', border: '1px solid #cbd5e1', background: '#f8fafc', color: '#334155', fontSize: '0.75rem', cursor: 'pointer' }}
              title="Remove scope"
            >
              {scope} ×
            </button>
          ))}
        </div>
      ) : (
        <div style={{ fontSize: '0.75rem', color: '#999' }}>No scopes selected yet.</div>
      )}
    </div>
  );
}
