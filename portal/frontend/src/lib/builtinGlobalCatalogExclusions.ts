import { BUILTIN_OMINI_PLATFORMS } from './builtinPlatforms';

/**
 * Hides a catalog (Nango) row when the same provider is also offered as a home-page built-in, so
 * we never list one service in both Global Services and “Chinese Services”.
 *
 * Nango can list GitHub, Feishu, etc.; those are not hidden here. Only Omini-only dashboard
 * providers (see `BUILTIN_OMINI_PLATFORMS`) are removed from the integration library.
 */
const DASHBOARD_ONLY_IDS_LOWER = new Set(BUILTIN_OMINI_PLATFORMS.map((p) => p.id.toLowerCase()));

export function catalogNameExcludedFromGlobalLibrary(catalogName: string): boolean {
  return DASHBOARD_ONLY_IDS_LOWER.has(catalogName.trim().toLowerCase());
}

export function filterCatalogExcludingBuiltinDupes<T extends { name: string }>(rows: T[]): T[] {
  return rows.filter((r) => !catalogNameExcludedFromGlobalLibrary(r.name));
}
