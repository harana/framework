# Actions Implementation Plan

This document tracks the implementation and testing status of all action modules.

**Total Modules:** 94  
**Total Functions to Implement:** 1,137  
**Last Updated:** January 27, 2026

**Implementation Status:**
- Core Utilities: 22/22 ✅
- Data Processing: 3/10
- Platform Services: 16/19
- Security & Auth: 7/8
- Workflow & Events: 3/5 ✅ (schedule, event, workflow)
- External Services: 0/12
- AWS Services: 0/18

---

## Recommended Crates

The following crates have been researched and selected based on:
- ✅ Recent updates (within last 3 months)
- ✅ High download counts (community trust)
- ✅ | `markdown` | 3 | ✅ | ✅ | `pulldown-cmark` (3.5M/mo) - CommonMark |ctive maintenance
- ✅ Good documentation
- ✅ Async-first where applicable

### Foundation (Required by most modules)
| Crate | Downloads/Month | Notes |
|-------|-----------------|-------|
| `serde` | 33.9M | Serialization framework - used everywhere |
| `tokio` | 25.9M | Async runtime - foundation for all async code |
| `anyhow` | 24.9M | Error handling - use for application code |
| `thiserror` | 42.9M | Error handling - use for library error types |
| `tracing` | 24M | Structured logging - #1 in Debugging |
| `bytes` | 27.5M | Binary data handling - required by many crates |
| `futures` | 20.6M | Async utilities and combinators |
| `async-trait` | 17.4M | Async methods in traits |
| `dashmap` | 11.1M | Concurrent HashMap for shared state |
| `parking_lot` | 29.6M | Better mutexes than std |
| `once_cell` | 30.4M | Lazy statics (now partly in std) |
| `itertools` | 41.8M | Extra iterator methods |
| `strum` | 19.9M | Enum-string conversions |
| `derive_more` | 14.4M | Derive macros for common traits |
| `derive_builder` | 6.7M | Builder pattern derive |
| `bitflags` | 48.2M | Bitflag types |
| `indexmap` | 42.1M | Ordered HashMap |
| `smallvec` | 30.4M | Stack-allocated vectors |
| `ahash` | 19.7M | Fast non-crypto hashing |
| `arc-swap` | 8.4M | Atomically swappable Arc |
| `scopeguard` | 24.4M | RAII scope guards |
| `dyn-clone` | 9.5M | Clone for trait objects |
| `nix` | 21M | Rust-friendly bindings to *nix APIs |
| `tinyvec` | 19.7M | 100% safe vec-like data structures |
| `num_cpus` | 19.4M | Get number of CPUs on a machine |
| `spin` | 19.6M | Spin-based synchronization primitives |
| `thread_local` | 18.5M | Per-object thread-local storage |
| `zerocopy` | 19.7M | Zero-cost memory manipulation |
| `bumpalo` | 14.2M | Fast bump allocation arena |
| `petgraph` | 13.7M | Graph data structure library |
| `static_assertions` | 13.6M | Compile-time assertions |
| `portable-atomic` | 9.2M | Portable atomic types including 128-bit |
| `bytemuck` | 9.1M | Safe transmutes between types |
| `linked-hash-map` | 9.1M | HashMap with insertion order |
| `indoc` | 9.1M | Indented document literals |
| `educe` | 2.6M | Fast derive macros for built-in traits |
| `smol_str` | 2.6M | Small-string optimized O(1) clone |
| `ouroboros` | 2.6M | Safe self-referential struct generation |

### AWS Services
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| All AWS | `aws-sdk-*` | 2.4M+ (S3) | Official AWS SDK, actively maintained |
| Config | `aws-config` | 4.3M | AWS SDK configuration and credentials |

