# Mole CLI 和 Mac 版，该选哪个

> 命令行版免费开源，Mac 版提供磁盘地图、实时监控与系统控制。看清两者的共同点、安全差异和适用场景。

Published: 2026-07-25 | Updated: 2026-09-27

Mole 有两个独立产品。`mo` 是通过 Homebrew 安装、采用 GPL-3.0 的免费开源命令行工具，Mole for Mac 是带五个页面和菜单栏入口的付费原生应用。它们不是基础版与专业版，而是两套有重叠能力的实现，也可以同时安装。

## 快速对照

| | Mole CLI（`mo`） | Mole for Mac |
| --- | --- | --- |
| 许可 | 免费开源，**GPL-3.0**（不是 MIT） | 付费专有应用 |
| 价格 | 免费 | **[$19](https://mole.fit/pricing.md)** 一次买断，2 台 Mac，持续更新 |
| 安装 | `brew install mole` | 从 **mole.fit** 下载 |
| 删除范围 | 更宽（假定你会看清单） | 更窄（相对 CLI 同级或更保守） |
| 仅终端 | `mo purge`、`mo installer`、`mo touchid`，以及 `--dry-run` / `--json` | - |
| 仅应用 | - | GUI、菜单栏、风扇、启动项、应用内更新、优先进废纸篓的卸载 |
| 本地共享 | `~/.config/mole/whitelist*`、`~/Library/Logs/mole/operations.log` | 同一路径 |

## 两者有哪些重叠

命令行版的 `mo clean`、`mo uninstall`、`mo optimize`、`mo analyze` 和 `mo status`，分别对应应用的「清理」「软件」「优化」「分析」与「状态」。两边都涵盖缓存、应用与残留、磁盘分析和系统指标，但实现与候选清单并不完全相同。

扫描都免费。Mac 应用只对执行删除等操作收费，每项破坏性工具在激活前可使用两次，足以先比较扫描结果。两者还共享三份本地数据：`~/.config/mole/whitelist`、`~/.config/mole/whitelist_optimize` 和 `~/Library/Logs/mole/operations.log`。在一边保护的路径，另一边也会跳过，两边的删除都会进入同一份日志。

## 命令行版独有的能力

### `mo purge`

扫描项目目录中的大型构建产物，包括 `target`、`build`、`dist`、`.next`、`DerivedData`、`__pycache__`、`coverage` 等三十多类，也包括需要软件包管理器重新下载的依赖目录。Purge 会永久删除，不经过废纸篓，最近七天有改动的项目默认不选，因此仍应先运行 `--dry-run`。

### `mo installer`

从下载、桌面、Homebrew 缓存、iCloud 和邮件中查找 DMG、PKG 与归档安装包，并标注来源。

### `mo touchid`

为 `sudo` 配置 Touch ID 认证，不是清理功能。

`mo clean`、`mo uninstall`、`mo optimize` 和 `mo purge` 支持 `--dry-run`，其他命令先查看各自的帮助，不把预览支持范围推及所有操作。`mo analyze`、`mo status` 和 `mo history` 支持 `--json`，`mo status` 通过管道输出时会自动切换为 JSON。只读与结构化输出也可以通过 SSH、在无界面 Mac 或脚本中运行，定时删除仍需明确的非交互策略，cron 不会让交互确认自动变安全。

## Mac 应用独有的能力

应用更适合持续显示和受控的系统操作。「分析」提供可下钻矩形树图，「状态」提供实时面板、趋势线、置顶进程和菜单栏 HUD，「卸载」会在移动文件前展示路径、所有者与大小，卸载文件进入废纸篓。风扇和支持机型的电池写入，通过 SMJobBless 安装的受限 root 辅助程序完成，启动项只直接切换身份可验证的 launchd 与 Service Management 项目，无法确认的后台项会转到系统设置。

应用还包括摄像头与麦克风使用提醒、把支持机型充电维持在约 75% 到 80% 的电池养护、防止休眠、清洁屏幕和只读诊断报告，这些功能需要图形界面或常驻状态，命令行版没有对应入口。另一项差异是更新第三方应用：Mac 应用可以识别 Sparkle、Homebrew cask 与 formula、Mac App Store、Electron、GitHub Releases 和网站元数据，无法验证并安全替换应用包时，只会打开应用或厂商页面。

授权为一次购买，可用于两台 Mac，包含免费更新，提供 14 天退款，要求 macOS 14 或更高版本。两款产品都不发送遥测。

## 为什么应用的清理范围更窄

Mac 应用不是调用 `mo` 的图形包装器，而是 Swift 重新实现。两边共享路径分类和安全原则，但扫描器、超时、缓存与回退各自独立，因此结果可以交叉检查，不承诺逐字节相同。应用遵循「相同或更安全」：候选项会分为默认可选、默认不选的仅检查项，以及完全阻止项，与命令行版有分歧时，应用采用更窄的范围。

`mo purge` 就是典型例子。命令行版会处理 `node_modules`、`Pods`、`venv` 和 `vendor` 等下载依赖目录，Mac 应用则排除它们。编译产物可以从本地输入重建，依赖树却需要网络，重新解析后还可能不同，所以应用只展示可由本地编译恢复的内容，并默认不勾选。应用扫描应用数据前还会检查完全磁盘访问，执行删除时重新验证路径，应用卸载走废纸篓。命令行版假定用户会仔细读清单，所以范围更广，应用则让清单本身尽量保守。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/two-front-ends.webp" width="1360" height="454" loading="lazy" alt="终端工具与 Mac 应用两个界面分别执行相同五类操作，但在磁盘上读写同一份白名单和同一份操作日志。">
  <figcaption>两款产品运行时不通信，但共享保护列表和操作日志。</figcaption>
</figure>

## 同时安装时要注意什么

两者可以共存。在应用设置中保护路径后，`mo clean` 会跳过，通过 `mo clean --whitelist` 加入路径，应用也会遵守。两边不会协调正在进行的任务，同时开始清理可能互相干扰，`mo clean` 出现应用不展示的类别则属正常，这是刻意保留的安全差异。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/same-or-safer.webp" width="1360" height="454" loading="lazy" alt="命令行版会删除的集合包围着 Mac 应用更小的删除集合，两者差异是 node_modules、Pods、venv 与 vendor 等需要网络才能恢复的下载依赖目录。">
  <figcaption>Mac 应用只采用相同或更窄的删除范围，不会比命令行版更激进。</figcaption>
</figure>

## 选哪一个

习惯终端、需要 `--dry-run`、`--json`、脚本或项目产物扫描，或者不想付费，选命令行版，对许多开发者，它已经足够。需要磁盘地图、实时面板、风扇控制、启动项管理和第三方应用更新，或要替不使用终端的人安装，选 Mac 应用，后一个场景正是应用诞生的原因，见 [Mole 的故事](https://mole.fit/zh/blog/the-story-of-mole)。

仍不确定时，免费命令行版的 `mo clean --dry-run` 是一个低成本起点，它不会删除内容，结果也能帮助判断[是否真的需要清理软件](https://mole.fit/zh/blog/do-you-need-a-mac-cleaner)。

---

Canonical HTML page: https://mole.fit/zh/blog/mole-cli-vs-mac-app
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
