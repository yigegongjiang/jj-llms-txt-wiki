```When Editing
本文档作用: 面向开发者的发版记录; CHANGELOG.md 的超集, 1:1 镜像 + 技术变更子项
遵循 AGENTS.md 文档编写规范
- 每条主项 = CHANGELOG.md 对应条目 (原文), 下方缩进子项承载技术变更
- 子项 MAY 写路径 / 函数 / 机制; ≤ 1 行
```

# Changelog (developer, follow [CHANGELOG.md](./CHANGELOG.md))

## [0.4.0] - 2026-07-10

### Added

- 同步跳过未变化文件：仅下载有更新的内容，同步摘要新增 `unchanged` 计数，显著减少带宽与耗时。
  - 站点目录内 `.llms-wiki.json` 存 `ETag`/`Last-Modified`；`If-None-Match`/`If-Modified-Since` 条件请求，`304` 复制旧快照文件并从其内容复用链接发现。

## [0.3.0] - 2026-07-10

### Added

- 输出目录自动初始化为独立 Git 仓库；每次有效同步提交全量变更，信息包含成功站点和 UTC 时间。
  - `sync` 抓取前验证独立 Git root，成功快照后执行 `git add -A` + 单次 commit。
- 内容未变化仍记录同步事件；多站点部分失败仍保存成功站点历史。
  - `--allow-empty` 保留事件；Git 初始化失败前置阻断，部分成功提交后仍返回站点错误。

## [0.2.1] - 2026-07-10

### Changed

- 跟随版本同步发布。
  - Artifact upload v4 → v7 / download v4 → v8，切换原生 Node 24 runtime。

## [0.2.0] - 2026-07-10

### Added

- 支持 TOML 配置及多站点添加、排序查看。
  - `clap` + `serde` + `toml` 实现嵌套 CLI、默认配置、校验及同目录原子写入。
- 支持递归同步同源 Markdown，并按 URL 层级生成完整本地快照。
  - Comrak AST + URL 安全映射 + Tokio/reqwest rustls 抓取器递归发现并分类响应。
- 支持临时覆盖并发数与请求间隔；失败时保留上一份完整快照。
  - 全局启动节流 + 并发上限 + 同文件系统临时目录替换/回滚 + indicatif 摘要。

## [0.1.1] - 2026-07-10

### Changed

- 跟随版本同步发布。
  - CI `actions/checkout` v4 → v6；`Cargo.toml` 版本 0.1.0 → 0.1.1。

## [0.1.0] - 2026-07-10

### Added

- 提供 `llms-wiki` 命令行骨架，支持 `curl` 一键安装（macOS arm64/x64）。
  - Rust 2024 edition，手写参数分派（`src/main.rs`）。
  - `release.yml`：`v*` tag 触发，校验 tag==版本，构建双架构 + `checksums.txt` + `gh release create`。
  - `install.sh` 从 latest Release 下载并校验 sha256，装入 `~/.local/bin`。
- 内置 `help` / `version` / `update` / `uninstall` 生命周期命令，可自更新与卸载，并校验下载校验和。
  - `update` 下载 latest 资产，比对 `checksums.txt`，`fs::rename` 原子替换二进制。

[0.4.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.4.0
[0.3.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.3.0
[0.2.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.1
[0.2.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.0
[0.1.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.1
[0.1.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.0
