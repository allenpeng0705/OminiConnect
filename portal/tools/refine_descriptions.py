#!/usr/bin/env python3
"""
Refine tool descriptions across all provider registry files.

Usage:
  python portal/tools/refine_descriptions.py --dry-run
  python portal/tools/refine_descriptions.py --apply
  python portal/tools/refine_descriptions.py --apply --provider github
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

import yaml


ROOT = Path(__file__).resolve().parents[2]
REGISTRY_DIR = ROOT / "portal" / "tools" / "registry"


def endpoint_resource_phrase(endpoint: str) -> str:
    clean = endpoint.split("?")[0]
    parts = [p for p in clean.split("/") if p and not p.startswith("{")]
    if not parts:
        return "resource records"
    tail = parts[-2:] if len(parts) >= 2 else parts
    phrase = " ".join(t.replace("-", " ").replace("_", " ") for t in tail)
    return phrase.strip() or "resource records"


def action_from_method(method: str, slug: str) -> str:
    s = slug.lower()
    m = (method or "GET").upper()
    if "search" in s:
        return "Search"
    if "list" in s or (m == "GET" and "get_" not in s):
        return "List"
    if "get" in s or "retrieve" in s:
        return "Get"
    if m == "POST":
        return "Create"
    if m in ("PUT", "PATCH"):
        return "Update"
    if m == "DELETE":
        return "Delete"
    return "Manage"


def summarize_params(input_schema: dict[str, Any]) -> tuple[list[str], list[str]]:
    required = input_schema.get("required") or []
    properties = input_schema.get("properties") or {}
    optional = [k for k in properties.keys() if k not in required]
    return list(required), optional


def build_description(tool: dict[str, Any]) -> str:
    slug = str(tool.get("slug", "tool"))
    name = str(tool.get("name", slug.replace("_", " ")))
    method = str(tool.get("method", "GET"))
    endpoint = str(tool.get("endpoint", ""))
    input_schema = tool.get("input_schema") or {}

    action = action_from_method(method, slug)
    resource = endpoint_resource_phrase(endpoint)
    required, optional = summarize_params(input_schema)

    sentence1 = f"{action} {resource} via {method.upper()}."
    sentence2 = f"Use this tool when the user asks to {name.lower()}."

    if required:
        req = ", ".join(required[:5])
        sentence3 = f"Required parameters: {req}."
    elif optional:
        opt = ", ".join(optional[:5])
        sentence3 = f"Optional parameters include: {opt}."
    else:
        sentence3 = "No input parameters are required."

    return f"{sentence1} {sentence2} {sentence3}"


def refine_file(path: Path, apply: bool) -> tuple[int, int]:
    raw = path.read_text(encoding="utf-8")
    data = yaml.safe_load(raw)
    if not isinstance(data, list):
        return 0, 0

    total = 0
    changed = 0
    for tool in data:
        if not isinstance(tool, dict) or "slug" not in tool:
            continue
        total += 1
        new_desc = build_description(tool)
        old_desc = str(tool.get("description", "")).strip()
        if new_desc != old_desc:
            changed += 1
            tool["description"] = new_desc

    if apply and changed > 0:
        path.write_text(
            yaml.safe_dump(data, sort_keys=False, allow_unicode=True),
            encoding="utf-8",
        )
    return total, changed


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--apply", action="store_true", help="Write changes to files")
    parser.add_argument("--dry-run", action="store_true", help="Show stats only")
    parser.add_argument("--provider", type=str, default="", help="Only process one provider file stem")
    args = parser.parse_args()

    apply = args.apply and not args.dry_run
    files = sorted(REGISTRY_DIR.glob("*.yaml"))
    if args.provider:
        files = [p for p in files if p.stem == args.provider]

    providers = 0
    tools_total = 0
    tools_changed = 0

    for file in files:
        total, changed = refine_file(file, apply=apply)
        if total == 0:
            continue
        providers += 1
        tools_total += total
        tools_changed += changed
        mode = "would change" if not apply else "changed"
        print(f"{file.stem}: {changed}/{total} {mode}")

    print(
        f"\nProcessed providers: {providers}, tools: {tools_total}, "
        f"updated descriptions: {tools_changed}"
    )


if __name__ == "__main__":
    main()

