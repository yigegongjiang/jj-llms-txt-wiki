# AppCleaner 的替代方案怎么选

> 比较 AppCleaner、Hazel、Nektony 和 Mole，重点看残留怎样归属、共享容器如何保护，以及删除能否恢复。

Published: 2026-07-20 | Updated: 2026-09-27

两款卸载工具扫描同一应用，给出不同文件清单很常见，差异通常来自三件事：怎样判断文件归属、哪些路径始终保留，以及管理员权限能做到哪一步。

## 残留文件怎样归属到应用

**Bundle ID 匹配**会查找 `com.vendor.app` 这类标识符。macOS 常用它命名偏好设置、沙盒容器、Saved Application State、WebKit 和 HTTPStorages。优点是准确，缺点是会漏掉按应用名或厂商名创建的 `Application Support` 目录。

**应用名与厂商名匹配**能找到更多目录，也更容易误报。Notes、Sync、Player 等通用名称可能匹配其他软件，厂商前缀还可能扫到仍在使用的同厂应用。

前者找得少但更准，后者找得多但更冒险。多数扫描结果的分歧都来自这里。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/matching-strategies.webp" width="1360" height="454" loading="lazy" alt="Bundle ID 匹配准确找到三个按标识符命名的文件，却漏掉厂商命名目录，显示名称匹配找到该目录，也额外误报两个属于其他已安装应用的路径。">
  <figcaption>Bundle ID 匹配容易漏，名称匹配容易误报。</figcaption>
</figure>

沙盒应用通常有 `~/Library/Containers/<bundle id>` 主容器，但还可能使用 App Group、Application Scripts、共享缓存、CloudKit 和钥匙串。非沙盒应用则能把数据分散到 Application Support、Caches、Preferences、Logs、WebKit 和 Cookies，归属更难判断。

三类内容要特别谨慎：

- **App Group 容器：** `~/Library/Group Containers` 中的数据可能由同一开发者的多个应用和扩展共享。卸载一款应用时删除它，可能影响另一款仍在使用的应用
- **LaunchAgents 与 LaunchDaemons：** 它们是已注册任务，不只是 plist 文件。应先卸载任务，再删除文件，系统级路径还需要管理员权限
- **特权辅助程序：** `/Library/PrivilegedHelperTools` 中的 root 程序通常与守护进程配对，也要按正确顺序卸载

系统扩展和网络扩展还会注册到 macOS。直接删文件不能可靠停用。VPN、杀毒、虚拟化和音频驱动应使用厂商卸载器。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/leftover-attribution.webp" width="1360" height="454" loading="lazy" alt="一个应用分散到多个状态目录，其中群组容器也被第二款应用使用并标记为不可删除，注册的系统扩展则超出普通文件卸载器范围。">
  <figcaption>共享容器和注册扩展，都超出普通名称匹配的安全范围。</figcaption>
</figure>

## 分歧具体是什么样子

同一款非沙盒应用，开发者又有好几条产品线，三个工具可能扫出三份清单：一份只列应用包和几个按标识符命名的文件，一份加上 Application Support 目录和一个群组容器，还有一份连启动任务也算进去。光看差异不能判断谁有问题；如果群组容器仍被另一款应用使用，把它列入删除清单才是风险。

候选项总大小不是质量分数，列得最多的工具也可能只是加入了共享容器等高风险路径，类别和归属比结果数字更能说明质量。

## 工具选择

### FreeMacSoft AppCleaner

