---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzmeghjk546
---

# 环境准备与单机服务部署
更新时间：2026-07-05 00:03:42
# 环境准备与单机服务部署
为了确保服务在高并发场景下的稳定运行，需要调整服务器的最大文件打开数及启用 IP 转发。
> **部署模式说明** ：本文档介绍单机环境下的 Docker 服务部署。如果您需要以集群（多台服务器）方式进行高可用部署，请优先查阅：[集群高可用部署方案](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzreghjk568)。
> **快速配置** ：您可以通过执行我们提供的自动化脚本一键完成系统参数配置：
> bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml
> 
```
curl -s https://tms-packages.oss-cn-hangzhou.aliyuncs.com/config/system_config.sh | sudo bash
  
> 
```

> 如果您想手动配置，请按以下步骤操作（需具备 root 或 sudo 权限）：
### 1. 修改系统内核参数 (`sysctl.conf`)
请执行以下命令，将文件数限制和 IP 转发配置追加到 `/etc/sysctl.conf` 中，并使之生效：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee -a /etc/sysctl.conf << 'EOF'

# 调整系统最大文件数限制
fs.file-max = 1048576
fs.nr_open = 1048576

# 开启 IP 转发（解决 Docker 容器间及外部通信流量转发问题）
net.ipv4.ip_forward = 1
EOF

# 使配置立即生效
sudo sysctl -p
  

```

### 2. 修改用户资源限制 (`limits.conf`)
执行以下命令，追加最大文件打开数和进程数限制：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee -a /etc/security/limits.conf << 'EOF'

# 调整用户资源限制
* soft nproc 1048576
* hard nproc 1048576
* soft nofile 1048576
* hard nofile 1048576
root soft nproc 1048576
root hard nproc 1048576
root soft nofile 1048576
root hard nofile 1048576
EOF
  

```

### 3. 修改安全限制配置 (`20-nproc.conf`)
执行以下命令，新建或覆盖限制配置文件：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /etc/security/limits.d/20-nproc.conf << 'EOF'
*       soft  nproc   1048576
root    soft  nproc   unlimited
EOF
  

```

> ‼️ **重要提示：使资源限制立即生效**
> 修改 `limits.conf` 和 `20-nproc.conf` 后，**当前已建立的 SSH 会话（bash）并不会自动加载新限制** 。如果直接在当前会话中运行后续的部署或测试命令，启动的进程依然会受到旧限制的约束。
> 请通过以下任一方式使配置生效：
>   1. **重新连接（推荐）** ：断开当前 SSH 连接，重新登录服务器。
>   2. **切换登录会话** ：在当前终端执行 `sudo -i` 或 `sudo su -` 重新登录到 root 账户，这将通过 PAM 模块加载新的资源限制。
>   3. **当前终端临时调整** ：如果是以 root 身份操作且不方便重连，可在当前终端直接运行以下命令（仅对当前终端及其子进程生效）：
> bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml
> 
```
ulimit -n 1048576
ulimit -u 1048576
  
> 
```

> 

> **验证生效方法** ：在启动部署前，在终端运行 `ulimit -n` 和 `ulimit -u`，确保其输出值已更新为 `1048576`。
* * *
# TMS 软件包准备与解压
在开始部署前，请先准备好您的 TMS 部署包。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 1. 创建并进入部署工作目录
mkdir -p /data && cd /data

# 2. [说明] 请在此步骤将您的 TMS.zip 部署包上传到服务器的 /data 目录下。

# 3. 解压部署包
unzip TMS.zip

# 4. 将核心配置文件和 SQL 导入脚本移动到工作目录根路径
cp TMS/docker-compose.yml .
cp TMS/tms.sql .

# 5. 确认文件结构
ls -lh /data/
  

```

* * *
# 安装 Docker 与 Docker-Compose
如果您目标服务器已安装 Docker（版本建议 ≥ 20.10.9$）及 Docker-Compose，可跳过此步。
### 1. 离线安装 Docker
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data

# 下载离线安装包
curl -O https://tms-packages.oss-cn-hangzhou.aliyuncs.com/docker/docker.zip
unzip docker.zip
cd docker

# 执行安装脚本（请将 docker-example.tgz 替换为解压出来的实际 tgz 文件名）
chmod +x install.sh
sudo sh install.sh docker-example.tgz
  

```

> 如需官方在线安装方式，请参考 Docker 官方文档：[Install Docker Engine | Docker Docs](https://docs.docker.com/engine/install/)。
### 2. 检查安装状态
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 检查 Docker 服务状态
systemctl status docker

# 确认 Docker 与 Docker Compose 版本（支持 v1 和 v2）
docker -v
docker compose version   # v2 插件格式（推荐）
# 或者旧版本：docker-compose -v
  

```

