# 卸载 Mac 上的 Office，别把 Outlook 邮件一起删了

> 先分清各应用的独立容器和存放 Outlook 配置的三份共享文件夹，备份后再按 Microsoft 的步骤卸载

Published: 2026-08-03 | Updated: 2026-09-23

Microsoft 提供了完整的 Office for Mac 卸载步骤，而且确实有效，中间有一句最值得停下来看：它让你删的文件夹里，有三个装的是 Outlook 邮件。

Microsoft 自己写得很清楚，多数照抄文件夹清单的教程却没提这一点，而清单又足够长，等到删到那三个时，人已经在机械操作了。

开始删除支持文件之前，先对照 [Microsoft Office 的现行官方卸载说明](https://support.microsoft.com/en-us/office/uninstall-office-for-mac-eefa1199-5b58-43af-8a3d-b73dc1a8cae3)。如果厂商提供自己的卸载器或应用内命令，应先走那条路径，再处理明确属于它的残留。

## 那三个不是残留的文件夹

Office 的状态数据分两处放在 Library 下，风险出在第二处。

`~/Library/Containers` 里每个应用各有一个文件夹：Microsoft Word、Microsoft Excel、Microsoft PowerPoint、Microsoft OneNote、Microsoft Outlook，以及 Microsoft Error Reporting、com.microsoft.Office365ServiceV2 这类服务文件夹。

`~/Library/Group Containers` 里有三个共享文件夹：UBF8T346G9.ms、UBF8T346G9.Office 和 UBF8T346G9.OfficeOsfWebHost。

Microsoft 对这第二组的说明里自带警告：把这三个文件夹移到废纸篓会一并清掉 Outlook 数据，删除前应先备份。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/office-outlook-data.webp" width="1360" height="454" loading="lazy" alt="各应用独立容器文件夹装的是缓存和应用状态；三个共享的 Group Containers 文件夹装的是 Outlook 配置文件和本地邮件，服务器上没有副本可以替代。">
  <figcaption>各应用的独立文件夹是应用状态。三个共享文件夹才是 Outlook 保存配置和本地邮件的地方，所以 Microsoft 要求先备份。</figcaption>
</figure>

IMAP 或 Exchange 账户中已同步且仍保留在服务器上的邮件，重装后通常能重新下载，但「本机」文件夹、从未上传的归档邮件或导入的 .olm，可能只有本地副本，不能靠账户类型判断是否有备份。

## 应用还在时先拷出去

在访达里按 Command Shift G，进入 `~/Library/Group Containers`，把三个 UBF8T346G9 文件夹复制到 Library 以外的地方，例如外置磁盘或桌面上的文件夹，等复制完成并核对内容后再继续。

在 Office 仍已安装且已退出时做这件事，事后再拷，拷的只是删剩的东西，不是同一回事。

## 再按 Microsoft 的顺序操作

先退出所有 Office 应用，把应用从「应用程序」移到废纸篓，再删 Containers 文件夹，再删三个 Group Containers 文件夹，Microsoft 最后还要求从程序坞去掉图标并重启。

Office 的后台服务和错误报告进程可能仍在使用文件，因此先退出相关程序，按官方步骤完成卸载并重启，再检查是否还有残留，不能只看应用图标是否消失。

## 注销许可和卸载是两回事

删掉应用并不会从 Microsoft 账户释放许可证，也不会取消订阅，若卸载是为了把 Office 迁到另一台 Mac，还要在账户侧停用，若原因是账单，到账户里取消订阅，文件没了，这两件事都不会自动发生。

## 之后哪些可以放心清

应用和容器处理完后，再检查偏好设置文件、缓存、已保存的应用状态和安装回执，确认归属和内容后再删，不要把所有以 Microsoft 开头的文件都当成垃圾。比起急着清完残留，前面的邮件备份更值得花时间。

字体也值得核对：Office 提供的字体可能被现有文档使用，删除包含字体的应用或支持文件前，先确认这些文档所需的字体是否仍可用。

## 清单够长，读完才有意义

卸载不难，难在烦琐，烦琐会让人在九个都以 Microsoft 开头的文件夹清单中途停读，那三个关键的和另外六个看起来一模一样。

正因为如此，需要把候选项放在一起看，[Mole](https://mole.fit/zh/) 会列出路径和体积，40 MB 的错误报告容器和 12 GB 的邮件库大小不同，但大小不能代替内容检查。普通文件移入废纸篓后，只要还未清空就可取回，邮件库仍应提前单独备份。

## 怎么判断邮件在不在服务器上

账户类型和 Outlook 的设置都要检查，最终以服务器上实际保留的邮件为准。

IMAP 和 Exchange 中已同步且仍在服务器上的邮件，重装登录后通常能重新下载，未同步内容仍要备份。POP 是否保留服务器副本取决于设置，[Microsoft 的同步说明](https://support.microsoft.com/en-us/outlook/sync-basics-what-you-can-and-cannot-sync)也列出了保留副本和定期删除等选项，不能只看账户类型就下结论。「我的电脑」文件夹和导入的历史邮件可能只有本机副本，应先导出。

在 Outlook 的账户设置里查看账户类型，再核对侧边栏中的本地文件夹。使用支持 .olm 导出的 Outlook 版本时，应手动导出这些内容；如果找不到导出选项，先保留原数据，按 Microsoft 对当前版本的说明处理。导出文件放到 Library 以外的地方，并确认能读取，别放在接下来要删的目录里。

Mole 核对这个应用时会列出什么、又会留下什么，写在 [卸载 Office，别顺手清掉 Outlook 邮件](https://mole.fit/zh/tested-apps/microsoft-office)。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-microsoft-office-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
