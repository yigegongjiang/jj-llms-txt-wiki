# 卸掉 Creative Cloud，别把还需要的文件一起清掉

> 先拷出 Adobe 点名的四项，按它要求的顺序卸载，并把 Cleaner 当成修复手段而不是深度清理

Published: 2026-08-05 | Updated: 2026-09-05

Adobe 当前的 Creative Cloud 桌面应用流程会先让你尝试“修复”。如果修复解决了启动、登录或更新问题，就不需要卸载；只有修复无效时才继续选择“卸载”。

真要卸掉时，风险不在残留文件，而在主目录里的一个文件夹：自 2025 年 2 月起，里面可能是你还需要的唯一副本，而卸载过程会把它改名成看起来完全像垃圾的样子。

先用 Adobe 的 [Creative Cloud 桌面应用卸载页](https://helpx.adobe.com/creative-cloud/help/uninstall-creative-cloud-desktop-app.html)选择正确的产品路径。只有标准卸载器失败或安装状态已经损坏时，才考虑 Adobe 官方的 Creative Cloud Cleaner 工具。

## 动手前先拷出这四样东西

Adobe 在卸载说明和 Cleaner 工具页面里点名了下面四项，从访达里看，没有一项一眼就能认出来。

**`~/Creative Cloud Files`。** 它不在 Library 里，而在主目录，和“文稿”“下载”同级。Adobe 已停用 Creative Cloud 同步文件并删除了云端副本，且写明删除后无法恢复，所以本地这个文件夹里有什么，就是你还剩什么，Adobe 建议把备份放到这个文件夹以外的地方。

它还会被改名，退出登录后文件夹不再隐藏，会变成类似 `Creative Cloud Files Personal Account you@example.com ####@AdobeID` 的名字。卸载刚结束时主目录里突然冒出这个名字，很容易当成残留垃圾，它不是。

**`~/Library/Application Support/Adobe/CoreSync/`。** Adobe 明确要求运行 Cleaner 工具前先备份这里，空间紧的话，重点看里面的 `cloudnative` 文件夹。

**插件、预设和工作区。** 第三方插件和偏好设置文件都在各 Adobe 产品目录里，会跟着产品一起走，你调了一年的 Photoshop 工作区是偏好设置文件，不是存在账户里的设置。

**旧版安装包。** Adobe 只提供最近两个版本的下载。卸掉更旧的应用却没留安装包，就没有受支持的恢复路径。

## Adobe 规定的卸载顺序

不能先卸 Creative Cloud 桌面应用，Adobe 设了门槛：Photoshop、Illustrator、Premiere 以及其余 Creative Cloud 应用都必须先卸完，桌面应用的卸载程序才会真正生效。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/adobe-uninstall-order.webp" width="1360" height="454" loading="lazy" alt="先卸载每一个 Creative Cloud 应用，再卸 Creative Cloud 桌面应用；只有前两步失败时才用 Cleaner 工具；动手前先拷出四项内容。">
  <figcaption>桌面应用要等它管理的应用都卸完才能卸；Cleaner 工具是安装损坏时的修复路径，不是正常卸载的下一步。</figcaption>
</figure>

然后运行 Creative Cloud Uninstaller，对话框里同时有“修复”和“卸载”，只有选卸载才会卸掉。在受管或企业 Mac 上，你可能两边都没权限，这要问机器管理员，不是该绕开的技术问题。

## 有一条官方指引已经失效

Adobe 卸载页仍让你在运行卸载程序前确认文件已同步，并警告未同步到账户的文件可能丢失，但这条说明早于 Adobe 自己同步文件页面里写的变更：桌面同步服务已停用，云端副本也在 2025 年 2 月被删除。

对旧的 Creative Cloud 同步文件，不能再把“已同步”当成备份确认，应把 `~/Creative Cloud Files` 拷到别处。这个停用范围不等于 Adobe 所有云文档服务都已停止，其他云文档仍需按对应产品检查同步和备份。

## Cleaner 工具是给装坏了用的，不是做彻底清理

Adobe 把 [Creative Cloud Cleaner 工具](https://helpx.adobe.com/creative-cloud/kb/cc-cleaner-tool-installation-problems.html)写成给有经验用户清理损坏安装的实用程序，并列出适用场景：修复或卸载后仍无法安装或更新、重装后桌面应用仍打不开、常规处理后仍登录失败。

列表里也有“彻底移除较旧的 Adobe 软件”，所以它常被当成例行深度清理来推荐，但它不是。Adobe 自己的准备步骤要求先备份文件和文件夹以防数据丢失，先收集日志以便需要支持时使用，先退出一组后台进程，包括 Creative Cloud Core Service、Adobe Content Synchronizer、Creative Cloud Libraries Synchronizer，以及 Creative Cloud Interprocess Service。

正常卸载已经成功的话，你不需要它。

## 卸完还剩什么，归谁管

Adobe 会在用户 Library 和系统 Library 写入支持文件、偏好设置、缓存、日志、特权助手和后台服务，能否删除要看具体内容，以及是否仍由其他 Adobe 产品使用。

搜索 “Adobe” 后整批删除，会把前面需要备份的内容也算进去。名字不能区分字体缓存和自己保存的预设，确认归属之后，还要检查内容、共享关系和备份，不能只凭匹配结果删除。

这时可以用 [Mole](https://mole.fit/zh/) 列出相关文件的位置和大小，逐项核对哪些属于仍在使用的 Adobe 产品、哪些已不需要。卸载所选文件会移到废纸篓，在清空之前保留恢复的机会，但不能代替前面的独立备份。

## 怎么确认那份副本真的到手了

复制完 `~/Creative Cloud Files` 之后，对一下两边的项目数和总体积，别只看文件夹出现了就算完。

项目数和体积只是初步检查，还要抽查重要文件能否正常打开。若内容缺失，先保留现有目录，再找本地备份、其他设备上的副本，必要时联系 Adobe 支持；旧的同步文件服务已经停用，不能再指望回 Creative Cloud 重新下载已删除的云端副本。

退出登录后主目录里那个改了名的文件夹长这样：`Creative Cloud Files Personal Account 邮箱 一串字符@AdobeID`，它看着像临时目录，实际就是原来那个，可以按自己习惯改名，但在确认副本齐全之前不要删。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-adobe-creative-cloud-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
