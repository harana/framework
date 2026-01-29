pub mod output;

use output::*;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Storage for policies.
static POLICIES: Lazy<DashMap<String, StoredPolicy>> = Lazy::new(DashMap::new);
/// Role to policies mapping.
static ROLE_POLICIES: Lazy<DashMap<String, HashSet<String>>> = Lazy::new(DashMap::new);
/// User to policies mapping.
static USER_POLICIES: Lazy<DashMap<String, HashSet<String>>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone)]
struct StoredPolicy {
    policy_id: String,
    name: String,
    description: Option<String>,
    actions: Vec<String>,
    resources: Vec<String>,
    effect: String,
    conditions: HashMap<String, Value>,
}

/// Check if a pattern matches a value (supports wildcards).
fn matches_pattern(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    // Convert pattern to regex
    let regex_pattern = pattern
        .replace(".", "\\.")
        .replace("*", ".*")
        .replace("?", ".");
    
    let regex_pattern = format!("^{}$", regex_pattern);
    
    match Regex::new(&regex_pattern) {
        Ok(re) => re.is_match(value),
        Err(_) => pattern == value,
    }
}

/// Evaluate conditions against context.
fn evaluate_conditions(conditions: &HashMap<String, Value>, context: &HashMap<String, Value>) -> (bool, HashMap<String, Value>) {
    let mut evaluated = HashMap::new();
    
    if conditions.is_empty() {
        return (true, evaluated);
    }
    
    for (key, condition) in conditions {
        let context_value = context.get(key);
        
        let result = match condition {
            Value::Object(obj) => {
                // Handle operators like {"equals": "value"}, {"in": ["a", "b"]}
                if let Some(eq_val) = obj.get("equals") {
                    context_value.map_or(false, |cv| cv == eq_val)
                } else if let Some(Value::Array(arr)) = obj.get("in") {
                    context_value.map_or(false, |cv| arr.contains(cv))
                } else if let Some(Value::Array(arr)) = obj.get("not_in") {
                    context_value.map_or(true, |cv| !arr.contains(cv))
                } else if let Some(gt_val) = obj.get("greater_than") {
                    match (context_value, gt_val) {
                        (Some(Value::Number(cv)), Value::Number(gt)) => {
                            cv.as_f64().unwrap_or(0.0) > gt.as_f64().unwrap_or(0.0)
                        }
                        _ => false,
                    }
                } else if let Some(lt_val) = obj.get("less_than") {
                    match (context_value, lt_val) {
                        (Some(Value::Number(cv)), Value::Number(lt)) => {
                            cv.as_f64().unwrap_or(0.0) < lt.as_f64().unwrap_or(0.0)
                        }
                        _ => false,
                    }
                } else {
                    // Direct comparison
                    context_value.map_or(false, |cv| cv == condition)
                }
            }
            _ => {
                // Direct value comparison
                context_value.map_or(false, |cv| cv == condition)
            }
        };
        
        evaluated.insert(key.clone(), json!(result));
        
        if !result {
            return (false, evaluated);
        }
    }
    
    (true, evaluated)
}

/// Attach Policy To Role.
pub async fn attach_to_role(
    policy_id: &str,
    role_id: &str,
) -> Result<AttachToRoleOutput, String> {
    // Verify policy exists
    if !POLICIES.contains_key(policy_id) {
        return Err(format!("Policy not found: {}", policy_id));
    }
    
    ROLE_POLICIES
        .entry(role_id.to_string())
        .or_insert_with(HashSet::new)
        .insert(policy_id.to_string());
    
    Ok(AttachToRoleOutput { success: true })
}

/// Attach Policy To User.
pub async fn attach_to_user(
    policy_id: &str,
    user_id: &str,
) -> Result<AttachToUserOutput, String> {
    // Verify policy exists
    if !POLICIES.contains_key(policy_id) {
        return Err(format!("Policy not found: {}", policy_id));
    }
    
    USER_POLICIES
        .entry(user_id.to_string())
        .or_insert_with(HashSet::new)
        .insert(policy_id.to_string());
    
    Ok(AttachToUserOutput { success: true })
}

/// Create Policy.
pub async fn create(
    actions: Vec<String>,
    effect: &str,
    resources: Vec<String>,
    name: &str,
    conditions: HashMap<String, Value>,
    description: Option<&str>,
) -> Result<CreateOutput, String> {
    // Validate effect
    let effect_lower = effect.to_lowercase();
    if effect_lower != "allow" && effect_lower != "deny" {
        return Err("Effect must be 'allow' or 'deny'".to_string());
    }
    
    let policy_id = Uuid::new_v4().to_string();
    
    let policy = StoredPolicy {
        policy_id: policy_id.clone(),
        name: name.to_string(),
        description: description.map(String::from),
        actions,
        resources,
        effect: effect_lower,
        conditions,
    };
    
    POLICIES.insert(policy_id.clone(), policy);
    
    Ok(CreateOutput {
        success: true,
        policy_id,
    })
}

/// Delete Policy.
pub async fn delete(
    policy_id: &str,
) -> Result<DeleteOutput, String> {
    POLICIES
        .remove(policy_id)
        .ok_or_else(|| format!("Policy not found: {}", policy_id))?;
    
    // Remove from all role attachments
    for mut entry in ROLE_POLICIES.iter_mut() {
        entry.remove(policy_id);
    }
    
    // Remove from all user attachments
    for mut entry in USER_POLICIES.iter_mut() {
        entry.remove(policy_id);
    }
    
    Ok(DeleteOutput { success: true })
}