### Core Utilities
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| `uuid` | `uuid` | 19.1M | #1 in Value formatting, v4/v7 support |
| `random` | `rand` | 43.4M | #1 in Algorithms |
| `random` | `rand_distr` | 4.2M | Sampling from distributions |
| `random` | `rand_xoshiro` | 3.2M | Fast xoshiro/splitmix64 RNGs |
| `random` | `nanorand` | 2.6M | Tiny, fast, zero-dep RNG |
| `date` | `chrono` | 21.4M | #1 in Date/time |
| `date` | `jiff` | 7.2M | Modern alternative to chrono, Temporal-inspired |
| `json` | `serde_json` | 33.9M | #1 in Encoding |
| `json` | `jsonpath-rust` | 2.6M | JSONPath query library |
| `text` | `regex` | 29.0M | #1 in Text processing |
| `text` | `regex-lite` | 4.1M | Lightweight regex, optimized binary size |
| `text` | `fancy-regex` | 5.3M | Regex with backreferences/look-around |
| `text` | `heck` | 35.5M | Case conversion (snake_case, camelCase, etc.) |
| `text` | `convert_case` | 13.2M | Convert strings into any case |
| `text` | `Inflector` | 3.9M | String inflections (pluralize, ordinalize) |
| `url` | `url` | 23.2M | Standard URL parsing |
| `url` | `form_urlencoded` | 20.8M | Form URL encoding parser/serializer |
| `validation` | `validator` | 1M+ | Derive-based validation |
| `log` | `tracing` | 24M | #1 in Debugging, structured logging |
| `log` | `tracing-subscriber` | 16.3M | Log formatting and output |
| `log` | `tracing-log` | 14.2M | tracing + log crate compatibility |
| `log` | `tracing-futures` | 5.1M | Instrument futures with tracing |
| `log` | `tracing-appender` | 2.6M | File appenders and non-blocking writers |
| `log` | `env_logger` | 18.3M | Environment-configured logging |
| `math` | `num` | 7.6M | Numeric traits and bigint |
| `math` | `num-integer` | 19.7M | Integer traits and functions |
| `math` | `rust_decimal` | 4.5M | Financial/fixed-precision calculations |
| `math` | `nalgebra` | 2.6M | Linear algebra library |
| `color` | `palette` | 300K+ | Color space handling |
| `color` | `color_quant` | 3.4M | Color quantization (reduce to 256 colors) |
| `csv` | `csv` | 5.8M | Standard CSV handling |
| `xml` | `quick-xml` | 12.6M | #1 in Parser implementations |
| `xml` | `xmlparser` | 3.2M | Pull-based, zero-allocation XML parser |
| `html` | `scraper` | 666K | HTML parsing with CSS selectors |
| `markdown` | `pulldown-cmark` | 3.5M | CommonMark parser |
| `image` | `image` | 4.4M | #3 in Images |
| `image` | `gif` | 3.4M | GIF encoder/decoder |
| `pdf` | `lopdf` | 322K | Read/write/manipulate PDF |
| `compress` | `flate2` (gzip), `zstd` | 18.3M, 10.6M | #1 and #7 in Compression |
| `compress` | `brotli` | 6.9M | Brotli compression (web) |
| `compress` | `lz4_flex` | 3.4M | Fastest LZ4 in Rust, safe by default |
| `compress` | `snap` | 3.4M | Snappy compression with streaming |
| `archive` | `zip`, `tar` | 7.9M, 5.9M | Archive handling |
| `archive` | `xattr` | 6.7M | Unix extended filesystem attributes |
| `excel` | `calamine` (read), `rust_xlsxwriter` (write) | 301K, 93K | xlsx/xls/ods support |
| `crypto` | `sha2`, `sha1`, `sha3` | 23.6M, 12.7M, 3.9M | SHA hash family |
| `crypto` | `blake2` | 4.3M | BLAKE2 hash functions |
| `crypto` | `argon2`, `pbkdf2` | 1.1M, 6.8M | Password hashing |
| `crypto` | `ring` | 19.3M | Fast cryptography |
| `crypto` | `aes`, `aes-gcm` | 7.5M, 3.4M | AES encryption |
| `crypto` | `rsa` | 6M | RSA encryption |
| `crypto` | `hmac` | 12.4M | HMAC message authentication |
| `crypto` | `blake3` | 5.5M | Fast hash function |
| `crypto` | `elliptic-curve` | 5.4M | General ECC support |
| `crypto` | `ecdsa` | 5.3M | ECDSA signing/verification |
| `crypto` | `p256` | 4.0M | NIST P-256 curve (ECDH/ECDSA) |
| `crypto` | `pkcs8` | 9.6M | PKCS#8 private key encoding |
| `crypto` | `pkcs1` | 5.4M | PKCS#1 RSA encoding |
| `crypto` | `crypto-bigint` | 6.8M | Constant-time big integers for crypto |
| `crypto` | `aead` | 5.3M | AEAD algorithm traits (AES-GCM, ChaCha20Poly1305) |
| `crypto` | `ctr` | 5.3M | CTR block cipher modes |
| `crypto` | `keccak` | 4.1M | Keccak sponge function |
| `encoding` | `base64` | 41.3M | Base64 encode/decode |
| `encoding` | `base64ct` | 9.1M | Constant-time Base64 (crypto-safe) |
| `encoding` | `hex` | 16M | Hex encode/decode |
| `encoding` | `base16ct` | 5.4M | Constant-time hex (crypto-safe) |
| `text` | `nom` | 19.9M | Parser combinators |
| `text` | `winnow` | 19.8M | Zero-copy parser combinators |
| `text` | `combine` | 6.7M | Fast parser combinators |
| `text` | `unicode-segmentation` | 14.6M | Unicode text handling |
| `text` | `unicode-normalization` | 18.1M | Unicode normalization (NFC, NFD, etc.) |
| `file` | `walkdir` | 14.3M | Directory walking |
| `file` | `tempfile` | 21.3M | Temporary files |
| `file` | `notify` | 4.2M | Filesystem watching |
| `file` | `glob` | 16.1M | File pattern matching |
| `file` | `globset` | 6.6M | Multiple glob pattern matching |
| `file` | `memmap2` | 9.1M | Memory-mapped files |
| `file` | `fs_extra` | 8.4M | Extended fs operations (recursive copy) |
| `file` | `fs-err` | 3.4M | Drop-in fs replacement with better errors |
| `file` | `ignore` | 4.9M | .gitignore pattern matching |
| `file` | `dirs` / `directories` | 7.8M / 1.6M | Platform-specific directories |
| `file` | `camino` | 6.6M | UTF-8 paths |
| `file` | `pathdiff` | 5.0M | Diffing paths to get relative paths |
| `text` | `textwrap` | 9.3M | Word wrapping |
| `text` | `similar` | 5M | Diff algorithm |
| `text` | `difflib` | 4.2M | Python difflib port |
| `url` | `urlencoding` | 7.1M | URL percent encoding |
| `url` | `percent-encoding` | 23.1M | URL encoding (low-level) |
| `encoding` | `encoding_rs` | 12.1M | Character encoding conversion |
| `encoding` | `data-encoding` | 9.8M | Base32/Base64/Hex encoding |
| `http` | `mime` | 14.3M | MIME types |
| `http` | `mime_guess` | 6.2M | MIME type detection |
| `http` | `http-body` | 22.9M | HTTP body trait |
| `http` | `http-body-util` | 9.2M | HTTP body utilities |
| `http` | `hyper-util` | 9.6M | Hyper utilities |
| `http` | `httparse` | 19.2M | Tiny, safe, zero-copy HTTP/1.x parser |
| `http` | `headers` | 5.0M | Typed HTTP headers |
| `http` | `serde_urlencoded` | 12.9M | URL-encoded form data |
| `http` | `multer` | 3.2M | Async multipart/form-data parser |
| `time` | `humantime` | 14.6M | Human-readable durations |
| `time` | `chrono-tz` | 4.1M | IANA timezone database |
| `async` | `tokio-util` | 21.1M | Tokio utilities |
| `async` | `tokio-stream` | 12.3M | Tokio streams |
| `async` | `async-stream` | 9.1M | Async stream macros |
| `async` | `async-channel` | 9.3M | Async MPMC channels |
| `async` | `async-io` | 6.8M | Async I/O and timers |
| `async` | `async-recursion` | 4.2M | Recursion for async functions |
| `async` | `flume` | 7.4M | Fast MPMC channel |
| `async` | `futures-lite` | 9.4M | Lightweight futures |
| `async` | `blocking` | 5.3M | Thread pool for blocking I/O in async |
| `signal` | `signal-hook` | 7.1M | Unix signal handling |
| `signal` | `ctrlc` | 3.2M | Ctrl-C handler |
| `checksum` | `crc` | 6.6M | CRC with various standard support |
| `template` | `tinytemplate` | 6.6M | Simple, lightweight template engine |
| `yaml` | `unsafe-libyaml` | 7.0M | Fast YAML via transpiled libyaml |
| `retry` | `backoff` | 3.4M | Retry with exponential backoff |

