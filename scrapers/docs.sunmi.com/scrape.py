"""Scrape docs.sunmi.com/zh-CN documentation as Markdown.

Layout:
  scrapers/docs.sunmi.com/
    scrape.py     <- this script
    links.txt     <- discovered URL list
  <wiki>/sunmi-zh/docs/zh-CN/...   <- markdown output, mirrors URL path

Pipeline:
  1) Playwright opens each top-level category page, expands the AntD Tree
     sidebar, and collects every /zh-CN/... link.
  2) crawl4ai renders each URL headlessly and dumps Markdown into the wiki
     site directory under a URL-mirroring directory tree.
"""

import asyncio
import re
import sys
import tomllib
from pathlib import Path
from urllib.parse import urljoin, urlparse

from crawl4ai import AsyncWebCrawler, CacheMode, CrawlerRunConfig
from playwright.async_api import async_playwright

HOST = "docs.sunmi.com"
PATH_PREFIX = "/zh-CN/"
BASE_DIR = Path(__file__).resolve().parent
LINKS_FILE = BASE_DIR / "links.txt"

SITE_NAME = "sunmi-zh"
CONFIG_FILE = Path.home() / ".config/jj-llms-txt-wiki/config.toml"
DEFAULT_WIKI_ROOT = Path.home() / ".config/jj-llms-txt-wiki/wiki"


def wiki_site_dir() -> Path:
    """Resolve <output_dir>/<SITE_NAME>/docs, honouring the CLI config file."""
    root = DEFAULT_WIKI_ROOT
    if CONFIG_FILE.exists():
        configured = tomllib.loads(CONFIG_FILE.read_text(encoding="utf-8")).get("output_dir")
        if isinstance(configured, str) and configured:
            root = Path(configured).expanduser()
    return root / SITE_NAME / "docs"


OUT_DIR = wiki_site_dir()

SEED_ENTRIES = [
    "https://docs.sunmi.com/zh-CN/ciczeghjk557/xdxmeghjk546",  # 首页
    "https://docs.sunmi.com/zh-CN/cdixeghjk491/xmafeghjk535",  # 集成开发指南
    "https://docs.sunmi.com/zh-CN/ceghjk502/fcmeghjk546",       # 硬件产品
    "https://docs.sunmi.com/zh-CN/cicmeghjk546/xmdmeghjk546",   # 软件产品手册&公告
]

EXPAND_AND_COLLECT_JS = """
async () => {
  const sleep = (ms) => new Promise(r => setTimeout(r, ms));
  for (let i = 0; i < 60; i++) {
    const closed = document.querySelectorAll('.ant-tree-switcher_close');
    if (closed.length === 0) break;
    closed.forEach(el => el.click());
    await sleep(180);
  }
  const anchors = document.querySelectorAll('.ant-tree a[href]');
  return Array.from(anchors, a => a.getAttribute('href'));
}
"""


def normalize(href: str, base: str) -> str | None:
    if not href:
        return None
    absu = urljoin(base, href).split("#", 1)[0].rstrip("/")
    u = urlparse(absu)
    if u.netloc != HOST or not u.path.startswith(PATH_PREFIX):
        return None
    return absu


def url_to_relpath(url: str) -> Path:
    parts = [seg for seg in urlparse(url).path.strip("/").split("/") if seg]
    if not parts:
        return Path("index.md")
    safe = [re.sub(r"[^A-Za-z0-9._-]", "-", seg)[:120] for seg in parts]
    return Path(*safe[:-1]) / (safe[-1] + ".md")


FOOTER_MARKERS = (
    "该文档是否有帮助",
    "我们使用cookies",
    "© 2026 上海商米",
    "版权所有",
    "沪ICP备",
    "反诈劝阻电话",
)


def clean_body(md: str) -> str:
    """Trim sunmi navigation shell: keep only the article body."""
    lines = md.splitlines()

    start = 0
    for i, line in enumerate(lines):
        if line.startswith("更新时间："):
            for j in range(i - 1, -1, -1):
                if lines[j].startswith("# "):
                    start = j
                    break
            break

    end = len(lines)
    for i in range(start + 1, len(lines)):
        if any(m in lines[i] for m in FOOTER_MARKERS):
            end = i
            break

    return "\n".join(lines[start:end]).strip()


