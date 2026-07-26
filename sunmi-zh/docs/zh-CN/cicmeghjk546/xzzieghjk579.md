---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzieghjk579
---

# Nginx 负载均衡配置
更新时间：2026-07-05 00:10:46
# Nginx 负载均衡配置
当采用集群部署，或需要统一做 SSL 证书卸载（SSL Termination）时，推荐使用 Nginx 作为前置负载均衡服务（LB）。
* * *
## 1. 端口与协议映射关系  
| 外部监听端口  | 协议类型  | Nginx 上游转发  | 转发协议  | 目标容器端口  | 业务说明  |  
| --- | --- | --- | --- | --- | --- |  
| **443**  | HTTPS (SSL 终结)  | `tms_web`  | HTTP  | 81  | TMS 管理平台 Web 访问  |  
| **8088**  | HTTPS (SSL 终结)  | `tms_api`  | HTTP  | 8088  | TMS API 服务及 WebSocket 接口  |  
| **8883**  | TCP+SSL (SSL 终结)  | `tms_emqx`  | TCP  | 1883  | 终端 MQTT 长连接 (加密)  |  
| **3478**  | TCP  | `webrtc`  | TCP  | 3478  | 远程协助控制通道 (TCP)  |  
| **3478**  | UDP  | `webrtc`  | UDP  | 3478  | 远程协助控制通道 (UDP)  |  
* * *
## 2. 安装 Nginx
由于需要用到 `stream` 和 `stream_ssl` 模块来进行 MQTT 及 WebRTC 端口代理，建议通过源码编译安装 Nginx。
### 2.1 创建运行用户与组
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo groupadd nginx
sudo useradd nginx -g nginx -s /sbin/nologin -M
  

```

### 2.2 安装编译依赖
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# Ubuntu / Debian
sudo apt update && sudo apt install -y gcc make zlib1g zlib1g-dev libpcre3 libpcre3-dev openssl libssl-dev wget

# CentOS / RHEL (供参考)
# sudo yum install -y gcc make zlib-devel pcre-devel openssl-devel wget
  

```

### 2.3 下载与编译安装
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cd /usr/local/src/
sudo wget -O "nginx-1.30.1.tar.gz" https://tms-packages.oss-cn-hangzhou.aliyuncs.com/config/nginx-1.30.1.tar.gz
sudo tar -xf nginx-1.30.1.tar.gz
cd nginx-1.30.1

# 配置编译参数
sudo ./configure --prefix=/usr/local/nginx --user=nginx --group=nginx 
  --with-http_ssl_module 
  --with-http_stub_status_module 
  --with-http_v2_module 
  --with-http_gzip_static_module 
  --with-http_realip_module 
  --with-http_sub_module 
  --with-stream 
  --with-stream_ssl_module 
  --with-stream_realip_module

# 编译并安装
sudo make && sudo make install

# 修改安装目录权限
sudo chown -R nginx:nginx /usr/local/nginx
  

```

### 2.4 配置系统环境变量
执行以下命令，将 Nginx 路径追加到系统环境变量中：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee -a /etc/profile.d/nginx.sh << 'EOF'
export NGINX_HOME=/usr/local/nginx
export PATH=$PATH:$NGINX_HOME/sbin
EOF

# 使环境变量生效
source /etc/profile.d/nginx.sh
  

```

### 2.5 配置 Systemd 服务管理
  1. **写入服务配置文件** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /lib/systemd/system/nginx.service << 'EOF'
[Unit]
Description=The NGINX HTTP server
After=syslog.target network.target remote-fs.target nss-lookup.target

[Service]
Type=forking
PIDFile=/usr/local/nginx/logs/nginx.pid
ExecStartPre=/usr/local/nginx/sbin/nginx -t
ExecStart=/usr/local/nginx/sbin/nginx
ExecReload=/bin/kill -s HUP $MAINPID
ExecStop=/bin/kill -s TERM $MAINPID
PrivateTmp=true

[Install]
WantedBy=multi-user.target
EOF
  

```

  2. **启动并设置开机自启** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo systemctl daemon-reload
sudo systemctl enable nginx --now

# 验证运行状态
systemctl status nginx
  

```



