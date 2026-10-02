# Mac 应用卸载残留，删除前先检查

> 不同工具扫出的残留为什么不一样？按路径确认文件归属，分开看应用和残留大小，保留共享文件与仍需要的个人数据。

Published: 2026-09-09 | Updated: 2026-09-24

把应用拖进废纸篓，移除的是应用本身。偏好设置、Application Support 文件夹、组容器、辅助程序和安装记录，仍可能留在磁盘上。不同卸载工具列出的残留不一样，往往是因为它们判断文件归属的规则不同。

清理前先看候选文件，把应用和残留的大小分开核对。涉及个人数据时，需要你主动勾选，不能只因为文件名相似就一起删除。

应用已经卸载，想继续查找残留，可以看 [Mac 应用卸载后如何清理残留文件](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)。从退出正在运行的应用开始，完整流程见 [Mac 应用彻底卸载指南](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)。这篇主要讲删除前怎样检查列表。

## 残留列表需要说明什么

扫描工具列出一个文件时，需要能回答三个问题：

1. **属于谁：** 这个路径是否属于已卸载的应用，通常要看 bundle identifier，不能只看文件夹名。
2. **能不能删：** 删除后，会不会影响仍在共用这个路径的其他应用。
3. **有没有查全：** 相关残留是否都检查过，包括那些可以列出、但默认不能勾选的内容。

任何一点说不清，多列几个文件都可能增加误删风险。文件大也不代表能删，Application Support 下一个 12 GB 的文件夹，可能是缓存，也可能是本地图库或共享媒体库。

## 残留通常分成哪些类别

Application Support、Containers 和 Group Containers 常被混为一谈，但归属方式不同。Containers 下的每个文件夹都是一款应用的沙盒目录；Group Containers 本来就是共享的，删除前要确认没有其他应用还在使用；Application Support 介于两者之间，很多子文件夹只属于一款应用，但 Google、Microsoft 这类以开发者命名的上级文件夹，可能同时存着好几款产品的数据。

| 类别 | 示例 | 一般怎么处理 |
|---|---|---|
| 应用本身 | `/Applications/Example.app` | 移到废纸篓，或用管理它的包管理器卸载 |
| 用户目录中的应用数据 | `~/Library/Application Support`、Preferences、Caches、Containers、Group Containers | 按 bundle identifier 核对，只删除已确认不要的内容 |
| 安装和更新残留 | pkg 安装记录、Sparkle 残留、后台更新程序 | 确认属于已卸载的应用后再清理 |
| 需要管理员权限的组件 | LaunchDaemons、特权辅助程序、内核扩展、网络扩展 | 优先用开发者提供的卸载程序，不用通用工具强删 |
| 共享媒体 | 多个应用使用的照片、下载文件或文稿 | 可以列出供检查，但不默认勾选 |

