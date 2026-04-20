#!/usr/bin/env python3
"""HTTP/1.1 mock corporate API for gateway MCP E2E and multi-server panda.yaml demos.

Serves several GET paths on one port so one process backs multiple `mcp.servers` entries
(`http_tool` / `http_tools`) through a single `egress.corporate.default_base`.

Paths mirror integration tests (docs/testing_mcp_api_gateway.md).
"""
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import sys

# Path (no query) -> JSON body for GET
ROUTES: dict[str, dict] = {
    "/allowed/toolpath": {"ok": True, "via": "mock_corp_api"},
    "/corp/service-a": {"service": "A"},
    "/corp/service-b": {"service": "B"},
    "/api/hi": {"via": "rest", "message": "hello"},
    "/v1/status": {"status": "ok", "component": "inventory"},
}


class Handler(BaseHTTPRequestHandler):
    def do_GET(self) -> None:
        path = self.path.split("?", 1)[0]
        payload = ROUTES.get(path)
        if payload is None:
            self.send_error(404)
            return
        body = json.dumps(payload).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, fmt: str, *args) -> None:
        pass


def main() -> None:
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 18081
    server = HTTPServer(("127.0.0.1", port), Handler)
    print(f"mock_corp_api listening on http://127.0.0.1:{port}", flush=True)
    print("paths:", ", ".join(sorted(ROUTES)), flush=True)
    server.serve_forever()


if __name__ == "__main__":
    main()
