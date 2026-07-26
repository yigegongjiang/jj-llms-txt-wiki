---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzzzeghjk557
---

# 部署验证与连接测试

更新时间：2026-07-04 23:49:38

## 测试 TMS 部署是否成功

浏览器访问：`http://<您的域名>:<端口>` (默认端口：81) 或 `https://<您的域名>:<端口>`

用户名：admin

密码：Admin123

登录后修改密码。

![](https://cdn.sunmi.com/public/image/mgt-document/b8d79e7a84ff4faba79b130fb76c1403.png)

添加分类测试

![](https://cdn.sunmi.com/public/image/mgt-document/132079eb6e3346dd94c680ef0d8574ce.png)

添加应用测试

![](https://cdn.sunmi.com/public/image/mgt-document/d1e18964392b483ab9fcf12a9eeb6520.png)

如上两图操作正常，TMS 测试程序正常。

## 浏览器输入 EMQX 登入域名进入程序

浏览器访问：`http://<您的EMQX域名>:<端口>` (默认端口：18083) 或 `https://<您的EMQX域名>:<端口>`

用户名：admin

密码：oWaGzH0HynpZRkiUYFzgrzG9sAys883u

登陆后修改密码。

![](https://cdn.sunmi.com/public/image/mgt-document/57b9bbcc46ed4181bfd262671263e8c3.png)

## EMQX 测试证书是否成功

### Windows 测试

下载地址：[MQTTX 下载官方页面](https://www.emqx.com/zh/downloads/MQTTX) （或特定版本下载链接，如将版本替换为实际需要的版本：`https://www.emqx.com/zh/downloads/MQTTX/v-example/MQTTX-Setup-example-x64.exe`）

#### 测试 1883 端口

将 Host 替换为公网 IP。

![](https://cdn.sunmi.com/public/image/mgt-document/d2475c5e7bca4831b673124e6b8b1146.png)

![](https://cdn.sunmi.com/public/image/mgt-document/ed4c3fe87eb249819e42ae3a33038a92.png)

#### 测试 8883 端口

> **注意：**
> 测试前提是 emqx 已配置证书，可通过域名访问。

将 Host 替换为 emqx 域名。

![](https://cdn.sunmi.com/public/image/mgt-document/3a3182801e404160a7014b6151a7bb29.png)

![](https://cdn.sunmi.com/public/image/mgt-document/f2888a739cf24a1185851d074565d6d0.png)

### Linux 测试

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
# 下载 MQTTX CLI（请将 v-example 替换为实际需要的版本，如 v1.9.6）
curl -LO https://www.emqx.com/en/downloads/MQTTX/v-example/mqttx-cli-linux-x64
sudo install ./mqttx-cli-linux-x64 /usr/local/bin/mqttx

# 将 tms-example.sunmi.com 替换为您的实际 EMQX 域名
/usr/local/bin/mqttx conn -h 'tms-example.sunmi.com' -l mqtt -p 1883 -i tms.link
/usr/local/bin/mqttx conn -h 'tms-example.sunmi.com' -l mqtts -p 8883 -i tms.link
```

![](https://cdn.sunmi.com/public/image/mgt-document/1d3f2a6a9e8849959a008557d14b08e0.png)

---

上一篇：环境准备与单机服务部署
下一篇：集群高可用部署方案
