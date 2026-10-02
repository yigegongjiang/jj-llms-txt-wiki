# 如何查找并安全删除 Mac 大文件

> 用系统存储、Finder、终端和磁盘地图找到大文件，确认所有者、备份和恢复方式后再删除。

Published: 2026-06-07 | Updated: 2026-09-28

找磁盘占用最快的办法是测量，不必在 Finder 里逐个翻文件夹，但看到结果后还要判断文件用途：40 GB 的照片图库和 40 GB 的安装包，处理方式完全不同。APFS 快照、克隆文件和可清除空间也会影响统计，因此两个工具显示不同总数，不一定有谁算错了。

## 文件总和为什么对不上磁盘占用

现代 Mac 使用 APFS 文件系统，它与 Finder 统计空间的角度不同，差异通常来自本地快照和可清除空间。

- **本地快照**会在两次 Time Machine 备份之间保存磁盘状态。文件已经删除，只要快照仍引用它，旧数据块就会继续占用空间。Apple 说明每小时快照通常保留约 24 小时，并在到期或空间不足时[自动删除](https://support.apple.com/102154)。查看本地快照：

```shell
tmutil listlocalsnapshots /
```

- **可清除空间**包括缓存、本地快照和能重新下载的内容，macOS 会在需要容量时自动回收。Finder 可能把它算进可用空间，但它不是一个可手动清空的目录。

可以用三条命令核对磁盘状态：

```shell
df -h /
diskutil apfs list
diskutil apfs listSnapshots /
```

`df` 显示文件系统的已用和可用容量，`diskutil apfs list` 显示共享容器与各个卷，`diskutil apfs listSnapshots /` 列出启动卷快照。三条读数合在一起，通常就能分清普通文件、快照和共享 APFS 空间。

## 用终端测量目录

`du` 适合查找大目录：

```shell
du -sh ~/* ~/Library 2>/dev/null | sort -h
```

`-s` 汇总每一项，`-h` 使用容易阅读的单位，`sort -h` 把最大的结果排在最后，每次深入一个大目录即可。扫描云端目录或大型开发目录会比较慢，受保护的应用数据还可能需要给终端“完全磁盘访问权限”。

目录大小不一定等于物理占用，硬链接给同一份数据多个文件名，APFS 克隆则会在副本修改前共享数据块。`du` 用来找候选项，高影响删除前再结合 Finder 的“显示简介”和 APFS 容量判断。

查找个人目录中的单个大文件：

```shell
find ~/Downloads ~/Movies ~/Desktop -type f -size +500M -print 2>/dev/null
```

从下载、影片和桌面开始，常能找到遗忘的视频导出、磁盘映像和压缩包，也不会钻进应用数据库，确有需要时再增加扫描目录。喜欢终端界面时可以用 `ncdu` 按大小浏览目录，它很适合定位，但陌生目录的归属并不明显，确认用途后再到 Finder 移到废纸篓，会保留恢复余地。

## 快照或可清除空间很大时怎么办

先释放一部分明确由自己管理的文件，给 macOS 留出工作空间，系统会继续缩减本地快照和可清除缓存。重新连接 Time Machine 磁盘可以补齐备份历史，但不会直接清空快照，多数时候不必强制处理这两类空间。

## 用树状图查看整个磁盘

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/analyze.webp" width="2584" height="1741" loading="lazy" alt="Mole 分析页把个人文件夹画成矩形树图，Library 占最大一块，Parallels、www、Downloads 等文件夹按大小排在旁边。">
  <figcaption>树状图按容量绘制目录，点击区块可以继续深入。这是 Mole 的“分析”视图。</figcaption>
</figure>

终端数字适合精确查看单个目录，树状图更容易比较整个磁盘。[Mole](https://mole.fit/zh/) 的“分析”视图会从根目录开始绘制，点击区块即可深入，也能在 Finder 中显示文件，或确认大小后移到废纸篓。主目录等导航根节点没有删除入口，命令和树状图回答的是同一个问题，选顺手的即可。

## 磁盘分析器为什么能更快

简单脚本会遍历所有目录，对每个文件执行 `stat`，全部相加后再排序，面对数百万个小文件时，这种做法很慢，也占内存。Mole 的[开源命令行工具](https://github.com/tw93/Mole)在 `cmd/analyze` 中限制并发数量，并对硬链接去重，原生 App 使用独立的 Swift 扫描器，思路相同。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/directory-size-pipeline.webp" width="1360" height="454" loading="lazy" alt="Mole CLI 磁盘分析器读取目录后把任务放入有上限的队列，分别统计文件与目录、去重硬链接，再用堆保留最大的结果">
  <figcaption>CLI 会限制目录任务、<code>du</code> 子进程和待处理队列的数量，并用 Top-N 堆只保留最大的结果。</figcaption>
</figure>

目录工作池会按 CPU 核心数在 2 到 12 之间调整，`du` 子进程最多同时运行 4 个，更多 `du` 只会让磁盘争抢读写。队列也有限制，避免待处理目录不断堆积。

扫描器只保留最大的结果：目录取前 30 项，文件取前 20 项。结果进入最小堆，堆满后只和当前最小项比较，不必把数百万条记录全部放进内存排序。

硬链接会让同一文件出现多个名字，扫描器记录第一次见到的 `(device, inode)`，再次遇到便跳过，避免重复计算。同一次扫描中依赖去重顺序得到的目录大小不会写入缓存，以免后续单独扫描时读到不完整结果。

## 先分类，再决定是否删除

- **可替代内容**：安装包、可重新构建的输出，以及应用明确允许重建的缓存。删除前考虑重新下载或构建的时间
- **个人或工作数据**：照片、信息、项目归档、虚拟机磁盘、模型权重和设备备份。通过对应应用导出、备份或停用
- **应用或系统管理的数据**：包管理器数据库、容器、照片或邮件资料库、快照和 `/System` 下的文件。使用应用提供的入口，或保持不动

如果大目录属于已经不用的应用，先走正式卸载流程，再[检查残留文件](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)。普通文件先移到废纸篓，确认应用和项目正常后再清空。

## 按这个顺序查找大文件

先比较 `df`、APFS 容量和存储设置，判断是真正缺少物理空间，还是分类显示有差异，接着测量选定目录，沿最大的分支逐层查看，再按归属和恢复方式分类。可替代内容优先处理，个人资料交给备份和对应应用管理，更完整的清理流程见[Mac 磁盘空间不足时如何安全释放空间](https://mole.fit/zh/blog/how-to-free-up-space-on-mac)。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-find-large-files-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
