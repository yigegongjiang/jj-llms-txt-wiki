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
curl -fsSL https://raw.githubusercontent.com/yigegongjiang/jj-llms-txt-wiki/main/scripts/install.sh | bash
jj-llms-txt-wiki --version
jj-llms-txt-wiki --help
```

生命周期命令：`update`（别名 `upgrade`）/ `uninstall`；版本用内置 `--version`。

## 设计原则

- 不调用 AI 模型，抓取结果完全由 CLI 参数、配置和远端内容决定。
- 支持多个站点，每个站点使用独立目录；单个站点可声明多个入口 URL，共享同一目录与同一份快照。
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

# 添加多入口站点（同站点的多个 section 索引 / 多个聚合包）
jj-llms-txt-wiki site add cloudflare https://developers.cloudflare.com/workers/llms.txt https://developers.cloudflare.com/pages/llms.txt

# 查看站点（多入口以空格分隔）
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

同步进度显示在 stderr：单行汇总实时展示站点序号、分类计数（下载 / 未变 / 缺失 / 失败 / 降级）、`inflight=<在途>/<并发上限>` 和耗时，失败与降级逐条打印为滚动记录。非终端环境（管道 / 重定向）自动隐藏动态刷新，仍输出摘要。每次同步结束固定打印一行末行总账：

<!-- prettier-ignore -->
| 结果 | 末行 | 退出码 |
| --- | --- | --- |
| 全部成功 | 绿色 `✓ N/N synced` | 0 |
| 成功但有降级页 | 黄色 `✓ N/N synced · M degraded → <日志路径>` | 0 |
| 存在失败 | 红色 `✗ N ok · M failed · K error → <日志路径>` | 非零 |

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

[sites.multi]
urls = [
  "https://example.com/workers/llms.txt",
  "https://example.com/pages/llms.txt",
]
```

### 站点入口

单入口写 `url`，多入口写 `urls` 数组；两者只能出现一个，字段名拼错直接报错（MUST NOT 静默把站点变成零入口）。`site add` 接受多个 URL，`site list` 以空格分隔展示。

多入口约束：

- 同一站点的入口 MUST 同类型（全 `llms.txt` 或全 `llms-full.txt`）；两条链路的快照策略互斥（递归链路续传 + manifest 条件请求，聚合链路每次从空重建）。
- 入口 URL MUST NOT 重复。
- 入口 domain MAY 相同或不同；白名单 = 全部入口 origin 的并集，再由各入口文档各自扩展。
- 所有入口的内容合并进同一站点目录，按 URL path + query 映射（与 host 无关）；不同入口的两个 URL 映射到同一本地路径 = 路径冲突 = 整站失败。

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

同一输出根还可容纳无 `llms.txt` 的站点，由 [scrapers/](./scrapers/README.md) 的抓取脚本写入（如 `sunmi-zh/`）。这类目录不出现在 `config.toml` 的 `[sites.*]` 中；`sync` 只暂存自己同步的站点子树，既不同步、不删除、也不提交它们，其 Git 提交由抓取侧自行负责。

`llms.txt` 站点目录内的 `.jj-llms-txt-wiki.json` 记录各文件的 HTTP validator（`ETag` / `Last-Modified`），供下次同步做条件请求；它随快照原子替换、随仓库提交一同版本化。`llms-full.txt` 每次全量抓取并重建快照，不创建 manifest。

### 快照镜像

同步结束后把数据仓库的本地 HEAD 推到源码仓库的 `wiki-data` 分支。best-effort：无权限 / 未联网 / 与远端历史分叉一律只打 `warning: push snapshot skipped: …`，不影响同步结果与退出码。永不 `--force`，远端已有内容不会被静默覆盖。

推送目标 = 构建时源码仓库的 `origin`（`build.rs` 编译进二进制），无 `.git` 目录可问时回退 `[package].repository`。fork 零配置即正确：谁 clone 谁构建，就推到自己 clone 的那个仓库，MUST NOT 推到上游。

推送凭证同样从目标 URL 推导：HTTPS 目标取 host + owner，向 `gh` 索取该账号的 token 并只注入本次 `git push`，不读写、不改动本机的活跃账号——本机登录多个 GitHub 账号时不会用错身份。无 `gh` / 该账号未登录 / SSH 目标 → 沿用本机默认凭证（SSH key、`osxkeychain` 等）。

`JJ_LLMS_TXT_WIKI_PUSH_URL` 仅作关闭开关（置空 = 不推送），e2e 测试用它保证测试运行永不触网推送。

镜像内容是第三方公开文档，正常文本会被 GitHub secret scanning 误判为 API key 而拒绝推送（如 HuggingFace 的 `Mistral3ForConditionalGeneration` 恰为 32 位字母数字，命中 Mistral key 模式）。源码仓库的 `.github/secret_scanning.yml` 用 `paths-ignore` 排除 `*.md` / `*.txt` / `*.json`；该文件只在默认分支生效，作用于全仓库所有分支的 push，源码扩展名不在排除内。

## 同步行为

`sync` 默认同步全部站点；传入站点名称时仅同步指定站点。入口 URL path 末段为 `llms-full.txt`（大小写不敏感）时走聚合链路，其他入口走递归链路；query / fragment 不参与识别。多入口站点的入口同类型，链路唯一。

### `llms.txt`

