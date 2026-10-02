# `.md` 孪生页（append_md）

日期：2026-10-02

## 问题

`https://mole.fit/zh/blog/llms.txt` 的 109 个链接全是 HTML URL（如 `/zh/blog/x`），`is_syncable_url` 只认 `.md` / `.markdown` / `/llms.txt` -> `sync` 得 0 页。站点在索引里声明 "append .md to any article URL"，`/zh/blog/x.md` 返回 `text/markdown`。

## 决策

- 按站点开关 `append_md`（`site add --append-md`），默认关闭
- 不全局启用：实测 `bun-en` 入口的 `bun.com/blog` -> `blog.md` 200 且链接 179 篇 `.md` -> 整个 blog 被卷入；`openai-en` 7 个孪生页 404 -> 每次同步恒有 `missing`
- 不做「入口无 `.md` 链接即启用」的自动探测：远端索引新增一个 `.md` 链接即翻转 -> 快照替换静默删光全站
- 仅入口文档改写：mole 文章正文里的无后缀链接（`/zh/tested-apps/*` 等 27 个）均无 `.md` 版本，改写只产生 404
- 仅白名单 origin；改写结果 MUST NOT 进入 `declared_links` / `allow()`
- 仅 path 末段非空且无 `.`：跳过目录 URL（规范的 `index.html.md` 无统一形式）与 `.html` / `.txt`

## 实现

- `url_map::markdown_twin`：`path + ".md"`，query 保留
- `discovery::discover(.., append_md)`：非 syncable 链接取孪生页，再过白名单
- `CrawlOptions.append_md` 由 `sync_site` 按站点注入；`crawler` 仅入口文档分支传 `append_md && entry_set.contains(url)`
- `SiteConfig.append_md`：`Raw` 接收；序列化仅在 `true` 时写出 -> 现有配置字节不变；`llms-full.txt` 站点声明即报错
- `site list` 开启时追加 `\tappend-md` 列

## 验收

- 单测：孪生页规则 / 入口改写 + 内容页不改写 / 白名单不扩展 / 配置往返 / CLI 解析
- 实测：`sync mole-blog-zh` 下载索引内全部文章
