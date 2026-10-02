# 怎么清理 Mac 上 AI 编程工具留下的东西

> AI 编程工具可能留下构建产物、旧版命令行程序和对话记录。先测量并确认用途，再按恢复成本决定清理顺序，不只看体积。

Published: 2026-08-21 | Updated: 2026-09-30

一块磁盘用了两年都还宽裕，换成每天用编程 agent，几个月就能填满。agent 本身很小，变的是这台机器编译的频率，命令行工具在磁盘上自我替换的频率，以及你自己的思考过程有多少已经变成文本住进了主目录。增长几乎都落在三类东西上，这三类得分开判断，因为找回来的代价差得太远。

模型权重最容易被怀疑，但在这个问题上多半是冤枉了它。Ollama、LM Studio 和 Hugging Face 维护的是内容寻址仓库，只有它们自己的工具才能安全修剪，那部分单独讲在[清理 AI 工具残留](https://mole.fit/zh/blog/how-to-remove-ai-tool-leftovers-mac)。这篇讲的是 AI 编程 agent 干活的时候在磁盘上留下了什么。

## 动手删之前先量一遍

两条命令就能回答大半问题。第一条汇总各个 agent 的主目录：

```
du -sh ~/.codex ~/.claude ~/.local/share/claude ~/.grok ~/.cache 2>/dev/null | sort -h
```

第二条把构建产物找出来，它们散在打开过的每一个项目里，把根目录换成你放代码的地方：

```
find ~/www ~/Projects -maxdepth 3 -type d \
  \( -name target -o -name node_modules -o -name .next -o -name dist -o -name build \) \
  -prune -exec du -sh {} + 2>/dev/null | sort -h | tail -20
```

`-prune` 让 `find` 不再往已经匹配上的目录里钻，这一点在这里很关键，没有它，`find` 会把一个 24 GB 的 `target/` 里每个文件都走一遍才往下走。写这篇时我实测的那台 Mac 上，排最前面的两行，一个是 24 GB 的 Rust `target/`，另一个是某个 Tauri 项目里 9.8 GB 的 `target/`，各个 `node_modules` 树则在 134 MB 到 1.5 GB 之间。比例才是重点，看着像依赖闯的祸，实际只占真实数字的百分之二。

想看地图而不是列表的话，[Mole](https://mole.fit/zh/) 的「分析」把同一批卷画成矩形树图，可以直接钻进最大的那个方块，比猜该给 `find` 传哪个根目录快得多。

## 第一类：构建产物，只是被放大了

这一类不是新东西，新的是量。人手写代码一天编译几次，agent 做一个任务，几乎每改一次就编译一次，跑测试，换第二种做法，再编译一次。原本要几个月才长起来的缓存，现在一个下午就长出来了，而增量构建目录本来就是用磁盘换速度的设计。

**Rust** 通常大出一大截。一个 `target/` 目录装着编译好的依赖、增量编译状态和构建脚本输出，还按 profile 分开存，所以 debug 和 release 是两份完整副本。不带参数的 `cargo clean` 「会删掉整个 target 目录」，先预览一下：

```
cargo clean --dry-run
cargo clean --release
```

`cargo clean -p <package>` 只清指定的包，问题出在工作区某一个成员上时，用它才对。

**JavaScript** 的产物摊得更薄。除了 `node_modules` 本身，还有打包器和转译器用的 `node_modules/.cache`、Next.js 构建用的 `.next`，以及工具链自己写出来的 `dist` 或 `build`。专门找那些缓存：

```
find ~/www -maxdepth 4 -type d -path '*/node_modules/.cache' -prune -exec du -sh {} + \
  2>/dev/null | sort -h | tail
```

**Python** 会在它导入的每个包旁边留下 `__pycache__`。单个极小，但有成千上万个。删之前先数一遍，同样一条命令末尾换成 `rm -rf`，根目录写错一个字就没得商量：

```
find ~/www -type d -name __pycache__ -prune -print | wc -l
```

源码还在时，字节码缓存会在下次导入时重新生成，不需要联网。先停下相关任务，再确认目录里确实只有缓存。

**Go** 维护的是一份全局构建缓存，不是每个项目一个目录。`go env GOCACHE` 打印它的位置，`go clean -cache` 「会让 clean 移除整个 go 构建缓存」，`go clean -testcache` 让缓存的测试结果过期，但不丢弃已编译的包。我这台机器上构建缓存有 183 MB，模块缓存只有 38 MB，所以哪怕下载的模块不大，构建那一侧也值得看看。

**Xcode** 要单独处理，因为 DerivedData、归档、设备支持文件和模拟器运行时是四种东西，恢复代价各不相同。这台 Mac 上的 DerivedData 目录量出来是 9.3 GB。[清理 Xcode 占用](https://mole.fit/zh/blog/how-to-clean-up-xcode-mac)讲了其中哪些能删、哪些要留着做符号化。Gradle 和 Maven 的分法一样，一边是各项目自己的 `build/` 目录，一边是 `~/.gradle` 或 `~/.m2` 下的全局仓库，全局那一侧和其他各种注册表一起归到[清理开发缓存](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)。

## 第二类：被替换掉的旧版命令行

这一类几乎没人会去找，机器上跑着好几个 agent 的话，它经常比所有缓存加起来还大。

agent 命令行的自更新方式，是下载一份完整的新版本发布包，再把启动器指过去。每个发布版都是自包含的，和上一版不共享任何文件，指针挪了，旧的那份留在原地，没有任何东西会去扫它，所以每更新一次数量就加一，一直加下去。

各家的布局是同一个模式，只是形式上有点差别：

- **Codex** 放在 `~/.codex/packages/standalone/releases/<version>-<arch>/`，上一层有个 `current` 软链接指向正在用的那个。
- **Claude Code** 放在 `~/.local/share/claude/versions/<version>`，每一项是单个可执行文件而不是目录，`~/.local/bin/claude` 是指向当前版本的软链接。
- **Grok** 以文件形式放在 `~/.grok/downloads/grok-<version>-macos-<arch>`，`~/.grok/bin/grok` 和 `~/.grok/bin/agent` 指向当前构建。
- **Cursor Agent** 放在 `~/.local/share/cursor-agent/versions/<date>-<sha>/`，启动器是 `~/.local/bin/cursor-agent`。
- **GitHub Copilot CLI** 用 npm 装的话是原地替换自己，但它的安装脚本会在一个前缀下写入带版本号的包，非 root 用户默认是 `$HOME/.local`。Mole 会去 `~/.copilot/pkg/universal` 按同样的结构查一遍。

一次全量出来：

```
du -sh ~/.codex/packages/standalone/releases/* \
       ~/.local/share/claude/versions/* \
       ~/.grok/downloads/* \
       ~/.local/share/cursor-agent/versions/* 2>/dev/null
```

写这篇用的那台 Mac 上，它打印出五个 Codex 发布版，每个 262 MB 到 310 MB，五个 Claude Code 可执行文件，每个 293 MB 到 306 MB，还有两个 Grok 构建和两个 Cursor Agent 版本，加起来大约 3.5 GB，其中在用的约 920 MB，其余全是已经被替换掉的二进制。Codex 自己的问题追踪器上还开着一条相关请求，报告者量出来每更新一次大约多 250 MB（[openai/codex#22293](https://github.com/openai/codex/issues/22293)）。

### 删任何一个目录之前，先把启动器解析开

诱人的捷径是按日期排序、留最新的那个，别这么做。两种很平常的情况会让它翻车，一是碰上一次回归，你有意钉在了旧版本上，二是一次更新已经把新目录放好，指针还没翻过去。删掉正在用的那个发布版，留下的是一个指向空处的启动器。

去问启动器本身。它是软链接，解析它：

```
readlink -f "$(command -v codex)"
readlink -f "$(command -v claude)"
readlink -f "$(command -v grok)"
```

它会打印出真实目标，比如 `~/.codex/packages/standalone/releases/0.147.0-aarch64-apple-darwin/bin/codex`，而 `ls -l "$(command -v codex)"` 显示的是链接指向，不一定是最终目标。这个结果只能确认当前启动器指向哪一版，不能证明其余同级目录都已闲置，还要排除其他进程正在使用、更新尚未完成的情况。确认不用的版本再移入废纸篓，拿不准就保留。移走后先试一次这个命令行工具，确认正常运行，不急着清空废纸篓。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/agent-cli-release-pin.webp" width="1360" height="454" loading="lazy" alt="PATH 上的启动器软链接经由 current 指针指向一个发布版目录，旁边的同级版本还需检查是否被其他进程或更新任务使用。">
  <figcaption>认出在用版本的是启动器，不是时间戳。有意钉住的降级，或者一次做到一半的更新，都会让最新那个目录变成错的答案。</figcaption>
</figure>

## 第三类：agent 的工作状态，这不是垃圾

第三类才是清理器真能造成实质损失的地方，偏偏它看起来和前两类一模一样。

会话记录、记忆、计划和生成的附件住在 `~/.codex/sessions`、`~/.codex/archived_sessions`、`~/.codex/memories`、`~/.claude/projects` 和 `~/.grok/sessions` 下。文件是 JSONL，按会话 id 命名，带时间戳，只追加，而且一直在长。通用清理器的每一条启发式规则都会判它是日志文件。我实测的那台 Mac 上，`~/.codex/sessions` 有 9.8 GB，`~/.claude/projects` 是 2,362 个记录文件共 2.7 GB，`~/.grok/sessions` 有 1.3 GB，数字又大又诱人，底下挂着一堆看上去随手就能丢的文件。

它们不是日志。一份记录写下的是一处改动怎么想出来的，试过哪几种做法又被否掉，是哪条约束否掉了其中一种，最后为什么长成这个样子。这些推理别处都不存在，提交信息记的是改了什么，代码记的是活下来的那个方案，不记被丢掉的另外四个。它们几个月几个月地悄悄堆着，等你第一次回头问「当初为什么要这么做」，才知道它值钱。

更大的风险其实不在第三方清理器。Claude Code 自带保留期清理，`cleanupPeriodDays` 默认 30 天，启动时会把超过天数的东西删掉，`projects/` 下的记录、计划文件、`file-history/` 里编辑前的快照，还有每个会话的任务列表。它的 [.claude 目录参考](https://code.claude.com/docs/en/claude-directory)写明了哪些路径会被清、哪些无限期保留。想留一年的记录，现在就把这个数字调大，别等出了事才回头去看默认值。反过来的情况同一页也写了，确实想彻底清掉某个项目的状态时，用 `claude project purge`。

[Mole](https://mole.fit/zh/) 的「清理」一样都不碰。这五条路径，加上 `~/.claude/file-history`、`~/.claude/plans`、`~/.claude/tasks`、`~/.codex/attachments` 和 `~/.codex/generated_images`，不管多旧都在它的保护清单上。没有年龄门槛，没有「超过 90 天」的例外，也没有开关能开出一个来。按年龄放行这几条路径的做法试过一次，当天就撤了，一份旧的对话记录不等于一份过期的对话记录。

旧会话另有一条要你亲手走的路。清理页远处的月亮通向 AI 清理与维护，那里只列超过你选定保留期的 Codex 会话，以及项目文件夹已经不在的 Claude Code 会话，每一条都带标题和时间，你勾上之前一条都不会动，Codex 的会话会先复制一份到废纸篓，再交给 Codex 自己的命令删除，它的索引不会乱，记忆、计划和技能文件在那里也不在任何清单上。

## 底层逻辑：按恢复代价排序，不按体积排序

这条规则让三个类别都变得能判断，也是整篇文章里唯一值得背下来的东西。给候选排序，看的是把它找回来要付出什么，不是它显示了多少 GB。

**本地可再生。** 构建产物、增量编译状态、字节码缓存、DerivedData。源码、依赖和工具链齐全时，通常能在本机重新生成，代价主要是重新构建的时间。先停下构建、测试和开发服务，确认没有混入需要保留的输出，再清理。

**重建代价高。** 各种包注册表、`node_modules`、CocoaPods、Python 虚拟环境、`vendor` 目录、模型权重、iOS DeviceSupport。每一项都要联网，要注册表还提供着锁文件里写死的那些版本，有时还要一套原生工具链。真实代价不是网络好的时候那几分钟，而是你在火车上还能不能干活。这些要一项一项审。

**不可替代。** 聊天记录、agent 的记忆、计划文件、项目状态、本地微调模型。多少 CPU 和带宽都换不回来。它们永远不该出现在批量删除里，也不该做成能被顺手勾上的选项。

陷阱在于第一档和第二档看起来一模一样。`target/` 和 `node_modules/` 都是项目根目录下的大目录，都塞满依赖产物，都写在 `.gitignore` 里，都能用一条命令重新生成，按体积排序还紧挨着。但 `cargo build` 是拿磁盘上已有的源码重建 `target/`，`npm ci` 要注册表还活着、锁文件还解析得出来。一个是一杯咖啡的工夫，另一个是耽误一下午，或者碰上一个被撤回的包，再也装不回来。把这两者混为一谈是这一类里最常见的错误，「先删最大的那几个文件夹」之所以是坏建议，就坏在这儿，哪怕它确实释放得最多。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/restore-cost-tiers.webp" width="1360" height="454" loading="lazy" alt="按恢复代价排出的三档，target 和 node_modules 画成看起来完全一样的项目目录，却落在不同档里，因为一个从本地源码重建，另一个需要注册表。">
  <figcaption>体积给出的排序是错的。项目根目录下两个长得一样的目录，恢复代价可能差出整整一个工作日。</figcaption>
</figure>

## 在 Mole 里怎么做

手动路线可行，也不花钱。[Mole](https://mole.fit/zh/) 多出来的是把三个类别装进同一份可审的清单，档位边界也替你划好了，不用自己去记哪个点开头的目录里装着对话记录。

打开「清理」跑一次扫描，扫描免费，也不需要授权。每个候选项都写清路径、归属和量出来的大小，你确认清单之前什么都不会动。低置信度的一律不勾选，默认动作永远是更小的那个。缓存默认永久删除，也可在设置中改为移到废纸篓，文件仍在废纸篓中时可恢复，批量操作也会报告跳过与失败的项目。

具体到那些被替换掉的旧版命令行，上面那套启动器解析，Mole 替你做了。它读每个 agent 命令行的启动器软链接，解析到正在用的那个发布版，再把这个版本排除出候选集，所以一次有意的降级会被完整保留，不会被当成旧版本。这个行为就是从 Codex 上量出来的，五个发布版共 1.2 GB，只有一个在用。

[Mole CLI](https://github.com/tw93/Mole) 免费开源，可用 `mo clean` 在终端里清理；清理、卸载和优化命令支持 `--dry-run`，动手前可以先看候选清单。两者共享 `~/.config/mole/whitelist` 这份用户保护清单，并使用 `~/Library/Logs/mole/operations.log` 记录操作，全部在本地跑，不上传也没有遥测。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/mole-clean-review.webp" width="2584" height="1741" loading="lazy" alt="Mole 清理审查页按类别列出缓存和各自大小，每类都有勾选框，底部是永久清理已选 5.14 GB 的按钮。">
  <figcaption>发现和删除是分开的两步，审查页按类别列出扫描找到的内容，你确认之前什么都不会删。</figcaption>
</figure>

有必要说白，Mole 不是备份，不是恶意软件处置，也替代不了软件自带的卸载器，尤其是装了驱动或系统扩展的那些。它不删模型权重，也不删 AI 聊天记录，根本不会提议去删，这些交给管它们的工具。

## 怎么让它别再长回来

三处配置改动就能覆盖大部分反弹。

**让 Rust 只用一个构建目录。** `CARGO_TARGET_DIR` 设定「放置所有生成产物的位置」，于是每个项目都写进同一棵树，量和清都只在一个地方做。代价也是实打实的，Cargo 会锁住构建目录，两个项目共用一个 target 目录，就只能一个一个构建，没法并行。经常同时跑多个构建的话，就让它们各自分开，改成定期清扫。

**全局仓库交给定期策略去修剪，别手动删。** 现在的 Cargo 跑正常的构建和拉取命令时，就会顺手把全局缓存里不再用的条目移掉，npm 也把自己的缓存描述为可自愈的，维护命令是 `npm cache verify`。让这些策略去跑，别自己上手删主目录里那些点开头的目录。[清理开发缓存](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)里有各工具对应的命令。

**确认 agent 命令行会不会自己修剪旧发布版，默认按不会来。** 写这篇的时候，Codex 和 Claude Code 我都没找到哪个参数或配置项写在文档里，能清掉被替换下来的发布版二进制，Codex 那条请求也还开着。Claude Code 的 `cleanupPeriodDays` 清的是会话数据，不是版本二进制，这件事上帮不上忙。在这一点变之前，它是一件周期性的活，也是清单里价值最高的一项，它按 agent 的发版节奏长回来。

## 常见问题

### AI 编程工具到底占多少磁盘

单个二进制不过几百 MB，真正要紧的是累积。这篇文章实测的那台机器上，四个 agent 命令行的版本目录加起来约 3.5 GB，其中在用的只有 920 MB，会话记录约 14 GB，单个 Rust `target/` 目录就有 24 GB。这些数字受语言的影响比受 agent 的影响大得多，所以跑一遍文章开头那两条 `du` 命令，别信任何人给的数字，包括这一份。

### 删掉旧版 Claude Code 或 Codex 安全吗

先确认哪些版本确实不用了。跑 `readlink -f "$(command -v claude)"`，或者换成对应工具的命令，保留它指向的版本，再检查其他版本是否仍被进程或更新任务使用。只移走确认闲置的版本，不要单凭日期保留最新的。移入废纸篓后先确认工具能正常运行，暂时保留恢复余地。

### Mac 清理工具会删掉我的 agent 聊天记录吗

有些会，那些文件看起来就是日志，这正是这一类特有的风险。Mole 从不碰 `~/.codex/sessions`、`~/.codex/archived_sessions`、`~/.codex/memories`、`~/.claude/projects` 和 `~/.grok/sessions`，不管多旧都不碰。跑任何清理器之前，先看这些路径有没有出现在它的候选清单里，要是这个工具动手之前根本不给你看清单，那本身就是答案。

### 清掉构建缓存会让什么变慢吗

通常会让下一次构建变慢，缓存重新生成后会逐渐恢复。前提是源码、依赖和工具链都还在，而且已经停止相关构建与测试。恢复代价低不等于可以边用边删；需要重新下载依赖的 `node_modules` 还要考虑网络和包仓库是否可用。

### Ollama 模型和 Hugging Face 缓存怎么办

这里不管，而且是有意不管。那些工具用的是内容寻址的仓库，两个模型可能共享同一个数据块，手动删文件，会让另一个还引用着这些块的模型断掉。用各自工具自己的删除命令，讲在[清理 AI 工具残留](https://mole.fit/zh/blog/how-to-remove-ai-tool-leftovers-mac)。

## 接下来看什么

按恢复代价排序，删发布版之前先解析启动器，对话记录一律别动。量出来最大的那一行如果是某个包仓库，[清理开发缓存](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)有各工具对应的修剪命令。如果是 Xcode，[清理 Xcode 占用](https://mole.fit/zh/blog/how-to-clean-up-xcode-mac)把可重建的目录和该留的归档分开了。如果是模型仓库，[清理 AI 工具残留](https://mole.fit/zh/blog/how-to-remove-ai-tool-leftovers-mac)解释了为什么删除这一步必须交给管它的那个工具。如果要整个卸掉桌面应用，[卸载 Claude 桌面版](https://mole.fit/zh/tested-apps/claude)和[卸载 Cursor](https://mole.fit/zh/tested-apps/cursor)两篇指南会把 Claude Code 用的 `~/.claude` 和单独安装的 Cursor Agent CLI 留在卸载范围之外。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clean-up-ai-coding-tools-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
