# Mac 应用卸不掉：七种常见原因与排查方法

> 提示正在使用、删掉又回来、没有反应或选项变灰时，结合助手进程、launchd、SIP、App 管理和 MDM 排查，不能只凭一个症状断定原因。

Published: 2026-08-18 | Updated: 2026-09-24

macOS 上的卸载失败不是同一个问题，是好几个。访达可能拒绝移动应用包，也可能移动成功了，下次登录它又回来了，还可能包没了，而对应的助手进程照跑不误。每一种都是不同机制、不同解法，所以先看这台 Mac 实际做了什么。常规顺序见[彻底卸载 Mac 应用](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)，下面这些都假设那一套已经失败了。

## 从症状开始

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/uninstall-failure-routing.webp" width="1360" height="454" loading="lazy" alt="六种卸载失败症状：提示项目已打开、删掉之后又回来、完全没反应、要求输入管理员密码、以 root 身份跑 rm 报操作不被允许、以及设置里的选项变灰，每一种各自指向不同的成因。">
  <figcaption>拒绝时的措辞本身就是诊断依据。这六种里有两种根本不弹任何对话框，所以常被读成「工具坏了」。</figcaption>
</figure>

- **「无法将项目移到废纸篓，因为它已打开。」** 有东西还在跑，往往不是你刚退出的那个应用。
- **删掉了，它又回来了。** 先分清是仍在磁盘上的助手又启动，还是应用包被管理软件或包管理器重新安装。
- **访达要求输入管理员密码。** 某个 `.pkg` 让应用包归 root 所有，这很正常。
- **完全没反应，没有对话框也没有报错。** App 管理权限是可能原因之一，还要核对操作结果和文件权限。
- **以 root 跑 `rm` 提示「操作不被允许」。** 可能涉及系统保护或其他限制，不能只凭这句报错判断为 SIP。
- **选项变灰，或者在公司的 Mac 上又冒出来。** 一份配置描述文件或 MDM 拥有它。

## 1. 应用本身或它的某个助手还在跑

应用包里只要还有文件开着，访达就不肯移动它，而退出你看得见的那个进程，并不会停掉这个应用启动的全部东西。活动监视器列的是每一个进程，不只是有窗口的那些。搜厂商名，然后把整组结果都读一遍：一个叫 Foo 的应用常常还带着 `Foo Helper`、一个 `FooUpdater`，以及一个显示名跟它毫不相干的登录项包。**苹果菜单 > 强制退出**那个窗口替代不了这一步，因为它只列有界面的应用。

```
pgrep -fl -i foo
lsof +D /Applications/Foo.app 2>/dev/null | awk '{print $1, $2}' | sort -u
```

`pgrep -fl` 匹配的是完整命令行，所以哪怕助手进程的进程名里没有厂商名，只要可执行文件路径里有，它也能抓到。`lsof +D` 报出所有正持有这个应用包内文件的进程。

强制退出父进程并不能可靠地停掉助手。助手是自己的进程、自己的 PID，杀掉父进程只是让它成了孤儿进程，除非父进程在退出时主动把它拆掉。而如果助手由 launchd 管理，杀掉其中任何一个都只是在告诉 launchd 再启动一次，这就是下一节的内容。

