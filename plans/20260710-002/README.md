# sync 进度日志可读性改造

日期：2026-07-10

## 目标

`sync` 现在只有单行 spinner，`concurrency>1` 时用户看不清「进行到哪一步 / 现在在跑什么 / 各类结果多少」。
本计划让 `sync` 输出对人类可读：站点进度定位 + 分类计数 + 实时并发感 + 失败可追溯。

## 当前现状

进度模板（[`src/progress.rs`](../../src/progress.rs)）：`{spinner:.cyan} {prefix} {pos} {msg}`

- `{prefix}` = 站点名
- `{pos}` = 所有完成事件累计（下载 + 未变 + 缺失 + 忽略 + 失败），**无标签、无法区分类型**
- `{msg}` = 最新一次事件的完整 URL（`Started` 时为 `GET <url>`）

缺陷：

- 8 路并发只显示「最后触发事件的那一条」，看不到全局
- `{pos}` 裸数字无语义
- 完整 URL 塞进单行 → 被终端宽度截断 → 尾部反复闪烁
- 无站点序号 / 无耗时 / 无分类计数
- spinner 原地重绘，无滚动历史，滚回去看不到已完成记录
- 失败仅进最终 summary（`take(5)`），过程中无即时提示

现有事件与报告（**本计划不改**）：

- `CrawlEvent`：`Started` / `Downloaded` / `Unchanged` / `Missing` / `Ignored` / `Failed`（均只带 `url: String`）
- `CrawlReport`：`downloaded` / `unchanged` / `missing` / `ignored` / `failures`

## 边界约束（MUST）

- 【MUST】改动收敛到 [`src/progress.rs`](../../src/progress.rs) + [`src/sync.rs`](../../src/sync.rs)（+ flag 所需的 [`src/cli.rs`](../../src/cli.rs)、传参所需的 [`src/main.rs`](../../src/main.rs)）
- 【MUST NOT】改 `crawler.rs` / `http.rs` / `manifest.rs` —— 这三个文件当前有在途未提交改动（manifest/增量），碰它们会撞车
- 【MUST NOT】新增 `CrawlEvent` 变体或字段；用户诉求全部可由现有事件在 `SyncProgress` 内部推导
- 【MUST】保留非 TTY 最终文本摘要（indicatif 非 TTY 自动隐藏 spinner，但 `summary_line` 那条 `eprintln` MUST 保留）
- 【MUST NOT】伪造未知总量：不加百分比 / 进度条 / 假 total；分类计数与 inflight 都是真实已知值，合规
- 【MUST NOT】引入 `tracing` / 日志文件 / MultiProgress 每并发一行等复杂设施

## 行为契约

### 汇总行（单行，原地刷新，stderr）

新模板字段（顺序建议）：`[i/n] <site>  <spinner>  <elapsed>  dl=A unchanged=B miss=C fail=D  inflight=K  · <短路径>`

- `[i/n]` = 站点序号 / 站点总数（`sync.rs` 传入）
- `<elapsed>` = 本站点已耗时（indicatif `{elapsed}`）
- `dl/unchanged/miss/fail` = 分类计数，`SyncProgress` 内部各一个 `AtomicUsize`，对应完成事件 bump
- `inflight` = `Started` 计数 − 完成事件计数（= 实时在途请求数，体现并发；**零 crawler 改动**）
- `· <短路径>` = 最新事件 URL 的 path 部分（相对 origin），避免长 URL 截断闪烁；SHOULD，解析失败时回退原 URL

### 逐条持久日志（`bar.println`，滚动在 spinner 上方）

按 verbosity 分档：

<!-- prettier-ignore -->
| 档位 | 逐条输出 | spinner | 最终 summary |
| --- | --- | --- | --- |
| 默认 | 仅 `Failed`：`[site] FAIL <短路径>` | 有 | 有 |
| `-v` | 每个完成事件都打，带类型标签（`OK`/`UNCHANGED`/`MISS`/`IGNORED`/`FAIL`） | 有 | 有 |
| `--quiet` | 无 | 无 | 有 |

- 默认 MUST 静默成功：`concurrency=8 × 数百 URL`，逐条打成功会刷屏
- `Failed` 事件只带 URL，详细 message 仍由现有 `format_report_failure` 最终汇总给出（不为拿 message 去改 crawler）
- `-v` / `--quiet` 互斥

### 站点边界（`sync.rs`）

- 每站点开始前 SHOULD 打一条头：`── [i/n] <site> ──`（`--quiet` 除外）
- 结束仍由 `SyncProgress::finish` 打 `summary_line`

## 实现任务

1. **`cli.rs`**：`Sync` 子命令新增 `-v/--verbose` 与 `-q/--quiet`（bool，`conflicts_with`）；补解析测试（默认 both false / 互斥报错）
2. **`main.rs`**：将 verbosity 透传给 `sync::run`
3. **`sync.rs`**：
   - `run` 新增 verbosity 参数
   - 站点循环带 index → `SyncProgress::new(site, i, n, verbosity)`
   - 按档位打站点头
4. **`progress.rs`**：
   - `SyncProgress` 持有站点序号 / 总数 / verbosity + 各类 `AtomicUsize`（含 started、completed 分类）
   - 换模板为带标签汇总行；`event` 中更新计数 + inflight + 短路径，不再无脑覆盖整条 URL
   - `Failed`（默认）/ 所有完成事件（`-v`）走 `bar.println`
   - 抽出纯函数 `summary_msg(counts) -> String`（汇总行动态段）与短路径提取，便于单测
5. 保留并扩展 `summary_line`（已含 `unchanged`）

## 测试

- `cli.rs`：`--verbose` / `--quiet` 解析；二者同时给出报错
- `progress.rs`：
  - `summary_msg` 给定各计数 → 期望字符串（含标签）
  - 短路径提取：完整 URL → path；非法 URL → 原样回退
  - 现有 `formats_non_tty_summary` 保持通过
- 手动/集成：非 TTY（重定向 stderr 到文件）仍含最终 `summary_line`

## 验收

```bash
llms-wiki sync --concurrency 8            # 默认：汇总行含 [i/n]/分类计数/inflight，仅失败逐条
llms-wiki sync anthropic -v               # 每条完成带类型标签
llms-wiki sync --quiet                    # 仅最终 summary
llms-wiki sync 2> out.log                 # 非 TTY：out.log 末尾有 summary_line
```

- 汇总行字段含义自解释，无裸数字
- 长 URL 不再撑爆/闪烁单行
- 全局质量门槛通过：`cargo fmt --all -- --check` / `cargo clippy --all-targets --locked -- -D warnings` / `cargo test --locked` / `git diff --check`

## 本计划不做

- 不改 `crawler.rs` / `http.rs` / `manifest.rs`，不新增事件
- 不显示总进度百分比 / 总量 / queue 剩余（queue 深度需 plumb crawler，defer）
- 不上 MultiProgress 每并发一行
- 不引入结构化日志 / 日志文件
