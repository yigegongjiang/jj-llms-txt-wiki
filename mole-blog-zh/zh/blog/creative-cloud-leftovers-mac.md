# Adobe 应用删了，Creative Cloud 还在 Mac 上跑

> 删掉 Lightroom 或 Photoshop 后，Creative Cloud 的八个应用、登录项和后台服务都还在，逐个说明它们是什么、哪些别的 Adobe 应用还要用，以及怎么删掉其余部分。

Published: 2026-09-26

有位 Mole 用户删掉 Lightroom 之后，发现 Core Sync、Creative Cloud、Creative Cloud Desktop App、Installer 和 Uninstaller 还留在 Mac 上，在卸载列表里却一个都找不到。这不是哪台机器的怪毛病，删 Adobe 应用本来就不会动 Creative Cloud，而 Creative Cloud 也不是装在一个地方的一个应用。

我在测试机上装了 Creative Cloud 6.10.0.253 和 Adobe Bridge 2026，换着顺序卸了几轮，把每一步留下了什么都记了下来。结论很简单，先卸 Adobe 应用，最后卸 Creative Cloud，有几样东西要等到 Adobe 的东西一个都不剩时才能删，因为别的 Adobe 应用还要用。

## Creative Cloud 是三个文件夹里的八个应用

Creative Cloud 不像大多数应用那样装在 `/Applications` 顶层，它把八个应用放进了 `/Applications/Utilities` 下几个属于 root 的文件夹：

- `Adobe Creative Cloud`：Creative Cloud 本体，以及 Creative Cloud Helper、Creative Cloud Desktop App、Installer、Uninstaller 和 Diagnostics，约 265 MB
- `Adobe Creative Cloud Experience`：CCXProcess，约 167 MB
- `Adobe Sync`：Core Sync 和它的访达扩展，约 165 MB

第四个文件夹 `/Applications/Adobe Creative Cloud` 里只有替身。每个 Adobe 应用另外还有自己的文件夹，比如 `/Applications/Adobe Bridge 2026`，里面是应用本身和一个 `Uninstall Adobe Bridge 2026` 替身，同样的替身在 `/Applications/Utilities/Adobe Installers` 里还有一份。

残留之所以不好找，就是因为这种布局，这些应用藏在两三层文件夹下面，只看 `/Applications` 顶层的列表自然看不到，文件夹又属于 root，访达移动之前会先要管理员密码。

## 哪些东西一直在跑

Creative Cloud 大部分是从旁边这几个文件夹里跑起来的，不是从 Creative Cloud.app 里。登录时 `/Library/LaunchAgents` 里的三个启动项会拉起 Creative Cloud、Adobe Desktop Service 和 CCXProcess，root 守护进程 `com.adobe.acc.installer.v2` 负责 Adobe 的安装辅助程序，Core Sync 还带了一个访达扩展，`Adobe OS Extension` 则在访达右键菜单里加了 Creative Cloud 的选项。

在测试机上把 Creative Cloud.app 移进废纸篓，上面这些一个都没停，Core Sync、CCXProcess、Creative Cloud Helper、Adobe Desktop Service 和 AdobeIPCBroker 照样从废纸篓里跑着，ElevationManager 里以 root 身份运行的 `Adobe Installer` 也还在。这样还在跑的进程下次注销或重启就会停，可负责重新拉起它们的登录项要等有人删掉才会消失。

## 别的 Adobe 应用还要用的部分

Creative Cloud 有一部分文件是共用的，Photoshop 或 Lightroom 还装着时删掉它们，这不是在清 Creative Cloud 的残留，而是把这些应用弄坏：

| 项目 | 为什么要留着 |
| --- | --- |
| `/Library/Application Support/Adobe/Adobe Desktop Common`，约 519 MB | Adobe Desktop Service 在这里，各个 Adobe 应用都依赖它 |
| `/Applications/Utilities/Adobe Installers` | 放着每个已装应用的卸载替身 |
| `/Library/Application Support/Adobe/Creative Cloud Libraries` | 「库」素材的本地副本，Photoshop 和 Illustrator 也会读 |
| `/Applications/Utilities` 里的 `Adobe Genuine Service` | Adobe 的正版验证，文件夹里有它自己的卸载程序 |
| `Adobe PCD`、`AdobeGCClient`，以及 `NGL` 和 `Adobe-Hub-App` 两个群组容器 | 所有 Adobe 产品都在用的授权组件 |

还有两样东西根本不该出现在清理范围里，一是 `~/Creative Cloud Files`，那是你自己的文件，Adobe 停掉旧的同步文件服务时删掉了云端副本，本地这份可能就是仅剩的一份，二是 `/Library/Application Support/Adobe` 和 `~/Library/Application Support/Adobe` 这两个上层文件夹，每个 Adobe 产品都在里面放了东西。

## 按顺序卸载

