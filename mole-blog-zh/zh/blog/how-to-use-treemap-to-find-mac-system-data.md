# 如何用树状图找到 Mac 系统数据

> 用树状图找到系统数据背后的真实目录，确认归属与恢复方式后，只删除可以找回的内容。

Published: 2026-09-04

系统数据是 macOS 的存储分类，不是一个文件夹。树状图能把分类背后的真实目录画出来，但“体积很大”从来不等于“可以删除”。

看到大色块后先问两件事：这些空间归哪个 App 或系统功能管理，删除后靠什么恢复。这样才能分清 App 缓存和正在使用的数据库、旧模拟器运行时和发布归档、本地副本和唯一副本。

## 系统数据为什么会变化

Apple 的[系统数据说明](https://support.apple.com/guide/mac-help/mchl3d437fbc/mac)确认它是日志、缓存、虚拟内存、临时文件和 App 支持等内容的汇总分类。

macOS 会把无法归入 App、文稿、照片等明确类别的内容放进系统数据，其中可能包括缓存、日志、App 支持文件、虚拟内存、本地快照、开发工具、设备备份和已下载模型。存储设置还会随着索引更新重新分类，因此数字变化不一定对应同样大小的可用空间变化。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/system-data-illustration.webp" width="1360" height="454" loading="lazy" alt="四个文件系统位置进入存储分类器，最后归入系统数据">
  <figcaption>系统数据来自多个目录。树状图测量这些目录，但不会把它们变成一个可以整体删除的文件夹。</figcaption>
</figure>

## 怎么读树状图

每个矩形代表一个文件或文件夹，面积对应大小。从最大的色块开始，每次只深入一层，并始终保留完整路径，这样才能知道自己位于个人目录、App 容器、`/Library` 还是受保护的系统位置。

大色块只是线索，不是清理建议。先确认归属：模拟器运行时交给 Xcode，镜像交给 Docker，设备备份交给 Finder，生成媒体交给对应的创作 App。归属不清时，可以先在 Finder 中显示并保持原位，直到找到恢复方式。

## 四步确认后再删除

这套检查同样适用于 `/Library`，路径属于系统级资源库时更要先找到所有者。

1. 测量顶层目录，确认这个色块确实解释了明显的空间占用。
2. 找到管理它的 App、账户或系统功能，文件夹名称和相邻文件通常比只搜一个名字更可靠。
3. 判断内容能否重新生成、重新下载、从备份恢复，或根本没有第二份，并估算恢复成本。
4. 有官方清理入口就优先使用；没有时，只把确认属于自己的文件移到废纸篓，测试对应 App 后再清空。

不要手动处理 `/System`、快照内部文件、交换文件、正在使用的邮件或照片数据库，也不要删除无法确认归属的容器。`~/Library/Application Support` 里既有可替换下载，也有真实 App 数据，它不是缓存目录。

文件移到废纸篓后，要清空废纸篓才会释放容量。APFS 快照仍可能引用旧数据块，存储设置也需要时间重新分类。清理后看系统设置或磁盘工具中的实际可用空间，不必追求让系统数据变成某个数字。

本地快照可先看[Time Machine 快照清理指南](https://mole.fit/zh/blog/how-to-delete-local-time-machine-snapshots-mac)。灰色分类很大时不要直接运行压缩快照的命令，macOS 通常会把快照空间视为可回收。

Mole 的分析页会把目录画成树状图，右键菜单可以在 Finder 中显示选中项，或把确认过的文件移到废纸篓。清理页是另一套经过审核的分类。分析页负责定位，不会把每个大色块都标成“可删”。想继续理解存储分类和 APFS 记账方式，可看[Mac 系统数据到底是什么](https://mole.fit/zh/blog/what-is-system-data-on-mac)。

## 按归属和恢复成本做决定

Apple 对[系统数据](https://support.apple.com/guide/mac-help/mchl3d437fbc/mac)的定义本来就包含日志、缓存、虚拟内存、临时文件、App 支持和插件，因此不要期待树状图上出现一个与灰色数字完全对应的目录。

| 大色块 | 先确认什么 | 正确入口 |
|---|---|---|
| `CoreSimulator` | 是否仍需要对应系统版本和测试状态 | Xcode 设置与 `simctl` |
| Docker 镜像或虚拟磁盘 | 哪些容器、卷仍在使用 | Docker 自己的存储管理 |
| iPhone 或 iPad 备份 | 设备、日期、是否有其他副本 | Finder 的设备备份管理 |
| Final Cut、Logic 生成媒体 | 原始素材是否在线 | 创作 App 的清理命令 |
| `Application Support` 数据库 | App 是否仍使用，是否有导出或同步 | 对应 App，归属不清就保留 |
| 本地快照 | 是否属于 Time Machine，空间是否已算可用 | Time Machine 与 macOS |

## 两个大色块的实际判断

### `CoreSimulator`

树状图能指出它大，但运行时、虚拟设备、DerivedData 和 Archives 的恢复成本完全不同，不能从 Finder 整体删除。

### Docker 虚拟磁盘

体积最大的文件可能只是虚拟磁盘容器，真正的镜像和卷关系要在 Docker 里确认。

## 为什么删除后容量没有立刻回来

先确认废纸篓已经清空，再比较磁盘工具或存储设置里的可用空间。APFS 快照可能仍引用旧数据块，macOS 也可能还在重新索引和分类。Apple 说明 [Time Machine 本地快照](https://support.apple.com/guide/mac-help/mh35933/mac)会在空间需要时自动移除，快照占用也会计入可用空间，因此不要只为了让灰色分类变小而强制缩减快照。

## 分析页负责定位，不替你决定

Mole 会保留完整路径并让你回到 Finder 或所属 App 继续处理，不会因为一个色块很大就自动把它列为垃圾。这条边界能避免把唯一数据库误当缓存。

## 常见问题

### 树状图里哪些东西不要删？
受保护的系统文件、快照内部数据、交换文件、正在使用的 App 数据库、云端占位文件，以及无法确认归属和恢复方式的内容，都不要手动删除。

### 怎么清理本地 Time Machine 快照？
先连接备份盘，让 Time Machine 完成备份。特定空间问题仍然存在时，再按快照指南列出当前快照，并使用 Apple 支持的工具处理。

### 树状图会自动减少系统数据吗？
不会。它只负责显示空间在哪里，是否能删仍取决于内容能否恢复，以及是否有对应 App 提供的清理入口。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-use-treemap-to-find-mac-system-data
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
