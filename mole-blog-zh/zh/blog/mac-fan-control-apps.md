# Macs Fan Control、TG Pro、iStat Menus 与 Mole 对比

> 比较 Macs Fan Control、TG Pro、iStat Menus 和 Mole 的传感器读取、风扇控制、特权组件、历史记录与 Apple Silicon 限制。

Published: 2026-07-23 | Updated: 2026-09-27

风扇控制软件功能不多，权限却不小，读取部分温度和转速可能无需提权，修改风扇目标通常要安装特权辅助程序，而它可能在应用删除后继续存在。

风扇只能加快散热，不能消除发热源，失控进程、堵塞风道和故障硬件仍要单独处理，所以选择工具时，它会写入什么、安装什么，以及能否完整卸载，比曲线数量更值得看。

## 提高最低转速有什么用

风扇更快，可以让芯片在长时间负载下更早散热，并延后降频，代价是噪声、风扇磨损、更多灰尘和笔记本电量。

iStat Menus 和 smcFanControl 都说明只允许提高最低转速，不会让风扇低于 Apple 要求的转速。这是这两款工具的控制方式，不能据此推定所有手动模式都相同，使用前要看各自的限制。

## 读取与写入的权限不同

Mac 通过系统管理控制器提供温度和风扇数据，可读传感器随机型和接口变化，部分工具连读取也需要辅助程序，写入通常要经过高权限进程。

常见实现包括 Service Management 管理的 launchd 守护进程、setuid 程序或 sudoers 规则，但安装方式只是第一层，辅助程序还需要验证调用者、校验请求，并把特权操作缩到最小，SMJobBless 会在安装和更新时检查签名要求，但不会自动保证 XPC 协议安全。

旧式辅助程序常位于 `/Library/PrivilegedHelperTools`，对应守护进程位于 `/Library/LaunchDaemons`，只把应用拖进废纸篓可能留下这些组件，厂商不说明安装内容与卸载方法时，也应把这种不透明计入选择。

Apple 芯片使用不同的 SMC 键，传感器名称和数量也随机型变化，所以工具需要持续更新机型表，无风扇 MacBook Air 没有可控制对象，而 macOS 的散热控制始终在底层运行，M3、M4 等硬件还可能直接限制第三方风扇控制，应用无法绕过。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/smc-read-write-gap.webp" width="1360" height="454" loading="lazy" alt="读取路径从用户进程直接访问系统管理控制器，写入路径会在权限边界前停下，需要绕经特权辅助程序，系统自己的散热循环始终运行在两条路径之下。">
  <figcaption>风扇写入通常要跨越权限边界，辅助程序和卸载路径与控制界面同样重要。</figcaption>
</figure>

## 工具对比

### Macs Fan Control：常见起点

