pub mod output;

use output::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use rand::Rng;
use serde_json::{json, Value};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use uuid::Uuid;

/// Storage for users.
static USERS: Lazy<DashMap<String, StoredUser>> = Lazy::new(DashMap::new);
/// Index by username.
static USERNAME_INDEX: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);
/// Index by email.
static EMAIL_INDEX: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);
/// Storage for tokens.
static TOKENS: Lazy<DashMap<String, TokenInfo>> = Lazy::new(DashMap::new);
/// Storage for refresh tokens.
static REFRESH_TOKENS: Lazy<DashMap<String, RefreshTokenInfo>> = Lazy::new(DashMap::new);
/// Storage for password reset tokens.
static RESET_TOKENS: Lazy<DashMap<String, ResetTokenInfo>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone)]
struct StoredUser {
    user_id: String,
    username: String,
    email: String,
    password_hash: String,
    roles: Vec<String>,
    metadata: HashMap<String, Value>,
    created_at: String,
    deleted: bool,
}

#[derive(Debug, Clone)]
struct TokenInfo {
    user_id: String,
    expires_at: String,
    claims: HashMap<String, Value>,
    revoked: bool,
}

#[derive(Debug, Clone)]
struct RefreshTokenInfo {
    user_id: String,
    access_token: String,
    expires_at: String,
}

#[derive(Debug, Clone)]
struct ResetTokenInfo {
    user_id: String,
    expires_at: String,
    used: bool,
}

/// Hash a password (simplified for demo).
fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(b"salt_for_demo");
    format!("{:x}", hasher.finalize())
}

/// Generate a random token.
fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Authenticate User Credentials.
pub async fn authenticate(
    username: &str,
    password: &str,
    _mfa_code: Option<&str>,
) -> Result<AuthenticateOutput, String> {
    // Find user by username
    let user_id = USERNAME_INDEX
        .get(username)
        .map(|v| v.clone())
        .ok_or("Invalid username or password")?;
    
    let user = USERS
        .get(&user_id)
        .ok_or("Invalid username or password")?;
    
    if user.deleted {
        return Err("Account has been deleted".to_string());
    }
    
    // Verify password
    let password_hash = hash_password(password);
    if user.password_hash != password_hash {
        return Err("Invalid username or password".to_string());
    }
    
    // Generate tokens
    let access_token = generate_token();
    let refresh_token = generate_token();
    let expires_at = (Utc::now() + Duration::hours(1)).to_rfc3339();
    let refresh_expires_at = (Utc::now() + Duration::days(7)).to_rfc3339();
    
    // Store access token
    let mut claims = HashMap::new();
    claims.insert("user_id".to_string(), json!(user.user_id.clone()));
    claims.insert("username".to_string(), json!(user.username.clone()));
    claims.insert("roles".to_string(), json!(user.roles.clone()));
    
    TOKENS.insert(access_token.clone(), TokenInfo {
        user_id: user.user_id.clone(),
        expires_at: expires_at.clone(),
        claims,
        revoked: false,
    });
    
    // Store refresh token
    REFRESH_TOKENS.insert(refresh_token.clone(), RefreshTokenInfo {
        user_id: user.user_id.clone(),
        access_token: access_token.clone(),
        expires_at: refresh_expires_at,
    });
    
    Ok(AuthenticateOutput {
        access_token,
        refresh_token,
        expires_at,
        success: true,
        user_id: user.user_id.clone(),
    })
}

/// Change User Password.
pub async fn change_password(
    new_password: &str,
    current_password: &str,
    user_id: &str,
) -> Result<ChangePasswordOutput, String> {
    let mut user = USERS
        .get_mut(user_id)
        .ok_or_else(|| format!("User not found: {}", user_id))?;
    
    // Verify current password
    let current_hash = hash_password(current_password);
    if user.password_hash != current_hash {
        return Err("Current password is incorrect".to_string());
    }
    
    // Update password
    user.password_hash = hash_password(new_password);
    
    Ok(ChangePasswordOutput { success: true })
}

/// Create New User.
pub async fn create_user(
    username: &str,
    password: &str,
    email: &str,
    roles: Option<Vec<String>>,
    metadata: Option<HashMap<String, Value>>,
) -> Result<CreateUserOutput, String> {
    // Check if username exists
    if USERNAME_INDEX.contains_key(username) {
        return Err("Username already exists".to_string());
    }
    
    // Check if email exists
    if EMAIL_INDEX.contains_key(email) {
        return Err("Email already exists".to_string());
    }
    
    let user_id = Uuid::new_v4().to_string();
    let password_hash = hash_password(password);
    
    let user = StoredUser {
        user_id: user_id.clone(),
        username: username.to_string(),
        email: email.to_string(),
        password_hash,
        roles: roles.unwrap_or_default(),
        metadata: metadata.unwrap_or_default(),
        created_at: Utc::now().to_rfc3339(),
        deleted: false,
    };
    
    // Update indexes
    USERNAME_INDEX.insert(username.to_string(), user_id.clone());
    EMAIL_INDEX.insert(email.to_string(), user_id.clone());
    USERS.insert(user_id.clone(), user);
    
    Ok(CreateUserOutput {
        success: true,
        user_id,
    })
}

