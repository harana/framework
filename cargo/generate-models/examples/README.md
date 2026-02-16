# Example Schema Files

This directory contains example schema files to demonstrate the code generator.

## Schema: User Management

```yaml
- object: user
  id: [id]
  schema:
    - id: id #required
    - email: string #required #email #unique
    - username: string #required #min(3) #max(50)
    - password_hash: string #required
    - is_active: bool = true
    - role: admin | user | guest = user
    - created_at: datetime = now()
    - updated_at: datetime?
```

## Directory Structure

Example schemas must be organised under category directories that match the schema
root layout:

```
examples/
  object/
    example.yml
```

## Run the Generator

From the project root:

```bash
# Generate both Rust and Python code
cargo generate-models generate \
  --schema-dir crates/cargo-generate-models/examples \
  --rust-output generated/rust \
  --python-output generated/python

# Generate only Rust
cargo generate-models generate \
  --schema-dir crates/cargo-generate-models/examples \
  --rust-output generated/rust \
  --rust-only

# Generate only Python
cargo generate-models generate \
  --schema-dir crates/cargo-generate-models/examples \
  --python-output generated/python \
  --python-only
```

## Generated Output

### Rust
- `generated/rust/object/example.rs` - Contains the generated User struct
- `generated/rust/object/mod.rs` - Module file
- `generated/rust/mod.rs` - Top-level module file

### Python
- `generated/python/object/example.py` - Contains the generated User class
- `generated/python/object/__init__.py` - Package initialization
- `generated/python/__init__.py` - Top-level package initialization
