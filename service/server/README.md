# Harana HTTP Server

A dual-target HTTP server built with [Axum](https://github.com/tokio-rs/axum) that supports both standalone (native binary) and [Cloudflare Workers](https://workers.cloudflare.com/) deployment.

## Architecture

The server uses Cargo features to conditionally compile for different targets:

- **`standalone`** (default) — Full-featured native binary with tokio, tracing, SSE events, sysinfo health checks, reqwest-based OAuth, and passkey support.
- **`cloudflare`** — Lightweight WASM module for Cloudflare Workers using `workers-rs`. Shares the same Axum router, auth, and core route handlers.

### Shared Code

The following modules work identically on both targets:
- `routes.rs` — Route definitions (event routes are no-op on Cloudflare)
- `handlers/auth.rs` — Authentication endpoints (JWT, OAuth, passkey)
- `handlers/general.rs` — Index, health, about, protected endpoints
- `auth/jwt.rs` — JWT token management
- `auth/oauth.rs` — OAuth 2.0 flows (external provider calls stubbed on CF)
- `auth/session.rs` — In-memory session management
- `config.rs` — Server configuration
- `extractors.rs` — Axum request extractors
- `middleware.rs` — Request middleware
- `error.rs` — Error types

### Standalone Only

- `handlers/event.rs` — Datastar SSE real-time event streaming
- Detailed system health checks (CPU, memory, disk via `sysinfo`)
- Full passkey/WebAuthn support via `harana-actions-passkey`
- External OAuth provider HTTP calls via `reqwest`

## Usage

### Standalone (native binary)

```bash
# Run with default features (standalone)
cargo run --bin harana-server

# Or explicitly
cargo run --bin harana-server --features standalone
```

### Cloudflare Workers

```bash
cd service/server

# Install dependencies
npm install

# Local development
npm run dev

# Deploy to Cloudflare
npm run deploy
```

The Cloudflare Workers build uses `worker-build` to compile the crate to WASM and bundle it with the Workers runtime. Configuration is in `wrangler.toml`.

## Features

| Feature | Standalone | Cloudflare Workers |
|---------|-----------|-------------------|
| Axum routing | ✅ | ✅ |
| JWT auth | ✅ | ✅ |
| OAuth 2.0 | ✅ | ✅ (local flows) |
| External OAuth | ✅ | ⏳ (use `worker::Fetch`) |
| Passkey/WebAuthn | ✅ | ⏳ |
| SSE events | ✅ | ❌ |
| System health | ✅ | ✅ (lightweight) |
| Session mgmt | ✅ | ✅ |
| Compression | ✅ | ❌ (handled by CF) |
| CORS | ✅ | ❌ (handled by CF) |
| Tracing | ✅ | ✅ (`tracing` crate) |
