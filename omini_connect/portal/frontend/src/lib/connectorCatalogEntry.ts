/** Matches server `catalog_entry_key` for managed connectors vs built-in platform id. */
export function catalogEntryKey(c: { engine?: string; platform: string; provider_key?: string }): string {
  const engine = (c.engine ?? 'omini_connect_native').trim();
  const pk = (c.provider_key ?? '').trim();
  if (engine === 'nango') {
    return (pk.split('__')[0] || pk).trim().toLowerCase();
  }
  return c.platform.trim().toLowerCase();
}

/** If this catalog provider (`name`) already has a managed connector, return its `platform` id. */
export function findConnectorPlatformForCatalogProvider(
  connectors: { engine?: string; platform: string; provider_key?: string }[],
  catalogProviderName: string,
): string | null {
  const want = catalogProviderName.trim().toLowerCase();
  for (const c of connectors) {
    if ((c.engine ?? '').trim() === 'nango' && catalogEntryKey(c) === want) {
      return c.platform;
    }
  }
  return null;
}
