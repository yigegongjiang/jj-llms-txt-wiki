```When Editing
本文档作用: scrapers/ 子工程说明 (非 llms.txt 站点的抓取脚本)
MUST NOT 写 Rust CLI 相关内容 (→ 根 README.md) / 发布流程 (→ workflow.md)
遵循 AGENTS.md 文档编写规范
- 每新增一个站点目录, 在「站点」段加一条: 目录名 + 一句用途 + 运行命令
```

# scrapers

无 `llms.txt` 的站点用这里的脚本抓取，产物写入与 Rust CLI 相同的 wiki 目录，供 qmd 统一索引。

- Rust CLI (`jj-llms-txt-wiki sync`) 只管配置文件里的 `llms.txt` / `llms-full.txt` 站点，MUST NOT 触碰这里生成的站点目录。
- 输出根从 `~/.config/jj-llms-txt-wiki/config.toml` 的 `output_dir` 读取，缺省 `~/.config/jj-llms-txt-wiki/wiki`。
- 每个站点一个目录：`<wiki>/<site>/docs/<URL path>.md`；每篇 Markdown 顶部 YAML frontmatter 记录原始 URL。
- 站点名沿用 wiki 约定 `<slug>-<lang>`。

## 目录结构

```text
scrapers/
├── pyproject.toml        uv 项目 (crawl4ai + trafilatura)
├── uv.lock
└── docs.sunmi.com/       站点专属目录 (domain 命名)
    ├── scrape.py         该站专用抓取脚本
    └── links.txt         发现的 URL 列表 (可复用 / 断点续跑)
```

## 环境

```bash
uv sync --project scrapers                          # 安装 Python 依赖
uv run --project scrapers playwright install chromium   # headless 浏览器 (首次)
```

- `crawl4ai` + `playwright`：JS 渲染站点（SPA / Next.js RSC 等）的 headless 抓取 + Markdown 输出
- `trafilatura`：静态 HTML 的正文提取，备用；SPA 站点无 JS 引擎抓不到正文

## 站点

新增站点后注册 qmd 索引（collection 名 = 站点目录名）：

```bash
qmd collection add ~/.config/jj-llms-txt-wiki/wiki/<site> --name <site>
qmd embed -c <site>
```

产物写入 wiki 后自行提交：`git -C <wiki> add -- <site> && git -C <wiki> commit -m "chore(scrape): <site> @ <UTC>"`。

### docs.sunmi.com -> `sunmi-zh`

商米开发者文档（SPA，AntD Tree 侧栏），306 篇 Markdown。

```bash
uv run --project scrapers python scrapers/docs.sunmi.com/scrape.py                # discover + crawl + 清理
uv run --project scrapers python scrapers/docs.sunmi.com/scrape.py --clean-only   # 只清理已有文件, 不重爬
```

流程：Playwright 打开 4 个顶级 category 入口展开侧栏收集 `/zh-CN/...` 链接 -> crawl4ai 逐 URL headless 渲染转 Markdown -> 去除导航壳 / 侧栏 / footer 噪音。

删除 `links.txt` 触发下次重新 discover；保留则跳过 discover 直接 crawl。

## trafilatura CLI（静态站备用）

```bash
uv run --project scrapers trafilatura --markdown -u "<URL>" > out.md       # 单页
uv run --project scrapers trafilatura --markdown -i links.txt -o out/      # 批量
uv run --project scrapers trafilatura --sitemap "<URL>" --markdown -o out/ # 走 sitemap
```
