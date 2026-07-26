---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzrceghjk502
---

# 系统版本升级与更新
更新时间：2026-07-04 23:54:25
# **系统版本升级与更新**
本章节介绍部署完成后的日常更新及版本升级流程。
* * *
## 1. 导入新的 TMS 业务镜像
  1. **解压更新包并导入镜像** ： 将获取到的更新镜像包上传至服务器工作目录，执行导入操作：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 1. 解压打包的镜像包（请将 example_tms_update 替换为实际下载的文件名）
tar -xf example_tms_update.tar.gz

# 2. 导入镜像到 Docker
docker load -i example_tms_update.tar
  

```

  2. **获取新镜像的版本标签 (Tag)** ： 运行以下命令，查找刚刚导入的 `tms` 镜像名称及版本 Tag：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker images | grep tms
  

```



* * *
## 2. 备份与更新 `docker-compose.yml`
在修改任何配置文件之前，务必进行备份：
  1. **备份当前配置文件** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data
cp docker-compose.yml docker-compose.yml-$(date +%F_%H-%M)
  

```

  2. **更新镜像标签** ： 编辑 `/data/docker-compose.yml`，在 `tms` 服务的配置段（`image`）中，修改镜像版本号为新导入的版本号：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
vim docker-compose.yml
  

```

**修改示例** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
services:
  tms:
    # 将此处 tag 更改为新版本（例如将 v1.0.0 修改为新版本 tag）
    image: tms:new-version-tag
  

```



* * *
## 3. 重启并验证服务
  1. **重建并运行新服务** ： 使用 Docker Compose 重建并运行受影响的容器：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 重新启动服务（Compose 会检测到 image 发生变更，只更新 tms 容器）
docker compose up -d
  

```

  2. **确认容器是否正常启动** ： 通过查看容器运行日志，验证升级是否成功，观察系统初始化是否有报错：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker logs -f tms
  

```



上一篇：数据库管理与日常维护
下一篇：常见故障排查手册
