# Benchmarks

## Proxy overhead (loopback, hyperfine)

This measures **extra latency from routing through Panda** versus calling the same upstream **directly**. Both requests hit a tiny local Python HTTP server that returns a minimal chat JSON immediately (no real OpenAI call).

On the same machine, the median delta is typically **well under 1 ms** for a minimal JSON response. End-to-end calls to `api.openai.com` are dominated by WAN RTT; the **incremental** cost of Panda is still the same proxy hop you measure here.

### Requirements

- `hyperfine` ([installation](https://github.com/sharkdp/hyperfine))
- `python3`, `curl`, `docker` (for chaos script only; not needed for hyperfine)
- A built `panda` binary (`cargo build -p panda-server`)

### Run

From the repository root:

```bash
./benchmarks/run_proxy_overhead_hyperfine.sh
```

Optional:

```bash
export BENCHMARK_PANDA_PORT=19222
export BENCHMARK_UPSTREAM_PORT=19223
export HYPERFINE_RUNS=50
./benchmarks/run_proxy_overhead_hyperfine.sh
```

Results print to stdout; with `hyperfine` 1.18+ you can add exports inside the script if you want CSV/JSON.

### wrk (throughput)

For sustained QPS, install [wrk](https://github.com/wg/wrk) and adapt the URLs from `run_proxy_overhead_hyperfine.sh` (same `BASE` paths). A minimal Lua script can POST the same JSON body; compare `Requests/sec` direct vs via Panda. Throughput tests are environment-sensitive; treat as regression detectors, not absolute scores.

## Related scripts

- Load profile: `scripts/load_profile_chat.sh`
- SSE soak: `scripts/soak_guard_sse.sh`
- Redis failover (TPM): `scripts/tpm_redis_failover_soak.sh`
- **Chaos (streaming + Redis + MCP):** `scripts/chaos_monkey_streaming.sh`
