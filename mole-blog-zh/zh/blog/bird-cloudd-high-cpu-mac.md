# Mac 上 bird、cloudd CPU 占用高怎么办

> 先在访达和 iCloud.com 核对同一份文件，再排查 bird、cloudd 的 CPU 占用，保留尚未上传的本机版本和独立备份。

Published: 2026-09-28

活动监视器里 `bird` 或 `cloudd` 占着 CPU，常常正好遇上 iCloud 云盘传文件，但只看进程名和百分比，分不清它是在上传、下载，还是卡在某个文件上。先到访达看那份文件的状态，再决定是否需要处理同步；想了解其他陌生进程怎么查，可以看[认识 Mac 上的进程](https://mole.fit/zh/blog/mac-processes-explained)。

这套检查针对 iCloud 云盘文件；macOS 的 `cloudd` 手册把它称为 CloudKit 服务，[Apple 也说明](https://developer.apple.com/documentation/cloudkit)，其他 App 可以用 CloudKit 同步数据。如果访达里的云盘文件没有异常，就对照哪些 App 的 iCloud 活动恰好出现，别把 `cloudd` 的 CPU 占用都算到云盘头上。

## 先看文件有没有往前走

在访达打开 iCloud 云盘，切成列表视图，进**显示 › 查看显示选项**，打开 **iCloud 状态**。Apple 的[状态说明](https://support.apple.com/guide/mac-help/mchlc994344b/mac)把传输中、等待上传、空间不足、不符合条件，以及只在 iCloud 上而尚未下载到 Mac 的文件分开列了出来。鼠标停在侧边栏的 iCloud 云盘上，也能点状态图标看整体传输；隔一会儿再看同一个文件和侧边栏，CPU 还在忙不等于文件已经传完。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/icloud-sync-check.webp" width="1360" height="454" loading="lazy" alt="示意图：依次核对访达中的 iCloud 云盘文件、iCloud.com 上的版本，以及单独保存的备份">
  <figcaption>用同一份文件核对本机和云端，再留一份独立备份；两端一致说明同步有进展，不代表已经有备份。</figcaption>
</figure>

用同一个 Apple 账户打开 [iCloud.com 云盘](https://www.icloud.com/iclouddrive/)，看文件在不在、最新修改有没有过去。如果访达写着「等待上传」，网页上还是旧版，先留住本机这份再动设置；如果文件只「在 iCloud 中」，离线使用前先在访达点「现在下载」，开了优化 Mac 储存空间时还可以选「保留已下载」，让这份文件留在本机，操作见 [Apple 的云盘指南](https://support.apple.com/guide/mac-help/mchl1a02d711/mac)。

## 按状态排查

| 看到的状态 | 接下来查什么 |
| --- | --- |
| 传输图标在变化，文件也出现在 iCloud.com | 保持网络连接，过一会儿再查同一文件；这时 CPU 占用可能只是正常传输。 |
| 等待上传 | 查网络和 iCloud 剩余空间；[Apple 说明](https://support.apple.com/guide/mac-help/mchle5a61431/mac)，云端空间不够时文件会先留在 Mac 上。 |
| 空间不足或不符合条件 | 先读访达给出的状态，不要因为 `bird` 正忙就认为这份文件已上传。 |
| iCloud 云盘顶部提示权限错误 | 按 [Apple 的权限指南](https://support.apple.com/guide/mac-help/mchlp1203/mac)，只针对这条提示点访达里的「修复」。 |
| 看不到进展，iCloud.com 上同一文件也没变 | 核对**系统设置 › Apple 账户 › iCloud › 云盘 › 同步此 Mac**，两端是否用了同一账户，再查网络和系统更新；设置路径见 [Apple 指南](https://support.apple.com/118443)。 |

如果文件状态一直不动，再到活动监视器看 CPU 和网络活动随时间怎么变，也可以对进程取样，看看线程正做什么；持续高 CPU 值得查，但不能单凭这个数字断定同步数据库损坏，CPU 安静下来也不能证明指定文件到了云端。

## 修复前先留一份能找回的副本

iCloud 云盘会把改动和删除同步到其他设备，[Apple 的桌面与文稿指南](https://support.apple.com/109344)也明确写了删除会跨设备发生，所以同步不是独立备份。重要文件先确认完整下载，再复制到 iCloud 云盘以外的存储位置，或者确认现有备份真的能取回；不要一上来删 `~/Library/Application Support/CloudDocs`、退出账户，或反复强制退出 `bird` 和 `cloudd`，这些动作都找不出卡住的文件，而等待上传的本机版本可能是目前唯一的新版本。

如果状态、账户、空间、权限和网络都查过，同一份文件还是不动，记下访达里的状态、iCloud.com 有没有最新版本，再带着这两项找 Apple 支持，问题会比「进程占 CPU」具体得多。

## 常见问题

### `bird` 或 `cloudd` 占 CPU 就是出故障了吗？

不是。首次同步或一次改了很多文件，都可能让它们忙一阵；要看传输状态和那份文件是否真的往前走。

### 删掉 CloudDocs 文件夹能让同步重新开始吗？

别把它当默认修复办法，先定位哪份文件没传上去，并给还在本机的最新版本留一份独立副本。

---

Canonical HTML page: https://mole.fit/zh/blog/bird-cloudd-high-cpu-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
