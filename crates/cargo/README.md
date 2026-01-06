# Cargo Plugins

This directory contains custom Cargo plugins for the Harana framework.

## Available Plugins

### 📦 generate-models

Generates Python and Rust model code from YAML schema definitions.

**Location:** `crates/cargo/generate-models`

**Usage:**
```bash
cd crates/cargo/generate-models
./install.sh
cargo generate-models generate
```

**Documentation:** See [generate-models/USAGE.md](generate-models/USAGE.md)

---

### 🔄 sync-actions

Syncs action methods from YAML schemas to Rust crates, automatically adding missing function signatures and output structs.

**Location:** `crates/cargo/sync-actions`

**Usage:**
```bash
cd crates/cargo/sync-actions
./install.sh
cargo sync-actions sync --dry-run
```

**Documentation:** See [sync-actions/USAGE.md](sync-actions/USAGE.md)

---

## Installation

Each plugin can be installed independently:

```bash
# Install generate-models
cd crates/cargo/generate-models && ./install.sh

# Install sync-actions
cd crates/cargo/sync-actions && ./install.sh
```

After installation, the plugins will be available globally as:
- `cargo generate-models`
- `cargo sync-actions`

## Development

Each plugin is a standalone Rust project with its own workspace to avoid dependency conflicts with the main framework.

### Building

```bash
# Build generate-models
cd crates/cargo/generate-models && cargo build --release

# Build sync-actions
cd crates/cargo/sync-actions && cargo build --release
```

### Structure

```
crates/cargo/
├── generate-models/
│   ├── src/
│   ├── Cargo.toml
│   ├── install.sh
│   ├── Makefile
│   ├── README.md
│   └── USAGE.md
└── sync-actions/
    ├── src/
    ├── Cargo.toml
    ├── install.sh
    ├── Makefile
    ├── README.md
    ├── USAGE.md
    └── STATUS.md
```

## License

MIT