/// Detach Policy From Role.
pub async fn detach_from_role(
    policy_id: &str,
    role_id: &str,
) -> Result<DetachFromRoleOutput, String> {
    let mut policies = ROLE_POLICIES
        .get_mut(role_id)
        .ok_or_else(|| format!("Role has no policies: {}", role_id))?;
    
    if !policies.remove(policy_id) {
        return Err(format!("Policy {} not attached to role {}", policy_id, role_id));
    }
    
    Ok(DetachFromRoleOutput { success: true })
}

/// Detach Policy From User.
pub async fn detach_from_user(
    policy_id: &str,
    user_id: &str,
) -> Result<DetachFromUserOutput, String> {
    let mut policies = USER_POLICIES
        .get_mut(user_id)
        .ok_or_else(|| format!("User has no policies: {}", user_id))?;
    
    if !policies.remove(policy_id) {
        return Err(format!("Policy {} not attached to user {}", policy_id, user_id));
    }
    
    Ok(DetachFromUserOutput { success: true })
}

/// Evaluate Policy.
pub async fn evaluate(
    action: &str,
    resource: &str,
    policy_id: &str,
    context: Option<HashMap<String, Value>>,
) -> Result<EvaluateOutput, String> {
    let policy = POLICIES
        .get(policy_id)
        .ok_or_else(|| format!("Policy not found: {}", policy_id))?;
    
    let context = context.unwrap_or_default();
    
    // Check if action matches
    let action_matches = policy.actions.iter().any(|a| matches_pattern(a, action));
    
    if !action_matches {
        return Ok(EvaluateOutput {
            allowed: false,
            reason: "Action not covered by policy".to_string(),
            evaluated_conditions: HashMap::new(),
        });
    }
    
    // Check if resource matches
    let resource_matches = policy.resources.iter().any(|r| matches_pattern(r, resource));
    
    if !resource_matches {
        return Ok(EvaluateOutput {
            allowed: false,
            reason: "Resource not covered by policy".to_string(),
            evaluated_conditions: HashMap::new(),
        });
    }
    
    // Evaluate conditions
    let (conditions_met, evaluated_conditions) = evaluate_conditions(&policy.conditions, &context);
    
    if !conditions_met {
        return Ok(EvaluateOutput {
            allowed: false,
            reason: "Conditions not met".to_string(),
            evaluated_conditions,
        });
    }
    
    // Determine result based on effect
    let allowed = policy.effect == "allow";
    let reason = if allowed {
        "Policy allows this action".to_string()
    } else {
        "Policy explicitly denies this action".to_string()
    };
    
    Ok(EvaluateOutput {
        allowed,
        reason,
        evaluated_conditions,
    })
}

/// Get Policy Details.
pub async fn get(
    policy_id: &str,
) -> Result<GetOutput, String> {
    let policy = POLICIES
        .get(policy_id)
        .ok_or_else(|| format!("Policy not found: {}", policy_id))?;
    
    Ok(GetOutput {
        name: policy.name.clone(),
        description: policy.description.clone().unwrap_or_default(),
        actions: policy.actions.clone(),
        resources: policy.resources.clone(),
        effect: policy.effect.clone(),
        conditions: policy.conditions.clone(),
    })
}

/// List Policies.
pub async fn list_policies(
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<ListPoliciesOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;
    
    let all_policies: Vec<HashMap<String, Value>> = POLICIES
        .iter()
        .map(|p| {
            let mut map = HashMap::new();
            map.insert("policy_id".to_string(), json!(p.policy_id.clone()));
            map.insert("name".to_string(), json!(p.name.clone()));
            map.insert("description".to_string(), json!(p.description.clone()));
            map.insert("effect".to_string(), json!(p.effect.clone()));
            map.insert("actions".to_string(), json!(p.actions.clone()));
            map.insert("resources".to_string(), json!(p.resources.clone()));
            map
        })
        .collect();
    
    let total = all_policies.len() as i32;
    let policies: Vec<_> = all_policies.into_iter().skip(offset).take(limit).collect();
    
    Ok(ListPoliciesOutput { policies, total })
}

/// Update Policy.
pub async fn update(
    policy_id: &str,
    name: Option<&str>,
    conditions: Option<HashMap<String, Value>>,
    description: Option<&str>,
    resources: Option<Vec<String>>,
    actions: Option<Vec<String>>,
    effect: Option<&str>,
) -> Result<UpdateOutput, String> {
    let mut policy = POLICIES
        .get_mut(policy_id)
        .ok_or_else(|| format!("Policy not found: {}", policy_id))?;
    
    if let Some(n) = name {
        policy.name = n.to_string();
    }
    
    if let Some(d) = description {
        policy.description = Some(d.to_string());
    }
    
    if let Some(a) = actions {
        policy.actions = a;
    }
    
    if let Some(r) = resources {
        policy.resources = r;
    }
    
    if let Some(e) = effect {
        let effect_lower = e.to_lowercase();
        if effect_lower != "allow" && effect_lower != "deny" {
            return Err("Effect must be 'allow' or 'deny'".to_string());
        }
        policy.effect = effect_lower;
    }
    
    if let Some(c) = conditions {
        policy.conditions = c;
    }
    
    Ok(UpdateOutput { success: true })
}

#[cfg(test)]
mod tests;
