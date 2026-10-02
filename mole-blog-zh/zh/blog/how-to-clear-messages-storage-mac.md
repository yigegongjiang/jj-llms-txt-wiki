# 如何清理 Mac 信息占用，又保留聊天记录

> 通过信息应用删除大附件和不需要的会话，先确认 iCloud 同步影响，直接修改聊天数据库风险更高。

Published: 2026-05-28 | Updated: 2026-08-22

「信息」可能保存多年的照片、视频、语音和文稿。启用「iCloud 云端信息」后，在 Mac 删除消息或附件，也会从同一账户的其他设备上删除，Apple 的[信息使用手册](https://support.apple.com/guide/messages/icht1035/mac)也说明了这一点。减少占用前，哪些历史仍需保留是更重要的决定。

## 信息附件存在哪里

数据库和接收文件位于：

```text
~/Library/Messages
```

文字会话通常不大，照片、视频和其他文件集中在 `~/Library/Messages/Attachments`。查看大小：

```shell
du -sh ~/Library/Messages/Attachments 2>/dev/null
```

使用多年的 iMessage 账户可能积累数 GB 附件。终端需要“完全磁盘访问权限”才能读取受保护目录，权限错误不代表目录为空。

## 先决定要保留多久

在 **信息 > 设置 > 通用** 中，“保留信息”可以设为 30 天、1 年或永久。缩短时间后，较早的会话和附件会自动删除，启用 iCloud 同步时，这项修改会影响所有设备上的消息历史。

更改前，保存只存在于聊天中的法律、财务、家庭或项目资料，保留多久取决于这些记录是否还有价值。

## 通过应用删除大附件

不想缩短全部历史时，可以只处理大文件：

- 打开会话详情，查看共享的照片和文件。点击联系人或群组图标，进入“照片”等类别，再用“显示更多”批量选择。唯一副本要先另存
- 确认不需要恢复后再清空“最近删除”。Apple 允许在最多 30 天内恢复消息，30 天后从设备移除，40 天后从 iCloud 永久删除。提前清空会失去恢复时间

部分 macOS 版本还会在 **系统设置 > 通用 > 存储空间 > 信息** 提供附件列表，适合按大小查找，删除前仍要结合会话和文件用途判断。

直接删除 `~/Library/Messages` 中的文件会绕过信息应用，因为数据库、附件、「最近删除」状态和 iCloud 同步记录需要一起更新。

## `chat.db` 与附件目录如何配合

会话保存在 `~/Library/Messages/chat.db` 这个 SQLite 数据库中，收到的文件放在 Attachments 目录，数据库记录会指向对应附件，多年聊天文字通常比媒体小得多。

把保留时间设为 30 天或 1 年时，“信息”会同时删除旧记录和对应附件，启用 iCloud 后，删除还会同步到其他设备。直接从 Finder 删除附件会留下失效的数据库引用。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/messages-chatdb.webp" width="1360" height="454" loading="lazy" alt="chat.db 中的会话记录指向 Attachments 目录里的文件，保留时间会一起清理旧记录和对应附件">
  <figcaption>chat.db 保存会话记录并指向附件文件。“保留信息”会同时清理两部分。</figcaption>
</figure>

## 先用磁盘地图确认附件占用

磁盘地图可以确认“信息”是否占了足够多空间，值得调整保留策略。[Mole](https://mole.fit/zh/) 的“分析”视图能显示隐藏的 Attachments 目录，但不会删除会话，是否保留同步的个人历史，仍要在“信息”中决定。

## 处理“信息”时按这个顺序

可以先测量附件目录，确认是否启用 iCloud 同步，并另存需要保留的唯一文件，应用内少量删除大附件的影响较容易观察，全局保留时间则会影响更多历史。只有愿意让旧记录从所有同步设备上消失时，30 天或 1 年才是合适选项，「最近删除」仍是最后的恢复窗口。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clear-messages-storage-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
