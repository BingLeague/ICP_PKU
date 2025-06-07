# Shard Library

A comprehensive permission control system for Internet Computer applications, providing role-based access control (RBAC) with multi-signature support and detailed audit logging capabilities.

## Features

- **Role-Based Access Control (RBAC)**
  - Hierarchical role system (Owner, Admin, Operator, User)
  - Fine-grained function-level permissions
  - Role-specific access control

- **Multi-Signature Support**
  - Two-step permission change process
  - Requires multiple approvals for critical changes
  - Prevents single point of failure

- **Audit Logging**
  - Detailed logs for all permission checks
  - Timestamp and caller tracking
  - Complete audit trail for permission changes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shard_lib = "0.1.0"
```

## Usage

### Basic Permission Check

```rust
use shard_lib::permission::{check_permission, Role};

// Check if a caller has permission to execute a function
let result = check_permission(caller, "function_name")?;
```

### Managing Permissions

```rust
use shard_lib::permission::{propose_permission_change, Role};

// Propose a permission change
let new_allowed_functions = HashSet::from(["function1", "function2"]);
propose_permission_change(caller, target, Role::Operator, new_allowed_functions)?;

// Approve a proposed change
approve_permission_change(admin_caller, target)?;
```

### Audit Logging

```rust
use shard_lib::permission::get_recent_logs;

// Get recent permission check logs
let logs = get_recent_logs(10); // Get last 10 logs
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
