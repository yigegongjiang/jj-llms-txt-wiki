# Mac 上怎么卸载搜狗、百度、微信输入法

> 第三方输入法装在 /Library/Input Methods，还会用启动项让后台进程一直在跑，先从输入法列表里移除、备份词库文件夹，再删剩下的部分。

Published: 2026-09-24 | Updated: 2026-09-26

我用官网安装包装好搜狗输入法再去卸，第一次直接被拒，提示应用正在运行，查下来是安装器往 `/Library/LaunchAgents` 里放了两个带 KeepAlive 的启动项，`SogouServices` 和 `SogouTaskManager` 刚被叫退出，launchd 马上又把它们拉起来。后来有用户卸腾讯的 Chatterfly 输入法，碰到的也是类似的拒绝，删除失败，提示在使用中。

输入法和普通应用不太一样，它有自己单独的文件夹，系统一直在后台跑着它，搜狗和百度还各自带着几个辅助进程，这篇就按搜狗、百度、微信输入法三款来讲，它们装在哪、哪些进程在跑、哪些残留是你自己的数据，以及按什么顺序卸才不会把键盘弄得打不了字。讯飞输入法我还没测，这篇先不写。

## 输入法装在哪

输入法不进 `/Applications`，它有两个专门的位置，`/Library/Input Methods` 是这台 Mac 上所有账户共用的，`~/Library/Input Methods` 只属于当前账户。搜狗和百度的官方安装器都把应用装进了 `/Library/Input Methods`，归 root 所有，移动它要输入管理员密码，微信输入法也在这里，路径是 `/Library/Input Methods/WeType.app`。

这也是输入法看起来总在运行的原因，macOS 是按需启动输入法的，打开系统设置里的键盘设置就会把装着的输入法全部加载一遍，所以在活动监视器里看到搜狗或百度的进程，不代表你最近真用它打过字。

## 先从输入法列表里移除

正在用的输入法直接卸掉，卸的过程中可能会有一小会儿打不了字，菜单栏里的输入法菜单也会一直显示它，直到菜单重启为止，所以动文件之前先把它从列表里拿掉：

1. 从菜单栏的输入法菜单切到别的输入法，比如 ABC。
2. 打开系统设置 › 键盘 › 文字输入 › 输入法 › 编辑。
3. 选中这个第三方输入法，点「移除」（减号按钮）。

