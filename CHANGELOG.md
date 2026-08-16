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

## [0.21.1] - 2026-08-16

### Fixed

- 上一次同步被中断后重跑，不再把远端已删除的页面、或旧命名规则留下的文件混进新快照。

## [0.21.0] - 2026-08-16

### Fixed

- 同一路径靠 query 区分内容的页面（如 `…/developer-commands.md?surface=cli` 与 `?surface=ide`）不再判定为路径冲突导致整站失败，两份内容各自落盘。

### Changed

- 带 query 的页面文件名改为在原名后追加 `__<query>`（如 `developer-commands__surface=ide.md`）；不带 query 的文件名不变，首次同步会重命名受影响的少量文件。

## [0.20.0] - 2026-07-29

### Added

- 单个站点现在可以配置多个入口 URL（配置里写 `urls = [...]`，或 `site add <名称> <URL> <URL> …`），适合把一个文档站按 section 拆开的多份 `llms.txt` / `llms-full.txt` 合并到同一个目录。
- 同一站点的入口必须同类型（不能把 `llms.txt` 和 `llms-full.txt` 混在一起），入口 URL 不能重复；配置里字段名拼错会直接报错，不再静默把站点变成零入口。

### Changed

- 原有的 `url = "…"` 单入口写法完全不变，升级后无需改动任何配置。

## [0.19.0] - 2026-07-29

### Added

- 新增对「标题 + 下一行裸 URL」这类聚合文档的支持（如 HuggingFace 各文档站），此前会因找不到 `URL:` 标记而整站失败；现在一次请求即可拆出全部页面，比逐页下载快得多。

## [0.18.2] - 2026-07-28

### Fixed

- fork 之后不再需要任何配置：快照镜像的推送目标改为构建时自动取源码仓库的 `origin`，谁 clone 谁构建就推到自己的仓库，不会推向上游。

## [0.18.1] - 2026-07-28

### Fixed

- 本机登录多个 GitHub 账号时，同步结束的快照镜像推送不再因为用错身份被拒而跳过：按推送目标的仓库归属方选取对应账号的凭证，且不读写、不改动本机的活跃账号。

## [0.18.0] - 2026-07-28

### Added

- 超时 / 连接错误 / `429` / `5xx` 现在自动重试（单个地址最多 3 次请求，尊重服务端的 `Retry-After`），偶发抖动不再让整站同步作废。

### Changed

- 少量内容页在重试后仍失败时不再阻塞整站：快照照常发布，这些页面标记为「降级」——有上次的副本就沿用，没有就本次缺失。末行显示黄色 `✓ N/N synced · M degraded`，退出码 0，明细进运行日志。
- 失败页超过 `max(3, 已处理页数 / 100)` 才判定为上游故障，整站失败并保留上一次完整快照（原先一页失败即如此）。

### Fixed

- 上游文档索引链接着自己已经渲染不出的页面（例如把 `404` 错报成 `500`）时，站点不再永久卡在「同步失败」而始终无法更新。

## [0.17.0] - 2026-07-26

### Changed

- 一键安装命令的地址改为 `…/main/scripts/install.sh`，旧地址不再可用；已安装用户可继续用 `update` 自更新，不受影响。

## [0.16.0] - 2026-07-26

### Changed

- `concurrency` 现在是真实同时在途的下载任务数：队列有货时始终打满，不再被请求间隔压成串行。
- `interval` 语义改为「单个下载任务完成后该槽位的休息时长」，只影响该槽位，不再是全局发起间隔。
- 默认值改为 `concurrency = 8` / `interval_ms = 100`，并新增上限 64；超过上限自动收敛并提示（旧配置里的大数值在新语义下会打爆连接）。
- 同步进度行的在途数改为 `inflight=<在途>/<并发上限>`，可直接看出槽位是否打满。

### Fixed

- 修复在途请求数偶发超过并发上限的显示错误（如 `inflight=9/8`）。

## [0.15.0] - 2026-07-17

### Changed

- 默认 `output_dir` 由 `~/jj-llms-txt-wiki` 改为 `~/.config/jj-llms-txt-wiki/wiki`，与 `config.toml` 同一配置目录、以 `wiki/` 子目录分隔数据。
- 升级方式：直接把旧数据目录中的站点子目录与 `.git` 移动到新位置即可（不保留旧位置兼容）。

## [0.14.0] - 2026-07-17

### Changed

