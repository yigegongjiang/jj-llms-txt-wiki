# 如何判断 Mac 上的陌生进程是否安全

> 进程名只能作为线索。结合用户、路径、父进程、代码签名、资源趋势和打开文件判断身份。

Published: 2026-06-16 | Updated: 2026-09-05

「活动监视器」会列出应用、辅助程序、代理、守护进程和系统服务，而进程名只能作为线索，因为不同软件可能使用相似名称，恶意软件也可以伪装，判断时要结合所属用户、路径、父进程、代码签名、资源趋势和触发操作。

选择「显示 > 所有进程」，再加入用户、CPU 时间、线程和种类等列，双击进程可查看父进程、打开的文件与端口，Apple 的[活动监视器说明](https://support.apple.com/guide/activity-monitor/actmntr1001/mac)还介绍了分层与系统进程视图，进程疑似卡住时，先用「取样进程」记录一小段线程活动，不必立即结束。

## 常见的高占用进程

- **kernel_task：** 反映 macOS 内核工作，高 CPU 可能与温度控制有关，但不能只凭使用率判断，排查路径见 [kernel_task 高 CPU](https://mole.fit/zh/blog/kernel-task-high-cpu-mac)
- **WindowServer：** 负责合成屏幕，负载会随显示器、窗口和动画增加，详细处理见 [WindowServer 高 CPU](https://mole.fit/zh/blog/windowserver-high-cpu-mac)
- **mds、mds_stores、mdworker：** 负责 Spotlight 索引，索引期间会比较忙，见 [mds 与 mdworker 高 CPU](https://mole.fit/zh/blog/mds-mdworker-high-cpu-mac)
- **Google Chrome Helper：** 承载 Chrome 标签页、扩展和浏览器服务，一个标签页可能对应多个进程，见 [Chrome Helper 高 CPU](https://mole.fit/zh/blog/google-chrome-helper-high-cpu-mac)

## 常见的后台守护进程

- **launchd：** macOS 的第一个用户态进程（PID 1），管理许多系统服务，但不一定是每个服务的直接父进程，始终运行是正常行为
- **trustd：** 在应用启动或建立安全连接时检查证书与签名，短暂升高很常见
- **nsurlsessiond：** 处理 iCloud、应用下载和更新等后台网络传输
- **cloudd、bird：** 负责 iCloud 同步，刚登录或加入大量文件后会较忙
- **coreaudiod：** 系统音频引擎，持续高 CPU 可能与音频应用或插件有关
- **photoanalysisd：** 分析照片中的人物和物体，调度会受供电、温度和资料库变化影响
- **backupd：** Time Machine 备份进程，备份期间活跃很正常
- **syspolicyd：** 执行应用评估等系统安全策略，安装或首次启动应用时可能短暂升高

## 看变化模式，不看固定时长

进程在打开应用、接入磁盘或开始同步后升高，工作有进展并最终下降，通常不必处理，而 CPU 长期不退、内存快速增长、反复崩溃，或每次同一操作都触发异常峰值，才需要深入检查。

强制退出前，先查看进程取样、打开文件和日志，许多系统服务会自动重启，因为请求它的应用或启动策略还在。

第三方进程的路径和签名可以用来核对厂商身份，而 Apple 服务出现异常时，工作量通常来自某个应用、文件、设备或网络操作，直接修改系统启动配置很少能解决源头。

## 退出之前先证明它属于谁

活动监视器的检查器最直观。终端里从 PID 开始：

```
PID=1234
ps -p "$PID" -o pid=,ppid=,user=,etime=,comm=
lsof -p "$PID" | head
```

父进程和可执行路径能把通用 helper 关联到启动它的应用，打开的文件则说明当前是哪一个项目、资料库、设备或数据库在喂给它工作。如果路径属于第三方应用，可以只读检查签名：

```
codesign -dv --verbose=4 "/path/to/the/binary" 2>&1
```

熟悉的名字却落在意外路径或由陌生身份签名，才值得继续调查。陌生名字位于签名正常的 Apple 系统位置，不会因为名字陌生就变得可疑。先退出所属应用，再看 helper 是否安静下来或自行退出，不要直接强退子进程。

## 为什么后台进程会不断变化

launchd 是 PID 1，负责注册和按需启动大多数服务，安全连接需要 trustd、文件需要元数据导入、应用发来 XPC 消息时，对应服务才会启动，工作结束后再回到空闲，所以进程列表会随操作不断变化，强制退出某个服务后，下一次触发条件出现时 launchd 仍会把它重新启动，某个名字是否存在并不重要，长期行为才是异常信号。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/launchd-on-demand.webp" width="1360" height="454" loading="lazy" alt="launchd 位于中心，在连接、文件或 XPC 消息等触发条件出现时按需启动小型服务，工作完成后再让它们空闲，因此运行中的集合会不断变化。">
  <figcaption>launchd 按触发条件启动服务，工作结束后让它们空闲。强制退出通常只能维持到下一次触发。</figcaption>
</figure>

## 通俗解释有什么用

「活动监视器」提供权威的进程数据，[Mole](https://mole.fit/zh/) 的「状态」页可以补充通俗说明和趋势，但按名称给出的解释只是起点，路径、用户、签名和行为才能确认当前进程的身份。

## 一次可重复的进程检查

遇到异常时，记录名称、用户、路径、父进程、签名、CPU 与内存趋势、打开文件和触发条件，问题发生时取样，再先停止发起请求的应用或输入源，这样更容易分清正常后台工作、卡住的厂商辅助程序和冒用名称的进程。

---

Canonical HTML page: https://mole.fit/zh/blog/mac-processes-explained
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
