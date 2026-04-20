/**
 * Matches crates/panda-server/schemas/live_trace_ws.v1.schema.json (envelope).
 */
export type LiveTraceKind =
  | "request_started"
  | "request_finished"
  | "request_failed"
  | "mcp_call"
  | "llm_trace";

export interface LiveTraceMessage {
  version: "v1";
  request_id: string;
  trace_id?: string | null;
  ts_unix_ms: number;
  stage: string;
  kind: LiveTraceKind;
  method: string;
  route: string;
  status?: number | null;
  elapsed_ms?: number | null;
  payload?: Record<string, unknown> | null;
}

export interface ConsoleBootstrap {
  admin_auth_required: boolean;
  admin_auth_header: string;
}

export interface TpmStatus {
  enforce_budget: boolean;
  bucket?: string;
  limit?: number;
  used?: number;
  remaining?: number;
  retry_after_seconds?: number;
  totals?: {
    prompt_tokens: number;
    completion_tokens: number;
  };
  tokens_per_minute?: {
    prompt_window_used: number;
    limit: number;
  };
  pricing?: {
    blend_usd_per_million_tokens: number;
    estimated_cumulative_usd: number;
    estimated_current_window_usd: number;
  };
}