### Data Processing
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| `sql` | `sqlx` | 3.3M | Compile-time checked queries |
| `sql` | `rusqlite` | 2M | SQLite (embedded database) |
| `mongodb` | `mongodb` | 351K | Official MongoDB driver |
| `memgraph` | `neo4rs` | N/A | Graph database support |
| `cache` | `redis` | 2.6M | Redis driver for Rust |
| `cache` | `lru` | 10M | In-memory LRU cache |
| `cache` | `moka` | 2.7M | Fast concurrent cache (Java Caffeine inspired) |
| `cache` | `deadpool` | 3.2M | Dead simple async connection pool |
| `search` | `tantivy` / `meilisearch-sdk` | Various | Full-text search |
| `http_client` | `reqwest` | 18M | #2 in HTTP client |
| `http_client` | `hyper` | 24.9M | Low-level HTTP (used by reqwest) |
| `http` | `http` | 28.4M | HTTP types and utilities |
| `http` | `tower` | 19M | HTTP middleware framework |
| `http` | `tower-http` | 13.4M | HTTP-specific Tower middleware |
| `http` | `axum` | 13.9M | Web framework (if needed) |
| `websocket` | `tokio-tungstenite` | 6.6M | WebSocket client/server |
| `grpc` | `tonic` | 10.6M | gRPC implementation |
| `protobuf` | `prost` | 14.7M | Protocol Buffers |
| `protobuf` | `prost-build` | 9.7M | Generate Prost types from .proto |
| `serialization` | `bincode` | 9.3M | Binary serialization |
| `serialization` | `rmp` | 3.9M | Pure Rust MessagePack |
| `serialization` | `rmp-serde` | 3.8M | MessagePack serialization |
| `serialization` | `toml` | 21.7M | TOML config files |
| `serialization` | `toml_edit` | 20.7M | Format-preserving TOML parser |
| `serialization` | `schemars` | 16.6M | JSON Schema generation |
| `serialization` | `serde_with` | 9.7M | Custom serde helpers |
| `serialization` | `serde_bytes` | 4.9M | Optimized byte handling |
| `serialization` | `ciborium` | 6.5M | CBOR serialization |
| `math` | `ndarray` | 5.3M | N-dimensional arrays |
| `math` | `ordered-float` | 13.9M | Total ordering for floats |
| `math` | `approx` | 4.2M | Approximate float comparisons |
| `checksum` | `crc32fast` | 18.6M | Fast CRC32 |
| `interop` | `pyo3` | 6.6M | Python bindings |
| `interop` | `wasm-bindgen` | 13.1M | WebAssembly JS interop |
| `interop` | `jni` | 4.3M | Java Native Interface bindings |
| `interop` | `cxx` | 2.6M | Safe Rust/C++ interop |

