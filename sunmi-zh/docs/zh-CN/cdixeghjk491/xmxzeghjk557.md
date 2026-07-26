---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmxzeghjk557
---

# 7、钱箱驱动器for Square
更新时间：2025-10-16 12:01:00
# 一、Square版本说明
钱箱驱动器为了配合Square POS机能够打开普通钱箱，专门为其开发了一个【for Square】版本，从外观上可以方便的识别Square版本与标准版本的差异（如下图示）。
  

![](https://cdn.sunmi.com/public/image/mgt-document/66de737752144e0dbefa01309464a963.png)  
| 类型  | 参数  |  
| --- | --- |  
| 输入  | Type-A USB，5V/2A  |  
| 蓝牙通讯  | BLE 5.0  |  
| 指示灯  | 电源灯、蓝牙灯  |  
| 钱箱输出  | RJ12，24V/1A，100ms  |  
| 短路保护  | 上电后、开启钱箱前，会自动对钱箱进行短路检测，检测正常才驱动钱箱开启  |  
| 操作系统  | Android（支持BT BLE模式）；   
  
iOS（支持BT BLE模式）；   
  
Square Register、 Square Stand（仅支持USB HID模式）；  |  
【钱箱驱动器 for Square】版本可以支持Square Stand、Square Register驱动打开钱箱。
  

![](https://cdn.sunmi.com/public/image/mgt-document/5ab9145726f44654b09116cf6c725d56.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/431ba966cbf244aebfce261bebd03463.png)
# 二、连接方式
  

![](https://cdn.sunmi.com/public/image/mgt-document/62ba330db1854c82918499bf23a17dfd.png)
需要进一步了解 Square 硬件相关问题，可以浏览 Square 官网上的说明。
<https://squareup.com/help/us/en/article/5152-cash-drawer-management>
# 三、钱箱接入操作
1、将钱箱的RJ12接口插入【钱箱驱动器】；
2、将【钱箱驱动器】的USB接口插入Stand支架集线器的USB端口；
3、USB集线器连接到Stand支架，然后接通电源适配器；
4、正确连接硬件后，启动Square应用程序；
5、点按屏幕底部导航栏的【More】；
6、点击【Settings】>【Hardware】，在【My hardware】中显示识别到【APG VB554】钱箱设备；
  

![](https://cdn.sunmi.com/public/image/mgt-document/584e75eb94e742c48f59e2842be94d94.PNG)
7、点击【APG VB554】或者【Cash drawers】进入详情
  

![](https://cdn.sunmi.com/public/image/mgt-document/879291c4f18f4d95a97f3ee7cd22fddb.PNG)
8、在【Available Cash Drawers】中列出了相应的钱箱；点击【Test Cash Drawer】确保钱箱可以正常工作；
  

![](https://cdn.sunmi.com/public/image/mgt-document/b4991f5c5c03445da4369576fec3248a.PNG)
9、这样在每次现金付款以后就会自动打开钱箱了。
上一篇：6、钱箱驱动器Windows测试工具
下一篇：8、钱箱驱动器APP使用说明
