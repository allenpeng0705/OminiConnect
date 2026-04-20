//! Optional hierarchical prompt-token budgets (Enterprise). Requires Redis.

use std::sync::Arc;

use panda_config::{BudgetHierarchyConfig, BudgetHierarchyDepartmentLimit};
const HIER_LUA: &str = r#"
local n = tonumber(ARGV[1])
local org_lim = tonumber(ARGV[2])
local dept_lim = tonumber(ARGV[3])

local function would_exceed(key, limit)
  if limit <= 0 then return false end
  local cur = tonumber(redis.call('GET', key) or 0)
  return (cur + n) > limit
end

if org_lim > 0 and would_exceed(KEYS[1], org_lim) then
  return 0
end
if dept_lim > 0 and would_exceed(KEYS[2], dept_lim) then
  return 0
end

if org_lim > 0 then
  redis.call('INCRBY', KEYS[1], n)
  redis.call('EXPIRE', KEYS[1], 120)
end
if dept_lim > 0 then
  redis.call('INCRBY', KEYS[2], n)
  redis.call('EXPIRE', KEYS[2], 120)
end
return 1
"#;

fn unix_minute() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() / 60)
        .unwrap_or(0)
}

fn dept_limit(departments: &[BudgetHierarchyDepartmentLimit], name: &str) -> u64 {
    departments
        .iter()
        .find(|d| d.department == name)
        .map(|d| d.prompt_tokens_per_minute)
        .unwrap_or(0)
}

pub struct BudgetHierarchyCounters {
    mgr: redis::aio::ConnectionManager,
    cfg: Arc<BudgetHierarchyConfig>,
}

/// Prompt-token counters for the current rolling minute in Redis (org / department), for ops visibility.
#[derive(Debug, Clone)]
pub struct HierarchyWindowSnapshot {
    pub rolling_minute: u64,
    pub org_prompt_tokens_used: u64,
    pub org_prompt_tokens_limit: u64,
    pub dept_prompt_tokens_used: u64,
    pub dept_prompt_tokens_limit: u64,
    pub redis_read_error: bool,
}

impl BudgetHierarchyCounters {
    pub async fn connect(cfg: Arc<BudgetHierarchyConfig>, redis_url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let mgr = redis::aio::ConnectionManager::new(client).await?;
        Ok(Self { mgr, cfg })
    }

    /// Best-effort read of `GET` counters for the same keys the Lua script updates. `None` when no hierarchy limit applies to this identity shape.
    pub async fn window_usage_snapshot(
        &self,
        department: Option<&str>,
    ) -> Option<HierarchyWindowSnapshot> {
        let minute = unix_minute();
        let org_lim = self.cfg.org_prompt_tokens_per_minute.unwrap_or(0);
        let dept_lim = match department {
            Some(d) if !d.trim().is_empty() => {
                let lim = dept_limit(&self.cfg.departments, d.trim());
                if lim == 0 {
                    // Unknown department: org-only snapshot when an org cap exists (matches try_reserve fail-closed without org).
                    if org_lim == 0 {
                        return None;
                    }
                    0
                } else {
                    lim
                }
            }
            _ => {
                if !self.cfg.departments.is_empty() && org_lim == 0 {
                    return None;
                }
                0
            }
        };
        if org_lim == 0 && dept_lim == 0 {
            return None;
        }
        let org_key = format!("panda:budget:hier:v1:org:{minute}");
        let dept_key = department
            .filter(|d| !d.trim().is_empty())
            .map(|d| format!("panda:budget:hier:v1:dept:{}:{minute}", d.trim()));
        let mut conn = self.mgr.clone();
        let mut org_used = 0u64;
        let mut dept_used = 0u64;
        let mut redis_read_error = false;
        if org_lim > 0 {
            match redis::cmd("GET")
                .arg(&org_key)
                .query_async::<Option<String>>(&mut conn)
                .await
            {
                Ok(Some(s)) => org_used = s.parse::<u64>().unwrap_or(0),
                Ok(None) => {}
                Err(e) => {
                    eprintln!("panda budget_hierarchy: redis GET org key failed: {e}");
                    redis_read_error = true;
                }
            }
        }
        if dept_lim > 0 {
            if let Some(ref dk) = dept_key {
                match redis::cmd("GET")
                    .arg(dk.as_str())
                    .query_async::<Option<String>>(&mut conn)
                    .await
                {
                    Ok(Some(s)) => dept_used = s.parse::<u64>().unwrap_or(0),
                    Ok(None) => {}
                    Err(e) => {
                        eprintln!("panda budget_hierarchy: redis GET dept key failed: {e}");
                        redis_read_error = true;
                    }
                }
            }
        }
        Some(HierarchyWindowSnapshot {
            rolling_minute: minute,
            org_prompt_tokens_used: org_used,
            org_prompt_tokens_limit: org_lim,
            dept_prompt_tokens_used: dept_used,
            dept_prompt_tokens_limit: dept_lim,
            redis_read_error,
        })
    }

    /// Returns false when a configured limit would be exceeded (HTTP 429).
    pub async fn try_reserve(&self, department: Option<&str>, n: u64) -> bool {
        if n == 0 {
            return true;
        }
        let minute = unix_minute();
        let org_lim = self.cfg.org_prompt_tokens_per_minute.unwrap_or(0);
        let dept_lim = match department {
            Some(d) if !d.trim().is_empty() => {
                let lim = dept_limit(&self.cfg.departments, d.trim());
                if lim == 0 {
                    return false;
                }
                lim
            }
            _ => {
                if !self.cfg.departments.is_empty() && org_lim == 0 {
                    return false;
                }
                0
            }
        };
        if org_lim == 0 && dept_lim == 0 {
            return true;
        }
        let org_key = format!("panda:budget:hier:v1:org:{minute}");
        let dept_key = department
            .filter(|d| !d.trim().is_empty())
            .map(|d| format!("panda:budget:hier:v1:dept:{}:{minute}", d.trim()))
            .unwrap_or_else(|| org_key.clone());

        let (k1_owned, k2_owned) = if org_lim > 0 && dept_lim > 0 {
            (org_key, dept_key)
        } else if org_lim > 0 {
            let o2 = org_key.clone();
            (org_key, o2)
        } else {
            let d2 = dept_key.clone();
            (dept_key, d2)
        };
        let k1 = k1_owned.as_str();
        let k2 = k2_owned.as_str();

        let mut conn = self.mgr.clone();
        let r: i64 = match redis::cmd("EVAL")
            .arg(HIER_LUA)
            .arg(2usize)
            .arg(k1)
            .arg(k2)
            .arg(n as i64)
            .arg(org_lim as i64)
            .arg(dept_lim as i64)
            .query_async(&mut conn)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("panda budget_hierarchy: redis EVAL failed: {e}");
                return self.cfg.fail_open;
            }
        };
        r == 1
    }
}
