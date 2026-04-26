import type { IntegrationCatalogRow } from '../api/client';

/** Nango may send `categories` as string[], a single string, or other shapes — normalize so we never crash on `.join` / React keys. */
export function normalizeCategories(raw: unknown): string[] {
  if (raw == null) return [];
  if (Array.isArray(raw)) {
    return raw.map((c) => (typeof c === 'string' ? c : String(c))).filter(Boolean);
  }
  if (typeof raw === 'string') return raw.trim() ? [raw.trim()] : [];
  return [];
}

export function normalizeCatalogRow(r: unknown): IntegrationCatalogRow | null {
  if (!r || typeof r !== 'object') return null;
  const o = r as Record<string, unknown>;
  const name = typeof o.name === 'string' ? o.name.trim() : '';
  if (!name) return null;
  const display_name = typeof o.display_name === 'string' && o.display_name.trim() ? o.display_name.trim() : name;
  const logo_url = typeof o.logo_url === 'string' ? o.logo_url : '';
  const auth_mode = typeof o.auth_mode === 'string' ? o.auth_mode : undefined;
  const docs = typeof o.docs === 'string' ? o.docs : undefined;
  // available_scopes: array of strings, each scope string
  let available_scopes: string[] | undefined;
  if (Array.isArray(o.available_scopes)) {
    available_scopes = o.available_scopes
      .map((s) => (typeof s === 'string' ? s.trim() : ''))
      .filter(Boolean);
    if (available_scopes.length === 0) available_scopes = undefined;
  }
  return {
    name,
    display_name,
    logo_url,
    auth_mode,
    categories: normalizeCategories(o.categories),
    docs,
    ...(available_scopes ? { available_scopes } : {}),
  };
}

export function providerSearchBlob(p: IntegrationCatalogRow): string {
  const cats = (p.categories || []).join(' ');
  const scopes = (p.available_scopes || []).join(' ');
  return `${p.name} ${p.display_name} ${cats} ${p.auth_mode || ''} ${scopes}`.toLowerCase();
}

/** Normalize raw API JSON into sorted rows (by display name). */
export function normalizeCatalogResponse(data: unknown): IntegrationCatalogRow[] {
  const cleaned = (Array.isArray(data) ? data : [])
    .map((r) => normalizeCatalogRow(r))
    .filter((r): r is IntegrationCatalogRow => r != null);
  cleaned.sort((a, b) => (a.display_name || a.name).localeCompare(b.display_name || b.name, undefined, { sensitivity: 'base' }));
  return cleaned;
}
