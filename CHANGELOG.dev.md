```When Editing
本文档作用: 面向开发者的发版记录; CHANGELOG.md 的超集, 1:1 镜像 + 技术变更子项
遵循 AGENTS.md 文档编写规范
- 每条主项 = CHANGELOG.md 对应条目 (原文), 下方缩进子项承载技术变更
- 子项 MAY 写路径 / 函数 / 机制; ≤ 1 行
```

# Changelog (developer, follow [CHANGELOG.md](./CHANGELOG.md))

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

[0.1.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.1
[0.1.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.0
