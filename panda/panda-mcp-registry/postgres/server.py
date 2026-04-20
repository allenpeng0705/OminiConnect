#!/usr/bin/env python3
"""MCP: read-only Postgres introspection for data-analysis agents."""
from __future__ import annotations

import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "common"))
from mcp_stdio import run_stdio, tool_text  # noqa: E402

try:
    import psycopg2
    from psycopg2.extensions import connection as PgConnection
except ImportError:
    psycopg2 = None  # type: ignore
    PgConnection = None  # type: ignore

TOOLS = [
    {
        "name": "pg_list_schemas",
        "description": "List non-system schemas (limit 50).",
        "inputSchema": {"type": "object", "properties": {}},
    },
    {
        "name": "pg_list_tables",
        "description": "List tables in a schema (default public).",
        "inputSchema": {
            "type": "object",
            "properties": {
                "schema": {
                    "type": "string",
                    "description": "Postgres schema name",
                    "default": "public",
                }
            },
        },
    },
    {
        "name": "pg_describe_table",
        "description": "Show column names and types for a table.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "schema": {"type": "string", "default": "public"},
                "table": {"type": "string", "description": "Table name"},
            },
            "required": ["table"],
        },
    },
    {
        "name": "pg_query_readonly",
        "description": "Run a single SELECT (read-only). Blocked if SQL is not a plain SELECT.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "sql": {"type": "string", "description": "SELECT ... statement"},
            },
            "required": ["sql"],
        },
    },
]


def connect() -> "PgConnection":
    if psycopg2 is None:
        raise RuntimeError("install psycopg2-binary: pip install psycopg2-binary")
    url = os.environ.get("DATABASE_URL", "").strip()
    if not url:
        raise RuntimeError("set DATABASE_URL (e.g. postgresql://user:pass@localhost:5432/db)")
    return psycopg2.connect(url)


def assert_readonly_sql(sql: str) -> None:
    s = " ".join(sql.strip().split())
    low = s.lower()
    if not low.startswith("select") and not low.startswith("with"):
        raise ValueError("only SELECT or WITH queries allowed")
    banned = (" insert ", " update ", " delete ", " drop ", " alter ", " truncate ", ";")
    check = f" {low} "
    for b in banned:
        if b in check:
            raise ValueError(f"forbidden token in SQL: {b.strip()}")


def dispatch(name: str, args: dict) -> dict:
    conn = connect()
    try:
        cur = conn.cursor()
        if name == "pg_list_schemas":
            cur.execute(
                "SELECT schema_name FROM information_schema.schemata "
                "WHERE schema_name NOT IN ('pg_catalog','information_schema') "
                "ORDER BY 1 LIMIT 50"
            )
            rows = cur.fetchall()
            return tool_text("\n".join(r[0] for r in rows) or "(no schemas)")
        if name == "pg_list_tables":
            schema = (args.get("schema") or "public").strip()
            cur.execute(
                "SELECT table_name FROM information_schema.tables "
                "WHERE table_schema = %s AND table_type = 'BASE TABLE' ORDER BY 1 LIMIT 200",
                (schema,),
            )
            rows = cur.fetchall()
            return tool_text("\n".join(r[0] for r in rows) or "(no tables)")
        if name == "pg_describe_table":
            table = args["table"].strip()
            schema = (args.get("schema") or "public").strip()
            cur.execute(
                "SELECT column_name, data_type FROM information_schema.columns "
                "WHERE table_schema = %s AND table_name = %s ORDER BY ordinal_position",
                (schema, table),
            )
            rows = cur.fetchall()
            if not rows:
                return tool_text(f"no columns for {schema}.{table}", is_error=True)
            lines = [f"{c}\t{t}" for c, t in rows]
            return tool_text("\n".join(lines))
        if name == "pg_query_readonly":
            sql = args["sql"].strip()
            assert_readonly_sql(sql)
            cur.execute(sql)
            colnames = [d[0] for d in cur.description] if cur.description else []
            rows = cur.fetchmany(500)
            out = "\t".join(colnames) + "\n" if colnames else ""
            out += "\n".join("\t".join(str(x) for x in r) for r in rows)
            if len(rows) == 500:
                out += "\n… truncated at 500 rows"
            return tool_text(out or "(empty)")
        return tool_text(f"unknown tool: {name}", is_error=True)
    finally:
        conn.close()


if __name__ == "__main__":
    run_stdio(TOOLS, dispatch, "panda-registry-postgres")
