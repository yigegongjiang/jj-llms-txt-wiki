# iStat Menus 的替代方案怎么选

> 想看实时负载，用免费工具通常足够，需要回看历史，再考虑 iStat Menus。这里比较采样成本、传感器和授权方式。

Published: 2026-07-22 | Updated: 2026-09-27

菜单栏监控器只是把系统指标放到随时可见的位置，它修不好高占用进程、满磁盘或散热降频，但可以指出问题落在哪项资源上，所以选择时，自己反复想回答的问题比界面能塞多少数字更重要。

## 只看实时，还是需要历史

**实时读数**每秒左右更新，用来回答「现在发生什么」，**历史记录**会保存样本，用来追查已经过去的问题，例如夜间发热或上周耗电。

大多数人只需要实时读数，付费监控器的重要差异往往是历史保留，从不回看就不必为它买单。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/readout-vs-record.webp" width="1360" height="454" loading="lazy" alt="左侧实时读数以一秒轨迹回答现在发生什么，右侧保存的历史样本回答上周二发生过什么。">
  <figcaption>实时读数回答现在，历史记录回答过去，选择取决于自己常问哪一种问题。</figcaption>
</figure>

## 监控器本身也有成本

多数工具都从相似的系统来源读取数据，但采样窗口、缓存策略、传感器命名和计算方式不同，小幅差异不一定代表谁错了。

逐进程功耗和细粒度 GPU 功耗通常依赖需要 root 的 `powermetrics`，工具要么请求管理员权限、安装辅助程序，要么使用较便宜的数据估算，安装前应看清来源和权限。

每秒轮询会唤醒进程并重绘界面，CPU、内存等简单计数很轻，枚举进程、扫描传感器、查询硬盘健康和 GPU 则更贵，较好的实现会分层采样，便宜指标频繁更新，昂贵探针降低频率或使用缓存。

内存应看压力，不应只看空闲 GB，macOS 会用空闲内存缓存文件，并在应用需要时释放，空闲内存低往往只是系统正在有效利用 RAM。

传感器覆盖也首先取决于机型，Apple 芯片和 Intel Mac 暴露的温度、风扇、电压与功耗传感器不同，无风扇 MacBook Air 自然没有转速，工具对传感器列表有差异时，常见原因是命名表不同。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/sampling-tiers.webp" width="1360" height="454" loading="lazy" alt="三条采样通道按成本排序：便宜的内核计数器每秒轮询，IOKit 传感器以较慢频率读取，需要 root 的 powermetrics 极少采样。">
  <figcaption>不同指标使用不同采样频率，监控器才不会成为新的负载。</figcaption>
</figure>

## 可选工具

### 活动监视器：系统自带

Apple 的[活动监视器](https://support.apple.com/guide/activity-monitor/welcome/mac)覆盖 CPU、内存、能耗、磁盘、网络和缓存，也能把 CPU 图表放进程序坞，进程检查器、取样和打开文件详情，比多数菜单栏工具更深入。

缺点是需要主动打开，Mac 一年只偶尔出现异常时，它已经足够。

### Stats：免费开源的默认选择

[Stats](https://github.com/exelban/stats) 采用 MIT 许可证，显示 CPU、GPU、内存、磁盘、网络、电池、传感器、蓝牙设备和多时区时钟，也有旧式风扇控制模式，它支持 macOS 12 及以后，可通过 `brew install stats` 安装。

项目仍持续发布新款 Apple 芯片修复，文档未说明长期历史存储，因此应把它当作实时读数工具，多数只想在菜单栏看系统状态的人，可以先用它。

### iStat Menus：长期历史最完整

Bjango 的 [iStat Menus](https://bjango.com/mac/istatmenus/) 提供逐核心 CPU 与历史图、负载平均值、内存压力与交换空间、磁盘 `S.M.A.R.T.`、按应用网络带宽、温度与风扇、电池和配件电量、天气与时钟，7.3 要求 macOS 11 或更高版本。

历史图和按应用带宽是它相对免费工具的主要价值，授权为个人版或家庭版一次购买，旧大版本可折扣升级，也提供试用并包含在 Setapp 中，具体条款以官网为准，它只做监控，不负责清理或修复。

### MenuMeters：推荐已经过期

常见分支 [yujitach/MenuMeters](https://github.com/yujitach/MenuMeters) 采用 GPL-2.0，支持说明截至 Big Sur，最后发布在 2021 年 11 月，最后提交在 2022 年 3 月，README 也建议改用 Stats 等仍在维护的工具，旧文章里的 MenuMeters 推荐已不适合当前 macOS。

### Sensei：硬件健康加清理

Cindori 的 [Sensei](https://cindori.com/sensei) 提供 CPU、GPU、电池、温度、风扇、S.M.A.R.T. 硬盘健康、磁盘测速、SSD Trim 和状态栏监控，也附带清理与卸载。

[商店页面](https://cindori.com/store/sensei)提供年度订阅和一次购买，最多覆盖三台 Mac，主要关心硬盘与电池健康时可以考虑，官网未明确最低 macOS 版本，购买前需确认兼容性。

### Mole：维护应用里的实时视图

[Mole](https://mole.fit/zh/mac-system-monitor) 的「状态」页显示 CPU、内存、GPU、磁盘、网络、电池、温度、风扇和运行时间，支持曲线的指标会保留 60 秒趋势。进程列表支持排序、置顶、结束进程和复制路径，菜单栏 HUD 与弹窗共用采样结果，不为各自显示重复采样。

它只回答「现在发生什么」，不能回看上周，需要长期历史时应选其他产品，Mole 一次购买，可用于两台运行 macOS 14 或更高版本的 Mac，包含免费更新，监控和扫描免费，不发送遥测，免费开源 CLI 也支持 `mo status` 与 `mo status --json`。

## 快速对比

| 工具 | 采样内容 | 保存历史 | 授权方式 |
| --- | --- | --- | --- |
| 活动监视器 | CPU、内存、能耗、磁盘、网络、缓存 | 仅实时 | macOS 自带 |
| Stats | CPU、GPU、内存、磁盘、网络、电池、传感器、蓝牙 | 文档未说明 | MIT 开源 |
| iStat Menus | 上述内容，加 `S.M.A.R.T.`、按应用带宽、天气 | 有历史图 | 一次购买，个人版或家庭版 |
| MenuMeters | CPU、内存、磁盘、网络 | 无 | GPL-2.0，2022 年后未维护 |
| Sensei | 硬件、温度、电池与硬盘健康、基准测试 | 查看官网 | 订阅或一次购买，最多三台 Mac |
| Mole | CPU、GPU、内存、磁盘、网络、电池、温度、进程 | 60 秒趋势线 | 一次购买，两台 Mac |

## 一套可重复的监控流程

想知道当前谁在占 CPU，活动监视器或 Stats 就能回答，想知道离开时发生什么，需要保存历史的工具，想判断硬盘或电池退化，则更接近健康报告，监控器自身一分钟的 CPU 占用也值得观察，从不查看的模块可以关闭。

监控器指出瓶颈后，修复仍在工作负载、应用、存储或通风，具体步骤见[诊断 Mac 变慢](https://mole.fit/zh/blog/why-is-my-mac-so-slow)和[如何查看 Mac 温度](https://mole.fit/zh/blog/how-to-check-mac-temperature)。

---

Canonical HTML page: https://mole.fit/zh/blog/istat-menus-alternative
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
