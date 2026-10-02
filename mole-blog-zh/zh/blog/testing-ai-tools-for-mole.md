# 给 Mole 做 AI 工具的清理与维护

> 从舍不得清理 AI 会话，到自己也攒下了旧会话和 worktree，记录实际使用、卸载 AI 工具的过程，以及这次想让 Mole 继续坚持的克制。

Published: 2026-09-29 | Updated: 2026-10-01

利用这个假期，想着继续给大伙上一个 Mole 有用的功能，帮大家清理不再需要的 AI 缓存、旧会话和 worktree，卸载之前装过、很久没有用的 AI 工具。

之前在[测试几百款 Mac 软件的卸载残留](https://mole.fit/zh/blog/testing-mac-apps-one-by-one)时，也有非常多的小伙伴提过 AI 垃圾清理的诉求，当时这些“垃圾”在我看来是有用的，比如你的全部 AI Coding 会话记录，我认为是非常重要的资产，所以这一块之前更多只是支持清理真正的缓存和旧版本。

不过随着 AI 的使用，我本地机器也有非常多没有那么大用的 AI 产物、会话以及 worktree 一直留在本地，还是非常有必要做这个事情，于是就开始弄了。也有用户的 `opencode.db` 一个文件就涨到了 62 GB，问该怎么清，这次就把桌面端和命令行工具都装上，登录账号，实际用过以后再查它们留下的东西。

## 这次是怎么测的

和上次只装、只卸不一样，这次能用上的工具，我都登录了，在自己的项目里真的用了一次，问问题、跑 Agent、建代码索引、设定时任务，因为很多数据只有用过才会出现，只装不开是看不到的。每装一个，AI 就对照装之前的状态，量它往家目录、`~/Library`、`/usr/local/bin` 和 shell 配置里写了什么，记进一份台账；然后看 Mole 状态页里它占了多少进程和内存，最后用 Mole 卸载，卸完再按名字把整个家目录扫一遍，找 Mole 没有列出来的东西，缺什么当晚补进代码，再复测一次。

桌面端测了 13 个：Devin、OpenCode、Trae、Trae CN、TRAE SOLO CN、Kiro、Qoder、QoderWork、Qoder CN、通义灵码、CodeBuddy CN、WorkBuddy 和豆包工作，其中 12 个真的用上了，灵码登录以后没有额度，只量了它写的文件。命令行这边新装并运行了 Amp、opencode、Kimi CLI、Kimi Code、Factory Droid、Copilot CLI、Gemini CLI、Qwen Code 和 iFlow，加上本机已有的 Claude Code、Codex、Grok 和 Cursor Agent，一共 13 个，另外还参考 CC Switch 支持的名单，量了 OpenClaw、Hermes 和 Pi 卸载以后留下的目录。用下来 Amp 做得还是挺不错的，非常有自己的调性，Qoder CN 应该是我当天测试过的最好的一个国内 AI 产品，整体还是比较简洁、比较清晰的。也有几个没能用上，这次没能用个人 Google 账号登录 Gemini CLI，Qwen Code 和 iFlow 都要自己准备接口和 Key。

## 装上用一次，才知道它们写了什么

最直观的是内存，13 个桌面端同时开着一共占了大约 23 GB，CodeBuddy CN 一个就是 41 个进程、4.49 GB。这些大多是 VS Code 或 Electron 系的应用，一个窗口背后是一串 helper，所以状态页里我让每个 App 只占一行，把它的所有子进程合在一起算，不然光看进程列表，很难知道到底是谁在吃内存。

更意外的是它们对系统做的小改动。Devin、CodeBuddy、Kiro、Kimi Code 都会往 `~/.zshrc` 里加 PATH 或终端集成的行，Kiro、Trae 和 Trae CN 第一次打开时，会往 `/usr/local/bin` 放一个 root 所有的命令链接，App 删掉以后这个链接就变成了指向空处的死链接。WorkBuddy 往 `~/.local/bin` 放了一个 `python3.12`，之后在终端里敲 `python3.12` 用的就是它带的那一份，还装了 127 MB 的 uv Python，我当时的原话是这个软件比我想的还要流氓。最有意思的是 Kimi CLI，我运行了一次，它先告诉我自己已经停止维护，然后没有问我，就直接下载安装了新的 Kimi Code，改了 `.zshrc`，还把原来的命令改名成了 `kimi-legacy`。

体积上，TRAE SOLO 第一次用 Work 模式会下载约 1 GB 的工具包，解压以后是 3.1 GB，里面是一整套自带的 LibreOffice、FFmpeg、OpenJDK；六个桌面工具每个只做了一两个任务，就一共写了 5.2 GB 的数据。我自己天天用的 `~/.codex` 已经 24 GB，其中会话占了 17 GB，Claude 桌面版 14 GB 里有 12 GB 是 Cowork 用的 Linux 虚拟机。opencode 的数据库是按事件记录的，一次提问写进去的内容是消息正文的三倍多，删掉会话以后也不会自己压缩，这大概就是有人会涨到 62 GB 的原因。

还有一个坑是 skills。很多 skill 安装器会往每一个 agent 的目录里都写一份 `skills/`，不管你装没装这个 agent，所以我的电脑上明明没用过 Qwen Code、iFlow 和 Droid，`~/.qwen`、`~/.iflow`、`~/.factory` 却都在，Mole 的第一版还把它们当成了 2.4 MB 的工具，差点把我装的 skills 当残留一起清掉。

## 卸掉以后，还有东西留着

第一轮 14 个 App 卸完再扫，家目录里还有十几个点目录 Mole 没有列出来，像 `~/.kiro`、`~/.qoder`、`~/.qodersec`、`~/.lingma`、`~/.codebuddy`，还有 Kiro 放在 `~/.aws` 里的登录令牌文件。它们不在 `~/Library` 下面，名字也不一定和 App 一样，只能装一个查一个。有些目录还是几个产品共用的，Trae CN 和 TRAE SOLO CN 共用 `~/.trae-cn`，Qoder 和 QoderWork 共用 `~/.qoder`，只卸其中一个的时候，另一个还要用，这时候就不能列出来。测国际版 Trae 时还发现它会把 Trae CN 的目录认成自己的，缓存还是默认勾选的，改完以后列表从 33 行变成 14 行。

另一个没想到的是进程。卸载完第二天早上再查，`~/.kimi-code` 和 `~/.factory` 删了又长了回来，原因是前一晚在终端里开着的 `kimi`、`droid` 会话进程还活着，程序文件已经在废纸篓里了，进程照样在往目录里写日志；接着又发现 `gemini`、`qwen`、`iflow`、`amp` 的会话也都还在跑。命令行工具没有窗口，关掉终端标签也不一定会结束它，卸载之前最好先确认它不在运行。

这个过程也发现了，还是 Claude、Codex、Cursor 这类产品在克制上保持得比较好，也是我常用的 3 个，测试过程中发现了非常多软件的乱象，比如下载安装包的时候，默认在你的剪贴板写一点东西，用于跟踪安装渠道、做数据统计，这种我感觉有点过了，也有安装后，给你弹窗让你去做活动获取 Token 的，牛皮癣满天飞，有一种捏着鼻子用的感觉。

但是整个过程中让我感觉不错的是居然 Qoder，保持住了自己的克制，审美在线，没有乱来，产品上挺让人惊讶，期待国产多一些这样的软件，加油加油。

## 你也可以自己先看一眼

这些检查不需要 Mole，在终端里就能看，下面的命令都只读，不会改任何东西。

先看 shell 配置里，有没有 AI 工具加的行：

```bash
grep -nE "codeium|codebuddy|kiro|kimi|lmstudio|\.local/bin" ~/.zshrc ~/.bashrc ~/.zprofile 2>/dev/null
```

每一行前面是文件名和行号，如果某个工具已经卸掉了，对应的行可以用文本编辑器删掉，改之前最好先复制一份这个文件，改完新开一个终端窗口确认一切正常。

再看 `/usr/local/bin` 里指向 App 的命令，以及已经失效的链接：

```bash
ls -l /usr/local/bin | grep "\.app/"
find /usr/local/bin -maxdepth 1 -type l ! -exec test -e {} \; -print
```

第一条列出指向某个 App 内部的命令，第二条列出目标已经不在的死链接，这些大多是 App 卸掉以后留下的，属于 root，删除需要管理员密码。

看看还有没有已经卸掉的命令行 agent 在后台跑：

```bash
ps -axo pid,lstart,command | grep -E "kimi|droid|gemini|qwen|iflow|amp|opencode" | grep -v grep
```

有输出就说明还有会话活着，先在它所在的终端里退出，找不到窗口的话，可以用 `kill` 加上进程号结束它。

最后量一下这些工具各占了多少：

```bash
du -sh ~/.codex ~/.claude ~/.grok ~/.gemini ~/.cursor ~/.local/share/opencode 2>/dev/null
```

这里面有你的会话记录，删之前先想清楚还要不要回头看，AI 的对话一旦删掉是找不回来的。

## Mole 现在是怎么做的

这次做的 AI 页目前在 Preview 版本，还需要继续测试，下一个版本再放出来给大伙使用。AI 页的入口是清理页远处的一轮月亮，只有检测到 AI 工具的数据时才会出现，不想要的话在设置里可以隐藏。点进去以后，它会按工具一个个梳理，把结果按删了会不会后悔分成三段：缓存和旧版本删了会自己重建，默认勾选；会话和工作区删了就回不来了，需要你自己确认；很久没用的工具和维护项放在最后，也不默认勾选。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/testing-ai-tools-review.webp" width="2400" height="1800" loading="lazy" alt="Mole 的 AI 清理与维护页，会话与工作区里展开了 6 条超过 90 天的 Codex 旧会话，下面是已完成的 worktree 和项目已删除的会话，每一行写明是什么、删了会怎样和大小。">
  <figcaption>AI 清理页的真实界面，截图里的数据来自一个样例目录，不是我的电脑。</figcaption>
</figure>

会话这一段里，Codex 超过保留期的旧会话可以按 15、30、90、120 天筛选，展开以后每一条都带标题和时间，也能单独取消，Mole 会先把会话文件复制一份到废纸篓，再调用 Codex 自己的删除命令，这样 Codex 的索引不会乱；Claude Code 只列出项目文件夹已经不在的会话；用完的 worktree 只有在 git 能证明里面没有未提交的改动时才会出现，已经合进主分支的会写出来。

闲置工具这一段，是这次花时间最多的部分。Mole 会在 npm、pipx、uv、官方安装器和 Homebrew 的固定位置里找 AI 命令行工具，30 天没用、也没有在运行的，才会列出来，卸载的时候把程序、指向它的命令链接和它的设置、会话、登录信息一起处理掉。别的软件也在用的数据会留下，比如 Antigravity 也在 `~/.gemini` 里放东西，opencode 的桌面版和命令行共用一个会话库；闲置工具超过 4 个时，最大的 3 个各占一行，其余的合成一行。Claude Code 和 Codex 不在这个名单里，因为它们的目录也是桌面版在用的。

用 `sudo npm` 装进系统目录的命令行工具也会列出来，卸载时需要输一次管理员密码。opencode 那个越用越大的 `opencode.db`，删掉会话以后空出来的地方只要超过一成、文件又超过 200 MB，Mole 会在这一段里给出压缩，不删任何会话，只把空间还给磁盘，压缩前要先退出 opencode。

卸载那边，上面提到的点目录现在都会作为待确认的行列出来，默认不勾，几个产品共用的目录，只要另一个还装着就不列；`/usr/local/bin` 里那种 root 所有的命令链接，也会在卸载时一起列出来，通过 Mole 的管理员助手移进废纸篓。除了 Homebrew 装的程序要走 Homebrew 自己的卸载命令，其他所有东西都是进废纸篓，不是直接删掉，删错了还能拖回来。

这些改动会跟着 Mole 的下一个版本一起发出来。

## 还没有做完的

有些事情这次还没做，比如 shell 配置里 AI 工具加的那几行，Mole 不会去改你的配置文件，打算放在卸载的详情里告诉你是哪几行；Cursor、Conductor 这类工具的 worktree，我电脑上还没有真实的数据，暂时没有加。opencode 和 Devin 的旧会话，现在还不能在 Mole 里按天数单独清理。MiniMax 这次没有实测，暂时没有支持。

加上之前测试那 700 多款软件的过程，发现了还是有不少软件没有遵守住一些软件工程开发的基本原则，也没有大家想的那么专业、纯净。更加让我想着 Mole 一定要保持住现有的简洁、把能力做深、不乱搞、不乱获取不该获取的东西，不上传用户的本地文件和会话内容，甚至任何使用统计都不加，继续保持强迫症的方式走下去应该会更加长远。

最后，大伙有任何对于 AI 清理和维护的诉求，欢迎到[讨论帖](https://github.com/tw93/Mole/discussions/1604)告诉我，我给放到测试里面去做，确认适合清理的就加到这个功能里面去，这一块应该会持续迭代维护很长时间。已经测过的软件都在[实测软件名单](https://mole.fit/zh/tested-apps)里，想自己动手清理的话，也可以看看[清理 AI 编程工具](https://mole.fit/zh/blog/how-to-clean-up-ai-coding-tools-mac)那一篇。

---

Canonical HTML page: https://mole.fit/zh/blog/testing-ai-tools-for-mole
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