1. 读取目标站点的全部 `llms.txt` 入口（入口始终无条件抓取）。
2. 以各入口文档中全部 Markdown 链接的 origin 扩展白名单（连同全部入口自身 origin 冻结）；提取白名单内的 Markdown URL，去重后加入抓取队列，白名单外 URL 直接忽略。多入口时全部入口抓完才放行内容页——否则先落地的入口的内容页会在白名单尚未完整时丢链接，且结果随网络时序漂移。入口之间互相链接不算内容页，MUST NOT 写盘。
3. 按「并发与限速」的槽位模型下载到临时站点目录，本地路径按 URL path + query 映射（命名规则见 [`llms-full.txt`](#llms-fulltxt) 第 4 步）；对上次已记录 validator 且本地仍存在的文件发条件请求（`If-None-Match` 优先，`If-Modified-Since` 兜底）。单个内容页超过 3 MiB 视为异常，主动剔除（不写盘、不记 validator、不参与递归），并计入 `oversize` 记录到运行日志；入口文档不受此限。
4. 从每个已下载 Markdown 中继续提取白名单内的 Markdown URL，将未处理的 URL 加入队列，直至队列为空；内容页不再扩展白名单。
5. 队列清空且不存在未确定的抓取错误后，清除临时目录中本次无 URL 认领的文件与空目录（中断残留的续传目录可能带着已删页面或旧命名规则的产物），再以该快照替换原站点目录；远端已删除或不再可达的 Markdown 随之从本地移除。404 页面的 URL 在发现阶段即已登记，其续传副本不受影响。
6. 每个站点成功后，仅暂存该站点子树（连同输出根的 `.gitignore`）并创建一次提交；即使内容未变化也记录同步事件，输出根内的其他文件不受影响。

远端返回 `304 Not Modified` 时跳过 body 下载，复制上次快照的本地文件并沿用其 validator，再从该文件重新提取链接继续递归——字节一致保证链接集合不变。服务器不带 `ETag` / `Last-Modified` 时自然退化为全量下载，无正确性风险。

### `llms-full.txt`

1. 逐个抓取入口，一次一个请求；并发数和请求间隔不参与该链路（聚合文件体积大，串行使峰值内存只受单个包约束）。
2. 识别代码块外的页面头，两种格式二选一（见下）。
3. 完整校验所有页头、URL、正文和本地路径；结构损坏、重复 URL（含跨入口同一页面 URL）或路径冲突（含跨入口）→ 整站失败且不写入旧快照。
4. 按页面 URL 映射 Markdown 路径：无后缀追加 `.md`，目录 URL 写入 `index.md`，已有 `.md` / `.markdown` 保持不变；带 query 的 URL 在文件名 stem 后追加 `__<query>`（非 `[A-Za-z0-9._=-]` 字符替换为 `-`，经替换或超 48 字符时再补 `-<fnv1a64 hex>`），无 query 的 URL 命名不变。
5. 写入全新临时快照；全部成功后原子替换站点目录并记录 Git 提交。每次从空快照重建，远端已删除页面不会残留。多入口的全部页面合并进同一份快照。

#### 页头格式

同一份聚合文件只用一种格式；`URL:` 标记优先，出现任一标记页头即全程按标记格式拆分。

- **`URL:` 标记式**：H1 + 可选 blockquote + 独立 `URL: <absolute HTTP(S) URL>`；其前可有 `---`。页头是页面内容，标题和描述保留进输出，仅移除 `URL:` 元数据行。
- **裸 URL 式**：任意级别标题 + 紧邻下一行的独立绝对 HTTP(S) URL（HuggingFace 风格）。页头是聚合分隔符，不进输出——正文自带标题；正文为空时回退为该标题的 H1。

裸 URL 式仅接受出现次数最多的标题级别，正文里偶发的「小节标题 + 链接行」不会被误判为页头。首个页头之前的内容（聚合标题 / 目录索引）不属于任何页面，直接丢弃。

缺少可验证页头的聚合格式直接报错，MUST NOT 猜测边界或静默生成错误文件。

输出根目录会在抓取前初始化为独立 Git 仓库。提交信息格式为 `chore(sync): <成功站点> @ <RFC 3339 UTC>`；部分站点失败时先记录成功站点，再以非零状态退出。

抓取仅跟随目标落在白名单内的重定向；白名单外重定向目标直接忽略，不视为同步错误。

### 抓取错误处理

`404` / `410` 视为确定不存在，计入 `missing`。

超时、连接错误、`429`、`5xx` 先重试：单个 URL 最多 3 次请求，退避 500 ms / 1500 ms，`Retry-After` ≤ 10 s 时优先采用。重试期间占用原下载槽位。

重试耗尽后：

- 入口文档失败 → 整站中止，保留上一次完整快照（白名单由入口冻结，缺它无法抓取）；多入口时任一入口失败即中止
- 内容页失败 → 计入 `degraded`，不阻塞快照：上次快照存在该文件则复制过来并沿用其 validator，同时从该副本继续提取链接（保证链接集合不收缩、仅经它可达的页面不被误删）；不存在则本次快照不含该页

`degraded` 页数 ≤ `max(3, 已处理页数 / 100)` → 快照照常发布并提交，退出码 0，末行黄色标注、逐条写入运行日志；超过阈值判定为上游故障或整体封禁（此时发布会静默删除真实内容），全部转为失败、整站保留上一次完整快照。

该阈值使「上游文档索引仍链接着自己已渲染不出的页面」（常见于 `5xx` 实为 `404`）不会让站点永久无法同步，上游修复后下次同步自动恢复。

首版不创建持久化 `.cache` 目录。同步时仅在输出根目录中创建与站点目录同级的临时目录，成功后替换原目录，失败后清理。