涉及系统组件时，Apple 也推荐使用开发者提供的卸载程序，见[删除或卸载应用](https://support.apple.com/102610)。通用清理工具不应在没有说明的情况下，悄悄移除需要管理员权限的辅助程序。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/app-uninstall-layers.webp" width="1360" height="454" loading="lazy" alt="通过 bundle identifier 关联应用本身、支持文件、缓存、偏好设置和后台项目">
  <figcaption>先确认文件属于哪款应用，再检查是否移到废纸篓，共享文件和需要管理员权限的组件要单独核对。</figcaption>
</figure>

## 删除前，按顺序检查这几项

无论手动清理还是使用卸载工具，都可以按这个顺序核对：

1. **确认选中的是要卸载的应用。** 应用可能重名，bundle identifier 才能帮助判断它是谁。
2. **分开看应用和残留的大小。** 应用本身占 400 MB、残留占 90 MB，比一个合计数字更容易核对。
3. **展开完整路径，不只看名称。** 看不出是什么时，在 Finder 中打开上一级文件夹。
4. 浏览器配置、聊天数据库和许可证文件，**只有确定不要了才勾选**，个人数据需要你主动选择。
5. 其他运行中的应用还在使用的文件，**先别碰**，不要强行删除。
6. 临时文件夹即使标为“空”，也要先确认归属，再决定是否删除。
7. 清理中途如果恢复或重装了应用，**立即停止清理**，继续操作可能删掉新安装所需的数据。

最后一条尤其重要。应用重新出现后，旧的残留列表就不能直接拿来删除，卸载工具也应该在这里停下来。

## 用 Finder 和终端手动检查

不使用专用工具时，可以先在终端列出可能相关的文件：

```bash
# List likely user-level residue by bundle-style name (example)
ls ~/Library/Application\ Support | grep -i example
ls ~/Library/Preferences | grep -i example
ls ~/Library/Caches | grep -i example
```

把 `example` 换成开发者名称或 bundle identifier 中容易区分的片段，再逐项核对：

- 确认要删的文件优先移到废纸篓，不直接用 `rm -rf`。
- 先退出相关应用及其辅助程序。
- 除非开发者文档明确列出路径，否则不要动 `/Library` 和 `/System`。
- 同一个 Group Container 仍被其他应用使用时，不能因为其中一款已卸载就删除。

通过 Homebrew 安装的应用，也用 Homebrew 卸载，让安装记录和 cask 管理的文件保持一致。只把应用拖到废纸篓、留下 Homebrew 的安装记录，可能影响以后重装。

## 共享文件和个人数据要单独看

下面这些内容看起来像残留，实际可能仍然有用：

| 看起来像垃圾的内容 | 实际可能保存着什么 | 更稳妥的处理 |
|---|---|---|
| 很大的应用支持文件夹 | 离线邮件、备忘录、项目包 | 不知道如何恢复就保留 |
| Containers 下的浏览器文件夹 | 浏览器配置、Cookie、付款方式 | 需要主动勾选，不批量全选 |
| 聊天应用的媒体目录 | 本地聊天记录和附件 | 先保留，需要时在聊天应用里清理 |
| Unity 或游戏数据文件夹 | 仍有用的项目、已下载资源 | 逐个查看路径 |
| 许可证或证书文件 | 应用激活信息 | 除非打算删除本地激活信息，否则保留 |

清理列表应显示这些内容及其大小，并拦住不能删除的敏感数据和使用中的文件。默认全选这些内容，虽然能让回收数字变大，也更容易误删。

## Mole 能帮上什么忙

Mole 的卸载流程会把应用大小和残留大小分开显示，同一应用不会重复列出。残留可以按路径检查，空的临时文件夹会标为空。浏览器个人数据需要手动勾选，共享媒体也不会默认选中。受保护的敏感数据、其他应用仍在使用的文件不会被删除。清理过程中如果应用被恢复或重装，Mole 会停下来，避免影响新的安装。

在其他地方卸载应用后，Mole 会等相关文件变化稳定下来，再检查残留大小，并说明哪些项被跳过，方便你核对这次清理。

想了解 Library 各目录的用途和查看大小的命令，可以继续看[卸载后的残留文件清理指南](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)。如果正在选工具，[AppCleaner 替代工具对比](https://mole.fit/zh/blog/appcleaner-alternative)介绍了不同选择，各工具的扫描范围和保护规则并不相同。

## 常见问题

### 为什么不同工具扫出的残留不一样？
它们判断归属、排除文件和处理权限的规则不同，结果不一致很常见。比起删除量大的工具，更值得看的是它能否说明某个文件为什么被跳过。

### Application Support 下与应用同名的文件都能删吗？
不能。先核对 bundle identifier，看不清归属的路径要打开检查。共享文件和个人数据，只有确定要删除时才勾选。Application Support、Containers 和 Group Containers 这三类目录不能一概而论。Application Support 存放应用数据，其中可能有数据库、下载的资源库和保存的工作内容，子文件夹也常以应用名或开发者名命名，而不是 bundle identifier，所以每个文件夹都要先确认归属、打开看过再删；Google、Microsoft 这类以开发者命名的上级文件夹，可能被同一开发者的多款应用共用。Containers 是单个应用的沙盒目录，通常跟着这款应用走，但浏览器或聊天应用的 Container 里还存着配置和历史记录，只在确定要删时才勾选。Group Containers 本来就是共享的，两款应用共用一个时，不能因为其中一款卸载了就删掉；只要还有已安装的应用声明同一个 group identifier，这个文件夹就仍在使用，单凭文件夹名称也无法证明归属。

### App Store 应用拖到废纸篓就够了吗？
废纸篓移除的是应用本身，相关数据仍可能保留。通过启动台删除，或将应用移到废纸篓，都是常见的卸载方式。如果还想清理残留，需要另外检查。

### 残留会影响重新安装吗？
有时会。旧的安装记录、后台更新程序或 Homebrew 元数据都可能干扰重装。确认归属后清理这些内容有用，不需要为了腾空间删除无关安装包。

---

Canonical HTML page: https://mole.fit/zh/blog/mac-uninstall-leftovers-review-before-delete
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
