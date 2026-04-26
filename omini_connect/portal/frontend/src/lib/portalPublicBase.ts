import { healthCheck } from '../api/client';

let cached: string | null | undefined;

/** Resolved once per page load; matches server `PORTAL_BASE_URL` when set. */
export async function getPortalPublicBase(): Promise<string> {
  if (cached !== undefined) {
    return cached ?? (typeof window !== 'undefined' ? window.location.origin : '');
  }
  try {
    const s = await healthCheck();
    const raw = (s.portal_base_url ?? '').trim().replace(/\/+$/, '');
    if (raw) {
      cached = raw;
      return raw;
    }
  } catch {
    // ignore
  }
  cached = null;
  return typeof window !== 'undefined' ? window.location.origin : '';
}