[AppCleaner](https://freemacsoft.net/appcleaner/) 免费并接受捐赠。把应用拖入窗口，检查相关文件清单后删除。对结构简单的应用通常足够。

下载页把 3.6.8 标为兼容 Mojave 至 Tahoe，但[发布记录](https://freemacsoft.net/appcleaner/releasenotes.html)显示该版本来自 2023 年 7 月。可以用于普通残留，群组容器和特权辅助程序仍要仔细核对。

### Nektony App Cleaner & Uninstaller

Nektony 的 [App Cleaner & Uninstaller](https://nektony.com/mac-app-cleaner) 覆盖应用卸载、旧残留、扩展、启动项和应用更新。官网说明支持 macOS 11 及以后，并有近期更新。

[购买页面](https://nektony.com/mac-app-cleaner/buy)同时提供订阅和一次购买。一次购买包含小版本更新，大版本另行收费，有一台、两台和五台 Mac 方案。适合经常需要整理应用、启动项和更新的人。

### AppZapper

[AppZapper](https://www.appzapper.com/) 仍销售一次购买许可证，提供拖放卸载、应用浏览器和授权备注。

但官网没有公开版本号、macOS 支持范围或更新记录，下载也是无版本归档。这不能证明它已停止维护，却让兼容性无法验证。删除系统目录的工具不应带着这种不确定性使用。

### Noodlesoft Hazel

[Hazel](https://www.noodlesoft.com/) 是文件自动化工具，不是完整卸载器。App Sweep 会观察废纸篓：手动删除应用后，列出相关支持文件供选择。

6.1.2 于 2026 年 2 月发布，要求 macOS 13 或更高版本，一次购买，大版本付费升级。它适合在删除发生时立即处理普通拖放应用，不适合审计多年积累的残留。应用自带卸载器时，Hazel 也建议优先使用厂商方案。

### 厂商卸载器

Apple 的[应用卸载说明](https://support.apple.com/102610)建议先查找第三方应用自带的 Uninstall 工具。厂商最清楚怎样停用系统与网络扩展、卸载守护进程、释放设备授权和清理安装凭据。通用文件扫描器无法替代这套生命周期。

### Mole「软件」页

[Mole](https://mole.fit/zh/mac-app-uninstaller) 把应用更新、启动项和卸载放在一个页面。它标记 iOS 封装应用，排除系统关键应用，对旧孤立残留采用保守的默认选择，并清理程序坞条目。root 所有的 `/Applications` 应用包每次会话只请求一次管理员授权。

发现和删除分开，普通移除进入系统废纸篓。它不能取代带驱动、系统扩展、VPN 或杀毒组件的厂商卸载器。授权为一次购买，可用于两台运行 macOS 14 或更高版本的 Mac，包含免费更新，扫描免费，每项破坏性工具在激活前可执行两次。免费 CLI 也支持 `mo uninstall`。

## 快速对比

| 选择 | 查找方式 | 卸载之外 | 授权方式 |
| --- | --- | --- | --- |
| AppCleaner | 拖入应用，匹配相关文件 | 无 | 免费，接受捐赠 |
| App Cleaner & Uninstaller | 扫描应用和旧残留 | 扩展、启动项、更新 | 订阅或一次购买 |
| AppZapper | 拖放与应用浏览器 | 保存授权备注 | 一次购买，无公开版本 |
| Hazel | 删除时观察废纸篓 | 通用文件自动化 | 一次购买，大版本付费升级 |
| 厂商卸载器 | 理解自己的安装清单 | 扩展、守护进程、授权 | 随应用提供 |
| Mole | Bundle ID 加残留类别 | 更新、启动项 | 一次购买，可用于两台 Mac，包含免费更新 |

## 安全卸载顺序

1. 先找厂商卸载器。带驱动、扩展、VPN 或杀毒组件时，直接使用它
2. 应用还能运行时，先导出数据并确认备份，再按需要退出账号、解除设备授权
3. 退出应用和辅助程序
4. 运行扫描器，按类别检查。缓存通常可重建，偏好设置只适合在准备重置时删除，Application Support 可能含本地数据库
5. 仍有同厂应用时，共享群组容器通常还在被使用
6. 启动任务适合由工具先卸载，手动删 plist 可能留下运行中的服务，处理 `/Library` 内容时出现管理员提示很正常
7. 先把内容留在废纸篓，正常使用一天，再清空

普通应用用免费工具往往已经足够。只有经常卸载，而且更新与启动项整合确实省时间时，才需要付费产品。手动步骤见[怎样完整卸载应用](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)，也可以先判断[是否需要清理工具](https://mole.fit/zh/blog/do-you-need-a-mac-cleaner)。

## 常见问题

### AppCleaner 用起来安全吗？

AppCleaner 不会自己决定删什么，它找出与应用相关的文件再列出来等你确认，决定权一直在你手上，这个设计也是它多年口碑的来源。它的风险和所有残留扫描工具一样，来自按名称归属，可能把另一款应用也在用的文件算进来，确认前先把清单读完，落在共享厂商目录里的项目要格外留意。

### App Cleaner & Uninstaller 和 AppCleaner 是同一款吗？

不是，两个名字太接近，确实造成了不少误会。AppCleaner 是 FreeMacSoft 维护多年的免费工具，App Cleaner & Uninstaller 是另一家厂商的商业产品，搜索结果经常把两者混在一起，下载前先看清楚页面上的开发者名称。

### 有必要买一款付费卸载工具吗？

经常卸载应用，统一的清单视图能省下时间时，可以考虑。偶尔卸载普通应用，免费工具或手动步骤通常已经够用；付费产品可能多一些检测和管理功能，但价格本身不能证明删除更安全，仍要看归属判断和删除前的检查。

### 工具不肯碰的残留怎么办？

它们会留在磁盘上，多数情况下这样才是对的。共享框架、授权文件，还有不只一款应用在用的厂商目录，本来就应该在一次卸载之后留下来，删掉它们解决不了问题，还可能弄坏你想留的应用。一款工具留下一小段能说清来历的残留，比每次都报零更值得信任。

---

Canonical HTML page: https://mole.fit/zh/blog/appcleaner-alternative
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
