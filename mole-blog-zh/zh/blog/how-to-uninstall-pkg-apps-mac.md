# 用 pkg 安装的 Mac 应用，怎么卸干净

> 安装包会在应用之外留下文件和一条收据，用 pkgutil 读出它装了什么，只删属于这个产品的部分，最后再忘掉收据。

Published: 2026-09-06 | Updated: 2026-09-26

用 `.pkg` 安装包装上的应用，和从磁盘映像里拖进「应用程序」的应用不是一回事，安装器是以 root 身份跑的，包里写了往哪放它就往哪放，装完还会在安装器的收据数据库里记一笔，把应用拖进废纸篓，只是拿走了这份记录里列着的一个文件夹，其余的东西连同这条记录本身都还在。这篇只讲这种情况，怎么确认应用是 pkg 装的，怎么看它往磁盘上放了什么，哪些该删、哪些该留。如果是访达根本不让删，或者删了又自己回来，先看 [Mac 应用卸不掉：七种常见原因与排查方法](https://mole.fit/zh/blog/mac-app-wont-uninstall)，自己拖进来的普通应用，走 [如何在 Mac 上彻底卸载应用，又不误删数据](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac) 里的常规流程就行。

## 为什么拖进废纸篓不够

安装包可以把文件放在应用旁边，不只是应用里面。我用官方安装器重装再卸载过一批应用，Zoom 的官方 pkg 在 `zoom.us.app` 之外留下了三条 launchd 任务和一个特权助手 `us.zoom.ZoomDaemon`，Logi Options+ 的安装器顺手装了另一个产品 LogiRightSight，带着自己的收据和启动代理，DisplayLink Manager 往 `/Library/LaunchAgents` 里加了一个启动代理，Microsoft Office 会装一个 Word、Excel 和 PowerPoint 共用的框架包，Microsoft AutoUpdate 和一个授权助手也跟着一起装上。这些都不会跟着应用包一起走。

收据也不会走，它存在安装器自己的数据库里，和它描述的那些文件是分开的，所以删掉应用之后，`pkgutil` 照样报告这个包还装着。DisplayLink 的官方卸载程序干干净净跑完一遍，它的两条收据还在。

访达要你输管理员密码，也是安装包留下的一个迹象，安装器以 root 身份运行，留下的应用包也归 root 所有，移动它就要认证，这很正常，[卸载排查指南](https://mole.fit/zh/blog/mac-app-wont-uninstall)里有专门一节。

## 确认它是不是安装包装的

先看收据列表和应用自己的路径：

```sh
pkgutil --pkgs | grep -i zoom
pkgutil --file-info /Applications/zoom.us.app
```

`pkgutil --pkgs` 列出安装器在启动卷上记过的所有包 ID，搜的时候用厂商名和产品名，不要只搜应用的 bundle 标识符，两者经常毫无关系，Zoom 的应用是 `us.zoom.xos`，收据却叫 `us.zoom.pkg.videomeeting`，装 Cookie.app 的收据叫 `app.fantasticthing.Bookkeeping`。连大小写都可能不一样，爱思助手的 bundle 标识符是 `cn.i4Tools.mac`，收据是 `cn.i4tools.mac`，而 `pkgutil` 按 ID 精确匹配，用大小写混写的那个去问，它会回答「No receipt」，收据其实就在那里，所以 ID 从 `--pkgs` 的输出里复制，别自己敲。

`pkgutil --file-info` 按理会在路径下面用 `pkgid:` 行说明是哪个包装了它，但查不到不等于不是包装的，我在 macOS 27 上发现它对大多数路径只打印 `volume` 和 `path`，不管文件还在不在，DisplayLink Manager 的应用就得到了这样一个空答案，而它的两条收据明明还在 Mac 上。反过来，有收据也不代表你亲手运行过 `.pkg`，Mac App Store 装某些应用时也会留一条，比如 `com.apple.pkg.TestFlight`。

## 收据里记了什么

看一个包装了什么、装在哪，用这两条命令：

```sh
pkgutil --pkg-info us.zoom.pkg.videomeeting
pkgutil --files us.zoom.pkg.videomeeting
```

`--pkg-info` 打印包 ID、版本、`volume`、`location` 和安装时间，`--files` 打印的是相对于这个卷和位置的路径，真实路径要把三段拼起来。位置是 `Applications` 的收据，列出来的是 `Foo.app/Contents/...`，位置为空的收据列出来的则是 `Applications/Foo.app/...`，所以先看位置再看列表，不然每条路径都会找错地方。

列表里既有文件也有目录，`--only-files` 和 `--only-dirs` 可以把两者分开，一张收据可能连 `Library` 和 `Library/LaunchAgents` 这种目录本身也列进去，这时候分开看就很有用。原始记录在 `/private/var/db/receipts` 里，每个包一个 `.bom` 加一个 `.plist`，不过 `pkgutil` 的手册写明收据文件存放的位置以后可能会变，要查一律通过 `pkgutil`。

## 收据是地图，不是删除清单

很容易想到把 `pkgutil --files` 的输出丢进一个循环全删掉，按包卸载把别的东西弄坏，多半就是这么来的。这份列表说的是这个包写过什么，不是只有这个包在用什么：

| 列表里有什么 | 为什么不能直接删 |
| --- | --- |
| `Applications`、`Library`、`Library/LaunchAgents` 这样的目录 | 包创建或碰过它们，但 Mac 上别的应用也都在用 |
| 厂商在多个应用之间共用的框架和助手 | Word、Excel 和 PowerPoint 都加载 Office 的共享框架，少了它们一启动就崩溃 |
| 几个应用共用的内容 | GarageBand、Logic Pro、MainStage 和 Final Cut Pro 共用 `/Library/Application Support/GarageBand`、`/Library/Application Support/Logic` 和 `/Library/Audio/Apple Loops`，记在 `com.apple.pkg.MAContent10_*` 这批收据下 |
| 同一个安装器顺带装的另一个产品 | LogiRightSight 跟着 Logi Options+ 装进来，自己单独也能用 |
| 启动代理和守护进程的 plist | 已加载的任务要先停掉再删 plist，否则 launchd 会留着一个磁盘上已经没有定义的任务 |

这份列表也有漏的，它记的是包里的载荷放到磁盘上的东西，安装脚本能做的事更多，安装包里也确实带着脚本，比如 Foxit 的包就有 postinstall 脚本和一个更新服务，应用跑起来之后又会往个人资源库里写自己的数据，这些都不在任何一张收据里，这一侧看 [卸载 Mac 应用后如何清理残留文件](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)。

## 先用厂商的卸载程序

Apple 的[删除应用说明](https://support.apple.com/102610)说得很直接，如果应用自带卸载程序，「这是删除该应用以及它可能存放在其他位置的登录项、扩展或其他数据的最佳方式」。用安装包分发的软件常常自带一个，因为厂商知道自己的脚本、助手和共用部分都在哪，可以去安装包所在的磁盘映像、「应用程序」里这个应用的文件夹、`/Applications/Utilities`、应用自己的菜单，以及厂商的支持网站上找，Splashtop Personal 就把卸载程序和安装包放在同一个磁盘映像里。

如果已经把应用拖进了废纸篓，就用「**文件 › 放回原处**」把它拿回来，打开它，从里面运行卸载程序。安全软件、VPN 客户端和驱动这几类，厂商的工具不是可选项，扩展和网络过滤器只能由拥有它们的应用撤下来，[卸载 Mac 上的杀毒软件，别把网络一起弄断](https://mole.fit/zh/blog/how-to-uninstall-antivirus-mac)里讲了这件事。

## 手动删除

没有卸载程序时，收据会告诉你去哪里找，按这个顺序来：

1. 退出应用和它在后台运行的所有东西。
2. 用 `--pkg-info` 看位置，再用 `--files` 看列表，把应用包以外的路径记下来。
3. 列表里的每个启动代理或守护进程，先用 `launchctl bootout` 卸载任务，再移动它的 plist，为什么顺序要这样，[排查指南里 launchd 那一节](https://mole.fit/zh/blog/mac-app-wont-uninstall)讲过。
4. 只把明确属于这个产品的东西移进废纸篓，包括应用包、以它的 bundle 标识符或产品名命名的文件夹和 plist、`/Library/PrivilegedHelperTools` 里它的助手，共用的上层文件夹和厂商其他应用还在用的东西都留着，`pkgutil --pkgs | grep -i vendorname` 能看出同一厂商还有没有别的包装着，还有的话，共用框架就不要动。
5. 最后才忘掉收据，前提是它列的东西都已经没了：

```sh
sudo pkgutil --forget com.vendor.pkg
```

`--forget` 丢掉收据，手册的原话是「不碰已安装的文件」，先运行它的话，文件一个不少地留着，却再也没有记录指向它们，后面的活只会更难干。它要 `sudo`，因为 `/private/var/db/receipts` 归 root 所有，ID 也要和 `--pkgs` 里的拼写完全一致。过期的收据里没有任何用户数据，拿不准它列的东西还有没有别的应用在用时，收据留着也没什么代价。

## 看看还剩什么

下面这些命令都只读不写，把 `com.vendor.pkg` 和 `vendorname` 换成上面查到的值：

```sh
id=com.vendor.pkg
loc=$(pkgutil --pkg-info "$id" | sed -n 's/^location: //p')
pkgutil --files "$id" --only-files | while IFS= read -r f; do
  p="/${loc:+$loc/}$f"
  [ -e "$p" ] && echo "$p"
done
pkgutil --pkgs | grep -i vendorname
ls /Library/LaunchAgents /Library/LaunchDaemons /Library/PrivilegedHelperTools | grep -i vendorname
launchctl list | grep -i vendorname
launchctl print system | grep -i vendorname
```

那个循环会把收据里仍然存在的文件带着完整路径逐行打印出来，没有输出说明载荷已经全部没了，可以放心忘掉收据，输出落在应用包里说明应用还装着，落在别处的就是删漏的，或者是你因为别的应用共用而决定留下的。如果 `pkgutil` 回答「No receipt for ... found」，说明收据已经忘掉了，或者 ID 拼写不对。`ls` 和 `launchctl` 那几行用来抓收据从来没列过的助手，`launchctl list` 看的是你自己的代理，`launchctl print system` 看的是守护进程，只要打印出一行就说明任务还加载着，第一列是非零数字的正在运行。

## Mole 怎么处理安装包装的应用

在 [Mole](https://mole.fit/zh/) 里移除一个应用时，它会读名字和这个应用的 bundle 标识符或产品名对得上的收据，把其中仍然存在的路径列出来，但只限一个很窄的范围，也就是「应用程序」里的这个应用包，以及 `/Library` 下以这个 bundle 标识符精确命名的 Application Support、Caches、Logs、PrivilegedHelperTools、WebKit 和 HTTPStorages 文件夹，再加上 Preferences、LaunchAgents 和 LaunchDaemons 里的 plist。这些行默认不勾选，读过再逐个勾上，`/Users`、`/opt`、`/usr` 和 `/private` 下的路径永远不会因为收据变成一行，收据文件本身也不会。

应用删掉之后，Mole 会忘掉以它的 bundle 标识符命名的收据，用的是 `pkgutil` 在磁盘上的拼写，还会忘掉通过 `pkgutil --file-info` 找到的厂商收据，后者只在这张收据列的文件一个都不剩时才忘。这一步只在管理员助手已经为这次移除授权过时才做，所以不会为了一条收据多弹一次密码框，查厂商收据时也会跳过 Apple 自己的 `com.apple.*` 收据。

局限就是上面说的那个，`pkgutil --file-info` 在 macOS 27 上很少给出答案，所以 ID 和应用毫无关系的收据，在 Mole 删完应用之后可能还留着，DisplayLink 的两条和 Zoom 的那条都是这样，它们只是记录，`sudo pkgutil --forget` 就能清掉。驱动、VPN 客户端和安全软件这几类，Mole 代替不了厂商的卸载程序。

## 常见问题

### pkgutil --forget 会卸载应用吗？

不会，手册写明它丢掉这个包的全部收据数据，但不碰已安装的文件，要在文件删完之后再运行，不能拿它代替删除。

### pkgutil --files 列出的文件能全删吗？

不能，列表里有 `Library`、`Applications` 这样的系统目录，有厂商其他应用要加载的框架，还有可能仍在运行的任务的 plist，用它找出要看的东西，再删只属于这个产品的那部分。

### 应用已经没了，pkgutil --pkgs 里还有它，是不是还装着？

不一定，删文件从来不会更新收据，收据会一直留到有人把它忘掉，跑一遍上面的检查循环，收据列的东西一样都不剩的话，剩下的就只是这条记录，`sudo pkgutil --forget` 能把它删掉。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-pkg-apps-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
