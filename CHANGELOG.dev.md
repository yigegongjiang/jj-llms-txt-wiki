```When Editing
本文档作用: 面向开发者的发版记录; CHANGELOG.md 的超集, 1:1 镜像 + 技术变更子项
遵循 AGENTS.md 文档编写规范
- 每条主项 = CHANGELOG.md 对应条目 (原文), 下方缩进子项承载技术变更
- 子项 MAY 写路径 / 函数 / 机制; ≤ 1 行
```

# Changelog (developer, follow [CHANGELOG.md](./CHANGELOG.md))

## [0.12.1] - 2026-07-12

### Added

- 每次 `sync` 结束固定打印一行末行总账：全部成功显示 `✓ N/N synced`，存在失败显示 `✗ N ok · M failed · K error` 并附日志路径、以非零状态退出，使个别站点失败不再淹没在成功输出中。
  - 新增 `report::print_summary` / `render_summary`；`sync::run` 收尾在 `print_failures` 后无条件调用，`Failed`→failed、`Aborted`→error 桶，仅列非零桶；退出码语义不变。

## [0.12.0] - 2026-07-12

### Added

- 支持将 `llms-full.txt` 直接添加为站点入口；仍使用统一 `sync`，自动按内嵌 URL 拆分并保留文档目录层级。
  - `EntryKind` 按入口 path 自动分流；`full::crawl` 单次抓取、识别 H1 + 可选 blockquote + `URL:` 页头、拆分后复用统一报告 / 提交链路。

### Changed

- 聚合文件每次从空快照完整重建；格式损坏、URL 重复或路径冲突时保留上一份完整快照。
  - `Snapshot::fresh` 丢弃中断 partial；full 路径映射覆盖无后缀 / 目录 / root URL，失败时 discard staging，不替换旧目录。

## [0.11.0] - 2026-07-12

### Removed

- 精简 `sync` 参数：移除 `-v/--verbose` 与 `-q/--quiet`。默认已实时显示进度并逐条打印失败，管道 / 重定向下自动隐藏动态刷新，无需额外开关。
  - 删除 `progress::Verbosity` 枚举与 `SyncProgress.verbosity` 字段；`SyncProgress::new` 去 verbosity 参数，不再显式 `set_draw_target`，恒用 `new_spinner` 默认（draw 到 stderr，indicatif 在非 TTY 自动抑制动画）。`complete` 拆为「仅计数」+ 新 `fail`（失败恒红粗 `FAIL` scrollback），删除 `styled_tag`（其余色标签已无调用点）。`sync::run`/`sync_site` 去 verbosity 参数与三处 `!= Verbosity::Quiet` 判定（站点头 / 续传提示 / push warning 恒打印），`sync_site` 降到 7 参去掉 `#[allow(clippy::too_many_arguments)]`。`cli.rs` 删 `Sync.verbose/quiet` 字段及互斥解析测试。
- 移除 `version` 子命令，改用标准 `--version`（`-V`）；`llms-wiki version` 不再可用。
  - 删除 `Command::Version` 及 `main.rs` 对应分支（clap `#[command(version)]` 已提供 `--version`/`-V`，输出同为 `llms-wiki <version>`）。`lifecycle::read_version` 自更新读版本改调 `--version`。

## [0.10.0] - 2026-07-12

### Added

- 同步失败时给出可读的结果报告：按站点列出失败的文件与原因、404/410 缺失数量，并提示重试命令，不再把错误挤成一行。
  - 新增 `src/report.rs`：`Outcome`（`Ok`/`Failed(CrawlReport)`/`Aborted(String)`）+ `SiteReport`；`print_failures` 只在有失败时打印聚合块（红粗标题、每站失败逐条 `✗ path — reason`、终端上限 `TERMINAL_FAILURE_CAP=8` 超出提示「N more」、缺失 404/410 计数、末尾 `full log` 路径 + 每失败站 `retry` 命令），无视 verbosity 始终打印（`-q` 也不吞错误）。`src/sync.rs::run` 重构：拆出 `sync_site` 返回 `Outcome`，收集 `Vec<SiteReport>`；失败时 `run` 返回 `Err(String::new())` 保留 FAILURE 退出码但不让 `main.rs` 再叠一行 `error:`。
