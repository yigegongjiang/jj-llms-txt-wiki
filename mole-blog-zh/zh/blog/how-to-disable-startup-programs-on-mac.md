# 关闭 Mac 开机启动项和登录项

> 在登录项与后台项目里逐个关闭，并核对 launchd 归属，避免误关系统服务。

Published: 2026-05-30 | Updated: 2026-09-28

Mac 的自启动项目不只包括登录后弹出的应用，还有后台项目、launchd 任务、系统扩展和配套服务，关闭它们可能缩短登录时间，也可能让同步、更新、快捷键、VPN、备份或安全功能失效，动手前先查清所属应用和用途。

真要动手时，从“系统设置 › 通用 › 登录项与扩展”开始就够了。先看“登录时打开”，再看“允许在后台”，每次只关一项，重新登录后确认同步、更新和快捷键是否还正常。名字看不懂的 launchd 任务，先找所属应用，不要直接删文件。

## 登录项：最直观的一份列表

打开 **系统设置 > 通用 > 登录项与扩展**，Apple 的[登录项使用说明](https://support.apple.com/guide/mac-help/mh15189/mac)介绍了这里的两个主要区域。

“登录时打开”列出登录后自动打开的应用、文档、文件夹或服务器，选中项目并点击减号，只会取消自动打开，不会卸载应用。

## 允许在后台：不一定有窗口的辅助程序

“允许在后台”按开发者显示更新器、同步辅助程序、菜单栏工具和配套服务，关闭开关后，相关应用不能继续以这种方式运行后台任务。

如果进程已经启动，先退出应用或注销一次，再测试菜单栏状态、同步、自动更新和硬件功能。

只想诊断登录变慢时，可以在登录过程中按住 Shift，直到程序坞出现，macOS 会在这次会话中暂时阻止自动登录项，如果问题消失，再回到设置中一次关闭一个候选项。

## 为什么应用退出后还会重新出现

应用可能由 launchd、Service Management 登录项、应用扩展、其他进程或自己的设置重新启动，旧式 launchd 文件常见于：

- `~/Library/LaunchAgents`
- `/Library/LaunchAgents`
- `/Library/LaunchDaemons`

这三个目录并不是完整清单，下面的命令可以只读查看当前用户域：

```
launchctl print gui/$(id -u)
```

服务标签不一定与文件名或应用名相同，单靠进程名猜 plist 容易找错对象，应用设置、macOS 后台开关或厂商卸载器掌握的归属信息更多。

## 怎样安全关闭自启动项

一次关闭一项，必要时注销后再测试，同步、备份、输入设备、VPN、安全防护或更新停止工作时，把它重新打开。

Apple 服务和 `/Library/LaunchDaemons` 不在普通启动项整理的范围内，厂商守护进程也涉及服务状态与特权组件，交给卸载器按顺序处理更合适。

## launchd 的工作方式，以及应用为什么无法保持退出

launchd 管理许多后台任务，macOS 13 以后，Apple 推荐应用通过 [`SMAppService`](https://developer.apple.com/documentation/servicemanagement/smappservice) 注册登录项、代理和守护进程，扩展则有自己的生命周期。

launchd plist 可以使用 `RunAtLoad`、`KeepAlive`、套接字、路径和定时器等条件，`KeepAlive` 只是重新启动的一种原因。`~/Library/LaunchAgents` 中的代理按用户运行，`/Library/LaunchDaemons` 中的守护进程可以在用户登录前运行，通常使用 root 身份，也可以指定其他用户。

从 macOS Ventura 开始，后台任务管理让用户能看到并控制许多第三方项目，但不是每个服务都能变成简单开关，相关数据库属于受保护的系统状态，不应直接编辑。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/launchd-relaunch-loop.webp" width="1360" height="454" loading="lazy" alt="退出应用后，launchd 发现 KeepAlive 为真并重新启动它，形成循环，用户代理与 root 守护进程显示为两个独立层级">
  <figcaption>KeepAlive 是 launchd 的一种重启策略，后台行为也可能来自登录项、扩展和应用设置。</figcaption>
</figure>

## 统一清单有什么用

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/mole-apps-startup.webp" width="2584" height="1741" loading="lazy" alt="Mole 启动项列表，列出 Bob、Shottr、Stash、Raycast 等登录项，每一项都有自己的开关。">
  <figcaption>登录项和后台辅助程序在同一个列表里，每一项都有可以关掉的开关。这是 Mole 的“启动项”视图。</figcaption>
</figure>

[Mole](https://mole.fit/zh/) 会汇总系统登录项和 launchd 项目，并只切换能够精确匹配到已验证任务的内容，身份无法确认的项目会转到系统设置处理，不会直接写受保护数据库或猜测服务标签。

## 一次安全的启动项检查

登录或空闲时的实际影响、项目所属厂商和功能，可以一起说明它是否值得关闭。通过应用设置或「登录项与扩展」一次调整一项，结果也更容易复测，列表更短不一定更好，备份、安全、同步和设备辅助程序的开销往往有必要。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-disable-startup-programs-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
