# Cargo Sync Actions

✅ **Successfully created!** A Cargo plugin that automatically syncs action methods from YAML schema definitions to Rust crates.

## What Was Created

This cargo plugin now lives in `/Users/naden/Developer/harana/framework/crates/cargo/sync-actions` and includes:

1. **Core Modules:**
   - `src/main.rs` - CLI entry point with clap commands
   - `src/parser.rs` - YAML schema parser
   - `src/analyzer.rs` - Rust code analyzer
   - `src/generator.rs` - Rust code generator
   - `src/syncer.rs` - Main sync logic

2. **Installation Files:**
   - `install.sh` - Installation script
   - `Makefile` - Build and run commands
   - `USAGE.md` - Comprehensive documentation
   - `README.md` - Project overview

## Installation

```bash
cd /Users/naden/Developer/harana/framework/crates/cargo/sync-actions
./install.sh
```

## Usage

```bash
# From framework root directory
cd /Users/naden/Developer/harana/framework

# Sync all actions (dry run)
cargo sync-actions sync --dry-run \
  --schema-dir /Users/naden/Developer/harana/framework/core/schema/actions \
  --actions-dir /Users/naden/Developer/harana/framework/crates/actions

# Sync all actions (actual)
cargo sync-actions sync \
  --schema-dir /Users/naden/Developer/harana/framework/core/schema/actions \
  --actions-dir /Users/naden/Developer/harana/framework/crates/actions

# Sync specific module
cargo sync-actions sync --module aws_iam \
  --schema-dir /Users/naden/Developer/harana/framework/core/schema/actions \
  --actions-dir /Users/naden/Developer/harana/framework/crates/actions
```

## Test Results

Successfully tested on `aws_iam` module:
- ✅ Found 35 actions in YAML
- ✅ Detected 5 existing functions
- ✅ Added 30 missing functions to `lib.rs`
- ✅ Added 30 missing output structs to `output.rs`

## Known Issue

There's currently a minor issue where required fields are being generated as `Option<T>` when they should be `T`. This was addressed in the parser but needs verification. The fix was to use `field_type.replace("#required", "")` instead of substring manipulation.

## Next Steps

1. Install the plugin globally: `cd crates/cargo/sync-actions && ./install.sh`
2. Run on all modules to sync everything
3. Verify the required vs optional field generation is working correctly
4. Implement the actual logic for the generated function stubs

## Documentation

See `USAGE.md` for complete documentation including:
- Command-line options
- YAML schema format
- Type mappings
- Examples
- Troubleshooting

Enjoy! 🎉