- 每次同步把完整结果（各站计数、全部失败原因、缺失链接）写入 `<输出目录>/.llms-wiki/last-run.log`，跑完仍可回看；该文件不进版本库。
  - `report::write_log` best-effort 覆盖写（`let _ =`，失败不影响同步；覆盖而非追加，避免无界增长）；`render_log` 输出纯文本（无 ANSI）全量记录。`crawler::CrawlReport` 新增 `missing_urls: Vec<String>`（`Missing` 分支 push），供日志逐条列出死链。`git.rs::ensure_gitignore` 追加锚定 `/.llms-wiki/`（只忽略根级报告目录，不误伤已提交的 `<site>/.llms-wiki.json`）。

### Changed

- 整站失败（如入口不可达）现在直接显示真实原因，不再出现 `failed` 却 `failed=0` 的空结果。
  - `crawl` 返回 `Err` 时改走 `progress.abort()`（仅清 spinner 不打 summary）+ `progress::error_line(site, reason)`（`error — <reason>` 红粗）并归为 `Aborted`；commit/record 失败在 `finish` 前把合成 `CrawlFailure` push 进 report，故 summary 行如实显示 `failed=N`。`progress::short_path` 改 `pub(crate)` 供 report 复用。
- 失败报告在 `-q/--quiet` 下仍会显示，错误绝不会被静默吞掉。
  - `print_failures` 独立于 spinner draw target，直接 `eprintln!`；仅逐条 scrollback 受 verbosity 控制。

## [0.9.0] - 2026-07-12

### Fixed

- 修复入口 `llms.txt` 所在 host 与内容链接 host 不一致的站点（如 `bun.sh` 入口、内容在 `bun.com`）一个文件都下载不了：现在信任入口文档声明的各 origin 并纳入抓取范围。
  - 弃用「与入口 URL 严格同源」判定。新增 `url_map::AllowedOrigins`（`Arc<Mutex<HashSet<String>>>`，键 = `Url::origin().ascii_serialization()`），种子为入口 origin；`discover` 与 `HttpClient` 重定向策略均改按 `AllowedOrigins::contains` 判定，替换原 `same_origin`（已删）。`crawler::crawl` 在入口文档（`item.url == canonical_entry`）下载后、`enqueue_discovered` 之前（其间无 await）用 `discovery::declared_links` 提取入口内全部 syncable 链接、`allow` 其 origin 一次性扩展并冻结；内容页不再扩展白名单。并发正确性依赖「入口为初始队列唯一项，其处理先于任何内容 fetch 的 spawn」，故冻结先于所有并发读。

## [0.8.0] - 2026-07-12

### Added

- 同步日志按状态着色：下载 / 未变 / 缺失 / 失败等分类和最终结果分色显示，成功与失败一眼可辨。
  - 复用 `indicatif` 已引入的 `console`（直接声明 `console = "0.16"`，无新增传递依赖）。`src/progress.rs`：`styled_tag` 按 tag 染色滚动记录（OK 绿 / RESUMED 青 / MISS 黄 / FAIL 红粗 / UNCHANGED·IGNORED 暗），先按明文补齐 `{:<9}` 再包色保对齐；`summary_msg` 的 `dl` 常绿、`fail` 仅 >0 转红粗；`summary_line` 的 `ok`/`failed` 状态词与 `downloaded`/`failed=N` 分色。`src/sync.rs` 站点分隔头青粗、续传提示青、push warning 黄；`src/main.rs` 顶层 `error:` 红粗。
- 输出到管道或文件时自动保持纯文本，不写入颜色码。
  - 全部走 `console::style(..).for_stderr()`，按 stderr 自身 tty / `NO_COLOR` / `CLICOLOR` 判定；非终端渲染为纯文本。`progress.rs` 两处 `assert_eq!` 精确匹配测试用 `console::set_colors_enabled_stderr(false)` 钉死，`--nocapture`/`CLICOLOR_FORCE` 下不漂移。

