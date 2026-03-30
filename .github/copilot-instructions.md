# Project Guidelines

## Build And Test
- Preferred on Windows: use `make.bat` from workspace root.
- Initial setup: `./make.bat setup`.
- Full local dev (backend + frontend): `./make.bat dev`.
- Run backend tests: `./make.bat test` (or `cargo test`).
- Rust lint/format: `./make.bat lint`, `./make.bat fmt`.
- Production build: `./make.bat build` (builds web first, then Rust).
- Frontend-only checks live in `web/`: `npm run check`, `npm run lint`, `npm run build`.
- Docs site commands live in `docs/`: `npm run docs:dev`, `npm run docs:build`.

## Architecture
- Rust workspace root is `Cargo.toml`; crates are under `crates/`.
- Main app crate: `crates/bili_sync`.
- Database models: `crates/bili_sync_entity`.
- Database migrations: `crates/bili_sync_migration`.
- Backend entry binary: `crates/bili_sync/src/main.rs` (`bili-sync-rs`).
- Frontend app (SvelteKit + Vite) lives in `web/` and compiles to static assets.
- Project docs (VitePress) live in `docs/`.

## Conventions
- Keep shared Rust dependencies in workspace-level `Cargo.toml` under `[workspace.dependencies]` when possible.
- Do not reintroduce file-based runtime config flow; configuration has been migrated to database-backed flow.
- Use `tracing` macros for logs in Rust code; avoid `println!` for runtime logging.
- For database schema changes, add a new migration in `crates/bili_sync_migration/src/` and register it in `crates/bili_sync_migration/src/lib.rs`.
- For API additions/changes, keep OpenAPI registration in sync under `crates/bili_sync/src/api/` (utoipa-based docs).
- Follow existing module boundaries in `crates/bili_sync/src/` (api/auth/bilibili/task/workflow/utils) rather than adding cross-cutting logic in `main.rs`.
- Treat `target/`, `web/build/`, and other build outputs as generated artifacts; do not manually edit generated files.

## Pitfalls
- Startup includes path normalization/migration logic (for example upper-face bucket normalization). Preserve migration safety when changing path logic.
- `make.bat` orchestrates required frontend pre-steps (including Svelte sync/meta generation paths); bypassing it can cause mismatched local build state.

## Docs To Link (Do Not Duplicate)
- Product usage and behavior: `docs/usage.md`.
- Quick onboarding: `docs/quick-start.md` and `docs/installation.md`.
- Feature overview: `docs/features.md`.
- Queue behavior and tuning: `docs/queue-management.md` and `docs/SYSTEM_CONFIG_QUEUE_SUMMARY.md`.
- CLI/runtime options: `docs/args.md`.
- Troubleshooting: `docs/faq.md`.
- Release history: `docs/changelog.md`.