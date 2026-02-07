pub mod output;

use output::*;
use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

/// Global storage for feature flags.
static FLAGS: Lazy<DashMap<String, FeatureFlag>> = Lazy::new(DashMap::new);
/// Global storage for targeting rules.
static RULES: Lazy<DashMap<String, TargetingRule>> = Lazy::new(DashMap::new);
/// Global storage for rollouts.
static ROLLOUTS: Lazy<DashMap<String, Rollout>> = Lazy::new(DashMap::new);
/// Global storage for environments.
static ENVIRONMENTS: Lazy<DashMap<String, Environment>> = Lazy::new(DashMap::new);
/// Global storage for evaluation counts.
static EVAL_COUNTS: Lazy<DashMap<String, HashMap<String, i32>>> = Lazy::new(DashMap::new);

/// Parse variations from HashMap<String, Value> format.
fn parse_variations(variations: &[HashMap<String, Value>]) -> Vec<FlagVariation> {
    variations
        .iter()
        .map(|v| FlagVariation {
            name: v.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string(),
            value: v.get("value").cloned().unwrap_or(Value::Null),
        })
        .collect()
}

/// Parse conditions from HashMap<String, Value> format.
fn parse_conditions(conditions: &[HashMap<String, Value>]) -> Vec<RuleCondition> {
    conditions
        .iter()
        .map(|c| RuleCondition {
            attribute: c.get("attribute").and_then(|a| a.as_str()).unwrap_or("").to_string(),
            operator: c.get("operator").and_then(|o| o.as_str()).unwrap_or("eq").to_string(),
            value: c.get("value").cloned().unwrap_or(Value::Null),
        })
        .collect()
}

/// Hash a string to get a consistent bucket (0-99).
fn hash_to_bucket(value: &str, seed: Option<&str>) -> u32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    if let Some(s) = seed {
        s.hash(&mut hasher);
    }
    value.hash(&mut hasher);
    (hasher.finish() % 100) as u32
}

/// Create Feature Flag.
pub async fn create_flag(
    key: &str,
    variations: Vec<HashMap<String, Value>>,
    name: &str,
    default_variation: Option<i32>,
    tags: Option<Vec<String>>,
    enabled: Option<bool>,
    description: Option<&str>,
) -> Result<CreateFlagOutput, String> {
    let flag_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    
    let flag = FeatureFlag {
        flag_id: flag_id.clone(),
        key: key.to_string(),
        name: name.to_string(),
        description: description.unwrap_or("").to_string(),
        enabled: enabled.unwrap_or(false),
        archived: false,
        variations: parse_variations(&variations),
        default_variation: default_variation.unwrap_or(0),
        tags: tags.unwrap_or_default(),
        created_at: now,
        updated_at: now,
    };
    
    FLAGS.insert(flag_id.clone(), flag);
    
    Ok(CreateFlagOutput {
        flag_id,
        key: key.to_string(),
        success: true,
    })
}