async def discover() -> list[str]:
    if LINKS_FILE.exists():
        cached = [l.strip() for l in LINKS_FILE.read_text(encoding="utf-8").splitlines() if l.strip()]
        if cached:
            print(f"[discover] using cached {LINKS_FILE.name} ({len(cached)} URLs)", flush=True)
            return sorted(set(cached))

    found: set[str] = set(SEED_ENTRIES)
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await (await browser.new_context()).new_page()
        for e in SEED_ENTRIES:
            print(f"[discover] {e}", flush=True)
            try:
                await page.goto(e, wait_until="networkidle", timeout=60000)
                hrefs = await page.evaluate(EXPAND_AND_COLLECT_JS)
            except Exception as exc:
                print(f"  ! {exc}", flush=True)
                continue
            new = 0
            for h in hrefs or []:
                nu = normalize(h, e)
                if nu and nu not in found:
                    found.add(nu)
                    new += 1
            LINKS_FILE.write_text("\n".join(sorted(found)) + "\n", encoding="utf-8")
            print(f"  + {new} new (total {len(found)})", flush=True)
        await browser.close()
    return sorted(found)


WAIT_FOR_HYDRATION_JS = """js:() => {
  const t = document.querySelector('.tiptap.ProseMirror');
  return !!(t && t.innerText.trim().length > 5);
}"""

POST_HYDRATION_JS = [
    """(async () => {
      const start = Date.now();
      while (Date.now() - start < 10000) {
        const t = document.querySelector('.tiptap.ProseMirror');
        if (t && t.innerText.trim().length > 5) break;
        await new Promise(r => setTimeout(r, 200));
      }
      const step = 800;
      for (let y = 0; y <= document.documentElement.scrollHeight; y += step) {
        window.scrollTo(0, y);
        await new Promise(r => setTimeout(r, 200));
      }
      window.scrollTo(0, 0);
      const imgs = [...document.querySelectorAll('.tiptap img')];
      await Promise.all(imgs.map(img => (img.complete && img.naturalWidth > 0) ? null : new Promise(r => {
        img.addEventListener('load', r, { once: true });
        img.addEventListener('error', r, { once: true });
        setTimeout(r, 3000);
      })));
    })()"""
]


async def crawl(urls: list[str]) -> int:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    cfg = CrawlerRunConfig(
        cache_mode=CacheMode.BYPASS,
        verbose=False,
        stream=True,
        wait_for=WAIT_FOR_HYDRATION_JS,
        page_timeout=45000,
        js_code=POST_HYDRATION_JS,
        delay_before_return_html=1.5,
    )
    saved = 0
    total = len(urls)
    async with AsyncWebCrawler() as crawler:
        async for r in await crawler.arun_many(urls=urls, config=cfg):
            if not r.success:
                print(f"[skip] {r.url} :: {r.error_message}", flush=True)
                continue
            md = clean_body(str(r.markdown or ""))
            if not md:
                print(f"[empty] {r.url}", flush=True)
                continue
            path = OUT_DIR / url_to_relpath(r.url)
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(f"---\nurl: {r.url}\n---\n\n{md}\n", encoding="utf-8")
            saved += 1
            print(f"[{saved}/{total}] {r.url}", flush=True)
    return saved


FRONTMATTER_RE = re.compile(r"^---\nurl: (.+?)\n---\n\n(.*)$", re.DOTALL)


def clean_existing_files() -> int:
    changed = 0
    for p in OUT_DIR.rglob("*.md"):
        text = p.read_text(encoding="utf-8")
        m = FRONTMATTER_RE.match(text)
        if not m:
            continue
        url, body = m.group(1), m.group(2)
        cleaned = clean_body(body)
        if not cleaned:
            continue
        new = f"---\nurl: {url}\n---\n\n{cleaned}\n"
        if new != text:
            p.write_text(new, encoding="utf-8")
            changed += 1
    return changed


async def main() -> None:
    if "--clean-only" in sys.argv:
        n = clean_existing_files()
        print(f"cleaned {n} files under {OUT_DIR}/")
        return

    urls = await discover()
    LINKS_FILE.write_text("\n".join(urls) + "\n", encoding="utf-8")
    print(f"discovered {len(urls)} URLs -> {LINKS_FILE}")
    n = await crawl(urls)
    print(f"done. saved {n} files under {OUT_DIR}/")


if __name__ == "__main__":
    asyncio.run(main())
