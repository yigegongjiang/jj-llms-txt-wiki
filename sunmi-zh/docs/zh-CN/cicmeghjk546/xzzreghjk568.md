---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzreghjk568
---

# 集群高可用部署方案
更新时间：2026-07-04 23:50:58
# 集群高可用部署方案
本章节介绍多台服务器的集群部署方案，主要包含 **NFS 共享存储配置** 和 **EMQX 消息队列集群配置** 。
* * *
## 1. 共享存储方案选择
在多台服务器集群部署中，为了保证多台服务器上的 TMS 容器挂载的是同一个物理磁盘（主要用于确保各节点间 `apk` 包、`ota` 升级包以及其他公共上传资源的数据完全一致），需要配置共享存储。
> **存储方案对比与推荐** ：
>   * **云服务器环境（云上部署 - 强烈推荐）** ：直接使用云服务商提供的**云共享存储（如阿里云 NAS、腾讯云 CFS、AWS EFS 等）**挂载至各台服务器。不仅配置简便，且自带高可用、弹性扩容及极高的读写性能。
>   * **物理机房/私有化部署（生产环境 - 强烈推荐）** ：采用 **GlusterFS 分布式高可用共享存储（方案一）** 。它采用多副本数据镜像，没有单点故障，并且内置自动愈合和客户端故障自动转移。
>   * **物理机房/私有化部署（简易测试开发环境）** ：采用 **NFS 共享存储（方案二）** 。配置简单、开销低，但属于单点存储，**如果 NFS 服务端宕机，整个集群的共享存储将处于不可用状态** ，请勿在生产高可用环境中使用。
> 

* * *
### 1.1 方案一：GlusterFS 高可用共享存储配置（私有化最佳实践）
本方案使用三节点组成的 GlusterFS 集群，其中两节点配置为数据复制副本，第三节点仅用作**仲裁（Arbiter）以防止两节点网络分区时的脑裂** 问题。
#### 1.1.1 节点规划示例  
| 角色  | 主机 IP  | 存储块物理路径 (Brick Path)  | 本地共享挂载点  |  
| --- | --- | --- | --- |  
| **复制节点 1**  | `10.10.1.1`  | `/data/glusterfs/brick1/brick`  | `/data/tms/public`  |  
| **复制节点 2**  | `10.10.1.2`  | `/data/glusterfs/brick1/brick`  | `/data/tms/public`  |  
| **仲裁节点 3**  | `10.10.1.3`  | `/data/glusterfs/brick1/brick`  | (仅起表决作用，无需作为应用挂载点)  |  
> **生产规范警告** ：
>   * **独立分区** ：生产环境中，所有的存储块物理路径（如 `/data/glusterfs/brick1`）必须挂载在**独立的物理磁盘或分区** 上，并推荐使用 `XFS` 文件系统。严禁直接使用系统根分区。
>   * **防火墙开放端口** ：如果启用了防火墙，需要在各节点间开放以下端口：
>     * TCP `24007` (Glusterd 服务端口)
>     * TCP `24008` (Gluster 管理端口)
>     * TCP `49152` 到 `49154` (视创建的 Brick 数量而定，每个 Brick 占用一个端口)
>     * 若节点位于安全的可信局域网内，建议直接对集群内互信 IP 放开所有流量。
> 

#### 1.1.2 安装并启动 GlusterFS（三节点均需执行）
根据操作系统选择对应的安装命令：
  * **Ubuntu / Debian** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo apt update
sudo apt install -y software-properties-common
sudo add-apt-repository ppa:gluster/glusterfs-9 -y
sudo apt update && sudo apt install -y glusterfs-server
sudo systemctl enable glusterd --now
  

```

  * **CentOS / RHEL** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo yum install -y centos-release-gluster9
sudo yum install -y glusterfs-server
sudo systemctl enable glusterd --now
  

```



#### 1.1.3 配置集群与创建复制卷
  1. **节点互联（在 10.10.1.1 上执行即可）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 探测其他两台节点以建立可信池
sudo gluster peer probe 10.10.1.2
sudo gluster peer probe 10.10.1.3

# 查看集群对等体状态
sudo gluster peer status
  

```

  2. **准备存储块目录（三节点均需执行）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 创建用于存储实际数据的目录（若有独立分区，应先将其挂载到 /data/glusterfs/brick1）
sudo mkdir -p /data/glusterfs/brick1/brick
  

```

  3. **创建带仲裁的复制卷（在 10.10.1.1 上执行即可）** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 创建名为 gv0 的卷，包含 2 个副本和 1 个仲裁节点
sudo gluster volume create gv0 replica 3 arbiter 1 
  10.10.1.1:/data/glusterfs/brick1/brick 
  10.10.1.2:/data/glusterfs/brick1/brick 
  10.10.1.3:/data/glusterfs/brick1/brick
  
# 启动卷
sudo gluster volume start gv0

# 查看卷状态信息
sudo gluster volume info
  

```

  4. **调优文件读写性能（可选，推荐在 10.10.1.1 上执行）** ： 为了优化 OTA 升级包大文件的读取与并发下载速度，建议开启以下策略：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo gluster volume set gv0 performance.read-ahead on
sudo gluster volume set gv0 performance.io-cache on
sudo gluster volume set gv0 performance.write-behind on
  

```



#### 1.1.4 客户端挂载与自动故障转移（在应用服务器 10.10.1.1 和 10.10.1.2 上执行）
  1. **安装客户端挂载工具** ：
     * Ubuntu / Debian：`sudo apt install -y glusterfs-client`
     * CentOS / RHEL：`sudo yum install -y glusterfs-fuse`
  2. **创建挂载点** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo mkdir -p /data/tms/public
  

