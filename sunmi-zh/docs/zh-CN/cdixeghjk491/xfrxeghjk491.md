---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfrxeghjk491
---

# 23、云打印机局域网HTTP打印
更新时间：2025-12-19 22:55:20
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
# 概述
本⽂档主要介绍通过WEB方式向局域网指定云打印POST打印数据。其核心实现方法就是向【http://<ip>/cgi-bin/print.cgi】URL地址发送十六进制字符串形式的ESC/POS指令。
# DEMO代码下载
JS DEMO代码下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) EscPosPrinter-js.zip 
# DEMO使用说明
1、运行【EscPosPrinter.html】，打开WEB页面如下：
使用时，请确保电脑的IP地址与打印机的IP地址处于同一网段，不同网段将无法通讯。
  

![](https://cdn.sunmi.com/public/image/mgt-document/19648484b7114f40b18d372708ba1d82.png)
2、获得云打印机IP地址和SN号：
双击打印机底部的配对按钮，打印机将播报“网络自检中，请稍候”；
  

![](https://cdn.sunmi.com/public/image/mgt-document/31b4dec5309f40b4acd631ff5ac562b9.png)
自检完成后将打印出“自检页”小票，依据打印机使用的局域网类型，可以获得WEB设置所需的IP地址和设备SN号。
  

![](https://cdn.sunmi.com/public/image/mgt-document/3ccb2730fc984f81adf00da3c3f5c4f0.png)
3、选择【Print Sample】把小票数据发送给指定打印机。提示【[status] 200】表示通讯成功
  

![](https://cdn.sunmi.com/public/image/mgt-document/3290e0f6ce434ed0b41a17104f3b2bc7.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/f489a884d06d49af951f058560f37886.png)
4、选择【Query Status】获得打印机状态数据。显示【[response]】表示各类状态信息。
  

![](https://cdn.sunmi.com/public/image/mgt-document/e2d42decc0af463f9aa3147565c50127.png)
5、选择图片文件，可以打印出带图片的测试页。选择【Diffuse Dithering】或【Threshold Dithering】改变图片算法。
  

![](https://cdn.sunmi.com/public/image/mgt-document/5c34189d0447444aa7e82e846ac99e15.png)
上一篇：22、云打印机Windows DLL接口
下一篇：24、商米助手云打印机功能管理
