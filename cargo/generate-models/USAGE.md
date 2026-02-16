# cargo-generate-models - Usage Guide

## Quick Start

1. **Install the plugin:**
   ```bash
   cd crates/cargo/generate-models
   ./install.sh
   ```

2. **Generate models from your schemas:**
   ```bash
   cargo generate-models generate \
     --schema-dir core/schema/data \
     --rust-output generated/rust \
     --python-output generated/python
   ```

## Command-Line Options

### `generate` Command

```bash
cargo generate-models generate [OPTIONS]
```

#### Options

- `--schema-dir <PATH>` - Path to directory containing YAML schema files (default: `core/schema/data`)
- `--rust-output <PATH>` - Output directory for Rust code (default: `generated/rust`)
- `--python-output <PATH>` - Output directory for Python code (default: `generated/python`)
- `--rust-only` - Generate only Rust code
- `--python-only` - Generate only Python code

## Schema Format Reference

### Directory Structure

Schema files are organised under category directories:

```
schema/
  config/       # Configuration models (key: "config")
  event/        # Event models (key: "event")
  flow/         # Flow action methods and helper objects (keys: "action", "object")
  object/       # Data object models (key: "object")
  webobject/    # Web component models (key: "webobject")
```

### Object / Config / Event Schemas

```yaml
- object: model_name
  id: [field1, field2]
  schema:
    - field_name: type [modifiers] [constraints] [reference]
```

The top-level key matches the category directory (`object`, `config`, or `event`).

### Flow Schemas

Flow files contain action methods and helper object classes:

```yaml
- action: method_name
  inputs:
    field_name: type [modifiers]
  outputs:
    field_name: type [modifiers]

- object: helper_class_name
  class:
    field_name: type
```

### WebObject Schemas

```yaml
- webobject: component_name
  attributes:
    - field_name: type [modifiers] [constraints]
  events:
    - event_name
```

### Supported Types

| YAML Type | Rust Type | Python Type |
|-----------|-----------|-------------|
| `string` | `String` | `str` |
| `int` | `i64` | `int` |
| `decimal` | `f64` | `float` |
| `bool` | `bool` | `bool` |
| `datetime` | `chrono::DateTime<Utc>` | `datetime` |
| `id` | `String` | `str` |
| `type1 \| type2` | Enum | Enum |

### Type Modifiers

- `?` - Makes field nullable/optional
  ```yaml
  - email: string?  # Optional[str] in Python, Option<String> in Rust
  ```

- `= value` - Sets default value
  ```yaml
  - is_active: bool = true
  - role: admin | user = user
  - created_at: datetime = now()  # Special: current timestamp
  ```

### Constraints

All constraints start with `#`:

| Constraint | Description | Example |
|------------|-------------|---------|
| `#required` | Field is required | `- email: string #required` |
| `#unique` | Field must be unique | `- username: string #unique` |
| `#min(n)` | Minimum value/length | `- age: int #min(18)` |
| `#max(n)` | Maximum value/length | `- bio: string #max(500)` |
| `#email` | Email validation | `- email: string #email` |
| `#url` | URL validation | `- website: string #url` |
| `#lowercase` | Must be lowercase | `- slug: string #lowercase` |
| `#uppercase` | Must be uppercase | `- code: string #uppercase` |

### References (Foreign Keys)

```yaml
- user_id: id #required -> User.id
```

## Examples

### Simple Model

```yaml
- object: tag
  id: [id]
  schema:
    - id: id #required
    - name: string #required #max(50)
    - slug: string #required #unique #lowercase
    - created_at: datetime = now()
```

**Generated Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}
```

**Generated Python:**
```python
class Tag(BaseModel):
    id: str
    name: str = Field(max_length=50)
    slug: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
```

### Model with Enums

```yaml
- object: article
  id: [id]
  schema:
    - id: id #required
    - title: string #required #max(200)
    - status: draft | published | archived = draft
    - visibility: public | private | unlisted = public
```

### Model with References

```yaml
- object: comment
  id: [id]
  schema:
    - id: id #required
    - article_id: id #required -> Article.id
    - author_id: id #required -> User.id
    - content: string #required #max(2000)
    - created_at: datetime = now()
```

### Complex Model

```yaml
- object: order
  id: [id]
  schema:
    - id: id #required
    - user_id: id #required -> User.id
    - order_number: string #required #unique
    - status: pending | processing | shipped | delivered | cancelled = pending
    - total_amount: decimal #required #min(0)
    - discount_amount: decimal = 0 #min(0)
    - tax_amount: decimal #required #min(0)
    - shipping_address: string #required
    - billing_address: string?
    - notes: string? #max(1000)
    - created_at: datetime = now()
    - updated_at: datetime?
    - shipped_at: datetime?
    - delivered_at: datetime?
```

## Generated Code Features

### Rust

- ✅ Structs with `#[derive(Debug, Clone, Serialize, Deserialize)]`
- ✅ Optional fields using `Option<T>`
- ✅ Enums for union types
- ✅ Serde attributes for JSON serialization
- ✅ Default value support
- ✅ Validation methods
- ✅ Documentation comments

### Python

- ✅ Pydantic `BaseModel` classes
- ✅ Type hints with `Optional[]`
- ✅ Enums as `str, Enum`
- ✅ Field validators for email, URL, etc.
- ✅ Default values with `Field()`
- ✅ Pydantic Config for ORM compatibility
- ✅ Comprehensive docstrings

## Integration

### Using Generated Rust Code

```rust
use generated::route::Route;
use chrono::Utc;

fn main() {
    let route = Route {
        created_at: Utc::now(),
        description: Some("API endpoint".to_string()),
        is_active: true,
        method: "GET".to_string(),
        middleware: None,
        path: "/api/users".to_string(),
        rate_limit: Some(100),
        updated_at: Utc::now(),
    };

    if let Err(e) = route.validate() {
        eprintln!("Validation error: {}", e);
    }

    let json = serde_json::to_string(&route).unwrap();
    println!("{}", json);
}
```

### Using Generated Python Code

```python
from generated.route import Route
from datetime import datetime

# Create instance
route = Route(
    path="/api/users",
    method="GET",
    description="API endpoint",
    rate_limit=100
)

# Validation happens automatically
print(route.json())

# From dictionary
data = {"path": "/api/posts", "method": "POST"}
route = Route(**data)

# To dictionary
route_dict = route.dict()
```

## Tips & Best Practices

1. **Keep schemas in version control** - Treat schema files as source of truth
2. **Generate into separate directories** - Keep generated code separate from handwritten code
3. **Add generated directories to .gitignore** if you prefer to generate on build
4. **Use references liberally** - Document relationships between models
5. **Leverage constraints** - Let the generator create validation logic
6. **Use meaningful defaults** - Make your APIs easier to use

## Troubleshooting

### No models found

- Check that schema directory path is correct
- Verify files have `.yml` extension
- Ensure YAML is valid
- Ensure schema files are under a recognised category directory (`config/`, `event/`, `flow/`, `object/`, `webobject/`)

### Validation errors

- Check that the correct category key is used (`object:`, `event:`, `config:`, `action:`, `webobject:`)
- For models without explicit `id` field, one will be auto-generated as `["id"]`

### Generated code doesn't compile

- Ensure schema types are valid
- Check for circular references
- Verify constraint values are appropriate for field types

## Contributing

To extend the generator:

1. **Add new type mappings** in `src/models.rs`
2. **Add new constraints** in `src/parser.rs`
3. **Extend generators** in `src/generators/rust.rs` or `src/generators/python.rs`
4. **Add tests** for new features
