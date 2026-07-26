---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzfeghjk535
---

# 17、云打印机macOS驱动
更新时间：2025-12-19 22:52:27
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
在macOS电脑下安装打印机驱动，请下载驱动程序：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) hprt-pos-printer-driver-v1.2.3.pkg 
# 1、安装驱动程序
执行驱动程序，启动安装器开始安装驱动。
  

![](https://cdn.sunmi.com/public/image/mgt-document/2baf06db5a7744968e62c3ca197bf02b.png)
选择【继续】 进行下一步安装。
![](https://cdn.sunmi.com/public/image/mgt-document/728909c96a60446f887ed543a51b5612.png)
安装驱动选择【Install Driver】>【继续】进行下一步。
  

![](https://cdn.sunmi.com/public/image/mgt-document/bac645a5f09f46b0ba3ed638b0b0b8fb.png)
# 2、添加USB打印机
打开【系统偏好设置】>【打印机与扫描仪】
  

![](https://cdn.sunmi.com/public/image/mgt-document/d0106c0d159c451493c4819567b98c2f.png)
点击【+】添加打印机
  

![](https://cdn.sunmi.com/public/image/mgt-document/aecdbde55bf8419b86720965d1ad99a1.png)
只有当USB接口上插着票据打印机时，才会出现【linux g_printer】打印机。
点击【使用】>【选择软件】，选择【HPRT TP808,1.2.2】>【好】
  

![](https://cdn.sunmi.com/public/image/mgt-document/fa3e77a021d14a31a2f22bfc780dd720.png)
选择【添加】
  

![](https://cdn.sunmi.com/public/image/mgt-document/c02183d437c542bf9d2d70665bd2c348.png)
完成打印机驱动安装
  

![](https://cdn.sunmi.com/public/image/mgt-document/0fbd16f1ad2f4de493cc22d21536df74.png)
# 3、添加IP打印机
打开【系统偏好设置】>【打印机与扫描仪】
  

![](https://cdn.sunmi.com/public/image/mgt-document/d0106c0d159c451493c4819567b98c2f.png)
点击【+】添加IP打印机。
## 获得打印机的IP地址
1、双击打印机底部的配对按钮，打印机将播报“网络自检中，请稍候”；
  

![](https://cdn.sunmi.com/public/image/mgt-document/31b4dec5309f40b4acd631ff5ac562b9.png)
2、自检完成后将打印出“自检页”小票，依据打印机使用的局域网类型，可以获得WEB设置所需的IP地址。
  

![](https://cdn.sunmi.com/public/image/mgt-document/c439512e0e3a4ca9b9f536eadd6b37b0.png)
【地址】填入打印机IP，【协议】选择【HP Jetdirect-Socket】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/0afa17a9c3dd4a7cad6f5b6e64ebe040.png)
点击【使用】>【选择软件】，选择【HPRT TP808,1.2.2】>【好】
![](https://cdn.sunmi.com/public/image/mgt-document/fa3e77a021d14a31a2f22bfc780dd720.png)
选择【添加】
  

![](https://cdn.sunmi.com/public/image/mgt-document/7670ca938e2143159378312862aa512c.png)
完成打印机驱动安装
  

![](https://cdn.sunmi.com/public/image/mgt-document/c2e932c3fac04f7faa345189d5cf9448.png)
# 4、设置功能
需要设置打印机功能时，选择【文件】【打印】【打印机特性】【功能设定】【General】中可以设置切刀
**USB打印机：**
![](https://cdn.sunmi.com/public/image/mgt-document/4319ca649ff04e948836ca179b4dfbe8.png)
**IP打印机：**
  

![](https://cdn.sunmi.com/public/image/mgt-document/abfa3dfdeb6d4bc799198309a9ff8d85.png)
上一篇：16、云打印机macOS SDK
下一篇：18、云打印机OPOS驱动
