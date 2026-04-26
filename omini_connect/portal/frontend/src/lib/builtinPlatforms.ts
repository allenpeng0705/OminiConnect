/**
 * OminiConnect “built-in” platform metadata.
 *
 * Policy: **do not** add a new dashboard (Chinese Services) card for a provider that the managed
 * integration library (Nango) already offers — users connect those via Global Services.
 *
 * `showOnDashboard: true` — listed on the home page (Omini-only additions: not in a typical Nango catalog).
 * `showOnDashboard: false` — legacy / metadata only: existing `platform` rows still get name + logo, but
 *   new users should use the integration library for the equivalent provider.
 *
 * `connectorId` in the DB is often `dingtalk` or a namespaced `dingtalk-a1b2c3`; the latter must still
 * resolve the same icon/name as the base id where applicable.
 */
export type BuiltinPlatform = {
  id: string;
  name: string;
  color: string;
  type: 'oauth2' | 'api_key';
  /** Optional default scopes (OAuth2) */
  scopes_default?: string;
  /**
   * Default logo: same path scheme as the integration library and `nango.rs` (`/images/template-logos/{nangoKey}.svg`).
   * Nango’s key for 企业微信 is `wecom`, not `wechatwork` — the UI id stays `wechatwork` for URLs and DB.
   */
  logo_url: string;
  /** If true, show a card on the home page under “Chinese Services”. */
  showOnDashboard: boolean;
};

const ALL: BuiltinPlatform[] = [
  {
    id: 'wechatwork',
    name: 'WeChat Work',
    color: '#07C160',
    type: 'oauth2',
    logo_url: '/images/template-logos/wecom.svg',
    showOnDashboard: false,
  },
  {
    id: 'dingtalk',
    name: 'DingTalk',
    color: '#1677FF',
    type: 'oauth2',
    logo_url: '/images/template-logos/dingtalk.svg',
    showOnDashboard: true,
  },
  {
    id: 'feishu',
    name: 'Feishu / Lark',
    color: '#00A1E0',
    type: 'oauth2',
    scopes_default: 'contact:user.base:readonly',
    logo_url: '/images/template-logos/feishu.svg',
    showOnDashboard: true,
  },
  {
    id: 'qqmail',
    name: 'QQ Enterprise Mail',
    color: '#12B7F5',
    type: 'api_key',
    logo_url: '/images/template-logos/tencent-qq.svg',
    showOnDashboard: true,
  },
  {
    id: 'maton',
    name: 'Maton.ai',
    color: '#6366F1',
    type: 'api_key',
    logo_url: '/images/template-logos/maton.svg',
    showOnDashboard: true,
  },
];

/** Longer `id` first (e.g. wechatwork before any future shorter prefix). */
const PREFIX_ORDER: BuiltinPlatform[] = [...ALL].sort((a, b) => b.id.length - a.id.length);

const BY_ID: Record<string, BuiltinPlatform> = Object.fromEntries(ALL.map((p) => [p.id, p]));

/**
 * Resolves a saved connector `platform` id to built-in row metadata, including
 * `dingtalk-abc123` → DingTalk.
 */
export function findBuiltinByConnectorId(connectorId: string | undefined): BuiltinPlatform | undefined {
  if (!connectorId) return undefined;
  const exact = BY_ID[connectorId];
  if (exact) return exact;
  for (const p of PREFIX_ORDER) {
    if (connectorId === p.id || connectorId.startsWith(`${p.id}-`)) {
      return p;
    }
  }
  return undefined;
}

/** Home page list: Omini-only providers; do not add an id here if Nango’s library already ships that integration. */
export const BUILTIN_OMINI_PLATFORMS: BuiltinPlatform[] = ALL.filter((p) => p.showOnDashboard);

/** Spreads into ConnectorConfig `PLATFORMS` for all known built-in metadata (incl. legacy ids). */
export const BUILTIN_OMINI_PLATFORM_RECORD: Record<string, Pick<BuiltinPlatform, 'name' | 'color' | 'type' | 'scopes_default' | 'logo_url'>> = Object.fromEntries(
  ALL.map((p) => [
    p.id,
    {
      name: p.name,
      color: p.color,
      type: p.type,
      scopes_default: p.scopes_default,
      logo_url: p.logo_url,
    },
  ])
);
