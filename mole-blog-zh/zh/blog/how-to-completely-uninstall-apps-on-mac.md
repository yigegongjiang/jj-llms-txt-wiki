# 如何在 Mac 上彻底卸载应用，又不误删数据

> 安全移除应用、辅助程序、登录项、扩展和已确认的残留，同时保留文档、共享容器，并单独处理订阅。

Published: 2026-06-19 | Updated: 2026-09-05

结构简单的应用拖进废纸篓，通常就卸载得差不多了，但 VPN、驱动、虚拟化工具等软件还会安装辅助程序、系统扩展、登录项或共享容器，卸载时要移除软件本身，同时保住文档和其他应用仍在使用的数据。

普通应用可以先走这条最短路径：保存工作并退出应用，打开 **Finder › 应用程序**，选择 **文件 › 移到废纸篓**。先不要清空废纸篓，确认相关文件、其他应用、设备和同步仍正常后再清空。只要应用带厂商卸载器、驱动、系统扩展或后台服务，就先不要这样做。

## 先判断：用 Finder 还是厂商卸载器

VPN、杀毒软件、音频驱动、虚拟化软件、云同步客户端、输入设备工具，以及带系统扩展、特权辅助程序或授权组件的应用，都更适合交给厂商卸载器，因为只有它知道停用扩展、卸载特权服务和清理安装记录的完整顺序，确认是普通独立应用、厂商也没有提供卸载流程时，才走 Finder 快速路径。