### External Services
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| `github` | `octocrab` | 1M | #3 in HTTP client |
| `slack` | `slack-morphism` | 90K | Modern Slack API client |
| `stripe` | `async-stripe` | 126K | #2 in Finance |
| `docker` | `bollard` | 1.6M | #1 in Docker |
| `email` | `lettre` | 384K | #1 in Email |
| `git` | `git2` | 3.4M | libgit2 bindings |
| `google_gemini` | `google-generative-ai-rs` | N/A | Google AI API |
| `google_pubsub` | `google-cloud-pubsub` | N/A | Google Pub/Sub |

### Security & Auth
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| `passkey` | `webauthn-rs` | 111K | #18 in Auth, WebAuthn/FIDO2 |
| `sign` (JWT) | `jsonwebtoken` | 5.2M | #1 in Authentication |
| `secret` | `secrecy` | 4.0M | Secret management, zeroizing |
| `secret` | `zeroize` | 21.3M | Secure memory wiping |
| `secret` | `password-hash` | 2.7M | Password hashing traits (PHC format) |
| `tls` | `rustls` | 27.6M | Modern TLS library |
| `tls` | `aws-lc-rs` | 3.3M | AWS-LC crypto (ring-compatible) |
| `tls` | `tokio-rustls` | 19.6M | Async TLS with Tokio |
| `tls` | `rustls-pemfile` | 16.8M | PEM file parsing |
| `tls` | `rustls-webpki` | 18.1M | Web PKI certificate verification |
| `tls` | `rustls-native-certs` | 13.1M | Platform native certificate store |
| `tls` | `webpki-roots` | 22.4M | Mozilla CA certificates |
| `crypto` | `ed25519-dalek` | 4.2M | EdDSA signatures |
| `crypto` | `x509-parser` | 3.9M | X.509 certificate parsing |
| `crypto` | `der-parser` | 3.9M | ASN.1 BER/DER parsing |
| `crypto` | `hkdf` | 6.3M | Key derivation |
| `crypto` | `subtle` | 20.2M | Constant-time operations |
| `crypto` | `rfc6979` | 5.0M | Deterministic DSA/ECDSA |

