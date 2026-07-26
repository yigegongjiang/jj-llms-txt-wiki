---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xffieghjk579
---

# 9、云打印机WEB配置使用说明
更新时间：2026-02-26 12:10:47
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
# 获得云打印机IP地址
1、双击打印机底部的配对按钮，打印机将播报“网络自检中，请稍候”；
![](https://cdn.sunmi.com/public/image/mgt-document/31b4dec5309f40b4acd631ff5ac562b9.png)
2、自检完成后将打印出“自检页”小票，依据打印机使用的局域网类型，可以获得WEB设置所需的IP地址。
![](https://cdn.sunmi.com/public/image/mgt-document/c439512e0e3a4ca9b9f536eadd6b37b0.png)
# 登录WEB设置页面
## 1、登录
打开浏览器，输入【http://[自检页中显示的IP地址]】，将显示如下的登录界面。默认密码为空，直接点击登录进入。
![](https://cdn.sunmi.com/public/image/mgt-document/bd1acb88f67449cbba37c8b7bcd6f5c1.png)
## 2、修改密码
登录进入以后，选择右上角的【更改密码】，输入【原密码】-【新密码】-【确认密码】，点击【确定】保存并更改密码。
![](https://cdn.sunmi.com/public/image/mgt-document/482bd6cdaaa44cb38bbf5152a6380cd3.png)
# 设置参数
## 1、设备信息
【设备信息】页面将显示设备的型号、SN号、版本信息和其他信息。
![](https://cdn.sunmi.com/public/image/mgt-document/a908d6dc31b44fc48d8c39e2230ec7b4.png)
## 2、以太网
【以太网】页面，将显示以太网络的动态IP地址和子掩码信息。如需设置静态IP地址，可在【IP地址(静态分配)】-【子网掩码(静态分配)】-【网关(静态分配)】输入IP地址。
本打印机支持多IP模式，“动态分配的IP地址”和“静态分配的IP地址”是共存的，可以同时使用。
  

![](https://cdn.sunmi.com/public/image/mgt-document/c386a1dbbac149b5b04ced60349deec2.png)
## 3、Wi-Fi
【Wi-Fi】页面，将显示WIFI网络的动态IP地址和子掩码信息。如需设置静态IP地址，可在【IP地址(静态分配)】-【子网掩码(静态分配)】-【网关(静态分配)】输入IP地址。
本打印机支持多IP模式，“动态分配的IP地址”和“静态分配的IP地址”是共存的，可以同时使用。
  

![](https://cdn.sunmi.com/public/image/mgt-document/c7ce10efd3a14746a818e5c2286cab7b.png)
  

进入【配置SSID】，可以配置WIFI连接的SSID名称与密码，让打印机接入WIFI无线网络。
![](https://cdn.sunmi.com/public/image/mgt-document/547b991400774221a8911cf35da30934.png)
## 4、蓝牙
【蓝牙】页面，将显示蓝牙的MAC地址和设备名称。蓝牙绑定设备时，根据所需的场景，分为【自动绑定】【手动绑定】【手动绑定+语音提醒】。
(1)选择【自动绑定】时，不用经过确认，上位机就可以直接配对绑定成功。
(2)选择【手动绑定】时，在上位机完成配对绑定后，还需要按一下打印机的配网按钮才能配对成功。如果不按配网键确认，上位机即使配对成功也无法发送数据。
(3)选择【手动绑定+语音提醒】时，在上位机完成配对绑定后，打印机会语音提示按配网键，此时按一下打印机配网按钮才能配对成功。如果不按配网键确认，上位机即使配对成功也无法发送数据。
![](https://cdn.sunmi.com/public/image/mgt-document/16320c774a584702a8f2740efed4f611.png)
  

选择【蓝牙绑定记录】，可以查看绑定成功的所有蓝牙设备信息，如果不希望某台设备进行蓝牙打印，可以选择设备后删除。
![](https://cdn.sunmi.com/public/image/mgt-document/e0fadb1cc44d4b8ebe38af4d0f384dc3.png)
## 5、打印设置
【打印设置】页面，可以设置一些打印机常用功能项。
![](https://cdn.sunmi.com/public/image/mgt-document/e8e88d1bef544d4b8515884f3a29eea9.png)
## 6、图像设置
【图像设置】页面，可以设置NV图像和灰阶图像。
![](https://cdn.sunmi.com/public/image/mgt-document/ed002a477a824c1fa9cc54909b68c147.png)
## 7、切刀设置
【切刀设置】页面，可以设置切刀动作后自动打印出图像。
![](https://cdn.sunmi.com/public/image/mgt-document/c09e7a08a2b8400b861c3f2886949606.png)
## 8、扫码设置
【扫码设置】页面，可以设置跟扫描条码相关的参数，同时还具备实时查看扫描图像的功能。
![](https://cdn.sunmi.com/public/image/mgt-document/338df542946a4801b5736cbb1ab3a40c.jpg)
## 9、打印日志
【打印日志】提供了一个检查发送数据信息完整性的功能。选择对应的打印记录，可以看到打印内容的十六进制和票据打印效果。
![](https://cdn.sunmi.com/public/image/mgt-document/f37b31eff061402aa6b2085a8947b200.png)
上一篇：8、标签打印机TSPL指令集
下一篇：10、云打印机Windows配置使用说明
