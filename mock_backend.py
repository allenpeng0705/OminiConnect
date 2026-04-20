#!/usr/bin/env python3
"""
Simple mock backend server to test MCP tool cache metrics
"""

from http.server import HTTPServer, BaseHTTPRequestHandler
import json
import threading

class MockHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = {"status": "ok", "data": "mock response"}
        self.wfile.write(json.dumps(response).encode())
    
    def do_POST(self):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = {"status": "ok", "data": "mock response"}
        self.wfile.write(json.dumps(response).encode())
    
    def log_message(self, format, *args):
        # Suppress logs
        pass

def start_mock_server(port=5023):
    server = HTTPServer(('127.0.0.1', port), MockHandler)
    thread = threading.Thread(target=server.serve_forever)
    thread.daemon = True
    thread.start()
    print(f"Mock backend server started on port {port}")
    return server

if __name__ == "__main__":
    server = start_mock_server()
    print("Press Ctrl+C to stop")
    try:
        while True:
            pass
    except KeyboardInterrupt:
        server.shutdown()
