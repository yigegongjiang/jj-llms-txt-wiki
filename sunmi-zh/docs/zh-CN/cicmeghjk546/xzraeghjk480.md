---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzraeghjk480
---

# EMQX 长连接 SSL 证书配置
更新时间：2026-07-04 23:52:55
# **EMQX 长连接 SSL 证书配置**
如果您**没有** 在架构前置部署负载均衡器（如 Nginx、F5 等）进行 SSL 卸载，您需要将 SSL 证书直接配置在 EMQX 容器内，以启用 MQTT 的加密长连接端口（8883）。
* * *
## 1. 拷贝出默认证书目录
我们可以利用已经拉取并运行的默认 EMQX 容器，将它的初始化证书目录拷贝到宿主机工作目录上，以便于修改和持久化：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 创建宿主机证书挂载目录
sudo mkdir -p /data/emqx

# 将容器内的证书目录拷贝到宿主机中
docker cp emqx:/opt/emqx/etc/certs/ /data/emqx/certs
  

```

* * *
## 2. 替换为您的域名证书
在宿主机对应的 `/data/emqx/certs` 目录中，将默认的证书文件内容替换为您自己域名的证书：
  1. **证书替换规则** ：
     * 将证书私钥（通常为 `.key` 文件）的内容写入：`/data/emqx/certs/key.pem`
     * 将服务端证书（通常为 `.crt` 或 `.pem` 文件）的内容写入：`/data/emqx/certs/cert.pem`
  2. **追加中继证书（CA Bundle）** ： 如果您使用的域名证书包含中继证书（`ca_bundle.crt`），请务必**将中继证书内容追加合并到服务端证书文件** `cert.pem` 中，否则移动端或终端设备连接时可能会报证书链信任异常（SSL Handshake Error）。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 合并服务端证书与中继证书
sudo cat server_cert.crt ca_bundle.crt > /data/emqx/certs/cert.pem
  

```



* * *
## 3. 修改 docker-compose.yml 挂载卷
编辑主工作目录下的 `/data/docker-compose.yml`，在 `emqx` 服务卷挂载配置（`volumes`）中追加以下映射配置：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
services:
  emqx:
    # 其他配置保持不变...
    volumes:
      # 将宿主机修改好的证书挂载覆盖容器内路径
      - /data/emqx/certs:/opt/emqx/etc/certs
  

```

* * *
## 4. 重启 EMQX 服务
挂载完成后，销毁旧的容器并重新拉起以载入证书配置：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /data

# 强行移除旧的 EMQX 容器（因为我们修改了挂载卷）
docker rm -f emqx

# 重新以守护进程启动服务
docker compose up -d
  

```

* * *
## 5. 验证 EMQX 启动状态
通过日志检查 EMQX 是否已正常加载证书并开始监听 8883 加密端口：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
docker logs -f emqx
  

```

**日志中如果包含类似以下信息，表示 SSL 证书加载成功** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
listener on ssl:8883 started
  

```

上一篇：Nginx 负载均衡配置
下一篇：数据库管理与日常维护
