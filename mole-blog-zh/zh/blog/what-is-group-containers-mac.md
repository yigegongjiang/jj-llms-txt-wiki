# Mac 上的 ~/Library/Group Containers 是什么，哪些能删？

> 讲清 App Group 文件夹是什么、UBF8T346G9.Office 和 group.com.apple.notes 这类名字怎么读，怎么查出一个组还被哪个已安装的应用在用，以及怎么只删掉卸载后留下的那些。

Published: 2026-09-08 | Updated: 2026-09-26

一台 Mac 用上一段时间，打开 `~/Library/Group Containers` 往往能看到一长列 `UBF8T346G9.Office`、`group.com.apple.notes`、`9K33E3U3T4.net.shinyfrog.bear` 这样的文件夹，有的只有几 KB 设置，有的装着某个应用的整个数据库，也有几个属于早就删掉的应用。名字看着像乱码，空间一紧张就容易被整个清掉，而这正是出问题最多的地方。

这篇讲清楚这个文件夹是什么、名字怎么读、怎么查出一个组还被哪个已安装的应用在用，以及卸载后留下的哪些组可以放心移到废纸篓。

## 这个文件夹是什么

App Group 是开发者写在应用代码签名里的一块共享空间，对应的授权是 `com.apple.security.application-groups`，Apple 的 [App Groups 授权文档](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.application-groups) 里说，同一个开发团队的几个应用靠它共享容器和钥匙串访问组，也靠它互相通信。在 Mac 上每个组对应 `~/Library/Group Containers/<group id>` 这一个文件夹，第一次用到时系统会在里面建好 `Library/Application Support`、`Library/Caches` 和 `Library/Preferences`。

Apple 给了两种标识符格式：

- `group.<group name>`，要在 Apple Developer 网站注册，iOS 用的也是这种。
- `<team identifier>.<group name>`，macOS 专有，以开发者十位的 Team ID 开头，不需要注册。

文件夹名就是标识符，所以直接列一下目录，每个组属于哪种形态就看得出来。

## 为什么应用删了，组还在

一个组很少只有一个成员，主应用、它的小组件、共享扩展或访达扩展、后台辅助工具常常加入同一个组，好读到同一份设置和文件。开发者也可以把几个独立的应用放进一个组，比如微软 Office 的几个应用共用 `UBF8T346G9.ms`、`UBF8T346G9.Office` 和 `UBF8T346G9.OfficeOsfWebHost`，Pages、Numbers 和 Keynote 共用 `group.com.apple.iWork`。