先把 `~/Creative Cloud Files` 复制到这个文件夹以外的地方，[卸载 Creative Cloud，不丢文件](https://mole.fit/zh/blog/how-to-uninstall-adobe-creative-cloud-mac)里写了退出登录后这个文件夹为什么会被改名，以及复制完怎么核对。

然后卸掉所有 Adobe 应用，在 Creative Cloud 里进入「应用程序（Apps）› 所有应用程序（All Apps）」，点应用旁边的「更多操作（More actions）」，选「卸载（Uninstall）」，Creative Cloud 打不开的话，就双击「应用程序」里该应用自己文件夹中的 `Uninstall` 替身，[Adobe 的离线卸载步骤](https://helpx.adobe.com/creative-cloud/apps/manage-apps/creative-cloud-apps/uninstall-remove-apps-offline.html)说的也是这个替身。退出 Adobe 应用要用它自己的菜单，Bridge 会把普通的终止信号当成崩溃，弹出「意外退出」的报告。

最后才用 Adobe 的 Creative Cloud Uninstaller 卸桌面应用，先选「修复（Repair）」，不行再选「卸载（Uninstall）」，Adobe 只有在其他 Creative Cloud 应用都卸掉之后才让它走完。这个卸载程序其实已经在 Mac 上了，位置是 `/Applications/Utilities/Adobe Creative Cloud/Utils/Creative Cloud Uninstaller.app`，下载地址在 [Adobe 的桌面应用卸载页](https://helpx.adobe.com/creative-cloud/apps/manage-apps/creative-cloud-desktop-app/uninstall-creative-cloud-desktop-app.html)。Creative Cloud Cleaner 是给已经坏掉的安装用的，不是更彻底的卸载，这条路怎么走写在 [Mac 上 Creative Cloud 卸载失败，怎么处理？](https://mole.fit/zh/blog/creative-cloud-uninstall-errors-mac)里。

## 应用已经在废纸篓里时

如果 Creative Cloud.app 是直接拖进废纸篓的，或者卸载程序根本没跑，剩下的部分都还在，包括 `/Applications/Utilities` 里的三个文件夹、放替身的那个文件夹、启动项和守护进程、Adobe Desktop Common 和 Adobe OS Extension。没有别的 Adobe 应用时这些都可以删，资源库里以 `com.adobe.acc…`、`com.adobe.accmac`、`com.adobe.ccd.helper` 和 `com.adobe.CCXProcess` 命名的缓存、偏好设置和日志也一起删掉，Photoshop 或 Lightroom 还装着的话，上面表格里的共用项留着别动。

移走这些 root 文件夹要输入管理员密码，删完重启一次，确保没有进程还在从废纸篓里跑。

## 看看还剩什么

```sh
pgrep -il 'core sync|ccxprocess|creative cloud|adobe desktop service|adobeipcbroker'
ls -d /Applications/Utilities/Adobe* "/Applications/Adobe Creative Cloud"
ls /Library/LaunchAgents /Library/LaunchDaemons /Library/PrivilegedHelperTools | grep -i adobe
```

`pgrep` 没有输出，说明 Creative Cloud 的东西都没在跑，它列出的每一行都是还在跑的进程，登录项删掉以后，重启一次就不会再回来。第二条应该只列出 `Adobe Genuine Service`，还装着别的 Adobe 应用时再加一个 `Adobe Installers`。第三条应该只剩属于正版验证服务的 `com.adobe.agsservice.plist`，还装着别的 Adobe 应用时再加一个 `com.adobe.AdobeDesktopService.plist`。

## Mole 怎么处理

在 [Mole](https://mole.fit/zh/) 里 Creative Cloud 只有一行，它的七个辅助应用都折叠进这一行，因为单独删掉 Core Sync 或 Uninstaller 只会让 Creative Cloud 坏掉、服务照样在跑，Creative Cloud.app 已经没了的话，还装着的第一个组件会接替它，带着同一份清单，删到一半的安装也能一次清完。

移动任何东西之前，Mole 会先让后台组件退出，不肯退出的再强制停掉。Creative Cloud 是最后一个 Adobe 应用时，它的文件夹、启动项、Adobe Desktop Common 和各种缓存默认勾选，还装着别的 Adobe 应用时，共用项根本不会列出来，其余系统级的项目会列出但不勾选。「库」（只在 Creative Cloud 是最后一个 Adobe 应用时列出）、Core Sync 的状态和登录数据会列出但从不预先勾选，`~/Creative Cloud Files`、Adobe 的上层文件夹和授权组件则从来不列。root 文件夹需要输入管理员密码，移进废纸篓后还能找回来，具体有哪些行、每行是什么，写在 [Creative Cloud 卸载指南](https://mole.fit/zh/tested-apps/creative-cloud)里。

## 常见问题

### Core Sync 能删吗？

Core Sync 是 Creative Cloud 的同步引擎，跟着 Creative Cloud 一起删就行，你同步的文件不在它里面，而是在 `~/Creative Cloud Files` 这个单独的文件夹里，删任何东西之前先把它复制出来。

### 为什么不能直接把 Creative Cloud 拖进废纸篓？

它的文件夹属于 root，大部分内容又是从应用旁边的文件夹里跑的，只拖走应用，另外七个应用、登录项和后台服务都还在，每次登录，这些登录项又会把服务拉起来。

### 我删了 Lightroom，为什么 Creative Cloud 还在？

Creative Cloud 负责安装和管理各个 Adobe 应用，但不属于其中任何一个，删掉一个应用不会动 Creative Cloud，也不会动它和其他 Adobe 应用共用的东西，还有别的 Adobe 应用在的时候，Adobe 本来就是这样设计的。

---

Canonical HTML page: https://mole.fit/zh/blog/creative-cloud-leftovers-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
