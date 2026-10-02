# 卸掉 Chrome，别把密码一起清掉

> Chrome 个人资料不只是缓存。确认哪些书签、密码与其他数据已同步，未同步或仅保存在本机的内容，删除前先导出或备份。

Published: 2026-07-26 | Updated: 2026-09-23

Google 给 Mac 版 Chrome 的卸载说明分两步，第二步标成可选：退出 Chrome，从应用程序拖进废纸篓，然后可以选择是否删除个人资料信息。

「可选」这两个字分量很重，它到底可不可选，完全取决于 Google 那页说明不知道的事：你有没有登录。

开始删除支持文件之前，先对照 [Google Chrome 的现行官方卸载说明](https://support.google.com/chrome/answer/95319?hl=en&co=GENIE.Platform%3DDesktop)。如果厂商提供自己的卸载器或应用内命令，应先走那条路径，再处理明确属于它的残留。

## 已登录和未登录，是两种卸载

登录 Chrome 并不等于所有数据都已同步，先检查同步的类别和完成状态。只有已经存入 Google 账户的内容才能重新下载，扩展本地数据、离线内容和未同步的改动仍可能只在这台 Mac 上。

若没登录，或同步关着，或用了从未绑到账户的独立个人资料，本地那份就是唯一副本，写进这份资料里的密码别处都没有。

Google 也写了反过来那条：退出登录并删掉本地，不会清掉 Google 服务器上已有的内容，本地删除和账户删除是两件事，做一件不等于做了另一件。

## 配置放在哪里

Google 给出的位置是：

```
~/Library/Application Support/Google/Chrome
```

里面每个个人资料各自一个目录：第一个叫 `Default`，之后是 `Profile 1`、`Profile 2`，依此类推，Chrome 个人资料切换器里显示的名字不会出现在这里，所以从外面看哪一夹对应哪个人并不明显。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/chrome-profile-copies.webp" width="1360" height="454" loading="lazy" alt="Chrome 只有已启用同步并完成同步的数据，才可能在 Google 账户中找回；Application Support 内仍可能保留唯一的本地数据。">
  <figcaption>个人资料不是缓存。删除前既要检查同步范围，也要备份仅存在本机的数据，不能只看是否登录。</figcaption>
</figure>

若多人共用同一个 macOS 账户，在核对清楚之前，把每一个带编号的个人资料都当成某个人的数据。

## 先导出只存在这里的内容

动手删之前再打开一次 Chrome，把没有服务端副本的内容导出来，密码和书签都能从 Chrome 自身设置导出，得到的是不装 Chrome 也能打开的文件。

这一步要趁应用还在跑时做，应用卸掉之后，没有官方支持的方式再读这份个人资料。

若是换浏览器，先保留 Chrome 和个人资料，按新浏览器的导入流程操作。支持哪些数据、是否需要退出 Chrome，各家不同；确认导入完整后再卸载。

## Chrome 会留下后台更新服务

Chrome 会安装 Google 的更新服务，它由多个 Google 应用共用，不会跟着浏览器一起走，若机器上只剩 Chrome 这一个 Google 应用、希望一并清掉，这项服务要单独卸，若还在用 Google Drive 或 Google Earth，留着是对的。

所以有时 Chrome 已经卸了，Mac 上仍出现 Google 相关登录项或后台进程，那不是卸载失败。

## 个人资料怎么处理定了之后，这些可以清

缓存、代码缓存、GPU 缓存、崩溃报告和媒体缓存又大又可丢，用了好几年的浏览器上它们往往体积最大，所以卸掉 Chrome 腾出的空间常比预期多，只清缓存而不卸载本身也是合理选项。

## 分清哪个个人资料是哪个

尴尬在于装着一年已存密码的文件夹，和随手建的测试个人资料在 Finder 里看起来一样，名字都像 `Profile 3`。

[Mole](https://mole.fit/zh/) 可以列出识别到的 Chrome 相关文件和体积，帮你定位大目录，但大小不能证明某份个人资料已不用。先在 Chrome 里确认账号、书签和本地数据，再决定删除；卸载所选文件会移到废纸篓，清空前留出核对时间。

## 导出密码和书签的确切位置

两样都在 Chrome 自己的设置里，卸载之后就没有受支持的读取方式了，所以要趁它还能打开的时候做。

密码在设置的密码管理器里，导出前会要求验证一次 Mac 登录密码。**导出的是明文 CSV**，任何能打开这台电脑的人都能读，所以导完尽快导入到新的密码管理器，然后把那个文件彻底删掉，不要留在下载文件夹里。

书签在书签管理器右上角的菜单里选导出，得到一个 HTML 文件，所有主流浏览器都认。

如果只是换浏览器，可以先尝试新浏览器的直接导入功能，按它的说明决定是否退出 Chrome；缺少的类型再单独导出，核对密码和书签都已到位后再删旧资料。

Mole 核对这个应用时会列出什么、又会留下什么，写在 [卸载 Chrome，不动其他 Google 应用](https://mole.fit/zh/tested-apps/chrome)。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-chrome-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
