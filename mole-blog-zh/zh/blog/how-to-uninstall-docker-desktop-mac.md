# 卸载 Docker Desktop，别把命名数据卷一起清掉

> 卸载前先备份命名数据卷，再分清 Docker 配置目录和虚拟机磁盘状态，哪些该留、哪些可删

Published: 2026-07-27 | Updated: 2026-09-24

Docker Desktop 不是一个单独的应用程序，它包含图形界面、后台虚拟机、挂到 `/usr/local/bin` 的命令行工具，还有一块会悄悄涨到几十 GB 的磁盘镜像，把它拖进废纸篓只去掉了界面，其余都还在。

它还有个别处少见的地方：官方卸载程序本身就是会删数据的那一步，事后清理反倒无关痛痒，下面按能保住数据的顺序来，并说明哪些残留可丢、哪些以后还想要回来。

开始删除支持文件之前，先对照 [Docker Desktop 的现行官方卸载说明](https://docs.docker.com/desktop/uninstall/)。如果厂商提供自己的卸载器或应用内命令，应先走那条路径，再处理明确属于它的残留。

## 卸载会销毁容器、镜像和数据卷

Docker 明确说明，卸载 Docker Desktop 会删除容器、镜像、数据卷以及其他本机 Docker 数据，不经过废纸篓。不能把确认对话框当成备份或恢复机制。

只有远端镜像或完整构建来源仍在，镜像和容器才可以重新取得或创建。数据卷和容器可写层都可能保存唯一数据，包括 Postgres 或 MySQL 的本地数据库，不能只备份带名字的数据卷。先检查实际存储位置，再决定保留什么。

在动手之前，先列出已有内容：

```
docker volume ls
```

某个数据卷重要的话，在卸载前备份。数据库优先用自身的导出或备份工具；下面这种文件打包方式要先停止向数据卷写入的容器，否则归档可能不一致，还应验证能否还原：

```
docker run --rm -v <volume>:/from -v "$PWD":/to alpine \
  tar czf /to/<volume>.tgz -C /from .
```

从未推送到远端的自建镜像同理，`docker image ls` 能看出本机有什么，不在仓库里的就只存在于这台 Mac。

## 若只是为了腾空间，未必需要卸载

Docker 的本地引擎数据主要放在虚拟磁盘镜像里。只想腾空间时，可以保留 Docker，先用 `docker system df` 看各项占比，再按需清理；prune 仍是不可撤销的删除，不是可逆替代。具体范围与磁盘镜像的回收方式见 [在 Mac 上清理 Docker，不必丢掉数据](https://mole.fit/zh/blog/how-to-clean-up-docker-mac)。

如果目标是彻底卸掉 Docker，继续往下。

## 用 Docker 自带的卸载程序

Docker 自带卸载程序，做的不只是删掉应用程序包，它会卸下后台服务，并去掉访达拖拽卸载后仍会留下的命令行符号链接。

从应用里操作的话，打开 Docker Desktop，点右上角疑难解答图标，选 Uninstall 再确认，在终端里则运行：

```
/Applications/Docker.app/Contents/MacOS/uninstall
```

然后把 Docker 从应用程序移到废纸篓。

Docker 文档仅说明卸载时访问容器内特定元数据文件出现的 `operation not permitted` 可以忽略，不能把所有同名错误都当作成功。要核对报错路径和官方示例；其他路径或退出失败仍需排查。

## 还剩什么，分别归谁

卸载程序跑完后仍会留下一些目录，Docker 点名了两个：

```
~/Library/Group Containers/group.com.docker
~/.docker
```

它们不是同一类东西，差别正是关键。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/docker-uninstall-boundary.webp" width="1360" height="454" loading="lazy" alt="卸载程序会直接销毁镜像、容器和命名数据卷，不经过废纸篓；Docker 配置目录、Group Containers 目录，以及普通缓存仍会留在磁盘上。">
  <figcaption>卸载程序本身就会删数据。只有远端副本或完整构建来源仍在时，镜像和容器才可重建，数据卷和容器内未保存的改动要另行备份。</figcaption>
</figure>

`~/.docker` 保存上下文、客户端设置，以及仓库认证或凭据辅助工具的配置，密码本身可能由其他存储管理。若以后重装 Docker，或改用 Colima、Rancher Desktop、OrbStack，先备份并检查这些配置，删除后可能需要重设连接和登录。

`~/Library/Containers/com.docker.docker` 是应用的容器目录，虚拟机和它的磁盘镜像 `Docker.raw` 都在里面，几十 GB 就在这里，上面说的 `operation not permitted` 受保护目录也是它；`~/Library/Group Containers/group.com.docker` 放的是 Docker Desktop 的设置，不是磁盘镜像。既然已接受容器和数据卷都没了，这两处就是值得清掉的部分。

除了这两处，Docker 还可能在 Library 下留下缓存、日志、偏好设置和应用状态。确认相关任务已停止、设置不再需要、日志也不用排障后再处理，不要只凭文件名含有 `docker` 就删除。

## 确认它真的卸干净了

做两项检查，先确认没有进程还在跑：

```
pgrep -fl -i docker
```

再确认命令行工具已断开链接：

```
which docker docker-compose
```

两者都应该没有输出，若 `docker` 仍能解析到路径，多半是单独用 Homebrew 装过命令行工具，那是另一套包，按设计会在 Docker Desktop 卸载后继续存在，`brew list | grep docker` 可以确认。

## 更快一次看清全部残留

手动路径可行，但前提是你已经知道有哪些路径、哪条才是数据，多数指南恰恰跳过这一步，而出错时代价也最大。

[Mole](https://mole.fit/zh/) 会列出它能识别的相关文件，按类别显示位置和大小。先核对目录内容，取消勾选要保留的项目；卸载所选文件会移到废纸篓，但无法恢复之前已被 Docker 官方卸载器删除的数据。

## 重装之后哪些会回来

有远端镜像或完整构建来源时，才能重新拉取或构建镜像，再创建容器。未推送的自建镜像、容器可写层里的改动和数据卷都可能是唯一副本，不会因为重装就回来，卸载前要逐项备份。

保留 `~/.docker` 有助于保留 context 和客户端配置，但登录能否继续使用还取决于凭据存储和有效期。删除它可能也会丢失自定义配置，不只是多登录一次。

虚拟磁盘镜像会重新创建，但是空的，它的体积会随着你重新拉镜像慢慢涨回去，所以「卸载 Docker 腾出几十 GB」这件事，在你重装并恢复日常使用之后基本会还原，真想长期把这块压住，得靠定期 `docker system prune`，而不是卸载重装。

Mole 核对这个应用时会列出什么、又会留下什么，写在 [在 Mac 上卸载 Docker Desktop](https://mole.fit/zh/tested-apps/docker)。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-uninstall-docker-desktop-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
