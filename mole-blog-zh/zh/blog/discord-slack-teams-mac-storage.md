# Mac 上的 Discord、Slack、Teams：缓存、历史与下载

> 聊天类 App 靠缓存、历史和下载变大。每个 App 都按同一个判断腾空间，又不丢历史：确认是缓存了再清。

Published: 2026-09-11 | Updated: 2026-09-25

Discord、Slack、Teams 在 Mac 上变大，原因都一样，处理办法也是同一个判断：删任何东西之前，先把缓存、历史记录和下载分开。缓存可重建、清了安全；消息历史和保存的文件不是缓存，误清它们正是这篇指南想避免的坏结果。

这是一篇决策指南，不是「一键清所有缓存」的小贴士清单。文中的「Mole」只指 mole.fit 上的 Mole for Mac，以及它自带的 `mo` 命令行工具；文中不给每个 App 的 GB 数字，因为差别太大没有参考价值。苹果自带的「信息」是另一个 App，清理它的附件见[清理 Mac 上的信息存储](https://mole.fit/zh/blog/how-to-clear-messages-storage-mac)。

<figure class="blog-diagram">
  <picture>
    <source media="(max-width: 900px)" srcset="/img/blog/chat-apps-cache-vs-history-mobile.webp">
    <img src="https://mole.fit/img/blog/chat-apps-cache-vs-history.webp" width="1536" height="1024" loading="lazy" alt="Mac 聊天类 App 留还是清：Discord、Slack、Teams 的三栏速查。缓存是为了提速的临时文件，App 会重建，可以清；历史是本地消息或活动数据，除非想从头开始，否则保留；下载是你从聊天里保存的文件，按需清理。">
  </picture>
  <figcaption>Discord、Slack、Teams 的判断都一样：确认某项不是历史记录、也不是你还要用的下载，再清缓存。</figcaption>
</figure>

## 每个聊天 App 的数据都分这四类

每个桌面聊天 App 分配磁盘的方式都一样。先把你看到的东西归进这四类，下面每个 App 的步骤就一目了然了。

| 类型 | 典型内容 | 怎么判断 |
|---|---|---|
| 缓存 | 媒体预览、头像、临时渲染文件 | 可清，App 会重建 |
| 历史与数据库 | 本地消息库、工作区或服务器状态 | 默认保留，可能是唯一本地副本 |
| 下载与附件 | 你保存或收到的文件 | 逐个复核，有些没有别的副本 |
| 应用本体 | 应用程序里的 App | 只有卸载时才移除 |

顺序很重要：先判断属于哪一类，再动手。数字大不是删的理由，确认是缓存才是。

## Mac 上的 Discord

Discord 会留一份媒体预览和头像的缓存，越用越大。桌面客户端没有官方说明过的清缓存按钮，所以手动处理要收得很窄：先用 ⌘Q 彻底退出 Discord（只关窗口它还在运行），在访达里按 Shift-Command-G 打开 `~/Library/Application Support/discord/`，只把 `Cache`、`Code Cache`、`GPUCache` 这三个文件夹移到废纸篓，Discord 下次启动会重建它们。`Local Storage` 和这个文件夹里的其他内容别动，也别把整个 `discord` 文件夹扔掉，里面还有你的本地设置和登录状态。它在 Mac 上留下的其他文件见[Discord 卸载指南](https://mole.fit/zh/tested-apps/discord)。

Discord 的历史大多存在它的服务器上，所以登录后会重新载入你的对话；本地占用主要是缓存和你下载过的文件。移除前先看一眼下载位置，查看任何 Library 文件夹前先退出 Discord，免得动到它还占用的文件。

## Mac 上的 Slack

Slack 把工作区缓存和本地状态分开，繁忙工作区里占大头的通常是缓存。优先用 Slack 自己的清理缓存或排障步骤（见 Slack 帮助），别去猜文件夹路径。文件夹在哪取决于你怎么装的 Slack：官网下载版放在 `~/Library/Application Support/Slack`，Mac App Store 版运行在沙盒里，数据在 `~/Library/Containers/com.tinyspeck.slackmacgap` 里面。Slack 的排障页面链接见[Slack 卸载指南](https://mole.fit/zh/tested-apps/slack)。

Slack 的 Clear Cache and Restart 会重启 App 并重新载入本地缓存，不会删掉工作区的消息；清除 App 数据是另一个操作，可能会让你登出。保存或导出的文件仍要单独核对。

## Mac 上的 Microsoft Teams

微软给新版 Teams 写了 Mac 上的缓存重置步骤：用 ⌘Q 退出 Teams，删除 `~/Library/Group Containers/UBF8T346G9.com.microsoft.teams` 和 `~/Library/Containers/com.microsoft.teams2` 这两个文件夹，再重新打开 Teams。在访达里把这两个文件夹移到废纸篓效果一样，也比微软文章里的 `rm -rf` 命令更容易撤回。下次启动会慢一些，Teams 要重建它们，可能还要重新登录。聊天和频道消息保存在 Microsoft 365 里，不在这两个文件夹中，所以重置不会丢消息。微软也写明，重置解决不了消息缺失或聊天加载不出来的问题，还会删掉排查需要的诊断日志，所以只在本地客户端出问题时才用。

只动这两个文件夹。Group Containers 里其他 `UBF8T346G9.*` 文件夹属于 Office、OneDrive 等微软应用。如果还留着 `~/Library/Application Support/Microsoft/Teams`，那是旧版经典 Teams 放缓存的地方，只单独复核这个子文件夹，上一层的 `Microsoft` 文件夹别碰，其他微软应用也在用它。从聊天或会议里下载的文件和这些都不是一回事。安装器可能额外装上的组件见[Teams 卸载指南](https://mole.fit/zh/tested-apps/teams)。

## 并排决策清单

1. 先量：在储存空间设置或磁盘视图里找到这个 App 的文件夹，看哪个真的大。
2. 用上面的表格，把每一项归成缓存、历史或下载。
3. 动任何 Library 文件夹之前，先用 App 自己的清缓存步骤。
4. 彻底退出 App，再小心复核它在 `~/Library/Caches` 和 `~/Library/Application Support` 里的条目。
5. 永远不要因为某个 App 看着大就整目录删共享的厂商文件夹，先确认归属。

「这东西可重建吗」背后的通用规则见[哪些 Mac 缓存可以安全删除](https://mole.fit/zh/blog/which-mac-caches-are-safe-to-delete)；卸载后复核 Library 残留见[删除前先复核残留](https://mole.fit/zh/blog/mac-uninstall-leftovers-review-before-delete)。

## Mole 在其中的位置

Mole 1.15（Preview）在「清理」的「通讯工具」分区列出这几个 App，覆盖范围比上面的手动步骤窄：

- Discord：`~/Library/Application Support/discord` 下的 `Cache`、`Code Cache`、`GPUCache`，以及 `~/Library/Caches/com.hnc.Discord`。
- Slack：官网下载版在 `~/Library/Application Support/Slack` 下的 `Cache`、`Code Cache`、`GPUCache`，以及 `~/Library/Caches/com.tinyspeck.slackmacgap`。App Store 版只能看到容器里 `Data/Library/Caches` 下的通用缓存。
- Teams：`~/Library/Caches/com.microsoft.teams2`，以及 `~/Library/Containers/com.microsoft.teams2/Data/Library/Caches` 里的缓存项。它不执行微软的重置，从不提供删除那两个整文件夹，用 Mole 卸载 Teams 时也会留下共用的 `UBF8T346G9.com.microsoft.teams` 组文件夹。

这些行都是可重建的缓存，所以默认勾选。`Application Support` 和 `Containers` 下的行只有在授予完全磁盘访问权限后才出现；Discord、Slack 或 Teams 还在运行时，Mole 会先不列出它的行，所以扫描前先退出 App。历史记录、下载和登录数据都不在清单里。扫描免费、不需要许可证，删什么由你决定。它到底删什么、又拒绝碰什么，见[Mole 安全吗](https://mole.fit/zh/blog/is-mole-safe)。

## 延伸阅读

- [哪些 Mac 缓存可以安全删除](https://mole.fit/zh/blog/which-mac-caches-are-safe-to-delete)：适用于任何 App 的可重建与否规则。
- [清理 Mac 上的信息存储](https://mole.fit/zh/blog/how-to-clear-messages-storage-mac)：苹果「信息」这个另一个 App 的同类活。
- [删除前先复核残留](https://mole.fit/zh/blog/mac-uninstall-leftovers-review-before-delete)：按归属判断 Library 残留。

## 常见问题

### 清缓存会把我登出或丢历史吗？

清缓存后，App 可能重新载入数据或要求登录，但服务器上的消息不会因此消失。Slack 的 Clear Cache and Restart 和清除 App 数据是两回事，后者会让你登出；下载过的文件也要单独核对。

### 能直接删掉整个 Slack 或 Discord 文件夹吗？

删整个 App 文件夹是卸载级别的动作，不是清缓存。它会把本地状态和只在本地的文件一起删掉，所以把它当作移除而不是腾空间。先用 App 清缓存，只有你真的想卸载时才删那个文件夹。Teams 是唯一有官方说明的例外：微软给新版 Teams 的缓存重置就是删掉上面说的两个文件夹，聊天仍保存在 Microsoft 365 里。

### Electron 类 App 都是同一种模式吗？

大体是的。很多桌面聊天 App 基于 Electron，会在 Application Support 下留一份可重建的媒体缓存加本地状态，所以缓存与历史的判断可以照搬。确切的文件夹名仍按 App 和版本不同，删之前先确认。

---

Canonical HTML page: https://mole.fit/zh/blog/discord-slack-teams-mac-storage
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