/// Delete User Account.
pub async fn delete_user(
    user_id: &str,
    hard_delete: Option<bool>,
) -> Result<DeleteUserOutput, String> {
    let hard_delete = hard_delete.unwrap_or(false);
    
    if hard_delete {
        let removed = USERS
            .remove(user_id)
            .ok_or_else(|| format!("User not found: {}", user_id))?;
        
        let user = removed.1;
        USERNAME_INDEX.remove(&user.username);
        EMAIL_INDEX.remove(&user.email);
    } else {
        let mut user = USERS
            .get_mut(user_id)
            .ok_or_else(|| format!("User not found: {}", user_id))?;
        
        user.deleted = true;
    }
    
    Ok(DeleteUserOutput { success: true })
}

/// Get User By ID.
pub async fn get_user(
    user_id: &str,
) -> Result<GetUserOutput, String> {
    let user = USERS
        .get(user_id)
        .ok_or_else(|| format!("User not found: {}", user_id))?;
    
    if user.deleted {
        return Err(format!("User not found: {}", user_id));
    }
    
    Ok(GetUserOutput {
        user_id: user.user_id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        roles: user.roles.clone(),
        metadata: user.metadata.clone(),
        created_at: user.created_at.clone(),
    })
}

/// List Users.
pub async fn list_users(
    limit: Option<i32>,
    roles: Option<Vec<String>>,
    offset: Option<i32>,
    search: Option<&str>,
) -> Result<ListUsersOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;
    
    let mut users: Vec<HashMap<String, Value>> = USERS
        .iter()
        .filter(|u| {
            // Skip deleted users
            if u.deleted {
                return false;
            }
            
            // Filter by roles if specified
            if let Some(ref required_roles) = roles {
                if !required_roles.iter().any(|r| u.roles.contains(r)) {
                    return false;
                }
            }
            
            // Filter by search term
            if let Some(term) = search {
                let term_lower = term.to_lowercase();
                if !u.username.to_lowercase().contains(&term_lower)
                    && !u.email.to_lowercase().contains(&term_lower)
                {
                    return false;
                }
            }
            
            true
        })
        .map(|u| {
            let mut map = HashMap::new();
            map.insert("user_id".to_string(), json!(u.user_id.clone()));
            map.insert("username".to_string(), json!(u.username.clone()));
            map.insert("email".to_string(), json!(u.email.clone()));
            map.insert("roles".to_string(), json!(u.roles.clone()));
            map.insert("created_at".to_string(), json!(u.created_at.clone()));
            map
        })
        .collect();
    
    let total = users.len() as i32;
    let users: Vec<_> = users.into_iter().skip(offset).take(limit).collect();
    
    Ok(ListUsersOutput { total, users })
}

/// Refresh Access Token.
pub async fn refresh_token(
    refresh_token: &str,
) -> Result<RefreshTokenOutput, String> {
    let refresh_info = REFRESH_TOKENS
        .get(refresh_token)
        .ok_or("Invalid refresh token")?;
    
    // Check if expired
    let expires_at = chrono::DateTime::parse_from_rfc3339(&refresh_info.expires_at)
        .map_err(|_| "Invalid expiration date")?;
    
    if expires_at < Utc::now() {
        return Err("Refresh token expired".to_string());
    }
    
    // Revoke old access token
    if let Some(mut token) = TOKENS.get_mut(&refresh_info.access_token) {
        token.revoked = true;
    }
    
    // Get user
    let user = USERS
        .get(&refresh_info.user_id)
        .ok_or("User not found")?;
    
    // Generate new tokens
    let new_access_token = generate_token();
    let new_refresh_token = generate_token();
    let new_expires_at = (Utc::now() + Duration::hours(1)).to_rfc3339();
    let new_refresh_expires_at = (Utc::now() + Duration::days(7)).to_rfc3339();
    
    // Store new access token
    let mut claims = HashMap::new();
    claims.insert("user_id".to_string(), json!(user.user_id.clone()));
    claims.insert("username".to_string(), json!(user.username.clone()));
    claims.insert("roles".to_string(), json!(user.roles.clone()));
    
    TOKENS.insert(new_access_token.clone(), TokenInfo {
        user_id: user.user_id.clone(),
        expires_at: new_expires_at.clone(),
        claims,
        revoked: false,
    });
    
    // Store new refresh token
    REFRESH_TOKENS.insert(new_refresh_token.clone(), RefreshTokenInfo {
        user_id: user.user_id.clone(),
        access_token: new_access_token.clone(),
        expires_at: new_refresh_expires_at,
    });
    
    // Remove old refresh token
    drop(refresh_info);
    REFRESH_TOKENS.remove(refresh_token);
    
    Ok(RefreshTokenOutput {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_at: new_expires_at,
    })
}

