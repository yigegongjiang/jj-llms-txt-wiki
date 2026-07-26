---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzrqeghjk513
---

# 常见故障排查手册
更新时间：2026-07-20 15:18:50
# 12. 常见故障排查手册
本章节整理了 TMS 系统在部署和运行过程中可能遇到的常见错误及其解决方法。
* * *
## 1. EMQX 服务启动失败：`kernel pid terminated`
### 1.1 现象说明
EMQX 容器启动后自动退出，运行 `docker logs emqx` 查看日志时，出现类似以下错误：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{"init terminating in do_boot", {error, {badmatch, {error, {econnrefused, [{node, 'emqx@localhost'}, ...
kernel pid terminated (init terminating in do_boot)
  

```

### 1.2 原因分析
这是由于主机的主机名（Hostname）被设置为 `localhost` 导致的，EMQX 节点的 Erlang 运行时无法成功解析并分配此主机名。
### 1.3 解决方法
修改系统的主机名为其他合法名称（例如 `tms-server` 或您自定义的主机名）：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 1. 临时且永久性修改主机名（将 tms-server 替换为实际需要的主机名）
sudo hostnamectl set-hostname tms-server

# 2. 编辑 /etc/hosts 确认本地解析包含新的主机名
# 确保 127.0.0.1 后面映射了您新设置的主机名

# 3. 重启 EMQX 服务
cd /data && docker compose restart emqx
  

```

* * *
## 2. Docker-Compose 运行时报 `libz.so.1` 错误
### 2.1 现象说明
在运行 `docker-compose` 时报错：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Failed to write all bytes for libz.so.1
...
dlopen: cannot map segment from shared object: Operation not permitted
  

```

### 2.2 原因分析
独立二进制版 `docker-compose`（通常是 PyInstaller 打包的程序）在运行时，需要将自身的 `.so` 共享库文件（如 `libz.so.1`）解压释放到系统的 `/tmp` 目录下并进行调用。 如果您的操作系统出于安全考量，对 `/tmp` 分区挂载了 `noexec` 属性（禁止在 `/tmp` 执行二进制程序），就会发生该报错。
### 2.3 解决方法
通过环境变量配置，让其解压到其他允许执行程序的临时目录（例如 `/var/tmp` 或您的工作目录下）：
  1. **临时生效** （在当前终端运行）：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
export TMPDIR=/var/tmp
# 然后继续执行您的 docker-compose 命令
  

```

  2. **永久生效** ： 将该环境变量写入系统 profile 文件中：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee -a /etc/profile.d/tms_temp.sh << 'EOF'
export TMPDIR=/var/tmp
EOF

source /etc/profile.d/tms_temp.sh
  

```



* * *
## 3. Kubernetes / Ingress-Nginx 导入数据报 `413 Request Entity Too Large`
### 3.1 现象说明
如果您的 TMS 部署在 Kubernetes 集群中，并通过 Ingress-Nginx 暴露服务。在导入大型 SQL 或上传应用包等操作时，页面报错：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
413 Request Entity Too Large
  

```

### 3.2 原因分析
Ingress-Nginx 默认限制客户端上传的文件大小（通常默认为 1MB）。当传输的文件包大于此限制时，Ingress 会直接拦截请求并返回 413 状态码。
### 3.3 解决方法
修改对应的 Kubernetes Ingress 资源配置，添加 `client-body-buffer-size` 和 `proxy-body-size` 相关的注解（Annotation），提升文件限制上限：
在您的 Ingress 配置文件中追加以下 `annotations`：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: tms-ingress
  annotations:
    # 允许上传的最大包大小（此处以 50m 为例）
    nginx.ingress.kubernetes.io/proxy-body-size: "50m"
    # 上传缓存区大小
    nginx.ingress.kubernetes.io/client-body-buffer-size: "20m"
# 其他配置项保持不变...
  

```

保存后应用配置即可：`kubectl apply -f <your-ingress-file>.yaml`。
* * *
## 4. 解决 Docker 网段冲突导致内网无法连接问题
### 4.1 现象说明
部署 Docker 后，服务器原本能够访问的某些内网 IP 地址突然无法访问，或者部分容器服务无法与外部正常通信。
### 4.2 原因分析
Docker 默认分配的桥接网段（通常为 `172.17.0.0/16`）与您内网的实际物理网络 IP 段发生了冲突，导致路由重叠，外部流量直接被路由到本地 Docker 虚拟网卡 `docker0` 上了。
### 4.3 解决方法
自定义修改 Docker 的默认网段（bip），避开您企业内网使用的物理网段：
  1. **配置**`daemon.json` ： 在宿主机修改或新建 `/etc/docker/daemon.json` 文件：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 备份旧配置（若存在）
[ -f /etc/docker/daemon.json ] && sudo cp /etc/docker/daemon.json /etc/docker/daemon.json.bak

# 写入新参数（将 172.30.0.1/24 替换为您内网不冲突的网段，并开启日志轮转限制）
sudo tee /etc/docker/daemon.json << 'EOF'
{
  "bip": "172.30.0.1/24",
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "500m",
    "max-file": "3"
  }
}
EOF
  

```

  2. **重新加载并重启 Docker 服务** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo systemctl daemon-reload
sudo systemctl restart docker
  

```

  3. **检查网段是否生效** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
ip addr show docker0
# 或者旧系统使用: ifconfig docker0
  

```

当看到 `docker0` 网卡的 IP 地址已变更为您新配置的地址段（如 `172.30.0.1`），说明配置生效。


* * *
## 5. 常用 Docker & Docker Compose 排查命令手册
在日常运维或故障排查时，熟练使用 Docker 和 Docker Compose 命令可以帮助您快速定位容器状态、资源瓶颈以及服务运行问题。以下是常用的排查命令汇总：
> **操作须知** ：故障排查命令（尤其是涉及服务重启、容器删除、磁盘清理的操作）具有一定的风险。在执行任何写操作或清理命令前，请务必确认操作的影响范围，并在必要时对核心配置和数据库进行备份。
### 5.1 容器运行状态排查
  * **查看所有运行中的容器** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker ps
  

```

  * **查看所有容器（包括已停止的）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker ps -a
  

```

> 如果某个服务持续报错或无法访问，可使用此命令检查其容器的 `STATUS` 是否为 `Exited (非0值)`。
  * **查看 Docker Compose 服务状态** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 在 /data 部署目录下运行
docker compose ps
# 或旧版本：docker-compose ps
  

```



### 5.2 容器日志查看与分析
  * **实时查看指定容器的全部日志** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker logs -f <容器名称或ID>
# 示例：查看 EMQX 日志
docker logs -f emqx
  

```

  * **查看最后 N 行的日志** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker logs --tail 100 <容器名称或ID>
  

```

  * **查看特定时间段的日志（例如最近30分钟）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker logs --since 30m <容器名称或ID>
  

```

  * **查看 Docker Compose 中所有或特定服务的日志** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 查看所有服务日志（实时滚动）
docker compose logs -f
# 查看特定服务日志
docker compose logs -f <服务名称>
# 示例：查看 MySQL 服务的日志
docker compose logs -f mysql
  

```



### 5.3 容器交互与文件操作
  * **进入容器内部执行命令行交互** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker exec -it <容器名称或ID> /bin/sh
# 如果容器内包含 bash
docker exec -it <容器名称或ID> /bin/bash
# 示例：进入 Redis 容器
docker exec -it redis sh
  

```

> 进入容器内进行调试时，请勿随意修改容器内的配置文件或系统参数，容器重启后未挂载的修改将会丢失。若需修改配置，请优先修改宿主机的映射文件。
  * **使用 Docker Compose 进入服务容器** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker compose exec <服务名称> sh
  

```

  * **在宿主机和容器之间复制文件** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 从宿主机复制文件到容器内
docker cp <宿主机路径> <容器名称或ID>:<容器内路径>
# 从容器内复制文件到宿主机
docker cp <容器名称或ID>:<容器内路径> <宿主机路径>
  

```



### 5.4 服务状态控制与重启
  * **重启单个容器** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker restart <容器名称或ID>
  

```

  * **重启 Docker Compose 下的所有/指定服务** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 重启所有服务
docker compose restart
# 重启指定服务（如只重启业务容器 tms）
docker compose restart tms
  

```

> 重启服务会导致短暂的业务中断，请尽量选择在业务低峰期或维护窗口进行操作。
  * **停止并清理容器与网络** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 停止并删除当前 Compose 定义的所有容器、网络（数据卷默认会保留）
docker compose down
  

```

> **警告** ：`docker compose down` 会立即停止并删除所有运行中的容器及虚拟网络，导致系统服务彻底中断（即出现 Downtime）。在生产环境中切勿在业务高峰期或未授权情况下执行此命令。
  * **重新编译/拉取镜像并后台启动服务** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker compose up -d
  

```



### 5.5 系统资源与磁盘清理
  * **查看容器资源占用实时统计（CPU、内存、网络、磁盘 I/O）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker stats
  

```

  * **查看 Docker 磁盘空间占用详情** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker system df
  

```

  * **安全清理未使用的 Docker 资源（释放磁盘空间）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 清理所有已停止的容器、未使用的网络、悬空的镜像
docker system prune

# [慎用] 深度清理：清理所有未被任何容器使用的镜像、未被使用的卷和网络
# 注意：在确认不需要保留旧的未挂载卷时使用，避免误删挂载了重要数据的匿名卷
docker system prune -a --volumes
  

```

> **高危警告** ：`docker system prune` 和 `docker system prune -a --volumes` 是破坏性命令。
>     * `docker system prune` 会删除**所有已停止的容器** 。如果有容器因为报错而停止，且您还未保存其内部日志或未导出的数据，这些数据将被永久删除！
>     * 添加 `-a` 或 `--volumes` 会清除所有未使用的镜像与数据卷。如果误删了未挂载的数据库匿名卷，将导致**数据丢失且无法恢复** ！执行前必须进行数据备份，并确认没有重要的遗留数据。


上一篇：系统版本升级与更新
下一篇：WinRE备份还原操作说明
