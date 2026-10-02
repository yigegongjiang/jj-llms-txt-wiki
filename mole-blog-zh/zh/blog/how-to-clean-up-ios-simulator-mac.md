# 如何清理 Mac 上的旧 iOS 模拟器运行时

> 分清模拟器设备和运行时镜像，再通过 Xcode 或 simctl 清理，同时保留可能还要用的归档与符号。

Published: 2026-09-01 | Updated: 2026-09-30

Xcode 的存储占用分成运行时、模拟器设备、DerivedData、DeviceSupport 和 Archives。它们在 Finder 里看起来相近，但由不同组件管理，删除后的恢复成本也不一样。

先测量，再删除。一个运行时镜像可能占几 GB，每台模拟器设备还会积累 App、缓存和测试数据。删错对象除了要重新下载，还可能丢掉复现问题需要的测试状态。

## 先做一张 Xcode 存储清单

Apple 建议在 [Xcode 的 Components 设置](https://developer.apple.com/documentation/xcode/downloading-and-installing-additional-xcode-components)中管理额外平台组件。确认不可用设备后再运行：

```bash
xcrun simctl list runtimes
xcrun simctl list devices
```

| 类型 | 删除后发生什么 | 何时保留 |
|---|---|---|
| 运行时 | 需要重新下载，依赖它的设备会不可用 | 测试矩阵、复现问题或近期项目仍需要 |
| 模拟器设备 | 该设备的 App、钥匙串、数据库和测试状态消失 | 还要复现某个问题 |
| DerivedData | 项目重新编译和索引 | 当前构建正在使用 |
| DeviceSupport | 可能失去对应真机系统版本的离线调试资料 | 仍连接该设备或处理相关崩溃 |
| Archives | 可能失去已发布构建匹配的 dSYM | 用户仍可能运行该版本 |

## 运行时镜像和模拟器设备不是一回事

运行时是下载到本机的 iOS、watchOS、tvOS 或 visionOS 系统镜像；模拟器设备则是使用某个运行时启动的虚拟 iPhone、iPad 或其他设备。多个模拟器设备可以共用一个运行时。

## 通过 Xcode 或命令行删除对应对象

前面的两条只读命令用于核对已安装的运行时和设备，不会删除任何内容。



要查看并移除已经下载的运行时，请用 **Xcode > Settings > Components**。Xcode 会显示平台版本和可回收大小，也能正确更新 CoreSimulator 的记录。不要直接删除 `/Library/Developer/CoreSimulator` 或 `/System/Library/AssetsV2` 里的运行时文件夹。

### 当前平台的最新运行时
开发中的平台至少保留最新版本，避免破坏当前测试入口。

### 仍要复现问题的旧版本
测试计划、CI 或用户问题仍指向某个版本时，它的价值不只是下载时间。

## 单独移除不可用设备

检查列表后，这条命令只删除运行时已经不可用的设备记录，不会卸载运行时。

```bash
xcrun simctl delete unavailable
```

## 只想清掉测试状态时，重置具体设备

某台模拟器里的 App 数据已经混乱时，启动这台设备，在 Simulator 中选择 **Device > Erase All Content and Settings**。这会重置当前虚拟设备，不会删除共用的运行时。它相当于抹掉一台测试手机，已安装 App、钥匙串、数据库和其他设备状态都会消失。

如果只想删除某一台虚拟设备，可在 **Window > Devices and Simulators** 中按名称检查，不必一次删除所有不可用设备。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/xcode-storage-map.webp" width="1360" height="454" loading="lazy" alt="Xcode 开发者目录分为可重建缓存，包括 DerivedData 和模块缓存，以及捕获产物，包括 DeviceSupport、Archives 与模拟器。">
  <figcaption>Xcode 存储同时包含可重建产物和捕获数据，设备、运行时、DeviceSupport 与 Archives 不能当成同一种缓存。</figcaption>
</figure>

## DerivedData、DeviceSupport 和 Archives 价值不同

DerivedData 是可以重新生成的构建产物和索引。先在 **Xcode > Settings > Locations** 中找到不再使用的项目目录；清空全部内容会让所有项目重新构建和索引。

DeviceSupport 保存实体设备和系统版本的符号与支持数据，旧目录仍可能用于调试。Archives 更需要谨慎：已经分发的构建依赖归档内匹配的 dSYM 来解析后续崩溃。请在 **Window > Organizer** 中检查，仍有用户运行的版本都应保留对应归档。

Mole 的清理确认页可以列出被新版本取代的模拟器运行时，并通过 Apple 的 `simctl runtime delete` 删除你选中的项目。分析页则能画出 DerivedData、CoreSimulator 和 Archives 的实际大小，帮你判断究竟哪一类占空间。设备、组件和发布归档仍应回到 Xcode 管理。

更完整的开发者存储流程，包括归档与 dSYM 的边界，可看[如何安全清理 Xcode 文件](https://mole.fit/zh/blog/how-to-clean-up-xcode-mac)。

## DeviceSupport 按真机和崩溃需求筛选

目录日期旧不代表无用，仍在连接的设备和待解析的崩溃决定是否保留。

## Archives 按仍在运行的版本保留

用户还能运行的版本应保留匹配归档，除非 dSYM 和构建已在其他位置验证。

## 一次可重复的安全清理

先退出 Xcode、Simulator 和相关构建，再分别测量这五类空间；从 Xcode 设置移除确认不再需要的旧运行时，然后检查设备列表并执行 `xcrun simctl delete unavailable`。DerivedData 只清旧项目，DeviceSupport 按真机与崩溃需求筛选，Archives 则要在 Organizer 里按仍在分发的版本保留。清完重新测量，空间够用就停止。

## Mole 的边界

Mole 只移除 Apple 标记为可删的旧运行时，不会清空设备状态或发布归档。

## 常见问题

### 删除模拟器会删掉项目代码吗？
不会，项目源代码不在模拟器中。但虚拟设备里的 App 和测试状态会被删除，需要复现问题的数据要先保留。

### 可以直接删除整个 DeviceSupport 吗？
不要把它当成可以整包清空的缓存。旧系统版本的目录也可能用于实体设备调试或崩溃解析，应逐项检查。

### Mole 能找到旧的模拟器运行时吗？
可以。Mole 会列出被取代的已下载运行时，并通过 `simctl` 删除选中项；其他 CoreSimulator 文件夹它不会手动删，唯一会清的是可以重新生成的系统缓存 `/Library/Developer/CoreSimulator/Caches`，而且只在没有模拟器运行时才清，它也不会替你判断仍在使用的测试运行时。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clean-up-ios-simulator-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
