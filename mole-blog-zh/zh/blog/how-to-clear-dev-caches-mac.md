# 如何清理开发工具缓存，又不破坏构建

> 区分可重建缓存、项目依赖、共享存储和本地数据库，再用 npm、pip、Cargo 等工具自己的命令清理。

Published: 2026-06-03 | Updated: 2026-09-24

开发环境的空间分散在软件包下载、构建输出、SDK、本地数据库和项目依赖树中，其中一部分可以随时重建，一部分要靠锁文件、软件源和工具链都在场才回得来，还有一些就是唯一的本地数据，而点号目录只说明它默认被隐藏，并不能说明它是缓存。

## 查看各工具的全局存储

```
du -sh ~/.npm ~/.cargo ~/.cache/pip ~/Library/Caches/pip ~/.gradle 2>/dev/null | sort -h
```

这些只是各工具的默认位置，实际路径可能被配置改掉，动手之前尽量让工具自己报告位置，例如 `npm config get cache`、`python -m pip cache dir` 和 `pnpm store path`，这比照着猜测的路径删要可靠得多。

- **npm：** 默认缓存位于 `~/.npm`，先运行 [`npm cache verify`](https://docs.npmjs.com/cli/commands/npm-cache) 让缓存自我检查，因为它本身可以自我修复，`npm cache clean --force` 只适合明确为了回收空间或排查故障的场景
- **Cargo：** `~/.cargo` 里不只有缓存，还可能装着已安装的二进制文件、配置、软件包记录与注册表凭据，当前版本的 Cargo 会定期清理未使用缓存，手动处理时也应落到具体子目录，整体删除会把这些非缓存内容一起带走，详见 [Cargo home](https://doc.rust-lang.org/cargo/guide/cargo-home.html)
- **pip：** 默认把 wheel 和 HTTP 响应缓存到 `~/Library/Caches/pip`，`python -m pip cache info` 会显示实际位置与大小，[`python -m pip cache purge`](https://pip.pypa.io/en/stable/cli/pip_cache/) 用于清空
- **pnpm、yarn：** 可以分别用 `pnpm store prune` 和 [`yarn cache clean`](https://yarnpkg.com/cli/cache/clean) 处理未引用或已缓存的包，动手前先确认版本，因为存储布局以及本地、全局行为都会随版本变化
- **Gradle、Maven：** `~/.gradle` 和 `~/.m2` 都可能很大，Gradle 已有[定期缓存清理](https://docs.gradle.org/current/userguide/directory_layout.html)，手动检查前可以先停止守护进程，Maven 仓库还可能含有本地构建或私有产物，整库删除会失去这些唯一副本

清掉这些内容可能触发大型下载和原生代码重编译，甚至让已经从软件源消失的历史版本再也无法构建，所以在称它为可重建之前，先确认锁文件、软件源和工具链仍然可用。

IDE 缓存是另一层。JetBrains IDE 把缓存、索引和 Local History 放在 `~/Library/Caches/JetBrains`，哪些能重建见[Mac 上的 JetBrains 缓存](https://mole.fit/zh/blog/jetbrains-caches-mac)；Android Studio 虽然基于同一平台，缓存却放在 `~/Library/Caches/Google/AndroidStudio*`，不在 JetBrains 文件夹里。

## 找出散落的 node_modules

真正占空间的往往不是一个全局缓存，而是多个旧项目各自的 `node_modules`，每个目录常有 200 至 500 MB，累积起来相当可观。

```
find ~/www ~/Projects -name node_modules -type d -prune -exec du -sh {} + 2>/dev/null
```

把示例根目录换成自己实际的项目位置，其中 `-prune` 用来避免继续进入依赖树，`-exec` 则能正确处理路径中的空格和特殊字符。

项目能否重建依赖，取决于源码、锁文件、软件包管理器版本、注册表访问和原生工具链是否齐全，保留这些输入之后，使用与锁文件对应的命令即可，例如 `npm ci`。

## 这些内容不能当缓存删

项目源码、`package.json` 这类项目配置，以及 `package-lock.json`、`Cargo.lock` 等锁文件定义了构建本身，它们不是缓存，全局共享存储正在被安装使用时也不能清理，否则可能损坏索引，动手前先停止构建、安装和相关守护进程。

## 共享存储和项目依赖树不同

pnpm 会在全局内容寻址存储中保留软件包版本，再通过硬链接接入各项目的 `node_modules`，让多个项目共享同一份磁盘内容，Cargo 和 Go 的缓存也常按版本或校验和组织。

传统 npm 安装则会在每个项目中生成一份依赖树，因此删除一个项目的 `node_modules` 与清理全局共享存储，影响范围完全不同，内容寻址能做去重和校验，却不能保证未来仍能从软件源下载同一个版本。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/content-addressed-store.webp" width="1360" height="454" loading="lazy" alt="左侧一个共享全局存储通过硬链接连接到多个项目，右侧同一个库被复制进每个项目的 node_modules，形成重复。">
  <figcaption>共享内容寻址存储和每个项目单独生成的依赖树，清理边界不同。</figcaption>
</figure>

## 用地图找目标，用软件包管理器清理

[Mole](https://mole.fit/zh/) 的「分析」页可以找出哪个项目或存储最大，软件包管理器自己的 verify、prune 和 clean 命令则真正理解索引与引用关系，先用地图选目标，再由所属工具执行清理。

## 安全的操作顺序

一个比较容易复核的顺序是：先停止构建和守护进程，测量选定目录，验证软件包存储，保留源码与锁文件，再处理依赖树，文档里的 prune 或 clean 命令通常比删除整个点号目录更了解引用关系，清空废纸篓之前，也可以用一个重要项目的完整构建确认输入仍然齐全。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clear-dev-caches-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
