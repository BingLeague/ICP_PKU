//! # Shard Library
//!
//! A comprehensive permission control system for Internet Computer applications.
//! This library provides role-based access control (RBAC) with multi-signature support
//! and detailed audit logging capabilities.
//!
//! ## Features
//!
//! - Role-based access control (RBAC)
//! - Multi-signature permission changes
//! - Detailed audit logging
//! - Granular permission control
//! - Type-safe implementation
//!
//! ## Example
//!
//! ```rust
//! use shard_lib::permission::{check_permission, Role};
//!
//! // Check if a caller has permission to execute a function
//! let result = check_permission(caller, "function_name");
//! ```

pub mod constant;
pub mod permission;
pub mod token;
pub mod types;
pub mod utils;

// Re-export commonly used types and functions
pub use constant::*;
pub use permission::*;
pub use token::*;
pub use types::*;
pub use utils::*;