删除这件事上 macOS 和 iOS 不一样，Apple 的 [`containerURL(forSecurityApplicationGroupIdentifier:)` 文档](https://developer.apple.com/documentation/foundation/filemanager/containerurl%28forsecurityapplicationgroupidentifier%3A%29) 写明，iOS 上组里的应用全部删掉后系统会一并删掉组目录，而 macOS 只在应用第一次需要时创建这个目录，之后从不删除，所以把应用拖进废纸篓，它的组会原封不动留在那里。

里面装的东西差别很大，可能是一个很小的偏好设置文件，可能是缓存，也可能是真正的数据。[微软提醒过](https://support.microsoft.com/zh-cn/office/uninstall-office-for-mac-eefa1199-5b58-43af-8a3d-b73dc1a8cae3)，把 Office 那三个共享文件夹移到废纸篓会删掉 Outlook 的数据，WhatsApp 则把收到的媒体放在 `group.net.whatsapp.WhatsApp.shared` 和 `group.net.whatsapp.WhatsApp.private` 里，能占到几十甚至几百 GB。

## 怎么读文件夹名

| 名字形态 | 例子 | 通常代表什么 |
| --- | --- | --- |
| `group.<bundle id>` | `group.com.apple.testflight` | 以某一个应用命名，这里是 TestFlight |
| `group.<bundle id>.<suffix>` | `group.net.whatsapp.WhatsApp.shared` | 锚定在一个应用的完整 id 上，归这个应用或它的扩展用 |
| `group.<shared name>` | `group.com.apple.iWork`、`group.net.whatsapp.family` | 几个应用共用，名字里没有任何一个应用的 id |
| `<TeamID>.<bundle id>` | `9K33E3U3T4.net.shinyfrog.bear` | 单个应用的 Mac 写法，这里是 Bear |
| `<TeamID>.group.<name>` | `G78RJ6NLJU.group.at.EternalStorms.Yoink` | 带团队前缀的组，可能是一个应用也可能是几个 |
| `<TeamID>.<short name>` | `UBF8T346G9.Office`、`UBF8T346G9.ms` | 整个套件共用，以产品或厂商命名 |

有两点让它比看上去难判断。Team ID 只说明是哪个开发者，不说明是哪个应用，Mac 上微软的组全都以 `UBF8T346G9.` 开头，不管属于 Teams 还是 Office。名字长得像 bundle id，也不一定是现在这个应用的 id，Teams 用的组叫 `UBF8T346G9.com.microsoft.teams`，应用本身的 id 却是 `com.microsoft.teams2`。名字只是线索，已安装应用的签名才是证据。

## 为什么按大小排序直接删很危险

这里最大的几个文件夹，通常正是还在用的那几个，按大小一排，排在最前面的多半是邮件库、备忘录数据库或聊天应用的媒体，没有一个是缓存。

Apple 自己的应用也住在这里，`group.com.apple.notes` 和 `group.com.apple.reminders` 属于备忘录和提醒事项，`group.com.apple.iWork` 属于 Pages、Numbers 和 Keynote，以 `systemgroup.` 开头的是系统组，就算名字不认识，它们也都不是残留。

共享组不跟着某一个成员走，删掉 Word 并不意味着 Excel 或 Outlook 还装着的时候 `UBF8T346G9.Office` 就能删，删了只会让剩下的应用被重置或者直接出问题。

组一旦删掉，里面的东西就回不来了，还要用这个组的应用下次会拿到一个新的空文件夹，原来的内容除非你另外留了副本，否则就没了。

## 查出一个组归谁

先只读地看一眼都有什么、各占多大：

```sh
ls ~/Library/Group\ Containers
du -sh ~/Library/Group\ Containers/* 2>/dev/null | sort -h | tail -15
```

第一行列出所有组，第二行列出最大的十五个，最大的在最后。macOS 可能会问终端能不能访问其他应用的数据，拒绝之后，或者文件夹报权限错误、没显示大小，都只是没量到，不代表它是空的。

再看一个已安装的应用是哪个开发者签的、声明了哪些组：

```sh
codesign -dv "/Applications/Microsoft Teams.app" 2>&1 | grep TeamIdentifier
codesign -d --entitlements - "/Applications/Microsoft Teams.app" 2>/dev/null
```

`TeamIdentifier` 那一行应该和你怀疑的那些组的前缀对得上，第二条命令打印应用的授权，它加入的组列在 `com.apple.security.application-groups` 下面。

想找出还有哪些已安装的应用声明了某个组，就逐个搜它们的签名：

```sh
find /Applications ~/Applications -maxdepth 3 -name "*.app" -prune 2>/dev/null |
while read -r app; do
  codesign -d --entitlements - "$app" 2>/dev/null | grep -qF "UBF8T346G9.Office" && echo "$app"
done
```

它打印出的每一行都是一个自己签名里认领了这个组的应用，这个组就得留着。没有输出是很强的信号，但不是定论，这个循环只读每个应用外层的签名，而应用里的小组件、扩展和辅助工具各有自己的签名，装在这两个目录之外的应用它也看不到。

## 哪些可以删

下面几条同时满足，这个组才算可以删掉的残留：

1. 名字里带着你已经删掉的那个应用的完整 id，或者带着一个已安装应用都没用到的 Team ID。
2. 按上面的方法查过，没有任何已安装的应用声明它，`/Applications` 以外的应用也算在内。
3. 不是 Apple 的组，也就是不以 `group.com.apple.` 或 `systemgroup.` 开头，除非它正好是你删掉的某个单独下载的 Apple 应用自己的组，比如 TestFlight。
4. 打开看过里面的内容，确认用不上了，聊天应用的媒体、Outlook 的邮件库，应用删了也还是你的东西。

然后退出这个开发者的所有应用，在访达里用 **前往 › 前往文件夹** 找到它，移到废纸篓，过一段时间再清空。以后重新安装这个应用，它会从一个新的组开始。`~/Library` 里其他地方的残留见 [卸载后怎么清理残留文件](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)，另外哪些地方看着能删其实不能删，见 [Mac 清理工具绝不该删的东西](https://mole.fit/zh/blog/what-mac-cleaners-should-never-delete)。

## Mole 怎么处理

[Mole](https://mole.fit/zh/) 只在卸载应用时才会删组容器，它找的是以这个应用 bundle id 命名的组、带它 Team ID 的组，以及应用本身和里面的扩展、登录项、XPC 服务、辅助工具在签名里声明的组。名字和应用 bundle id 完全对应的 `group.<bundle id>` 默认勾选，`<TeamID>.<bundle id>` 文件夹、只是以应用 id 开头的组、应用声明的共享组都列出来但不勾，如果某个组属于一个还装着、id 更长更具体的应用，就根本不列。

每个组真正移走之前，Mole 会再查一遍，重新读取同一开发者的已安装应用和已启用的系统扩展，只要其中任何一个声明了这个组，或者它检查过的那个应用变了、又被装回来了，或者这份清单没能按时读出来，就拒绝移动，而且一行的放行结论不会拿去给下一行用。Apple 的 `group.com.apple.` 组一律保护，只有 TestFlight 这类单独分发的 Apple 应用自己的那个组例外，所以卸载 Pages 时 `group.com.apple.iWork` 会留给 Numbers 和 Keynote。我在测试机上用 Mole 卸载 Microsoft Teams，留下了 `UBF8T346G9.com.microsoft.teams`、`UBF8T346G9.com.microsoft.oneauth` 和 `UBF8T346G9.com.microsoft.entrabroker` 三个组，Mole 把它们报成受保护路径，没有去猜，Office 的 `UBF8T346G9.Office` 和它的几个同伴也是刻意不碰的。

卸载删掉的东西都进废纸篓。卸载流程之外，Group Containers 在 Mole 受保护的用户数据位置清单里，清理页里的已卸载应用残留从不列 `group.` 开头的组或共享组，能列出的组容器只有 `<TeamID>.<bundle id>` 这种单个应用的文件夹，而且要确认没有任何已安装软件认领这个 id，这一步查不完就一个都不列。其他清理话题可以看 [Mac 上哪些缓存可以安全删除](https://mole.fit/zh/blog/which-mac-caches-are-safe-to-delete) 和 [Office 卸载指南](https://mole.fit/zh/blog/how-to-uninstall-microsoft-office-mac)。

## 常见问题

### 可以把整个 Group Containers 文件夹删掉吗？

不可以，里面有还装着的应用的数据，备忘录、提醒事项、iWork，还有你在用的邮件和聊天应用都在这里，只能在确认没有已安装应用声明之后，一个组一个组地删。

### 为什么应用卸载了，它的组还在？

macOS 在应用第一次需要时创建组文件夹，之后从不删除，而且同一个组可能还被这个开发者的其他应用或扩展用着，把应用拖进废纸篓不会动到它。

### 删了一个还在被使用的组会怎样？

应用会丢掉里面的内容，比如设置、登录状态或本地数据，下次再要这个组时拿到的是一个空文件夹。所以组要移到废纸篓而不是直接删掉，哪个应用出了问题还能放回去。

---

Canonical HTML page: https://mole.fit/zh/blog/what-is-group-containers-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
