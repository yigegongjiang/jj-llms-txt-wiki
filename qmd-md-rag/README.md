# qmd-rag

基于 [tobi/qmd](https://github.com/tobi/qmd) 的本地文档索引与检索。BM25 全文 + 向量语义 + LLM 重排，全部本地运行。**索引与配置均为全局**（不再放在本仓库），本目录只保留这份使用说明。

## 前置

- 已全局安装 qmd：`bun add -g @tobilu/qmd`（或 `npm i -g @tobilu/qmd`）；确认 `which qmd` 有输出
- 首次 `qmd embed` / `qmd query` 会自动从 Hugging Face 下载 GGUF 模型到 `~/.cache/qmd/models/`

## 索引管理

- `init` — 在当前目录生成项目本地索引（本仓库已切到全局模式，不再需要）
- `status` — 查看索引与 collection 健康状态
- `update [--pull]` — 重新索引所有 collection；`--pull` 会先 `git pull`
- `cleanup` — 清缓存、`VACUUM` 数据库

## Collection（要索引的目录）

- `collection add <path> --name <name>` — 注册目录，例：`collection add ~/notes --name notes`
- `collection list` — 列出全部
- `collection show <name>` — 详情
- `collection rename <old> <new>` — 改名
- `collection remove <name>` — 移除
- `ls [name[/subpath]]` — 查看已索引的文件

## Context（人工补写的语义摘要，提升检索质量）

- `context add qmd://<name> "<描述>"` — 例：`context add qmd://notes "个人笔记与灵感"`
- `context list`
- `context rm qmd://<name>`

## Embedding（向量化）

- `embed` — 为所有 collection 生成/刷新向量
- `embed -f` — 强制重跑
- `embed -c <name>` — 只处理指定 collection
- `--max-docs-per-batch <n>` / `--max-batch-mb <n>` — 限制单批载入内存的文档数 / UTF-8 MB
- `--chunk-strategy <auto|regex>` — 分块模式（默认 `regex`；`auto` 对代码文件走 AST）

## 检索

- `search "<关键词>"` — BM25 全文（快，无 LLM）
- `vsearch "<自然语言>"` — 纯向量语义
- `query "<问题>"` — 混合检索 + 查询扩展 + 重排（推荐，质量最高）

### query 语法

单行 = 隐式扩展查询；多行 = 结构化查询文档（每行须带类型前缀）。两者不可混用。

- `intent: <text>` — 可选首行，声明检索意图
- `lex: <text>` — BM25 词法；支持 `"精确短语"` 与 `-排除词`
- `vec: <text>` — 向量语义
- `hyde: <text>` — 假设性答案（HyDE）
- `expand: <text>` — 显式扩展（等价于单行不带前缀）

```sh
query "how does auth work"                          # 单行 → 隐式扩展
query $'lex: CAP theorem\nvec: consistency'         # 组合词法 + 语义
query $'lex: "exact matches" sports -baseball'      # 精确短语 + 排除
query $'hyde: Hypothetical answer text'             # 仅 HyDE
```

### 检索通用选项（search / vsearch / query）

- `-n <num>` — 最大结果数（默认 5；`--format files|json` 时为 20）
- `--all` — 返回全部匹配（配合 `--min-score`）
- `--min-score <num>` — 最低相似度阈值
- `--full` — 输出完整文档而非片段
- `-C, --candidate-limit <n>` — 送入重排的候选上限（默认 40，越小越快）
- `--no-rerank` — 跳过 LLM 重排（仅用 RRF 分数，CPU 上快得多）
- `--no-gpu` — 强制 CPU（等价 `QMD_FORCE_CPU=1`）
- `-c, --collection <name>` — 按 collection 过滤（可多个）
- `--format <cli|json|csv|md|xml|files>` — 输出格式（默认 `cli`；`json` 适合喂给 Agent）
- `--explain` — 附检索打分溯源（`query`；CLI 或 `--format json`）
- `--full-path` — 显示磁盘路径而非 `qmd://` + docid
- `--line-numbers` — search 加行号（get/multi-get 默认已带）

## 文档获取

- `get <file>[:from[:count]]` — 显示单文档（带行号，头部含 `#docid`）
- `get "#<docid>"` — 通过 docid 获取
- `multi-get "<glob 或 CSV>"` — 批量获取
  - `-l <num>` — 每文件最大行数
  - `--max-bytes <num>` — 跳过大于 N 字节的文件（默认 10240）
  - `--no-line-numbers` — 关闭行号
  - `--format <...>` — 同检索的格式

## MCP / Skills（面向 AI Agent）

- `mcp` — 启动 MCP 服务（stdio 传输），供 Claude Code 等接入
  - `mcp --http ...` / `mcp --http --daemon` — 自定义传输（高级）
- `skills list` / `skills get <name>` / `skills path` — 内置 runtime skills
- `skills get qmd --full` — 取版本匹配的 Agent 指令
- `skill show`（别名 `--skill`）— 展示 QMD skill
- `skill install` — 安装到 `./.agents/skills/qmd`
- `skill install --global` — 安装到 `~/.agents/skills/qmd`

## Benchmark

- `bench <fixture.json>` — 用 fixture 跑检索质量评测

## 全局选项与环境变量

- `--index <name>` — 使用命名索引（默认 `index`），可在同项目维护多套
- `QMD_EDITOR_URI` — TTY 输出中可点击链接的编辑器模板
- `QMD_FORCE_CPU=1` — 强制 CPU 模式（同 `--no-gpu`）

## 目录结构（全局）

- `~/.config/qmd/index.yml` — 配置（collection、模型 URI）
- `~/.cache/qmd/index.sqlite` — 索引与向量数据
- `~/.cache/qmd/models/` — GGUF 模型缓存（跨项目共享）

> 若在某仓库根执行了 `qmd init`，会生成 `.qmd/`，qmd 会**优先使用它**并覆盖全局配置——除非确需多套并存，否则删除该目录即可回到全局。

## 默认模型

- Embedding：`ggml-org/embeddinggemma-300M-GGUF`
- Reranking：`ggml-org/Qwen3-Reranker-0.6B-Q8_0-GGUF`
- Generation（查询扩展）：`tobil/qmd-query-expansion-1.7B-gguf`

更换模型：改 `~/.config/qmd/index.yml` 里 `models:` 下的 HF URI。
