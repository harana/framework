# Action to Library Mapping

This document maps each action in `/core/schema/actions` to the relevant Rust libraries available in the workspace or suggested additions.

| Action | Current Libraries | Suggested Additions |
|--------|-------------------|---------------------|
| `address` | `geo-uri`, `latlon`, `geoutils` | `address-formatter`, `libpostal` |
| `ai` | - | `async-openai`, `ollama-rs`, `langchain-rust` |
| `archive` | - | `zip`, `tar`, `flate2`, `bzip2`, `xz2` |
| `audit` | `tracing`, `opentelemetry` | `audit-log` |
| `aws_cloudfront` | `aws-sdk-cloudfront`, `aws-config` | - |
| `aws_ecr` | `aws-sdk-ecr`, `aws-config` | - |
| `aws_eventbridge` | `aws-config` | `aws-sdk-eventbridge` |
| `aws_iam` | `aws-sdk-iam`, `aws-config` | - |
| `aws_s3` | `aws-sdk-s3`, `aws-config`, `aws-smithy-types` | - |
| `aws_secrets` | `aws-sdk-secretsmanager`, `aws-config` | - |
| `aws_ses` | `aws-sdk-sesv2`, `aws-config` | - |
| `aws_sqs` | `aws-sdk-sqs`, `aws-config` | - |
| `aws_sts` | `aws-sdk-sts`, `aws-config`, `aws-credential-types` | - |
| `blob` | `opendal`, `object_store_opendal`, `bytes` | - |
| `cache` | `fred` (Redis) | `moka`, `cached` |
| `calendar` | `calendar-link`, `google-calendar3`, `chrono` | `icalendar` |
| `color` | `csscolorparser`, `hex_color` | - |
| `compress` | - | `flate2`, `zstd`, `lz4`, `brotli` |
| `crypto` | `aws-sdk-kms`, `base64` | `ring`, `rust-argon2`, `bcrypt`, `sha2`, `aes-gcm` |
| `csv` | - | `csv`, `polars` |
| `date` | `chrono`, `time` | `chrono-tz` |
| `email` | `email_address`, `emval` | `lettre`, `mail-parser` |
| `event` | `eventix` | `tokio-broadcast` |
| `feature_flag` | - | `flagsmith`, `unleash-api-client` |
| `file` | `infer`, `opendal` | `notify`, `walkdir` |
| `form` | `askama`, `minijinja` | `validator` |
| `geoip` | - | `maxminddb`, `geoip2` |
| `geolocation` | `geo-uri`, `latlon`, `geoutils` | `geo`, `geojson` |
| `github` | - | `octocrab`, `hubcaps` |
| `google_gemini` | - | `google-generative-ai-rs`, `genai` |
| `google_pubsub` | - | `google-cloud-pubsub` |
| `group` | - | - |
| `healthcheck` | - | `health-check` |
| `html` | `loki_text`, `askama` | `scraper`, `select`, `ammonia` |
| `http_client` | `ureq`, `http` | `reqwest`, `hyper` |
| `http_response` | `axum`, `http`, `bytes` | - |
| `identity` | `jsonwebtoken`, `webauthn-rs` | `oauth2`, `openidconnect` |
| `image` | - | `image`, `imageproc`, `fast_image_resize` |
| `job` | `apalis`, `croner` | `cron` |
| `json` | `serde_json`, `simd-json-derive` | `jsonpath-rust`, `jsonschema` |
| `log` | `tracing`, `tracing-subscriber`, `sentry` | `log` |
| `markdown` | - | `pulldown-cmark`, `comrak`, `markdown` |
| `math` | `cpc` | `num`, `nalgebra`, `rust_decimal` |
| `memgraph` | `rsmgclient` | - |
| `metric` | `opentelemetry`, `aws-sdk-cloudwatch` | `metrics`, `prometheus` |
| `microsoft_excel` | - | `calamine`, `xlsxwriter`, `rust_xlsxwriter` |
| `microsoft_teams` | - | `ms-graph-rs` |
| `mongodb` | `mongodb`, `bson` | - |
| `passkey` | `webauthn-rs` | `passkey` |
| `pdf` | `krilla` | `lopdf`, `printpdf`, `pdf-extract` |
| `policy` | - | `casbin`, `opa` |
| `privacy` | - | `data-encoding`, `subtle` |
| `push` | `a2` (APNs), `fcm-service`, `web-push-native` | - |
| `queue` | `aws-sdk-sqs`, `fred`, `asynk-strim` | `lapin` (RabbitMQ) |
| `random` | - | `rand`, `uuid` |
| `role` | - | `casbin` |
| `route` | `axum`, `datastar` | - |
| `schedule` | `croner`, `apalis` | `tokio-cron-scheduler` |
| `search` | - | `meilisearch-sdk`, `tantivy`, `elasticsearch` |
| `secret` | `secret-vault`, `aws-sdk-secretsmanager` | `secrecy` |
| `session` | `fred` | `tower-sessions` |
| `slack` | - | `slack-morphism`, `slack-api` |
| `sql` | `sqlx` | `sea-orm`, `diesel` |
| `stripe` | `async-stripe` | - |
| `successfactors` | - | (Custom HTTP client needed) |
| `text` | `stringzilla`, `textwrap`, `language-tags` | `unicode-segmentation`, `regex` |
| `transform` | `serde`, `serde_json`, `serde_yaml` | `jaq` (jq), `jsonpath` |
| `url` | `url` | - |
| `user` | - | - |
| `uuid` | `uuid` | `ulid`, `nanoid` |
| `validation` | `email_address`, `emval`, `phonelib`, `semver` | `validator`, `garde` |
| `webhook` | `axum`, `http` | `svix`, `hmac` |
| `workflow` | `apalis` | `temporal-sdk`, `saga` |
| `xml` | - | `quick-xml`, `serde-xml-rs`, `roxmltree` |
| `zoom` | - | (Custom HTTP client needed) |

## Missing Libraries Summary

The following libraries are recommended additions to `Cargo.toml`:

### Archive/Compression
```toml
zip = "2"
tar = "0.4"
flate2 = "1"
zstd = "0.13"
```

### AI/ML
```toml
async-openai = "0.27"
ollama-rs = "0.3"
```

### Text Processing
```toml
pulldown-cmark = "0.12"
regex = "1"
```

### Image Processing
```toml
image = "0.25"
fast_image_resize = "5"
```

### Data Formats
```toml
csv = "1"
quick-xml = "0.37"
calamine = "0.27"
rust_xlsxwriter = "0.85"
```

### Search
```toml
meilisearch-sdk = "0.28"
tantivy = "0.22"
```

### Random/IDs
```toml
rand = "0.8"
nanoid = "0.4"
```

### HTTP Client
```toml
reqwest = { version = "0.12", features = ["json"] }
```

### Validation
```toml
validator = { version = "0.19", features = ["derive"] }
garde = { version = "0.22", features = ["derive"] }
```

### Geo/IP
```toml
maxminddb = "0.24"
geo = "0.29"
```

### GitHub
```toml
octocrab = "0.41"
```

### Slack
```toml
slack-morphism = "2"
```

### Messaging
```toml
lapin = "2"  # RabbitMQ
google-cloud-pubsub = "0.30"
aws-sdk-eventbridge = "1"
```
