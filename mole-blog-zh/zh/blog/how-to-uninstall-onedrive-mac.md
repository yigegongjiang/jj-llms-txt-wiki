# 卸载 Mac 上的 OneDrive，不删除文件

> 卸载 OneDrive 会停止本地同步，但云端文件仍在。释放空间和删除仅在线文件，对云端数据的影响不同。

Published: 2026-08-28 | Updated: 2026-09-23

在 Mac 上卸载 OneDrive，和删除 OneDrive 文件不是一回事。Microsoft 说明，
卸载应用会让本地 OneDrive 文件夹停止同步，而 OneDrive 中的文件和数据仍可在
OneDrive.com 登录后访问。

真正容易出错的是把同步位置里的内容当作缓存直接删除。尤其是“仅在线”文件，
在 Finder 里看得到，却可能只在云端有实际内容。需要移除的是应用，就单独移除
应用，文件该怎么处理则另作决定。

## 可以移除应用，不会删除云端副本

[Microsoft 的 OneDrive 说明](https://support.microsoft.com/en-us/onedrive/reinstall-onedrive)
写得很明确，卸载 OneDrive 不会丢失数据。本地 OneDrive 文件夹会停止同步，
文件仍可通过 OneDrive.com 访问。在 Mac 上，移除应用这一步本身就是把 OneDrive
应用移到废纸篓。

这并不代表每个文件都成了本地备份，而是说明应用移除后，云端账户仍然保存着
已同步的数据。

## 移除应用前先确认云端副本

卸载前打开 OneDrive.com，看一遍真正重要的文件。刚修改的文档、大文件上传，
以及离线时完成的工作，都值得确认一次。等网页上出现预期的版本，再移除应用。

这不是多余的步骤。同步停止后，尚未上传的本地改动更难判断。先在浏览器里确认，
可以清楚分开已经进入 OneDrive 的文件和仍需处理的工作。

## “释放空间”不等于“删除”

[Microsoft 的按需文件说明](https://support.microsoft.com/en-us/onedrive/save-disk-space-with-onedrive-files-on-demand-for-mac)
区分得很清楚。选择“释放空间”会把本地可用的文件改回“仅在线”，它不再占用
本机储存空间，但仍保留在 OneDrive，联网后可以再次打开。

删除仅在线文件则是另一回事。Microsoft 说明，从设备删除这类文件，会同时从
OneDrive、其他设备和网页端删除。因此，云朵图标并不表示这些文件可以像临时
下载一样随手清掉。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/onedrive-cloud-file-boundary.webp" width="1360" height="454" loading="lazy" alt="卸载 OneDrive 应用会保留 OneDrive 中的云端文件；“释放空间”会把本地文件改为仅在线；删除仅在线文件会删除云端副本。">
  <figcaption>这是三种不同的操作。卸载移除应用，“释放空间”回收本地储存，删除则可能直接从 OneDrive 移除文件。</figcaption>
</figure>

## 数据确认后再移除应用

重要文件已经在网页端确认无误后，再退出 OneDrive 并把应用移到废纸篓。以后若
重新安装，Microsoft 提醒，之前选过的同步文件夹可能要在同步完成后重新选一次。

不需要为了证明卸载成功，就把 OneDrive 文件夹删除。先单独决定本地副本是否还
有用，也不要在 Finder 中对同步项目使用“删除”，除非本来就想从 OneDrive 中
一并删除它。

## 断开账号、卸载应用和删除文件是三件事

同一台 Mac 上可能同时有个人 OneDrive、工作或学校账号，以及通过 OneDrive 同步的 SharePoint 资料库。操作前逐个确认菜单栏里的云朵图标和账号名称，一个账号同步完成，不代表其他账号也已经完成。在 OneDrive 设置里断开账号，只会停止这个账号的同步；移除应用会停止客户端本身。这两个动作都不是删除云端文件的指令。

如果只是想释放本地空间，用“释放空间”通常更合适。已经确认在云端、以后可以重新下载的内容可以变成仅联机状态；必须离线使用的文件则继续保留在本机。工作账号还要看清共享资料库或快捷方式属于谁，取消本机同步和删除团队共享内容完全不是一回事。

操作后登录网页版，实际打开几份最近的文件，不要只看文件名是否存在。再确认 Mac 上的 OneDrive 已经不再运行或同步。在这些检查完成前先保留本地文件夹。如果某个文件只在本机，或者本机修改时间比网页版更新，先把它复制到 OneDrive 目录之外，打开副本确认内容无误，再处理剩余内容。

Mole 核对这个应用时会列出什么、又会留下什么，写在 [卸载 OneDrive，保留云端文件](https://mole.fit/zh/tested-apps/onedrive)。

## 常见问题

### 卸载 OneDrive 会删除文件吗？

不会。Microsoft 说明，卸载会让本地文件夹停止同步，文件仍可通过 OneDrive.com
访问。

### 需要腾出磁盘空间时，“释放空间”安全吗？

它会把本地可用文件改为仅在线，不会从 OneDrive 删除。下次打开文件时需要联网。

### 卸载 OneDrive 后，可以删除仅在线文件吗？

只有在确实想从 OneDrive 一并删除时才可以。Microsoft 说明，删除仅在线文件会
从 OneDrive、所有设备和网页端移除它。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-onedrive-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
