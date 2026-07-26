---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmmeghjk546
---

# 调试说明（仅适用于T1设备）
更新时间：2025-11-10 15:51:11
**该文档说明仅适用于T1设备**
### 背景
  * 为什么需要通过网络调试？


目前T1主屏与副屏通过USB连接通信，主屏作为主设备，副屏作为从设备，USB外设也是从设备。当PC通过USB线连接主屏（或副屏）进行调试时，PC作为主设备，主屏（或副屏）会作为从设备，此时主屏与副屏的通信会断开，外接USB设备的连接也会断开。因此，PC通过连USB线只能单独调试主屏或是副屏，并且会导致USB外设连接断开，也就是无法调试同时用到主副屏的应用，也无法调试使用到USB外设的应用。
  * 商米提供了解决以上问题的解决方案：


PC通过有线或无线局域网连接T1设备，可以在不影响T1双屏通信及外设使用的情况下，对使用T1双屏或是使用USB外设的应用进行调试。
### 调试原理
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/7397593677815748.png)
说明：
  * T1的主副屏已经通过内置USB线连接，无需另外再通过USB线连接；
  * PC需要通过网络连接T1双屏机器（主屏）；
  * 调试主屏：PC直接通过WiFi/有线网络调试主屏；
  * 调试副屏：PC先通过网路连接主屏，通过主屏桥接来调试副屏；


### 前置条件：
  1. WiFi/有线局域网，网络连接正常；
  2. PC与T1主屏在同一个局域网内；
  3. T1双屏系统版本均支持双屏ADB调试；（14主屏：V1.11.4及以上，14寸副屏：V1.8.3及以上）
  4. T1主屏开启USB调试；（T1主屏系统设置-开发者设置-USB调试 开启）
  5. T1双屏机器上不要外接USB调试线；（主屏/副屏外接USB调试线会导致双屏通讯断开，将只能调试其中一块屏幕）
  6. PC支持ADB调试环境；
  7. T1主副屏通讯正常；（打开主屏“副屏设置”，可查看副屏的系统版本信息）


### 操作步骤：
1.获取主屏IP地址
在T1主屏系统设置-关于-状态信息查看主屏IP，记录IP地址。
例如：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
192.168.1.12
  

```

2.添加终端-主屏
在PC上打开终端命令行工具，输入以下命令：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
adb connect 192.168.1.12:5555
  

```

说明：“192.168.1.12”要替换为记录的主屏IP地址，主屏对应的端口号是“5555”，不可修改。此时应该显示：Connected to 192.168.1.12：5555，表示已连接上主屏。
3.添加终端-副屏接步骤2，输入以下命令：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
adb connect 192.168.1.12:5554
  

```

说明：“192.168.1.12”要替换为记录的主屏IP地址，副屏对应的端口号是“5554”，不可修改。此时，应该会显示：Connected to 192.168.1.12：5554，表示已连接上副屏。 **强调：必须先连接上主屏，才能连接副屏。**
4.查询已添加的终端
接步骤2或者步骤3，输入以下命令：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
adb devices
  

```

说明：此时，应该会显示1~2个终端设备。
5.调试指定终端
接步骤3，调试主屏需输入以下命令：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
adb -s 192.168.1.12:5555 shell ls
  

```

说明：“192.168.1.12”要替换为主屏的IP地址，主屏对应的端口号是“5555”，不可修改。此时，应当显示主屏存储的目录。“shell ls”可以替换为其它的ADB命令。
接步骤3，调试副屏需输入以下命令：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
adb -s 192.168.1.12:5554 shell ls
  

```

说明：“192.168.1.12”要替换为主屏的IP地址，副屏对应的端口号是“5554”，不可修改。此时，应当显示副屏存储的目录。“shell ls”可以替换为其它的ADB命令。
6.断开终端连接
接步骤3，断开主屏副屏需输入以下命令：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
adb disconnect 192.168.1.12:5555
  

```

说明：“192.168.1.12”要替换为主屏的IP地址，主屏对应的端口号是“5555”，不可修改。此时，应当同时断开主屏和副屏的连接。 **强调：断开主屏连接时，会同时断开副屏连接。**
### 示意图
添加终端：
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/3594456102221184.jpg)
调试指定终端-主屏：
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/1366702737126666.jpg)
调试指定终端-副屏：
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/06903846008955639.jpg)
断开终端连接-主屏和副屏：
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/43728165749185166.png)
上一篇：T2 Mini点阵屏开发
下一篇：T1副屏内置副显程序对接（仅适用于T1设备）
