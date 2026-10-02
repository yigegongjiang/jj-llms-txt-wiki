# 如何清理 Docker for Mac，又不丢数据

> 先用 Docker 清点镜像、容器、构建缓存和数据卷，再逐类清理，避免误删数据库和其他唯一数据。

Published: 2026-06-26 | Updated: 2026-09-24

Docker Desktop 的镜像、容器可写层、构建缓存和数据卷都放在 Linux 虚拟磁盘中，很容易占用几十 GB，其中构建缓存通常能重建，数据库卷却可能是重要数据的唯一副本，所以 Docker 自己的清单比范围最大的 prune 命令更适合作为起点。如果想卸载 Docker Desktop 而不是在里面腾空间，请看[在 Mac 上卸载 Docker Desktop](https://mole.fit/zh/tested-apps/docker)。

## 为什么空间没有立刻回来

Docker Desktop 在 macOS 上运行一台 Linux 虚拟机，数据通常保存在稀疏虚拟磁盘 `Docker.raw` 中，它的 [Mac 存储说明](https://docs.docker.com/desktop/troubleshoot-and-support/faqs/macfaqs/#where-does-docker-desktop-store-linux-containers-and-images)给出了入口：「Docker Desktop > Settings > Resources > Advanced」，这里能看到位置、上限和实际占用，也能安全移动虚拟磁盘，直接用 Finder 搬动则可能破坏配置。

稀疏文件的逻辑上限可能很大，实际分配却小得多，所以 `ls -lh` 看到的数字不能代表真实占用，Docker 内部统计更接近各类对象的实际情况：

```
docker system df -v
```

详细视图会分开列出镜像、容器、数据卷和构建缓存，其中「Reclaimable」只表示 Docker 没发现当前引用，并不能判断以后是否还会用到已停止容器、旧镜像或数据卷，`docker ps -a` 和 `docker volume ls` 可以补上这部分背景。

## 从范围最小的清理开始

直接删除虚拟磁盘文件等同于丢掉整个 Docker 环境，按类别从 Docker 内部回收更容易控制范围：

```
docker builder prune --filter until=168h
docker image prune
docker container prune
```

第一条移除符合筛选条件的七天前未使用构建缓存，可以按自己的工作周期调整，后两条会在删除悬空镜像和已停止容器前确认。prune 不经过废纸篓，也不能撤销；停止的容器仍可能有唯一的可写层数据，悬空镜像也未必能重新取得，确认不再需要或已有可验证备份后再继续。每一步之后重新运行 `docker system df -v`，看实际回收了什么。

Docker 的[清理说明](https://docs.docker.com/engine/manage-resources/pruning/)把 `docker system prune` 定义为范围更广的便捷命令，加入 `-a` 会删除所有未使用镜像而不只是悬空层，加入 `--volumes` 还会处理未使用的匿名数据卷，其中可能有数据库和其他状态。

`docker system prune -a --volumes` 的范围很宽，尤其会触及可能保存数据库的数据卷，卷名、标签和归属可以先这样查看：

```
docker volume ls
docker volume inspect <volume-name>
```

Compose 数据卷通常带有项目与服务标签，确认所属项目后，优先用数据库自身工具导出数据；做文件级备份前先停止写入，并检查备份能否恢复，再删除无法重建的数据所在卷。

## 让虚拟磁盘归还物理空间

`Docker.raw` 不能靠在 Finder 里删除来回收空间，它是整台 Linux 虚拟机的磁盘，在 Finder 里删掉或用 `rm` 删除，会把里面所有镜像、容器和数据卷一起毁掉，而不只是空闲的那部分，Docker 下次启动时会重建一个空磁盘。回收要从 Docker 内部做：先 prune，再让稀疏磁盘归还空闲块，磁盘镜像的位置和上限在 Settings 里调整。看到的巨大数字往往是稀疏文件的逻辑上限，不是实际占用的物理空间，先量出实际分配，再判断能回收多少。

prune 会先在 Linux 文件系统内部释放块，macOS 不一定同时拿回空间，当前 Docker 文档说明 `Docker.raw` 通常会在数秒内归还符合条件的块，旧式 `Docker.qcow2` 依赖后台过程，可能要几分钟。

这里更有意义的是重新测量物理占用而不是逻辑上限，恢复出厂设置也不是压缩，它会销毁本地容器、镜像、数据卷和设置，只适合已经决定丢弃整个环境并导出重要数据的情况。

## Docker.raw 为什么只增不减

`Docker.raw` 是 Linux 虚拟机的稀疏磁盘，来宾系统写入时它会增长，删除对象时则先把来宾文件系统中的块标为空闲，只有来宾发出 TRIM 或 discard、Docker Desktop 再执行相应回收，macOS 才能拿回符合条件的物理块，时间取决于 Docker Desktop 版本和磁盘镜像实现。

因此要分别测量 Docker 内部逻辑空间和 macOS 物理容量，直接删除 `Docker.raw` 等同于销毁整个 Docker 环境。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/docker-raw-growth.webp" width="1360" height="454" loading="lazy" alt="稀疏 Docker.raw 文件随来宾虚拟机写入而增长，并通过 discard 与压缩把符合条件的块归还给 macOS。">
  <figcaption>删除 Docker 对象先释放虚拟机内部空间，之后才可能把物理空间归还 macOS。</figcaption>
</figure>

## 磁盘地图能看到什么

[Mole](https://mole.fit/zh/) 的「分析」页可以发现虚拟磁盘及其实际物理占用，Docker 命令行则负责判断内部对象的引用关系，前者回答空间在哪里，后者回答里面是什么。

## 安全的操作顺序

一个较容易复核的顺序是：运行 `docker system df -v`，确认旧项目和有状态数据卷，导出唯一数据，再逐类清理，并在每一步复查 Docker 统计和 macOS 容量，最容易安全回收的通常是旧构建缓存，而不是身份不明的数据卷。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-clean-up-docker-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
