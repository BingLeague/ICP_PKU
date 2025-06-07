use candid::{CandidType, Principal};
use ic_cdk::api::{self};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub const SECONDS: u64 = 1_000_000_000;

pub fn check_caller(state: PermissionState, function_name: &str) -> Result<(), String> {
    let caller = api::caller();
    if let Some(permission) = state.permissions.get(&caller) {
        // Owner role has access to all functions
        if permission.role == Role::Owner {
            return Ok(());
        }
        // Admin role has access to all admin functions and their allowed functions
        if permission.role == Role::Admin {
            if function_name.starts_with("admin_")
                || permission.allowed_functions.contains(function_name)
            {
                return Ok(());
            }
        }
        // For other roles, check if the function is in their allowed functions
        if permission.allowed_functions.contains(function_name) {
            Ok(())
        } else {
            Err(format!(
                "Unauthorized: No permission to call function '{}'",
                function_name
            ))
        }
    } else {
        Err("Unauthorized: No permissions found".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
pub enum Role {
    Owner,    // Highest privilege, can do everything
    Admin,    // Can manage operators and users
    Operator, // Can perform system operations
    User,     // Basic privileges
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
pub struct Permission {
    pub role: Role,
    pub allowed_functions: HashSet<String>,
    pub created_at: u64,
    pub last_modified: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
pub struct PermissionChange {
    pub target: Principal,
    pub new_role: Role,
    pub new_allowed_functions: HashSet<String>,
    pub approvals: HashSet<Principal>,
    pub proposed_by: Principal,
    pub proposed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
pub struct PermissionLog {
    pub timestamp: u64,
    pub caller: Principal,
    pub action: String,
    pub function_called: String,
    pub result: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, CandidType)]
pub struct PermissionState {
    pub permissions: HashMap<Principal, Permission>,
    pub pending_changes: Vec<PermissionChange>,
    pub logs: Vec<PermissionLog>,
    pub required_approvals: usize,
}

impl Default for PermissionState {
    fn default() -> Self {
        Self {
            permissions: std::collections::HashMap::new(),
            pending_changes: Vec::new(),
            logs: Vec::new(),
            required_approvals: 2, // Default requirement of 2 approvals
        }
    }
}

pub fn check_permission(
    state: &mut PermissionState,
    caller: Principal,
    function: &str,
) -> Result<(), String> {
    let result = match state.permissions.get(&caller) {
        Some(permission) => {
            if permission.allowed_functions.contains(function) {
                Ok(())
            } else {
                Err(format!("No permission to call {}", function))
            }
        }
        None => Err("Unauthorized caller".to_string()),
    };

    // Log the permission check
    let log = PermissionLog {
        timestamp: api::time() / SECONDS,
        caller,
        action: "permission_check".to_string(),
        function_called: function.to_string(),
        result: result.is_ok(),
        details: result.as_ref().err().cloned().unwrap_or_default(),
    };
    state.logs.push(log);
    result
}

pub fn propose_permission_change(
    state: &mut PermissionState,
    caller: Principal,
    target: Principal,
    new_role: Role,
    new_allowed_functions: HashSet<String>,
) -> Result<(), String> {
    // Check if caller has admin privileges
    if let Some(permission) = state.permissions.get(&caller) {
        if permission.role != Role::Admin && permission.role != Role::Owner {
            return Err("Only admin or owner can propose permission changes".to_string());
        }
    } else {
        return Err("Unauthorized caller".to_string());
    }

    let change = PermissionChange {
        target,
        new_role,
        new_allowed_functions,
        approvals: HashSet::from([caller]),
        proposed_by: caller,
        proposed_at: api::time() / SECONDS,
    };

    state.pending_changes.push(change);
    Ok(())
}

pub fn approve_permission_change(
    state: &mut PermissionState,
    caller: Principal,
    target: Principal,
) -> Result<(), String> {
    // Check if caller has admin privileges
    if let Some(permission) = state.permissions.get(&caller) {
        if permission.role != Role::Admin && permission.role != Role::Owner {
            return Err("Only admin or owner can approve permission changes".to_string());
        }
    } else {
        return Err("Unauthorized caller".to_string());
    }

    // First find the pending change index
    let change_idx = state
        .pending_changes
        .iter()
        .position(|c| c.target == target)
        .ok_or_else(|| "No pending change found for target".to_string())?;

    // Get required approvals count before borrowing change
    let required_approvals = state.required_approvals;

    // Get the change and update it
    let change = &mut state.pending_changes[change_idx];
    change.approvals.insert(caller);

    // If we have enough approvals, apply the change
    if change.approvals.len() >= required_approvals {
        let now = api::time() / SECONDS;

        let new_permission = Permission {
            role: change.new_role.clone(),
            allowed_functions: change.new_allowed_functions.clone(),
            created_at: now,
            last_modified: now,
        };

        // Apply the change
        state.permissions.insert(target, new_permission.clone());

        // Add to logs
        state.logs.push(PermissionLog {
            timestamp: now,
            action: format!(
                "Permission change approved for {:?}: {:?}",
                target, new_permission
            ),
            caller,
            function_called: "approve_permission_change".to_string(),
            result: true,
            details: format!("Permission changed for {:?}", target),
        });

        // Remove the approved change
        state.pending_changes.remove(change_idx);

        Ok(())
    } else {
        Ok(())
    }
}

pub fn get_permission(state: &PermissionState, principal: Principal) -> Option<Permission> {
    state.permissions.get(&principal).cloned()
}

pub fn get_pending_changes(state: &PermissionState) -> Vec<PermissionChange> {
    state.pending_changes.clone()
}

pub fn get_recent_logs(state: &PermissionState, limit: usize) -> Vec<PermissionLog> {
    let state = state;
    state.logs.iter().rev().take(limit).cloned().collect()
}