```

  3. **配置高可用开机挂载** ： 配置 `backupvolfile-server` 选项，当挂载节点本地的 `glusterd` 进程出现问题时，客户端将自动漂移至另一个正常的备份节点，保证读写不中断。
编辑 `/etc/fstab`：
     * **在节点 1 (**`10.10.1.1`**) 的**`/etc/fstab`**中追加** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
localhost:gv0  /data/tms/public  glusterfs  defaults,_netdev,backupvolfile-server=10.10.1.2  0  0
  

```

     * **在节点 2 (**`10.10.1.2`**) 的**`/etc/fstab`**中追加** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
localhost:gv0  /data/tms/public  glusterfs  defaults,_netdev,backupvolfile-server=10.10.1.1  0  0
  

```

  4. **执行挂载** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo mount -a
  

```



* * *
### 1.2 方案二：NFS 共享存储配置（单点简易部署）
此方案适用于开发测试或非高可用架构，部署步骤仅限两台机器：一台充当 NFS 服务端，另一台作为客户端。
#### 1.2.1 节点规划示例  
| 角色  | 主机 IP  | 共享目录路径  |  
| --- | --- | --- |  
| **NFS 服务端** (`nfs-server`)  | `10.10.1.1`  | `/data/tms/public`  |  
| **NFS 客户端** (`nfs-client`)  | `10.10.1.2`  | `/data/tms/public`  |  
#### 1.2.2 NFS 服务端配置 (`10.10.1.1`)
  1. **安装 NFS 服务端并创建共享目录** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo apt update && sudo apt install -y nfs-kernel-server nfs-common
sudo mkdir -p /data/tms/public
  

```

  2. **配置共享目录访问权限** ： 向 `/etc/exports` 中追加允许访问的客户端 IP（以 `10.10.1.2` 为例）：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee -a /etc/exports << 'EOF'
/data/tms/public/ 10.10.1.2(rw,sync,no_root_squash,no_all_squash)
EOF
  

```

  3. **启动并启用 NFS 服务** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo systemctl enable nfs-kernel-server --now
sudo exportfs -rv
  

```



#### 1.2.3 NFS 客户端配置 (`10.10.1.2`)
  1. **安装 NFS 客户端工具并创建本地挂载点** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo apt update && sudo apt install -y nfs-common
sudo mkdir -p /data/tms/public
  

```

  2. **测试与挂载共享目录** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 测试是否可以访问服务端的共享目录
showmount -e 10.10.1.1

# 手动挂载目录
sudo mount -t nfs -o nolock 10.10.1.1:/data/tms/public /data/tms/public
  

```

  3. **配置开机自动挂载** ： 将自动挂载配置追加到 `/etc/fstab`：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee -a /etc/fstab << 'EOF'
10.10.1.1:/data/tms/public  /data/tms/public  nfs  nolock  0  0
EOF
  

```



* * *
## 2. EMQX 集群配置
在多机集群部署下，多台主机的 EMQX 节点需要建立集群，保证各节点间的消息能够实时互通。
### 2.1 节点规划示例  
| 主机服务名  | 主机 IP  | 节点名称 (Node Name)  |  
| --- | --- | --- |  
| `emqx1`  | `10.10.1.1`  | `emqx@10.10.1.1`  |  
| `emqx2`  | `10.10.1.2`  | `emqx@10.10.1.2`  |  
### 2.2 节点 1 配置 (`emqx1`)
编辑 `emqx1` 服务器上的 `/data/docker-compose.yml`，在 `emqx` 服务定义中追加以下内容：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
services:
  emqx:
    # 启用 host 网络模式
    network_mode: host
    environment:
      - EMQX_NAME=emqx
      - EMQX_NODE__NAME=emqx@10.10.1.1
      - EMQX_HOST=10.10.1.1
      - EMQX_CLUSTER__DISCOVERY=static
      # 种子节点列表，需包含所有集群节点的 Node Name
      - EMQX_CLUSTER__STATIC__SEEDS=emqx@10.10.1.1,emqx@10.10.1.2
      # 必须保证集群内所有节点的 Cookie 完全一致
      - EMQX_NODE_COOKIE=2qigcjawUYYo6T
  

```

### 2.3 节点 2 配置 (`emqx2`)
编辑 `emqx2` 服务器上的 `/data/docker-compose.yml`，同样进行对应的集群参数配置：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
services:
  emqx:
    network_mode: host
    environment:
      - EMQX_NAME=emqx
      - EMQX_NODE__NAME=emqx@10.10.1.2
      - EMQX_HOST=10.10.1.2
      - EMQX_CLUSTER__DISCOVERY=static
      - EMQX_CLUSTER__STATIC__SEEDS=emqx@10.10.1.1,emqx@10.10.1.2
      - EMQX_NODE_COOKIE=2qigcjawUYYo6T
  

```

> **缩进与格式警告** ：在 YAML 格式的 `docker-compose.yml` 中追加环境变量时，请务必保证正确的空格缩进，避免语法错误。
### 2.4 启动与验证集群
  1. **按顺序启动 EMQX 节点** ： 在 `emqx1` 宿主机执行：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data && docker compose up -d
  

```

随后在 `emqx2` 宿主机执行：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data && docker compose up -d
  

```

  2. **检查启动日志** ： 在各节点检查 EMQX 服务是否正常启动：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker logs -f emqx
  

```

  3. **验证集群建立状态** ： 在任意一台服务器上运行以下命令，查询集群状态：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker exec emqx emqx_ctl cluster status
  

```

**预期正常输出示例** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Cluster status: [{emqx@10.10.1.1,running},{emqx@10.10.1.2,running}]
  

```

如上所示，当看到所有节点状态均为 `running` 时，代表集群已成功建立。


上一篇：部署验证与连接测试
下一篇：Nginx 负载均衡配置
