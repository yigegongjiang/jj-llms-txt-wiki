---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xzareghjk568
---

# MINI AP本地web管理后台用户手册

更新时间：2026-06-25 18:03:47

# AP发射模式的MINI AP的本地web管理后台

## 1. 登录

（\*进入本地web管理后台的方法和首次配置流程请见：[MINI AP用户手册](https://docs.sunmi.com/zh-CN/ceghjk502/xzazeghjk557)）

（\*从DMP平台进入MINI AP的本地web管理后台时，无须登录）

-   使用电脑浏览器，通过AP发射模式的MINI AP的IP地址，进入登录页；


![](https://cdn.sunmi.com/public/image/mgt-document/8f1966e8ea884411ae1da402482118aa.png)

-   设备的SN、固件版本、当前处于AP/Client模式，始终可以在页面底部查看；

-   页面右上角可以切换语言，当前支持简体中文、英文两种语言；

-   输入设备的管理密码，点击“登录”；


## 2. 首页

![](https://cdn.sunmi.com/public/image/mgt-document/57b491cc2f47421aa59a49d68370b7ab.png)

-   顶部网络拓扑图，可以查看设备当前网络状况、设备名称（点击可修改）、AP模式说明；

-   无线连接：可查看Wi-Fi名称、密码、无线频宽、无线信道、Wi-Fi MAC等参数；

-   有线连接：可查看LAN IP地址、子网掩码、默认网关、DNS服务器、LAN MAC等参数；

-   SUNMI Link：用于自动发现并连接附近的商米客户端；


## 3. 客户端管理

-   用于查看此时连接本台AP发射模式的MINI AP的所有客户端设备的信息；

-   列表右上角可刷新查看最新连接的客户端设备列表；


![](https://cdn.sunmi.com/public/image/mgt-document/84869a74c9314336beed50c53dbb1d2c.png)

## 4. 系统设置

![](https://cdn.sunmi.com/public/image/mgt-document/3ccef44fe3c64cb89ffa5cbddbd2f27a.png)

### 4.1. Wi-Fi设置

-   用于查看和修改Wi-Fi名称、密码、加密信息等参数；

    ![](https://cdn.sunmi.com/public/image/mgt-document/d823a7498d084c41a4ef02bb7f6ecda9.png)


### 4.2. 有线设置

-   用于查看和设置设备的动态IP和静态IP参数；


![](https://cdn.sunmi.com/public/image/mgt-document/b2a20c63de28454da219f0ff4941dea2.png)

### 4.3. DHCP设置

-   用于查看和修改DHCP服务器的相关参数；


![](https://cdn.sunmi.com/public/image/mgt-document/34fd6620bdfa4005baaf91b725e5cfb9.png)

### 4.4. 修改管理密码

-   用于修改设备的管理密码；

-   管理密码每次登录本地web管理后台时均需要，请务必妥善保管你的密码！切勿随意对外泄漏！


![](https://cdn.sunmi.com/public/image/mgt-document/d91738f7de6b49fda42f7f87ecac903f.png)

### 4.5. 备份与恢复

-   备份系统配置：将当前设备的配置参数导出一份到电脑里；导出的文件请妥善保存，且不要修改文件内容；


![](https://cdn.sunmi.com/public/image/mgt-document/46b74f7b9a1c47d1914a328c2ae3f820.png)

-   备份文件：

    ![](https://cdn.sunmi.com/public/image/mgt-document/aa784f66d74b49b694cc962f2367bccd.png)


-   从本地恢复：日后需要恢复为该配置时，可直接选择该备份文件导入；


![](https://cdn.sunmi.com/public/image/mgt-document/0b96f36751e049cbb483cbc0bb0711a9.png)

-   恢复配置需要一段时间，设备请勿断电；

-   恢复过程中设备会重启断网，此时连接该AP的所有终端设备均会断连，失去网络；配置恢复完成后连接也会自动恢复；


![](https://cdn.sunmi.com/public/image/mgt-document/96ca887768d74e34912d027795af6b26.png)

### 4.6. 在线系统升级

-   用于查看和修改设备当前的自动升级策略；

-   可查看和检查当前是否有可用的新固件；若有，可选择该固件进行升级；

-   升级过程中设备会重启断网，所有连接这台AP的Client接收模式的MINI AP和终端均会断开连接，失去网络连接；

-   升级成功后会自动恢复连接；


![](https://cdn.sunmi.com/public/image/mgt-document/7582ac83ce07480396b0695b5cfe97a3.png)

### 4.7. 本地系统升级

-   用于从电脑里选择升级文件来升级设备；

-   升级文件请向对接你的商米技术支持/售后同事获取；

-   升级过程中设备会重启断网，所有连接这台AP的Client接收模式的MINI AP和终端均会断开连接，失去网络连接；

-   升级成功后会自动恢复连接；


![](https://cdn.sunmi.com/public/image/mgt-document/84d0df54f92341d793fac42dc9018cda.png)

### 4.8. 恢复出厂设置

-   用于恢复设备出厂设置；


![](https://cdn.sunmi.com/public/image/mgt-document/f7d42b2305034e419b928e8a796252c4.png)

-   恢复出厂设置过程中设备会重启；

-   重启过程中设备会断网。所有连接这台AP的Client接收模式的MINI AP和终端均会断开连接，失去网络连接；


![](https://cdn.sunmi.com/public/image/mgt-document/440cfd8362174538a5a0cb923353972f.png)

-   恢复出厂设置后，需要再次对设备进行首次配置，首次配置流程请见：[MINI AP用户手册](https://docs.sunmi.com/zh-CN/ceghjk502/xzazeghjk557)


### 4.9. 设备重启

-   用于重启设备；

-   重启过程中设备会断网。所有连接这台AP的Client接收模式的MINI AP和终端均会断开连接，暂时失去网络连接；

-   重启成功后会自动恢复重启前的连接；


![](https://cdn.sunmi.com/public/image/mgt-document/e2baed13b4d04c73a56fb89bc9f6303a.png)

### 4.10. 时间设置

-   用于查看和设置设备的系统时间；


![](https://cdn.sunmi.com/public/image/mgt-document/6e18f79cd28c40d7a4d5f5c32ee3730a.png)

### 4.11. 闲时重启

-   用于开启、关闭和设置设备的闲时重启策略；

-   定期重启有助于清除设备缓存，避免缓存过多影响设备使用；

-   建议将重启时间设置在业务低峰期或休息期；


![](https://cdn.sunmi.com/public/image/mgt-document/416fd3f7ba0e486f952047d433b97186.png)

### 4.12. 系统日志

-   用于查看设备的系统日志，可导出；


![](https://cdn.sunmi.com/public/image/mgt-document/317605e8ced94e2da507631ff30c4382.png)

# Client接收模式的MINI AP的本地web管理后台

## 1. 登录

（\*进入本地web管理后台的方法和首次配置流程请见：[MINI AP用户手册](https://docs.sunmi.com/zh-CN/ceghjk502/xzazeghjk557)）

（\*从DMP平台进入MINI AP的本地web管理后台时，无须登录）

-   使用电脑浏览器，通过Client接收模式的MINI AP的IP地址，进入首页；


![](https://cdn.sunmi.com/public/image/mgt-document/fd94fad4c26841b1b68751736c7ce1b9.png)

-   设备的SN、固件版本、当前处于AP/Client模式，始终可以在页面底部查看；

-   页面右上角可以切换语言，当前支持简体中文、英文两种语言；

-   输入设备的管理密码，点击“登录”；


## 2. 首页

![](https://cdn.sunmi.com/public/image/mgt-document/94e10fa2f5cc4a6985ad7284fb7cec4b.png)

-   顶部网络拓扑图，可以查看设备当前网络状况、设备名称（点击可修改）、Client模式说明；

-   无线连接：可查看前端Wi-Fi名称、前端Wi-Fi密码、无线频宽、无线信道、Wi-Fi MAC等参数；

-   有线连接：可查看LAN IP地址、子网掩码、默认网关、DNS服务器、LAN MAC等参数；

-   SUNMI Link：Client模式下该功能不可用；


## 3. Wi-Fi状态

-   用于查看这台Client接收模式的MINI AP此时连接的前端AP发射模式的MINI AP的Wi-Fi信息；


![](https://cdn.sunmi.com/public/image/mgt-document/966a5ea2d1014c4c8a3a81dcd9b6828c.png)

## 4. 系统设置

![](https://cdn.sunmi.com/public/image/mgt-document/2afb104d78cf48208454188b726da75e.png)

### 4.1. Wi-Fi设置

-   用于查看设备当前连接的前端AP的Wi-Fi信息；

-   若有多个可使用的AP发射模式的MINI AP，可以直接输入对应AP的Wi-Fi名称和密码，点击“保存”进行连接（请输入正确的前端AP Wi-Fi名称和密码，否则会无法连接成功）；


![](https://cdn.sunmi.com/public/image/mgt-document/8a214f32445b408d8df7aba7034bbb4c.png)

-   若有多个可使用的AP发射模式的MINI AP，也可以点击“自动扫描”，搜索并选择你想要连接的前端AP Wi-Fi；


![](https://cdn.sunmi.com/public/image/mgt-document/5e4af19199824389b7d225896a35b0f2.png)

-   在找到的前端AP Wi-Fi列表中，选择需要连接的前端AP Wi-Fi，点击后会出现密码输入框。输入密码后点击“加入网络”，即可连接对应的前端AP（若无密码则选择后可直接连接）；


![](https://cdn.sunmi.com/public/image/mgt-document/a75cb94b0c7e400397fcbcbb281c832c.png)

### 4.2. 其他设置项

其他设置项的内容均和AP发射模式的MINI AP的设置内容一致，请见上文AP发射模式的功能介绍。

---

上一篇：MINI AP用户手册
下一篇：MINI AP商米助手App用户手册