### Platform Services
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| `calendar` | `icalendar` | 12K | iCalendar RFC 5545 |
| `schedule` | `cron` | 446K | Cron expression parsing |
| `queue` | `lapin` | Various | RabbitMQ client |
| `metric` | `metrics` | 4.6M | Application metrics facade |
| `metric` | `metrics-util` | 2.7M | Helper types for metrics ecosystem |
| `metric` | `prometheus` | 5.5M | Prometheus metrics |
| `metric` | `opentelemetry` | 7.2M | OpenTelemetry API |
| `metric` | `opentelemetry_sdk` | 6.4M | OpenTelemetry SDK |
| `metric` | `opentelemetry-otlp` | 4.4M | OTLP exporter |
| `metric` | `opentelemetry-semantic-conventions` | 4.0M | OTel semantic conventions |
| `metric` | `tracing-opentelemetry` | 5.7M | Tracing + OTel integration |
| `metric` | `quanta` | 4.3M | High-speed timing library |
| `parallel` | `rayon` | 12.4M | Data parallelism |
| `parallel` | `crossbeam` | 3.9M | Concurrent programming tools |
| `parallel` | `crossbeam-channel` | 15.3M | MPMC channels |
| `parallel` | `threadpool` | 4.2M | Simple thread pool |
| `config` | `config` | 3.3M | Layered configuration |
| `config` | `dotenvy` | 4.8M | .env file loading |
| `system` | `sysinfo` | 5.1M | System information |
| `system` | `hostname` | 5.2M | Host name operations |
| `system` | `which` | 12.8M | Find executables in PATH |
| `system` | `cargo_metadata` | 6.6M | Structured access to cargo metadata |
| `parser` | `pest` | 9.2M | PEG parser generator |
| `parser` | `semver` | 25.1M | Semantic versioning |
| `allocator` | `tikv-jemallocator` | 2.6M | jemalloc allocator |
| `inotify` | `inotify` | 3.9M | Linux filesystem event watching |

### Testing
| Purpose | Crate | Downloads/Month | Notes |
|---------|-------|-----------------|-------|
| HTTP Mocking | `wiremock` | 2.1M | #6 in Testing |
| General Mocking | `mockall` | 4.3M | Mock generation |
| Snapshot Testing | `insta` | 2.8M | Snapshot/approval testing |
| Property Testing | `proptest` | 5.4M | Hypothesis-style testing |
| Test Fixtures | `rstest` | 4.6M | Fixture-based testing |
| Async Testing | `tokio-test` | 3.9M | Tokio test utilities |
| Pretty Assertions | `pretty_assertions` | 5.5M | Colorful diffs in tests |
| CLI Testing | `assert_cmd` | 2.5M | Test CLI applications |
| Predicates | `predicates` | 5.4M | Boolean predicate functions |
| Serial Tests | `serial_test` | 4.6M | Sequential test execution |
| Fuzzing | `arbitrary` | 4.6M | Structured data from random |
| Benchmarking | `criterion` | 9.3M | Statistical benchmarking |
| Forking | `rusty-fork` | 4.0M | Cross-platform test sub-processes |
| Normalization | `normalize-line-endings` | 3.3M | Normalize line endings for tests |

### CLI & Terminal (for tooling)
| Purpose | Crate | Downloads/Month | Notes |
|---------|-------|-----------------|-------|
| Argument Parsing | `clap` | 29.5M | #1 CLI argument parser |
| Progress Bars | `indicatif` | 6.9M | Progress indicators |
| Colors | `colored` | 6M | Terminal colors |
| Colors | `owo-colors` | 4.9M | Zero-alloc colors |
| Colors | `nu-ansi-term` | 12.7M | ANSI terminal colors and styles |
| Colors | `yansi` | 6.8M | Dead simple ANSI color painting |
| Colors | `colorchoice` | 14.8M | Global color control override |
| Terminal UI | `ratatui` | 1.6M | Terminal UI framework |
| Terminal UI | `crossterm` | 6.8M | Cross-platform terminal |
| Prompts | `dialoguer` | 2.4M | Interactive prompts |
| Console | `console` | 10.7M | Terminal abstraction |
| Tree Viz | `termtree` | 5.4M | Visualize tree-like data in terminal |
| Diagnostics | `codespan-reporting` | 4.2M | Beautiful diagnostic reporting |

### External Services (Additional)
| Module | Crate | Downloads/Month | Notes |
|--------|-------|-----------------|-------|
| `python` | `pyo3` | 6.6M | Python interop |
| `wasm` | `wasm-bindgen` | 13.1M | WebAssembly JS interop |
| `actix` | `actix-web` | 2.6M | Actix Web framework |
| `actix` | `actix-server` | 2.7M | General purpose TCP server |
| `actix` | `actix-service` | 2.7M | Service trait for request/response |