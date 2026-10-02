# Mac 系统数据是什么？如何安全减少占用

> 看懂系统数据包含什么、存储数字为何对不上，并按风险顺序释放空间，避开快照、数据库和系统文件。

Published: 2026-06-05 | Updated: 2026-09-05

打开 **Apple 菜单 › 系统设置 › 通用 › 存储空间**，常会看到一块很大的“系统数据”。它不是某个文件夹，也不是稳定不变的测量结果，而是 macOS 暂时无法归入应用、文稿、照片或邮件的数据总和。系统重新索引或分类后，即使没有删除文件，这个数字也可能变化。

Apple 的[存储空间说明](https://support.apple.com/guide/mac-help/syspf5a64aa6/mac)列出的内容包括日志、缓存、虚拟内存、临时文件、字体、应用支持文件和插件。真正要解决的问题不是“如何删除系统数据”，而是“哪些实际文件占用了容量、由谁创建、能不能恢复”。

## 30 秒判断是否需要清理

1. 在“存储空间”里先看**可用空间**，不要只看灰色区块。
2. 打开“磁盘工具”，选择内置磁盘的 APFS 容器，对照容器剩余容量。
3. 如果 Mac 还能保存文件、安装更新并完成日常工作，系统数据很大本身并不代表故障。
4. 只有可用空间确实不足时，才继续找最大的可确认目录。

Apple 会区分“空闲空间”和“可用空间”。前者是磁盘上物理空闲的容量，后者还包括 macOS 可按需回收的缓存，因此一部分空间可能同时显示为已使用和可用。灰色区块不适合作为清理目标。

## 系统数据是分类，不是位置

系统数据可能来自很多地方：

- `~/Library/Caches` 与 `/Library/Caches` 下的应用和系统缓存
- macOS 与应用写入的日志、崩溃报告和诊断归档
- APFS 上的 Time Machine 本地快照与系统更新快照
- `~/Library` 下的应用支持文件、容器、数据库和下载内容
- iPhone 与 iPad 本地备份、系统更新文件、字体和插件
- 随内存压力变化的交换文件与虚拟内存

这些内容没有一个可以整体清空的“系统数据目录”。仅凭分类名称删除目录，可能删掉照片库、虚拟机、离线模型，或某个应用数据库的唯一副本。

“其他用户与共享”也是单独的储存空间分类，数字大不等于可以直接删除账户目录。动 `/Users` 或 `/Users/Shared` 前，先看[这个分类的边界](https://mole.fit/zh/blog/other-users-shared-storage-mac)。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/system-data-illustration.webp" width="1360" height="454" loading="lazy" alt="macOS 将四个位置的数据归入系统数据这一储存类别">
  <figcaption>macOS 会从 <code>~/Library</code>、<code>/Library</code>、<code>/private/var</code> 和快照中汇总系统数据。</figcaption>
</figure>

## 系统数据变大的六个常见原因

1. **缓存和日志没有受到良好限制。** macOS 会按需移除部分安全缓存，正常应用也会限制缓存大小，但出现故障的应用仍可能写入数 GB 数据。判断依据应是所有者和恢复成本，而不是“缓存”这个名称。
2. **快照保留了已经变化的数据块。** Time Machine 会创建本地快照，系统更新前也可能生成快照。文件即使已经删除，只要快照还引用旧数据块，容量就暂时不会释放。Apple 说明快照空间会计入可用容量，并在快照过期或系统需要空间时自动移除。
3. **设备备份不断累积。** iPhone 与 iPad 本地备份可能很大，也可能多年没有再使用。应在 Finder 的“管理备份”中核对设备和日期，再决定是否删除。Apple 的[设备备份说明](https://support.apple.com/108809)列出了这个流程。
4. **开发工具保留了构建产物。** Xcode、包管理器、模拟器、构建系统和容器会保存多个版本，但项目、凭据和工具链常在附近。使用各工具支持的清理入口，或参考[开发缓存清理指南](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)。
5. **虚拟机和容器使用稀疏磁盘映像。** 它们的逻辑大小、显示大小和实际分配大小可能不同。先确认没有唯一的项目或数据库，再通过所属应用删除虚拟机或映像。Docker 用户可以从 [Docker 清理指南](https://mole.fit/zh/blog/how-to-clean-up-docker-mac)开始。
6. **应用支持目录保存了真实内容。** 浏览器、游戏、媒体应用、AI 工具和同步客户端会在 Library 中保存离线媒体、模型、索引或云端占位文件。这些内容可能可以重建，也可能下载成本很高，甚至只有本机这一份。`Application Support` 不能当作缓存整体处理。

## 为什么几个数字对不上

| 查看方式 | 反映的内容 | 数字不同的原因 |
| --- | --- | --- |
| 存储空间设置 | 分类后的估算值 | 索引和重新分类可能有延迟 |
| Finder“显示简介” | 文件或目录的逻辑大小 | 克隆文件、稀疏文件和权限会影响结果 |
| `du` | 能够遍历到的目录数据块 | 受保护路径可能被跳过 |
| `df` | 已挂载文件系统的已用与可用空间 | 它不按存储空间分类统计 |
| APFS 容器 | 同一容器内所有卷共享的容量 | 快照和其他卷也会占用容器空间 |

Apple 的 [APFS 说明](https://support.apple.com/guide/disk-utility/dskua9e6a110/mac)指出，同一容器里的卷会共享空闲容量，并按需分配。把 Finder 中每个文件夹的大小相加，不会得到某一条存储分类的数字。

## 用只读方式测量

以下命令不会修改磁盘：

```shell
df -h /
diskutil apfs list
diskutil apfs listSnapshots /
```

`df` 显示已挂载文件系统的已用和可用空间，`diskutil apfs list` 显示共享容器、卷与剩余容量，`diskutil apfs listSnapshots /` 列出启动卷快照。也可以按 Apple 的[快照查看指南](https://support.apple.com/guide/disk-utility/view-apfs-snapshots-dskuf82354dc/mac)，在“磁盘工具”中选择**显示 › 显示 APFS 快照**查看快照元数据。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/apfs-space-model.webp" width="1360" height="454" loading="lazy" alt="APFS 容器由用户文件、快照、可清除空间和空闲空间组成，缓存、日志、交换文件与快照又可能被归入系统数据">
  <figcaption>这是概念示意，不是可以直接相加的容量分区：可清除空间可能包含文件和快照，系统数据则是内容分类，不是额外占用的一块空间。</figcaption>
</figure>

列出 Time Machine 本地快照：

```shell
tmutil listlocalsnapshots /
```

查看 Library 中的大目录：

```shell
du -sh ~/Library/* 2>/dev/null | sort -h
```

扫描可能需要一些时间，没有“完全磁盘访问权限”时也会跳过受保护的位置。把最大的结果当作调查起点，不要直接当作删除清单。权限错误表示测量不完整，不表示那个路径可以丢弃。

## 按风险从低到高处理

1. 先处理可以重新下载的安装包、重复导出文件和自己能确认的大文件。
2. 优先通过应用自己的设置清理已确认的缓存；只有按文档手动删除缓存目录时，才先退出应用。清理后首次启动可能变慢，也可能需要重新下载内容。
3. 在 Finder 的“管理备份”中核对设备和日期后，删除不再需要的 iPhone 或 iPad 备份。
4. 通过所属工具处理构建产物、模拟器、容器映像、虚拟机和下载模型，有预览功能时先查看计划。
5. 除非正在按 Apple 的恢复流程处理明确问题，否则让 macOS 自己管理快照。

磁盘分析工具可以把目录大小画成地图。[Mole](https://mole.fit/zh/mac-disk-analyzer) 的“分析”视图按大小显示文件夹，“清理”视图只处理经过检查的类别。重点是找到空间归属，不是把系统数据压到某个固定数字。

## 普通清理绝不能碰什么

- `/System` 下的任何内容
- APFS 与 Time Machine 快照的内部结构
- 交换文件和虚拟内存
- Mail、信息、照片、浏览器与同步应用正在使用的数据库
- 无法确认所有者和恢复方式的容器
- 尚未确认完整原件位置的云端占位文件或本地文件

文件很大，只能说明它影响容量，不能证明它可以丢弃。

## 删除后数字为什么会延迟

文件移到废纸篓后，只有清空废纸篓才会释放容量。即使已经永久删除，APFS 快照仍可能引用旧数据块，存储空间设置也需要时间重新索引和分类。macOS 还可能在真正清除可回收内容之前，就把它计入可用空间。

让 Mac 空闲一段时间后，再检查 `df -h /`、APFS 容器和刚才处理的目录。不要因为一个分类还没刷新，就继续扩大删除范围。

## 常见问题

### 50 GB 系统数据正常吗？

没有适用于所有 Mac 的正常值。安装了 Xcode、虚拟机、本地设备备份或大型应用资料库的 Mac，通常会比轻度使用的 Mac 多。先看可用容量，再找最大的实际目录。

### 可以清空 `~/Library/Caches` 吗？

不要整体清空。退出所属应用，只处理已经确认、后果清楚的大缓存。少数应用会把离线内容或尚未同步的状态放在附近，详情可看 [Library 缓存清理指南](https://mole.fit/zh/blog/how-to-clear-cache-on-mac)。

### 应该删除本地快照吗？

通常不用。Apple 说明 Time Machine 会把快照空间计入可用容量，并在快照过期或需要容量时[自动移除](https://support.apple.com/102154)。命令能列出快照，不等于它正在阻塞正常工作。

### 重启能减小系统数据吗？

重启可以完成更新或清理卡住进程的临时状态，但不是日常存储维护方法。macOS 与应用继续工作后，交换文件、缓存和分类数字仍会变化。

### 什么时候可以停？

Mac 已经有足够容量完成日常工作和更新，而且最大的已确认问题已经解决，就可以停。若空间仍不足，应[按恢复风险清理磁盘](https://mole.fit/zh/blog/how-to-free-up-space-on-mac)，不要追着灰色分类继续删除。

---

Canonical HTML page: https://mole.fit/zh/blog/what-is-system-data-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