## [0.7.0] - 2026-07-11

### Added

- 同步完成后自动把数据仓库快照推送到源码仓库的 `wiki-data` 分支；无访问权限或未联网时静默跳过，不影响同步结果。
  - `Repository::push_snapshot(url)`：以 ad-hoc URL（不给数据仓库落 remote）推 `HEAD:refs/heads/wiki-data`；非交互式（`GIT_TERMINAL_PROMPT=0` + `GIT_SSH_COMMAND="ssh -o BatchMode=yes -o ConnectTimeout=10"` + `http.lowSpeedLimit=1000` / `http.lowSpeedTime=10`）保证认证 / DNS / 网络异常快速失败不挂起；不加 `--force`，分歧即失败。`sync::run` 新增 `push_url: Option<String>` 参数，仅在至少一个站点提交成功后触发一次；错误经 `eprintln!` 记录为 warning，Quiet 模式下抑制。`src/main.rs::push_snapshot_url` 从 `CARGO_PKG_REPOSITORY` 派生默认 URL（末尾自动补 `.git`），可用运行时 `LLMS_WIKI_PUSH_URL` 覆盖或设空串禁用（`tests/e2e.rs::cli` 统一注入空串隔离测试）。

## [0.6.3] - 2026-07-10

### Changed

- 每个站点单独提交一个 commit，并发运行不同站点的同步不再互相锁死；提交前遇到外部工具短暂占用会自动重试。
  - `Repository::record_sync` → `Repository::record_site`：`git add -- <site> .gitignore` 只提交本站子树；单站流程内跨进程用 `fs2::FileExt::lock_exclusive` 锁 `.git/llms-wiki.commit.lock` 串行化 git 临界区，`git` 子命令遇 `index.lock`/`unable to create *.lock` 时指数退避重试 200/500/1000/2000/4000/8000ms（`is_lock_contention` + `finish_with_retry`）；`src/sync.rs` 在每次 `snapshot.commit` 成功后立即 `record_site`，失败记入错误但内容已落盘，下次覆写即恢复。

## [0.6.2] - 2026-07-10

### Changed

- 默认并发数 4 → 1000，默认请求间隔 500ms → 50ms；同步更快，节流以请求间隔为准。
  - `DEFAULT_CONCURRENCY` 4 → 1000、`DEFAULT_INTERVAL_MS` 500 → 50（`src/config.rs`）；`RequestGate` 全局串行化请求启动时刻，故并发上限主要作在途请求数封顶，实际节流由 `interval` 决定。README 配置示例同步更新并新增 `claude-code` / `cloudflare` 站点示例。

## [0.6.1] - 2026-07-10

### Fixed

- 修复站点的 llms.txt 仅链接到下级 llms.txt 索引（如 Cloudflare）时同步一个文件都不下载：现在会跟进 llms.txt 索引并抓取其中的文档。
  - `is_markdown_url` → `is_syncable_url`，新增放行 `path.ends_with("/llms.txt")`（避免误伤 `foollms.txt`）；`discover` 据此发现嵌套 llms.txt 索引，`crawl` 逐层递归至 `.md`。

## [0.6.0] - 2026-07-10

### Added

- 同步中断可续传：手动中断或异常退出后，下次同步自动复用已下载的文件，只补未完成部分，不重复下载、不重发已完成请求。
  - `Snapshot` 收养 `output_root` 下最新 `.{site}.sync.*` 残留为 working（零拷贝）并 GC 其余；working 不再随 Drop 自动删除，中断即保留供续传。`crawl` 抓取前若文件已在快照目录存在则读本地内容重新发现链接并跳过 `fetch`/`RequestGate`，`CrawlReport.resumed` + `CrawlEvent::Resumed` 计数。

### Fixed

- 写文件改为原子操作，中断不再留下写了一半的文件。
  - `write_document` 写 `<target>.part` 后 `rename`；收养残留时 sweep 陈旧 `.part`；`Repository::prepare` 写数据仓库 `.gitignore` 忽略 `.*.sync.*` / `.*.backup.*`，残留不进版本内容。

