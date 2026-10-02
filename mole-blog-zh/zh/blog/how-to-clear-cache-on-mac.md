# Mac 的 ~/Library/Caches 是什么，哪些可以删除

> 分清缓存、状态、数据库和个人数据，先测量归属，再用应用自己的清理入口，并在清空废纸篓前验证。

Published: 2026-06-17 | Updated: 2026-09-28

真正的缓存可以重新生成，但名字里带 Cache 的目录并不自动等于能删，因为应用有时会把离线下载、会话状态、索引和未同步内容就近放在缓存旁边。Apple 的 [Library 目录说明](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/MacOSXDirectories/MacOSXDirectories.html)把缓存定义为可重新生成、应用不应依赖其持续存在的内容。动手前最好先弄清文件属于谁、从哪里重建，以及删掉之后会失去什么，再决定清哪一层。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/cache-lifecycle.webp" width="1360" height="454" loading="lazy" alt="一次缓存查询会进入命中或未命中分支，未命中时重新获取源数据并建立缓存">
  <figcaption>缓存未命中时，应用会重新获取源数据并生成副本。</figcaption>
</figure>

命中时应用直接读本机副本继续跑，未命中就回到源头重新取一份，再决定要不要存下来。说不出这个源头在哪里，就没法判断一次未命中还能不能恢复。

## `~/Library/Caches` 和 `/Library/Caches` 的范围不一样

- `~/Library/Caches` 保存当前用户的应用缓存，子目录通常使用 Bundle ID，比如 `com.google.Chrome`
- `/Library/Caches` 保存系统范围的缓存
- `/System` 受到系统完整性保护，不在手动清理范围内

Apple 记录了以 Bundle ID 命名子目录的惯例，但眼熟的名字不等于安全证明。同一个应用可能把缓存数据库、会话状态和下载内容并排放在一起，沙盒应用还会把相关内容放进 Containers，而不是用户 Caches 根目录。

Apple 的[文件系统说明](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html)明确把 `/System/Library` 留给系统。不要递归删除任一 Caches 根目录，也不要把权限错误当成给通用清理命令加 `sudo` 的理由。

## 决定之前先给内容分类

| 类型 | 常见内容 | 删除后会发生什么 | 默认处理 |
| --- | --- | --- | --- |
| 缓存 | 缩略图、编译输出、可重新下载的副本 | 首次打开变慢，并增加耗电和流量 | 先确认归属，再用应用自己的清理入口 |
| 状态或会话 | 窗口、标签页、Cookie、草稿、登录令牌 | 丢失工作现场、登录状态或离线连续性 | 保留，除非应用提供明确的重置项 |
| 数据库或索引 | `.db`、`.sqlite`、加密索引、内容寻址目录 | 可能丢失搜索、离线记录或唯一的本地副本 | 只有所有者明确说明可重建时才处理 |
| 个人数据 | 文稿、照片、邮件、聊天、模型和凭据 | 永久丢失，或需要付出大量下载成本 | 不按缓存处理 |

同一个目录里常常三类内容混在一起，所以 `~/Library/Caches` 更像一条判断线索，并不能单独证明整棵目录树都可以清空。

适合清理的情况大致有三种：缓存确实占掉了急需的空间，应用的官方排障文档明确要求清理，或者索引已经确认损坏、过期。macOS 和许多应用会在空间紧张时自动淘汰缓存，定期把整棵 Caches 树清空，反而会暂时拖慢启动，并多付一轮耗电和下载量。

Application Support、Containers、Group Containers、偏好设置、钥匙串和隐藏的工具目录，默认都按状态或个人数据处理。只有所有者明确说明其中某个子目录可以重建，才把它归入缓存。

## 先量出最大的那几个所有者

```shell
du -sh ~/Library/Caches/* 2>/dev/null | sort -h
du -sh /Library/Caches/* 2>/dev/null | sort -h
```

从下往上读，最后几行就是最大的那几项，把每一项对上具体的应用、Bundle ID 或有文档说明的工具缓存，认不出所有者就停在那里。一个 50 MB、所有者明确的缓存，比一个 20 GB 的不明目录更值得评估。这些数字只是测量结果，不是可回收空间的承诺，文件可能仍被打开，APFS 会通过克隆和快照共享数据块，Finder、`du` 和系统设置对存储的归类方式也各不相同。

