# Control plane: Postgres writes outside Panda

Panda’s Postgres path uses **`LISTEN panda_cp_ingress`** (when `control_plane.store.postgres_listen: true`) and **`pg_notify('panda_cp_ingress', '')`** after mutations performed **through Panda’s SQL store**.

If another process changes **`panda_control_plane_ingress_route`** with raw SQL, replicas **do not** receive `NOTIFY` unless you add a database trigger (or rely on **`control_plane.reload_from_store_ms`** polling).

## Optional: statement-level trigger

Run once per database. On older Postgres, replace **`EXECUTE FUNCTION`** with **`EXECUTE PROCEDURE`** (same function name).

```sql
CREATE OR REPLACE FUNCTION panda_cp_ingress_notify() RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify('panda_cp_ingress', '');
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS panda_cp_ingress_notify_tr ON panda_control_plane_ingress_route;

CREATE TRIGGER panda_cp_ingress_notify_tr
AFTER INSERT OR UPDATE OR DELETE ON panda_control_plane_ingress_route
FOR EACH STATEMENT
EXECUTE FUNCTION panda_cp_ingress_notify();
```

**Note:** Panda may already `NOTIFY` on its own writes; listeners will reload twice, which is harmless.

## Alternative

Set **`control_plane.reload_from_store_ms`** to a safe interval so all replicas eventually converge without triggers.
