# `llms-full.txt` 统一同步支持

日期：2026-07-12

## 决策

- 用户动作只有 `sync`；MUST NOT 新增 `sync-full`。
- 入口 URL 末段为 `llms-full.txt`（大小写不敏感）→ 内部分流到聚合文件链路。
- 其他入口 → 既有 `llms.txt` 递归链路；现有配置 / 命令 / 输出目录兼容。
- 两条链路只分离抓取与解析；快照提交 / Git / 报告统一复用。

```bash
# llms.txt
llms-wiki site add anthropic https://platform.claude.com/llms.txt

# llms-full.txt
llms-wiki site add deno https://docs.deno.com/llms-full.txt

# 用法完全一致
llms-wiki sync deno
llms-wiki sync
```

## 事实边界

- `llms-full.txt` 的名称与“聚合完整内容”用途已形成通用约定；无需暴露另一套 CLI。
- [`llms.txt` proposal](https://llmstxt.org/) 未规定聚合文件的页面边界语法，解析器 MUST 基于可验证结构，MUST NOT 假设任意 `llms-full.txt` 均可拆分。
- 2026-07-12 实测 [`Deno llms-full.txt`](https://docs.deno.com/llms-full.txt)：页面使用 H1 + 可选 blockquote + `URL: <absolute-url>` 头部；包含正文 `URL:` 与仅标题页面；共 479 个页头。数量会随远端变化，验收 MUST NOT 固定为 479。

## 架构

```text
sync::sync_site
  -> EntryKind::from_url
     -> Index -> crawler::crawl
     -> Full  -> full::crawl
  -> Snapshot::commit
  -> Repository::record_site
  -> 统一进度 / 报告
```

### 入口识别

新增 `EntryKind::{Index, Full}`：

- 使用解析后的 URL path 最后一个 segment 判断；query / fragment 不参与。
- `llms-full.txt`（大小写不敏感）→ `Full`。
- 其他合法入口 → `Index`，保持现有宽松 URL 契约。
- MUST NOT 在 `site add` 发网络请求或持久化类型；URL 是唯一事实源，无配置迁移。

### `full::crawl`

新增 [`src/full.rs`](../../src/full.rs)：

```rust
pub async fn crawl(
    entry: Url,
    snapshot_root: &Path,
    timeout: Duration,
    observer: Arc<dyn CrawlObserver>,
) -> Result<CrawlReport, String>
```

流程：

1. 复用 `HttpClient` 的 user-agent / timeout / redirect / UTF-8 / HTTP 状态语义；单次 GET，不进入并发队列 / 限速 gate。
2. 完整解析并校验为 `Vec<FullPage>`；校验结束前 MUST NOT 写文件。
3. 预计算全部本地路径并完成 URL / 路径冲突校验；任一失败 → 整站失败。
4. 将页面写入 fresh staging snapshot；全部成功后由 `sync_site` 统一 commit + Git 记录。
5. `CrawlReport.downloaded` = 拆分页数；进度只报告聚合 GET，MUST NOT 为每个拆分页伪造 HTTP 请求事件。

MUST NOT 复用 `Manifest` / `discovery` / 并发队列；它们解决的是多 URL 增量抓取。

MUST 复用 `HttpClient` / `CrawlReport` / `Snapshot` / `Repository`；上一版用裸 `reqwest::get` + 在 `full::run` 重写提交链路会造成超时、重定向、报告与 Git 语义漂移。

### 聚合解析

`full.rs` 内新增纯函数：

```rust
fn split(body: &str) -> Result<Vec<FullPage>, String>

struct FullPage {
    url: Url,
    markdown: String,
}
```

规则：

- 逐行扫描并跟踪 fenced code block；代码块内的 `URL:` / `---` / heading MUST NOT 参与识别。
- 页面头 = H1 + 可选 blockquote + 独立 `URL: <absolute HTTP(S) URL>`；其前可有 `---`。
- H1 至下一有效页面头 = 当前页；输出保留标题 / 描述 / 正文，仅移除聚合分隔符与 `URL:` 元数据行。
- 首个带 URL 的页头之前内容 = 聚合文档前言，不作为页面输出。
- 正文中的 `URL:` 原样保留；仅 H1 头部位置的 `URL:` 参与页面识别。
- 校验：至少一页 / 页头 URL 合法且唯一；允许仅标题页面。
- 任一结构不满足 → 明确报错 + 不替换旧快照；MUST NOT 猜边界或降级为错误拆分。

### URL → Markdown 路径

现有 `local_path()` 只接受文件型 URL，不覆盖 Deno 的无后缀 / 目录 URL。新增 full 专用映射：

- `/a/b` → `a/b.md`
- `/a/b/` → `a/b/index.md`
- `/` → `index.md`
- 已有 `.md` / `.markdown` → 保持原路径
- 其他文件后缀 → 追加 `.md`，如 `/a.html` → `a.html.md`
- query 不进入路径；不同 URL 映射到同一路径 → 大声失败
- 继续拒绝空 segment / `.` / `..` / 编码后路径逃逸

扩展 `PathRegistry` 支持注册“已计算路径”，复用既有碰撞检测，MUST NOT 复制第二套 registry。

### 快照语义

既有 `Snapshot::new` 会恢复中断的递归抓取 partial；聚合文件每次提供完整真相，复用 partial 会残留已删除页面。

- 新增 `Snapshot::fresh(root, site)`：清理该站点旧 partial 后创建空 staging。
- `Index` 继续使用可恢复的 `Snapshot::new`。
- `Full` MUST 使用 `Snapshot::fresh`。
- 网络 / 解析 / 写入失败 → 保留旧站点目录，不 commit。

## 实现顺序

1. 先更新根 [`README.md`](../../README.md)：统一 `sync` 契约 + `llms-full.txt` 示例 + 自动分流行为。
2. `url_map.rs`：`EntryKind` / full 路径映射 / `PathRegistry` 复用接口 + 单测。
3. `snapshot.rs`：`Snapshot::fresh` + partial 清理测试。
4. `full.rs`：解析器 + 单次抓取 / 写入 + 单测。
5. `sync.rs`：按 `EntryKind` 分派；统一 commit / Git / 报告。
6. `main.rs`：注册 `full` 模块；`cli.rs` MUST NOT 新增命令。
7. `tests/e2e.rs`：混合 `llms.txt` / `llms-full.txt` 同步与失败回滚。

## 测试

- 入口识别：大小写 / query / fragment / 非 `llms-full.txt`。
- 解析：带/不带描述、首项无分隔符、正文 `---` / `URL:`、代码块伪标记、仅标题页面、重复 URL、损坏页头、无页面。
- 路径：无后缀 / 目录 / root / 已有 Markdown 后缀 / path traversal / query 冲突 / 跨 origin 同路径冲突。
- HTTP：2xx / 404 / 5xx / 非 UTF-8 / redirect；错误不替换旧快照。
- 快照：Full 中断 partial MUST NOT 被恢复；Index resume 行为不变。
- 混合站点：`llms-wiki sync` 可连续同步两种入口，分别提交成功站点；单站点失败不影响其他站点。
- 回归：既有 `crawler.rs` / CLI / e2e 测试全部保持通过。

真实验收：

```bash
llms-wiki site add deno https://docs.deno.com/llms-full.txt
llms-wiki sync deno
```

验收：生成页数 = 本次有效页头数；随机抽查 URL 路径 / 标题 / 描述 / 正文一致；再次同步无陈旧文件；无需 `sync-full`。

## 不做

- 不新增 subcommand / flag / site type 配置。
- 不从 `llms.txt` 自动切换到其链接的 `llms-full.txt`；站点入口由用户配置，避免悄然改变抓取范围。
- 不支持缺少可验证 `URL:` 页头的任意聚合格式。
- 首版不做聚合入口 ETag / Last-Modified；每次全量 GET + 全量重建，后续可在 `full.rs` 内独立优化。