/// Update Feature Flag.
pub async fn update_flag(
    flag_id: &str,
    name: Option<&str>,
    enabled: Option<bool>,
    description: Option<&str>,
    variations: Option<Vec<HashMap<String, Value>>>,
    tags: Option<Vec<String>>,
) -> Result<UpdateFlagOutput, String> {
    let mut flag = FLAGS
        .get_mut(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    if let Some(n) = name {
        flag.name = n.to_string();
    }
    if let Some(e) = enabled {
        flag.enabled = e;
    }
    if let Some(d) = description {
        flag.description = d.to_string();
    }
    if let Some(v) = variations {
        flag.variations = parse_variations(&v);
    }
    if let Some(t) = tags {
        flag.tags = t;
    }
    flag.updated_at = Utc::now().timestamp();
    
    Ok(UpdateFlagOutput {
        flag_id: flag_id.to_string(),
        success: true,
    })
}

/// Toggle Feature Flag.
pub async fn toggle_flag(
    flag_id: &str,
    enabled: bool,
) -> Result<ToggleFlagOutput, String> {
    let mut flag = FLAGS
        .get_mut(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    flag.enabled = enabled;
    flag.updated_at = Utc::now().timestamp();
    
    Ok(ToggleFlagOutput {
        flag_id: flag_id.to_string(),
        enabled,
        success: true,
    })
}

/// Delete Feature Flag.
pub async fn delete_flag(
    flag_id: &str,
) -> Result<DeleteFlagOutput, String> {
    FLAGS
        .remove(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    // Also delete associated rules and rollouts
    let rules_to_delete: Vec<String> = RULES
        .iter()
        .filter(|r| r.flag_id == flag_id)
        .map(|r| r.rule_id.clone())
        .collect();
    for rule_id in rules_to_delete {
        RULES.remove(&rule_id);
    }
    
    let rollouts_to_delete: Vec<String> = ROLLOUTS
        .iter()
        .filter(|r| r.flag_id == flag_id)
        .map(|r| r.rollout_id.clone())
        .collect();
    for rollout_id in rollouts_to_delete {
        ROLLOUTS.remove(&rollout_id);
    }
    
    Ok(DeleteFlagOutput { success: true })
}

/// Get Feature Flag.
pub async fn get_flag(
    flag_id: &str,
) -> Result<GetFlagOutput, String> {
    let flag = FLAGS
        .get(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    Ok(GetFlagOutput {
        flag: flag.clone(),
    })
}

/// List Feature Flags.
pub async fn list_flags(
    enabled: Option<bool>,
    limit: Option<i32>,
    tags: Option<Vec<String>>,
    offset: Option<i32>,
) -> Result<ListFlagsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;
    
    let flags: Vec<FeatureFlag> = FLAGS
        .iter()
        .filter(|f| {
            // Filter by enabled status
            if let Some(e) = enabled {
                if f.enabled != e {
                    return false;
                }
            }
            // Filter by tags
            if let Some(ref t) = tags {
                if !t.iter().any(|tag| f.tags.contains(tag)) {
                    return false;
                }
            }
            // Exclude archived flags
            !f.archived
        })
        .skip(offset)
        .take(limit)
        .map(|f| f.clone())
        .collect();
    
    let total = flags.len() as i32;
    
    Ok(ListFlagsOutput { flags, total })
}

/// Evaluate Feature Flag.
pub async fn evaluate_flag(
    flag_id: &str,
    user_id: Option<&str>,
    context: Option<HashMap<String, Value>>,
) -> Result<EvaluateFlagOutput, String> {
    let flag = FLAGS
        .get(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    // Track evaluation
    EVAL_COUNTS
        .entry(flag_id.to_string())
        .or_default()
        .entry("total".to_string())
        .and_modify(|c| *c += 1)
        .or_insert(1);
    
    // If flag is disabled, return default variation
    if !flag.enabled || flag.archived {
        let variation = flag.variations
            .get(flag.default_variation as usize)
            .cloned()
            .unwrap_or(FlagVariation {
                name: "off".to_string(),
                value: Value::Bool(false),
            });
        
        return Ok(EvaluateFlagOutput {
            flag_id: flag_id.to_string(),
            enabled: false,
            variation,
            reason: "Flag is disabled".to_string(),
        });
    }
    
    // Check targeting rules
    let rules: Vec<TargetingRule> = RULES
        .iter()
        .filter(|r| r.flag_id == flag_id)
        .map(|r| r.clone())
        .collect();
    
    let mut sorted_rules = rules;
    sorted_rules.sort_by(|a, b| a.priority.cmp(&b.priority));
    
    for rule in sorted_rules {
        if evaluate_rule_conditions(&rule.conditions, &context) {
            let variation = flag.variations
                .get(rule.variation as usize)
                .cloned()
                .unwrap_or(FlagVariation {
                    name: "default".to_string(),
                    value: Value::Bool(true),
                });
            
            // Track variation
            EVAL_COUNTS
                .entry(flag_id.to_string())
                .or_default()
                .entry(variation.name.clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
            
            return Ok(EvaluateFlagOutput {
                flag_id: flag_id.to_string(),
                enabled: true,
                variation,
                reason: format!("Matched rule: {}", rule.name),
            });
        }
    }
    
    // Check percentage rollouts
    if let Some(user) = user_id {
        let rollouts: Vec<Rollout> = ROLLOUTS
            .iter()
            .filter(|r| r.flag_id == flag_id)
            .map(|r| r.clone())
            .collect();
        
        for rollout in rollouts {
            let bucket = hash_to_bucket(user, rollout.seed.as_deref());
            let mut cumulative = 0u32;
            
            for (variation_idx, percentage) in &rollout.percentages {
                cumulative += *percentage as u32;
                if bucket < cumulative {
                    let idx: usize = variation_idx.parse().unwrap_or(0);
                    let variation = flag.variations
                        .get(idx)
                        .cloned()
                        .unwrap_or(FlagVariation {
                            name: "default".to_string(),
                            value: Value::Bool(true),
                        });
                    
                    // Track variation
                    EVAL_COUNTS
                        .entry(flag_id.to_string())
                        .or_default()
                        .entry(variation.name.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                    
                    return Ok(EvaluateFlagOutput {
                        flag_id: flag_id.to_string(),
                        enabled: true,
                        variation,
                        reason: "Percentage rollout".to_string(),
                    });
                }
            }
        }
    }
    
    // Return default variation
    let variation = flag.variations
        .get(flag.default_variation as usize)
        .cloned()
        .unwrap_or(FlagVariation {
            name: "default".to_string(),
            value: Value::Bool(true),
        });
    
    // Track variation
    EVAL_COUNTS
        .entry(flag_id.to_string())
        .or_default()
        .entry(variation.name.clone())
        .and_modify(|c| *c += 1)
        .or_insert(1);
    
    Ok(EvaluateFlagOutput {
        flag_id: flag_id.to_string(),
        enabled: true,
        variation,
        reason: "Default variation".to_string(),
    })
}

/// Evaluate rule conditions against context.
fn evaluate_rule_conditions(conditions: &[RuleCondition], context: &Option<HashMap<String, Value>>) -> bool {
    let ctx = match context {
        Some(c) => c,
        None => return false,
    };
    
    for condition in conditions {
        let ctx_value = match ctx.get(&condition.attribute) {
            Some(v) => v,
            None => return false,
        };
        
        let matches = match condition.operator.as_str() {
            "eq" | "equals" => ctx_value == &condition.value,
            "neq" | "not_equals" => ctx_value != &condition.value,
            "contains" => {
                if let (Some(haystack), Some(needle)) = (ctx_value.as_str(), condition.value.as_str()) {
                    haystack.contains(needle)
                } else {
                    false
                }
            }
            "starts_with" => {
                if let (Some(s), Some(prefix)) = (ctx_value.as_str(), condition.value.as_str()) {
                    s.starts_with(prefix)
                } else {
                    false
                }
            }
            "ends_with" => {
                if let (Some(s), Some(suffix)) = (ctx_value.as_str(), condition.value.as_str()) {
                    s.ends_with(suffix)
                } else {
                    false
                }
            }
            "gt" | "greater_than" => {
                if let (Some(a), Some(b)) = (ctx_value.as_f64(), condition.value.as_f64()) {
                    a > b
                } else {
                    false
                }
            }
            "gte" | "greater_than_or_equals" => {
                if let (Some(a), Some(b)) = (ctx_value.as_f64(), condition.value.as_f64()) {
                    a >= b
                } else {
                    false
                }
            }
            "lt" | "less_than" => {
                if let (Some(a), Some(b)) = (ctx_value.as_f64(), condition.value.as_f64()) {
                    a < b
                } else {
                    false
                }
            }
            "lte" | "less_than_or_equals" => {
                if let (Some(a), Some(b)) = (ctx_value.as_f64(), condition.value.as_f64()) {
                    a <= b
                } else {
                    false
                }
            }
            "in" => {
                if let Some(arr) = condition.value.as_array() {
                    arr.contains(ctx_value)
                } else {
                    false
                }
            }
            "not_in" => {
                if let Some(arr) = condition.value.as_array() {
                    !arr.contains(ctx_value)
                } else {
                    true
                }
            }
            _ => false,
        };
        
        if !matches {
            return false;
        }
    }
    
    true
}

/// Create Targeting Rule.
pub async fn create_targeting_rule(
    variation: i32,
    name: &str,
    conditions: Vec<HashMap<String, Value>>,
    flag_id: &str,
    priority: Option<i32>,
) -> Result<CreateTargetingRuleOutput, String> {
    // Verify flag exists
    if !FLAGS.contains_key(flag_id) {
        return Err(format!("Flag not found: {}", flag_id));
    }
    
    let rule_id = Uuid::new_v4().to_string();
    let rule = TargetingRule {
        rule_id: rule_id.clone(),
        flag_id: flag_id.to_string(),
        name: name.to_string(),
        priority: priority.unwrap_or(0),
        variation,
        conditions: parse_conditions(&conditions),
    };
    
    RULES.insert(rule_id.clone(), rule);
    
    Ok(CreateTargetingRuleOutput {
        flag_id: flag_id.to_string(),
        rule_id,
        success: true,
    })
}

/// Update Targeting Rule.
pub async fn update_targeting_rule(
    rule_id: &str,
    name: Option<&str>,
    variation: Option<i32>,
    conditions: Option<Vec<HashMap<String, Value>>>,
    priority: Option<i32>,
) -> Result<UpdateTargetingRuleOutput, String> {
    let mut rule = RULES
        .get_mut(rule_id)
        .ok_or_else(|| format!("Rule not found: {}", rule_id))?;
    
    if let Some(n) = name {
        rule.name = n.to_string();
    }
    if let Some(v) = variation {
        rule.variation = v;
    }
    if let Some(c) = conditions {
        rule.conditions = parse_conditions(&c);
    }
    if let Some(p) = priority {
        rule.priority = p;
    }
    
    Ok(UpdateTargetingRuleOutput {
        rule_id: rule_id.to_string(),
        success: true,
    })
}

/// Delete Targeting Rule.
pub async fn delete_targeting_rule(
    rule_id: &str,
) -> Result<DeleteTargetingRuleOutput, String> {
    RULES
        .remove(rule_id)
        .ok_or_else(|| format!("Rule not found: {}", rule_id))?;
    
    Ok(DeleteTargetingRuleOutput { success: true })
}

/// Create Percentage Rollout.
pub async fn create_rollout(
    flag_id: &str,
    percentages: HashMap<String, Value>,
    seed: Option<&str>,
) -> Result<CreateRolloutOutput, String> {
    // Verify flag exists
    if !FLAGS.contains_key(flag_id) {
        return Err(format!("Flag not found: {}", flag_id));
    }
    
    let rollout_id = Uuid::new_v4().to_string();
    
    // Convert percentages to i32
    let percentages_i32: HashMap<String, i32> = percentages
        .iter()
        .filter_map(|(k, v)| {
            v.as_i64().map(|n| (k.clone(), n as i32))
        })
        .collect();
    
    let rollout = Rollout {
        rollout_id: rollout_id.clone(),
        flag_id: flag_id.to_string(),
        percentages: percentages_i32,
        seed: seed.map(String::from),
    };
    
    ROLLOUTS.insert(rollout_id.clone(), rollout);
    
    Ok(CreateRolloutOutput {
        flag_id: flag_id.to_string(),
        rollout_id,
        success: true,
    })
}

/// Update Percentage Rollout.
pub async fn update_rollout(
    rollout_id: &str,
    percentages: HashMap<String, Value>,
) -> Result<UpdateRolloutOutput, String> {
    let mut rollout = ROLLOUTS
        .get_mut(rollout_id)
        .ok_or_else(|| format!("Rollout not found: {}", rollout_id))?;
    
    rollout.percentages = percentages
        .iter()
        .filter_map(|(k, v)| {
            v.as_i64().map(|n| (k.clone(), n as i32))
        })
        .collect();
    
    Ok(UpdateRolloutOutput {
        rollout_id: rollout_id.to_string(),
        success: true,
    })
}

/// Get Flag Evaluation Count.
pub async fn get_evaluation_count(
    flag_id: &str,
    _start_date: Option<i32>,
    _end_date: Option<i32>,
) -> Result<GetEvaluationCountOutput, String> {
    let counts = EVAL_COUNTS
        .get(flag_id)
        .map(|c| c.clone())
        .unwrap_or_default();
    
    let total = counts.get("total").cloned().unwrap_or(0);
    let mut variation_counts = counts.clone();
    variation_counts.remove("total");
    
    Ok(GetEvaluationCountOutput {
        flag_id: flag_id.to_string(),
        total_evaluations: total,
        variation_counts,
    })
}

/// Create Flag Environment.
pub async fn create_environment(
    name: &str,
    key: &str,
    description: Option<&str>,
) -> Result<CreateEnvironmentOutput, String> {
    let environment_id = Uuid::new_v4().to_string();
    
    let env = Environment {
        environment_id: environment_id.clone(),
        key: key.to_string(),
        name: name.to_string(),
        description: description.unwrap_or("").to_string(),
    };
    
    ENVIRONMENTS.insert(environment_id.clone(), env);
    
    Ok(CreateEnvironmentOutput {
        environment_id,
        key: key.to_string(),
        success: true,
    })
}

/// Clone Flag To Environment.
pub async fn clone_flag(
    source_environment: &str,
    flag_id: &str,
    target_environment: &str,
) -> Result<CloneFlagOutput, String> {
    // Verify environments exist
    if !ENVIRONMENTS.iter().any(|e| e.key == source_environment) {
        return Err(format!("Source environment not found: {}", source_environment));
    }
    if !ENVIRONMENTS.iter().any(|e| e.key == target_environment) {
        return Err(format!("Target environment not found: {}", target_environment));
    }
    
    // Get the flag
    let flag = FLAGS
        .get(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    // Create a new flag for the target environment
    let new_flag_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    
    let new_flag = FeatureFlag {
        flag_id: new_flag_id.clone(),
        key: format!("{}_{}", flag.key, target_environment),
        name: flag.name.clone(),
        description: flag.description.clone(),
        enabled: flag.enabled,
        archived: false,
        variations: flag.variations.clone(),
        default_variation: flag.default_variation,
        tags: flag.tags.clone(),
        created_at: now,
        updated_at: now,
    };
    
    FLAGS.insert(new_flag_id.clone(), new_flag);
    
    Ok(CloneFlagOutput {
        flag_id: new_flag_id,
        success: true,
    })
}

/// Archive Feature Flag.
pub async fn archive_flag(
    flag_id: &str,
) -> Result<ArchiveFlagOutput, String> {
    let mut flag = FLAGS
        .get_mut(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    flag.archived = true;
    flag.enabled = false;
    flag.updated_at = Utc::now().timestamp();
    
    Ok(ArchiveFlagOutput {
        flag_id: flag_id.to_string(),
        archived: true,
        success: true,
    })
}

/// Restore Feature Flag.
pub async fn restore_flag(
    flag_id: &str,
) -> Result<RestoreFlagOutput, String> {
    let mut flag = FLAGS
        .get_mut(flag_id)
        .ok_or_else(|| format!("Flag not found: {}", flag_id))?;
    
    flag.archived = false;
    flag.updated_at = Utc::now().timestamp();
    
    Ok(RestoreFlagOutput {
        flag_id: flag_id.to_string(),
        archived: false,
        success: true,
    })
}

#[cfg(test)]
mod tests;
