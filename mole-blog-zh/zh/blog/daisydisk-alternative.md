# DaisyDisk 的替代方案怎么选

> 比较 DaisyDisk、GrandPerspective、ncdu 和 Mole，也解释 APFS 克隆、硬链接与快照为何会让统计结果不同。

Published: 2026-07-21 | Updated: 2026-09-28

磁盘分析器只回答一个问题：空间集中在哪些目录。两款正常工具扫描同一磁盘，也可能得到不同总数。APFS 克隆、硬链接、稀疏文件、快照和无权读取的目录，都会改变计数。

选择时看两点：哪种展示最容易读，以及它的总数到底计算了什么。

## 展示方式有什么不同

空间视图各不相同：DaisyDisk 是同心放射环，GrandPerspective 和 Mole 是矩形树图，OmniDiskSweeper 和 ncdu 都是按大小排序的列表。macOS 存储页只给分类条形图，不显示路径。

找到大文件后的操作也很重要。「在 Finder 中显示」把决定留给用户，直接删除则依赖工具的恢复设计。不会经过废纸篓的工具，只适合处理确定可重建的数据。

## 工具选择

### macOS 存储页：适合起步

打开「Apple 菜单 > 系统设置 > 通用 > 存储」。Apple 的[存储设置说明](https://support.apple.com/guide/mac-help/change-storage-settings-mchl3d437fbc/mac)列出了分类条形图、磁盘卷详情和三项系统建议。点击类别旁的信息按钮可以浏览文件，「文稿」还能按名称、种类、访问时间和大小排序。

文稿或媒体类别明显偏大时，这已经够用。它的边界是「系统数据」。Apple 的[存储说明](https://support.apple.com/102624)指出，这个类别由 macOS 管理，无法直接控制。空间落在这里时，才需要路径级地图继续定位。

### DaisyDisk：成熟的放射环

[DaisyDisk](https://daisydiskapp.com/) 仍在积极维护。4.34.2 于 2026 年 7 月发布，4.34 增加了 APFS 克隆计数。它提供放射环、空格预览、批量收集器、管理员扫描、云盘扫描、快照管理，以及硬链接与克隆检测，要求 macOS 10.13 或更高版本。

授权为一次购买，可覆盖多台个人 Mac，并提供退款期，具体见[价格页面](https://daisydiskapp.com/support/pricing/)。它负责找出和删除文件，不会判断目录是否属于已卸载应用，也不知道缓存能否安全重建。

### GrandPerspective：免费开源矩形树图

[GrandPerspective](https://grandperspectiv.sourceforge.net/) 已维护接近二十年，3.7.2 于 2026 年 5 月发布，采用 GNU GPL，要求 macOS 11 或更高版本。

它支持多种颜色映射、过滤、硬链接、云存储、快速刷新、图片与文本导出，以及十一种本地化。SourceForge 版本免费，Mac App Store 销售同一应用以支持开发。界面较老，但功能足够，多数人可以先用它。

### OmniDiskSweeper：简单排序列表

The Omni Group 仍免费提供 [OmniDiskSweeper](https://www.omnigroup.com/more)。它从大到小列出文件，并允许打开或移到废纸篓，没有地图，也几乎没有学习成本。

项目维护较慢。Omni 博客记录了 2025 年 9 月更新，同月测试渠道发布要求 macOS 14 或更高版本的 1.16 构建。下载后要确认具体构建与当前系统兼容。

### ncdu：终端与远程磁盘

[ncdu](https://dev.yorhel.nl/ncdu) 采用 MIT 许可证，可以通过 SSH 使用。截至 2026 年 7 月，Zig 版 2.9.2 发布于 2025 年 10 月，C 版长期支持 1.22 发布于 2025 年 3 月。macOS 可通过 Homebrew 或 MacPorts 安装。

它支持并行扫描、JSON 导入导出、压缩格式和界面内删除，也会区分表观大小与实际磁盘用量，适合脚本和远程访问。删除不易恢复时，把它用于定位、再通过 Finder 处理会留出更多恢复余地。

### Mole「分析」页：维护工具里的磁盘地图

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/analyze.webp" width="2584" height="1741" loading="lazy" alt="Mole 分析页把个人文件夹画成矩形树图，Library 占最大一块，Parallels、www、Downloads 等文件夹按大小排在旁边。">
  <figcaption>Mole 的矩形树图已经点进个人文件夹，每一块按占用的空间决定大小，再点一下就进入下一层。</figcaption>
</figure>

[Mole](https://mole.fit/zh/) 使用可下钻矩形树图和面包屑导航。右键可移到系统废纸篓或在 Finder 中显示，`/`、`/Users`、`/Applications` 等导航根目录不能删除。无法计算大小的条目会明确标记并允许重试，不会静默算成零。

授权为一次购买，两台 macOS 14 或更高版本的 Mac。扫描结果缓存 24 小时，可能暂时过期，Mac 应用没有 JSON 导出，免费 CLI 支持 `mo analyze --json`。

## 为什么总数会不同

- **APFS 克隆：** Finder 复制的文件会共享底层块，直到一方修改。按逻辑大小相加会把共享数据重复计算
- **硬链接：** 一个 inode 可以有多个名称。按名称计数会放大总数，按 `(device, inode)` 去重则能区分共享字节
- **稀疏文件：** `Docker.raw` 等文件的表观大小可能远大于已分配块。逻辑大小和物理占用是两种答案
- **快照与可清除空间：** 普通目录扫描不会包含快照保留的全部历史块；可清除空间则可能包含可见的缓存或云文件，不能把它全部视为目录树之外的数据
- **不可读目录：** 没有完全磁盘访问或 root 权限时，部分目录无法统计。工具应显示读取失败，不能把它当成空目录
- **符号链接：** 跟随链接会把目标算到错误位置，也可能重复计数。只解析而不继续进入通常更安全

扫描方法也会造成差异。进程内使用 `fts(3)` 或 `getattrlistbulk`，更容易控制进度、取消和硬链接去重，调用 `du(1)` 可以复用成熟遍历器，但会继承它的计数方式。混合两种方法的工具可能同时带来两套模型。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/why-totals-disagree.webp" width="1360" height="454" loading="lazy" alt="同一个磁盘卷的扫描分为 APFS 克隆、硬链接、稀疏文件、快照和扫描器无权读取的目录五条计数分支，最后得到两个不同总数。">
  <figcaption>克隆、链接、稀疏文件、快照和读取权限，都可能让正确的分析器得出不同总数。</figcaption>
</figure>

## 快速对比

| 工具 | 展示方式 | 找到项目后 | 授权方式 |
| --- | --- | --- | --- |
| 存储设置 | 分类条形图，无路径 | 从分类浏览器删除 | macOS 自带 |
| DaisyDisk | 放射环 | 收集后批量删除 | 一次购买 |
| GrandPerspective | 矩形树图 | 显示、删除或导出 | GPL 开源 |
| OmniDiskSweeper | 大小排序列表 | 移到废纸篓或打开 | 免费 |
| ncdu | 终端列表 | 原地删除，JSON 导出 | MIT 开源 |
| Mole | 可下钻矩形树图 | 显示或移到废纸篓，保护根目录 | 一次购买 |

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/scan-methods.webp" width="1360" height="454" loading="lazy" alt="左侧进程内目录遍历使用 fts 或 getattrlistbulk，提供实时进度、干净取消和硬链接去重，右侧调用 du，在结束时返回一个数字，并继承 du 的计数方式。">
  <figcaption>进程内遍历和 du 都可以正确，关键是工具要说明总数采用哪种定义。</figcaption>
</figure>

## 可复现的排查步骤

`df`、APFS 容量和系统存储页可以先帮助判断是真实物理压力，还是快照与可清除空间造成的差异，之后的分析器如果会明确标记不可读区域，结果也更容易解释。

沿最大分支找到能够确认所有者的目录，再用 `du -sh` 或 Finder「显示简介」交叉检查。可替代数据适合优先处理，并留在废纸篓中，直到相关应用验证正常。手动流程见[如何找出大文件](https://mole.fit/zh/blog/how-to-find-large-files-on-mac)，「系统数据」异常则参考[系统数据是什么](https://mole.fit/zh/blog/what-is-system-data-on-mac)。

---

Canonical HTML page: https://mole.fit/zh/blog/daisydisk-alternative
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
