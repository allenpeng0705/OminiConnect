#!/usr/bin/env python3
"""MCP: RAG-lite — index local text files and keyword-search (stdlib only)."""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "common"))
from mcp_stdio import run_stdio, tool_text  # noqa: E402

# doc_id -> full text
_INDEX: Dict[str, str] = {}
_TOKEN_RE = re.compile(r"[a-z0-9]+", re.I)


def tokenize(text: str) -> List[str]:
    return [t.lower() for t in _TOKEN_RE.findall(text)]


def index_docs(root: str, glob_pat: str = "**/*.txt", max_files: int = 200) -> int:
    global _INDEX
    _INDEX = {}
    base = Path(root).expanduser().resolve()
    if not base.is_dir():
        raise NotADirectoryError(root)
    n = 0
    for p in sorted(base.glob(glob_pat)):
        if not p.is_file() or n >= max_files:
            break
        try:
            txt = p.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        doc_id = str(p.relative_to(base))
        _INDEX[doc_id] = txt
        n += 1
    return n


def search(query: str, top_k: int = 8) -> List[Tuple[str, float]]:
    q_terms = set(tokenize(query))
    if not q_terms:
        return []
    scores: List[Tuple[str, float]] = []
    for doc_id, body in _INDEX.items():
        terms = tokenize(body)
        if not terms:
            continue
        tf = {}
        for t in terms:
            tf[t] = tf.get(t, 0) + 1
        score = 0.0
        for t in q_terms:
            score += min(tf.get(t, 0), 10)
        score += sum(body.lower().count(t) * 0.01 for t in q_terms if len(t) > 2)
        if score > 0:
            scores.append((doc_id, score))
    scores.sort(key=lambda x: -x[1])
    return scores[:top_k]


TOOLS = [
    {
        "name": "rag_index",
        "description": "Index text files under a root directory (default glob **/*.txt).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "root": {"type": "string", "description": "Directory to scan"},
                "glob": {
                    "type": "string",
                    "description": "Glob relative to root",
                    "default": "**/*.txt",
                },
                "max_files": {"type": "integer", "default": 200},
            },
            "required": ["root"],
        },
    },
    {
        "name": "rag_search",
        "description": "Keyword search over indexed documents; returns top passages.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "query": {"type": "string"},
                "top_k": {"type": "integer", "default": 5},
                "snippet_chars": {"type": "integer", "default": 400},
            },
            "required": ["query"],
        },
    },
    {
        "name": "rag_stats",
        "description": "Show how many documents are indexed.",
        "inputSchema": {"type": "object", "properties": {}},
    },
]


def snippet(doc_id: str, body: str, query: str, max_chars: int) -> str:
    low = body.lower()
    q = query.lower().split()
    pos = 0
    for term in q:
        if len(term) < 3:
            continue
        i = low.find(term)
        if i >= 0:
            pos = max(0, i - 80)
            break
    chunk = body[pos : pos + max_chars].replace("\n", " ")
    return f"--- {doc_id} ---\n{chunk}\n"


def dispatch(name: str, args: dict) -> dict:
    if name == "rag_index":
        root = args["root"]
        glob_pat = args.get("glob") or "**/*.txt"
        max_files = int(args.get("max_files") or 200)
        n = index_docs(root, glob_pat, max_files)
        return tool_text(f"indexed {n} files under {root} ({glob_pat})")
    if name == "rag_search":
        if not _INDEX:
            return tool_text("index empty: call rag_index first", is_error=True)
        q = args["query"]
        top_k = int(args.get("top_k") or 5)
        snip = int(args.get("snippet_chars") or 400)
        hits = search(q, top_k=top_k)
        if not hits:
            return tool_text("no hits")
        parts = []
        for doc_id, sc in hits:
            parts.append(f"[{sc:.2f}] {snippet(doc_id, _INDEX[doc_id], q, snip)}")
        return tool_text("\n".join(parts))
    if name == "rag_stats":
        return tool_text(f"documents indexed: {len(_INDEX)}")
    return tool_text(f"unknown tool: {name}", is_error=True)


if __name__ == "__main__":
    # Optional default root from env for demos
    default_root = os.environ.get("RAG_LITE_ROOT", "").strip()
    if default_root:
        try:
            index_docs(default_root)
        except OSError:
            pass
    run_stdio(TOOLS, dispatch, "panda-registry-rag-lite")
