# cargo-sync-actions

A Cargo plugin that automatically syncs action methods from YAML schema definitions to Rust crates.

## Overview

`cargo-sync-actions` reads YAML action schemas from `core/schema/actions/` and syncs them with Rust crates in `crates/actions/`. It automatically:

- ✅ Detects missing function implementations
- ✅ Detects missing output structs
- ✅ Generates proper Rust function signatures with correct parameter types
- ✅ Generates output structs with proper field types
- ✅ Preserves existing code and only adds what's missing
- ✅ Supports dry-run mode to preview changes

## Installation

```bash
cd crates/cargo/sync-actions
./install.sh
```

Or use Make:

```bash
make install
```

## Quick Start

```bash
# Preview what would change
cargo sync-actions sync --dry-run

# Sync all actions
cargo sync-actions sync

# Sync only one module
cargo sync-actions sync --module aws_iam
```

## Usage

See [USAGE.md](USAGE.md) for detailed documentation.

## Example

Given this YAML schema in `core/schema/actions/aws_iam.yml`:

```yaml
- name: Create IAM Role
  class: create_role
  inputs:
    role_name: string #required
    assume_role_policy_document: string #required
    description: string
    max_session_duration: integer = 3600
  outputs:
    arn: string
    role_id: string
    success: boolean
```

The tool will generate in `crates/actions/aws_iam/src/lib.rs`:

```rust
/// Create IAM Role
pub async fn create_role(
    role_name: &str,
    assume_role_policy_document: &str,
    description: Option<&str>,
    max_session_duration: Option<i32>,
) -> Result<CreateRoleOutput, String> {
    unimplemented!("create_role")
}
```

And in `crates/actions/aws_iam/src/output.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleOutput {
    pub arn: String,
    pub role_id: String,
    pub success: bool
}
```

## Features

- **Smart Type Mapping**: Converts YAML types to appropriate Rust types
- **Required vs Optional**: Automatically uses `Option<T>` for optional parameters
- **List Support**: Handles `list[type]` with proper `Vec<T>` types
- **Object Support**: Maps object types to `HashMap<String, Value>`
- **Non-Destructive**: Only adds missing code, never removes or modifies existing code
- **Dry Run**: Preview changes before applying them
- **Selective Sync**: Sync specific modules with `--module` flag

## Type Mappings

| YAML Type | Rust Input | Rust Output |
|-----------|------------|-------------|
| `string` | `&str` | `String` |
| `integer` | `i32` | `i32` |
| `float` | `f64` | `f64` |
| `boolean` | `bool` | `bool` |
| `datetime` | `&str` | `String` |
| `bytes` | `&[u8]` | `Vec<u8>` |
| `object` | `HashMap<String, Value>` | `HashMap<String, Value>` |
| `list[T]` | `Vec<T>` | `Vec<T>` |

Optional fields automatically get wrapped in `Option<T>`.

## Development

```bash
# Build
make build

# Test
make test

# Run in dev mode
make dev

# Dry run example
make run-example
```

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust best practices
2. Tests pass: `cargo test`
3. Documentation is updated
4. USAGE.md reflects any new features

## License

MIT

## Author

Harana Team
