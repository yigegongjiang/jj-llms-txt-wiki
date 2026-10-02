# 有哪些好用的免费 Mac 清理工具

> OnyX、AppCleaner、Stats、ncdu 和 GrandPerspective 各有所长。按具体任务选择，也要留意维护状态与权限边界。

Published: 2026-07-24 | Updated: 2026-09-27

Mac 清理不一定要付费。许多免费工具并非残缺试用版，而是把范围做得很窄，或依靠开源与捐赠维持。判断是否值得用，发布记录和功能边界比价格更重要。

## 先用 macOS 自带工具

「系统设置」有存储分类和登录项，「活动监视器」能查看进程的 CPU、内存、能耗与磁盘读写，Finder 可以按大小排序，终端有 `du` 和 `df`，Time Machine 则提供删除出错后的恢复能力。

问题只是入口分散，而且隐藏的资源库目录很难看出属于谁。只有某款工具能简化一项经常重复的工作，才值得安装。

## 值得用的免费工具

### OnyX：明确的系统维护

Titanium Software 的 [OnyX](https://titanium-software.fr/en/onyx.html) 免费使用并接受捐赠。它提供系统文件验证、清理、维护、应用卸载、缓存删除、数据库与索引重建，以及 Finder、程序坞和 Safari 设置。

OnyX 会为每个 macOS 大版本发布独立构建。下载与当前系统匹配的版本，只执行自己能说清用途的操作。

### AppCleaner：卸载普通应用

FreeMacSoft 的 [AppCleaner](https://freemacsoft.net/appcleaner/) 免费使用并接受捐赠。把应用拖入窗口后，它会查找相关文件，再由用户确认删除。

截至 2026 年 7 月，官网把 3.6.8 标为兼容 Mojave 至 Tahoe，但发布记录显示该版本来自 2023 年 7 月。它适合普通应用，匹配结果仍要检查。带驱动、VPN 组件、系统扩展或特权辅助程序的软件，应先使用厂商卸载器。

### GrandPerspective：查看空间分布

[GrandPerspective](https://grandperspectiv.sourceforge.net/) 用矩形树图显示文件大小，采用 GPL 开源，要求 macOS 11 或更高版本。3.7.2 于 2026 年 5 月发布，项目仍在维护。

SourceForge 版本免费，Mac App Store 销售同一款应用以支持开发。它能指出最大的目录，但不会判断内容能否删除。

### ncdu：终端里的磁盘分析

[ncdu](https://dev.yorhel.nl/ncdu) 是采用 MIT 许可证的文本界面磁盘分析器。截至 2026 年 7 月，Zig 版为 2.9.2，发布于 2025 年 10 月，长期支持 C 版为 1.22，发布于 2025 年 3 月。

它可以按大小浏览目录，也能在确认后删除。适合已经在终端工作的人，但不提供安全判断。

### Stats：菜单栏系统监控

exelban 的 [Stats](https://github.com/exelban/stats) 采用 MIT 许可证，支持 macOS 12 及以后，可通过 `brew install stats` 安装。

它显示 CPU、GPU、内存、磁盘、网络、电池、蓝牙设备、时钟和部分传感器。README 把风扇控制标为旧模式，但项目仍持续修复风扇辅助程序与新芯片兼容，2.12.15 于 2026 年 5 月发布。把风扇控制视为需要单独确认机型兼容的附加功能即可。

### Mole CLI：终端清理

[Mole 命令行工具](https://github.com/tw93/Mole) 免费开源，可通过 `brew install mole` 安装。

它提供清理、卸载、优化、分析和状态查看。`mo clean`、`mo uninstall`、`mo optimize` 和 `mo purge` 支持 `--dry-run`，清理操作记录在 `~/Library/Logs/mole/operations.log`，其他命令的选项以各自帮助为准。

## 免费工具的优势和成本

开源代码可以检查，范围克制的项目也更容易维护。扫描数字不会直接带来收入时，产品较少有动力夸大结果。但免费和开源本身不保证规则始终正确，仍要看更新记录。

清理工具最难的部分不是执行删除，而是长期维护「什么可以删」。macOS 会改变数据位置和保护方式，应用也会迁移目录、拆分容器，甚至把离线资料放进看似缓存的位置。两年前正确的规则，今天可能已经危险。

OnyX 为每个 macOS 大版本单独发布，就是因为底层维护行为与系统版本相关。遵循厂商卸载器、更新规则和处理误删反馈，都需要持续投入。销售、捐赠或维护者时间只是不同的支持方式，更新记录更能说明项目是否可靠。

如果一款产品的收入依赖扫描结果有多吓人，它就有动力制造更大的数字，相对可信的结果往往范围更小，也更容易解释。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/free-tier-boundary.webp" width="1360" height="454" loading="lazy" alt="边界一侧是免费工具提供的可审查代码、克制范围和不夸大结果，另一侧是免费模式难以长期支撑的持续成本，包括更新删除规则、遵循厂商卸载器并提供支持。">
  <figcaption>删除很容易，长期保持删除规则准确才是成本。</figcaption>
</figure>

## 识别恐吓式清理软件

- 通过无关网页弹窗、广告或安装器捆绑主动找到你
- 扫描未完成，甚至权限不足时，就报告巨大的问题数
- 付款后才能看见结果内容，而不只是付款后执行
- 没展示任何结果就要求完全磁盘访问或管理员密码
- 订阅容易开始，却没有明确取消入口
- 没有清楚的公司主体、更新记录、版本历史和操作说明
- 承诺修复存储硬件、增加内存或消除硬件散热限制

如果弹窗声称 Mac 突然出现严重问题，可先阅读 Apple 的[诈骗识别说明](https://support.apple.com/102568)。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/scareware-flow.webp" width="1360" height="454" loading="lazy" alt="用户主动寻找的工具先展示候选项，再为执行操作收费，主动找到用户的工具先制造警报，再要求付费才能看到它声称发现的内容。">
  <figcaption>可信工具先展示候选项，再为操作收费，恐吓软件先制造警报。</figcaption>
</figure>

## 什么时候值得付费

如果每个月都要依次使用多款免费工具，综合产品能节省实际时间。Mac 应用 [Mole](https://mole.fit/zh/mac-cleaner) 一次购买，可用于两台 Mac，包含免费更新，要求 macOS 14 或更高版本，并提供 14 天退款。扫描免费，每项破坏性工具在激活前可执行两次。

它不是恶意软件响应工具或备份，也不能取代系统级软件的厂商卸载器。习惯终端的人，免费开源的 Mole CLI 可能已经足够。两者共享保护列表与操作日志，也都不发送遥测。

## 免费优先的操作顺序

macOS 自带工具可以作为起点，不够时再按任务补一款：磁盘地图用于找空间，AppCleaner 用于卸载普通应用，Stats 用于查看温度和负载，OnyX 用于明确的维护操作。支持预览或 `--dry-run` 的工具，也更容易在真正修改前理解范围。

一年只整理几次，免费方案通常足够。决定前可再看[是否需要 Mac 清理软件](https://mole.fit/zh/blog/do-you-need-a-mac-cleaner)和[怎样安全清理缓存](https://mole.fit/zh/blog/how-to-clear-cache-on-mac)。

---

Canonical HTML page: https://mole.fit/zh/blog/free-mac-cleanup-tools
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
