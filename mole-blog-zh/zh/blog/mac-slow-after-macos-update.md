# macOS 更新后 Mac 变慢，怎么排查

> 检查 Spotlight 索引和照片同步是否仍在进行，再排查 App、登录项和储存空间，用同一项操作确认是否恢复。

Published: 2026-09-28

Mac 更新系统并重新启动后，桌面已经能用了，后台的工作却可能还没结束，Spotlight 要整理索引，照片或云盘也可能继续同步。时间碰巧挨着更新，不等于问题一定出在 macOS 身上，一个不兼容的 App、登录后自动启动的程序，或者快满的启动磁盘，都可能让操作变慢，先看清谁在忙，再决定要不要动手。

## 用同一件事比较前后

挑一件确实变慢的事，比如打开本地文件夹、启动某个 App、导出一份文件，记下卡顿发生在哪一步，以及是整个 Mac 都慢，还是只有这个 App 慢。卡顿出现时打开活动监视器，看 CPU、内存和磁盘这三页；如果只有联网操作在等，也看看这个 App 的网络状态。等一切恢复正常后才截一张资源图，解释不了刚才那次卡顿。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/post-update-check.webp" width="1360" height="454" loading="lazy" alt="macOS 更新后的一项操作变慢，先区分正在推进的索引或同步，与持续存在的 App、登录项或储存空间问题，最后重做同一项操作">
  <figcaption>先认出正在做的事，一次只处理一个原因，然后重做刚才变慢的操作。</figcaption>
</figure>

## 看后台工作有没有往前走

Apple 说明，[安装软件更新后 Spotlight 可能建立索引](https://support.apple.com/102321)，打开 Spotlight 看有没有索引进度，同时在活动监视器里观察 `mds` 或 `mdworker`。资料量不同，索引可能花几小时，也可能几天，不能拿一个固定时长当作故障线；如果活动逐渐平息，同一项操作也恢复了响应，多半是暂时的工作。要是 CPU 和磁盘占用反复回来，却看不到进展，先[查是不是有文件夹或磁盘卷一直在变化](https://mole.fit/zh/blog/mds-mdworker-high-cpu-mac)，别一上来就重建索引。

照片占用资源时，打开照片 App 的图库，选所有照片并滚到最下面，Apple 记录的[图库状态](https://support.apple.com/119921)会显示上次与 iCloud 同步的时间、当前进度和未能同步的项目。状态写着暂停或失败，就按提示查原因，不要把关闭 iCloud 照片当作加速办法；其他云盘也看它自己的同步状态，光有网络流量不代表任务在推进。索引或同步确实在推进时，让 Mac 接上电源和网络，等它完成，再看原来那件事是否恢复。

## 一直慢，就一次隔离一个原因

先到**系统设置 › 通用 › 储存空间**看启动磁盘还有多少空间，Apple 也把空间不足列为 [Mac 变慢的原因](https://support.apple.com/guide/mac-help/mchlp1731/mac)。如果那项任务需要写入或交换空间，就移动或删除你认得的文件，然后重做任务；不要因为系统数据那一栏很大，就去删不认识的目录。

接着在卡顿时看活动监视器的内存压力和进程列表。如果只有最近更新的某个 App 不响应，或它反复占满 CPU，先退出它，再查开发者是否提供兼容当前系统的版本。如果一登录就慢，到**系统设置 › 通用 › 登录项与扩展**检查启动项目。Apple 的[登录项排查方法](https://support.apple.com/guide/mac-help/mh21210/mac)是先记下并移除原有项目，重启后若问题消失，再逐项加回，每加一项就重启比较；能逐个检查时就一次只改一个熟悉的项目，同时确认它所属的 App 仍能正常工作。问题只落在一个 App 上，就处理那个 App，不必把整个系统清理一遍。

## 再做一次原来的任务

在相近条件下重做一开始变慢的操作，对照延迟、CPU、内存压力、磁盘活动和同步状态。如果操作恢复，后台的索引或同步也结束了，就停在这里；如果仍慢又找不到明确的来源，可以重启一次排除卡住的会话，再按同样方法比较。磁盘报错、App 打不开、搜索结果缺失，各有自己的排查路径，批量清缓存、重建整个 Spotlight 索引或重置 SMC，都不是系统更新后的例行步骤。要继续按资源逐项找原因，可以看[为什么 Mac 会变慢](https://mole.fit/zh/blog/why-is-my-mac-so-slow)。

---

Canonical HTML page: https://mole.fit/zh/blog/mac-slow-after-macos-update
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
