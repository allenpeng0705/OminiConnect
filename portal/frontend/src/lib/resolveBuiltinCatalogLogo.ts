/**
 * OminiConnect “Chinese services” use stable platform ids (`wechatwork`, `feishu`, …) while the
 * Nango integration library uses the **provider** keys from `providers.json` (`wecom`, `feishu`, …).
 * Logos in the API are the same as the global library: `/images/template-logos/{provider}.svg`
 * (see `omini_connect/portal/src/nango.rs`). We look up the catalog by these aliases and fall back
 * to the built-in `logo_url` (same path scheme) so both sections show the same artwork.
 */
const NANGO_CATALOG_NAME_ALIASES: Readonly<Record<string, readonly string[]>> = {
  wechatwork: ['wecom', 'wechatwork', 'wechat'],
  feishu: ['feishu', 'lark', 'lark-international'],
  dingtalk: ['dingtalk'],
  qqmail: ['tencent-qq', 'tencentqq', 'qq', 'exmail-qq', 'tencent-corp'],
  maton: ['maton', 'maton-ai', 'matonai'],
};

/**
 * Prefer the integration-catalog `logo_url` when a row exists for a matching Nango provider name;
 * otherwise use `fallbackUrl` (typically `/images/template-logos/...` from the built-in list).
 */
export function resolveBuiltinCatalogLogo(
  builtinId: string,
  catalogNameToLogo: Map<string, string>,
  fallbackUrl: string
): string {
  const tryKeys: string[] = [builtinId, ...(NANGO_CATALOG_NAME_ALIASES[builtinId] || [])];
  for (const k of tryKeys) {
    const u = catalogNameToLogo.get(k.toLowerCase().trim());
    if (u?.trim()) {
      return u.trim();
    }
  }
  return fallbackUrl;
}
