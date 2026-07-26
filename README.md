```When Editing
本文档作用: 工程总览 + 产品唯一事实源 (价值主张 / 使用 / 架构 / 产品规格); plans/ 各阶段据此实现
MUST NOT 写发布流程 (→ workflow.md) / LLM 约束 (→ AGENTS.md) / dev 命令 (cargo run 等)
遵循 AGENTS.md 文档编写规范
- 产品行为变更 MUST 先改本文档, 再同步 plans/ 与代码
- 章节按需增删, 只留项目真有的; 短并列项用表格; 可执行步骤 fenced + `#` 注释同行
- NEVER 写「开发」段 (VibeCoding 不向人类解释 dev 命令)
```

# jj-llms-txt-wiki

同步 `llms.txt` / `llms-full.txt` 文档，并按站点和 URL 路径保存到本地。

## 使用

支持 macOS arm64/x64：

```bash
curl -fsSL https://raw.githubusercontent.com/yigegongjiang/jj-llms-txt-wiki/main/install.sh | bash
jj-llms-txt-wiki --version
jj-llms-txt-wiki --help
```

生命周期命令：`update`（别名 `upgrade`）/ `uninstall`；版本用内置 `--version`。

## 设计原则

- 不调用 AI 模型，抓取结果完全由 CLI 参数、配置和远端内容决定。
- 支持多个站点，每个站点使用独立目录。
- 根据入口 URL 自动选择链路：`llms.txt` 递归抓取链接；`llms-full.txt` 按内嵌页面 URL 拆分完整内容；用户统一使用 `sync`。
- 抓取范围 = 入口文档声明的 origin 白名单（入口自身 origin + 入口内 Markdown 链接的各 origin），入口抓取后冻结；兼容入口 host 与内容 host 不同的站点（如 `bun.sh` 入口指向 `bun.com` 内容）。
- 递归只跟踪白名单内 origin 的 Markdown URL；内容页只能引用白名单内 origin，不能扩大白名单。白名单外链接和重定向目标一律忽略。
- 并发数 = 同时在途的下载任务数，请求间隔 = 单任务完成后的休息时长（见「并发与限速」）。
- 保留 URL 层级，映射为本地子目录。
- 输出根目录自动维护 Git 历史，每次有效同步均可追溯。

## 技术选型

- 使用 Rust 开发原生 CLI。
- 使用 `clap` 解析命令，`serde` + `toml` 管理配置，`serde_json` 持久化站点级 sync 元数据。
- 使用 Tokio 和 `reqwest`（`rustls`）执行异步抓取、并发控制与请求限速。
- 使用 Comrak 解析 `llms.txt` CommonMark/GFM AST，使用确定性行解析器拆分 `llms-full.txt`，使用 `url` 解析和规范化 URL。
- 首版不实现 TUI，使用 `indicatif` 展示同步进度。

## CLI

```bash
# 添加站点
jj-llms-txt-wiki site add anthropic https://platform.claude.com/llms.txt
jj-llms-txt-wiki site add deno https://docs.deno.com/llms-full.txt

# 查看站点
jj-llms-txt-wiki site list

# 同步全部站点
jj-llms-txt-wiki sync

# 同步指定站点
jj-llms-txt-wiki sync anthropic
jj-llms-txt-wiki sync deno

# 同步全部站点，并临时覆盖抓取限制
jj-llms-txt-wiki sync --concurrency 16 --interval 0ms
```

CLI 参数优先于配置文件，未提供的参数使用配置值或默认值。

同步进度显示在 stderr：单行汇总实时展示站点序号、分类计数（下载 / 未变 / 缺失 / 失败）、`inflight=<在途>/<并发上限>` 和耗时，失败逐条打印为滚动记录。非终端环境（管道 / 重定向）自动隐藏动态刷新，仍输出摘要。每次同步结束固定打印一行末行总账：全部成功为绿色 `✓ N/N synced`；存在失败为红色 `✗ N ok · M failed · K error → <日志路径>` 并以非零状态退出，使失败结果不会淹没在成功输出中。

## 配置

默认配置文件：

```text
~/.config/jj-llms-txt-wiki/config.toml
```

```toml
output_dir = "~/.config/jj-llms-txt-wiki/wiki"
concurrency = 8
interval_ms = 100

[sites.claude-code]
url = "https://code.claude.com/docs/llms.txt"

[sites.cloudflare]
url = "https://developers.cloudflare.com/llms.txt"

[sites.anthropic]
url = "https://platform.claude.com/llms.txt"

