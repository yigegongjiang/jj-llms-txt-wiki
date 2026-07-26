---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzmeghjk546
---

# 19、云打印机Windows蓝牙驱动
更新时间：2025-12-19 22:53:36
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
# **添加蓝牙打印机**
## 1、打开设置
打开windows【设置】，选择【设备】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/c1d652db4d5e47a98bfdbda8e9a687a5.png)
选择【蓝牙和其他设备】，点击【添加蓝牙或其他设备】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/91f55416b41b4bdaa1f22e6c50a9c323.png)
## 2、添加设备
选择【蓝牙】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/177e78c43e344fe6af88aae41cccc6b6.png)
搜索蓝牙设备，找到设备名称为【CloudPrint_xxxx】的云打印机，然后点击【连接】按钮。
  

![](https://cdn.sunmi.com/public/image/mgt-document/1a8f14ddebf246b581eb11d3ef997363.png)
连接成功后显示“已配对”，蓝牙打印机添加成功。
  

![](https://cdn.sunmi.com/public/image/mgt-document/aa88d705abf94f95a854ef0b0d042a08.png)
# **安装打印机驱动**
## 1、添加打印机
打开windows【设置】，选择【打印机和扫描仪】，点击【添加打印机或扫描仪】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/43344cc9b7304df58d902a7d3c5f78d3.png)
## 2、选择驱动
等待几秒后点击【我需要的打印机不在列表中】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/d8cc228cf33544d1937a09a81900bff2.png)
选择【通过手动设置添加本地打印机或网络打印机】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/2cb9fc4c50354ff398f6c5c1efff7815.png)
打开【设备管理器】，在【端口(COM 和 LPT)】下面 ，找到【蓝牙链接上的标准串行(COMxx)】。
COM端口可能有多个，需要逐个测试，找到蓝牙对应的端口。
  

![](https://cdn.sunmi.com/public/image/mgt-document/32020b122dcd4f5abaec6dda614a34e3.png)
回到【添加打印机】页面，在【使用现有的端口】下拉复选框里选择对应的COM端口。
  

![](https://cdn.sunmi.com/public/image/mgt-document/fe697d6437e4451288bb6ba9d561b8a0.png)
在打印机驱动程序里选择对应的驱动名称。如果没有，请【[参照驱动](https://developer.sunmi.com/docs/zh-CN/xeghjk491/dimeghjk546)】安装一下 。
  

![](https://cdn.sunmi.com/public/image/mgt-document/70e5e9c1d29f48d98afe4eb9d2f08ec2.png)
## 3、打印测试页
打开windows【设置】，选择对应的打印机驱动名称，右键选择【打印机属性】。点击【打印测试页】，打印机打印出小票。
![](https://cdn.sunmi.com/public/image/mgt-document/348b8c598812422d8fbf1c35be53709b.png)
上一篇：18、云打印机OPOS驱动
下一篇：20、云打印机Windows驱动TCP打印
