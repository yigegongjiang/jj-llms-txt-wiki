# 阶段 3：URL 规则与 Markdown 发现

## 目标

实现纯函数化的链接发现、URL 去重、同源过滤和本地路径映射，为异步抓取提供明确且可测试的安全边界。

## 前置状态

- 阶段 2 已通过验收。
- 站点入口 URL 和输出目录已经过配置校验。

## 行为契约

- 使用 `url` 解析和 join 相对链接，不手写 URL 拼接。
- origin 必须与入口 URL 的 scheme、host、effective port 全部一致。
- 只提取 Comrak AST 中的普通 Markdown link；图片、代码块、HTML、纯文本 URL 不进入队列。
- 相对链接以当前文档最终 URL 为 base 解析。
- 只接受 HTTP/HTTPS 且 path 以 `.md` 或 `.markdown` 结尾的候选链接，扩展名大小写不敏感。
- canonical URL 删除 fragment，保留 query；canonical URL 用于去重。
- 入口 `llms.txt` 不作为输出文档，指回入口的链接也不进入快照。
- 本地相对路径由 URL path 去掉开头 `/` 得到，保留百分号编码和目录层级。
- 空路径、目录路径、`.`、`..`、路径逃逸和无法表示为单个安全文件路径的 URL 被拒绝。
- 两个不同 canonical URL 映射到同一路径时返回冲突错误，不能以完成顺序决定覆盖结果。
- 不同源、非 Markdown、无法解析或带不支持 scheme 的链接直接忽略，不视为同步失败。

## 实现任务

- 在 `Cargo.toml` 添加 `comrak`。
- 新增 `src/discovery.rs`：
  - 解析 CommonMark/GFM AST。
  - 遍历 `NodeValue::Link`。
  - 解析相对/绝对目标并过滤候选 URL。
  - 返回去重后的 canonical URL 集合。
- 新增 `src/url_map.rs`：
  - `same_origin(entry, candidate)`。
  - `canonicalize(url)`。
  - `is_markdown_url(url)`。
  - `local_path(url)`。
  - 输出路径冲突检测。
- 类型边界中区分入口 URL、canonical URL 和本地相对路径，避免在调用点重复校验。
- 所有文件写入前再次以 `starts_with(site_temp_dir)` 断言目标仍在临时站点目录内。

建议阶段末结构：

```text
src/
├── discovery.rs
├── url_map.rs
└── ...
```

## 测试

- 绝对链接、相对链接、`../`、root-relative URL 和 fragment。
- 默认端口等价与 scheme/host/port 任一不同的跨源情况。
- `.md`、`.MARKDOWN`、非 Markdown、图片、代码块、HTML link。
- 重复链接和仅 fragment 不同的链接去重。
- query 保留，以及两个 query 变体导致本地路径冲突。
- `/docs/api/messages.md` 映射为 `docs/api/messages.md`。
- `%2F`、`%2E%2E`、Unicode 百分号编码和路径逃逸用例。
- 链接指回入口 `llms.txt` 时被忽略。

## 验收

- 链接发现和路径映射测试不启动网络，也不读写真实用户目录。
- 任意接受的本地相对路径都无法逃逸站点临时目录。
- 同一 Markdown 输入多次解析得到相同 URL 集合，不依赖 AST 遍历之外的并发顺序。
- `cargo test --locked` 及全局质量门槛通过。

## 本阶段不做

- 不发 HTTP 请求，不处理状态码和重定向。
- 不提交站点快照。
- 不改写下载后的 Markdown 内部链接。
