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

---

# Fork 改动说明与 Merge 注意事项

> 以下内容记录本 fork（[Deforb/bili-sync-up](https://github.com/Deforb/bili-sync-up)）相对 upstream（[NeeYoonc/bili-sync-up](https://github.com/NeeYoonc/bili-sync-up)）的实质性改动，以及合并 upstream 变更时的高风险点与验证步骤。

### 1. 目录模式从二值升级为三态（核心改动）

- 新增 `folder_mode` 枚举（0=normal, 1=flat, 2=weak_flat）及配套工具函数
  - `crates/bili_sync/src/utils/folder_mode.rs`
- 适配器层新增 `folder_mode()` / `weak_flat_folder()` 抽象，`flat_folder()` 退化为兼容语义
  - `crates/bili_sync/src/adapter/mod.rs`
- API 请求/响应增加 `folder_mode` 字段，保留 `flat_folder` 兼容旧客户端
  - `crates/bili_sync/src/api/request.rs`
  - `crates/bili_sync/src/api/response.rs`
- 前端新增源页面改为目录模式选择（normal / flat / weak_flat），而非单一平铺开关
  - `web/src/routes/add-source/+page.svelte`
  - `web/src/routes/video-sources/+page.svelte`
- 前端类型与 API 客户端增加 `FolderMode` 类型
  - `web/src/lib/api.ts`
  - `web/src/lib/types.ts`

### 2. 数据库新增 `folder_mode` 列并迁移旧数据

- 新 migration：为 `collection` / `favorite` / `submission` / `watch_later` / `video_source` 增加 `folder_mode` 列，并用 `flat_folder` 回填
  - `crates/bili_sync_migration/src/m20260327_000001_add_folder_mode.rs`
- 对应实体字段补齐
  - `crates/bili_sync_entity/src/entities/video_source.rs`
  - `crates/bili_sync_entity/src/entities/collection.rs`
  - `crates/bili_sync_entity/src/entities/favorite.rs`
  - `crates/bili_sync_entity/src/entities/submission.rs`
  - `crates/bili_sync_entity/src/entities/watch_later.rs`

### 3. 视频源刷新流程集成"已删视频扫描+标记+本地清理"

- 刷新时记录 `seen_bvids`，缺失视频自动标记 `deleted=1`，并触发本地文件清理计划
  - `crates/bili_sync/src/workflow.rs`
- watch_later 页面新增"一次性触发清理"按钮
  - `web/src/routes/video-sources/+page.svelte`

### 4. 路径迁移/目录重命名安全性增强

- 切换 `folder_mode` 后自动触发历史文件迁移（`reset_video_source_path_internal`）
- 四步重命名新增边界保护（禁止目标位于源子目录内）
- 重命名步骤 2/3 失败时回滚恢复原路径
- 文件移动失败时跳过该视频的数据库路径重算，避免文件与 DB 状态错位
  - `crates/bili_sync/src/api/handler.rs`

### 5. 其他修复

- 弹幕刷新过滤已删除视频（`deleted=1` 不参与）
  - `crates/bili_sync/src/workflow_danmaku.rs`
- 关键词过滤弹窗与新增源页输入校验/交互细节优化
  - `web/src/lib/components/keyword-filter-dialog.svelte`
  - `web/src/routes/add-source/+page.svelte`

## Merge 注意事项

### 最高风险：数据库 migration 冲突

- **冲突文件**：`crates/bili_sync_migration/src/lib.rs`
- **原因**：upstream 若新增同时间段 migration，模块注册顺序和命名很容易冲突。
- **处理原则**：
  - 保持 migration 时间戳递增，不要打乱已有顺序
  - 确保 `m20260327_000001_add_folder_mode` 只执行一次
  - 合并后运行 `cargo test` 验证 migration 可正常 up/down

### 高风险：API 请求/响应契约冲突

- **冲突文件**：
  - `crates/bili_sync/src/api/request.rs`
  - `crates/bili_sync/src/api/response.rs`
  - `web/src/lib/api.ts`
  - `web/src/lib/types.ts`
- **处理原则**：
  - `folder_mode` 作为主字段，`flat_folder` 只做兼容映射，不要让两个字段语义打架
  - 前端 `folder_mode` 优先级高于 `flat_folder`

### 中风险：刷新流程语义冲突

- **冲突文件**：`crates/bili_sync/src/workflow.rs`
- **原因**：本 fork 在刷新中嵌入了"缺失即标记删除+清理"逻辑，upstream 若在同区域改了分页/增量逻辑，容易产生行为回归。
- **处理原则**：
  - 保留 `seen_bvids` 为空时跳过删除的保护（避免接口异常时误删全库）
  - 验证 `scan_deleted_videos()` 开关与上游新增配置不冲突

### 中风险：路径迁移与清理边界

- **冲突文件**：`crates/bili_sync/src/api/handler.rs`
- **处理原则**：
  - 保留"目标是源子目录时拒绝迁移""失败回滚""移动失败不改 DB"三条安全线
  - 清理范围不要扩大到整个根目录全树扫描（参考 `folder_cleanup_boundary.md`）

### 低风险：前端交互与后端开关一致性

- **冲突文件**：
  - `web/src/routes/video-sources/+page.svelte`
  - `web/src/routes/add-source/+page.svelte`
- **处理原则**：目录模式循环切换、一次性扫描开关要与后端字段完全一致，避免 UI 显示已切换但后端未生效
