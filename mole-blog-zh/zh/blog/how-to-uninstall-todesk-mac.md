# Mac 上卸载 ToDesk，拖进废纸篓之后还剩什么

> ToDesk 在应用之外还装了启动项、特权助手、音频驱动和一份设备登记文件，逐个说明它们是什么、哪个该留，以及按什么顺序删。

Published: 2026-09-25 | Updated: 2026-09-26

把 ToDesk 拖进废纸篓，删掉的其实只是「应用程序」里那一个应用，它的安装器还在 `/Library` 里放了启动项、特权助手和一个音频驱动，又在 `/etc` 下写了一份设备登记文件，这些东西不删，其中一个服务能在你退出之后把 ToDesk 再拉起来，重装时它也会被认成原来那台远程设备。

我 2026 年 9 月在测试机上用官方安装包装了 ToDesk 5.1.0.0，再用 Mole 卸掉，五个启动项、卸载助手、支持目录、日志和 `~/Library/ToDesk` 都走了，最后剩下两样，一个是音频驱动，一个是只放着一份 `reg.conf` 的 `/private/etc/todesk`，这两样是我手动移进废纸篓的，之后 Mole 就把它们也列了进来。下面这份组件清单最早是从 ToDesk 4.10.1.0 的安装包里量出来的，`/etc` 那个文件夹是 5.1.0.0 这一轮才补上的。

## ToDesk 在应用之外装了什么

ToDesk 官网给的 dmg 其实是个下载器，名字叫 `ToDesk_Installer.app`，它再去下载真正的安装包，我测的这版是 `ToDesk_5.1.0.0.pkg`，安装时要输管理员密码，因为它写的地方远不止「应用程序」：

- `/Library/LaunchAgents` 里的两个启动代理，`com.youqu.todesk.startup.plist` 和 `com.youqu.todesk.session.plist`
- `/Library/LaunchDaemons` 里的三个守护进程，`com.youqu.todesk.service.plist`、`com.youqu.todesk.UninstallerHelper.plist` 和 `com.youqu.todesk.UninstallerWatcher.plist`
- 一个特权助手，`/Library/PrivilegedHelperTools/com.youqu.todesk.UninstallerHelper`
- 一个支持目录，`/Library/Application Support/ToDesk`
- 一个音频驱动，`/Library/Audio/Plug-Ins/HAL/ToDeskOutputDriver.driver`，测试机上是 1.3.0 版
- 一个设备登记文件夹，`/private/etc/todesk`，里面是 `reg.conf`

