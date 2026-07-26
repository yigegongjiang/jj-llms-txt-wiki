---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xzazeghjk557
---

# MINI AP用户手册

更新时间：2026-06-25 18:01:22

# 1. MINI AP设备使用说明

## a. 产品图

![](https://cdn.sunmi.com/public/image/mgt-document/ab1bf17b59cf47739a7f517ee5d794c5.png)

![](https://cdn.sunmi.com/public/image/mgt-document/ec7fae1d0feb463283f5fc875155434a.png)

设备SN、LAN MAC、默认Wi-Fi名称和密码，均可从机身铭牌上获取。

## b. 指示灯、按键和接口说明

![](https://cdn.sunmi.com/public/image/mgt-document/f5d0a6000c2d42b89fce67311d419fea.png)

<!-- prettier-ignore -->
| 电源指示灯 | 白色闪烁：系统启动中<br>白色常亮：系统正常工作 |
| --- | --- |
| **无线指示灯** | **AP发射模式**<br>蓝色常亮：无线正常工作<br>蓝色闪烁：等待Client接入<br>**Client接收模式**<br>绿色常亮：连接上AP，信号良好<br>绿色闪烁：搜索AP信号中<br>橙色常亮：连接上AP，信号不佳<br>熄灭：连接不上AP |

![](https://cdn.sunmi.com/public/image/mgt-document/2ba6098ba7e34821b6e802b067b2cd6d.png)

![](https://cdn.sunmi.com/public/image/mgt-document/3cde29f7ed0c4d9ca5babca5f7ce694c.png)

## c. 线缆连接和设备配对

![](https://cdn.sunmi.com/public/image/mgt-document/806ea5f1a4394f74a913c379886a68ab.png)

**AP端（发射模式）：**

1.  通过切换开关将MINI AP设置为AP发射模式(设备出厂默认为AP模式)；

2.  可以通过电源或路由器的USB口为MINI AP供电；

3.  使用网线，连接已通电通网的路由器的LAN口和MINI AP的LAN口；

4.  MINI AP将自动连上路由器并发出可用的Hyper Wi-Fi。


**Client端（接收模式）：**

1.  通过切换开关将MINI AP设置为Client接收模式；

2.  可以通过台式机或电源的USB口为MINI AP供电；

3.  使用网线，连接终端设备的LAN口和MINI AP的LAN口；

4.  此时观察Client端的MINI AP的无线指示灯是否绿色闪烁，如没有可以按下Client端的MINI AP无线配网按键，则对应无线指示灯会开始绿色闪烁，表示它正在搜索AP并连接。

5.  接着观察AP端的MINI AP的无线指示灯蓝色闪烁，如没有同样按下AP端的MINI AP无线配网按键，将自动配对Client端的MINI AP；


![](https://cdn.sunmi.com/public/image/mgt-document/8263159de1ae489783a373732fb2c0b1.png)

6\. 配对成功后，AP发射模式的MINI AP的无线指示灯会保持蓝灯常亮，Client端的MINI AP的无线指示灯会保持绿灯常亮；

-   AP的配对过程会持续5分钟，过程中蓝灯会持续闪烁，以支持多台Client接入；

-   如Client端的MINI AP无线指示灯显示为橙色，则表示能连接但信号不佳，如完全熄灭，则表示搜索不到信号连接不上，建议调整设备放置位置，尽量挂高以获得更好的无线覆盖。


7\. 配对成功后，终端设备即可使用Hyper Wi-Fi。

-   注意：此时Client接收模式的MINI AP会充当终端设备的外接网线，所以终端设备会被视为在使用有线网，因此终端设备的Wi-Fi列表里不会出现Hyper Wi-Fi这一无线网络。


# 2. MINI AP本地web管理后台

MINI AP本地web管理后台是MINI AP的设备后台，用于查看和设置设备的详细参数。

-   进入AP发射模式的MINI AP的管理后台，需要AP发射模式的MINI AP连接上路由器（Router）；

-   进入Client接收模式的MINI AP的管理后台，需要Client接收模式的MINI AP连接上AP发射模式的MINI AP。


## a. 进入AP发射模式的MINI AP本地web管理后台

（1）此时AP发射模式的MINI AP需已连上通电通网的路由器。使用电脑，连接该路由器的Wi-Fi；

（2）打开电脑浏览器，登录路由器的后台；

-   不同品牌路由器进入本地后台的方式不同，可咨询路由器设备厂家

-   本文以商米W1路由器举例


（3）在路由器的本地后台可以查看所有连接至本路由器的设备；

（4）在AP发射模式MINI AP的机身上，可以找到MINI AP的MAC地址。通过MAC地址，可以在连接路由器的设备列表里找到这台MINI AP，并获取到分配给MINI AP的IP地址；

![](https://cdn.sunmi.com/public/image/mgt-document/84e312571ada4a36866cd45af6e830da.png)

（5）保持电脑连接路由器的Wi-Fi，复制这个IP地址到浏览器地址输入框，前往该地址，即可进入AP发射模式的MINI AP的本地web管理后台。

### ⅰ. 首次进入本地web管理后台

-   首次配置时，勾选同意协议按钮，点击“开始设置”；

-   此时页面底部可以查看设备SN、固件版本号、当前为AP模式；


![](https://cdn.sunmi.com/public/image/mgt-document/abf6a07bb14b4b9dbea69120f562c815.png)

-   设置MINI AP的管理密码。设置后点击“下一步”；


-   管理密码每次登录本地web管理后台时均需要，请务必妥善保管你的密码！切勿随意对外泄漏！


![](https://cdn.sunmi.com/public/image/mgt-document/d860f0ec4085468fa28850993ddeb88d.png)

-   设置设备所属时区。设置后点击下一步；


![](https://cdn.sunmi.com/public/image/mgt-document/153fe65ffe7844d8aaac3e5df616ff04.png)

-   进入Wi-Fi设置页，你可以设置Wi-Fi名称、密码、加密信息等信息。设置后点击下一步；

-   AP发射模式下，MINI AP默认的Wi-Fi名称和密码同时也已标注在设备机身上；


![](https://cdn.sunmi.com/public/image/mgt-document/98bf268685ff42b6b4c92d040f6aa77a.png)

![](https://cdn.sunmi.com/public/image/mgt-document/8a11d71a6e5a484cb0cc7c70a1ee4cb6.png)

-   设置成功后，自动跳转至本地web管理后台；


![](https://cdn.sunmi.com/public/image/mgt-document/5f2c513604574d63bcdd4f8063abe224.png)

-   进入本地后台，可查看设备的全部具体参数，并可按需修改；


![](https://cdn.sunmi.com/public/image/mgt-document/40b21ceb431f44188b26999d1ded6e8b.png)

![](https://cdn.sunmi.com/public/image/mgt-document/93a02ce2a18f4deabce2f5093fb53adb.png)

![](https://cdn.sunmi.com/public/image/mgt-document/df155151d488488480af1b5ac7224643.png)

-   本地后台用户手册详见：[MINI AP本地web管理后台用户手册](https://docs.sunmi.com/zh-CN/ceghjk502/xzareghjk568)


### ⅱ. 再次进入本地web管理后台

-   首次配置流程只用执行一次，后续再次进入本地web管理后台时的界面如下：

-   输入首次配置流程中设置的管理密码，点击登录，即可进入本地web管理后台；


![](https://cdn.sunmi.com/public/image/mgt-document/4b1601128b4a4c5d90385c6b260d15a8.png)

## b. 进入Client接收模式的MINI AP本地web管理后台

-   流程和进入AP发射模式的MINI AP的本地web管理后台的流程大致相同；


（1）此时Client接收模式的MINI AP需已连上AP发射模式的MINI AP；AP发射模式的MINI AP需已连上通电通网的路由器。使用电脑，连接该路由器的Wi-Fi；

（2）打开电脑浏览器，登录路由器的后台；

（3）在路由器的本地后台可以查看所有连接至本路由器的设备；

（4）在Client发射模式MINI AP的机身上，可以找到MINI AP的MAC地址。通过MAC地址，可以在连接路由器的设备列表里找到这台MINI AP，并获取到分配给MINI AP的IP地址；

![](https://cdn.sunmi.com/public/image/mgt-document/f12f5f3793f44965bca6a0839f51c4f1.png)

-   此时在AP发射模式的MINI AP的本地web管理后台中，可以看到AP的客户端列表中也有这台Client接收模式的MINI AP，且IP地址相同；


![](https://cdn.sunmi.com/public/image/mgt-document/20c095346ed94e8e80b3deb991e9d73d.png)

（5）保持电脑连接路由器的Wi-Fi，复制这个IP地址到浏览器地址输入框，前往该地址，即可进入Client接收模式的MINI AP的本地web管理后台。

### ⅰ. 首次进入本地web管理后台

-   首次配置时，勾选同意协议按钮，点击“开始设置”；

-   此时页面底部可以查看设备SN、固件版本号、当前为Client模式；


![](https://cdn.sunmi.com/public/image/mgt-document/a687592175a147dab31ed26a9552792d.png)

-   设置MINI AP的管理密码。设置后点击“下一步”；


-   管理密码每次登录本地web管理后台时均需要，请务必妥善保管你的密码！切勿随意对外泄漏！


![](https://cdn.sunmi.com/public/image/mgt-document/7074774cbf93464d99582d0544049e7d.png)

-   设置设备所属时区。设置后点击下一步；


![](https://cdn.sunmi.com/public/image/mgt-document/947f737b6b52458184e27612f02282ba.png)

-   Wi-Fi设置页，会默认显示此时已连接的AP发射模式的MINI AP的Wi-Fi名称和密码；

-   若无需改动连接的前端AP，可直接点击下一步；

-   若有多个可使用的AP发射模式的MINI AP，可以直接输入对应AP的Wi-Fi名称和密码，点击“下一步”进行连接（请输入正确的前端AP Wi-Fi名称和密码，否则会无法连接成功）；


![](https://cdn.sunmi.com/public/image/mgt-document/c5273e8fd5104c7da685ec99d6f1b0e8.png)

-   若有多个可使用的AP发射模式的MINI AP，也可以点击“自动扫描”，搜索并选择你想要连接的前端AP Wi-Fi；


![](https://cdn.sunmi.com/public/image/mgt-document/0045353831a4427694195c650173c4fa.png)

-   在找到的前端AP Wi-Fi列表中，选择需要连接的前端AP Wi-Fi，点击后会出现密码输入框。输入密码后点击“加入网络”，即可连接对应的前端AP（若无密码则选择后可直接连接）；


![](https://cdn.sunmi.com/public/image/mgt-document/7f925e0ff56e4abeba6da6bdb1511768.png)

-   Wi-Fi设置完成后，自动跳转至本地web管理后台；


![](https://cdn.sunmi.com/public/image/mgt-document/55c5e2d8ba0c47feac2d18aa7554ef19.png)

-   进入本地后台，可查看设备的全部具体参数，并可按需修改；


![](https://cdn.sunmi.com/public/image/mgt-document/16cceda37c9043aa88684396f7adaf90.png)

![](https://cdn.sunmi.com/public/image/mgt-document/f5008bcb1bfe4884aa3d7d333fab05c2.png)

![](https://cdn.sunmi.com/public/image/mgt-document/18cb408fbe784afc9f588c414e51e982.png)

-   本地后台用户手册详见：[MINI AP本地web管理后台用户手册](https://docs.sunmi.com/zh-CN/ceghjk502/xzareghjk568)


### ⅱ. 再次进入本地web管理后台

-   首次配置流程只用执行一次，后续再次进入本地web管理后台时的界面如下：

-   输入首次配置流程中设置的管理密码，点击登录，即可进入本地web管理后台；


![](https://cdn.sunmi.com/public/image/mgt-document/f277812fa11c4cf180e2f937b98ebaa4.png)

---

上一篇：MINI AP产品参数
下一篇：MINI AP本地web管理后台用户手册
