# Mac 变慢了，可以从哪里排查

> 在卡顿发生时检查 CPU、内存压力、磁盘空间与读写、散热和网络，找到真正饱和的资源。

Published: 2026-07-04 | Updated: 2026-09-28

Mac 变慢，通常是某项资源在卡顿发生时已经饱和：CPU、内存、磁盘空间、磁盘读写、散热或网络服务。等问题消失后再看一张状态截图，很难找到原因。

## 别把空闲内存当成目标

macOS 会主动用空闲内存缓存近期文件，需要空间时再释放。可用内存很少，不等于内存不足。

系统还会压缩不活跃的内存页面，减少写入较慢交换空间的次数。查看计数：

```
vm_stat
```

输出以页面为单位，页面大小在顶部附近。重点不是 `Pages free`，而是内存压力和变化速度。`Swapins`、`Swapouts` 是开机以来的累计值，数字大不代表此刻有问题。在卡顿期间运行两次，观察增量，也可以直接看「活动监视器」内存页。

Apple 的[内存说明](https://support.apple.com/guide/activity-monitor/actmntr34865/mac)把内存压力分为绿色、黄色和红色，压力图、交换空间趋势和应用列表放在一起，比空闲内存更有参考价值。

## 找到正在饱和的资源

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/status.webp" width="2584" height="1741" loading="lazy" alt="系统状态面板显示 CPU 占用 16%、温度 47 度，GPU 占用 0%，内存占用 72%、压力 28%，以及磁盘、网络和风扇，下方按 CPU 与内存列出高占用进程。">
  <figcaption>同时查看 CPU、温度、内存压力和高占用进程。这是 Mole 的「状态」页。</figcaption>
</figure>

在卡顿发生时，依次检查：

- **CPU：** 运行 `top -o cpu`，或在「活动监视器」CPU 页排序。%CPU 按核心计算，超过 100% 只表示进程同时使用多个核心。构建和导出时很正常，空闲应用长期如此才异常
- **内存：** 看内存压力和交换空间是否持续上升
- **磁盘容量：** 运行 `df -h /`。APFS 需要空间保存交换数据和临时文件，磁盘接近用尽会妨碍更新和大型任务。没有通用的安全百分比，要看剩余容量是否满足当前任务。容量不足时，先[找出大文件](https://mole.fit/zh/blog/how-to-find-large-files-on-mac)，再[安全释放空间](https://mole.fit/zh/blog/how-to-free-up-space-on-mac)
- **磁盘读写：** 在「活动监视器」磁盘页按读取或写入字节排序，并观察卡顿时的曲线。`iostat -w 1` 提供实时命令行视图。同步、备份、构建和故障外置盘，都可能在空间充足时造成延迟
- **散热：** 见后文
- **网络与服务：** 如果只有一款云端应用变慢，本地应用仍流畅，先检查它的网络活动和服务状态

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/performance-bottlenecks.webp" width="1360" height="454" loading="lazy" alt="一项工作负载分叉为 CPU 饱和、内存压力、磁盘读写和散热降频，最后都表现为延迟。">
  <figcaption>CPU、内存、磁盘和散热问题都会表现为卡顿，但处理方法不同。</figcaption>
</figure>

## CPU 被某个进程占满

`top -o cpu` 会把高占用进程放在顶部。常见原因是应用卡住、同步客户端重复扫描，或辅助程序陷入循环。确认可以安全结束后，正常退出它。

进程几秒后自动重启，说明背后可能有 launchd 任务或登录项，只结束进程不会持久生效。系统进程也不能按普通应用处理，例如 `kernel_task` 升高常常是在芯片过热时限制其他任务继续发热，它更像散热信号。

## 散热降频

Mac 过热时会降低芯片频率，高负载任务因此变慢。查看系统记录的散热状态：

```
pmset -g therm
```

采集一次功耗与散热压力样本：

```
sudo powermetrics --samplers thermal,cpu_power -n 1
```

相同负载下的变化比单个温度更可靠，因为不同机型没有统一上限。瓶颈确实来自降频时，方向通常是减少负载、改善通风或检查硬件，清缓存不会改变散热，无风扇 Mac 也会降频，[MacBook 风扇为什么很响](https://mole.fit/zh/blog/macbook-fan-loud-overheating)介绍了更完整的背景。

## Spotlight 和登录项

macOS 更新、迁移或大量移动文件后，Spotlight 可能重新索引。`mdutil -s /` 只能说明索引是否开启，不能证明正在重建。持续的 `mds` 或 `mdworker` CPU、磁盘读写和 Spotlight 进度一起出现，才说明索引正在工作。所需时间取决于数据量、磁盘速度、权限和文件是否持续变化，没有固定的一小时承诺。

再检查「系统设置 > 通用 > 登录项与扩展」。登录时打开的应用和允许后台运行的软件可能使用 Service Management、launchd、扩展或辅助程序。一次只关闭一个身份明确的项目，并确认所属应用仍能正常工作。

## 实时监控如何保持轻量

Mole 的[开源命令行版](https://github.com/tw93/Mole) `cmd/status` 和原生 Mac 应用使用不同采集器，但都会把快速指标与慢速信息分开处理。

命令行版解析 `vm_stat` 的文件缓存页数，并读取 `memory_pressure`，不会根据空闲字节自行推算压力。原生应用直接通过 `host_statistics64` 获取 VM 计数，通过 `kern.memorystatus_vm_pressure_level` 读取压力等级。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/tiered-metrics-sampling.webp" width="1360" height="454" loading="lazy" alt="快速、较慢和一次性系统指标经过并发采集器或缓存，合并为单一快照，进入环形缓冲区，再显示在状态页与菜单栏 HUD 中。">
  <figcaption>快速指标实时刷新，慢速探针复用缓存，合并后的快照只保留有限历史。</figcaption>
</figure>

CPU、内存和网络等低成本指标约每秒更新。进程、GPU 和磁盘信息降低频率，硬件与设备探针使用更长缓存，趋势线只保留近期样本，以减少监控本身的开销。

## 工具能省下什么

这些数据都能用系统命令读取，诊断并不依赖第三方应用。[Mole](https://mole.fit/zh/) 的「状态」页只是把 CPU、内存压力、磁盘、温度和高占用进程集中到一个页面。点击进程还能查看它由什么启动、是否阻止休眠，以及正在读写什么。

避免会同时修改无关状态的「一键加速」。强行清空有用的文件缓存不会带来持久提升。重启可以结束卡住的进程，也是部分更新的必要步骤，但它可能暂时掩盖会再次出现的问题。

## 可复现的诊断

复现卡顿后，同时观察 CPU、内存压力、磁盘容量与读写、散热和网络。每次只改变一个变量，再用相同负载复测。找到饱和资源，才知道该关闭进程、释放空间、改善散热，还是等待网络服务恢复。

---

Canonical HTML page: https://mole.fit/zh/blog/why-is-my-mac-so-slow
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
