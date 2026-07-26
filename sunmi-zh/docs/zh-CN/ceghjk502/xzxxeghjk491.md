---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xzxxeghjk491
---

# MINI AP商米助手App用户手册

更新时间：2026-06-25 18:05:57

使用商米助手App的免登录模式，无需创建企业、门店，现场人员即可快速配置和管理现场的MINI AP设备。

用手机相机扫码下载商米助手App（或在手机应用市场里搜索“商米助手”下载）；

![](https://cdn.sunmi.com/public/image/mgt-document/5758c4fc1dc440f8b984327e3f3ea483.png)

-   商米助手App需要更新至1.32.0及以上版本。


# 1. 前置工作

管理AP发射模式的MINI AP，需要设备已通电且连上已通电通网的路由器。

管理Client接收模式的MINI AP，需要设备已通电且和已通电通网的AP发射模式的MINI AP配对。

手机需要连接至AP发射模式的MINI AP连接的路由器的Wi-Fi下。

# 2. 查找和连接设备

进入商米助手，选择“免登录管理”（若已账号登录，则需要在“我的-我的设置”里，退出登录）。

![](https://cdn.sunmi.com/public/image/mgt-document/ace27741fde248d3bc935a5129df08fd.png)

阅读页面说明，勾选同意协议，点击“开始搜索”。

![](https://cdn.sunmi.com/public/image/mgt-document/f1d48af68b764c8fa43c8c67bdb2b9cd.png)

可以搜索到当前局域网内的全部MINI AP设备，对需要配置、管理的MINI AP，点击右侧的“连接”。

-   设备名称前的AP/Client表示设备此时所使用的模式；后接的4位字符为设备LAN MAC的后四位（可以在机身铭牌上查看设备的LAN MAC）；

-   SN为设备SN（可以在机身铭牌上查看设备的SN）；

-   IP为设备此时分配到的IP地址，可以通过该IP地址进入设备的本地web管理后台；


![](https://cdn.sunmi.com/public/image/mgt-document/1a8cdba49c8e41f5a8477d894974f120.png)

若是首次配置的设备，需要设置设备管理密码。

-   管理密码每次管理设备时均需要，请务必妥善保管你的密码！切勿随意对外泄漏！


-   如果忘记密码，请长按机身重置按键约5秒，等电源指示灯闪烁后松开，即可恢复出厂设置，重新设置管理密码。


![](https://cdn.sunmi.com/public/image/mgt-document/d548fd989a1941cea16cbbacb955fc9d.png)

# 3. 查看设备基础信息

连上设备后，可查看设备的基础信息，包括SN、固件版本、工作模式、有线连接信息等。

-   右上角可一键复制全部设备信息。


![](https://cdn.sunmi.com/public/image/mgt-document/0005c75ff1da4014b5dd95cd5bdb6fbd.png)

-   点击“展开无线连接信息”可展开查看无线连接信息。


![](https://cdn.sunmi.com/public/image/mgt-document/691371180b8443c68ea424362be840b5.png)

-   设备管理页下方提供了基础的设备管理功能。


# 4. 管理设备

## 4.1. 重启设备

点击后对设备完成重启。

-   重启过程中设备会断网。若重启的是AP发射模式的MINI AP，那么所有连接这台AP的Client接收模式的MINI AP和终端均会断开连接，暂时失去网络连接；

-   重启成功后会自动恢复重启前的连接；


## 4.2. 登录管理后台

点击可以进入设备的本地web管理后台。

![](https://cdn.sunmi.com/public/image/mgt-document/d4f932edca16482d90f3e324ea2508ec.png)

-   本地web管理后台用户手册：[MINI AP本地web管理后台用户手册](https://docs.sunmi.com/zh-CN/ceghjk502/xzareghjk568)

-   其中“备份恢复”和“本地升级”两个功能，由于需要操作本地文件，因此请使用电脑登录本地web管理后台来完成文件操作。


## 4.3. 修改管理密码

可修改设备的管理密码。

---

上一篇：MINI AP本地web管理后台用户手册
下一篇：MINI AP DMP用户手册
