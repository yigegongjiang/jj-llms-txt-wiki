# Mac 上的 JetBrains 缓存（IntelliJ、WebStorm、PyCharm）

> 缓存和索引下次启动会重建，配置、插件和项目数据不会。先用 Invalidate Caches，再量一量，然后才动 JetBrains 的文件夹。

Published: 2026-09-20 | Updated: 2026-09-25

IntelliJ IDEA、WebStorm、PyCharm 这些 JetBrains IDE 会在 Mac 上留下好几个动辄数 GB 的目录，其中只有一部分能安全清理。缓存和索引下次启动会重建；配置、插件和项目数据不会。分清这层差别，才不会把一次清理变成丢设置或弄坏项目。

这篇文章是通用开发缓存指南的 JetBrains 专篇。文中的「Mole」只指 mole.fit 上的 Mole for Mac，以及它自带的 `mo` 命令行工具；文中不给每个 IDE 的 GB 数字，因为差别太大没有参考价值，你在自己机器上测。

<figure class="blog-diagram">
  <picture>
    <source media="(max-width: 900px)" srcset="/img/blog/jetbrains-caches-layers-mobile.webp">
    <img src="https://mole.fit/img/blog/jetbrains-caches-layers.webp" width="1536" height="1024" loading="lazy" alt="JetBrains 在 Mac 上的数据：可重建的缓存和索引、日志，以及可能记着个人修改的 Local History。">
  </picture>
  <figcaption>缓存和索引会重建。日志用于排查问题，Local History 可能记着你自己的修改。</figcaption>
</figure>

## JetBrains 存了什么，哪些不只是缓存

一个 JetBrains 安装是好几个恢复成本差别很大的目录。把它们当成一个文件夹，正是清理出错的地方。

| 层 | 存的是什么 | 能重建吗？ |
|---|---|---|
| 缓存 | 编译好的检索数据和临时工作文件 | 能，下次启动 |
| 索引 | IDE 为浏览代码建立的项目索引 | 能，重新索引即可 |
| 日志 | 排查问题用的诊断文本 | 不能，但排查完问题就可以丢掉 |
| 配置和设置 | 键位、代码风格、偏好设置 | 不能，这是你的配置 |
| 插件 | 已装插件及其数据 | 部分，重装会丢本地插件状态 |
| 项目数据 | `.idea` 文件夹和 Local History | 不能，这是项目和个人成果 |

缓存和索引会自我重建。日志删了不会回来，但排查完问题就可以丢掉。后三层是你的配置和成果，无论放在哪个文件夹里都不是缓存。

## 优先用 IDE 自带的工具

删任何文件夹之前，先用 IDE 内置的入口。File › Invalidate Caches 会清掉缓存和索引并提议重启，这是不动设置就修好陈旧索引的受支持做法。具体菜单文案以你当前的 IntelliJ、WebStorm 或 PyCharm 版本为准，菜单标签会随版本变化。

旧版本通常才是占空间最多的地方。每个大版本的 IDE 都有自己的一套文件夹，升级后旧的那套会留在原地。Help › Delete Leftover IDE Directories… 会找出旧版本 IDE 的设置、缓存和日志目录，由你选择删哪些，仍然安装着的版本一开始就不勾选。当前的 IntelliJ 平台版本还会在升级大约一周后自动做一部分清理：对 180 天没用过的版本，删掉它的系统目录（缓存、索引以及那个版本的 Local History）和日志，保留设置和插件。

手动查看任何文件夹前先退出 IDE，免得删到运行中进程还占用的文件。

## 在 Mac 上找到系统目录

JetBrains 按版本记录它的目录位置，而产品加版本的具体子文件夹在每台机器上都不同，所以通过 IDE 定位你自己的（Help › Show Log in Finder 会打开日志文件夹），别去粘贴过时的绝对路径。在当前版本里，macOS 上的父文件夹是 `~/Library/Caches/JetBrains` 和 `~/Library/Application Support/JetBrains`，日志在 `~/Library/Logs/JetBrains`。

动手前先量一量父目录，看空间到底花在哪里：

