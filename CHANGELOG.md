```When Editing
本文档作用: 面向使用者的发版记录; 只写用户感受得到的变化, MUST NOT 写技术细节 (→ CHANGELOG.dev.md)
遵循 AGENTS.md 文档编写规范
- 写: 新功能 / 行为修复 / 体验 / 安全 / 命令迁移
- MUST NOT 写: 文件路径 / 函数名 / 组件名 / 依赖包名 / 重构细节
- 单条 ≤ 2 行, 单版本 ≤ 5 条; 段落: Added / Changed / Fixed / Removed / Security
- 无用户可感知变化 → 占位: `跟随版本同步发布`
```

# Changelog

[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) + [SemVer](https://semver.org/).

## [0.6.0] - 2026-07-10

### Added

- 同步中断可续传：手动中断或异常退出后，下次同步自动复用已下载的文件，只补未完成部分，不重复下载、不重发已完成请求。

### Fixed

- 写文件改为原子操作，中断不再留下写了一半的文件。

## [0.5.0] - 2026-07-10

### Added

- 同步进度更清晰：单行汇总实时显示站点序号、分类计数、在途请求数和耗时。
- 新增 `-v/--verbose`（逐条打印）与 `-q/--quiet`（仅摘要）控制进度详略。

### Changed

- 默认仅将失败逐条打印，成功保持安静；进度不再输出无标签数字和易截断的完整链接。

## [0.4.0] - 2026-07-10

### Added

- 同步跳过未变化文件：仅下载有更新的内容，同步摘要新增 `unchanged` 计数，显著减少带宽与耗时。

## [0.3.0] - 2026-07-10

### Added

- 输出目录自动初始化为独立 Git 仓库；每次有效同步提交全量变更，信息包含成功站点和 UTC 时间。
- 内容未变化仍记录同步事件；多站点部分失败仍保存成功站点历史。

## [0.2.1] - 2026-07-10

跟随版本同步发布。

## [0.2.0] - 2026-07-10

### Added

- 支持 TOML 配置及多站点添加、排序查看。
- 支持递归同步同源 Markdown，并按 URL 层级生成完整本地快照。
- 支持临时覆盖并发数与请求间隔；失败时保留上一份完整快照。

## [0.1.1] - 2026-07-10

跟随版本同步发布。

## [0.1.0] - 2026-07-10

### Added

- 提供 `llms-wiki` 命令行骨架，支持 `curl` 一键安装（macOS arm64/x64）。
- 内置 `help` / `version` / `update` / `uninstall` 生命周期命令，可自更新与卸载，并校验下载校验和。

[0.5.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.5.0
[0.4.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.4.0
[0.3.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.3.0
[0.2.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.1
[0.2.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.0
[0.1.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.1
[0.1.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.0