这个设置页在 Apple 的[在 Mac 上更改「输入法」设置](https://support.apple.com/guide/mac-help/mchl84525d76/mac)里有说明。从列表里移除并不会删掉它，应用本身、辅助进程和数据都还在 Mac 上。

## 哪些进程一直在跑

我看的这三款，每款都不止一个进程：

- **搜狗**：输入法本体 `SogouInput`，加上 `SogouServices` 和 `SogouTaskManager`，后两个由 `/Library/LaunchAgents` 里的 `com.sogou.SogouServices` 和 `com.sogou.SogouTaskManager` 两个启动项守着，都是 KeepAlive，启动项还加载着的时候，退出进程没有用。
- **百度**：输入法本体 `BaiduIM`，`/Library/LaunchAgents` 里的 `com.baidu.InputService` 从应用内部跑 `enableInput`，`/Library/LaunchDaemons` 里还有一个 `com.baidu.baiduService`，在系统域里从应用内部跑 `baiduService`。
- **微信输入法**：输入法本体 `WeType`。

顺序很重要，要先移除或卸载启动项，再退出进程，反过来的话它们马上又回来了，手动删文件的话，删完重启一次，免得还有进程从废纸篓里跑着。

## 三款输入法各自留下什么

搜狗，用官方 6.25.1 安装器装的：

| 项目 | 是什么 | 怎么处理 |
| --- | --- | --- |
| `/Library/Input Methods/SogouInput.app` | 输入法本体，约 549 MB | 删，要管理员密码 |
| `/Library/LaunchAgents/com.sogou.SogouServices.plist` 和 `com.sogou.SogouTaskManager.plist` | 两个 KeepAlive 启动项 | 删，要管理员密码 |
| `/Library/QuickLook/SogouSkinFileQuickLook.qlgenerator` | 预览搜狗皮肤文件的快速查看插件，约 3 MB | 删，要管理员密码 |
| `~/Library/Application Support/Sogou/InputMethod` | 搜狗的用户数据，我测试机上有 485 MB | 先复制一份到别处，再决定 |
| `~/Library/Application Support/Sogou/ResHub` 和 `SkinShop` | 搜狗的资源和皮肤文件夹 | 删 |
| 以 `com.sogou.inputmethod.sogou`、`SogouServices`、`SogouHelper` 或 `com.sogou.SogouTaskManager` 命名的 Caches、HTTPStorages 和 Preferences | 输入法和几个辅助进程的设置与缓存 | 删 |
| `~/.sogouinput` | 我测试机上是空的 | 删 |
| `~/Library/Application Support/Sogou` 这个文件夹本身 | 搜狗别的产品也可能往里放东西的上层文件夹 | 留着，只删上面几个子文件夹 |
| `~/Library/Application Support/com.tencent.bugly` | 别的应用也在用的崩溃上报 SDK | 留着 |

百度，用官方 5.7 安装器装的：

| 项目 | 是什么 | 怎么处理 |
| --- | --- | --- |
| `/Library/Input Methods/BaiduIM.app` | 输入法本体，68 MB | 删，要管理员密码 |
| `/Library/LaunchAgents/com.baidu.InputService.plist` | 跑 `enableInput` 的启动项 | 删，要管理员密码 |
| `/Library/LaunchDaemons/com.baidu.baiduService.plist` | 跑 `baiduService` 的系统守护进程 | 删，要管理员密码 |
| `~/Library/Application Support/BaiduInput` | 用户词库文件夹，我测试机上有 2.9 MB | 先复制一份到别处，再决定 |
| 以 `com.baidu.inputmethod.BaiduIM` 命名的 Caches、HTTPStorages 和 Preferences | 设置与缓存 | 删 |
| `~/Library/sapi` 和 `~/Library/Application Support/Baidu` | 我没法证明它们属于输入法 | 不清楚是谁写的就留着 |
| 以 `com.baidu.BaiduNetdisk-mac` 或 `baidunetdisk` 命名的东西 | 百度网盘，另一个产品 | 留着 |

微信输入法是我每天在用的输入法，所以没有卸过它，在这台 Mac 上 Mole 给它列出 17 项、默认勾选 11 项，唯一没列出来的是 `~/Library/Preferences/com.tencent.WeTypeSettings.plist`，它属于微信输入法的设置助手，微信输入法卸掉之后，这个文件也可以删。

这里真正值得留的只有词库文件夹，删任何东西之前，先把 `BaiduInput` 或 `Sogou/InputMethod` 复制到 `~/Library` 以外的地方，以后还打算装回来的话，这份副本就一直留着。

## 看看还剩什么

下面几条命令只查看，不会改动任何东西：

```sh
ls "/Library/Input Methods" ~/Library/Input\ Methods 2>/dev/null
pgrep -il 'sogou|baiduim|baiduservice|enableinput|wetype'
ls /Library/LaunchAgents /Library/LaunchDaemons ~/Library/LaunchAgents | grep -iE 'sogou|com\.baidu\.(inputservice|baiduservice)'
ls -d ~/Library/Application\ Support/Sogou/* ~/Library/Application\ Support/BaiduInput ~/.sogouinput 2>/dev/null
```

第一条列出还装着的输入法，输出里有 `SogouInput.app`、`BaiduIM.app` 或 `WeType.app`，说明那一款还在。`pgrep` 什么都没打印，说明它们的进程都没在跑，如果打印出来的是已经删掉的那款，它是在从废纸篓里跑，下次重启就没了，退出后几秒又冒出来的进程，说明还有启动项加载着。启动项和守护进程都删掉以后，第三条应该什么都不输出。最后一条列出数据文件夹，列出来的都还在盘上，留不留在备份之后由你决定。

## Mole 怎么处理

[Mole](https://mole.fit/zh/) 会扫两个输入法文件夹，所以输入法会和别的应用一样出现在应用列表里，它不给输入法显示「活跃中」的提示和最近使用日期，因为不管你用没用，macOS 都会让输入法一直跑着，选中一款输入法时，Mole 会提醒你先切到别的输入法。

输入法本体，以及程序就在输入法包里的启动项和守护进程，默认都会勾选，在让正在运行的输入法退出之前，Mole 会先卸载这些勾选的启动项，KeepAlive 就没法再把辅助进程拉起来，上面搜狗那次被拒之后改的就是这一处。root 所有的项目要输入管理员密码，用户数据文件夹（`BaiduInput`、`Sogou/InputMethod`、`~/.sogouinput`）、搜狗的资源文件夹、辅助进程的缓存和快速查看插件会列出来，但不勾选，Mole 只认这几个具体的子文件夹，不收 `Sogou` 上层文件夹、`~/Library/sapi` 和共用的 Bugly 文件夹，所有东西都进废纸篓，还能找回来。

一批卸载里有输入法被删掉时，Mole 会重启输入法菜单，让旧条目消失，在这一处改之前我卸 Chatterfly，输入法菜单里还挂着它，看起来就像卸载失败了。残留和后台辅助进程的区别，可以看 [Mac 应用卸了还在后台跑？](https://mole.fit/zh/blog/app-still-running-after-uninstall-mac)和[关闭 Mac 开机启动项和登录项](https://mole.fit/zh/blog/how-to-disable-startup-programs-on-mac)。

## 常见问题

### 能不能直接把输入法拖进废纸篓？

输入管理员密码后可以把应用从 `/Library/Input Methods` 里移走，但启动项还留着，它们拉起来的搜狗、百度辅助进程会一直跑到你重启，正确的顺序是先从输入法列表里移除，再删启动项，最后删应用。

### 输入法已经删了，输入法菜单里还有它，是不是没卸干净？

多半不是，输入法菜单用的还是它启动时读到的那份列表，注销再登录一次就好，Mole 的优化里也有一项「输入法切换」，重启的就是这个菜单。

### 自己造的词会丢吗？

只要不删词库文件夹就不会丢，删之前先把 `~/Library/Application Support/BaiduInput` 或 `~/Library/Application Support/Sogou/InputMethod` 复制到 `~/Library` 以外的地方，Mole 会列出这两个文件夹，但不会勾选。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-input-method-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