```
du -sh ~/Library/Caches/JetBrains ~/Library/Application\ Support/JetBrains ~/Library/Logs/JetBrains
```

这样能看出系统目录、配置目录和日志各占多少，某一行提示「No such file or directory」只说明这台 Mac 上没有那个文件夹。Caches 的大小也包含 Local History，不能整个当成可删的缓存；设置和插件则在 Application Support。

## 安全顺序

1. 量一量 JetBrains 的父文件夹，知道空间在哪。
2. 运行 File › Invalidate Caches，不勾选另外删除 Local History 的选项，然后重启 IDE。
3. 等重新索引完成后再量一次。索引会重建，所以腾出的空间会有一部分回来。
4. 旧版本交给 Help › Delete Leftover IDE Directories… 处理，别手动删文件夹；旧日志等排查完问题再删。
5. 绝不随手删 `.idea` 或 Local History，那是项目和个人成果。

和包管理器、散落的 `node_modules` 的重叠属于另一篇：[如何清理 Mac 上的开发缓存](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)讲 npm、Cargo、pip 和 Gradle，[如何清理 Xcode](https://mole.fit/zh/blog/how-to-clean-up-xcode-mac)讲 DerivedData（如果你也用苹果工具链构建）。IDE 不是 JDK，移除运行时是另一件事，见[在 Mac 上卸载 Java](https://mole.fit/zh/blog/how-to-uninstall-java-mac)。

## 磁盘视图什么时候有用

一旦你知道 JetBrains 的文件夹很大，磁盘视图就能帮你看清空间花在哪个产品、哪个版本上。Mole 的「分析」会把大目录连同路径和大小列出来；扫描免费、不需要许可证，删什么由你决定。Mole 的「清理」刻意不碰 `~/Library/Caches/JetBrains`、`~/Library/Application Support/JetBrains` 和 `~/Library/Caches/com.jetbrains.toolbox`，因为它们在缓存旁边还放着 Local History 和设置，清理这些仍然要靠 IDE 自己的工具。在 Mole 里卸载某个 JetBrains IDE 时，它按版本命名的文件夹会列出但默认不勾选，而 Caches 和 Application Support 下的那几个同样受这层保护，会留在原处；如果你不再需要那个 IDE 的设置或 Local History，自己核对后再删。

这和 `mo purge` 是两回事，后者扫的是项目构建产物和依赖文件夹，而不是 IDE 缓存，见[讲清 Mac 上的 purge](https://mole.fit/zh/blog/purge-on-mac-explained)。Mole 复核什么、又拒绝碰什么，见[Mole 安全吗](https://mole.fit/zh/blog/is-mole-safe)。

## 延伸阅读

- [如何清理 Mac 上的开发缓存](https://mole.fit/zh/blog/how-to-clear-dev-caches-mac)：包管理器和 `node_modules` 那一面。
- [哪些 Mac 缓存可以安全删除](https://mole.fit/zh/blog/which-mac-caches-are-safe-to-delete)：可重建与否的通用规则。
- [Mole 安全吗](https://mole.fit/zh/blog/is-mole-safe)：一个复核式删除工具会删什么、不会删什么。

## 常见问题

### 清缓存会删掉我的项目吗？

默认的 Invalidate Caches 不会删项目文件或 Local History，但 Local History 就在系统目录里，对话框另有删除它的选项，别勾选。

### 用 Invalidate Caches 还是删系统目录？

Invalidate Caches 是受支持的做法：它在重启时清掉缓存和索引，不会删设置和插件，只要不勾选额外选项，Local History 也会保留；手动删掉整个系统目录则可能把 Local History 一起删掉。设置和插件通常放在 Application Support，而不是那个系统目录。

### 多个 IDE 或 Toolbox：清一次还是逐产品清？

每个产品和版本各有子文件夹，缓存按产品、也按版本累积。通过各个 IDE 逐产品清理，不再用的版本交给 Help › Delete Leftover IDE Directories…，动手前量一量 JetBrains 的父文件夹，看清到底哪个安装大。

---

Canonical HTML page: https://mole.fit/zh/blog/jetbrains-caches-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