一次快速查看的预览或者聚焦的索引过程，也可能持有包内的文件，重启一次两者都会释放。提示反复出现的话，Apple 的[删除应用指南](https://support.apple.com/guide/mac-help/mh35835/mac)建议进安全模式。

## 2. 某个 launchd 代理或守护进程把它拉了回来

这就是「我删了它又回来了」那一种，也是最常被误诊的：人们打开活动监视器，什么都没看到，于是断定这个应用没在跑，可按需启动的东西时有时无，看不见不算证据。

launchd 负责看管后台任务。一个任务就是一份属性列表，带 `Label`、一个程序，以及运行它的条件。`RunAtLoad` 在任务加载时启动它，`KeepAlive` 在它退出时重启它。两者都没有的任务照样会回来：`MachServices`、`Sockets`、`WatchPaths`、`QueueDirectories` 和 `StartInterval` 都会让 launchd 在有人来要的那一刻把进程拉起来，所以有的助手空闲十秒就退出，靠 Mach 服务再被唤起，你每次去看它都不在。

定义住在三个地方。`~/Library/LaunchAgents` 只属于当前用户，跑在登录会话里；`/Library/LaunchAgents` 在每个用户登录时运行；`/Library/LaunchDaemons` 在任何人登录之前就以 root 身份全系统运行，这也是为什么手动卸载之后守护进程往往活下来、而代理常常没有。`/System/Library/Launch*` 是 Apple 的，受保护。

```
launchctl list | grep -i foo
launchctl print-disabled gui/$(id -u) | grep -i foo
grep -l -i foo ~/Library/LaunchAgents/*.plist /Library/LaunchAgents/*.plist \
  /Library/LaunchDaemons/*.plist 2>/dev/null
```

在 `launchctl list` 里，带 PID 的标签是此刻正在跑，带短横线的是已加载、正等触发条件；`print-disabled` 读的是持久化的禁用状态数据库，那和「plist 存不存在」是两码事。用 `plutil -p <path>` 读一读候选项，确认 `Program` 或 `ProgramArguments` 指向的确实是你要删的那个应用，厂商不总是按里面的标签给文件命名。

### 顺序很重要

先停掉并禁用任务，再删 plist，最后删应用。

```
launchctl bootout gui/$(id -u)/com.vendor.foo.helper
sudo launchctl bootout system/com.vendor.foo.daemon
launchctl disable gui/$(id -u)/com.vendor.foo.helper
```

任务还加载着就删 plist，launchd 手里就留下一个服务，而它的定义在磁盘上已经没有了，这个服务会一直跑到下次重启，还可能在禁用状态数据库里留下一条陈旧记录。先删应用更糟：任务会对着一个不存在的可执行文件反复重生并失败，「它明明没了，登录项里还显示着」就是这么来的。

macOS 13 及以后，应用可以通过 [`SMAppService`](https://developer.apple.com/documentation/servicemanagement/smappservice) 注册后台组件，助手及其定义可能随应用包分发，不一定在上面三个目录里另放一份 plist。注册状态由系统管理，不是保存在应用包里的那份文件，可以在**系统设置 › 通用 › 登录项与扩展**里查看和控制（见[关掉开机启动项](https://mole.fit/zh/blog/how-to-disable-startup-programs-on-mac)）。

## 3. 它是受系统完整性保护的系统应用

系统完整性保护是一条内核级策略，不是一个权限位。Apple 的描述是它用内核权限限制关键系统文件的可写性，作用于「系统上运行的每一个进程，不论该进程是否运行在沙盒中或是否具有管理员权限」（[Apple 平台安全](https://support.apple.com/guide/security/system-integrity-protection-secb7ea06b49/web)）。从 Big Sur 起，系统内容还放在一个独立的、经过加密封存的卷上。`csrutil status` 会打印保护是否开启，而 Apple 明说「你无法删除 Mac 所需的应用」，这一组包括邮件、音乐、图书、备忘录、播客、地图、新闻和股市。

为了删掉一个应用包去关系统完整性保护，是笔亏本买卖。那意味着进恢复模式，改一条影响整台机器的安全策略；Apple 指出在 Intel 机型上，关掉它会让这块物理存储上的每一个分区都失去保护，而 Apple 芯片机型会离开完全安全性等级。而且收益也留不住：下次 macOS 更新时系统卷会被整卷替换，也一点可用空间都收不回来。

更好的做法是：把图标从程序坞里拖走，在**系统设置 > 通用 > 登录项与扩展**里移除它，如果它老是抢着打开文件，就用**显示简介 > 打开方式 > 全部更改**换掉默认处理程序。

## 4. 它是由 MDM 或配置描述文件装上的

在受管理的 Mac 上，一个应用可能按管理服务器的排期被推回来，而一份描述文件可以被标记为不可移除。症状可能是删完后又被安装、某个控件是灰的，或者在 launchd 里找不到对应任务，因为重新安装由远端发起。先看**系统设置 › 通用 › 设备管理**，没有看到这一节也不要仅凭界面排除管理状态，继续核对下面的注册信息，公司或学校的 Mac 可以向 IT 确认。

```
profiles status -type enrollment
sudo profiles list
```

第一条报的是自动设备注册的状态，以及有没有经过用户批准，第二条列出已安装的描述文件，需要 root。然后去找 IT。Apple 给的建议是，遇到自己移除不掉的描述文件就去问提供它的人，同时提醒移除一份描述文件会一并删掉这份文件配置过的所有东西，所以带着你邮件账户的那份，会把账户也带走。

## 5. 归属，以及那个悄无声息失败的权限

**访达要求输入管理员密码。** 很正常。`.pkg` 安装程序以 root 身份运行，留下的应用包归 root 所有，所以移动它需要认证，用 `ls -ld /Applications/Foo.app` 就能确认。顺带说说安装记录：`pkgutil --files <id>` 列出某个包放下的路径，`sudo pkgutil --forget <id>` 把这条记录从 `/private/var/db/receipts` 里去掉，一个文件都不会删。

**完全没有反应。** 没有提示，没有报错，应用还在 `/Applications` 里，你用的那个工具报了个含糊的失败，或者干脆转头去做别的了。在 macOS 14 及以后，这通常是 App 管理，Apple 对它的描述是「允许 App 更新或删除 Mac 上的其他 App」。判据是这条路径按 POSIX 是可写的，写却仍然失败：

```
test -w /Applications/Foo.app && echo "posix says yes"
```

这句只检查应用路径是否可写，不能证明可以删除它，删除还取决于父目录的写入和搜索权限等条件。如果权限检查没有解释失败，再去**系统设置 › 隐私与安全性 › App 管理**核对执行删除的应用是否获准，修改权限后重新启动它。

## 三种长得一样的拒绝，底下不是一回事

POSIX 权限是第一道闸：应用包归 root 所有，你不是 root，`sudo` 就能解决，因为这道检查只看你是哪个用户。TCC，也就是系统设置背后那层隐私机制，是在 POSIX 通过之后才判断的。App 管理是针对一种操作的 TCC 闸门，也就是修改或删除另一个应用的包。管理员身份不能满足它，`sudo` 也不行，因为它认的是发起请求的那个程序而不是用户，这也是为什么它的拒绝会表现成一个没有提示的笼统错误。

从表面现象就能分开这三种情况。访达弹出密码框，是普通的归属问题，输入密码认证就能解决。App 管理拦下删除时不会弹出密码框，macOS 通常会在通知中心发一条横幅，说这个应用被阻止修改 Mac 上的 App，这时要做的是去隐私设置里打开开关、再重新启动那个应用，而不是输入密码。以 root 身份得到 `Operation not permitted`，说明挡路的是输入密码也解除不了的保护。App 管理就是其中之一，发起命令的终端没有获得这项授权时，连 `sudo` 也会得到这个报错。别的保护机制也会报同样的错，所以先按路径和发起请求的应用排查，再考虑是不是 SIP。

系统完整性保护也会限制 root，但不能把报错文本和某一种保护机制直接画等号。`EACCES`（`Permission denied`）可能来自父目录的写入或搜索权限，`EPERM`（`Operation not permitted`）也不只由 SIP 引起，具体条件见 [Apple 的文件删除说明](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unlink.2.html)。先确认失败路径和权限，不要看到报错就提高权限或关闭系统保护。

## 6. 某个系统扩展或网络过滤器还活着

安全软件、VPN 客户端和虚拟化产品都会装系统扩展。扩展是应用包注册上去的，应用包本身不是扩展。在扩展还活着的时候删掉承载它的容器，注册记录就没了归属，这正是为什么一台 Mac 会继续用你以为已经删掉的软件过滤流量。

```
systemextensionsctl list
```

输出会显示团队标识符、bundle 标识符和形如 `[activated enabled]` 的状态。停用属于承载它的那个应用，而且往往需要重启，所以先跑厂商自己的卸载器。[怎么卸载 Mac 上的杀毒软件](https://mole.fit/zh/blog/how-to-uninstall-antivirus-mac)讲了这一类的拆除顺序。

## 7. 它来自 App Store，或者来自 Homebrew

**Mac App Store 应用。** 删掉应用包并不会取消购买。App Store 有「自动下载你在其他 Mac 电脑和设备上从 App Store 购买的 App」的设置，它针对其他设备上的购买行为，不代表另一台 Mac 仍装着某个应用，本机就一定会重新安装。遇到自动下载时，可以在 **App Store › 设置**里核对这一项。

**Homebrew cask。** 如果当初是用 `brew install --cask foo` 装的，把应用扔进废纸篓之后，Homebrew 仍然认为它装着。`brew list --cask` 里还会显示这个名字，下一次 `brew upgrade` 就可能把你刚删掉的应用装回来，这也是一种实打实的「它又回来了」，跟 launchd 没关系。改用 `brew uninstall --cask foo`。Homebrew 的手册把 `--zap` 描述为移除「与某个 cask 相关的所有文件」，同时警告它「可能会移除多个应用之间共享的文件」，所以这个参数要在想清楚之后再用。

## 删除成功之后，还会留下什么

**登录项与扩展里还列着它。** 要么是一条「登录时打开」记录，它指的那条路径已经不在了，要么是一条后台任务管理记录，它对应的那个服务程序已经没了。在**系统设置 > 通用 > 登录项与扩展**里用减号移除，macOS 也常常在重启后自己清掉。

**开机时冒出一个菜单栏图标。** 还有东西在加载某个二进制文件，通常是一个你没删掉的启动代理，或者安装程序复制到 `~/Library/Application Support/<vendor>` 里的一个助手。

**一个特权助手活了下来。** 需要 root 的软件会装到 `/Library/PrivilegedHelperTools/<label>`，配一份对应的 `/Library/LaunchDaemons/<label>.plist`。两者都归 root 所有，也都在应用包之外，所以把应用扔进废纸篓，哪个都碰不到。

```
ls -la ~/Library/LaunchAgents /Library/LaunchAgents /Library/LaunchDaemons \
  /Library/PrivilegedHelperTools 2>/dev/null
```

数据那一侧见[卸载之后的残留文件](https://mole.fit/zh/blog/how-to-remove-leftover-files-after-uninstalling-mac-apps)。

## 在 Mole 里做这套诊断

手动排查需要查看活动监视器、三个 launchd 目录、登录项与扩展、隐私与安全性，以及访达。[Mole](https://mole.fit/zh/) 把识别到的同一应用的相关项目放在同一页，而且扫描一直免费，可以先用它查看，不必立即购买或删除。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/en/uninstall.webp" width="2584" height="1741" loading="lazy" alt="一个卸载审核界面，某个应用被展开，显示它的应用包以及 ~/Library/Application Support 和 ~/Library/HTTPStorages 下的残留项，每一项带大小和复选框，底部是删除按钮。">
  <figcaption>这次删除会碰到的每一项，都在任何东西移动之前带着准确的路径和大小列出来。</figcaption>
</figure>

打开 Mole 的「软件」页，它分三段：已安装应用清单、可用更新和启动项。最后那一段，就是前面让你手动去拼的那份 launchd 与登录项清单，所以「它又回来了」的成因在删之前就看得见，不是删完才发现。

选中一个应用，Mole 会解析出它的 bundle 标识符，再去找这个身份拥有什么：Application Support、Caches、Preferences、Containers、HTTPStorages，还有启动代理和守护进程，前提是 plist 里确实引用了这个应用，而不是名字里凑巧有同一个词。每个候选项都带着路径、大小，以及把它关联进来的那份证据，低置信度的一律不勾选。

点确认之后，跑的就是这篇文章推荐的顺序，而且是强制执行不是靠记性。Mole 会退出这个应用以及嵌在它包里的助手，停掉它注册的登录项助手，在删除 plist 之前先用 `launchctl` 卸载每一个已批准的启动项，真正删除时再把每条路径校验一遍。删除进废纸篓，出了错是拖回来。

Mole 会按路径报告跳过项和失败项，帮助你判断是 App 管理权限、管理员认证，还是其他原因阻止了删除，不能只看成功总数。扫描与清理在本地执行，清理操作写入 `~/Library/Logs/mole/operations.log`，日志记录不代表文件现在仍保留在废纸篓。终端里，[Mole CLI](https://github.com/tw93/Mole) 免费开源，`mo uninstall` 支持 `--dry-run`，可以先读路径清单。

有三种成因 Mole 打不赢，它也不去硬碰：它不会关掉系统完整性保护，也不会从封存的系统卷上删掉 Apple 自带应用；在受管理的 Mac 上，配置描述文件要把软件装回来，它盖不过去；而对安全代理、VPN 客户端和虚拟化产品，它替代不了那个知道拆除顺序的厂商卸载器。App 管理被拒时，Mole 报告这次失败，不会绕过去提权。

## 常见问题

### 为什么应用删掉之后又回来了

先分清重新出现的是助手进程还是应用包。launchd 可以重新启动尚未移除的助手，但不会凭一份任务定义还原已删除的应用包。应用被重新安装时，再检查 Homebrew 的记录是否仍在、是否用过 `brew uninstall --cask`，以及 MDM 部署和 App Store 的其他设备购买自动下载设置。

### 关掉系统完整性保护就能删 Apple 自带应用吗

技术上可以，但这是笔亏本买卖：为了一个应用包，从恢复模式里把整台机器的安全等级降一级，而下次 macOS 更新替换掉封存的系统卷时，它还会回来。改成把它从程序坞和登录项里移除。

### 访达要我输密码才能删应用，是出问题了吗

不一定是问题。例如，`.pkg` 安装程序以 root 身份安装的应用，移动时可能需要认证。如果没有提示、没有报错却也没有删除，App 管理权限被拒是可能原因之一，还要结合实际错误、目录权限和进程状态判断。App 管理认的是发起请求的程序，不能靠 `sudo` 绕过。

## 延伸阅读

- [怎么彻底卸载 Mac 应用](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)，讲常规顺序和它会复核的各层资源库目录。
- [怎么卸载 Mac 上的杀毒软件](https://mole.fit/zh/blog/how-to-uninstall-antivirus-mac)，讲带系统扩展的软件，那类东西拆除顺序就是全部工作。
- [怎么关掉开机启动项](https://mole.fit/zh/blog/how-to-disable-startup-programs-on-mac)，讲没有东西跟你作对之后，launchd 和登录项这一侧该怎么弄。

---

Canonical HTML page: https://mole.fit/zh/blog/mac-app-wont-uninstall
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
