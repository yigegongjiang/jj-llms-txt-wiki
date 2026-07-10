# 阶段 4：异步递归抓取引擎

## 目标

实现单站点抓取：读取入口 `llms.txt`，递归发现同源 Markdown，在并发和启动间隔约束下下载到调用方提供的临时目录，并返回可判定是否提交的结果。

## 前置状态

- 阶段 3 已通过验收。
- URL 身份、同源判断、链接过滤和本地路径均由已测试模块提供。

## 行为契约

- 使用 Tokio 多线程 runtime 和异步 `reqwest` client，TLS backend 为 `rustls`。
- 抓取入口 `llms.txt` 后只保存其发现的 Markdown，不保存入口本身。
- 中央队列保存未处理 canonical URL；`seen` 集合保证每个 URL 每次同步最多入队一次。
- 同时在途请求不超过 `concurrency`；参数为 `0` 在配置阶段直接拒绝。
- 所有顶层请求共享启动节流器：首个立即启动，后续启动时间至少相隔 `interval`；调度延迟不能形成补发突发。
- `reqwest` 仅跟随同源重定向，并设置有限跳数；不同源重定向停止并记为 ignored。
- 设置固定请求超时，超时属于未确定错误；首版不自动重试。
- 响应分类：
  - `2xx`：读取 UTF-8 Markdown、写入临时目录、继续发现链接。
  - `404`/`410`：记为 missing，不写文件，继续队列。
  - 不同源重定向：记为 ignored，不写文件，继续队列。
  - `429`、`5xx`、其他 `3xx`/`4xx`、网络/超时、重定向循环、无效 UTF-8、解析和文件错误：记为 failure。
- 已启动任务必须安全收敛；队列结束后只要有 failure，站点结果就是失败。
- 写文件前创建父目录；同一路径冲突在写入前失败。

## 实现任务

- 在 `Cargo.toml` 添加：
  - `tokio`：macros、multi-thread runtime、sync、time。
  - `reqwest`：关闭默认 TLS，启用 `rustls-tls`。
  - 轻量 duration parser，用于 CLI 的 `500ms`、`1s` 等输入。
- 新增 `src/http.rs`：
  - 构建带 User-Agent、超时、同源 redirect policy 的 client。
  - 将 transport/HTTP 响应转换为明确分类。
- 新增 `src/crawler.rs`：
  - 动态队列、`seen`、在途任务集合和节流器。
  - 下载、写入、递归发现和聚合 `CrawlReport`。
- 将 `main.rs` 切换为 Tokio async 入口。
- 增加 `sync <site> --concurrency <n> --interval <duration>` 分派，
  阶段内只接单个指定站点；全站点默认行为在阶段 5 完成。
- 抓取器接受目标临时目录参数，不负责替换正式快照。

## 测试

使用进程内本地 HTTP server，禁止依赖公共网络：

- 入口发现两层以上 Markdown，包含相对链接、重复链接和环。
- 并发峰值不超过配置值。
- 请求启动时间满足 interval 下限；慢响应仍允许达到并发上限。
- 同源重定向成功；不同源重定向不请求目标。
- `404`/`410` 继续，`429`/`500`/超时使报告失败。
- 失败与成功并存时所有已启动任务正常结束，报告保留完整错误上下文。
- 下载目录与 URL path 一致，入口 `llms.txt` 不落盘。
- 非 UTF-8 响应和路径冲突不会产生部分覆盖。

## 验收

- `sync <site>` 可对本地 fixture 完成递归抓取。
- 抓取报告至少包含 downloaded、missing、ignored、failed 数量和失败 URL。
- 任一 failure 导致命令返回非零。
- 不同源 URL 从未被发起请求。
- 临时目标目录之外没有新增文件。
- 全局质量门槛通过。

## 本阶段不做

- 不替换正式站点目录，不删除旧快照。
- 不实现 `sync` 全部站点或 `indicatif` 进度。
- 不实现缓存、断点续传、增量同步或重试策略。
