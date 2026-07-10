# `llms-wiki` 分阶段实施计划

日期：2026-07-10

## 最终目标

以项目根目录的 [`README.md`](../../README.md) 为唯一产品事实源，
将现有 Rust 发布骨架实现为可实际使用的 CLI：管理多个 `llms.txt` 站点，
递归抓取同源 Markdown，并以完整快照保存到本地。

全部阶段完成后，以下命令必须可用：

```bash
llms-wiki site add anthropic https://platform.claude.com/llms.txt
llms-wiki site list
llms-wiki sync
llms-wiki sync anthropic
llms-wiki sync --concurrency 2 --interval 1s
llms-wiki update
llms-wiki uninstall
```

## 当前基线

- 已有 Rust 2024 Edition 的 Hello World CLI。
- 已有 `help`、`version`、`update`/`upgrade`、`uninstall`。
- 已有 macOS arm64/x64 Release workflow、checksum 和 `install.sh`。
- 尚无业务 CLI、配置、Markdown 解析、抓取、快照和同步进度。
- 当前工作区已有未提交内容；实施时不得写入 staging area，也不得覆盖无关改动。

## 固定实现契约

- 只处理 HTTP/HTTPS Markdown 链接；入口 `llms.txt` 只用于发现，不写入站点快照。
- 同源按 URL origin 判断，即 scheme、host、effective port 全部相同。
- URL fragment 不参与抓取身份；query 保留。不同 URL 映射到同一本地路径时，本次同步失败，避免并发覆盖。
- Markdown 候选 URL 的 path 以 `.md` 或 `.markdown` 结尾，大小写不敏感。
- 本地路径使用 URL path 的层级；保留百分号编码，并拒绝任何可能逃逸站点目录的路径。
- `concurrency` 限制同时在途请求；`interval` 限制全局请求启动间隔，首个请求立即开始，`0` 表示不等待。
- `404`、`410` 和不同源重定向是确定性缺失，跳过且不阻止提交新快照。
- 超时、网络错误、`429`、`5xx`、其他非成功状态、重定向异常、解析或文件错误均使对应站点同步失败。
- 每个站点独立生成完整临时快照；成功后替换旧目录，失败则清理临时目录并保留旧目录。
- `sync` 多站点按配置名称排序执行；单站点失败不回滚已成功站点，但命令最终返回非零。
- 抓取前将 `output_dir` 初始化为独立 Git 仓库；至少一个站点成功后全量暂存并提交，内容不变也保留一次同步记录。
- 不增加 TUI、AI 调用、持久化 `.cache`、增量更新、跨站点事务或隐式重试。

这些约定只消解 README 中实现所必需的歧义；如果后续调整产品行为，应先更新根 `README.md`，再同步计划和代码。

## 执行顺序

1. [`01-cli-foundation.md`](01-cli-foundation.md)：替换手写参数分派，建立可扩展 CLI 入口并保留生命周期命令。
2. [`02-config-and-sites.md`](02-config-and-sites.md)：实现 TOML 配置及 `site add/list`。
3. [`03-url-and-discovery.md`](03-url-and-discovery.md)：实现 URL 规范化、
   同源过滤、本地路径映射和 Comrak 链接发现。
4. [`04-crawl-engine.md`](04-crawl-engine.md)：实现单站点异步递归抓取、并发、限速、重定向和错误分类。
5. [`05-snapshot-and-sync.md`](05-snapshot-and-sync.md)：实现快照替换、多站点编排和同步进度。
6. [`06-acceptance-and-release.md`](06-acceptance-and-release.md)：
   完成端到端验收、README 状态更新和 Release 实测。

必须按顺序实施；当前阶段的验收门槛全部通过后，才能进入下一阶段。

## README 覆盖

- 设计原则：阶段 3–5。
- Rust CLI 与技术选型：阶段 1–5。
- `site add`、`site list`：阶段 2。
- `sync` 全部/指定站点及临时覆盖：阶段 4–5。
- 默认配置、优先级和多站点配置：阶段 2、5。
- URL 层级输出：阶段 3、5。
- 递归、去重、同源链接和重定向：阶段 3–4。
- 并发数和请求间隔：阶段 4。
- 完整快照、Git 历史、删除远端缺失文件、失败保留旧快照：阶段 5。
- 无 `.cache`、临时目录清理：阶段 5。
- `indicatif` 进度：阶段 5。
- 安装、自更新、卸载和双架构 Release：阶段 6。

## 全局质量门槛

每阶段至少执行：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
git diff --check
```

阶段 6 另执行：

```bash
cargo build --release --locked
```

完成判定：六个阶段的验收项全部有当前代码、测试输出或 Release 产物作为证据；仅有实现意图或未执行的测试不算完成。

## 技术依据

检索日期：2026-07-10。

- [`llms.txt` proposal](https://llmstxt.org/)
- [`clap` derive tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/)
- [`toml` Serde integration](https://docs.rs/toml/latest/toml/)
- [`Comrak` AST nodes](https://docs.rs/comrak/latest/comrak/nodes/enum.NodeValue.html)
- [`reqwest` redirect policy](https://docs.rs/reqwest/latest/reqwest/redirect/struct.Policy.html)
- [`Tokio` semaphore](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html)
- [`tempfile` temporary directories](https://docs.rs/tempfile/latest/tempfile/)
- [`indicatif` progress rendering](https://docs.rs/indicatif/latest/indicatif/)
- [`git init`](https://git-scm.com/docs/git-init) / [`git add`](https://git-scm.com/docs/git-add) / [`git commit`](https://git-scm.com/docs/git-commit)
- [`Jiff` timestamp](https://docs.rs/jiff/latest/jiff/struct.Timestamp.html)