## [0.5.0] - 2026-07-10

### Added

- 同步进度更清晰：单行汇总实时显示站点序号、分类计数、在途请求数和耗时。
  - `SyncProgress` 内部 `AtomicU64` 分类计数 + inflight（started−completed），模板 `[i/n] site {spinner} {elapsed} {msg}`，不改 `CrawlEvent`。
- 新增 `-v/--verbose`（逐条打印）与 `-q/--quiet`（仅摘要）控制进度详略。
  - `cli.rs` 两 flag `conflicts_with` 互斥 → `progress::Verbosity`；逐条日志 TTY 走 `bar.println`，非 TTY 走 `eprintln` 避免丢失。

### Changed

- 默认仅将失败逐条打印，成功保持安静；进度不再输出无标签数字和易截断的完整链接。
  - URL 收敛为 origin 相对 `path[?query]`；`--quiet` 用 hidden draw target。

## [0.4.0] - 2026-07-10

### Added

- 同步跳过未变化文件：仅下载有更新的内容，同步摘要新增 `unchanged` 计数，显著减少带宽与耗时。
  - 站点目录内 `.llms-wiki.json` 存 `ETag`/`Last-Modified`；`If-None-Match`/`If-Modified-Since` 条件请求，`304` 复制旧快照文件并从其内容复用链接发现。

## [0.3.0] - 2026-07-10

### Added

- 输出目录自动初始化为独立 Git 仓库；每次有效同步提交全量变更，信息包含成功站点和 UTC 时间。
  - `sync` 抓取前验证独立 Git root，成功快照后执行 `git add -A` + 单次 commit。
- 内容未变化仍记录同步事件；多站点部分失败仍保存成功站点历史。
  - `--allow-empty` 保留事件；Git 初始化失败前置阻断，部分成功提交后仍返回站点错误。

## [0.2.1] - 2026-07-10

### Changed

- 跟随版本同步发布。
  - Artifact upload v4 → v7 / download v4 → v8，切换原生 Node 24 runtime。

## [0.2.0] - 2026-07-10

### Added

- 支持 TOML 配置及多站点添加、排序查看。
  - `clap` + `serde` + `toml` 实现嵌套 CLI、默认配置、校验及同目录原子写入。
- 支持递归同步同源 Markdown，并按 URL 层级生成完整本地快照。
  - Comrak AST + URL 安全映射 + Tokio/reqwest rustls 抓取器递归发现并分类响应。
- 支持临时覆盖并发数与请求间隔；失败时保留上一份完整快照。
  - 全局启动节流 + 并发上限 + 同文件系统临时目录替换/回滚 + indicatif 摘要。

## [0.1.1] - 2026-07-10

### Changed

- 跟随版本同步发布。
  - CI `actions/checkout` v4 → v6；`Cargo.toml` 版本 0.1.0 → 0.1.1。

## [0.1.0] - 2026-07-10

### Added

- 提供 `llms-wiki` 命令行骨架，支持 `curl` 一键安装（macOS arm64/x64）。
  - Rust 2024 edition，手写参数分派（`src/main.rs`）。
  - `release.yml`：`v*` tag 触发，校验 tag==版本，构建双架构 + `checksums.txt` + `gh release create`。
  - `install.sh` 从 latest Release 下载并校验 sha256，装入 `~/.local/bin`。
- 内置 `help` / `version` / `update` / `uninstall` 生命周期命令，可自更新与卸载，并校验下载校验和。
  - `update` 下载 latest 资产，比对 `checksums.txt`，`fs::rename` 原子替换二进制。

[0.12.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.12.1
[0.12.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.12.0
[0.10.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.10.0
[0.9.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.9.0
[0.8.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.8.0
[0.7.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.7.0
[0.6.3]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.3
[0.6.2]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.2
[0.6.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.1
[0.6.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.0
[0.5.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.5.0
[0.4.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.4.0
[0.3.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.3.0
[0.2.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.1
[0.2.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.0
[0.1.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.1
[0.1.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.0
