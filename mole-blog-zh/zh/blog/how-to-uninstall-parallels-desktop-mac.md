# 卸载 Parallels Desktop，不删除虚拟机

> 移除 Parallels Desktop 不会删除虚拟机。虚拟机是否保留、归档或移到废纸篓，应当单独决定。

Published: 2026-08-29 | Updated: 2026-09-23

从 Mac 卸载 Parallels Desktop，不会把它运行过的虚拟机一起删掉。这正是
它应有的行为。应用、PVM 虚拟机文件，以及里面的 Windows 文件本来就是
三件事，所以可以先移除应用，再单独决定虚拟机要保留、归档还是删除。

重点不在 Applications 里还有没有 Parallels，而在于虚拟机里是否还有需要
保留的东西。把这两步分开，就不容易误删。

## 应用和虚拟机是两件事

[Parallels 的官方说明](https://kb.parallels.com/en/124255)明确写着，移除
Parallels Desktop 不会删除 PVM 格式的虚拟机文件。Windows、其中安装
的应用和保存在里面的文件，都不会因为 Mac 上的应用被卸载而受影响。

PVM 文件不是缓存，而是一台虚拟电脑本身。删掉 Parallels Desktop 只能释放应用
占用的空间，并不会替人决定那个往往更大的虚拟磁盘该怎么办。

## 正常卸载其实很短

先关闭每一台虚拟机，再退出 Parallels Desktop。打开“应用程序”，把
Parallels Desktop 移到废纸篓，确认后再清空废纸篓。这就是厂商给出的完整
应用卸载步骤。

应用移走后仍能看到 PVM 文件，并不代表卸载失败。它留下来正是为了避免应用卸载
顺手带走用户数据。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/parallels-app-and-vm-boundary.webp" width="1360" height="454" loading="lazy" alt="从“应用程序”移除 Parallels Desktop 后，独立的 PVM 虚拟机及其中的 Windows 文件仍会保留。删除虚拟机是另一个决定。">
  <figcaption>这是两件独立的事。移除应用不会动虚拟机，删除虚拟机才会回收它占用的磁盘空间。</figcaption>
</figure>

## 回收虚拟机空间是另一项决定

如果目标是腾出虚拟机占用的空间，应当看
[Parallels 的虚拟机删除说明](https://kb.parallels.com/en/5029)，而不是把应用
卸载当成全部操作。在控制中心里，Parallels 给出了两种含义完全不同的选择：
“保留文件”会把虚拟机从列表中移走，但文件仍留在原处；“移到废纸篓”则用于
不再需要其中任何数据的虚拟机。

Parallels 把“移到废纸篓”标为不可逆操作。它是一次数据处理决定，不是卸载应用
的最后一步。应用以后还能重新安装，但虚拟机里可能放着唯一一份 Windows 工作
环境、项目文件或软件配置。

## 移到废纸篓前先检查虚拟机

在删除前打开一次虚拟机，或在 Finder 中确认内容。重要文件应当已经复制到虚拟
磁盘以外的位置，依赖的备份也应当确认完成。如果只是暂时不用，选“保留文件”
更合适，它会让虚拟机离开 Parallels 的列表，却不会擦掉内容。

只有在里面的内容确实不再需要时，再选“移到废纸篓”。清空废纸篓才会让这件事
真正无法回头，没有必要为了让卸载看起来更彻底而急着做完。

## 先找全虚拟机，再计算释放的空间

虚拟机常见于个人目录下的 Parallels 文件夹，也可能被移到其他目录或外接硬盘。不要看到控制中心里只有一台，就认为 Mac 上也只有一个虚拟机。在 Finder 里搜索 `.pvm`，并检查曾经用过的外接磁盘。一个虚拟机包里包含虚拟磁盘、配置、快照和暂停状态，实际占用通常比表面看起来大。

备份或移动前，先真正关闭虚拟机，不要只让它保持暂停。复制时要保留完整的 `.pvm` 包，不要进入包内挑文件。一个合格的归档应该能够找到、辨认，并能在兼容的 Parallels 版本中重新打开。如果里面有加密内容、授权软件或工作账号，也要把以后恢复时需要的信息一并记好。

最后把两件事分开验证：Parallels Desktop 应该已经从“应用程序”里移除，也不再登录时启动；选择保留的虚拟机仍在记录的位置。只有删除虚拟机或快照才会明显腾出大块空间，所以单纯卸载应用后空间只增加一点很正常，并不表示 macOS 没有卸载干净。

Mole 核对这个应用时会列出什么、又会留下什么，写在 [卸载 Parallels Desktop，不删虚拟机](https://mole.fit/zh/tested-apps/parallels)。

## 常见问题

### 卸载 Parallels Desktop 会删除 Windows 吗？

不会。Parallels 说明，卸载 Mac 上的应用不会删除 PVM 虚拟机、其中的 Windows
文件或已经安装的应用。

### 能否从控制中心移除虚拟机，但保留文件？

可以。“保留文件”会把虚拟机从 Parallels 的列表中移除，文件仍在原来的位置，
以后还可以再决定怎么处理。

### 为什么 Parallels 卸载后，虚拟机还在 Mac 上？

因为应用卸载本来只处理应用。虚拟机是独立的用户数据，会一直保留到明确选择
保留、归档或删除它为止。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-parallels-desktop-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
