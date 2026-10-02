# 如何减少 Mac 邮件占用的空间

> 先确认账户与附件来源，再通过邮件应用清理下载和废纸篓，直接修改邮件资料库风险更高。

Published: 2026-06-10 | Updated: 2026-09-05

Apple Mail 为了支持搜索和离线阅读，会在 Mac 保存邮件与附件，时间久了可能占用数 GB。处理前要分清两件事：减少本机下载通常可以恢复，从 IMAP 邮件中移除附件则可能同时修改服务器原件。

## Mail 的数据在哪里

邮箱、邮件文件和数据库主要位于 `~/Library/Mail`，沙盒工作数据位于 `~/Library/Containers/com.apple.mail`。查看大小：

```shell
du -sh ~/Library/Mail ~/Library/Containers/com.apple.mail 2>/dev/null
```

附件通常占得最多，具体取决于账户类型和下载策略。终端或磁盘分析器可能需要“完全磁盘访问权限”才能读取这些目录。

## 分清本地清理和永久修改

**邮件 > 移除附件** 会从邮件本身移除附件，不只是删除临时下载。Apple 在[附件说明](https://support.apple.com/guide/mail/mlhlp1123/mac)中提醒，IMAP 账户上的附件也会从服务器删除，之后无法再从服务器取回，操作前先另存重要附件并打开确认。

如果只想减少以后下载到本机的附件，打开 **邮件 > 设置 > 账户 > 账户信息 > 下载附件**。不同服务商可能提供“全部”“最近”或“无”等选项，这项设置改变今后的离线保存策略，不会立刻清空旧附件，部分媒体也可能继续自动下载。

还可以检查以下内容：

- 垃圾邮件和废纸篓，确认没有需要恢复的内容后再清空
- 旧订阅邮件和自动通知，可以用 **显示 > 排序方式 > 大小** 或搜索找到大邮件，再在 Mail 中删除
- 需要保留的大附件，先导出到普通文件夹，备份并打开验证后再从邮件中移除

直接删除 `~/Library/Mail` 中的文件会绕过邮件应用，邮件、邮箱状态、搜索索引和服务器状态需要一起维护，Finder 无法完成这些更新。

## 重建邮箱用于修复，不用于清理

**邮箱 > 重新构建** 用来处理邮件缺失或搜索错误，并不能直接释放空间。IMAP 和 Exchange 账户在重新构建时，可能丢弃本地副本后重新下载整个邮箱，短时间内还会增加网络和磁盘活动。

Apple 的[Mail 存储空间说明](https://support.apple.com/guide/mail/mlhlp1001/mac)建议检查大邮件，需要时先保存再移除附件，并在确认后清空已删除邮件，只有索引异常时才需要重新构建。

## Envelope Index 与 `.emlx` 文件

Mail 把每封邮件保存成独立的 `.emlx` 文件，另用名为 Envelope Index 的 SQLite 数据库记录发件人、主题、日期和旗标等搜索信息。邮件文字通常不大，主要占用来自附件。

邮箱列表依赖 Envelope Index，直接删除 `.emlx` 会让索引继续指向不存在的邮件。应用内命令能一起更新文件和索引，但“移除附件”仍可能把修改同步到 IMAP 服务器。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/mail-envelope-index.webp" width="1360" height="454" loading="lazy" alt="每封邮件对应一个 emlx 文件和附件，Mail 通过 Envelope Index 数据库显示并搜索邮箱">
  <figcaption>Mail 会让邮件文件、附件和搜索索引保持一致。通过应用编辑，才能同时更新本机和服务器副本。</figcaption>
</figure>

## 先用磁盘地图确认 Mail 占用

Mail 的文件分散在隐藏的 Library 目录，磁盘地图可以先确认它是否值得处理。[Mole](https://mole.fit/zh/) 的“分析”视图负责测量，“清理”视图只处理明确的缓存，邮件和附件仍应在 Mail 中管理，因为只有应用会显示服务器端影响。

## 处理 Mail 时按这个顺序

两个目录的大小只能帮助定位占用，不能单凭大小分清缓存和邮件本身。接下来到「邮件」里检查账户和附件下载设置，不需要的大邮件也在应用内处理。对 IMAP 账户，「移除附件」会永久修改服务器副本，操作前要另存并验证重要附件。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-reduce-mail-storage-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
