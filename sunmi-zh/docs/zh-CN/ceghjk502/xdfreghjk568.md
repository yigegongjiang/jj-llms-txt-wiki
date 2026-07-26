---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xdfreghjk568
---

# FLEX 3
更新时间：2026-04-07 19:03:12
## **1、FLEX 3 产品介绍**
FLEX 系列采用模块化设计，支持灵活组合与个性化配置，满足多元化商业应用，为门店带来高效便捷的解决方案。
FLEX 系列分为高配版、标配版、KDS版三个不同的版本。[更多产品详情](https://www.sunmi.com/zh-CN/flex-3/)  
|  ![](https://cdn.sunmi.com/public/image/mgt-document/12013a7dc6ba4d50861214f5a6cd3917.png)  |  ![](https://cdn.sunmi.com/public/image/mgt-document/37d6cd0796cd44558ce07a39b615cb32.png)  |  
| --- | --- |  
  * 技术规格
    * 高配版、标配版
![](https://cdn.sunmi.com/public/image/mgt-document/d2bbcabff65b491382112d534a20b126.png)
    * KDS版
![](https://cdn.sunmi.com/public/image/mgt-document/f6c4aada88a9483188cfb65c2d7cf449.png)


## **2、软件开发说明**
  * 开启设备的USB调试
    * [调试设备说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrzeghjk557/)
  * 设备开发集成
    * [如何获取设备信息](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdqieghjk579)
    * 打印
      * 参考：[打印SDK概览](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdzceghjk502/)
    * 扫码
      * [摄像头扫码说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfafeghjk535/)
      * [扫码头引擎（红外线扫码）](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfareghjk568/)
    * 主屏NFC
      * 参考谷歌官方文档，NFC API接口说明： [谷歌官方的安卓原生NFC API接口说明](https://developer.android.google.cn/reference/android/nfc/package-summary)
    * 指纹
      * [生物识别（指纹）开发指南](https://developer.sunmi.com/docs/preview/zh-CN/xzcxeghjk491)
    * 其他开发小贴士
      * [自定义音量键](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrieghjk579/)
      * [如何实现应用全屏显示](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrceghjk502/)
      * [如何避免重复申请外设权限](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrqeghjk513/)
      * [如何设置应用自动安装/更新](https://developer.sunmi.com/docs/zh-CN/cicmeghjk546/xcrieghjk579)
  * 应用发布
    * [商米应用市场发布应用说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/qaaeghjk480)


## **3、配件**  
| **配件名称**  | **涉及SDK说明文档**  |  
| --- | --- |  
| 三合一配件：扫码头  | [扫码头服务SDK](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfareghjk568/)  |  
| 三合一配件：NFC  | [NFC相关SDK说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xmaqeghjk513/)  |  
| 三合一配件：磁条卡服务  | [磁条卡服务说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xmaceghjk502/)  |  
| 3D摄像头  | [谷歌官方的安卓原生摄像头 API接口说明](https://developer.android.com/media/camera/camerax?hl=zh-cn)  |  
| 状态灯  | [状态灯服务说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xmaxeghjk491/)  |  
### **3.1 三合一配件**
##### **3.1.1简介**
  * **即插即用型多功能设备** ：通过单一硬件接口同时支持扫码、NFC及磁条卡三种读取功能
  * **典型应用场景：** 自助服务、餐饮行业、零售行业
##### **3.1.2 接口服务**
三合一配件涉及以下三种接口功能
  * **扫码：**[扫码头服务SDK](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfareghjk568/)
  * **NFC：**
    * NFC控制服务：[NFC相关SDK说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xmaqeghjk513/)
  * **磁条卡：**
    * 磁条刷卡模块服务：[磁条卡服务说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xmaceghjk502/)


### **3.2 3D摄像头**
##### **3.2.1 简介**
  * **配件功能：** 通过USB 2.0与主机通信的一款3D摄像头配件
  * **典型应用场景：** 人脸识别、人脸支付


##### **3.2.2 接口服务**
  * 参考谷歌官方文档，摄像头 API接口说明：[谷歌官方的安卓原生摄像头 API接口说明](https://developer.android.com/media/camera/camerax?hl=zh-cn)


### **3.3 状态灯**
##### **3.3.1 简介**
  * **配件功能：** 通过micro USB与主机进行连接的一款可控三色灯配件
  * **典型应用场景：** 硬件状态指示、用户交互引导、故障诊断


##### **3.3.2 接口服务**
  * 状态灯服务：[状态灯服务说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xmaxeghjk491/)


## **4、常见问题**
### **Q1. 如何安装FLEX 3？**
开始使用前，首先得找到设备背面，打开藏线盖可见外设接口，用于连接所需的外部设备。
  * 接口用途：
  * 3 个 USB Type A 端口
  * 用途：标准 USB 接口，用于连接鼠标、键盘、U 盘等外设。
  * 1 个 RJ45 网络接口
  * 用途：用于连接以太网网线，提供有线网络接入。
  * 4 个 Micro USB 端口
  * 用途：小型 USB 接口，配合主机的配件使用。
  * 1 个 Type C 调试端口
  * 用途：USB-C 接口，用于连接电脑进行设备调试或系统升级。
  * 1 个 POE 供电端口
  * 用途：Power over Ethernet（以太网供电），通过网线同时传输数据和电力。
  * 1 个 Type C 电源端口
  * 用途：USB-C 接口，用于为设备充电或外接电源适配器。


### **Q2. 如何对FLEX 3进行开机？**
  * 初次开启商米设备，完成初次设置只需简单几步：
  * 连接电源适配器，短按电源键开机启动，进入首次开机界面，点击【开始设置】按提示操作即可。本机可选择2种上网模式。
  * 连接WLAN
  * 点击要连接的WLAN，若选择一个加密的网络，则需要输入访问密码方可连接。
  * 通过LAN连网
  * 插入网线连接网络，网线接口位于底座背面塑料壳内，从底下缺口轻轻打开。
  * 如何下载／使用应用
  * 相关应用可在联网模式下，打开应用市场，浏览并下载应用即可完成安装。


上一篇：D1s
下一篇：FT2
