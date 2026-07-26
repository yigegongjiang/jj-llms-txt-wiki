---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmxmeghjk546
---

# 6、钱箱驱动器Windows测试工具
更新时间：2025-10-31 17:35:01
# 一、概述
本工具实现了对“钱箱驱动器”的硬件版本信息查看、基本功能测试、固件升级、调试Log的采集功能。
# 二、工具下载
下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) sunmi-cash-drawer-driver-utilityV1.1.zip 
# 三、工具使用说明
钱箱驱动器无需安装驱动程序，只需将其连接到Windows系统的USB接口上即可自动识别为USB CDC串行设备。在Windows的【设备管理器】中可以看到为其分配的串口号，例如“COM3”。
  

![](https://cdn.sunmi.com/public/image/mgt-document/b8ece57f740743d99d4b442a934f589f.png)
## 1、Serial
1)启动程序，主界面有【Serial】【BLE】两个页面，表示当前“钱箱驱动器”在哪个模式下进行通讯。
2)插上“钱箱驱动器”以后，会自动出现串行设备选项，点击【Open】打开设备端口进行通讯。
3)显示设备的SN序列号和固件版本，并且可以对钱箱功能进行测试。
4)对“钱箱驱动器”进行固件升级。
5)在必要的时候，可以通过串口模式对“钱箱驱动器”的蓝牙通讯内容进行调试，显示通讯过程log。
  

![](https://cdn.sunmi.com/public/image/mgt-document/6411cf725a7843d2859f9d227dbc3449.png)
## 2、Functional Test
1)【Short-Circuit Detection】：对钱箱进行短路测试，检测正常时显示【No short-circuit detected.】。如果出现钱箱pin短路，则显示【Short-circuit detected on the following pin pair(s): x-x】，后面的数字是发生短路的管脚。
  

![](https://cdn.sunmi.com/public/image/mgt-document/d6b1a2bbf4ab419eb3b1e85c88da6a71.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/17f7f32730fc43f6b897fed6449ffec8.png)
2)【Open Cash Drawer】：执行打开钱箱动作。
3)【Get Cash Drawer Open State】：获取钱箱打开/关闭状态。
## 3、Device Firmware Update
1)点击【Enter Device Firmware Update Mode】按钮，让驱动器进入升级模式。
  

![](https://cdn.sunmi.com/public/image/mgt-document/46a279f7716940758b4fa341e8de72d9.png)
2)“钱箱驱动器”会自动重新，绿灯和蓝灯同时亮起。
串行设备会变成一个新的名称，点击【Open】打开新的串口。
点击【Browse】打开需要升级的固件文件。
点击【Download】开始下载新的固件程序，升级完成“钱箱驱动器”会自动重启，亮绿灯。
  

![](https://cdn.sunmi.com/public/image/mgt-document/44360994a57647338f493938aae5a771.png)
## 4、BLE Trace
选择【BLE Trace】，启动串口模式下，对蓝牙通讯的内容进行调试，便于发生蓝牙通讯过程中的问题。
窗口内显示通讯过程日志。
调试完成，点击【SAVE】保存成一个log文件，文件可以发送给客服进行技术分析。
  

![](https://cdn.sunmi.com/public/image/mgt-document/981ebf6dc3f4461d8f1cf8d41bd0a0d8.png)
## 5、BLE
1)在系统设置中添加设备，选择【蓝牙】
如果设备在上电5分钟内未进行配对绑定，设备将退出配对模式。
如果需要再次配对，请重新拔插USB进行上电操作，并在5分钟内完成绑定。
  

![](https://cdn.sunmi.com/public/image/mgt-document/ee03d3398dbc4fee8c0c8aac1b5a8de6.png)
2)添加设备页开始搜索钱箱驱动器，找到以【CashDrawer_xxxxxx】命名的设备名称；
  

![](https://cdn.sunmi.com/public/image/mgt-document/0b0b6d679d814f01bf50ebb65ba14afc.png)
3)点击设备名称，出现输入设备PIN码，PIN码是设备名称后6位数字；
  

![](https://cdn.sunmi.com/public/image/mgt-document/d0235ba45e4944c798713360ac5942d5.png)
4)设备连接成功；
  

![](https://cdn.sunmi.com/public/image/mgt-document/b41b658d9d67485fbc4470a61f23b877.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/fbeb53c8a29140d59ba36c056861497c.png)
5)回到工具界面，切换到【BLE】页面；
界面中BLE内容是连接的设备名称，【Scan…】可以进行搜索蓝牙设备。
【Functional Test】显示设备的SN序列号和固件版本，并且可以对钱箱功能进行测试。
【Serial Trace】在必要的时候，可以通过蓝牙模式对“钱箱驱动器”的串口通讯内容进行调试，显示通讯过程log。
  

![](https://cdn.sunmi.com/public/image/mgt-document/268998769c8a4caa8ca720621417b419.png)
## 6、Scan
1)点击【Scan…】搜索蓝牙“钱箱驱动器”设备。
如果设备在上电5分钟内未进行配对绑定，设备将退出配对模式。
如果需要再次配对，请重新拔插USB进行上电操作，并在5分钟内完成绑定。
2)等待一会，搜索到设备列表【CashDrawer_xxxxxx】。
3)点击【OK】进行配对。
  

![](https://cdn.sunmi.com/public/image/mgt-document/40ce2371850546919b69f6d0009181d9.png)
4)windows系统弹出添加设备对话框，点击【添加设备CashDrawer_xxxxxx】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/473a80ecdbaf4898b979b771cb3284df.png)
5)windows系统弹出输入配对PIN对话框，输入设备名称【CashDrawer_xxxxxx】后6位数字PIN码进行配对。
  

![](https://cdn.sunmi.com/public/image/mgt-document/906aa1935fb141aa9a1068ab04b31777.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/d9ede3146b0c40109e422af4b5539283.png)
上一篇：5、钱箱驱动器macOS驱动说明
下一篇：7、钱箱驱动器for Square
