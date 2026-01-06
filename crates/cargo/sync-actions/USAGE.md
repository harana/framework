# cargo-sync-actions - Usage Guide

A Cargo plugin that syncs action methods from YAML schema definitions to Rust crates, automatically adding missing functions and output structs.

## Quick Start

1. **Install the plugin:**
   ```bash
   cd crates/cargo/sync-actions
   ./install.sh
   ```

2. **Sync all actions:**
   ```bash
   cargo sync-actions sync
   ```

3. **Dry run to see what would change:**
   ```bash
   cargo sync-actions sync --dry-run
   ```

## Command-Line Options

### `sync` Command

```bash
cargo sync-actions sync [OPTIONS]
```

#### Options

- `--schema-dir <PATH>` - Path to directory containing YAML action schema files (default: `core/schema/actions`)
- `--actions-dir <PATH>` - Path to the actions crates directory (default: `crates/actions`)
- `--dry-run` - Show what would be changed without making changes
- `--module <NAME>` - Only process a specific action module (e.g., `aws_iam`)

## Examples

### Sync all actions

```bash
cargo sync-actions sync
```

### Dry run to preview changes

```bash
cargo sync-actions sync --dry-run
```

### Sync only a specific module

```bash
cargo sync-actions sync --module aws_iam
```

### Use custom paths

```bash
cargo sync-actions sync \
  --schema-dir /path/to/schemas \
  --actions-dir /path/to/actions
```

## How It Works

1. **Parse YAML schemas**: The tool reads all YAML files from `core/schema/actions/`
2. **Analyze Rust crates**: It examines existing Rust code in `crates/actions/`
3. **Identify gaps**: Compares YAML definitions with existing functions and output structs
4. **Generate code**: Creates missing Rust function signatures and output structs
5. **Insert code**: Adds new code to the appropriate files (`lib.rs` and `output.rs`)

## YAML Schema Format

The tool expects YAML files in the following format:

```yaml
- name: Create IAM User
  class: create_user
  inputs:
    user_name: string #required
    path: string = /
    permissions_boundary: string
    tags: object
  outputs:
    arn: string
    create_date: datetime
    success: boolean
    user_id: string
    user_name: string
```

### Field Format

Fields can be specified with:
- **Type only**: `field_name: string`
- **Required marker**: `field_name: string #required`
- **Default value**: `field_name: string = default_value`
- **Combined**: `field_name: string = default #required`

### Supported Types

| YAML Type | Rust Input Type | Rust Output Type |
|-----------|----------------|------------------|
| `string` | `&str` / `Option<&str>` | `String` |
| `integer` | `i32` / `Option<i32>` | `i32` |
| `float` | `f64` / `Option<f64>` | `f64` |
| `boolean` | `bool` / `Option<bool>` | `bool` |
| `datetime` | `&str` / `Option<&str>` | `String` |
| `bytes` | `&[u8]` / `Option<&[u8]>` | `Vec<u8>` |
| `object` | `HashMap<String, Value>` / `Option<...>` | `HashMap<String, Value>` |
| `list[string]` | `Vec<String>` / `Option<Vec<String>>` | `Vec<String>` |
| `list[object]` | `Vec<HashMap<String, Value>>` / `Option<...>` | `Vec<HashMap<String, Value>>` |

## Generated Code

### Function Signature

For an action like:

```yaml
- name: Create IAM User
  class: create_user
  inputs:
    user_name: string #required
    path: string = /
  outputs:
    arn: string
    success: boolean
```

The tool generates:

```rust
/// Create IAM User
pub async fn create_user(
    user_name: &str,
    path: Option<&str>,
) -> Result<CreateUserOutput, String> {
    unimplemented!("create_user")
}
```

### Output Struct

```rust
// create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub arn: String,
    pub success: bool
}
```

## Project Structure

After running the tool, your action crates will have:

```
crates/actions/aws_iam/
├── src/
│   ├── lib.rs       # Function signatures (synced)
│   └── output.rs    # Output structs (synced)
├── Cargo.toml
└── ...
```

## Workflow Integration

### Development Workflow

1. Define new actions in YAML schemas (`core/schema/actions/`)
2. Run `cargo sync-actions sync --dry-run` to preview changes
3. Run `cargo sync-actions sync` to add missing methods
4. Implement the actual logic in the generated function stubs
5. Commit both YAML and Rust changes

### CI/CD Integration

Add to your CI pipeline to ensure schemas and code stay in sync:

```bash
# Check if sync would add anything (exit code 0 if in sync)
cargo sync-actions sync --dry-run
```

## Maintenance

### Updating Existing Functions

The tool **only adds** missing functions and structs. It does not:
- Modify existing functions
- Remove functions that aren't in YAML
- Change function signatures

To update an existing function:
1. Update the YAML schema
2. Manually update the Rust function signature
3. Run tests to ensure compatibility

### Handling Conflicts

If a function exists in Rust but not in YAML:
- The tool will not remove it
- Consider adding it to YAML if it should be part of the API

If a function signature doesn't match the YAML:
- Manually reconcile the differences
- The tool won't override existing code

## Troubleshooting

### "Crate directory does not exist"

The tool expects the crate directory to exist. If you see this error:
1. Create the crate directory: `mkdir -p crates/actions/module_name`
2. Add basic `src/lib.rs` and `Cargo.toml` files
3. Run the sync command again

### Parse Errors

If YAML parsing fails:
- Check YAML syntax with a validator
- Ensure all required fields (`name`, `class`) are present
- Verify field definitions follow the expected format

### Missing Imports

If generated code has compilation errors:
- Check that `output.rs` has necessary imports:
  ```rust
  use serde::{Deserialize, Serialize};
  use serde_json::Value;
  use std::collections::HashMap;
  ```

## Advanced Usage

### Custom Module Structure

If your modules don't follow the standard structure, you can:
1. Modify the `analyzer.rs` to match your structure
2. Update path resolution in `syncer.rs`

### Adding Custom Attributes

To add custom attributes to generated code:
1. Edit `generator.rs`
2. Modify the template strings in `generate_function()` and `generate_output_struct()`

## Contributing

When adding new features:
1. Update the parser for new YAML syntax
2. Update the generator for new Rust patterns
3. Add tests for the new functionality
4. Update this documentation

## License

MIT
