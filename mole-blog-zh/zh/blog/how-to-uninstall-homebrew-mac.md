# 用官方脚本卸载 Homebrew，并先导出清单

> 先用 --dry-run 跑官方卸载脚本，导出 formula 与 cask 清单，再清理 shell PATH 残留与失败情形

Published: 2026-08-04 | Updated: 2026-09-25

Homebrew 自带官方卸载脚本，行为与说明一致。真正要留意的是范围：它会卸掉 Homebrew，也会卸掉经它装上的一切，这两件事的分量，取决于你为什么来到这里。

如果只是想腾出空间，先看看 Homebrew 实际占了多少再决定要不要卸。如果只是某个 formula 出了问题，不必把整套环境一起拆掉。

动手前先记下 `brew leaves` 和 cask 列表，再用 `--dry-run` 跑一遍官方卸载脚本，把输出当成删除计划来读。如果问题只在某一个软件包，到这里就该停下来，改用 `brew uninstall`，或走[更安全的 Homebrew 清理路径](https://mole.fit/zh/blog/how-to-clean-up-homebrew-mac)。

## 什么时候不该卸载 Homebrew

下面这些情况，先别上「核选项」：

- 只是某一个 formula 或 cask 坏了：用 `brew uninstall <name>`，依赖成孤儿后再跑 `brew autoremove`。
- 只是缺空间：`brew cleanup -s` 能清掉旧版本和下载缓存，不会删掉安装前缀。
- 你还需要来自 Homebrew 的 `git`、`node`、`python`、数据库服务或命令行工具。卸载脚本会连 Cellar 和 Caskroom 一起带走。
- 你打算明天在同一台 Mac 上重装同一套工具。先导出下面的清单，用 `brew cleanup` 清理，保留安装本身。

真正该卸载的时候，是你准备彻底离开 Homebrew、在坏掉的安装之后迁到干净前缀，或者清掉一台不再需要那些软件包的机器。

## 先认清前缀：Intel、Apple Silicon 与 Rosetta 残留

Homebrew 的默认前缀取决于芯片：

| Mac | 默认前缀 | `brew` 二进制 |
| --- | --- | --- |
| Apple Silicon | `/opt/homebrew` | `/opt/homebrew/bin/brew` |
| Intel | `/usr/local` | `/usr/local/bin/brew` |

删任何东西之前，先确认自己真正用的是哪一套：

```
uname -m
brew --prefix
which brew
```

`arm64` 是 Apple Silicon。`x86_64` 是 Intel（或 Rosetta 终端）。从 Intel 迁过来的 Apple Silicon Mac 上，`/usr/local` 下的旧 Intel Homebrew 可能和 `/opt/homebrew` 并存。官方卸载脚本会找当前机器的默认前缀；在 `arm64` 上也会考虑 `/usr/local`，以免漏掉残留的 Intel 前缀。如果你有意保留两套前缀，请传 `--path`，只卸你指定的那一套。

在 Intel 上不要整棵手删 `/usr/local`。那个目录还和其他软件共用。用官方脚本，它只动 Homebrew 自己的树。

## 官方卸载脚本

Homebrew 的 [FAQ](https://docs.brew.sh/FAQ#how-do-i-uninstall-homebrew) 指向 Homebrew/install 仓库里的卸载脚本。若 Homebrew 日后改了命令，优先从那一页复制。当前的一行命令是：

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh)"
```

想先读脚本再跑，可以先下载：

```
curl -O https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh
less uninstall.sh
/bin/bash uninstall.sh --help
```

常用参数：

- `--dry-run`（也可写 `-n`）：只打印将删除的内容，不做改动。在你在意的机器上，第一次调用就该从这里开始。
- `--path=PATH`：指定前缀（例如 Apple Silicon 上遗留的 Rosetta 时代 `/usr/local`）。
- `--force`：跳过确认提示（设置了 `NONINTERACTIVE` 时也会隐含启用）。
- `--skip-cache-and-logs`：不动缓存和日志目录。

别理旧的 mxcl gist，也别跟那些还在写 `cd $(brew --prefix)` 再用 `git ls-files` 删文件的教程。那套流程面向早期 `/usr/local` 布局，在现在的安装上不安全。

## 脚本实际会删掉什么

卸载器会移除 Homebrew 安装前缀及其下的目录：`bin`、`etc`、`include`、`lib`、`opt`、`sbin`、`share`、`var`、`Frameworks`，以及 `Cellar` 与 `Caskroom`。

后两者才是关键。Cellar 里是每个 formula 的文件，Caskroom 里是 cask 元数据。卸掉 Homebrew 等于一并卸掉它们，经它装上的命令行工具会同时消失。

它还会清理 `~/Library/Caches/Homebrew` 与 `~/Library/Logs/Homebrew`、系统级 Homebrew 缓存、`/etc/paths.d/homebrew`，以及 `/Applications` 与 `~/Applications` 里由 Homebrew 管理的应用。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/homebrew-uninstall-scope.webp" width="1360" height="454" loading="lazy" alt="卸载脚本会移除含 Cellar 与 Caskroom 的 Homebrew 前缀，以及缓存和日志；软件包自己写入的配置和 shell 配置行会留下。">
  <figcaption>脚本带走的是软件包，不只是包管理器。留下的是那些软件包写过的配置，以及你自己加进 shell 配置的行。</figcaption>
</figure>

## 动手前先记下已装了什么

卸载前先用这两条命令导出清单：

```
brew leaves > ~/Desktop/brew-formulae.txt
brew list --cask > ~/Desktop/brew-casks.txt
```

`brew leaves` 列出没有被其他已装 formula 或 cask 依赖的 formula。它是有用的起点，但不是你曾经显式安装过的完整记录。拿去重建环境前，先通读这两个文件。

软件包数据和软件包文件要分开看。家目录里的配置可能还在，但 `/opt/homebrew/var` 或 `/usr/local/var` 下的 Postgres 数据库仍在 Homebrew 前缀里。卸载脚本会删掉 `var`，那里的数据可能一起走。需要留下的数据，先备份到前缀之外。若在用服务，先停掉：

```
brew services list
brew services stop --all
```

## 可选：先卸 formulae 和 casks

如果你想分阶段拆，而不是一次核删，可以在 `brew` 还能用时先卸软件包：

```
brew list --formula
brew uninstall --force $(brew list --formula)
brew list --cask
brew uninstall --cask --force $(brew list --cask)
brew autoremove
brew cleanup -s
```

这样会清空 Cellar 和 Caskroom，但保留 Homebrew 安装本身。适合你还不确定 `var` 下有没有要留的数据，或只想腾出软件包占用、稍后用更小的工具集继续留着 `brew`。真正告别 Homebrew 时，仍要跑官方卸载脚本，把前缀、tap 和缓存一并清干净。

## 执行卸载

1. 先带 `--dry-run` 跑一遍，逐条核对路径。
2. 去掉 `--dry-run` 再跑，提示时输入 `y`；若受保护目录需要 `sudo`，再输入 macOS 密码。
3. 终端若打印 “possible Homebrew files were not deleted” 一类列表，关掉窗口前先复制下来。

下载脚本后的预览示例：

```
/bin/bash uninstall.sh --dry-run
```

若用远程一行命令，也建议先下载再传参数，避免引号套引号，然后执行 `/bin/bash uninstall.sh`。

## 残留文件核对清单

脚本结束时可能提示：部分可能属于 Homebrew 的文件未能删除，建议你自行处理，并附上路径列表。

这些路径通常只在这里出现一次。常见原因是 Homebrew 自己没创建过的目录权限不对，列表一般不长。关掉终端前先另存一份，因为在已经卸干净的安装上重跑卸载脚本，不会再打出这份清单。

典型残留会落在你刚卸掉的前缀下：

- Apple Silicon：`/opt/homebrew/etc`、`share`、`var`、`Frameworks` 以及类似空目录或权限残缺的目录
- Intel：`/usr/local` 下同名目录，但绝不要凭猜测删掉无关的 `/usr/local` 树

若脚本点名了路径，且你确认它只属于 Homebrew，再显式删除（`-rf` 后面要有空格）：

```
sudo rm -rf /opt/homebrew/share
```

然后核对：

```
ls /opt/homebrew
ls /usr/local/Homebrew
ls ~/Library/Caches/Homebrew
ls ~/Library/Logs/Homebrew
```

这些位置消失后，预期会看到 “No such file or directory”。如果脚本已经跑完，而 `/opt/homebrew` 或某棵纯 Homebrew 树还在，通常再用 `sudo rm -rf` 删那个路径。按回车前再确认一遍路径。

前缀之外的软件配置常常会故意留下：`~/.gitconfig`、语言版本管理器、`~/Library` 下的应用支持目录，以及你已经挪出 `var` 的数据库。那些不属于 Homebrew 前缀。只有在你清楚归属时再删。

## Shell 配置里还在提它

卸载脚本不会改 `.zprofile`、`.zshrc`、`.zshenv`、`.zlogin` 或 `.bash_profile`。Homebrew 安装器常会写入类似这样的行：

```
eval "$(/opt/homebrew/bin/brew shellenv)"
```

Intel Mac 的默认 PATH 已经包含 `/usr/local/bin`，很多安装根本不需要这行。Apple Silicon 上则需要它才能找到 `brew`；卸载之后，每个新开的 shell 都会去跑一个已经不存在的二进制。

搜索并删掉 `brew shellenv`（以及你不再需要、自己加过的 `/opt/homebrew/bin` 或 `/usr/local/bin` PATH 导出）：

```
grep -n "brew shellenv\|/opt/homebrew\|/usr/local/bin" ~/.zprofile ~/.zshrc ~/.zshenv ~/.zlogin ~/.bash_profile ~/.bashrc 2>/dev/null
```

然后开一个新的终端标签页。若警告消失，且 `which brew` 没有任何输出，PATH 就清理完了。只有打算立刻重装时，才保留那一行。

## 常见失败情形

**残留路径权限被拒绝。** 脚本列出了路径却删不掉。对每个点名的路径用 `sudo rm -rf`，`-rf` 和路径之间要有空格。写成 `rm -rf/opt/...` 会报 `illegal option`，因为路径被当成了参数。

**卸到一半 / 提示 “Homebrew is already installed”。** 上一次尝试留下了 `.git`、`Cellar` 或 `bin/brew`。用正确的 `--path` 再跑官方卸载脚本，或按脚本报告清掉残留前缀；若还想用 Homebrew，再按官方安装文档重装。

**清理做到一半后出现 `brew: command not found`。** 要么卸载已经完成、PATH 仍指向旧前缀（修 shell 那一行），要么 `brew` 没了但软件包还在。别去追经典 gist。前缀目录还在的话，用官方脚本加 `--path`。

**Apple Silicon 上卸错前缀。** 你卸掉了 `/opt/homebrew`，但 Rosetta 时代的工具还在 `/usr/local`。确认那棵树确属 Homebrew、不是和其他软件混在一起之后，再以 `--path=/usr/local` 跑脚本。

**服务和登录项还在调用已消失的二进制。** 能停的话，卸载前先停 `brew services`。之后检查「登录项」以及仍引用 `/opt/homebrew` 或 `/usr/local/Cellar` 的 LaunchAgents。

**旧博文和 2011 年的 gist。** 靠 `git ls-files` 和 `pbcopy` 删除的指南属于更早的 Homebrew 布局。只认 `https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh`。

## 要不要先做更小的清理

更小的清理：`brew cleanup` 清旧版本和下载缓存，`brew uninstall <formula>` 卸单个软件包，`brew autoremove` 清掉不再需要的依赖。在 [Mole 的「分析」视图](https://mole.fit/zh/mac-disk-analyzer) 里，清理前先扫一遍 Homebrew 目录，清理后再扫一次，对比体积变化。Mole 不会自动跟踪这些命令带来的变化。

## 用导出的清单重建

新机器装好 Homebrew 之后，两条命令就能装回原来的内容：

```
xargs brew install < brew-formulae.txt
xargs brew install --cask < brew-casks.txt
```

必须分开跑，因为 cask 需要 `--cask`，混在一起会有一半失败。

`brew list` 会带上依赖，整份重放会把那些依赖也标成显式安装，之后 `brew autoremove` 能删掉的东西就变了。`brew leaves` 能避开不少这种情况，但它也不等于「你当初点过名的安装历史」：你主动装过的东西，也可能同时被别的软件包依赖。重装前通读清单，把还需要的工具补进去。

Homebrew 这个 Mac 应用，Mole 会列出什么、又会给 brew 命令留下什么，写在 [卸载 Homebrew 应用，留下 brew 命令](https://mole.fit/zh/tested-apps/homebrew)。

## 常见问题

### 卸载 Homebrew 会删掉它装过的全部软件包吗？

还在 Cellar 和 Caskroom 里的会。官方脚本会移除前缀，formula 和 cask 元数据一起走。前缀之外的配置和数据可能留下。

### `/opt/homebrew` 和 `/usr/local` 有什么区别？

`/opt/homebrew` 是 Apple Silicon 的默认前缀。`/usr/local` 是 Intel 的默认前缀，迁过来的 Mac 上也可能还装着旧的 Rosetta Homebrew。删之前务必先看 `brew --prefix`。

### 跑卸载脚本前，要不要先卸掉每个 formula？

只有当你想分阶段清理，或需要先检查 `var` 里的数据时才有必要。脚本本来就会清掉前缀里已装的软件包。无论哪种做法，只要可能重装，都先导出清单。

### 为什么卸载后还会留下目录？

权限问题，或 Homebrew 自己没创建过的目录，都可能挡住删除。脚本只会打印一次这些路径。确认它们是 Homebrew 残留后，再用 `sudo` 删。

### 整棵 `/usr/local` 都能删吗？

不能。在 Intel Mac 上这个目录是共用的。用官方卸载脚本，若还有残留，只删它点名的路径。

### 每个新终端都还在报 brew shellenv 错误，怎么办？

从 zsh 或 bash 的启动文件里删掉 `eval "$(...brew shellenv)"` 那一行，再开一个新标签页。卸载脚本不会替你改这些文件。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-homebrew-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
