# 如何更新 Mac 上的所有应用

> App Store、官网、Homebrew 和 Electron 应用的更新方式不同。先确认安装来源，再选择正确入口。

Published: 2026-07-17 | Updated: 2026-09-28

Mac 应用可能来自 App Store、开发者官网、Homebrew 或企业管理系统，不同来源有不同的签名、回退和自动更新规则，更新前先确认安装渠道，避免丢失授权，或让同一应用出现两个分别受管理的副本。

| 安装来源 | 权威更新入口 | 不要随意替换成 |
|---|---|---|
| Mac App Store | App Store 的更新 | 覆盖商店托管副本的厂商下载包 |
| 开发者网站 | 应用内更新器或厂商下载页 | 未明确迁移就改用 App Store 或 Homebrew |
| Homebrew formula 或 cask | `brew outdated` 与 `brew upgrade`，部分 cask 使用应用自更新 | 未确认 cask 配置就混用更新方式 |
| 企业管理 | 组织的管理门户 | 本地管理员绕过方案 |

## App Store 应用

App Store 只更新通过商店安装的应用，打开 App Store，在侧边栏选择“更新”，自动应用更新位于 **App Store > 设置**，与 macOS 的“软件更新”分开，Apple 的 [App Store 更新说明](https://support.apple.com/guide/app-store/fir9b01adda3/mac)列出了这两个入口。

安装了第三方命令行工具 `mas` 时，可以查看它识别出的商店应用：

```
mas list
```

应用没有出现在结果中，不足以证明它并非来自 App Store，收据、账号权限和工具限制都可能影响识别。

## 从开发者网站下载的应用

官网直装应用可能使用 Sparkle、Electron 更新器、自建服务，也可能只提供下载页，许多应用会在菜单中提供“检查更新”，有些只在运行时检查，有些会安装登录项或后台辅助程序，具体行为要看应用自己的更新设置。

## Homebrew 应用

先查看过期项目，再升级指定软件包或全部项目：

```
brew outdated
brew upgrade wget
```

把 `wget` 换成 `brew outdated` 返回的 formula 或 cask 名称，不带名称执行 `brew upgrade` 会升级所有过期项目。

刷新软件包元数据和升级软件是两件事，`brew upgrade` 才会执行这里的软件包升级，部分 cask 中的图形应用也有自己的更新器。根据 [Homebrew 常见问题](https://docs.brew.sh/FAQ#how-do-i-update-my-local-packages)，Homebrew 能比较一部分 cask 的应用版本，标记为 `version :latest` 或内容难以判断的 cask 可能会被跳过，重要服务和开发工具链升级前，先查看 `brew info` 与不兼容变更。

## 更新 macOS 本身

macOS 系统更新使用独立渠道，先在终端列出可用更新：

```
softwareupdate -l
```

也可以打开 **系统设置 > 通用 > 软件更新**，macOS 更新不会替你更新所有第三方应用，安装前先备份，并确认驱动、虚拟化软件、音频工具和其他系统级软件兼容，Apple 的 [macOS 更新说明](https://support.apple.com/108382)也建议先备份。

## 安全地更新，而不只是尽快更新

- 确认近期备份可以恢复，并为关键工具保留安装包或归档
- 阅读发布说明，关注兼容性、数据库迁移和最低系统版本
- 确认官网下载来自厂商，应用已有代码签名或公证
- 先更新少量非关键应用，检查启动、文档、插件和后台服务后再继续
- 除非准备迁移管理权，否则保持原来的安装渠道

回退可靠、兼容记录稳定的应用适合自动更新，生产工具链、插件宿主、驱动和维护窗口很窄的电脑，更适合分批手动更新。

## 为什么 macOS 没有一个能更新所有应用的按钮

App Store 依靠 Apple 收据管理软件，Sparkle 使用开发者发布的 appcast，配置正确时会同时校验 EdDSA 签名和 Apple 代码签名，具体机制见 Sparkle 的[安全说明](https://sparkle-project.org/documentation/security-and-reliability/)，Electron 应用和厂商更新器使用各自的订阅源，Homebrew 则按 formula 与 cask 升级。

这些渠道彼此独立，统一列表需要保留应用来源和安装规则。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/update-channels.webp" width="1360" height="454" loading="lazy" alt="App Store、Sparkle appcast、Electron 订阅源、Homebrew 和 GitHub Releases 五个独立更新渠道各自连接到一款应用，彼此不可见，Mole 再把它们读入统一更新列表">
  <figcaption>统一清单会汇总更新渠道，但每款应用仍按原来的来源、签名和安装方式更新。</figcaption>
</figure>

## 统一更新清单有什么用

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/mole-apps-updates.webp" width="2584" height="1741" loading="lazy" alt="Mole 更新列表，列出已安装的 App 和它们的版本、大小、最近使用时间，每个都标着更新来源，比如 App Store、Homebrew 或 Electron。">
  <figcaption>所有 App 在同一个列表里，每个都标着更新渠道，你能看清该交给哪个更新器。这是 Mole 的“更新”视图。</figcaption>
</figure>

[Mole](https://mole.fit/zh/) 的“软件”页面把多种渠道汇总到一份清单，并保留每款应用的来源，它省去了逐个寻找入口的时间，但不会把所有更新变成同一种操作。

## 更新时按这个顺序

列出应用来源，通过对应渠道检查更新，阅读兼容说明并备份关键状态，再分批安装，每批完成后，用原来的文档和工作流复测，无需一次把所有版本号变成最新，先保住可用环境。

每一批更新后，打开应用，从“关于”面板或 bundle 版本确认真正启动的副本，不要假定安装器一定覆盖成功。再验证一个与你工作有关的真实文档、插件、设备或后台服务，最后回到原更新渠道复查。仍显示过期，往往意味着磁盘上有两份应用，或更新器替换的不是你启动的那个 bundle。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-update-mac-apps
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