[sites.deno]
url = "https://docs.deno.com/llms-full.txt"
```

## 并发与限速

`concurrency` = 下载槽位数，`interval` = 单个槽位完成一次请求后的休息时长。

- 单槽位循环：取 URL -> 休息 `interval` -> 发起请求 -> 完成 -> 取下一个 URL。休息只作用于该槽位，MUST NOT 阻塞其他槽位。
- `interval = 0` -> 无休息，队列有货时在途请求数恒等于 `concurrency`。
- `interval > 0` -> 在途请求数 = `concurrency` 减去正在休息的槽位数，稳态均值 ≈ `concurrency × latency / (latency + interval)`；休息从请求真正结束起算。
- 吞吐 ≈ `concurrency / (latency + interval)`。
- 入口文档同样占一个槽位并在完成后休息，因此首轮内容请求为 `concurrency - 1` 个立即发起。
- 剩余 URL 少于槽位数时，在途请求数取剩余数量。
- 上限 64：配置或 CLI 参数超过时收敛到 64 并打印 warning（防止 socket 耗尽与远端 429）。
- 站点串行同步，并发作用于单站点内部的抓取队列。
- 命中本地续传的文件不占槽位、不触发休息 —— 无网络请求。

## 输出目录

默认站点根目录为 `~/.config/jj-llms-txt-wiki/wiki/`（配置目录下的 `wiki/` 子目录），可通过 `output_dir` 修改：

```text
~/.config/jj-llms-txt-wiki/
├── config.toml
└── wiki/
    ├── .git/
    ├── anthropic/
    │   ├── .jj-llms-txt-wiki.json
    │   └── docs/
    │       └── api/
    │           └── messages.md
    └── another-site/
```

每个站点使用配置名称作为顶层目录，远端 URL 路径映射为其下的文件路径。

`llms.txt` 站点目录内的 `.jj-llms-txt-wiki.json` 记录各文件的 HTTP validator（`ETag` / `Last-Modified`），供下次同步做条件请求；它随快照原子替换、随仓库提交一同版本化。`llms-full.txt` 每次全量抓取并重建快照，不创建 manifest。

## 同步行为

`sync` 默认同步全部站点；传入站点名称时仅同步指定站点。入口 URL path 末段为 `llms-full.txt`（大小写不敏感）时走聚合链路，其他入口走递归链路；query / fragment 不参与识别。

### `llms.txt`

1. 读取目标站点的 `llms.txt`（入口始终无条件抓取）。
2. 以入口文档中全部 Markdown 链接的 origin 扩展白名单（连同入口自身 origin 冻结）；提取白名单内的 Markdown URL，去重后加入抓取队列，白名单外 URL 直接忽略。
3. 按「并发与限速」的槽位模型下载到临时站点目录；对上次已记录 validator 且本地仍存在的文件发条件请求（`If-None-Match` 优先，`If-Modified-Since` 兜底）。单个内容页超过 3 MiB 视为异常，主动剔除（不写盘、不记 validator、不参与递归），并计入 `oversize` 记录到运行日志；入口文档不受此限。
4. 从每个已下载 Markdown 中继续提取白名单内的 Markdown URL，将未处理的 URL 加入队列，直至队列为空；内容页不再扩展白名单。
5. 队列清空且不存在未确定的抓取错误后，以本次得到的完整快照替换原站点目录；远端已删除或不再可达的 Markdown 随之从本地移除。
6. 至少一个站点成功后，对输出根目录执行 `git add -A` 并创建一次提交；即使内容未变化也记录同步事件。

远端返回 `304 Not Modified` 时跳过 body 下载，复制上次快照的本地文件并沿用其 validator，再从该文件重新提取链接继续递归——字节一致保证链接集合不变。服务器不带 `ETag` / `Last-Modified` 时自然退化为全量下载，无正确性风险。

### `llms-full.txt`

1. 单次抓取入口；只有一个请求，并发数和请求间隔不参与该链路。
2. 识别代码块外的页面头：H1 + 可选 blockquote + 独立 `URL: <absolute HTTP(S) URL>`；其前可有 `---`。
3. 完整校验所有页头、URL、正文和本地路径；结构损坏、重复 URL 或路径冲突 → 整站失败且不写入旧快照。
4. 按页面 URL 映射 Markdown 路径：无后缀追加 `.md`，目录 URL 写入 `index.md`，已有 `.md` / `.markdown` 保持不变。
5. 写入全新临时快照；全部成功后原子替换站点目录并记录 Git 提交。每次从空快照重建，远端已删除页面不会残留。

拆分保留页面标题、描述和正文，仅移除聚合分隔符与 `URL:` 元数据行。缺少可验证页头的聚合格式直接报错，MUST NOT 猜测边界或静默生成错误文件。

输出根目录会在抓取前初始化为独立 Git 仓库。提交信息格式为 `chore(sync): <成功站点> @ <RFC 3339 UTC>`；部分站点失败时先记录成功站点，再以非零状态退出。

抓取仅跟随目标落在白名单内的重定向；白名单外重定向目标直接忽略，不视为同步错误。

`404` 和 `410` 视为确定不存在；超时、`429` 和 `5xx` 等错误会使本次同步失败，并保留上一次完整快照。

首版不创建持久化 `.cache` 目录。同步时仅在输出根目录中创建与站点目录同级的临时目录，成功后替换原目录，失败后清理。
