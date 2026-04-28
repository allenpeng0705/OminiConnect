#!/usr/bin/env python3
"""Generate realistic query files from tool registry."""

import argparse
import re
from pathlib import Path
import yaml

REGISTRY_DIR = Path("tools/registry")
QUERIES_DIR = Path("tests/llm_tool_selection/queries")

ACTION_ALIASES = {
    "get": "read",
    "retrieve": "read",
    "read": "read",
    "list": "list",
    "create": "create",
    "update": "update",
    "patch": "update",
    "delete": "delete",
    "search": "search",
    "send": "create",
    "assign": "update",
}
ACTION_ORDER = ["list", "read", "create", "update", "delete", "search"]
STOPWORDS = {
    "a", "an", "the", "of", "for", "to", "my", "me", "all", "new",
    "create", "get", "list", "update", "delete", "search", "find", "retrieve",
}


def singularize(noun):
    if noun.endswith("ies") and len(noun) > 4:
        return noun[:-3] + "y"
    if noun.endswith("ses") and len(noun) > 4:
        return noun[:-2]
    if noun.endswith("s") and not noun.endswith("ss") and len(noun) > 3:
        return noun[:-1]
    return noun


def normalize_query(q):
    q = re.sub(r"\s+", " ", q).strip()
    if not q:
        return q
    q = q[0].upper() + q[1:]
    if not q.endswith((".", "?", "!")):
        q += "."
    return q


def extract_action(slug):
    parts = slug.lower().split("_")
    if len(parts) > 1 and parts[1] in ACTION_ALIASES:
        return ACTION_ALIASES[parts[1]]
    for action in ACTION_ORDER:
        if action in slug.lower():
            return action
    return "list"


def infer_resource_from_tool(tool):
    slug = tool.get("slug", "")
    name = (tool.get("name") or "").strip().lower()
    provider = (tool.get("provider") or "").strip().lower()
    parts = [p for p in slug.split("_") if p]
    if parts and parts[0] == provider:
        parts = parts[1:]
    if parts and parts[0] in ACTION_ALIASES:
        parts = parts[1:]
    candidate = " ".join(parts).strip() or name or "records"
    tokens = [t for t in re.split(r"[\s\-_]+", candidate) if t and t not in STOPWORDS]
    return " ".join(tokens) if tokens else "records"


def infer_sample_value(param_name):
    p = param_name.lower()
    if p.endswith("id") or p.endswith("_id"):
        return f"{p}_123"
    if "email" in p:
        return "user@example.com"
    if "name" in p:
        return "sample-name"
    if "title" in p:
        return "Sample Title"
    if "query" in p or "search" in p:
        return "status:open"
    if "date" in p:
        return "2026-01-01"
    return f"{p}_value"


def build_templates(action, resource, required_params):
    singular = singularize(resource)
    req_hint = ""
    if required_params:
        pairs = [f"{p}={infer_sample_value(p)}" for p in required_params[:2]]
        req_hint = " with " + " and ".join(pairs)

    templates = {
        "list": [
            f"List my {resource}",
            f"Show all {resource}",
            f"What {resource} do I have",
        ],
        "create": [
            f"Create a new {singular}{req_hint}",
            f"Add a {singular}{req_hint}",
            f"Make a {singular}{req_hint}",
        ],
        "read": [
            f"Get details for {singular}{req_hint}",
            f"Show me this {singular}{req_hint}",
            f"Retrieve {singular} information{req_hint}",
        ],
        "update": [
            f"Update this {singular}{req_hint}",
            f"Modify the {singular}{req_hint}",
            f"Edit {singular}{req_hint}",
        ],
        "delete": [
            f"Delete this {singular}{req_hint}",
            f"Remove the {singular}{req_hint}",
            f"Archive this {singular}{req_hint}",
        ],
        "search": [
            f"Search {resource}{req_hint}",
            f"Find {resource} matching criteria{req_hint}",
            f"Look up {resource}{req_hint}",
        ],
    }
    return [normalize_query(q) for q in templates.get(action, templates["list"])]


def build_queries_for_tool(tool, action):
    examples = [
        normalize_query(q)
        for q in (tool.get("example_queries") or [])
        if isinstance(q, str) and q.strip()
    ]
    resource = infer_resource_from_tool(tool)
    required = ((tool.get("input_schema") or {}).get("required") or [])
    generated = build_templates(action, resource, required)

    if examples:
        base = examples[0]
        vars_ = []
        for q in examples[1:3] + generated:
            if q and q != base and q not in vars_:
                vars_.append(q)
            if len(vars_) >= 2:
                break
        return base, vars_[:2]

    return generated[0], generated[1:3]


def generate_query_file(provider, tools, output_path):
    queries = []
    tools_by_action = {}
    for tool in tools:
        slug = tool.get("slug", "")
        if not slug:
            continue
        action = extract_action(slug)
        tools_by_action.setdefault(action, []).append(tool)

    for action in ACTION_ORDER:
        if action not in tools_by_action:
            continue
        for tool in tools_by_action[action][:4]:
            slug = tool.get("slug", "")
            base_query, variations = build_queries_for_tool(tool, action)
            queries.append({
                "query": base_query,
                "expected_tool": slug,
                "category": action,
                "variations": variations,
                "description_notes": None,
            })

    if not queries:
        return False

    content = {"provider": provider, "queries": queries}
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(f"# {provider.title()} Tool Selection Queries\n")
        f.write("# Auto-generated from registry (realistic mode)\n\n")
        yaml.dump(content, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
    return True


def main():
    parser = argparse.ArgumentParser(description="Generate realistic tool-selection query files.")
    parser.add_argument("--provider", default="", help="Only generate for one provider")
    parser.add_argument("--overwrite", action="store_true", help="Overwrite existing query files")
    args = parser.parse_args()

    registry_path = Path(REGISTRY_DIR)
    queries_path = Path(QUERIES_DIR)
    queries_path.mkdir(parents=True, exist_ok=True)

    generated = 0
    existing = {f.stem for f in queries_path.glob("*.yaml")}

    for yaml_file in sorted(registry_path.glob("*.yaml")):
        provider = yaml_file.stem
        if args.provider and provider != args.provider:
            continue
        if provider in existing and not args.overwrite:
            continue

        try:
            with open(yaml_file, "r", encoding="utf-8") as f:
                data = yaml.safe_load(f)
            if not data or not isinstance(data, list):
                continue
            has_tools = any("slug" in item for item in data if isinstance(item, dict))
            if not has_tools:
                continue
            output_path = queries_path / f"{provider}.yaml"
            if generate_query_file(provider, data, output_path):
                generated += 1
                print(f"Generated: {provider}.yaml ({len(data)} tools)")
        except Exception as e:
            print(f"Error processing {yaml_file.name}: {e}")

    print(f"\nGenerated {generated} query files")
    print(f"Overwrite mode: {args.overwrite}")


if __name__ == "__main__":
    main()