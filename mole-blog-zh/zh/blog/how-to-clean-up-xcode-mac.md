# 如何清理 Xcode，又不丢失发布归档

> 分别检查 DerivedData、模拟器、DeviceSupport 和 Archives，只删除可重建内容，保留线上版本的归档与 dSYM。

Published: 2026-07-12 | Updated: 2026-09-05

Xcode 的空间分散在构建输出、索引、归档、设备支持、模拟器设备和平台运行时中，有些可以重建，有些却是已发布版本归档与 dSYM 的唯一副本，清理前应分别测量，并尽量通过 Xcode 自己来管理。

## 先看空间花在哪里

```
du -sh ~/Library/Developer/Xcode/* ~/Library/Developer/CoreSimulator 2>/dev/null | sort -h
```

常见大项包括：

- **DerivedData：** 构建产物、索引和模块缓存，通常每个项目一个目录，可以重建，但全部清空会让之后的索引和构建变慢
- **DeviceSupport：** 从物理设备和特定系统版本收集的符号与支持数据，旧条目仍可能用于崩溃符号化
- **Archives：** 导出或发布应用时保存的构建，已分发版本的归档包含匹配的二进制文件和 dSYM，之后分析崩溃仍会用到，从未离开本机的归档更适合清理
- **模拟器设备与运行时：** 设备记录位于 CoreSimulator，运行时由 Xcode 组件管理，它们不是同一种缓存

## 分类别清理

**DerivedData** 适合按项目处理，先退出 Xcode，通过「Xcode > 设置 > 位置」打开 Derived Data，找出不用的项目目录并移到废纸篓，只有全局索引或构建故障明确需要时，才清空全部内容。

删除运行时已经不可用的模拟器设备前，先确认其中没有需要保留的测试数据，并停止模拟器和相关测试任务；下面的命令会删除设备及其数据，不经过废纸篓：

```
xcrun simctl delete unavailable
```

这只删除设备记录，不会卸载运行时镜像，通过「Xcode > 设置 > 组件」可以查看已安装平台、运行时和可回收大小，再移除能够重新下载的项目，Apple 的 [Xcode 组件说明](https://developer.apple.com/documentation/xcode/downloading-and-installing-additional-xcode-components)记录了这条受管理的路径，设备记录则在「窗口 > 设备与模拟器」中管理。

在「窗口 > Organizer」中检查归档，对已经分发、仍可能收到崩溃报告的构建，归档和 dSYM 还有实际价值，Apple 的[调试信息说明](https://developer.apple.com/documentation/xcode/building-your-app-to-include-debugging-information)也提到，二进制文件与 dSYM 只有构建 UUID 匹配时才能配合使用，DeviceSupport 同样更适合逐项检查，整目录清空会丢掉特定系统版本的符号资料。

移动 DerivedData 前先退出 Xcode，并停止命令行构建和测试，避免索引或构建数据库正在写入。下次打开项目时 Xcode 会重新索引、解析依赖并生成输出，所需时间取决于项目和本地输入。

## 哪些内容能重建

DerivedData 根据源码、设置、工具链和依赖生成，包含目标文件、模块缓存、索引和构建产物，输入仍然存在时可以重建，代价是时间和可能的网络下载。

DeviceSupport 是调试物理设备时保存的特定系统支持与符号数据，模拟器设备和运行时也是独立的受管理对象，Archives 更需要保留，因为其中的 dSYM 可能是符号化线上崩溃的唯一匹配文件。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/xcode-storage-map.webp" width="1360" height="454" loading="lazy" alt="Xcode 开发者目录分为可重建缓存，包括 DerivedData 和模块缓存，以及捕获产物，包括 DeviceSupport、Archives 与模拟器。">
  <figcaption>DerivedData 可以重建，Archives、dSYM 和部分设备支持数据可能长期有用。</figcaption>
</figure>

## 先用磁盘地图确认 Xcode 占用

[Mole](https://mole.fit/zh/) 的「分析」页可以确认空间主要来自 DerivedData、CoreSimulator 还是 Archives，实际移除仍交给 Xcode 的「组件」「设备」和 Organizer，这些入口理解运行时、设备与发布产物。

## 安全的操作顺序

停止构建后可以分别测量 Xcode 与 CoreSimulator，从不用项目的 DerivedData、不可用模拟器设备和闲置运行时开始，归档则需要与已发布版本及符号化需求逐一核对，重新打开一个重要项目并完成构建，也能在清空废纸篓前验证环境是否完整。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clean-up-xcode-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
