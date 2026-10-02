# 卸载 Mac 上的 Steam，别丢游戏和存档

> 卸载 Steam 和删除游戏库、本地存档是不同的事。先确认每个游戏库的位置和 Steam Cloud 实际同步了什么，再动数据。

Published: 2026-08-30 | Updated: 2026-09-23

卸载 Steam 和删除游戏库不是一件事。Steam 的[官方卸载说明](https://help.steampowered.com/en/faqs/view/30EB-87BF-531F-512D)
明确提到，如果不想丢掉游戏下载内容和本地存档，应保留
`~/Library/Application Support/Steam` 里的 `SteamApps` 文件夹。想保留已安装的游戏或
可能还需要的资料时，先把这个边界分清楚。

Steam Cloud 云存档也不是对所有游戏都一样的备份。每款游戏是否支持、保存哪些内容和本地
存档放在哪里，都要按游戏本身确认。不要因为看到云存档标记，或 Steam 能正常登录，就认为
所有存档都可以直接删除。

## 先决定要保留什么

先想清楚你要做的是只删掉 Steam 客户端、删除部分游戏来腾空间，还是清掉所有本地 Steam
资料。这几种目标的恢复方式不同。删除客户端不会注销 Steam 账户，但删除本地资料可能会
删掉需要重新下载的游戏，也可能删掉无法找回的进度。

如果是为了解决 Steam 本身的问题，重新安装客户端可能就够了。如果是为了释放磁盘空间，
应该先看游戏库，而不是默认应用本体占了大头。如果 Mac 要交给别人使用，也要先分别决定
每个游戏和每份存档是否还需要。

## 卸载前先保留 SteamApps

Steam 的[官方说明](https://help.steampowered.com/en/faqs/view/30EB-87BF-531F-512D)
把客户端和本地资料分开处理。在 Mac 上，它指出 Steam 的资料位于
`~/Library/Application Support/Steam`，并说明如果不想丢掉游戏下载内容和本地存档，
应保留 `SteamApps`。

这就是需要守住的边界。不要为了重新安装客户端就把整个 Steam 资料文件夹一起删掉。想保留
游戏库时，可以先让 `SteamApps` 留在原处，或复制到已经确认可靠的储存位置，再确认副本
完整后再删其他内容。重新安装可以生成客户端需要的辅助文件，删掉的游戏库却不会自动回来。

如果曾把游戏装到其他位置或外接磁盘，也要在 **Steam > 设置 > 存储空间** 里逐一确认各个
游戏库，并保留每个要留下的游戏库中的 `steamapps` 文件夹。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/steam-library-and-saves.webp" width="1360" height="454" loading="lazy" alt="Mac 上的 Steam 客户端、SteamApps 游戏库和按游戏分别保存的本地存档，分别对应不同的保留选择。">
  <figcaption>Steam 客户端、SteamApps 游戏库和各游戏的存档需要分别决定。保留 SteamApps 可以留住已安装的游戏和部分本地存档，但云存档状态仍要逐个游戏确认。</figcaption>
</figure>

## 只卸载客户端，别凭感觉删资料

改动文件前先退出 Steam。若只想卸载应用，可以把“应用程序”里的 Steam 移到废纸篓，再保留
Steam 资料文件夹，直到你确认要保留什么。若要按 Steam 的完整卸载流程处理，本地游戏库或存档仍
要保留时，先留下 `SteamApps`，再只删除已经确认不要的其他 Steam 资料。

不要对没有检查过内容的文件夹直接使用终端删除命令。先移到废纸篓能留出一小段恢复时间。
需要重新安装时，也应先确认保留的游戏库和重要存档仍在，再安装新的 Steam 客户端。

## Steam Cloud 和本地存档要按游戏确认

Steam Cloud 不是每个游戏都把所有进度存到线上。有的游戏支持云同步，有的支持范围或设置
不同，本地存档的位置也会因游戏而异。存档可能在 Steam 的游戏资料里，也可能在 Library
目录的其他位置，或由游戏自己的文档另行说明。

删除 `SteamApps` 或某个游戏的资料前，先看这个游戏的 Steam 页面或支持文档，条件允许时
打开游戏确认存档状态。对无法重新获得的本地存档保留一份副本。应以已确认的游戏资料为准，
不要用对 Steam Cloud 的笼统理解代替检查。

## 分清重装和彻底移除

为了排障而重装 Steam，不等于要删掉所有游戏。按照 Steam 的 Mac 说明，可以保留 `SteamApps`，只移除 Steam 支持目录里的其他内容。新客户端装好后仍可能校验文件或下载更新，但不需要从头下载整个游戏库。原文件先留在原路径或复制到容量足够的磁盘，确认新客户端已经识别后再清空废纸篓。

如果准备彻底移除，就要把 **Steam > 设置 > 存储空间** 里列出的每一个游戏库都检查一遍，包括外接硬盘和自己改过的路径。存档也要按游戏确认，有的跟着 Steam，有的放在其他 Library 目录，还有的完全由游戏自己管理。动手前也确认账号、密码和 Steam Guard 验证方式仍然可用，别在删掉可用客户端后才发现无法登录。

重装完成后，打开一两个重要游戏，确认游戏库和存档都正常，再删保留的副本。彻底移除后，则确认没有误留或误删其他位置的游戏库，再看释放了多少空间。这一步比立刻清空废纸篓慢一点，但比重新下载几十 GB 的游戏或找回本地存档省事得多。

Mole 核对这个应用时会列出什么、又会留下什么，写在 [卸载 Steam，留下游戏](https://mole.fit/zh/tested-apps/steam)。

## 常见问题

### 卸载 Steam 会删掉我的游戏吗？

把 Steam 应用移到废纸篓，本身不会删除游戏库。默认游戏库和相关资料通常在
`~/Library/Application Support/Steam` 下，但也可能有其他位置的游戏库。删除游戏库会移除
已安装的游戏，所以要在 **Steam > 设置 > 存储空间** 里逐一确认，留下需要的 `steamapps`
文件夹。

### 有 Steam Cloud 就可以删掉 SteamApps 吗？

不一定。云存档支持和游戏实际保存的本地文件都会因游戏而异。删除 `SteamApps` 或相关
资料前，先确认这款游戏自己的云存档和本地存档情况。

### 重新安装 Steam 后还能继续用原来的游戏库吗？

通常可以，关键是先保留 `SteamApps`，这样已安装的游戏和部分本地存档不会一起丢掉。
重新安装前先保留或复制游戏库，删除原始资料前再逐个确认游戏的存档状态。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-steam-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
