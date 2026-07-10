# jj-llms-txt-wiki

从 [`llms.txt`](https://llmstxt.org/) 入口递归抓取同源 Markdown，并按站点和 URL 路径保存到本地。

当前仓库处于产品设计阶段，CLI 尚未实现。

## 设计原则

- 不调用 AI 模型，抓取结果完全由 CLI 参数、配置和远端内容决定。
- 支持多个站点，每个站点使用独立目录。
- 递归跟踪已下载 Markdown 中引用的同源 Markdown URL，不处理不同源链接和重定向目标。
- 支持限制并发数和请求间隔。
- 保留 URL 层级，映射为本地子目录。

## 技术选型

- 使用 Rust 开发原生 CLI。
- 使用 `clap` 解析命令，`serde` 和 `toml` 管理配置。
- 使用 Tokio 和 `reqwest`（`rustls`）执行异步抓取、并发控制与请求限速。
- 使用 Comrak 解析 CommonMark/GFM AST，使用 `url` 解析和规范化 URL。
- 首版不实现 TUI，使用 `indicatif` 展示同步进度。

## CLI

```bash
# 添加站点
llms-wiki site add anthropic https://platform.claude.com/llms.txt

# 查看站点
llms-wiki site list

# 同步全部站点
llms-wiki sync

# 同步指定站点
llms-wiki sync anthropic

# 同步全部站点，并临时覆盖抓取限制
llms-wiki sync --concurrency 2 --interval 1s
```

CLI 参数优先于配置文件，未提供的参数使用配置值或默认值。

## 配置

默认配置文件：

```text
~/.config/llms-wiki/config.toml
```

```toml
output_dir = "~/llms-wiki"
concurrency = 4
interval_ms = 500

[sites.anthropic]
url = "https://platform.claude.com/llms.txt"
```

## 输出目录

`~/llms-wiki/` 是默认站点根目录，可通过 `output_dir` 修改：

```text
~/llms-wiki/
├── anthropic/
│   └── docs/
│       └── api/
│           └── messages.md
└── another-site/
```

每个站点使用配置名称作为顶层目录，远端 URL 路径映射为其下的文件路径。

## 同步行为

`sync` 默认同步全部站点；传入站点名称时仅同步指定站点。

1. 读取目标站点的 `llms.txt`。
2. 提取与入口 URL 同源的 Markdown URL，去重后加入抓取队列；不同源 URL 直接忽略。
3. 按并发数和请求间隔下载到临时站点目录。
4. 从每个已下载 Markdown 中继续提取同源 Markdown URL，将未处理的 URL 加入队列，直至队列为空。
5. 队列清空且不存在未确定的抓取错误后，以本次得到的完整快照替换原站点目录；远端已删除或不再可达的 Markdown 随之从本地移除。

抓取仅跟随同源重定向；不同源重定向目标直接忽略，不视为同步错误。

`404` 和 `410` 视为确定不存在；超时、`429` 和 `5xx` 等错误会使本次同步失败，并保留上一次完整快照。

首版不创建持久化 `.cache` 目录。同步时仅在输出根目录中创建与站点目录同级的临时目录，成功后替换原目录，失败后清理。
