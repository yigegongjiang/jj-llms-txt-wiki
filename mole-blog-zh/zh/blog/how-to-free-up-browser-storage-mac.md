# 如何安全清理浏览器占用的磁盘空间

> 测量 Chrome、Safari 等配置目录，只清可丢弃缓存，把登录与站点数据库当作用户数据。

Published: 2026-07-31 | Updated: 2026-09-24

在开发机和日常使用的 Mac 上，浏览器大概是最安静的占盘大户之一，单个 Chromium 配置里，**Cache**、**Code Cache**、**GPUCache** 与 Service Worker 存储加起来可以到数 GB，而书签与密码相对很小，系统设置里的存储空间又常把这些归进[系统数据](https://mole.fit/zh/blog/what-is-system-data-on-mac)，而不是清晰的「Chrome」一行。

下面按层把配置体积拆开看，先说清 Safari 的现代路径与旧 Caches 为何不同，再说怎么在不必全部退出登录的前提下腾空间，读完应该能自己量出一个配置、只清可丢弃的那几层，并确认登录还在。

**短答案**：先测量浏览器配置目录的内部分布，缓存、网站离线数据和旧配置都可能占用空间。优先用浏览器自带的清除数据界面，只清图片和文件缓存通常能保留登录；整个 profile 可能含唯一的书签和本地数据，确认归属、用途与备份后再删除。

## 浏览器配置里真正变大的是什么

按层去想，而不是把它当成一个笼统的「浏览器文件夹」，因为每一层的重建代价完全不同，有的删了无感，有的删了要重新登录：

| 层 | 作用 | 重建代价 |
|---|---|---|
| HTTP 磁盘缓存 | 图片、脚本、媒体以加快加载 | 低；首次访问变慢 |
| Code / JS 字节码缓存 | 编译脚本缓存 | 低；下次启动费一点 CPU |
| GPU / Dawn / WebGPU 缓存 | GPU 侧资源 | 低 |
| Service Worker + Cache Storage | 离线壳、PWA | 中；应用会再缓存 |
| IndexedDB / Local Storage | 应用状态、离线库 | 未同步时高 |
| Cookie / Login Data | 会话与凭据 | 高；需重新登录 |
| 扩展及扩展存储 | 功能与其本地库 | 中到高 |
| 历史 / 书签 | 导航与同步 | 取决于账号同步 |

清**缓存**与清**网站数据**不是同一次操作，浏览器界面把它们分开是有理由的，缓存可以随时重建，网站数据则牵着登录态，手动删文件夹往往不分这个。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/browser-profile-layers.webp" width="1360" height="454" loading="lazy" alt="左栏为可丢弃的浏览器缓存，右栏为不应批量删除的身份与站点状态">
  <figcaption>同一配置有两档风险。退出后可清左栏；右栏当作重置，不是整理。</figcaption>
</figure>

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/cache-lifecycle.webp" width="1360" height="454" loading="lazy" alt="缓存被写入、复用，再安全丢弃，状态仍然保留">
  <figcaption>HTTP 与 GPU 缓存本就可丢弃。登录库与许多 Local Storage 树是状态，不是垃圾。</figcaption>
</figure>

## 实验：先量浏览器，不只看系统数据

动手测量之前先退出浏览器，避免文件仍在写入，终端或清理工具要看到容器路径，可能还需要完全磁盘访问，这里报权限错误说明是访问问题，不是目录为空，不要因为报错就以为里面没东西，容器路径上尤其如此。

### Chrome / Chromium 家族

路径会随渠道以及是否为 Chrome for Testing 略有不同，按机器上的实际目录名来量：

```
du -sh ~/Library/Application\ Support/Google/Chrome \
  ~/Library/Caches/Google/Chrome 2>/dev/null
du -sh ~/Library/Application\ Support/Google/Chrome/* 2>/dev/null | sort -h
```

总量可疑的话，再下钻到 `Default` 或 `Profile N` 这一层，逐个子目录量一遍，大项通常就藏在这里：

```
du -sh ~/Library/Application\ Support/Google/Chrome/Default/* 2>/dev/null | sort -h
```

配置下常见的大目录有 `Cache`、`Code Cache`、`GPUCache`、`Service Worker`，有时还有 `IndexedDB`，其中只有 `Cache`、`Code Cache` 和 `GPUCache` 属于可低风险清理的缓存，`Service Worker` 和 `IndexedDB` 要按站点数据处理，清理时只碰前三个，后两个交给浏览器自己的站点数据界面。

### Safari

现代 Safari 的重量多在容器目录里，旧的 Caches 路径可能几乎为空，两处都要看一眼：

```
du -sh ~/Library/Containers/com.apple.Safari/Data/Library/Caches \
  ~/Library/Safari \
  ~/Library/Caches/com.apple.Safari 2>/dev/null
```

其中较重的现代缓存，通常在容器里的这个路径下：

```
~/Library/Containers/com.apple.Safari/Data/Library/Caches/com.apple.Safari
```

旧的 `~/Library/Caches/com.apple.Safari/WebKitCache` 看起来相关，实际可能几乎为空，所以两边都量过再下结论，只看旧路径会误以为 Safari 很轻。

### Firefox

思路和 Chromium 家族相同，Application Support 放配置树，Caches 放可重建的缓存，先量总量再决定动不动手：

```
du -sh ~/Library/Application\ Support/Firefox \
  ~/Library/Caches/Firefox 2>/dev/null
```

### Edge、Brave、Arc、Dia 等 Chromium 构建

同一思路：Application Support 放配置树，Caches 可能重复或镜像其中一部分，多配置的机器还会积累废弃的 `Profile 12`，删任何目录之前先分别测量，别凭目录名字猜哪个能删。

浏览器自己的界面是第一选择：Chrome 的[清除浏览数据](https://support.google.com/chrome/answer/2392709)、Safari 的[历史记录与网站数据控制](https://support.apple.com/guide/safari/sfri47acf5d6/mac)，以及 Firefox 的[缓存清理说明](https://support.mozilla.org/zh-CN/kb/how-clear-firefox-cache)。它们能区分缓存、Cookie、登录状态和整个配置文件，手动删目录做不到。

## 优先用内置清理

能走浏览器自己的清理界面就先走界面，下面这些路径分别对应各家浏览器的做法。

### Chrome、Edge、Brave、Arc 等

1. CPU 症状用浏览器自带的**任务管理器**查（另见 [Chrome Helper](https://mole.fit/zh/blog/google-chrome-helper-high-cpu-mac)），磁盘占用是另一项作业，别混在一起查
2. 打开**清除浏览数据**，先只勾**缓存的图片和文件**这一类，其他先别勾
3. 时间范围很重要，选「时间不限」再加上 Cookie，会让各站全部退出登录
4. 在配置选择器里移除不用的**配置**，这样才能保持内部注册一致，不要猜删文件夹
5. 禁用或卸载长期不用的扩展，有些扩展会悄悄保留很大的本地存储

### Safari

1. 在开发菜单里**清空缓存**（先在设置的高级里打开开发菜单），或按时间范围清除历史记录
2. 支持多重配置的版本，可以直接在设置里删除整个次要配置，比手动删目录干净
3. 优先走 Safari 自己的界面，不要整棵删 `~/Library/Safari`，历史、阅读列表等状态都混在其中，删了找不回来

### Firefox

**设置，隐私与安全，Cookie 和网站数据，清除数据**，不想退出登录的话，只清缓存的网络内容。

界面优先的原因是浏览器自己维护索引与配额记账，乱删叶子目录，下次启动可能换来更久的自动修复，省下的空间未必划算，所以能走界面就走界面。

## 界面不够时的手动路径

只有在内置清理不够、浏览器又确实退出之后才动手，并且只针对自己理解的叶子目录，先把这几条量一遍：

```
du -sh ~/Library/Application\ Support/Google/Chrome/Default/Cache \
  ~/Library/Application\ Support/Google/Chrome/Default/Code\ Cache \
  ~/Library/Application\ Support/Google/Chrome/Default/GPUCache 2>/dev/null
```

删的是**缓存叶**这几个目录，不要删整个 `Default`，因为删掉整份配置等于把这个身份从头重置，扩展、登录、设置会一起没了。

## 实例

举个例子，一台开发机上「系统数据」占了约 90 GB，按上面的路径逐个测量，结果大致是：

```
~/Library/Application Support/Google/Chrome/Default/Cache          18G
~/Library/Application Support/Google/Chrome/Default/Code Cache      4G
~/Library/Application Support/Google/Chrome/Profile 3               22G
```

其中 Profile 3 是一个早就不用的工作配置，单个就有 22 GB。先确认书签、密码和本地网站数据已妥善备份，或确实不再需要，再按以下步骤处理：

1. 打开 Chrome 的配置选择器，如果里面仍显示 Profile 3，先从界面把它移除，让注册保持一致
2. 若文件夹还在且确认不用，先退出 Chrome，再只删该配置目录，删之前确认它不是你要保留的 Default
3. 用「清除浏览数据」里缓存的文件一项清 Default 的 Cache 和 Code Cache，或退出 Chrome 后只删这两个叶
4. 复测一遍目录大小，确认 Default 里仍有 Login Data，常去的工作站点仍能正常登录

因此不要为了「省事」把整棵 `Google/Chrome` 一起删掉，所有配置都住在这棵目录下面，删了就是全部重置。

## 更新器残留与废弃配置

Chromium 更新器有时会留下旧版本目录。不能只看哪个版本启动成功：先确认旧版本没有进程在运行、没有安装或更新正在使用，拿不准就保留。废弃配置也应先核对书签和本地数据的备份，再从浏览器界面移除。

部分系统上的 Edge 更新器还会保留安装载荷。比**已安装 Edge 版本更旧**的文件也要确认没有进程或安装任务在使用，等于或更新的可能是待装更新。读不到已装版本或无法确认用途时，保留这些候选，不凭“最新一份”推断其余都能删。

## 同步副作用

清**缓存**或 **Cookie 与网站数据**都不等于删除书签，后者主要影响站点登录和本地状态。若直接删除已同步的书签、历史或其他同步项目，变更则可能传到其他设备，具体范围以浏览器的确认提示为准。

## 工具能帮到哪里

清理工具可以汇总已知的浏览器缓存类别、显示大小，并把敏感配置标为仅供审核或默认不勾选，多浏览器、旧配置共存时能减少漏看路径的机会，但它不应替代浏览器自己的清除数据界面做 Cookie 级决策，也不应把整个 Application Support 浏览器树当作一个「安全」复选框。

用分析工具或 `du` **定位**哪个配置巨大，用浏览器自己或经审核的缓存列表**删除**，定位和删除最好用不同的工具，再配合一般的[缓存卫生](https://mole.fit/zh/blog/how-to-clear-cache-on-mac)就够了。

如果要删掉的是浏览器本身，而不只是给配置腾空间，可以看[卸载 Chrome](https://mole.fit/zh/tested-apps/chrome)和[卸载 Firefox](https://mole.fit/zh/tested-apps/firefox)指南，两篇都先讲怎么保存书签和浏览器资料。

## 常见错误

**为了腾空间勾选「全部浏览数据」**，付出去的是所有站点重新登录的代价，不只是那几 GB，别为了省空间选它。

**整份删掉 `Default`**，那是重置，不是整理，等于把这个浏览器身份从头再来，想清缓存不该走这条路。

**只信 Safari 旧 Caches 路径**，容器里的现代路径也要一起量，两处差距可能很大，漏掉一处就白量了。

**Chrome 还开着就删缓存叶**，写入方可能正握着这些文件，删了也未必干净，先退出再动手。

## 如何验证

1. 重新打开浏览器，确认需要的配置与登录状态都还在，别只看能不能启动
2. 复测刚才量过的路径，缓存目录应变小，配置根目录及需要的设置、登录数据仍应保留
3. 先访问几个常用的重页面，首次加载变慢是缓存重建的正常现象，第二次访问就该恢复
4. 登录和站点都确认无误之后，再清空废纸篓，别在验证之前清

## 操作顺序

1. 先退出浏览器，运行中的浏览器可能正在写入这些文件，删了也未必干净
2. 先测量 Application Support 与 Caches 两棵根，再下钻体积最大的那个配置，别凭感觉挑
3. 优先用浏览器自带的「清除浏览数据」，只勾缓存文件，别勾 Cookie 和其他站点数据
4. 从浏览器的配置选择器界面移除废弃配置，不要直接删文件夹，界面会保持内部注册一致
5. 空间仍不够时，再手动删除已确认归属的缓存叶目录，别碰配置根和站点数据
6. 重新打开浏览器，确认登录身份与常用站点都正常
7. 一切确认没问题之后，再清空废纸篓，把不可逆的一步放在最后

## 延伸阅读

- [系统数据](https://mole.fit/zh/blog/what-is-system-data-on-mac)
- [清理缓存](https://mole.fit/zh/blog/how-to-clear-cache-on-mac)
- [Chrome Helper 占 CPU](https://mole.fit/zh/blog/google-chrome-helper-high-cpu-mac)
- 各浏览器「清除数据」功能的官方说明文档

浏览器占盘说到底是缓存与配置管理问题，不需要为此重装 macOS，先量配置树，先清可丢弃缓存，即使目录名里有 Cache，也要把登录与站点数据库当作用户数据，这两步的顺序别反过来。


## 多浏览器并存时的测量顺序

机器上同时装着 Safari、Chrome、Arc 时，先别猜谁占得多，按总量排个序再决定从哪棵树下钻：

```
du -sh ~/Library/Application\ Support/Google/Chrome \
  ~/Library/Application\ Support/Arc \
  ~/Library/Containers/com.apple.Safari \
  ~/Library/Application\ Support/Firefox 2>/dev/null | sort -h
```

从最大的根开始下钻，一个废弃配置的体积，往往就足以解释系统数据里那笔难以判断的增量，先别怀疑系统本身。

## 站点数据与扩展的取舍

只想腾空间时：

- 勾选缓存文件
- 不要勾选 Cookie、密码、自动填充
- 扩展逐个审视：长期不用的扩展连同其本地库一起卸，比整库清空更可控

需要彻底重置某个站点时，用浏览器的「清除该网站数据」，而不是删整个配置目录，前者只影响那一个站点。

## 与系统数据的关系

浏览器的大配置经常被算进系统数据，在配置树里量到 20 GB 缓存并清掉后，系统数据那个数字可能滞后一两天才重新分类，以 `du` 与实际可用空间为准就好，不必盯着分类条焦虑，详见[系统数据](https://mole.fit/zh/blog/what-is-system-data-on-mac)。

## 常见问题

### 清理浏览器存储会让网站退出登录吗？

只清浏览器界面里的图片和文件缓存，通常不会退出登录。Cookie、Local Storage、IndexedDB 等网站数据可能包含会话或离线内容，Service Worker 相关存储也不能一概当作无关状态的缓存，清理后应检查常用站点。

### 浏览器的占用为什么显示在「系统数据」里？

浏览器 profile 存放在 `~/Library` 下，macOS 的储存空间设置把这棵目录树的大部分归入系统数据条，于是一个 10 GB 的 profile 会撑大一个看起来与浏览器无关的数字，清掉之后这个数字回落也可能滞后一两天。

### 旧更新器文件和废弃 profile 可以删吗？

确认归属只是第一步。已卸载浏览器的 profile 仍可能保存唯一的书签、扩展数据或离线内容，先核对备份；更新器文件也要确认没有运行中的进程或安装任务在使用。仍在用的浏览器优先通过配置管理界面删除，不因目录没列在界面里就直接认定无用。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-free-up-browser-storage-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
