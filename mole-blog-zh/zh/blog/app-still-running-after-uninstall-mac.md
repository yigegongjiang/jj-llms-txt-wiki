# Mac 应用卸了还在后台跑？定位登录项与启动任务

> 卸载后后台进程仍可能重新启动，先查清对应的登录项或 launchd 服务，再处理残留文件。

Published: 2026-09-14 | Updated: 2026-09-24

把应用拖进废纸篓，通常只删了「应用程序」里的应用包。已注册的登录项、LaunchAgent 或后台服务，不一定会一起停掉。卸载后在活动监视器里还能搜到它，先别反复强制退出，查查是谁又把它启动了。

这篇只处理「卸了还在跑」：认出进程、查清归属，再决定关哪一项。完整卸载步骤见 [如何彻底卸载 Mac 应用](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)；登录项与 launchd 的总览见 [如何关闭 Mac 开机启动项](https://mole.fit/zh/blog/how-to-disable-startup-programs-on-mac)。

## 先确认留下的是什么进程

1. 打开「活动监视器」，搜索进程名。
2. 看 CPU 和内存占用，以及退出后会不会马上回来。
3. 选中进程，用「取样」或「打开的文件和端口」查看路径是在 `/Applications`、`~/Library` 还是 `/Library`。
4. 若退出后到下次登录才出现，多半是登录时启动，不是反复崩溃重启。
5. 若几秒内就重新出现，查 LaunchAgent，或是否有其他应用在自动重启它。

同一厂商的更新器、同步客户端、输入法助手，可能被多个产品共用。没查清归属就强制退出，可能影响还在用的软件。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/launchd-relaunch-loop.webp" width="1360" height="454" loading="lazy" alt="应用包删了之后，登录项或 launchd 仍可能把辅助进程拉起来">
  <figcaption>删掉应用包后，登录项和 launchd 仍可能启动辅助进程。</figcaption>
</figure>

## 删任何东西之前，先查清进程归属

| 线索 | 去哪里看 | 通常说明什么 |
| --- | --- | --- |
| 路径仍在 `/Applications/某某.app` | 访达 | 应用其实还在，只是删了替身或另一份副本 |
| 路径在 `~/Library/Application Support` 或厂商目录 | 访达 / `ls` | 支持文件还在，辅助程序可能仍注册着 |
| `launchctl` 里有匹配厂商的标签 | 终端（只读） | LaunchAgent 或 LaunchDaemon 在启动它 |
| 登录项列表里有对应条目 | 系统设置 | 下次登录还会打开 |
| 名字很泛，如 `Helper`、`Updater`，归属不清 | 厂商说明 | 先别删；共享助手要按对应产品的卸载说明处理 |

以下命令只查看，不会修改系统：

```shell
ps aux | grep -i [name]
launchctl print gui/$(id -u) 2>/dev/null | grep -i [name]
```

把 `[name]` 换成进程名里较短、较独特的一段。搜索结果只是线索，不是删除清单。

## 优先关掉登录项和后台项

Apple 的 [登录时自动打开项目](https://support.apple.com/guide/mac-help/mh15189/mac) 说明了登录项用法。

1. 打开「系统设置 › 通用 › 登录项与扩展」。
2. 在「登录时打开」里，只移除已卸应用对应的那一项。
3. 在「允许在后台」里，若有对应开发者，关掉该开关。
4. 注销再登录，查看活动监视器。
5. 确认进程消失后，再考虑清理残留文件。

关掉登录项不等于卸载，也不会取消订阅，只是不再自动启动。

## 若是 launchd 在反复拉起

登录项里没有条目，进程却不断回来时，到以下目录查找对应的代理配置：

- `~/Library/LaunchAgents`
- `/Library/LaunchAgents`
- `/Library/LaunchDaemons`

有厂商卸载器就优先用。凭感觉删 plist，可能删了配置却没卸载已加载的服务。卸载失败、必须手动处理残留代理时：

1. 确认标签对应的就是已卸产品。
2. 按厂商文档把它从 launchd 卸载（unload）；只有文档点名该标签时，才使用 `launchctl bootout`。
3. 确认已从 launchd 卸载后，再把 plist 移到废纸篓。
4. 重启后确认进程已消失，其他应用仍正常。

不要删 `/System` 下的内容。安全软件、MDM 代理和 VPN 助手，没有获得授权就不要停用。

## 共享助手和真残留怎么分

| 情况 | 更稳妥的做法 |
| --- | --- |
| 助手路径在仍在使用的应用里 | 留着；只卸你打算卸的产品 |
| 只有打开同一厂商另一个应用时才出现 | 助手属于那个应用 |
| 登录就启动，对应应用已不在 | 先关登录项或后台开关，再核对残留文件 |
| 名字像广告软件或未知未签名程序 | 先找到并隔离文件，别运行；查清楚再动资源库 |

进程停下后，再检查偏好设置和支持文件。[Mac 卸载残留：删除前先核对](https://mole.fit/zh/blog/mac-uninstall-leftovers-review-before-delete) 介绍了如何按归属和体积核对，不要把整个资源库当垃圾。

## 不要做的事

- 不要反复强制退出，去查是谁在启动助手。
- 不要因为某个名字眼熟就清空整个 `~/Library/LaunchAgents`。
- 不要关掉你还依赖的备份、安全或管理类 LaunchDaemon。
- 不要以为清空废纸篓就移除了登录项；它们不在应用包里。

如果 Mole 卸载时列出了残留代理，也要先确认它属于哪个产品，再决定删不删，别一键全删。

## 常见问题

### 为什么强制退出后进程又回来？

登录项、LaunchAgent、其他应用或厂商更新器仍可能启动它。强制退出只结束当前进程，不会阻止它再次启动。

### 能按应用名删光所有同名文件吗？

不能。共享容器、插件和文档可能都带有厂商名。要核对 bundle ID 或厂商文档列出的路径，别把工程文件也当成卸载残留。

### 需要进安全模式吗？

只有进程不断重启、访达又删不掉时才考虑。Apple 的卸载说明也把安全模式作为后续办法，不是第一步。

### 关掉「后台项目」等于卸载了吗？

不等于。它只限制对应开发者项目的自动后台运行，应用包还在的话，仍需单独移除。

---

Canonical HTML page: https://mole.fit/zh/blog/app-still-running-after-uninstall-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