/// Request Password Reset.
pub async fn request_password_reset(
    email: &str,
) -> Result<RequestPasswordResetOutput, String> {
    let user_id = EMAIL_INDEX
        .get(email)
        .map(|v| v.clone())
        .ok_or("Email not found")?;
    
    let reset_token = generate_token();
    let expires_at = (Utc::now() + Duration::hours(1)).to_rfc3339();
    
    RESET_TOKENS.insert(reset_token.clone(), ResetTokenInfo {
        user_id,
        expires_at,
        used: false,
    });
    
    Ok(RequestPasswordResetOutput {
        success: true,
        reset_token,
    })
}

/// Reset Password With Token.
pub async fn reset_password(
    reset_token: &str,
    new_password: &str,
) -> Result<ResetPasswordOutput, String> {
    let mut reset_info = RESET_TOKENS
        .get_mut(reset_token)
        .ok_or("Invalid reset token")?;
    
    // Check if already used
    if reset_info.used {
        return Err("Reset token already used".to_string());
    }
    
    // Check if expired
    let expires_at = chrono::DateTime::parse_from_rfc3339(&reset_info.expires_at)
        .map_err(|_| "Invalid expiration date")?;
    
    if expires_at < Utc::now() {
        return Err("Reset token expired".to_string());
    }
    
    // Update password
    let mut user = USERS
        .get_mut(&reset_info.user_id)
        .ok_or("User not found")?;
    
    user.password_hash = hash_password(new_password);
    
    // Mark token as used
    reset_info.used = true;
    
    Ok(ResetPasswordOutput { success: true })
}

/// Revoke Access Token.
pub async fn revoke_token(
    token: &str,
) -> Result<RevokeTokenOutput, String> {
    let mut token_info = TOKENS
        .get_mut(token)
        .ok_or("Token not found")?;
    
    token_info.revoked = true;
    
    Ok(RevokeTokenOutput { success: true })
}

/// Update User Details.
pub async fn update_user(
    user_id: &str,
    metadata: Option<HashMap<String, Value>>,
    roles: Option<Vec<String>>,
    email: Option<&str>,
) -> Result<UpdateUserOutput, String> {
    let mut user = USERS
        .get_mut(user_id)
        .ok_or_else(|| format!("User not found: {}", user_id))?;
    
    if user.deleted {
        return Err(format!("User not found: {}", user_id));
    }
    
    if let Some(new_email) = email {
        // Check if new email is already taken by another user
        if let Some(existing_id) = EMAIL_INDEX.get(new_email) {
            if existing_id.value() != user_id {
                return Err("Email already in use".to_string());
            }
        }
        
        // Update email index
        EMAIL_INDEX.remove(&user.email);
        EMAIL_INDEX.insert(new_email.to_string(), user_id.to_string());
        user.email = new_email.to_string();
    }
    
    if let Some(new_roles) = roles {
        user.roles = new_roles;
    }
    
    if let Some(new_metadata) = metadata {
        user.metadata = new_metadata;
    }
    
    Ok(UpdateUserOutput { success: true })
}

/// Verify Access Token.
pub async fn verify_token(
    token: &str,
) -> Result<VerifyTokenOutput, String> {
    let token_info = TOKENS
        .get(token)
        .ok_or("Token not found")?;
    
    // Check if revoked
    if token_info.revoked {
        return Ok(VerifyTokenOutput {
            valid: false,
            user_id: token_info.user_id.clone(),
            claims: token_info.claims.clone(),
            expires_at: token_info.expires_at.clone(),
        });
    }
    
    // Check if expired
    let expires_at = chrono::DateTime::parse_from_rfc3339(&token_info.expires_at)
        .map_err(|_| "Invalid expiration date")?;
    
    let valid = expires_at > Utc::now();
    
    Ok(VerifyTokenOutput {
        valid,
        user_id: token_info.user_id.clone(),
        claims: token_info.claims.clone(),
        expires_at: token_info.expires_at.clone(),
    })
}

#[cfg(test)]
mod tests;
