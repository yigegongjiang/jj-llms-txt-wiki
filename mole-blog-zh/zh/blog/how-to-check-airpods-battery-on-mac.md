# 在 Mac 上查看 AirPods 和配件电量

> AirPods 在控制中心的“声音”里查看，Magic 配件在 Bluetooth 里查看；同时说明 pmset 能报告什么，以及设备不公开电量时为什么无法补出数字。

Published: 2026-08-13 | Updated: 2026-09-05

在 iPhone 上，电量看一眼小组件就有。到了 Mac，想知道 AirPods、Magic Mouse 或 Magic Keyboard 是不是快没电，得先找对系统入口。macOS 没有一个列出所有配件的统一电池面板。

AirPods 要先打开充电盒或取出耳机，放在 Mac 附近，再到 **控制中心 > 声音** 里选中 AirPods。Magic Keyboard、Magic Mouse 和 Magic Trackpad 的电量在 **控制中心 > Bluetooth**。命令行里的 `pmset -g accps` 只会列出 macOS 电源服务公开的配件。

## 两个系统入口

Magic Keyboard、Magic Mouse 和 Magic Trackpad 按照 [Apple 的说明](https://support.apple.com/102292)，在控制中心点 Bluetooth 就能看到电量。菜单栏的 Bluetooth 项目显示的是同一类信息。Magic Mouse 充电时不能使用，键盘和触控板可以。

AirPods 走另一条路径。Apple 建议[打开充电盒或取出耳机，把它们放在 Mac 附近，然后进入控制中心的“声音”](https://support.apple.com/119912)。选中 AirPods 后，macOS 会显示当前拿到的耳机和充电盒电量，不要求正在播放音频。

Apple 现在的 Mac 指南没有通知中心里的 iPhone 式“电池”小组件。让你去添加这个小组件的说明，通常是把 iPhone 或 iPad 的步骤搬到了 macOS。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/accessory-battery-reporting-path.webp" width="1360" height="454" loading="lazy" alt="打开并靠近 Mac 的 AirPods 通过控制中心的声音面板上报电量，Magic 输入设备通过 Bluetooth 上报，命令行只能看到 macOS 电源服务公开的设备。">
  <figcaption>不同配件走不同入口，Mac 上没有一块完整的配件电量总表。</figcaption>
</figure>

## 从命令行查看

`pmset` 是电源管理工具，它的配件子命令会列出会上报电量的设备：

```
pmset -g accps
```

什么配件都没连的时候，输出只覆盖内置电池：

```
Now drawing from 'AC Power'
 -InternalBattery-0 (id=22282339)	79%; AC attached; not charging present: true
```

这份列表显示的是 macOS 电源服务公开的配件电量，适合脚本读取，但不一定覆盖菜单里出现的所有设备。这里没有某个配件时，还要去对应的控制中心入口核对。

要先确认到底连上了什么：

```
system_profiler SPBluetoothDataType
```

它会先打出控制器，然后是 **Connected** 和 **Not Connected** 两段，每台设备带着地址、厂商、产品 ID、固件版本和次要类型（Headphones、Keyboard、Mouse），电量相关的键只出现在已连接设备下面，所以先看这一段，再判断一台设备是不是根本不上报电量。

## 为什么设备不显示电量

不是每台 Bluetooth 配件都会上报电量，下面这些情况都不是故障：

| 情况 | 实际在发生什么 |
|---|---|
| 设备已配对但未连接 | macOS 没有可读取电量的活动会话 |
| 第三方耳机 | 通过 Bluetooth 上报电量是可选能力，很多品牌根本没做 |
| AirPods 在充电盒里 | 打开盒盖并靠近 Mac，再从控制中心的声音面板查看，合盖时可能没有当前读数 |
| 设备连着另一台 Apple 设备 | 只有当前连着它的那台设备才能读到电量 |
| 较老或杂牌配件 | Bluetooth 配置里根本没有电池服务 |

第三方耳机始终没有电量，是设备自己没往外发，Mac 上没有任何应用能凭空造出一个数字。

## AirPods 和充电盒

Mac 可以显示受支持 AirPods 的充电盒电量。关键条件不在设置里，而在设备状态：打开盒盖或取出耳机，让它们靠近 Mac，再到控制中心的“声音”面板查看。盒盖关闭并且离 Mac 很远时，没有持续的活动会话，信息缺失或停留在旧值都很正常。

如果充电盒仍然不出现，就看盒上的状态灯，或用 iPhone、iPad 复查。这是上报失败时的备用办法，不是 macOS 永远看不到盒子电量。

## 电量低了，系统可能不提醒

Apple 说明了 AirPods 在 iPhone 和 iPad 上的低电量通知，以及耳机自己发出的提示音，但没有提供一个通用的 Mac 配件低电量通知设置。不要把充电习惯建立在一条 macOS 可能不会发出的提醒上。

更稳妥的做法是按节奏充电，并在会议前检查一次。Magic Mouse 的充电口在底部，充电时不能使用，提前补几分钟电比彻底没电后再处理省事。

## 监视工具放在哪

[Mole](https://mole.fit/zh/) 把已连接配件的电量和 Mac 自身电池、CPU、内存、散热状态放在同一块状态视图里，答案就在菜单栏，不用点进 Bluetooth 再点三下，它报的是系统为已连接设备公开的电量，设备自己不发就不显示，不会猜一个数字。

Mac 自身电池，包括循环次数和电池状况，见
[如何查看 MacBook 电池健康](https://mole.fit/zh/blog/how-to-check-macbook-battery-health)。

## 常见问题

### 为什么 AirPods 在 Mac 上不显示电量？

打开充电盒或取出耳机，把它们放在 Mac 附近，然后到 **控制中心 > 声音** 查看。连着另一台设备或距离太远时，Mac 可能没有当前电量。

### 能在 Mac 上看到 AirPods 充电盒电量吗？

支持的 AirPods 可以。打开盒盖并放在 Mac 附近，再在 **控制中心 > 声音** 里选择 AirPods。如果上报没有出现，就看盒上状态灯，或用 iPhone、iPad 复查。

### macOS 会在配件电量低时提醒吗？

Apple 没有说明一个通用的 Mac 配件低电量通知设置。AirPods 自己会发出低电量提示音，设备正常上报时，也可以从上面的入口查看当前电量。

### 为什么第三方鼠标没有百分比？

电量上报是 Bluetooth 配置里的可选能力，很多厂商没有实现。设备自己不发这个数字，Mac 上的应用也无法凭空补出来。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-check-airpods-battery-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