CrystalIDEA 的 [Macs Fan Control](https://crystalidea.com/macs-fan-control) 显示风扇、温度和第三方硬盘 S.M.A.R.T. 传感器，可设置固定 RPM，或让风扇跟随指定传感器。

截至 2026 年 7 月，它支持 Intel 与 Apple 芯片 Mac，2026 年 4 月版本包含当前 M 系列支持，基础版免费，Pro 版按电脑一次购买，增加预设保存。

它的[卸载说明](https://crystalidea.com/uninstall)公开了 `com.crystalidea.macsfancontrol.smcwrite.plist` 守护进程和 `com.crystalidea.macsfancontrol.smcwrite` 辅助程序，当前版本可从 More 菜单移除，适合需要传感器驱动曲线的人，但要记得自己设置过哪些规则。

### TG Pro：规则、日志和诊断

Tunabelly Software 的 [TG Pro](https://www.tunabellysoftware.com/tgpro/) 提供逐核心 CPU、GPU、S.M.A.R.T. 存储、电池和主板传感器，并支持 Auto Boost 规则、警报、CSV 日志与诊断报告。

截至 2026 年 7 月，支持范围从 macOS 10.13 到 macOS 26、Intel 到 M5，最近版本发布于 2026 年 3 月，产品一次购买，其[常见问题](https://www.tunabellysoftware.com/support/faq/)说明个人许可证最多覆盖三台 Mac，2.x 更新免费。

风扇写入通过 `/Library/PrivilegedHelperTools/com.tunabellysoftware.TGFanHelper` 和对应守护进程完成，需要自动规则和日志时，它比单个滑块更合适。

### iStat Menus：监控器附带风扇控制

Bjango 的 [iStat Menus](https://bjango.com/mac/istatmenus/) 首先是系统监控器，7.3 要求 macOS 11 或更高版本，传感器包括温度、风扇、频率和电压。

[风扇说明](https://bjango.com/help/istatmenus7/fans/)提供自动、按温度变化的曲线和手动模式，并明确只能提高转速，读取温度与风扇也需要独立辅助程序，授权为个人版或家庭版一次购买，也包含在 Setapp 中，需要完整监控器时再选它，更多比较见 [iStat Menus 替代品](https://mole.fit/zh/blog/istat-menus-alternative)。

### smcFanControl：历史项目

[smcFanControl](https://github.com/hholtmann/smcFanControl) 采用 GPL-2.0，原则是只提高最低转速，但最后一个稳定版 2.6 发布于 2016 年 10 月，之后只有 2018 年测试标签，最后提交在 2022 年 12 月，README 链接的构建仍来自 2016 年。

仓库对 Apple 芯片的描述也缺少正式发布验证，它适合了解这类工具的历史，不适合当前 Mac。

### Mole：三个临时预设

[Mole](https://mole.fit/zh/) 只有「自动」「降温」和「强冷」，不提供自定义曲线，SMC 写入由 `com.tw93.MoleApp.systemhelper` root XPC 辅助程序完成，通过 SMJobBless 安装，并验证 XPC 对端签名，不使用 sudoers 或 setuid。

预设只在应用运行时保持，退出后恢复自动控制，风扇与温度显示在「状态」页 60 秒趋势线和菜单栏中，它不支持传感器曲线，也不能修复散热问题，授权为一次购买，可用于两台运行 macOS 14 或更高版本的 Mac，包含免费更新，不发送遥测。

## 快速对比

| 工具 | 读取内容 | 风扇控制 | 特权组件 | 授权方式 |
| --- | --- | --- | --- | --- |
| Macs Fan Control | 风扇、传感器、硬盘 S.M.A.R.T. | 固定 RPM 或按传感器 | 路径公开的辅助程序与守护进程 | 免费，可选一次购买 Pro |
| TG Pro | 广泛传感器、电池与硬盘健康 | Auto Boost 规则，硬件允许时可手动 | 风扇辅助程序与守护进程，路径公开 | 一次购买，最多三台 Mac |
| iStat Menus | 完整监控器与传感器 | 曲线或手动，只提高 | 读取传感器也需独立辅助程序 | 一次购买，个人版或家庭版 |
| smcFanControl | 温度与风扇 | 只设置最低转速 | 仓库未说明 | GPL-2.0，2016 年后无正式发布 |
| Mole | 「状态」页中的温度与风扇 | 三个预设，退出恢复自动 | SMJobBless 辅助程序，验证签名 | 一次购买，两台 Mac |

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/helper-outlives-app.webp" width="1360" height="454" loading="lazy" alt="风扇应用被移到废纸篓后，位于 Library PrivilegedHelperTools 的特权辅助程序和位于 Library LaunchDaemons 的启动守护进程仍留在磁盘，并继续以 root 身份在开机时加载。">
  <figcaption>删除应用包不一定会移除独立辅助程序，应使用厂商提供的卸载路径。</figcaption>
</figure>

## 使用顺序

编译、导出或游戏时风扇加速，通常说明散热系统正在工作，坚硬平面和畅通风口往往比手动提高最低转速更值得先试，不过这里也没有适用于所有环境的固定答案。

手动模式只适合一项已知任务，任务结束后应恢复自动控制，无法在退出或睡眠时自动恢复的工具不值得长期使用，卸载时也要按文档移除独立辅助程序。

空闲时风扇仍很响，需要的是诊断，见[为什么 MacBook 风扇很响](https://mole.fit/zh/blog/macbook-fan-loud-overheating)，担心温度数字时，可看[如何查看 Mac 温度](https://mole.fit/zh/blog/how-to-check-mac-temperature)。大多数人不需要手动曲线，固件比第三方工具更了解机型与硬件限制。

---

Canonical HTML page: https://mole.fit/zh/blog/mac-fan-control-apps
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
