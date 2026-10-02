# Mac 上如何用 brew cleanup 和 autoremove 清理 Homebrew

> 先预览 brew cleanup、检查下载缓存，再安全使用 brew autoremove；不要手动删除 Cellar、Caskroom 或缓存目录。

Published: 2026-06-25 | Updated: 2026-09-05

Homebrew 在安装和升级时会顺手清理相关旧版本，也会定期执行全局清理，但长期使用之后，下载缓存、固定版本、构建产物和旧依赖仍可能慢慢累积，手动删除目录会绕过 Homebrew 自己的版本与依赖记录，用它自带的命令查看和清理会更稳妥。

## Homebrew 会留下什么

- **旧版本：** cleanup 通常会删除被取代的版本，但被固定或刚升级的版本可能会继续保留一段时间
- **下载归档：** bottle 与 cask 的下载会进入缓存，避免下次再下载一遍
- **未使用依赖：** 当前版本的 cleanup 和 uninstall 通常会自动移除不再需要的 formula 依赖，但旧安装、中断过的操作或关闭自动移除之后，仍可能有残留

先用两条命令查看缓存的位置和实际大小：

```
brew --cache
du -sh "$(brew --cache)" 2>/dev/null
```

## 先预览，再清理

下面是实际执行清理的命令，先不要运行，先看后面的预览步骤：

```
brew cleanup
```

当前 [Homebrew 手册](https://docs.brew.sh/Manpage#cleanup-options-formulacask-)说明，普通 cleanup 默认移除旧安装版本和超过 120 天的下载，保留时间可以用 `HOMEBREW_CLEANUP_MAX_AGE_DAYS` 调整，动手前先用 `-n` 预览一遍，`-s` 则会更深入地清理缓存，但仍保留当前已安装 formula 和 cask 所需的下载：

```
brew cleanup -n
brew cleanup -s
```

想单独检查未使用依赖时，先运行下面第一条预览，确认列出的依赖没有被自己的脚本或项目直接使用，再单独运行第二条删除：

```
brew autoremove --dry-run
brew autoremove
```

`brew list` 列出已安装软件包，`brew outdated` 显示待更新项目，`brew --cellar`、`brew --caskroom` 和 `brew --cache` 会返回真实路径，Intel 与 Apple 芯片的安装位置不同，写死 `/opt/homebrew` 很容易找错目录。

## 分开测量三个储存区

一个总数会掩盖三种不同决策。清理前先记录已安装 formula、cask 元数据和下载缓存各占多少：

```
du -sh "$(brew --cellar)" "$(brew --caskroom)" "$(brew --cache)" 2>/dev/null
```

缓存大最容易回收，因为 Homebrew 可以重新下载。Cellar 大多是已安装软件，`cleanup` 只能移除被替代的版本。Caskroom 很大也不表示应用本体在那里，cask 应用通常位于 `/Applications`。清理后再跑一次同样命令，确认究竟是哪一处变小。

## 为什么不用 Finder 手动删

`brew cleanup` 会根据安装版本、保留策略和依赖关系选择清理对象，`brew autoremove` 只处理依赖判断，它们比手动删目录安全，但仍可能移除用于回退的旧版本或离线安装所需的 bottle，所以要先预览，并有意保留固定版本与关键工具链。

## Cellar、Caskroom 和 cleanup 的判断依据

Homebrew 会把 formula 的每个版本安装到 `brew --cellar` 返回的 Cellar，再把当前版本链接到前缀，升级时新旧版本会先并存，然后活动链接才切换，cask 元数据位于 Caskroom，应用包通常在 `/Applications`，下载则保存在 `brew --cache` 返回的位置。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/homebrew-cellar.webp" width="1360" height="454" loading="lazy" alt="Homebrew Cellar 同时保存一个 formula 的旧版本与当前版本，bin 符号链接指向当前版本，cleanup 移除被取代版本和旧缓存下载。">
  <figcaption>Homebrew 根据 Cellar 版本、活动链接和缓存记录判断哪些内容已经被取代。</figcaption>
</figure>

## 其他工具应该止步的地方

磁盘地图可以发现 Homebrew 缓存并显示大小，[Mole](https://mole.fit/zh/) 也能展示这一类别，但版本和依赖关系仍以 Homebrew 自己的记录为准，找到位置并不等于已经知道能否删除。

## 安全的操作顺序

路径命令能说明各存储实际有多大，`brew cleanup -n` 能预览普通清理的候选项；考虑 `-s` 时，也要给该模式加上预览选项重新确认范围，不能拿普通预览当作更大范围的删除清单。依赖关系可以单独用 `brew autoremove --dry-run` 检查，自动清理正常、磁盘空间充足时，没有必要设定固定周期。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clean-up-homebrew-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
