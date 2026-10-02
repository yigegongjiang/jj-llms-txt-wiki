# 卸载 Mac 上的杀毒软件，别把网络一起弄断

> 安全软件可能包含系统扩展、网络过滤器、后台任务和管理配置，拖走应用未必能完整移除。按厂商流程卸载，再核对残留组件与网络状态。

Published: 2026-08-15 | Updated: 2026-09-05

杀毒软件通常不只有应用包，还可能包含系统扩展、网络过滤配置、launchd 任务和特权助手，受管理的 Mac 上还可能有配置描述文件。把应用包拖进废纸篓，未必能一并移除这些组件，所以第一步应查看厂商的卸载说明，而不是直接用访达删除。

## 除了应用包，安全类产品还装了什么

任何一台 Mac 上，一分钟就能只读地清点一遍它到底装了什么：

```
systemextensionsctl list
ls -la /Library/LaunchDaemons /Library/LaunchAgents ~/Library/LaunchAgents
ls -la /Library/PrivilegedHelperTools
profiles list
pkgutil --pkgs | grep -i vendorname
```

**系统扩展**可以承载检查文件和流量的代码，许多产品用它替代旧的内核扩展，但不能把旧教程里删除 `.kext` 的步骤直接套到当前版本。Apple 的[系统扩展指南](https://developer.apple.com/documentation/systemextensions/installing-system-extensions-and-drivers)说明，系统扩展位于应用的 `Contents/Library/SystemExtensions` 文件夹内，通常需要用户批准，受管理设备也可能由策略预先批准。Apple 的[部署技术说明](https://developer.apple.com/documentation/technotes/tn3134-network-extension-provider-deployment)还指出，以这种方式打包的提供程序运行在独立于登录用户的全局环境中。

**网络内容过滤器或透明代理**是另一个独立对象，断网的多半就是它。让它生效的那份配置属于系统，不属于应用。[NEFilterManager](https://developer.apple.com/documentation/networkextension/nefiltermanager) 说得很直接：「过滤器配置保存在由 Network Extension 框架管理的 Network Extension 偏好设置中」，而且只有拥有它的应用显式保存时，改动才会生效。

**launchd 任务**负责把后台部件拉起来。`/Library/LaunchDaemons` 在开机时以 root 身份运行，比任何人登录都早，`/Library/LaunchAgents` 在每个用户登录时运行，`~/Library/LaunchAgents` 只在当前账户运行。这些目录里的 plist 是一条向 launchd 注册的记录，不是普通的设置文件。

`/Library/PrivilegedHelperTools` 下的**特权助手**是一个 root 所有的可执行文件，配着一个守护进程，文件挪走之前该先把它卸载。公司或学校发的 Mac 上还有**配置描述文件**，带着扩展批准和过滤器载荷，软件装的时候不用问任何人，同一套机制也能把它装回来。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/antivirus-install-surface.webp" width="1360" height="454" loading="lazy" alt="一个杀毒安装程序展开成多样东西：应用程序里的一个应用包、包内的一个系统扩展、保存在 Network Extension 偏好设置里的一份网络过滤配置、注册到 launchd 的守护进程与代理、一个特权助手工具，以及一份配置描述文件。">
  <figcaption>应用包之外，还有独立文件和由 macOS 管理的注册信息，不能靠拖走一个应用包完成卸载。</figcaption>
</figure>

## 为什么拖进废纸篓不可能管用

Apple 确实写了一条好消息，但它比第一眼看上去窄得多：「当用户删除对应的应用时，系统会自动卸载任何系统扩展。」这一条覆盖的是应用包内部的那个扩展，对其他注册记录只字未提，而那些东西没有一个在包里。

过滤器配置是需要检查的一处。它由 Network Extension 偏好设置管理，厂商卸载流程需要处理相应配置，不能假定拖走应用就完成了这一步。若过滤器仍启用，提供程序却已缺失，可能出现 DNS 查询失败、网页卡住或连接超时，即使 Wi-Fi 信号满格也一样。不过这些症状也有其他原因，仍要核对过滤器状态和实际网络连接。

launchd 任务也要检查。删掉可执行文件，不等于注销负责启动它的任务，某些保活配置可能反复尝试启动失败的服务，但不能只凭风扇变响就断定是这个原因。反过来，删除 plist 也不等于卸载已加载的任务，它可能继续保留到被显式卸载或系统重启。特权助手和配置描述文件需要分别处理，设备管理的部署策略也可能重新安装被移除的软件。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/antivirus-half-removal.webp" width="1360" height="454" loading="lazy" alt="应用包已经被移进废纸篓，而网络过滤配置、已加载的守护进程、特权助手和配置描述文件全都活了下来，造成 DNS 失败、反复重启的循环，以及被重新装回来的软件。">
  <figcaption>未移除的过滤器、后台任务和管理配置，都可能让卸载后的问题继续存在，需要分别核对。</figcaption>
</figure>

## 难卸是一个设计决定

安全软件需要防止未经授权的停用。系统扩展通常要由用户在设置中批准，受管理的 Mac 也可能由设备管理策略预先批准，停用则应走厂商支持的流程。Apple 的[提示说明页](https://support.apple.com/120363)指向 macOS 15 及以后的登录项与扩展，更早的版本在隐私与安全性里。直接拼一串 `rm` 命令，不能代替这些注册信息的正常移除。

## 铁律：先跑厂商自带的卸载器

永远如此，而且要排在动任何别的东西之前。只有承载它的那个应用能为自己的扩展提交停用请求，只有保存了那份过滤配置的应用能把它从 Network Extension 偏好设置里删掉，也只有厂商自己的工具清楚，停止顺序一旦搞错，哪个组件会把别的组件重新装回来。第三方卸载工具拿不到这些句柄，所以残留扫描器，包括 [Mole](https://mole.fit/zh/)，管的是厂商工具跑完之后剩下的东西，替代不了它。

如果你已经把应用拖进了废纸篓，用**文件 > 放回原处**把它放回去，打开它，用它自己的卸载器。下面各节讲的是每家厂商把卸载器放在哪。

## 每家厂商的卸载器在哪

下面每个链接都指向厂商自己的支持站，最新步骤以那里为准。支持页面会挪位置，两边说法不一致时，跟着页面走，不要跟着这段摘要走。

### McAfee

卸载器已经在 Mac 上了，就在应用程序里，名字随产品而定，Total Protection 和 LiveSafe 用的不是同一个。McAfee 的[这篇文章](https://www.mcafee.com/support/s/article/000002432?language=en_US)要求以管理员身份登录，退出每一个 McAfee 应用，然后打开**前往 > 应用程序**双击那个卸载器。有两步能说明为什么要照着读、别自由发挥，它提醒 macOS 弹出系统扩展提示时要点允许，而且把重启写成了最后一个编号步骤，不是一句建议。程序坞上可能会残留一个已经失效的图标，得手动拖走。

### Norton

同样是应用程序里已有的一个独立卸载器。Norton 的[卸载页](https://support.norton.com/sp/en/us/home/current/solutions/v134746139)走的是**前往 > 应用程序**，双击 **Norton Uninstaller**，然后「输入管理员账户密码，点击 **Install Helper**」，再输一次密码，最后 **Finish**。安装助手这一步是卸载器拿权限的地方，它得靠这份权限去拆自己那些系统级组件。

### Avast

应用内卸载，它的页面一开头就把最省事的那种替代做法先排除掉了：「把 Avast Security 拖进废纸篓或者使用 Clean My Mac，都不会完整卸载这个应用。」[文档给的路径](https://support.avast.com/en-us/article/uninstall-mac-security/)是菜单栏里的 **Avast Security > Uninstall Avast Security**，点 **Continue**，然后输入 Mac 开机密码并点 **Install Helper**。同一个卸载器也放在**应用程序**里，叫 **Avast Security Uninstaller**，应用打不开的时候用它。

### Bitdefender

是一个专门的卸载程序，不是拖进废纸篓。[卸载页](https://www.bitdefender.com/consumer/support/answer/1784/)要求打开 Bitdefender 文件夹，双击 **Bitdefender Uninstaller**，勾选产品，点 **Uninstall**，再输入管理员用户名和密码。同一轮还会问要不要一并删掉 Bitdefender VPN，不打算继续用就点同意，那个组件自己也注册了一个网络扩展。

### Malwarebytes

一个应用内的菜单项。[卸载文章](https://help.malwarebytes.com/hc/en-us/articles/31589300070683-Uninstall-Malwarebytes-for-Windows-and-Mac)让你走菜单栏的 **Help > Uninstall Malwarebytes**，确认一次，再输 Mac 密码。这一类里少见的是，同一页也接受直接用访达删除，厂商等于替自己占的那点地方表了个态。旧的 `support.malwarebytes.com` 卸载文章现在会跳转，用 `help.malwarebytes.com` 那一页。

### Sophos

两款产品，两个答案。面向消费者的 **Sophos Home** 用一个独立应用 **Remove Sophos Home**，从聚焦里启动。[那一页](https://support.home.sophos.com/hc/en-us/articles/115005499786-Uninstalling-Sophos-Home-on-Mac-computers)对另一种做法说得毫不客气：「不要把 Sophos Home 拖进废纸篓，那样并不会卸载这个程序。」它会装一个助手，问你要密码，还需要重启。同一页附了一节可选内容，讲卸载之后怎么再移除 Sophos 的系统扩展，那套流程把 `systemextensionsctl uninstall` 夹在关闭和重新开启系统完整性保护之间。边界在哪，这一句说得最清楚，哪怕是厂商自己的卸载器，也不总能把自己的扩展一并带走。这些步骤是可选的，厂商自己就这么标的，系统完整性保护值得一直开着。

面向企业的 **Sophos Endpoint** 用启动台里的 **Remove Sophos Endpoint**，但[文档](https://docs.sophos.com/esg/endpoint/help/en-us/help/Uninstall/)加了一道门：「如果防篡改保护已开启，你需要先关闭它才能卸载 Sophos Endpoint。」这需要 Sophos Central 管理员，或者存在那个控制台里的单机密码，公司发的 Mac 上，这件事轮不到用户自己做。

### Kaspersky

应用内，走支持面板。[卸载页](https://support.kaspersky.com/us/kaspersky-for-mac/25/118671)走的是 **Help > Support**，**Uninstall**，再点一次 **Uninstall**，然后输入管理员凭据，并且提醒你 Chrome 和 Firefox 的扩展会比应用活得更久。Kaspersky 还发布了一个独立的[清除工具](https://support.kaspersky.com/16048) `kavremover-mac`，应用内那条路走不通时用它。两个页面都标着 2024 年的日期，所以要对着自己的版本再确认一遍。

## 如果 Mac 受管理，答案在 IT 那边

在这件事上花掉一整晚之前，先确认这个决定是不是轮得到你做：

```
profiles status -type enrollment
profiles list
```

第一条不需要管理员权限就能打印 DEP 和 MDM 的注册状态，第二条列出当前账户安装的配置描述文件，同样的信息也出现在**系统设置 > 通用 > 设备管理**里。Apple 的[设备管理指南](https://support.apple.com/guide/mac-help/mh35474/mac)把后果说得很直白：「有些描述文件只能由这台 Mac 的系统管理员移除。如果你无法移除某个描述文件，这台 Mac 可能是受管理的电脑。」

在受管理的 Mac 上，安全软件是那套管理的一个载荷。你在本地删掉，描述文件会把它装回来；想在本地关掉防篡改保护也做不到，密码在一个你进不去的控制台里。找 IT 把它取消分配。

## 怎么验证真的卸干净了

半卸载在这一步抓得住，代价也低。要在厂商卸载器跑完、Mac 重启之后再做，因为下面好几项检查只有开过一次机才说真话。

**1. 系统扩展。**

```
systemextensionsctl list
```

在一台什么都没装的 Mac 上，输出是一行 `0 extension(s)`。否则每一行会写出团队标识符、扩展的 bundle 标识符、版本，以及方括号里的状态。哪一行还带着刚删掉那家厂商的标识符，就说明停用没走完。这个工具确实提供 `systemextensionsctl uninstall <teamID> <bundleID>`，有些厂商也正是为这种情况写了文档，但只有厂商页面让你用的时候才去用它。

**2. launchd 注册记录。** 在三个位置里找厂商的标签，再问 launchd 实际加载了什么：

```
ls -la /Library/LaunchDaemons /Library/LaunchAgents ~/Library/LaunchAgents
launchctl list | grep -i vendorname
sudo launchctl list | grep -i vendorname
```

两条 `launchctl list` 查看不同范围：第一条是当前登录会话，第二条是系统域。有 plist 而没有已加载任务，可能是未加载、已禁用或遗留的定义；有已加载任务而找不到 plist，也不代表它只能靠重启移除。即使两边都有，也要看任务状态才能判断是否正在运行。Mole 会把这三个目录中的项目放在一份清单里，方便继续核对。

**3. 网络过滤器。** 检查**系统设置 > 网络**中的过滤器项目。应用移除后仍有对应项，是继续检查厂商卸载流程的线索，不足以单独证明它造成断网；没有看到项目，也要结合系统版本和其他检查判断。

**4. 登录项与扩展。** 打开**系统设置 > 通用 > 登录项与扩展**，登录列表和下面的后台项都要读一遍，厂商组件在这里比应用活得久是常事。

**5. 正在运行的进程。** 在活动监视器里搜厂商名，并从**显示**菜单切到**所有进程**，这样也能检查 root 所有的守护进程。重启后再核对，搜索结果为空只是线索，因为辅助程序可能使用不同名称，还要结合执行路径和前面的注册信息。

**6. 安装记录。** `pkgutil --pkgs | grep -i vendorname` 会说出安装程序写了什么，等活跃组件都清掉之后，它能给你一批路径，值得再去 `~/Library/Application Support` 底下核对一遍。

这些检查要结合起来看，前三项没有结果，还不能证明所有后台组件都已移除，登录项和运行中的进程也要确认。剩余文件是否只是占用空间，要看它有没有被其他任务继续使用。

## Mole 在这件事里做什么，又拒绝做什么

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/uninstall.webp" width="2584" height="1741" loading="lazy" alt="一个卸载审核界面，某个应用被展开，显示它的应用包以及 ~/Library/Application Support 和 ~/Library/HTTPStorages 下的残留项，每一项带大小和复选框，底部是删除按钮。">
  <figcaption>一次卸载会碰到的每一项，都在任何东西移动之前连着路径和大小列出来。低置信度的行默认不勾选，所以默认动作永远是更小的那个。</figcaption>
</figure>

[Mole](https://mole.fit/zh/) 的「软件」页把应用清单和启动项放在同一屏，所以一款产品注册的启动代理、守护进程和登录项，就摆在注册它们的那个应用旁边，各自带着真实路径。

对于残留，Mole 按 bundle 身份匹配而不是按名字，它认识的系统级路径就是上面那些：`/Library/LaunchDaemons/<bundle-id>.plist`、`/Library/LaunchAgents/<bundle-id>.plist`、`/Library/PrivilegedHelperTools/<bundle-id>`，以及 `/private/var/db/receipts` 下的安装记录。每一个系统级匹配都是「待审核」置信度，默认不勾选，你读完每条路径再逐个勾上，而不是从一份预先勾好的清单里往下取消。删除进废纸篓，出了错是拖回来，跳过项和失败项都会报出来，全部在本地跑。

有两处拒绝比上面这些都重要。Mole 会扫 `/Library/SystemExtensions`，看有没有扩展属于正在卸的这个应用，找到就给一条提示，点名那个扩展，说明它可能在应用删掉之后还留着。它不会去尝试停用，它做不到，那次请求必须由拥有它的应用发出。还有一类厂商产品，卸载被锁死、受管理，或者拆着卸就会出事，包括 ESET、CrowdStrike、SentinelOne、Jamf、Palo Alto GlobalProtect 和 Cisco Secure Client，Mole 干脆拒绝卸载，直接指向厂商官方的卸载器。

Mole 不处理恶意软件，不做备份，也替代不了带驱动、VPN 组件或系统扩展的软件自带的卸载器。对杀毒软件来说，Mole 处理的是厂商卸载器跑完之后的残留。在终端里，免费开源的 [Mole CLI](https://github.com/tw93/Mole) 提供 `mo uninstall`，它支持 `--dry-run`，可以先把路径清单读一遍。

## 常见问题

### 删掉杀毒软件之后网就断了，怎么办

几乎可以肯定是一份网络过滤配置活得比应用久。它住在 Network Extension 偏好设置里，由框架独立于应用包来管理，所以删掉应用之后，剩下一个过滤器还开着，它指向的提供程序已经不在了。去**系统设置 > 网络**看有没有「过滤器」条目，再用 `systemextensionsctl list` 看有没有活下来的扩展。解法是把厂商的产品装回来，跑它自己的卸载器。为了删掉一个软件先把它装回来，是别扭，但这确实是最短的路。

### 我能不能直接删掉 /Library/LaunchDaemons 里的 plist

不要把这当作第一步。plist 是任务定义，删掉它不等于卸载已加载的任务。先运行厂商卸载器并按要求重启，再结合 `sudo launchctl list`、其他会话范围、文件归属和实际进程判断残留。单次列表没有结果，不足以证明文件可以删；仍在使用的软件所需的 plist 应保留。

### 厂商卸载器要了密码然后失败了，接下来怎么办

查三件事。这台 Mac 是不是受管理的，如果是，受防篡改保护或者某份描述文件限制，答案在 IT 那边。产品是不是还在跑，有些卸载器在自己的守护进程还占着文件时收不了尾，重启再试一次。还有卸载器是不是当前版本，有些厂商会另外发一个清除工具，跟应用内那条路是分开的。三件事都干净的话，下一站是厂商支持。

### 检查是否卸干净之前需要重启吗

需要。扩展的拆除和 launchd 的注销都要跨一次开机才完成，所以卸载器刚跑完就去查，可能查出一批本来就已经排队要消失的残留，也同样可能漏掉一个还会回来的守护进程。

## 延伸阅读

- [怎么彻底卸载 Mac 应用](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)，讲通用顺序、bundle 标识符和共享容器。
- [卸载 Mac 应用后怎么清理残留](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)，讲活跃组件确认清掉之后的残余。
- [Mac 清理工具永远不该删什么](https://mole.fit/zh/blog/what-mac-cleaners-should-never-delete)，是同一种判断的另一面。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-antivirus-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
