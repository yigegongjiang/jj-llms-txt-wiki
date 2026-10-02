# 讲清 Mac 上的 purge：命令、可清除空间与清理软件

> 在 Mac 上，purge 混指三件事：APFS 可清除空间、purge 内存命令，以及清理软件和 Mole 的 mo purge 命令。动手删之前先分清它们。

Published: 2026-09-21 | Updated: 2026-09-27

在 Mac 上，「purge」这个词指向三件毫不相干的事，而搜索结果常把它们混在一起。一个是访达里的储存空间数字，一个是操作内存的终端命令，还有一类是号称能腾出磁盘空间的软件。删错东西，或者对结果有错误的预期，往往就是从混淆这三者开始的。

这篇文章用一屏把它们分清楚，再分别指向各自的深入指南。文中的「Mole」只指 mole.fit 上的 Mole for Mac，以及它自带的 `mo` 命令行工具。

## 先分清：三种不同的 purge

把你搜索的词对到下面对应的那一行。三者里只有一种能稳定地按需腾出磁盘空间，而且往往不是大多数人以为的那种。

| 你搜的词 | 它其实是什么 | 能按需腾出磁盘吗？ |
|---|---|---|
| purgeable space、清除储存 | APFS 的一类统计口径，指 macOS 在需要空间时可回收的文件 | 大多不能，由 macOS 决定；本地 Time Machine 快照可以按需删除，代价是失去最近的还原点 |
| purge、sudo purge | 一个清空文件缓存（内存里缓存的磁盘数据）的终端命令 | 不能；它作用于内存，不是磁盘 |
| Purge app、purge app mac、mo purge | 清理类软件，以及删除项目构建产物和依赖文件夹的 Mole CLI 命令 | 能，但只删你能重建或重新下载的文件 |

一句话判断：在访达或储存空间设置里看到这个词，看可清除空间那节；在终端教程里读到，看命令那节；在挑工具，看软件那节。

## 如果你在找一款叫「Purge」的软件

搜「purge app mac」时，磁盘清理器、内存工具和命令行工具会混在一起，因为三者都借用了同一个词。所以同一份列表里会同时出现干着完全不同活的产品。

一款清理器值得信任，前提是它在删除之前把候选清单摆给你看，而不是删完再说。一个盲目「全部清除」的按钮恰恰相反：它把唯一重要的决定藏了起来。Mole 把复核本身当成产品。「清理」和「分析」会扫描并列出每个候选项及其路径和大小，扫描无需许可证、免费，删什么由你决定，不去比谁清出的数字更大。

## purge 命令（内存），以及它做不到的事

`sudo purge` 是一个真实存在的 macOS 命令，它请求内核清空文件缓存（内存里缓存的磁盘数据）。它需要管理员权限，作用于内存，而不是你的启动磁盘。它不会清空访达里的可清除空间，也不会删除文件。