* * *
## 3. Nginx 配置说明
Nginx 默认配置文件路径为 `/usr/local/nginx/conf/nginx.conf`。为了结构清晰，我们采用模块化配置。
### 3.1 覆盖 `nginx.conf` 主配置
备份并覆盖主配置文件：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo cp /usr/local/nginx/conf/nginx.conf /usr/local/nginx/conf/nginx.conf-$(date +%F_%H-%M)

sudo tee /usr/local/nginx/conf/nginx.conf << 'EOF'
user  nginx;
worker_processes  auto;

error_log  logs/error.log;
pid        logs/nginx.pid;

worker_rlimit_nofile 65535;

events {	
    use epoll;
    worker_connections  65535;
}

http {
    include       mime.types;
    default_type  application/octet-stream;

    log_format  main  '$remote_addr $remote_user [$time_local] "$request" '
                      '$status $body_bytes_sent "$http_referer" '
                      '$http_user_agent $http_x_forwarded_for $request_time $upstream_response_time $upstream_addr $upstream_status';

    access_log  logs/access.log  main;

    sendfile        on;
    charset utf-8;
    server_tokens off;
    tcp_nopush     on;
    client_header_buffer_size    20m;
    large_client_header_buffers  4 2048k;
    client_max_body_size 20m;
    proxy_buffering off;
    proxy_buffer_size 64k;
    proxy_buffers   4 32k;
    proxy_busy_buffers_size 64k;
    proxy_temp_file_write_size 64k;
    proxy_ignore_client_abort  on;
    keepalive_timeout  65;
    gzip_min_length 1k;
    gzip_buffers 4 16k;
    gzip_http_version 1.1;
    gzip_comp_level 2;
    gzip_types text/plain text/css application/json application/x-javascript text/xml application/xml application/xml+rss text/javascript image/jpeg image/gif image/png application/javascript;
    gzip_proxied any;
    gzip_disable "MSIE [1-6]\.";

    # 载入 HTTP 配置
    include /usr/local/nginx/conf/vhosts/http.d/*.conf;
}

stream {
    log_format proxy '$remote_addr [$time_local] '
                     '$protocol $status $bytes_sent $bytes_received '
                     '$session_time "$upstream_addr" '
                     '"$upstream_bytes_sent" "$upstream_bytes_received" "$upstream_connect_time"';

    access_log /usr/local/nginx/logs/tcp-access.log proxy;
    open_log_file_cache off;

    # 载入 TCP/UDP 配置
    include /usr/local/nginx/conf/vhosts/tcp.d/*.conf;
}
EOF
  

```

### 3.2 配置子目录及证书存放路径
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo mkdir -p /usr/local/nginx/conf/certs
sudo mkdir -p /usr/local/nginx/conf/vhosts/http.d
sudo mkdir -p /usr/local/nginx/conf/vhosts/tcp.d
  

```

> [!NOTE] 请将您的 SSL 证书（如 `cert.pem` / `key.pem`）上传至 `/usr/local/nginx/conf/certs/` 目录下。
### 3.3 Upstream 负载均衡源站配置 (`upstreams.conf`)
创建并编辑 `/usr/local/nginx/conf/vhosts/http.d/upstreams.conf`，定义服务集群上游节点：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /usr/local/nginx/conf/vhosts/http.d/upstreams.conf << 'EOF'
upstream tms_web {
    ip_hash;
    server 10.10.1.1:81  weight=1 max_fails=3  fail_timeout=10s;  
    server 10.10.1.2:81  weight=1 max_fails=3  fail_timeout=10s;
    # 如果是单机部署，请删除多余的 server 节点及 ip_hash 配置
}

upstream tms_api {
    ip_hash;
    server 10.10.1.1:8088  weight=1 max_fails=3  fail_timeout=10s;  
    server 10.10.1.2:8088  weight=1 max_fails=3  fail_timeout=10s;
} 

upstream tms_api_socket {
    # WebSocket 与核心指令接口建议绑定到单个节点以保障会话连续性
    server 10.10.1.1:8088  weight=1 max_fails=3  fail_timeout=10s;  
}

upstream tms_emqx {
    server 10.10.1.1:18083  weight=1 max_fails=3  fail_timeout=10s;
    server 10.10.1.2:18083  weight=1 max_fails=3  fail_timeout=10s;
}
EOF
  

```

### 3.4 默认安全拦截配置 (`defaults.conf`)
为了防止未经授权的域名解析或恶意扫描，创建并编辑 `/usr/local/nginx/conf/vhosts/http.d/defaults.conf`，默认拒绝所有未匹配域名的请求：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /usr/local/nginx/conf/vhosts/http.d/defaults.conf << 'EOF'
server {
    listen 443 ssl default_server;
    server_name _; 
    
    # 默认拦截证书路径
    ssl_certificate /usr/local/nginx/conf/certs/default.pem;
    ssl_certificate_key /usr/local/nginx/conf/certs/default.key;
    
    return 403; 
}

server {
    listen 8088 ssl default_server;
    server_name _;
    
    ssl_certificate /usr/local/nginx/conf/certs/default.pem;
    ssl_certificate_key /usr/local/nginx/conf/certs/default.key;
    
    return 403;
}

server {
    listen 18083 ssl default_server;
    server_name _;
    
    ssl_certificate /usr/local/nginx/conf/certs/default.pem;
    ssl_certificate_key /usr/local/nginx/conf/certs/default.key;
    
    return 403;
}
EOF
  

```

> [!NOTE] 默认拦截配置需要 `/usr/local/nginx/conf/certs/default.pem` 与 `default.key` 证书文件存在，您可以通过 openssl 快速生成一套自签名证书以供初始化启动。
### 3.5 Web 访问代理配置 (`tms-web.conf`)
创建并编辑 `/usr/local/nginx/conf/vhosts/http.d/tms-web.conf`，代理 Web 前端页面：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /usr/local/nginx/conf/vhosts/http.d/tms-web.conf << 'EOF'
server {
    listen 443 ssl;
    server_name tms-web.example.com; # 请根据实际域名进行修改

    ssl_certificate      /usr/local/nginx/conf/certs/cert.pem;
    ssl_certificate_key  /usr/local/nginx/conf/certs/key.pem;
    ssl_session_timeout 5m;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE:ECDH:AES:HIGH:!NULL:!aNULL:!MD5:!ADH:!RC4;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    server_tokens off;

    access_log /usr/local/nginx/logs/tms_web_access.log main;

    location / {
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_pass http://tms_web;
    }
}
EOF
  

```

### 3.6 API 与 WebSocket 代理配置 (`tms-api.conf`)
创建并编辑 `/usr/local/nginx/conf/vhosts/http.d/tms-api.conf`，代理接口请求、终端远程认证与 WebSocket 链接：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /usr/local/nginx/conf/vhosts/http.d/tms-api.conf << 'EOF'
server {
    listen 8088 ssl;
    server_name tms-api.example.com; # 请根据实际域名进行修改

    ssl_certificate      /usr/local/nginx/conf/certs/cert.pem;
    ssl_certificate_key  /usr/local/nginx/conf/certs/key.pem;
    ssl_session_timeout 5m;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE:ECDH:AES:HIGH:!NULL:!aNULL:!MD5:!ADH:!RC4;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    server_tokens off;

    access_log /usr/local/nginx/logs/tms_api_access.log main;

    location / {
        proxy_pass http://tms_api;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # 终端接入鉴权
    location = /mdm/mrc/auth/accessToken {
        proxy_pass http://tms_api_socket;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # 远程控制管理
    location = /web/v1/mdm/remote/device/applyControl {
        proxy_pass http://tms_api_socket;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # 转发 WebSocket (Socket.io) 链接
    location ^~ /socket.io {
        proxy_pass http://tms_api_socket;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
EOF
  

```

### 3.7 EMQX Dashboard 管理端代理配置 (`tms-emqx.conf`)
创建并编辑 `/usr/local/nginx/conf/vhosts/http.d/tms-emqx.conf`，代理 EMQX 控制后台页面：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /usr/local/nginx/conf/vhosts/http.d/tms-emqx.conf << 'EOF'
server {
    listen 18083 ssl;
    server_name tms-emqx.example.com; # 请根据实际域名进行修改

    ssl_certificate      /usr/local/nginx/conf/certs/cert.pem;
    ssl_certificate_key  /usr/local/nginx/conf/certs/key.pem;
    ssl_session_timeout 5m;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE:ECDH:AES:HIGH:!NULL:!aNULL:!MD5:!ADH:!RC4;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    server_tokens off;

    access_log /usr/local/nginx/logs/tms_emqx_dashboard_access.log main;

    location / {
        proxy_pass http://tms_emqx;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
EOF
  

```

### 3.8 TCP/UDP 代理配置 (`vhosts/tcp.d/tms-emqx.conf`)
创建并编辑 `/usr/local/nginx/conf/vhosts/tcp.d/tms-emqx.conf`，在 `stream` 模块下完成 MQTT 加密长连接监听 (8883) 与 WebRTC 远程控制端口 (3478) 的四层转发：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo tee /usr/local/nginx/conf/vhosts/tcp.d/tms-emqx.conf << 'EOF'
upstream tms_emqx_tcp {
    server 10.10.1.1:1883 fail_timeout=1s max_fails=1;
    server 10.10.1.2:1883 fail_timeout=1s max_fails=1;
}

upstream webrtc_tcp_udp {
    # 主备负载均衡，保障远程连接质量
    server 10.10.1.1:3478 fail_timeout=10s max_fails=3;
    server 10.10.1.2:3478 fail_timeout=10s max_fails=3 backup;
}

# 1. EMQX 终端长连接四层代理与 SSL 卸载 (Port: 8883)
server {
    listen 8883 ssl;
    ssl_certificate     /usr/local/nginx/conf/certs/cert.pem;
    ssl_certificate_key /usr/local/nginx/conf/certs/key.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_handshake_timeout 15s;

    proxy_pass tms_emqx_tcp;
}

# 2. WebRTC 远程控制通道 TCP 代理 (Port: 3478)
server {
    listen 3478;
    proxy_pass webrtc_tcp_udp;
    proxy_connect_timeout 1s;
    proxy_buffer_size 4k;
}

# 3. WebRTC 远程控制通道 UDP 代理 (Port: 3478)
server {
    listen 3478 udp;
    proxy_pass webrtc_tcp_udp;
    proxy_connect_timeout 1s;
    proxy_buffer_size 4k;
}
EOF
  

```

### 3.9 重新加载 Nginx 配置
完成所有配置后，测试配置文件的正确性并重载 Nginx 服务：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sudo nginx -t
sudo systemctl reload nginx
  

```

* * *
## 4. 选配：跨域资源共享 (CORS) 配置
在多域名部署模式下（例如 Web 端使用 `tms-web.example.com`，而 API 端使用 `tms-api.example.com`），由于协议、域名或端口不同，浏览器的同源策略（Same-Origin Policy）会默认拦截 AJAX 请求。
如果页面控制台出现跨域限制报错（如 `CORS Policy: Access-Control-Allow-Origin ...`），您需要在 Nginx API 配置中添加跨域相应头。
### 配置方法
编辑 `/usr/local/nginx/conf/vhosts/http.d/tms-api.conf`，在 `location /` 以及各特定的接口 location 块中追加以下响应头配置：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
# 允许来自前端 Web 域名的跨域请求（请修改为您的实际前端域名）
add_header Access-Control-Allow-Origin 'https://tms-web.example.com' always;
add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS, PUT, DELETE' always;
add_header Access-Control-Allow-Headers 'DNT,X-Mx-ReqToken,Keep-Alive,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Authorization' always;
add_header Access-Control-Allow-Credentials 'true' always;

# 拦截并快速响应浏览器预检 OPTIONS 请求，直接返回 204
if ($request_method = 'OPTIONS') {
    return 204;
}
  

```

* * *
## 5. 选配：单域名多端口部署方案
如果您的企业环境中不方便申请多个子域名，可以使用同一个主域名（如 `tms.example.com`），配合不同的端口来区分并部署各个服务。
### 配置方法
保持现有的多配置文件结构不变，仅将所有配置文件（`tms-web.conf`、`tms-api.conf`、`tms-emqx.conf`）中的 `server_name` 统一修改为同一个主域名即可：
  * `tms-web.conf`：将 `server_name` 修改为 `tms.example.com;` (监听 443 端口)
  * `tms-api.conf`：将 `server_name` 修改为 `tms.example.com;` (监听 8088 端口)
  * `tms-emqx.conf`：将 `server_name` 修改为 `tms.example.com;` (监听 18083 端口)


> ‼️ **仍然存在跨域 (CORS)** ：因为浏览器判断跨域的依据是“协议、主机名、端口”三者。虽然域名统一了，但由于各服务监听的端口不同，在浏览器眼中仍然属于跨域请求。因此，您仍需按照**第 4 节** 在 `tms-api.conf` 中追加跨域配置以保证接口能够被正常访问。
上一篇：集群高可用部署方案
下一篇：EMQX 长连接 SSL 证书配置
