# 如何安全删除 Mac 上的旧 iPhone 备份

> 先确认仍有可用恢复路径，再通过 Finder 识别、归档或删除完整的 iPhone 与 iPad 本地备份。

Published: 2026-06-15 | Updated: 2026-09-05

旧 iPhone 和 iPad 备份可能占掉几十 GB，而目录使用长标识符命名，单看文件夹看不出设备、日期和恢复点。Finder 能识别完整备份，也更方便在确认其他恢复路径后处理过期内容。

## 备份保存在什么位置

设备备份到 Mac 时，数据会进入：

```
~/Library/Application Support/MobileSync/Backup
```

查看总量：

```
du -sh ~/Library/Application\ Support/MobileSync/Backup 2>/dev/null
```

终端报告权限错误时，不能据此认为没有备份，可能是未获得“完全磁盘访问权限”或其他访问限制。Apple 的[备份管理说明](https://support.apple.com/108809)也列出了这个目录，但目录视图只适合定位，不能用来判断该删哪一份。

## 按设备名处理，比按文件夹可靠

连接 iPhone 或 iPad，在 Finder 侧边栏选中设备，打开“通用”，再选择“管理备份”。列表会显示设备名称与日期，按住 Control 点击备份，可以删除、在 Finder 中显示或归档，锁形图标表示备份已经加密。

安装 iOS 测试版、迁移设备或大幅修改应用数据前，可以先归档备份，归档会保留固定恢复点，不再被下一次备份更新。仍在使用的设备至少保留一份近期且已验证可用的备份，另外留下确实需要的归档，备份较旧，并不自动代表可以删除。

## iCloud 与本地备份并不自动重复

设备启用了 iCloud 备份，也要先检查最近一次成功备份的日期。本地和 iCloud 还可能覆盖不同数据，其中一种存在，不代表另一种多余。

加密本地备份可以包含未加密备份省略的已存密码、Wi-Fi 设置、健康数据和通话记录，Apple 的[加密备份说明](https://support.apple.com/108353)列出了差别。删除最后一份加密本地备份前，确认记得密码，替代备份也包含需要的数据，重置忘记的备份密码只能用于创建新备份，无法解开旧备份。

## 为什么备份看起来像一堆无法理解的文件

设备备份不是可以直接浏览的手机文件副本，每个文件都会按原始路径和域生成哈希名称，分散在多个子目录中，再由 SQLite 数据库 `Manifest.db` 映射回真实名称。

单独删除其中某个文件会让 manifest 继续引用已经消失的内容，整套备份可能因此损坏。不要根据哈希文件名判断归属，Finder「管理备份」会显示设备名称和日期，便于按完整备份检查。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/iphone-backup-metadata.webp" width="1360" height="454" loading="lazy" alt="备份元数据包括文件路径索引和设备信息，Finder“管理备份”按设备名称和日期列出完整备份">
  <figcaption>备份由哈希文件和 Manifest.db 组成。Finder“管理备份”能识别并删除完整备份。</figcaption>
</figure>

## 先用磁盘地图确认备份占用

磁盘地图可以发现 `MobileSync/Backup` 很大，却无法判断哪个恢复点可以舍弃。[Mole](https://mole.fit/zh/) 的“分析”视图适合查找占用，Finder“管理备份”适合识别、归档和删除。

## 删除备份时按这个顺序

可以先测量备份目录，在 Finder 中识别每套备份，再核对最新 iCloud 或其他本地恢复点。迁移和测试版仍需要的备份值得保留，过期内容则通过「管理备份」按整套处理，避免留下 manifest 与文件不一致的状态。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-delete-iphone-backups-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