在一台健康的 Mac 上，它其实也很少有用。macOS 本来就在管理这块内存，它看起来「回收」出来的空闲内存，通常几秒内就会被那份让 Mac 保持流畅的文件缓存重新填满。刚执行完 `sudo purge` 的一小会儿，Mac 可能反而变慢，因为刚丢掉的文件要重新从磁盘读回来；判断内存够不够，看活动监视器里的「内存压力」，而不是剩余内存有多少。如果你真正想问的是要不要装内存清理器，[这个误区另有专文](https://mole.fit/zh/blog/do-you-need-a-mac-memory-cleaner)。

## 储存空间设置里的可清除空间

可清除空间指的是 macOS 计入「可用」的那部分磁盘，因为它存的是需要空间时可以移除的文件：本地 Time Machine 快照、已缓存的下载，以及优化储存的副本。它是一个数字，不是一个文件夹，其中大部分没有受支持的按钮可以清空，macOS 会在写入需要空间时自行回收。

本地 Time Machine 快照是例外。`tmutil deletelocalsnapshots /` 可以删除它们；有本地快照时，Mole 的「清理」也可能在「可清除空间」下列出一行「Time Machine 本地快照」，默认勾选。无论用哪种方式，都会失去最近的还原点，[如何删除 Mac 本地 Time Machine 快照](https://mole.fit/zh/blog/how-to-delete-local-time-machine-snapshots-mac)讲了什么时候值得这样做。

完整的口径、访达 / `df` / 储存空间设置为何对同一个磁盘各说各话，以及那个数字里到底汇了些什么，都在[什么是 Mac 上的可清除空间](https://mole.fit/zh/blog/what-is-purgeable-space-on-mac)里。本文只把它和另外两种 purge 分开。

## mo purge：清理项目构建产物（Mole CLI）

`mo purge` 是免费 Mole CLI 里的一个命令。它扫描你的项目目录，找出构建产物和下载来的依赖，并提议删除。产物类型包括 `node_modules`、`target`、`.build`、`build`、`dist`、`venv`、`vendor` 和 `Pods`，默认扫描路径包括 `~/Projects`、`~/GitHub` 和 `~/dev`。最近七天内有文件改动、或 Mole 无法确认改动时间的产物默认不勾选；含有部署密钥文件、嵌套 Git 仓库或受 Git 跟踪文件的目录会被保护。

每次都先预览。用 dry-run 模式跑一遍，读清单，再真正执行：

```
mo purge --dry-run
mo purge
```

`mo purge --dry-run` 会显示它将移除什么但不删除任何东西，`mo purge --paths` 用来配置扫描哪些目录。`mo purge` 删除的东西是永久删除，不进废纸篓，所以 dry-run 很重要：要拿回来得重新构建或重装依赖，而这可能需要联网。

Mole for Mac 的「清理」里有一个范围更窄的同类清理。它列出本地构建就能重新生成的项目构建产物，比如 `target`、`.next`、`.build`、`dist`、`build`、`DerivedData` 和 `coverage`，只列 100 MB 及以上的文件夹。这些行列出但默认不勾选，你勾选的内容会移到废纸篓。App 刻意不碰 `node_modules`、`Pods`、`venv`、`vendor` 这类依赖文件夹，因为它们只能联网重新下载，而且重新下载的版本未必和原来装的一致。

这和清理器扫缓存、也和 IDE 自带的「invalidate caches」都是两回事。如果你在终端和 App 之间拿不定主意，[Mole CLI 还是 Mole for Mac](https://mole.fit/zh/blog/mole-cli-vs-mac-app)这篇对照讲清了各自能做什么。

## 安全的操作顺序

1. 用上面的表格，先确定你到底指的是哪一种 purge。
2. 动手前先测量：在储存空间设置里、在活动监视器的「内存」标签里，或对某个文件夹用 `du` 看真实数字。
3. 复核具体文件，别只看一个总数。
4. 用对的工具动手，再确认可用空间确实变了。

如果你要的是一次和「purge」这个词无关的通用磁盘清理，[如何在 Mac 上腾出空间](https://mole.fit/zh/blog/how-to-free-up-space-on-mac)是那份有序清单，而[Mole 安全吗](https://mole.fit/zh/blog/is-mole-safe)记录了 Mole 到底会删什么、又拒绝碰什么。

## 延伸阅读

- [什么是 Mac 上的可清除空间](https://mole.fit/zh/blog/what-is-purgeable-space-on-mac)：储存数字背后的 APFS 口径。
- [Mole CLI 还是 Mole for Mac](https://mole.fit/zh/blog/mole-cli-vs-mac-app)：`mo purge` 的定位，以及只有 App 能做的事。
- [你需要 Mac 内存清理器吗](https://mole.fit/zh/blog/do-you-need-a-mac-memory-cleaner)：为什么清空内存很少有用。

## 常见问题

### purge 和可清除空间是一回事吗？

不是。可清除空间是一个 APFS 储存数字，指 macOS 日后可回收的文件；`sudo purge` 是内存命令。两者只共用一个词根，别无关联。`sudo purge` 腾不出任何磁盘空间，而可清除空间大多只在 macOS 需要时才释放，本地 Time Machine 快照是其中唯一能按需删除的部分。

### purge 会删掉我的照片或聊天记录吗？

`sudo purge` 什么都不删，只清空内存里的文件缓存。可清除空间存的是 macOS 可以放掉的数据，比如本地快照和优化储存的副本，不是你文件的原件。不过快照是还原点：macOS 会生成新的，但删掉的那个时间点找不回来。`mo purge` 会删除项目文件夹里的构建产物，以及 `node_modules`、`vendor`、`Pods`、`venv` 这类依赖文件夹，后者要联网下载才能恢复，dry-run 会先把清单列出来。

### Mole 扫描需要许可证吗？

不需要。在 Mac App 里，扫描和查看结果都免费，无需许可证，也没有时间限制。只有动手时才需要许可证，每个付费工具都会先免费运行两次再询问。Mole for Mac 是一次性购买，19 美元可用于两台 Mac，含持续更新和 14 天退款。

### Mole CLI 免费吗？

免费。`mo` 命令行工具在 GPL-3.0 下开源免费，用 `brew install mole` 安装。付费 App 是独立的原生实现，不依赖 CLI。

---

Canonical HTML page: https://mole.fit/zh/blog/purge-on-mac-explained
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
