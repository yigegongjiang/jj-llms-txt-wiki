# 如何清理 Mac 磁盘空间，又不丢失文件

> 从备份和测量开始，按恢复风险处理下载项、应用、备份、缓存、照片和开发数据，并在清空废纸篓前验证结果。

Published: 2026-06-12 | Updated: 2026-09-28

磁盘快满时，按大小直接删除很容易碰到应用数据库或个人资料，真正的目标只是给日常工作、系统更新和临时文件留出空间。测量现状、备份重要数据，再处理能恢复或重新生成的内容，通常已经够用。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/review-before-remove.webp" width="1360" height="454" loading="lazy" alt="清理流程依次经过扫描、测量、分类、检查和路径验证，最后只把允许删除的内容移到废纸篓">
  <figcaption>扫描和删除分开，确认内容与路径后再移到废纸篓。</figcaption>
</figure>

## 清理前先保护唯一副本

大规模清理前，确认 [Time Machine](https://support.apple.com/104984) 或其他备份已经完成，重要文件也确实在备份中。云同步不能代替备份，删除同步文件可能让它从所有设备上一起消失，重要资料最好从备份中打开几份，或做一次小范围恢复测试。

### 磁盘已满到无法保存或更新时

先用一个明确可替代的项目腾出少量应急空间，例如可重新下载的 `.dmg`、源文件和交付副本都存在的视频导出、已另存的旧压缩包，或废纸篓中已经核对过的大文件。最初几 GB 是为了让测量、备份和更新恢复工作，不要从 Library 数据库、快照、交换文件或系统目录开始。

## 先测量，再决定删什么

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/analyze.webp" width="2584" height="1741" loading="lazy" alt="Mole 分析页把个人文件夹画成矩形树图，Library 占最大一块，Parallels、www、Downloads 等文件夹按大小排在旁边。">
  <figcaption>树状图按大小显示文件夹。这是 Mole 的“分析”视图。</figcaption>
</figure>

先按 Apple 的[存储空间指南](https://support.apple.com/102624)打开**系统设置 › 通用 › 存储空间**，查看个人文件和总体分类。但[系统数据](https://mole.fit/zh/blog/what-is-system-data-on-mac)不会告诉你具体是哪一个目录。

终端里的 `du` 可以统计目录大小：

```shell
du -sh ~/* ~/Library 2>/dev/null | sort -h
```

最大的目录会排在最后。找到大项后再深入一层：

```shell
du -sh ~/Library/* 2>/dev/null | sort -h
```

下面这条命令只查下载、影片和桌面中的大文件：

```shell
find ~/Downloads ~/Movies ~/Desktop -type f -size +500M -print 2>/dev/null
```

它常能找到遗忘的视频导出、磁盘映像和压缩包，但这些目录也可能包含项目数据，桌面还可能与 iCloud 同步，搜索范围小不代表结果都能删。树状图则把大小画成区块，[Mole](https://mole.fit/zh/) 的“分析”视图可以逐层查看目录，并在 Finder 中显示文件或移到废纸篓。想继续了解文件总和与磁盘容量为何不一致，可以看[如何查找并安全删除 Mac 大文件](https://mole.fit/zh/blog/how-to-find-large-files-on-mac)。

## 按优先级处理安全的清理项

按恢复风险从低到高处理，一旦空间够用就停止。下载项、废纸篓、闲置应用、旧设备备份、应用管理的缓存和离线内容，通常比系统管理的数据更适合作为目标。Mole 的“清理”页也采用先检查再执行的方式。

| 优先级 | 目标 | 恢复确认 |
| --- | --- | --- |
| 1 | 下载项、安装包、重复导出 | 能重新下载或重新生成 |
| 2 | 废纸篓 | 清空前逐项确认里面的大文件 |
| 3 | 闲置应用 | 文稿保留，订阅另行取消，有厂商卸载器就先用它 |
| 4 | 旧设备备份 | 在 Finder 的“管理备份”中核对设备和日期 |
| 5 | 应用管理的缓存和离线内容 | 先退出应用，接受重建或重新下载的代价 |
| 6 | 受管理的媒体库和开发工具占用 | 用所属应用或工具处理，再测试图库或项目 |

### 下载项和重复输出

旧安装包、磁盘映像、视频导出和重复压缩包通常风险最低。确认另一份副本或原始素材存在后，将它们移到废纸篓，并在测试期间先保留在那里。

### 不再使用的应用

如果应用提供自己的卸载器，Apple 建议优先使用，因为它可能一并处理登录项或扩展。否则可把应用移到废纸篓。Apple 说明[卸载应用](https://support.apple.com/102610)不会删除你用它创建的文稿，也不会取消订阅。需要检查残留时，使用[完整卸载指南](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)，不要删除其他应用共享的数据。

### 旧 iPhone 和 iPad 备份

在 Finder 中选择设备，再打开**通用 › 管理备份**，按设备和日期核对。Apple 的[备份管理说明](https://support.apple.com/108809)提供删除、归档和在 Finder 中显示等操作。不要因为备份目录名字看不懂就直接删除整个文件夹。

### 缓存和浏览器存储

优先使用应用自己的存储控制，清理有文档说明的缓存前先退出应用，并接受首次启动变慢或离线内容重新下载的代价。先了解[缓存清理会改变什么](https://mole.fit/zh/blog/how-to-clear-cache-on-mac)和[浏览器存储的处理方法](https://mole.fit/zh/blog/how-to-free-up-browser-storage-mac)，再碰隐藏目录。

### 照片、视频和其他受管理的资料库

使用照片、音乐、邮件或内容所属应用删除或迁移资料。Apple 的[照片图库迁移说明](https://support.apple.com/108345)允许把图库移到格式合适的外置磁盘，但打开“照片”前磁盘必须可用。验证迁移后的图库，必要时将其设为系统照片图库，然后才考虑原文件。完整步骤见[照片存储指南](https://mole.fit/zh/blog/how-to-free-up-photos-storage-mac)。

### 开发工具、容器和虚拟机

构建产物和下载的运行时可能很大，但项目、凭据、数据库和工具链常在附近。优先使用工具自带的预览、清理或 prune 命令，不要整目录删除。可从[开发缓存指南](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)、[Xcode 指南](https://mole.fit/zh/blog/how-to-clean-up-xcode-mac)或 [Docker 指南](https://mole.fit/zh/blog/how-to-clean-up-docker-mac)开始。

## 这些内容不在普通清理范围内

- `/System` 中的任何内容
- 正在使用的 Mail、信息和照片资料库
- 仍然需要的虚拟机与 iPhone、iPad 备份
- APFS 快照、交换文件和其他系统管理的存储
- 无法说明用途和恢复方式的文件

文件很大，只说明删错后的影响也大，拿不准时，所属应用、Bundle ID 和恢复方式可以提供更多背景。

## 可用空间数字为什么会延迟更新

删除大文件后，可用空间没有明显变化并不罕见。Time Machine 本地快照可能仍引用旧数据块，macOS 缩减快照后才会释放，系统还会把缓存、本地快照和可重新下载的数据标为“可清除”，需要容量时再自动回收。

查看快照：

```shell
tmutil listlocalsnapshots /
diskutil apfs listSnapshots /
```

可清除空间是 APFS 的容量状态，不是一个能手动清空的文件夹。Time Machine 会在快照到期或空间不足时[自动删除本地快照](https://support.apple.com/102154)，通常不用强制处理。

## 清空废纸篓前验证

文件还可恢复时，重新打开相关应用、最近文稿、照片图库和媒体项目，启动重要虚拟机与容器，确认云端文件能重新下载且本地唯一副本仍存在。若有内容缺失，从废纸篓“放回原处”或从备份恢复；确认工作正常且文件不再需要后才清空废纸篓，再检查**系统设置 › 通用 › 存储空间**并运行 `df -h /`，复核实际空间变化。

## 什么时候可以停下来

Mac 已经有足够空间完成日常工作、更新和临时任务时就可以停下，没必要删光缓存，也不用追求很小的系统数据。优先处理最大的可替代内容，含义不明或由系统管理的数据保持不动。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-free-up-space-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
