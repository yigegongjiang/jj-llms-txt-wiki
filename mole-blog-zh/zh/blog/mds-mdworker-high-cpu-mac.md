# mds 和 mdworker 占用 CPU 过高怎么办

> 系统更新或大量文件变化后，Spotlight 短暂高占用很正常。长期不退时，再检查反复变化的目录和磁盘卷。

Published: 2026-06-29 | Updated: 2026-09-05

`mds`、`mds_stores` 和 `mdworker` 都属于 Spotlight 索引系统。macOS 更新、数据迁移、恢复或大量文件变化后，它们短时间占用 CPU 很正常。`mdutil -s` 只能确认索引是否开启，不能显示重建进度，也没有固定的完成时间。

## 这些进程分别做什么

**mds** 负责协调索引，**mds_stores** 管理索引数据库，**mdworker** 扫描文件并提取可搜索的内容和元数据。索引是最新状态时，它们大多空闲，大量文件发生变化后，它们会重新工作。

## 为什么占用会突然升高

- macOS 更新或数据迁移后，磁盘上大量内容变了
- 恢复、移动或复制大量文件后
- 接入尚未索引的外置盘或网络卷后
- 下载、构建或同步让一个目录持续快速变化时

检查索引是否启用：

```
mdutil -s /
```

「Indexing enabled」是状态，不是进度。把持续的 `mdworker` CPU 与「活动监视器」中的磁盘读写、近期文件变化放在一起看。工作仍在推进时，让 Mac 接通电源并等待完成。

大型源码目录、邮件数据库、外置卷、慢速存储，以及反复同步都会延长时间。

## 长时间不结束怎么办

先保存工作，再逐个暂停持续改写文件的应用；需要排查外置盘时，等写入停止并从 Finder 正常推出后再断开。这样更容易看出是否有某个目录或磁盘一直产生新变化。

在「系统设置 > Spotlight > 搜索隐私」中排除目录，可以减少索引工作，代价是该目录不再出现在搜索结果中。这个取舍更适合明确不需要搜索的构建产物、虚拟磁盘或归档，整个个人目录和启动磁盘的影响就大得多。

如果搜索结果已经缺失，或索引能确认卡住，可以按 Apple 的 [Spotlight 重建说明](https://support.apple.com/102321)操作：把受影响磁盘或文件夹加入「搜索隐私」，短暂等待，再移除。不同 macOS 版本可能显示「搜索隐私」或「Spotlight 隐私」。移除后，Spotlight 会重新索引。

重建会把昂贵工作再做一遍，只适合修复确认存在的索引问题，不是看到 CPU 升高后的第一步。

## Spotlight 索引怎样工作

Spotlight 会在每个卷隐藏的 `.Spotlight-V100` 中维护倒排索引，也就是从词语和元数据映射回文件。

文件变化后，FSEvents 通知 mds，mds 把文件交给 mdworker，mdworker 加载对应的 `.mdimporter` 插件，提取文本和元数据。普通编辑只处理少量变化，完整重建则要重新导入所有符合条件的文件。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/spotlight-index-pipeline.webp" width="1360" height="454" loading="lazy" alt="变化后的文件依次经过 FSEvents、mds、mdworker 和导入插件，进入 Spotlight 倒排索引，重建则让所有文件重新经过相同流程。">
  <figcaption>文件变化依次经过 mds、mdworker 和元数据导入器。重建会让符合条件的内容重新走完整条流程。</figcaption>
</figure>

## 监控工具能确认什么

「活动监视器」或 [Mole](https://mole.fit/zh/) 的「状态」页可以确认 CPU 与磁盘占用来自 Spotlight。接下来要判断它是在处理一批有限变化，还是某个目录一直被改写。查看近期活动并逐项隔离，比强制结束工作进程更有效。

## 可复现的诊断

索引工作有进展时，等待通常就够了，长期不退，才需要隔离持续变化的目录或磁盘。排除目录会失去搜索结果，重建也会重复昂贵工作，因此两者都适合有明确证据的情况，结束 `mdworker` 本身并不能清理索引问题。

---

Canonical HTML page: https://mole.fit/zh/blog/mds-mdworker-high-cpu-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