应用本体在 `/Applications/ToDesk.app`，只要启动项还在，退出它并不能让它一直关着，其中一个服务负责让 ToDesk 保持运行，从你退出应用到它的启动文件被删掉之间，它随时可能把应用重新打开，这类「卸了还在跑」的通用排查写在 [Mac 应用卸了还在后台跑？](https://mole.fit/zh/blog/app-still-running-after-uninstall-mac)里。

## 每一项是什么，哪些要留

| 项目 | 是什么 | 留还是删 |
| --- | --- | --- |
| `/Applications/ToDesk.app` | 应用本体 | 删 |
| 五个 `com.youqu.todesk` 启动文件 | 登录和开机时启动 ToDesk 的服务 | 删，要比它们启动的程序先删 |
| `com.youqu.todesk.UninstallerHelper` | `/Library/PrivilegedHelperTools` 里的特权助手 | 删，等启动文件删完再删 |
| `/Library/Application Support/ToDesk` | ToDesk 的系统支持目录 | 删，等启动文件删完再删 |
| `ToDeskOutputDriver.driver` | ToDesk 的音频输出驱动，由 Core Audio 加载 | 删 |
| `/private/etc/todesk` | 设备登记，只有一份 `reg.conf` | 看你自己，见下文 |
| `~/Library/ToDesk` | `record` 里的会话录屏 | 先看再定，这是你的数据 |

`~/Library/ToDesk` 和其他几项不一样，ToDesk 把会话录屏放在它的 `record` 文件夹里，那是你自己录下的文件，不算残留，我这台是空的，因为我从来没录过，你的那份先打开看看，想留的先复制出来再说。

`reg.conf` 里存的是这台 Mac 在 ToDesk 那边的身份，包括一个随机 ID、一个 GUID、设备名称和一些私有数据，安装器写它的时间和写安装收据是同一秒，删掉应用它也还在，留着的话重装回来还是原来那台远程设备，所以 Mac 要转手或者你想换个新设备身份，就把它删掉，只有打算重装、而且想沿用原来那台设备时才留着。

别在 Mac 上直接搜 `todesk` 然后一股脑删掉，Cursor 和其他用 ToDesktop 打包的应用，标识符里都带着 `com.todesktop`，同样的搜索会把它们一起搜出来，按上面列的确切路径删就行。

## 按这个顺序删

1. 先把 `~/Library/ToDesk/record` 里想留的录屏复制出来。
2. 退出 ToDesk，再把 `/Applications/ToDesk.app` 移到废纸篓，如果 ToDesk 又自己打开了，那就是那个保活服务在拉它，等它的启动文件没了就不会再来。
3. 在访达里选「前往 › 前往文件夹」，先后打开 `/Library/LaunchAgents` 和 `/Library/LaunchDaemons`，把五个 `com.youqu.todesk` 文件移到废纸篓，访达会要管理员密码。
4. 接着移走 `/Library/PrivilegedHelperTools` 里的助手、`/Library/Application Support/ToDesk`、`/Library/Audio/Plug-Ins/HAL` 里的 `ToDeskOutputDriver.driver`，如果决定不要原来的设备身份，再把 `/private/etc/todesk` 也移走。
5. 重启一次，移走启动文件并不会停掉 launchd 已经在跑的服务，所以 Mole 每移一个文件之前都先把对应的服务停掉，手动删的话重启一次效果一样。

启动文件要先于助手和支持目录删，因为指向这两个程序的正是它们，用户资源库那一侧的残留怎么找，可以看 [卸载 Mac 应用后如何清理残留文件](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)。

## 看看还剩什么

```sh
pgrep -il todesk
ls /Library/LaunchAgents /Library/LaunchDaemons /Library/PrivilegedHelperTools | grep youqu.todesk
ls -d "/Library/Application Support/ToDesk" /Library/Audio/Plug-Ins/HAL/ToDeskOutputDriver.driver /private/etc/todesk ~/Library/ToDesk
pkgutil --pkgs | grep youqu
```

`pgrep` 没有输出，说明 ToDesk 的东西都没在跑，打出 `ToDesk` 或 `ToDesk_XPCHelper` 这样的行，就是还开着的进程，启动文件删干净之后重启一次就没了。第二条没有输出，说明五个启动文件和助手都已经不在，打出来的每一行都是还留着的那个。第三条对每个路径最好都回一句 `No such file or directory`，原样打回来的路径就是还在，如果你选了保留 `~/Library/ToDesk` 或 `/private/etc/todesk`，看到它们是正常的。`pkgutil` 打出来的是安装收据，也就是安装器在包数据库里留的记录，不是会运行的程序。

## Mole 怎么处理

在 [Mole](https://mole.fit/zh/) 里，ToDesk 的系统组件来自上面那份确切路径清单，从来不靠搜 `todesk` 去猜，所以 Cursor 和其他 ToDesktop 应用不会被卷进来。五个启动文件、助手、支持目录、驱动和 `/private/etc/todesk` 都会列出来，但一个都不默认勾选，`~/Library/ToDesk` 和 `/var/folders` 下的两个安装器缓存 `com.todesk.downloader`、`com.youqu.todesk.UninstallerClient` 也一样，删哪些由你来勾。

Mole 会先退出 ToDesk、移走应用本体，系统组件要等 `ToDesk.app` 不在了才会动，需要输一次管理员密码，移走的东西都进废纸篓。每个勾选的服务都是在它的启动文件移走之前才停掉，助手和支持目录要等没有任何剩下的启动文件再指向它们时才移。如果勾了 UninstallerWatcher 这个守护进程，Mole 会在移走应用之前先把它关掉，移动失败就再打开。驱动只有在代码签名的 Team ID 和签名标识符都和 ToDesk 对得上时才会移，`/private/etc/todesk` 也只有在它是个真的文件夹、里面除了 `reg.conf` 什么都没有时才会移。如果保活服务中途又把 ToDesk 拉了起来，等所有启动文件都走了，Mole 会再退出它一次，两份卸载器的安装收据，也要等应用和这些系统组件全都不在之后才会注销。

## 常见问题

### /private/etc/todesk 可以删吗？

可以，等 ToDesk.app 删掉之后再删，里面只有 `reg.conf`，也就是这台 Mac 在 ToDesk 那边的设备身份，留着的话重装回来还是同一台设备，删它需要管理员密码。

### 卸载 ToDesk 会删掉我的会话录屏吗？

把应用移进废纸篓不会动它们，录屏在 `~/Library/ToDesk/record` 里，Mole 会列出这个文件夹但不勾选，你不勾它就一直留着。

### 为什么退出 ToDesk 之后它又自己打开了？

它有一个启动服务负责让它保持运行，把应用和五个 `com.youqu.todesk` 启动文件都移进废纸篓，再重启一次，它就不会再开了。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-todesk-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
