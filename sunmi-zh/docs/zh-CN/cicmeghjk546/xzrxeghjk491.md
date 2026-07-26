---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzrxeghjk491
---

# 数据库管理与日常维护
更新时间：2026-07-04 23:53:41
# 数据库管理与日常维护
本章节介绍 TMS 系统数据库（MySQL 与 Redis）的维护规范，涵盖 **PaaS 云数据库连接准备与授权** 以及 **本地自建 MySQL 定时备份方案** 。
* * *
## 1. PaaS 云服务数据库准备与授权
如果选择云服务商托管的 PaaS 服务（如阿里云 RDS MySQL/云数据库 Redis），请遵循以下环境准备及授权指南。
### 1.1 PaaS MySQL 配置要求
  1. **基本规格要求** ：
     * 字符集建议：`utf8mb4`
     * 排序规则（Collation）：`utf8mb4_general_ci` 或 `utf8mb4_unicode_ci`


**数据库创建与账号授权** ： 连接至您的 PaaS 数据库管理端，执行以下 SQL 命令进行初始化：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
-- 1. 创建专用的 cpt_tms 数据库，并指定字符集为 utf8mb4
CREATE DATABASE IF NOT EXISTS `cpt_tms` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

-- 2. 创建专用应用管理用户（此处以 'tms_db_user' 作为用户，密码以 'Example_Password_123' 为例）
CREATE USER 'tms_db_user'@'%' IDENTIFIED BY 'Example_Password_123';

-- 3. 授予该账号对 cpt_tms 库的完全读写与管理权限
GRANT ALL PRIVILEGES ON `cpt_tms`.* TO 'tms_db_user'@'%';

-- 4. 刷新权限使其生效
FLUSH PRIVILEGES;
  

```

请务必将 Example_Password_123 替换为强密码。同时在 docker-compose.yml 中对应的 MySQL 环境变量里更新该账号密码。
### 1.2 PaaS Redis 配置要求
在云托管 Redis 实例中，请准备以下连接配置提供给部署程序：
  * **连接地址 (Host)** ：例如 `r-xxx.redis.rds.aliyuncs.com`
  * **连接端口 (Port)** ：默认 `6379`
  * **实例密码 (Password)** ：强密码，需填写至 `docker-compose.yml` 的 Redis 密码项。


* * *
## 2. 本地自建 MySQL 数据库自动备份方案
如果您使用的是本地容器化部署的 MySQL，为防止硬件损坏或误删除操作导致数据丢失，应在部署后配置定期备份脚本。
### 2.1 创建自动备份脚本
在宿主机上创建备份脚本 `/data/mysql_backup.sh`：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /data/mysql_backup.sh << 'EOF'
#!/bin/bash

# <mark></mark><mark></mark><mark></mark><mark></mark>= 配置区 <mark></mark><mark></mark><mark></mark><mark></mark>=
# 备份文件名追加的日期格式
DATE=$(date "+%Y%m%d")
# 宿主机上备份文件保存目录
MYSQL_DATA="/data/mysql_data_backup"
# MySQL 容器名称
CONTAINER_NAME="mysql"
# 备份保留份数最大上限
NUM_BAK=100
# MySQL 密码（请改为实际的 root 密码）
DB_PASSWORD="SunmiAdmin666"
# <mark></mark><mark></mark><mark></mark><mark></mark><mark></mark><mark></mark><mark></mark><mark></mark><mark></mark><mark></mark>==

mkdir -p ${MYSQL_DATA}

config_bak() {
  # 计算当前已备份的文件数量
  COUNT_BAK=$(ls -lrt ${MYSQL_DATA} | grep "mysql_" | wc -l)
  if [ ${COUNT_BAK} -gt ${NUM_BAK} ]
  then
    # 自动删除最早创建的备份文件以节省磁盘空间
    OLD_FILE=$(ls -lt ${MYSQL_DATA} | grep "mysql_" | tail -n 1 | awk '{print $9}')
    rm -f "${MYSQL_DATA}/${OLD_FILE}"
  fi
}

main(){
  # 调用 mysqldump 进行全库备份
  docker exec ${CONTAINER_NAME} sh -c "exec mysqldump --all-databases -uroot -p\"${DB_PASSWORD}\"" > ${MYSQL_DATA}/mysql_${DATE}.sql
  config_bak
}

main
EOF

# 赋予可执行权限
sudo chmod +x /data/mysql_backup.sh
  

```

### 2.2 配置定时任务 (Cron Job)
通过定时任务实现每周日凌晨 2:00 自动执行备份，保留 100 份的历史备份归档。
执行以下命令，自动将备份任务添加至系统 Cron 服务，避免通过交互式 `crontab -e` 带来配置错误：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 向当前用户 Cron 表中追加定时任务
(sudo crontab -l 2>/dev/null; echo "0 2 * * 0 /bin/bash /data/mysql_backup.sh >/dev/null 2>&1") | sudo crontab -
  

```

上一篇：EMQX 长连接 SSL 证书配置
下一篇：系统版本升级与更新
