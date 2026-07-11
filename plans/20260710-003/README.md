# 跨 origin 入口兼容改造

日期：2026-07-12

## 目标

修复入口 `llms.txt` 所在 host 与其内容链接 host 不一致的站点：`bun.sh/docs/llms.txt` 入口把全部 `.md` 链接指向 `bun.com`，旧的「与入口 URL 严格同源」过滤把每条链接判为不同源丢弃，结果 0 下载、阻断性不可用。

## 当前现状（改造前）

- `url_map::same_origin(entry, candidate)`：scheme + host + effective port 三者全等才算同源。
- `discovery::discover`：逐链接 `same_origin(entry, url)` 过滤。
- `http::HttpClient`：重定向策略与响应后校验都以入口 origin 判定，非同源重定向记 `IgnoredRedirect`。

缺陷：入口 host ≠ 内容 host 时（`bun.sh` vs `bun.com`，且非同一 registrable domain，eTLD+1 匹配无效），唯一能把两者关联的信号是「入口文档自己声明的链接」，旧模型无法利用。

## 设计

信任模型：用户显式配置入口 → 入口 `llms.txt` 是站点清单，声明其内容所在 origin → 信任入口声明的 origin。

- 抓取白名单 = {入口 origin} ∪ {入口文档中全部 syncable 链接的 origin}，入口抓取后**冻结**。
- **仅入口文档**扩展白名单；内容页（`.md`）只能引用白名单内 origin，不能引入新 origin（防止顺链接爬遍全网 + 保证确定性）。
- discovery 过滤与 HTTP 重定向策略统一按白名单判定。

## 实现

- `url_map`：新增 `AllowedOrigins`（`Arc<Mutex<HashSet<String>>>`，键 = `Url::origin().ascii_serialization()`，默认端口归一）与 `origin_key`；删除 `same_origin`。
- `discovery`：抽出私有 `syncable_links` 复用解析；`discover(md, base, entry, allowed)` 按 `allowed.contains` 过滤；新增 `declared_links(md, base)` 供入口扩展。
- `http`：`HttpClient` 持有 `AllowedOrigins` 替代 `entry`，重定向闭包与响应后校验改按 `contains` 判定。
- `crawler`：`crawl` 建 `AllowedOrigins::new(&entry)` 传入 client；入口文档（`item.url == canonical_entry`）下载后、`enqueue_discovered` 之前（其间**无 await**）以 `declared_links` 扩展并冻结。

## 并发正确性（命门）

入口是初始队列唯一项：iteration 1 只 spawn 入口，join 后在 Document 分支扩展白名单；任何内容 fetch（含其重定向闭包读白名单）都到 iteration 2 才 spawn，必在冻结之后。故扩展 MUST 严格早于 `enqueue_discovered` 且中间不插入 await。

## 验证

- 单测：`crawler` 双 server 回归（入口 host A、链接 host B → B 被下载；内容页引用的第三 origin 不可达）；`url_map` / `discovery` 白名单扩展用例。
- 守门测试原样保持绿：入口只声明同源链接时白名单只含入口 origin，跨 origin 链接/重定向仍被忽略。
- 真实站点：`bun.sh/docs/llms.txt` 同步 downloaded=315、failed=0。

## 不在本次范围（既有行为，非回退）

- 入口 URL 本身跨 origin 302（bun 是 200 直出）。
- 嵌套 `llms.txt` 索引引入新 origin 被丢弃（仅入口扩展白名单）。
- 多 origin 同 path 映射冲突：沿用既有 host 无关路径映射，冲突时报错（大声失败，非静默错误）。
