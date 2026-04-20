import { useCallback, useEffect, useMemo, useState } from "react";
import type { ConsoleBootstrap, LiveTraceMessage, TpmStatus } from "./types";

const OPS_STORE = "panda_console_ops_secret";

interface TraceRow {
  events: LiveTraceMessage[];
  thought: string;
  route: string;
  method: string;
}

function fmtPayload(o: LiveTraceMessage): string {
  try {
    return JSON.stringify(o.payload ?? {}, null, 2);
  } catch {
    return "";
  }
}

export default function App() {
  const [traces, setTraces] = useState<Map<string, TraceRow>>(() => new Map());
  const [selected, setSelected] = useState<string | null>(null);
  const [filter, setFilter] = useState("");
  const [wsStatus, setWsStatus] = useState("connecting…");
  const [bootstrap, setBootstrap] = useState<ConsoleBootstrap | null>(null);
  const [opsSecret, setOpsSecret] = useState(() =>
    typeof sessionStorage !== "undefined" ? sessionStorage.getItem(OPS_STORE) ?? "" : "",
  );
  const [opsDraft, setOpsDraft] = useState("");
  const [tpm, setTpm] = useState<TpmStatus | null>(null);

  const opsHeaders = useMemo(() => {
    if (!bootstrap || !opsSecret.trim()) return new Headers();
    const h = new Headers();
    h.set(bootstrap.admin_auth_header, opsSecret.trim());
    return h;
  }, [bootstrap, opsSecret]);

  const saveOpsSecret = useCallback(() => {
    const v = opsDraft.trim();
    sessionStorage.setItem(OPS_STORE, v);
    setOpsSecret(v);
  }, [opsDraft]);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const r = await fetch("/console/api/meta");
        if (!r.ok) return;
        const j = (await r.json()) as ConsoleBootstrap;
        if (!cancelled) setBootstrap(j);
      } catch {
        /* ignore */
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    const t = window.setInterval(async () => {
      try {
        const r = await fetch("/tpm/status", { headers: opsHeaders });
        if (!r.ok) return;
        const j = (await r.json()) as TpmStatus;
        setTpm(j);
      } catch {
        /* ignore */
      }
    }, 1500);
    void (async () => {
      try {
        const r = await fetch("/tpm/status", { headers: opsHeaders });
        if (r.ok) setTpm((await r.json()) as TpmStatus);
      } catch {
        /* ignore */
      }
    })();
    return () => clearInterval(t);
  }, [opsHeaders]);

  useEffect(() => {
    const proto = location.protocol === "https:" ? "wss:" : "ws:";
    let url = `${proto}//${location.host}/console/ws`;
    if (opsSecret.trim()) {
      const q = new URLSearchParams({ panda_ops_secret: opsSecret.trim() });
      url += `?${q.toString()}`;
    }
    const ws = new WebSocket(url);
    ws.onopen = () => setWsStatus("connected");
    ws.onclose = () => setWsStatus("disconnected");
    ws.onerror = () => setWsStatus("websocket error");
    ws.onmessage = (ev) => {
      let o: LiveTraceMessage;
      try {
        o = JSON.parse(ev.data) as LiveTraceMessage;
      } catch {
        return;
      }
      const id = o.request_id || "unknown";
      setTraces((prev) => {
        const next = new Map(prev);
        const row = next.get(id) ?? {
          events: [],
          thought: "",
          route: "",
          method: "",
        };
        if (o.route) row.route = o.route;
        if (o.method) row.method = o.method;
        row.events = [...row.events, o];
        if (o.kind === "llm_trace") {
          const tail = o.payload?.text_tail;
          if (typeof tail === "string") row.thought = tail;
        }
        next.set(id, row);
        return next;
      });
      setSelected((sel) => sel ?? id);
    };
    return () => ws.close();
  }, [opsSecret]);

  const q = filter.toLowerCase();
  const sidebarIds = useMemo(() => {
    const ids = [...traces.keys()].filter((id) => {
      if (!q) return true;
      const t = traces.get(id);
      return (
        id.toLowerCase().includes(q) ||
        (t?.route && t.route.toLowerCase().includes(q))
      );
    });
    return ids.slice(-80).reverse();
  }, [traces, q]);

  const selRow = selected ? traces.get(selected) : undefined;

  const tpmUsed = tpm?.used ?? tpm?.tokens_per_minute?.prompt_window_used ?? 0;
  const tpmLimit = tpm?.limit ?? tpm?.tokens_per_minute?.limit ?? 0;
  const tpmPct =
    tpmLimit > 0 ? Math.min(100, (tpmUsed / tpmLimit) * 100) : 0;

  return (
    <>
      <header
        style={{
          padding: "10px 14px",
          borderBottom: "1px solid #252a35",
          display: "flex",
          alignItems: "center",
          gap: 14,
          flexWrap: "wrap",
          background: "var(--panel)",
        }}
      >
        <h1 style={{ fontSize: 14, fontWeight: 600, margin: 0 }}>
          Panda Live Trace
        </h1>
        <span style={{ fontSize: 11, color: "var(--muted)" }}>
          Brain of the agent — real-time
        </span>
        <input
          style={{
            flex: 1,
            minWidth: 140,
            maxWidth: 280,
            background: "#1a1f28",
            border: "1px solid #2a3140",
            color: "var(--fg)",
            borderRadius: 6,
            padding: "6px 10px",
            fontSize: 12,
          }}
          type="search"
          placeholder="Filter by route / id…"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
        />
        <span style={{ fontSize: 11, color: "var(--accent)" }}>{wsStatus}</span>
        {bootstrap?.admin_auth_required ? (
          <div
            style={{
              display: "flex",
              gap: 8,
              alignItems: "center",
              fontSize: 11,
            }}
          >
            <input
              type="password"
              placeholder="Ops secret"
              value={opsDraft}
              onChange={(e) => setOpsDraft(e.target.value)}
              style={{
                background: "#1a1f28",
                border: "1px solid #2a3140",
                color: "var(--fg)",
                borderRadius: 6,
                padding: "4px 8px",
                width: 140,
              }}
            />
            <button
              type="button"
              onClick={saveOpsSecret}
              style={{
                background: "#2a3140",
                border: "none",
                color: "var(--fg)",
                borderRadius: 6,
                padding: "4px 10px",
                cursor: "pointer",
              }}
            >
              Apply
            </button>
          </div>
        ) : null}
      </header>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(180px, 1fr))",
          gap: 10,
          padding: "10px 14px",
          background: "#0e1117",
          borderBottom: "1px solid #252a35",
        }}
      >
        <div
          style={{
            background: "var(--panel)",
            borderRadius: 8,
            padding: "10px 12px",
            border: "1px solid #252a35",
          }}
        >
          <div
            style={{
              fontSize: 10,
              textTransform: "uppercase",
              color: "var(--muted)",
              letterSpacing: "0.06em",
            }}
          >
            TPM (prompt window)
          </div>
          <div style={{ fontSize: 20, fontWeight: 600, marginTop: 4 }}>
            {tpm?.enforce_budget && tpmLimit > 0
              ? `${tpmUsed.toLocaleString()} / ${tpmLimit.toLocaleString()}`
              : tpm?.enforce_budget
                ? "—"
                : "no cap"}
          </div>
          {tpm?.enforce_budget && tpmLimit > 0 ? (
            <div
              style={{
                height: 6,
                background: "#252a35",
                borderRadius: 3,
                marginTop: 8,
                overflow: "hidden",
              }}
            >
              <div
                style={{
                  height: "100%",
                  width: `${tpmPct}%`,
                  background:
                    tpmPct > 90
                      ? "var(--danger)"
                      : tpmPct > 70
                        ? "var(--warn)"
                        : "var(--ok)",
                  transition: "width 0.3s ease",
                }}
              />
            </div>
          ) : null}
          <div style={{ fontSize: 10, color: "var(--muted)", marginTop: 6 }}>
            bucket: {tpm?.bucket ?? "—"}
          </div>
        </div>

        <div
          style={{
            background: "var(--panel)",
            borderRadius: 8,
            padding: "10px 12px",
            border: "1px solid #252a35",
          }}
        >
          <div
            style={{
              fontSize: 10,
              textTransform: "uppercase",
              color: "var(--muted)",
              letterSpacing: "0.06em",
            }}
          >
            Totals (prompt + completion)
          </div>
          <div style={{ fontSize: 16, marginTop: 4 }}>
            in: {(tpm?.totals?.prompt_tokens ?? 0).toLocaleString()} · out:{" "}
            {(tpm?.totals?.completion_tokens ?? 0).toLocaleString()}
          </div>
        </div>

        <div
          style={{
            background: "var(--panel)",
            borderRadius: 8,
            padding: "10px 12px",
            border: "1px solid #252a35",
          }}
        >
          <div
            style={{
              fontSize: 10,
              textTransform: "uppercase",
              color: "var(--muted)",
              letterSpacing: "0.06em",
            }}
          >
            Est. spend (blend)
          </div>
          <div style={{ fontSize: 20, fontWeight: 600, marginTop: 4 }}>
            {tpm?.pricing
              ? `$${tpm.pricing.estimated_cumulative_usd.toFixed(4)}`
              : "—"}
          </div>
          <div style={{ fontSize: 10, color: "var(--muted)", marginTop: 6 }}>
            {tpm?.pricing
              ? `window ~$${tpm.pricing.estimated_current_window_usd.toFixed(4)} @ $${tpm.pricing.blend_usd_per_million_tokens}/M`
              : "Set PANDA_CONSOLE_BLEND_PRICE_PER_MILLION_TOKENS"}
          </div>
        </div>
      </div>

      <main
        style={{
          flex: 1,
          display: "grid",
          gridTemplateColumns: "minmax(200px, 280px) 1fr",
          minHeight: 0,
        }}
      >
        <nav
          style={{
            borderRight: "1px solid #252a35",
            overflow: "auto",
            background: "#0e1117",
            padding: "8px 0",
          }}
        >
          {sidebarIds.map((id) => {
            const t = traces.get(id)!;
            return (
              <div
                key={id}
                role="button"
                tabIndex={0}
                onClick={() => setSelected(id)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" || e.key === " ") setSelected(id);
                }}
                style={{
                  padding: "8px 12px",
                  cursor: "pointer",
                  fontSize: 12,
                  borderLeft:
                    id === selected
                      ? "3px solid var(--accent)"
                      : "3px solid transparent",
                  background: id === selected ? "#1c2430" : "transparent",
                }}
              >
                <div
                  style={{
                    fontFamily: "ui-monospace, Menlo, monospace",
                    fontSize: 11,
                    color: "var(--accent)",
                    wordBreak: "break-all",
                  }}
                >
                  {id.slice(0, 36)}
                  {id.length > 36 ? "…" : ""}
                </div>
                <div style={{ color: "var(--muted)", fontSize: 10, marginTop: 2 }}>
                  {t.method} {t.route} · {t.events.length} ev
                </div>
              </div>
            );
          })}
        </nav>
        <section
          style={{
            display: "flex",
            flexDirection: "column",
            minHeight: 0,
            overflow: "hidden",
          }}
        >
          <div
            style={{
              padding: "10px 14px",
              borderBottom: "1px solid #252a35",
              fontSize: 12,
              color: "var(--muted)",
            }}
          >
            {selRow ? (
              <>
                <strong style={{ color: "var(--fg)" }}>{selected}</strong> ·{" "}
                {selRow.method} {selRow.route}
              </>
            ) : (
              <>Select a request to inspect the AI path.</>
            )}
          </div>
          <div
            style={{
              flex: 1,
              display: "grid",
              gridTemplateRows: "1fr 1fr",
              minHeight: 0,
            }}
          >
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                minHeight: 0,
                borderBottom: "1px solid #252a35",
              }}
            >
              <h2
                style={{
                  margin: 0,
                  padding: "8px 14px",
                  fontSize: 11,
                  fontWeight: 600,
                  textTransform: "uppercase",
                  letterSpacing: "0.06em",
                  color: "var(--muted)",
                  background: "#0e1117",
                }}
              >
                Timeline
              </h2>
              <div
                style={{
                  flex: 1,
                  overflow: "auto",
                  padding: "10px 14px",
                  fontFamily: "ui-monospace, Menlo, monospace",
                  fontSize: 11,
                  lineHeight: 1.5,
                }}
              >
                {!selRow?.events.length ? (
                  <span style={{ color: "var(--muted)", fontStyle: "italic" }}>
                    No events.
                  </span>
                ) : (
                  selRow.events.map((ev, i) => {
                    const border =
                      ev.kind === "mcp_call"
                        ? "var(--warn)"
                        : ev.kind === "llm_trace"
                          ? "var(--ok)"
                          : "#2a3140";
                    let detail = "";
                    if (ev.status) detail += ` HTTP ${ev.status}`;
                    if (ev.elapsed_ms != null) detail += ` ${ev.elapsed_ms}ms`;
                    if (ev.payload && ev.kind === "mcp_call") {
                      detail += "\n" + fmtPayload(ev).slice(0, 600);
                    }
                    return (
                      <div
                        key={i}
                        style={{
                          padding: "4px 0",
                          borderLeft: `2px solid ${border}`,
                          paddingLeft: 10,
                          marginBottom: 4,
                          whiteSpace: "pre-wrap",
                          wordBreak: "break-word",
                        }}
                      >
                        <span style={{ color: "var(--muted)" }}>
                          [{ev.ts_unix_ms}]{" "}
                        </span>
                        <span
                          style={{ color: "var(--accent)", fontWeight: 500 }}
                        >
                          {ev.kind}
                        </span>
                        <span style={{ color: "var(--fg)" }}>{detail}</span>
                      </div>
                    );
                  })
                )}
              </div>
            </div>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                minHeight: 0,
              }}
            >
              <h2
                style={{
                  margin: 0,
                  padding: "8px 14px",
                  fontSize: 11,
                  fontWeight: 600,
                  textTransform: "uppercase",
                  letterSpacing: "0.06em",
                  color: "var(--muted)",
                  background: "#0e1117",
                }}
              >
                Thought stream
              </h2>
              <div style={{ flex: 1, overflow: "auto", padding: "10px 14px" }}>
                <div
                  style={{
                    whiteSpace: "pre-wrap",
                    wordBreak: "break-word",
                    fontFamily: "ui-serif, Georgia, serif",
                    fontSize: 13,
                    lineHeight: 1.55,
                    color: "#d8dde4",
                  }}
                >
                  {selRow?.thought ? (
                    selRow.thought
                  ) : (
                    <span style={{ color: "var(--muted)", fontStyle: "italic" }}>
                      No streaming text for this request.
                    </span>
                  )}
                </div>
              </div>
            </div>
          </div>
          <div
            style={{
              padding: "6px 14px",
              borderTop: "1px solid #252a35",
              fontFamily: "ui-monospace, Menlo, monospace",
              fontSize: 10,
              color: "var(--muted)",
              maxHeight: 120,
              overflow: "auto",
              whiteSpace: "pre-wrap",
            }}
          >
            {selRow?.events.length
              ? JSON.stringify(
                  selRow.events[selRow.events.length - 1],
                  null,
                  2,
                )
              : ""}
          </div>
        </section>
      </main>
      <style>{`
        @media (max-width: 720px) {
          main { grid-template-columns: 1fr !important; }
          nav { max-height: 28vh; border-right: none !important; border-bottom: 1px solid #252a35; }
        }
      `}</style>
    </>
  );
}