* * *
# 部署镜像与配置
### 1. 导入 TMS 依赖镜像
> **快速导入** ：您可以直接下载并执行脚本一键导入镜像： `curl -s https://tms-packages.oss-cn-hangzhou.aliyuncs.com/config/load_images.sh | bash`
**手动导入步骤** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data/TMS/tms-images

# 导入所有基础镜像与业务镜像
docker load -i emqx.tar
docker load -i mysql.tar
docker load -i redis.tar
docker load -i webrtc.tar.gz
docker load -i tms_example.tar.gz # 请将 tms_example.tar.gz 替换为您的实际版本文件名（如 tms_20231019.tar.gz）
  

```

### 2. 配置 docker-compose.yml
编辑 `/data/docker-compose.yml`，根据您的实际服务器网络和环境修改相关参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
vim /data/docker-compose.yml
  

```

**重点修改项说明：**
  * **MySQL 密码** ：建议修改 `MYSQL_ROOT_PASSWORD` 的默认密码。
  * **EMQX 外部访问 IP** ：检查并修改 EMQX 相关环境变量中的 IP 地址，确保与宿主机或内网 IP 一致。


> 详细修改位置可参考以下配置结构图：
![](https://cdn.sunmi.com/public/image/mgt-document/001b9491531542d192c81c7ede8f8733.png)
![](https://cdn.sunmi.com/public/image/mgt-document/18e70407e4564d5789d6f0ccb8e02f32.png)
* * *
# 启动 TMS 服务
在 `/data` 工作目录下执行以下命令启动所有容器：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data
docker compose up -d     # v2 格式（推荐）
# 或者旧版本：docker-compose up -d
  

```

* * *
# 导入数据库初始化脚本 (`tms.sql`)
请根据您的数据库部署类型（本地容器化部署或 PaaS 托管服务），选择以下一种导入方式：
### 方案 A：本地容器化部署的 MySQL
如果您的 MySQL 是通过上述 `docker-compose.yml` 在本地容器启动的，请执行：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 1. 创建数据库（若不存在）
docker exec -i mysql mysql -uroot -p"SunmiAdmin666" -e "CREATE DATABASE IF NOT EXISTS cpt_tms CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;"

# 2. 直接将宿主机的 tms.sql 导入容器中
docker exec -i mysql mysql -uroot -p"SunmiAdmin666" cpt_tms < /data/tms.sql
  

```

安全警告：部署完成后，请务必立即修改 MySQL 的默认 root 密码，避免安全风险。
### 方案 B：PaaS 托管数据库（如阿里云 RDS/云数据库）
如果您使用的是云服务商提供的托管 MySQL，我们可以通过运行临时的 Docker 容器来作为客户端执行导入：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 1. 确保远端数据库已创建（请将 <remote_host> 替换为您的云数据库连接地址）
docker run --rm -i mysql:latest mysql -h <remote_host> -uroot -p"SunmiAdmin666" -e "CREATE DATABASE IF NOT EXISTS cpt_tms CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;"

# 2. 将本地的 tms.sql 导入到远端数据库
docker run --rm -i mysql:latest mysql -h <remote_host> -uroot -p"SunmiAdmin666" cpt_tms < /data/tms.sql
  

```

_(您也可以直接使用 Navicat、DBeaver 等可视化数据库客户端连接至您的 PaaS 数据库，手动执行_`/data/tms.sql` _。)_
* * *
# 后续步骤与进阶配置
至此，单机环境下的 TMS 服务基础部署已完成。接下来您可以：
  1. **测试验证** ：点击查看 [部署验证与连接测试](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzzeghjk557) 确认服务是否工作正常。
  2. **进阶架构配置** ：
     * **集群高可用部署** ：参考 [集群高可用部署方案](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzreghjk568) 配置挂载 NFS 共享存储与 EMQX 消息队列集群。
     * **前置负载均衡** ：如果需要在前置配置反向代理或配置 SSL 证书卸载，请参考 [Nginx 负载均衡配置](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzieghjk579)。
     * **EMQX 证书直配** ：如果不使用前置负载均衡且需要开启 MQTT 的 MQTTS (8883) 端口，请参考 [EMQX 长连接SSL 证书配置](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzraeghjk480)。
  3. **日常运维管理** ：
     * **数据库日常维护** ：查阅 [数据库管理与日常维护](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzrxeghjk491) 配置本地自建数据库的定时自动备份脚本。
     * **系统版本升级** ：查阅 [系统版本升级与更新](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzrceghjk502) 了解如何平滑升级业务镜像版本。
  4. **故障排查** ：在部署或使用过程中如果遇到任何报错问题，请直接查阅 [常见故障排查手册](https://docs.sunmi.com/zh-CN/cicmeghjk546/xzrqeghjk513)。


上一篇：网络端口映射图解
下一篇：部署验证与连接测试
