#!/usr/bin/env python3
"""Generate query files from tool registry."""

import os
import yaml
from pathlib import Path

REGISTRY_DIR = Path("tools/registry")
QUERIES_DIR = Path("tests/llm_tool_selection/queries")

ACTION_PATTERNS = {
    "list": {"keywords": ["list", "get all", "show", "find", "search"], "expected_prefix": "list_"},
    "create": {"keywords": ["create", "add", "new", "make", "insert"], "expected_prefix": "create_"},
    "read": {"keywords": ["get", "retrieve", "show", "view", "lookup", "find one"], "expected_prefix": "get_"},
    "update": {"keywords": ["update", "edit", "modify", "change", "set", "patch"], "expected_prefix": "update_"},
    "delete": {"keywords": ["delete", "remove", "destroy", "erase", "drop"], "expected_prefix": "delete_"},
    "search": {"keywords": ["search", "query", "find", "lookup", "filter"], "expected_prefix": "search_"},
}

def extract_action(slug):
    """Guess action from slug."""
    for action, pattern in ACTION_PATTERNS.items():
        if any(p in slug.lower() for p in pattern["keywords"]):
            return action
    if "list" in slug.lower():
        return "list"
    elif "create" in slug.lower():
        return "create"
    elif "get" in slug.lower() or "retrieve" in slug.lower():
        return "read"
    elif "update" in slug.lower() or "edit" in slug.lower():
        return "update"
    elif "delete" in slug.lower():
        return "delete"
    elif "search" in slug.lower():
        return "search"
    return "list"

def get_query_for_action(action, name):
    """Generate a query for an action."""
    name_lower = name.lower()
    queries = {
        "list": [f"list all {name_lower}s", f"show my {name_lower}s", f"find {name_lower}s"],
        "create": [f"create a new {name_lower}", f"add a {name_lower}", f"make a new {name_lower}"],
        "read": [f"get {name_lower} details", f"show me the {name_lower}", f"what is this {name_lower} about"],
        "update": [f"update a {name_lower}", f"edit the {name_lower}", f"modify {name_lower}"],
        "delete": [f"delete a {name_lower}", f"remove a {name_lower}", f"erase a {name_lower}"],
        "search": [f"search for {name_lower}", f"find {name_lower}s", f"lookup {name_lower}"],
    }
    return queries.get(action, queries["list"])

def generate_query_file(provider, tools, output_path):
    """Generate a query YAML file for a provider."""
    queries = []

    # Group tools by action type
    tools_by_action = {}
    for tool in tools:
        slug = tool.get("slug", "")
        name = tool.get("name", slug.replace("_", " "))
        action = extract_action(slug)

        if action not in tools_by_action:
            tools_by_action[action] = []
        tools_by_action[action].append((slug, name))

    # Generate queries for each action, prioritizing list/read/create
    action_order = ["list", "read", "create", "update", "delete", "search"]

    for action in action_order:
        if action not in tools_by_action:
            continue
        tools_for_action = tools_by_action[action]

        for i, (slug, name) in enumerate(tools_for_action[:3]):  # Max 3 per action
            base_query = get_query_for_action(action, name)[0]
            variations = get_query_for_action(action, name)[1:]

            queries.append({
                "query": base_query,
                "expected_tool": slug,
                "category": action,
                "variations": variations,
                "description_notes": None,
            })

    if not queries:
        return False

    content = {
        "provider": provider,
        "queries": queries,
    }

    with open(output_path, "w") as f:
        f.write(f"# {provider.title()} Tool Selection Queries\n")
        f.write(f"# Auto-generated from registry\n\n")
        yaml.dump(content, f, default_flow_style=False, sort_keys=False, allow_unicode=True)

    return True

def main():
    registry_path = Path(REGISTRY_DIR)
    queries_path = Path(QUERIES_DIR)
    queries_path.mkdir(parents=True, exist_ok=True)

    generated = 0
    existing = set()

    # Load existing queries
    for f in queries_path.glob("*.yaml"):
        existing.add(f.stem)

    for yaml_file in sorted(registry_path.glob("*.yaml")):
        provider = yaml_file.stem

        if provider in existing:
            continue

        try:
            with open(yaml_file, "r") as f:
                data = yaml.safe_load(f)

            if not data or not isinstance(data, list):
                continue

            tools = data

            # Check if file has tools with slugs
            has_tools = any("slug" in item for item in tools if isinstance(item, dict))
            if not has_tools:
                continue

            output_path = queries_path / f"{provider}.yaml"
            if generate_query_file(provider, tools, output_path):
                generated += 1
                print(f"Generated: {provider}.yaml ({len(tools)} tools)")
        except Exception as e:
            print(f"Error processing {yaml_file.name}: {e}")

    print(f"\nGenerated {generated} new query files")
    print(f"Total query files now: {len(existing) + generated}")

if __name__ == "__main__":
    main()