---
url: https://docs.sunmi.com/zh-CN/ceghjk502/czreghjk568
---

# T1
更新时间：2025-11-10 11:21:30
## 软件开发说明
  * [调试设备说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrzeghjk557)
  * [发布应用说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/qaaeghjk480)
  * [权限、全屏、获取系统版本等代码说明](https://developer.sunmi.com/docs/zh-CN/xeghjk491/qaxeghjk491)
  * [打印和客显说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdideghjk524)
  * [外接接口说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdixeghjk491/)
  * [自定义音量键对接文档](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrieghjk579/)


#### 开启无线调试
T1因MicroUSB调试线插入会导致其他外接USB设备无法工作，所以需要在T1设备上开启无线调试（如果渠道设备已开启调试保护，您需要先登录调试员进行授权）
  1. adb shell 进入T1控制台后输入: setprop service.adb.tcp.port 5555
  2. ifconfig查看ip地址
  3. 拔掉USB
  4. adb connect
  5. 如：adb connect 192.168.0.219
  6. 在显示：connected to :5555 后可以正常调试
  7. 如果出现连接被拒绝:插入usb调试线 adb tcpip 5555
  8. 在连接上主屏之后在连接副屏，副屏的端口号为5554


如果此时连接失败可以重新打开一下USB调试开关,或者查看是否在同一网络
#### T1副屏对接引导
T1设备可选配7寸或14寸副屏，副屏主要面向消费者，以显示和简单交互为主。其中7寸屏不支持触控操作和安装自定义客显程序，14寸支持触控操作和安装自定义客显程序。 主副屏是两套独立的系统，两个屏幕之间通过USB虚拟串口进行通讯。 判断副屏是否存在： Settings.Global.getInt(getContentResolver(), "sunmi_sub", 1);// 1为双屏，其他为单屏 关于如何定制副屏显示内容，我们提供了两种方案： 1. [内置副显程序对接文档](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfmreghjk568) 我们在副屏中内置了一个程序，并设计了多套显示方案，可以通过主屏应用直接调用封装好的方法，快捷地将信息显示在副屏上。当前已支持图、文、列表、视频等多种显示方案，可以显示例如订单信息、营销广告、支付二维码等内容。 2. [自定义副显程序对接文档](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfzdeghjk524) T1设备仅14寸副屏支持自定义副屏显示。 当商米内置副屏程序无法满足您的定制化需求，还可以直接制作用于副屏显示、交互的APP。
上一篇：T2s LITE
下一篇：T1 MINI