读到权限错误就停下来，不要为了看完整结果加 `sudo`。`~/Library/Caches`
只影响当前用户，`/Library/Caches` 的范围更大，可能服务多个账户或系统组件。
系统设置里的“系统数据”只是没有归入其他类别的汇总，不是一个目录。Apple 的 [Mac 存储空间说明](https://support.apple.com/102624)也采用这一分类方式，因此测量结果应以具体所有者目录和实际可用空间为准。

## 删除前先回答五个问题

1. 这个目录属于哪个应用、命令行工具或系统服务
2. 所有者能否重建，从哪个原始文件或远端来源重建
3. 缓存未命中会付出多少编译时间、电量、网络流量和离线能力
4. 应用、下载器、包管理器或模型服务是否仍在运行
5. 永久删除前有没有预览、废纸篓或明确的重新下载路径

其中任何一项答不出来，就先保留。

## 优先使用所有者的清理入口

所有者知道哪些内容还被引用、哪些 blob 是共享的、哪个版本仍在使用，也知道哪些文件看着旧却仍然必需，所以它自己的清理入口往往删得更少，回收得也更稳。

### 浏览器

浏览器缓存可能达到数 GB。Chrome 的[清除浏览数据说明](https://support.google.com/chrome/answer/2392709?hl=en-uk)把缓存图片和文件与历史记录、Cookie、密码、网站设置及离线数据分开。只想释放缓存空间时，只选“缓存的图片和文件”，不要删除整个浏览器配置目录。

其他浏览器只是叫法不同，分类方式差不多，从浏览器自己的存储或隐私设置进去，把勾选的每一项都读一遍再确认。

### 开发工具

开发工具的缓存往往很大，因为它们拿磁盘空间换构建和安装速度，所以清理命令要对上具体的所有者。

Xcode Derived Data 可以重建，但重建会消耗时间和电量。Homebrew 先按[官方清理说明](https://docs.brew.sh/Manpage.html#cleanup-options-formulacask-)运行 `brew cleanup --dry-run` 预览，再决定是否执行 `brew cleanup`。npm 的缓存会自我修复，应先运行 `npm cache verify`；[npm 缓存文档](https://docs.npmjs.com/cli/v11/commands/npm-cache/)说明，除非是明确的空间回收或排障，不需要把 `npm cache clean --force` 当成日常维护。

```bash
npm cache verify
```

先确认源码、依赖和工具链仍可用，再退出 Xcode 和相关构建、测试，只处理准备重建的项目，并尽量走 Xcode 自己的清理或设置入口。包仓库、签名素材、模拟器数据和源码检出，不会因为同属开发工具就变成 Derived Data。

### AI 工具和下载的模型

```bash
hf cache ls
hf cache rm model/example --dry-run
hf cache prune --dry-run
```

```bash
ollama ls
ollama rm <model>
```

AI 工具尤其要用自己的清理接口。Hugging Face 先用 `hf cache ls`
查看模型和版本，再用 `hf cache rm model/example --dry-run` 或
`hf cache prune --dry-run` 预览，具体参数以 [Hugging Face 缓存命令](https://huggingface.co/docs/huggingface_hub/main/en/guides/cli#hf-cache)为准；Ollama 先用 `ollama ls` 查看模型，再按 [Ollama CLI 说明](https://github.com/ollama/ollama/blob/main/docs/cli.mdx)用
`ollama rm <model>` 删除明确选中的模型。模型是主动下载的内容，不是普通
缓存，聊天记录、凭据、会话、配置和仍在使用的数据库也不能顺手清掉。

清理前先退出对应应用，因为运行中的应用可能正在写入，或以内存映射方式使用缓存，直接删文件会把缓存数据库弄坏。浏览器里的 Cookie、网站数据、历史记录、密码和页面缓存是不同选项，只想释放空间时选缓存内容即可，清除全部浏览数据可能让网站退出登录，却不一定多腾出多少空间。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/mole-clean-review.webp" width="2584" height="1741" loading="lazy" alt="Mole 清理审查页按类别列出缓存和各自大小，每类都有勾选框，底部是永久清理已选 5.14 GB 的按钮。">
  <figcaption>删除之前，审查页列出每类缓存和它的大小，只清理勾选的部分。这是 Mole 的“清理”视图。</figcaption>
</figure>

手动清理没有问题，只是 Bundle ID 和目录名不总是好认。清理工具最好在操作前就把归属、路径、大小和类别摊开，默认排除配置、文稿、模型目录和聊天记录。[Mole](https://mole.fit/zh/) 的“清理”视图采用清理前确认的方式，明确哪些内容会跳过，并对每条路径重新验证，这比扫描出多少文件更重要。

## 仍需手动处理时

手动处理应该放在最后：退出所有者和相关后台进程，只把一个已经确认的缓存子目录移到废纸篓，重新打开应用并重复原来的工作，再检查文稿、登录、离线数据、偏好、构建产物和模型是否仍正常。Apple 的 [Mac 存储空间说明](https://support.apple.com/102624)提醒，文件移到废纸篓后，只有清空废纸篓才会真正释放空间。这个延迟正好提供恢复窗口。

Mole 的开源命令行工具在 `lib/clean` 中展示了类似检查，原生 App 用 Swift 实现同类规则。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/cache-safety-gates.webp" width="1360" height="454" loading="lazy" alt="缓存候选项先经过有超时的容量检查和安全规则，再进入确认，运行中的工具、保护文件和受保护目录会让它被跳过">
  <figcaption>工具状态、保护文件和受保护目录会决定候选项进入确认，还是留在原位。</figcaption>
</figure>

较稳妥的实现会优先调用包管理器自己的清理接口，避开仍被后台服务使用的构建缓存，对解析后的路径再次验证，并为缓慢的容量探测设超时。所属工具不存在、路径含义不清，或目录被明确保护时，保留比猜测更合适，递归删除整个 Caches 目录会绕过这些判断。

- AI 助手的聊天历史和对话记录
- 偏好设置和保存的登录信息
- `/System` 下的内容
- 无法确认用途的目录

## 为什么缓存和系统数据还会长回来

一个相对容易复核的顺序是：先测量大小和归属，退出应用，再用应用自己的存储设置或清理命令，一次只处理一个类别。重新打开应用，确认登录、离线内容和项目仍正常后，再清空废纸篓。偏好设置、描述文件、聊天、文稿，以及含义不明的 Application Support 数据，本来就不算缓存。

缓存重新长回来通常是正常现象，应用会重新下载文件、生成缩略图、编译输出和建立索引。下一次启动因此可能暂时多用 CPU、电量、网络和磁盘。系统设置里的“系统数据”是没有归入其他类别的总和，不是一棵可以直接删除的目录，统计也可能晚于真实文件变化。清理后看实际可用空间和应用是否正常，不要为了追一个延迟更新的分类继续扩大删除范围。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clear-cache-on-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