开始前导出设置或本地数据，并检查订阅、设备授权是否要另外解除，Apple 的[卸载应用说明](https://support.apple.com/102610)也建议优先使用自带卸载器，并提醒删除应用不会取消订阅，也不会删除用它创建的文档。

## 应用仍在使用时怎么办

先正常退出，让应用保存状态并关闭数据库。没有响应时，可打开 **Apple 菜单 › 强制退出**；强制退出可能丢失尚未保存的内容。Finder 仍提示应用正在使用时，重启后在重新打开应用前再试；如果问题依旧，Apple 的[Finder 卸载说明](https://support.apple.com/guide/mac-help/mh35835/mac)建议进入安全模式。持续自动启动的辅助进程，往往也说明应该回头查找厂商卸载器，而不是猜测要删哪个进程。

## 删除应用、文档和订阅是三件事

删除应用和处理文档、订阅是三件事。用应用创建的项目可能在「文稿」、iCloud Drive、外置磁盘或自定义目录；应用支持目录里也可能有唯一的数据库和配置。订阅不会因为应用被删而取消。根据 Apple 的[订阅说明](https://support.apple.com/118428)，App Store 订阅需要打开 **App Store › 姓名 › 账户设置 › 订阅 › 管理**，再单独取消；向厂商购买的订阅则在厂商账户中处理。

## 应用包只是其中一层

`.app` 是应用主体，运行后产生的偏好、缓存、支持文件和后台项目会写到其他目录，删除应用包不会带走这些内容。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/app-uninstall-layers.webp" width="1360" height="454" loading="lazy" alt="Bundle ID 将应用包与支持数据、缓存、偏好设置、容器和启动代理关联起来，经过检查后再移除">
  <figcaption>Bundle ID 可以把应用包与资源库中的相关文件联系起来，确认后再移到废纸篓。</figcaption>
</figure>

小型应用可能只留下几个偏好文件，浏览器、游戏、开发工具和媒体应用则可能留下数 GB，大小不能说明能否删除，其中可能有缓存，也可能有配置、项目、消息、插件和离线内容。

## 记住资源库的位置，但不要只按位置删除

在 Finder 中按住 Option 再打开「前往」菜单，才能进入当前用户的「资源库」。也可以使用「前往文件夹」，Apple 的[路径说明](https://support.apple.com/guide/mac-help/mchlp1236/mac)介绍了这两种方法。常见位置有：

- `~/Library/Application Support/`：支持数据，常是最大的残留
- `~/Library/Caches/`：通常可以重建的缓存
- `~/Library/Preferences/`：`.plist` 设置文件
- `~/Library/Containers/` 和 `~/Library/Group Containers/`：沙盒数据与共享容器
- `~/Library/HTTPStorages/`：部分应用的网站和网络存储
- `~/Library/Logs/`：应用日志
- `~/Library/Saved Application State/`：窗口和会话状态
- `~/Library/LaunchAgents/`：登录后运行的后台代理

与系统结合较深的软件还可能在顶层 `/Library` 中安装内容，它会影响所有用户，更不能按名称批量删除。群组容器可能被同一开发者的多个应用和扩展共用。Apple 的[App Group 文档](https://developer.apple.com/documentation/xcode/accessing-app-group-containers)说明了这种共享关系，只有识别出所有参与者并确认数据去向后，才考虑移除。

## 把 Bundle ID 当作身份证据

macOS 经常用 **Bundle ID** 而不是应用显示名来命名文件，比如 `com.spotify.client`，所以先取得标识符：

```
osascript -e 'id of app "Spotify"'
mdls -name kMDItemCFBundleIdentifier -r /Applications/Spotify.app
```

再把 Bundle ID 和厂商名作为搜索线索，在个人资源库里过滤一遍：

```
find ~/Library -maxdepth 4 \( -iname "*spotify*" -o -iname "*com.spotify*" \) -print 2>/dev/null
```

搜索结果只是候选项，要逐项检查内容、所属应用和用途，删除偏好设置并非卸载必需步骤，还会让以后重装时无法沿用旧设置，占用很小时可以保留。

## 检查登录项和扩展

打开 **Apple 菜单 › 系统设置 › 通用 › 登录项与扩展**，检查「登录时打开」、后台活动和各类扩展。Apple 的[设置说明](https://support.apple.com/guide/mac-help/mtusr003/mac)列出了当前控制项。普通残留登录项可以先关闭并验证，但系统扩展和驱动应由厂商流程停用。Apple 的[系统扩展文档](https://developer.apple.com/documentation/systemextensions/installing-system-extensions-and-drivers)说明了由所属应用停用并在需要时重启的流程。不要根据 plist 文件名猜测 launchd 项的所有者。

## 一套稳妥的手动卸载顺序

1. 导出重要数据，确认近期备份可用
2. 查看厂商说明，应用带辅助程序、扩展、驱动或授权状态时，使用厂商卸载器
3. 退出应用及其可见辅助进程
4. 简单应用先把 `.app` 移到废纸篓，再检查与 Bundle ID 和厂商精确匹配的文件，区分缓存、配置、项目、数据库与共享容器
5. 在 **系统设置 › 通用 › 登录项与扩展** 中检查剩余登录项；系统扩展仍交给厂商流程
6. 只有厂商明确要求或进程仍被占用时才重启，然后测试相关应用、文件类型、设备和同步功能
7. 让删除内容在废纸篓里保留一段验证时间。误删时选择项目并执行 **文件 › 放回原处**
8. 确认无误后再清空。Apple 的[废纸篓说明](https://support.apple.com/guide/mac-help/mchlp1093/mac)提醒，清空后内容会永久删除

## 复杂产品请使用对应指南

- [在 Mac 上卸载 Google Chrome](https://mole.fit/zh/blog/how-to-uninstall-chrome-mac)
- [在 Mac 上卸载 Docker Desktop](https://mole.fit/zh/blog/how-to-uninstall-docker-desktop-mac)
- [在 Mac 上卸载 Adobe Creative Cloud](https://mole.fit/zh/blog/how-to-uninstall-adobe-creative-cloud-mac)
- [在 Mac 上卸载 Microsoft Office](https://mole.fit/zh/blog/how-to-uninstall-microsoft-office-mac)
- [在 Mac 上卸载 Homebrew](https://mole.fit/zh/blog/how-to-uninstall-homebrew-mac)

这些产品会涉及配置文件、共享服务、容器、命令行链接或 shell 配置，通用清单不能替代各自的卸载顺序。

## 卸载工具能减少查找，不能代替判断

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/uninstall.webp" width="2584" height="1741" loading="lazy" alt="卸载检查界面展开了一款应用，列出应用包以及位于资源库 Application Support 和 HTTPStorages 中的残留，每项都显示大小和复选框，底部有移除按钮">
  <figcaption>卸载前列出应用和资源库残留，确认后才执行。这是 Mole 的卸载检查界面。</figcaption>
</figure>

专门的卸载工具可以解析 Bundle ID，汇总可能相关的资源库文件，[Mole](https://mole.fit/zh/mac-app-uninstaller) 会先展示候选项和大小，校验路径，再把普通文件移到废纸篓，它能减少搜索工作，但不能取代厂商卸载器，也不会自动删除共享容器。

## 内部原理：发现不能变成删除

Mole 命令行版与 Mac App 的实现彼此独立，但都把卸载分成发现、检查和确认执行，下面是 Mac App 的流程。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/uninstall-safety-sequence.webp" width="1360" height="454" loading="lazy" alt="安全卸载器依次确认应用身份、发现残留、展示检查计划、等待确认、停止应用和辅助进程、校验每条路径、卸载已批准的任务、把允许删除的项目移到废纸篓，并记录跳过或已删除的结果">
  <figcaption>发现阶段不会删除内容。确认后，每个启动项和文件路径仍要再次通过校验。</figcaption>
</figure>

Bundle ID 只有符合反向域名格式时才参与匹配，避免异常输入放大搜索范围，扫描器会读取内嵌登录项 `Info.plist` 中的标识符，不从文件名猜测。

用户批准计划后，应用才退出目标程序并停止精确匹配的辅助进程，启动项要先通过 `PathGuard`，`DeletionExecutor` 也会在移到废纸篓前再次校验每个 URL，不存在、受保护或被拒绝的路径会明确记为跳过。

## 值得记住的安全原则

系统集成软件用厂商卸载器，普通应用先退出并把程序包移到废纸篓，再按 Bundle ID 和数据类型检查残留。文档、配置、数据库和群组容器在归属确认前保留，订阅单独处理，验证相关应用和服务正常后再清空废纸篓。搜索到路径只代表发现候选项，不代表获得删除权限。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