- CLI 命令由 `llms-wiki` 更名为 `jj-llms-txt-wiki`，配置目录改为 `~/.config/jj-llms-txt-wiki/config.toml`，默认输出目录改为 `~/jj-llms-txt-wiki/`。
- 站点内 manifest 文件由 `.llms-wiki.json` 更名为 `.jj-llms-txt-wiki.json`，运行日志目录由 `.llms-wiki/` 更名为 `.jj-llms-txt-wiki/`。
- 环境变量 `LLMS_WIKI_PUSH_URL` 更名为 `JJ_LLMS_TXT_WIKI_PUSH_URL`。
- 升级方式：重装 `curl … install.sh | bash` 拉取新二进制；旧配置与数据目录需手动迁移。

## [0.13.1] - 2026-07-12

### Fixed

- 超大文件剔除现在也覆盖通过 `304` 条件请求复用的旧文件：升级前已缓存的超大页面不再被无限期保留，会在剔除时一并从快照移除。

## [0.13.0] - 2026-07-12

### Added

- 递归抓取（`llms.txt`）时，单个内容页超过 3 MiB 视为异常并主动剔除：不写入本地、不参与后续递归，仅计数并记入运行日志；入口文档与 `llms-full.txt` 聚合文件不受此限。

## [0.12.1] - 2026-07-12

### Added

- 每次 `sync` 结束固定打印一行末行总账：全部成功显示 `✓ N/N synced`，存在失败显示 `✗ N ok · M failed · K error` 并附日志路径、以非零状态退出，使个别站点失败不再淹没在成功输出中。

## [0.12.0] - 2026-07-12

### Added

- 支持将 `llms-full.txt` 直接添加为站点入口；仍使用统一 `sync`，自动按内嵌 URL 拆分并保留文档目录层级。

### Changed

- 聚合文件每次从空快照完整重建；格式损坏、URL 重复或路径冲突时保留上一份完整快照。

## [0.11.0] - 2026-07-12

### Removed

- 精简 `sync` 参数：移除 `-v/--verbose` 与 `-q/--quiet`。默认已实时显示进度并逐条打印失败，管道 / 重定向下自动隐藏动态刷新，无需额外开关。
- 移除 `version` 子命令，改用标准 `--version`（`-V`）；`llms-wiki version` 不再可用。

## [0.10.0] - 2026-07-12

### Added

- 同步失败时给出可读的结果报告：按站点列出失败的文件与原因、404/410 缺失数量，并提示重试命令，不再把错误挤成一行。
- 每次同步把完整结果（各站计数、全部失败原因、缺失链接）写入 `<输出目录>/.llms-wiki/last-run.log`，跑完仍可回看；该文件不进版本库。

### Changed

- 整站失败（如入口不可达）现在直接显示真实原因，不再出现 `failed` 却 `failed=0` 的空结果。
- 失败报告在 `-q/--quiet` 下仍会显示，错误绝不会被静默吞掉。

## [0.9.0] - 2026-07-12

### Fixed

- 修复入口 `llms.txt` 所在 host 与内容链接 host 不一致的站点（如 `bun.sh` 入口、内容在 `bun.com`）一个文件都下载不了：现在信任入口文档声明的各 origin 并纳入抓取范围。

## [0.8.0] - 2026-07-12

### Added

- 同步日志按状态着色：下载 / 未变 / 缺失 / 失败等分类和最终结果分色显示，成功与失败一眼可辨。
- 输出到管道或文件时自动保持纯文本，不写入颜色码。

## [0.7.0] - 2026-07-11

### Added

- 同步完成后自动把数据仓库快照推送到源码仓库的 `wiki-data` 分支；无访问权限或未联网时静默跳过，不影响同步结果。

## [0.6.3] - 2026-07-10

### Changed

- 每个站点单独提交一个 commit，并发运行不同站点的同步不再互相锁死；提交前遇到外部工具短暂占用会自动重试。

## [0.6.2] - 2026-07-10

### Changed

- 默认并发数 4 → 1000，默认请求间隔 500ms → 50ms；同步更快，节流以请求间隔为准。

## [0.6.1] - 2026-07-10

### Fixed

- 修复站点的 llms.txt 仅链接到下级 llms.txt 索引（如 Cloudflare）时同步一个文件都不下载：现在会跟进 llms.txt 索引并抓取其中的文档。

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

[0.12.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.12.1
[0.12.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.12.0
[0.10.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.10.0
[0.9.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.9.0
[0.8.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.8.0
[0.7.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.7.0
[0.6.3]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.3
[0.6.2]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.2
[0.6.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.1
[0.6.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.6.0
[0.5.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.5.0
[0.4.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.4.0
[0.3.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.3.0
[0.2.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.1
[0.2.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.2.0
[0.1.1]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.1
[0.1.0]: https://github.com/yigegongjiang/jj-llms-txt-wiki/releases/tag/v0.1.0
