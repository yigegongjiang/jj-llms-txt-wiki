---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfcxeghjk491
---

# 6.收银设备连接打印机
更新时间：2025-09-24 19:11:54
设备安装的第一步是将数据采集云打印机连接到原有的收银机上，进行设置后保证打印能够正常进行。根据收银机及其接口的不同，存在多种连接方式，请根据下面的提示操作：
# **USB连接打印机**
## 【Android收银机】
请先确认【收银机和收银软件】支持USB外接打印机，再进行如下操作：   
  
1、将收银机与云打印机使用USB Type-C数据线连接   
  
2、打开收银软件，允许软件访问USB外接设备（根据操作系统而定）   
  
3、在收银软件中添加外部USB打印机   
  
4、根据提示步骤操作完成配置
https://player.youku.com/embed/XNDQwNjk4NjYxNg
## 【Windows收银机】
请先确认【收银机和收银软件】支持USB外接打印机，再进行如下操作：
1、收银机与云打印机使用USB Type-C数据线连接
2、下载**打印机驱动** ：[SUNMI_Cloud_Printer_Driver.zip](https://file.cdn.sunmi.com/SUNMIDOCS/SUNMI_Cloud_Printer_Driver.zip)
3、运行“CloudPrinter Driver v2.6.7.1.exe”，选择“安装打印机驱动程序”，单击“下一步”
4、在端口列表中选择“USB001”，打印机型号下拉框中选择相应打印机型号，再单击“下一步”
  * **TP582** ：58打印机
  * **TP808** ：80打印机


  

![](https://cdn.sunmi.com/public/image/mgt-document/d93ff46a40e74cbeb282f60ebd8e41ce.png)
5、单击“完成”，在系统的打印机设备里，应该可以看到名字为TP582或者TP808的打印机。可以打印一张测试页测试一下。
  

![](https://cdn.sunmi.com/public/image/mgt-document/e2fe52575979408b98656ea97ea8438f.png)
**提示：如果出现打印导致系统蓝屏的情况，请安装Windows系统更新。**
新闻链接：[微软第三次发布Windows 10累积更新 修复打印蓝屏问题](https://www.cnbeta.com/articles/tech/1103863.htm)
# **蓝牙连接打印机**
1、打开手机或安卓收银机蓝牙功能（不推荐在Windows上使用蓝牙打印）
2、在收银软件中打开添加蓝牙打印机功能
3、搜索CloudPrint_XXXX（XXXX为SN后四位）
4、配对，完成蓝牙连接
https://player.youku.com/embed/XNDQwNTA2NjA4MA
# **网线/Wi-Fi连接打印机**
> 商米80打印机：带网口，可插网线进行局域网打印；
> 商米58打印机：带Wi-Fi，同样支持局域网打印。
## 1. 确保同一局域网
局域网打印最重要的是网络配置，收银机和打印机一定要在同一个局域网内。对于简单的网络环境，可以按照下图判断是否可用局域网进行打印。
  

![](https://cdn.sunmi.com/public/image/mgt-document/8967e7ce50534a5fb0a0fd1ec8469eda.png)
## 2. 打印机配置
**2.1 连接网络**
对于80打印机，直接插上网线；对于58打印机，通过商米助手配置连接到WiFi。
具体请参考：<https://www.yuque.com/jiangwancheng/mt0gd0/xdnpyt>
**2.2 获取打印机IP地址**
**【对于自动获取IP的方式】**
**方法1：进入路由器后台查看打印机分配到的IP**
  

![](https://cdn.sunmi.com/public/image/mgt-document/4e8864db2aa04fa3b924b2a188bb54d0.png)
**方法2：双击背后接口处的【配网键】，打印网络自检页，在小票上获取打印机IP**
  

![](https://cdn.sunmi.com/public/image/mgt-document/39c34470c75e413b9d5de2a11e514f73.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/2f34569a4f0d4fb081941ddfb599b051.png)
**【对于使用固定IP的方式】**
下载商米云打印机配置软件，在Windows电脑上预先配置打印机IP，然后再将打印机放到需要打印的位置，连上网线或者Wi-Fi。
配置工具下载：[下载地址点击跳转](https://thoughts.teambition.com/share/61d80abd76cb060041723587#title=%E5%95%86%E7%B1%B3%E4%BA%91%E6%89%93%E5%8D%B0%E6%9C%BA%E8%AE%BE%E7%BD%AE_%E4%BD%BF%E7%94%A8%E8%AF%B4%E6%98%8E_v1.3)
**注意事项：打印机型号为NT211和NT212不支持此工具。其他型号如果无法连接，请先升级固件再使用工具。**
**配置步骤如下：**
1、打印机上电开机，通过USB Type-C数据线连接电脑和打印机
2、解压下载的文件，运行【sunmi-cloud-printer-setup.exe】
  

![](https://cdn.sunmi.com/public/image/mgt-document/e4012c0854c94050b2cc75225a8bcb37.png)
3、点击连接，在里面选择USB连接的打印机
  

![](https://cdn.sunmi.com/public/image/mgt-document/1ae98c688fff414b9428cfcd7f736258.png)
4、在【以太网】或者【Wi-Fi】设置需要给打印机固定的IP地址，点击【保存】，再重启打印机生效。
IP地址根据需要设置；子网掩码一般情况下设置为【255.255.255.0】
  

![](https://cdn.sunmi.com/public/image/mgt-document/4a5cf3036dc547ccaf980c9db1703644.png)
## 3. 收银软件配置
连接好打印机后，在收银软件中配置局域网打印的参数，这里需要用到上一节中获取到的IP地址。
**因为收银软件千差万别，这里以银豹为例：**
1、打开银豹收银软件，进入【系统设置】
  

![](https://cdn.sunmi.com/public/image/mgt-document/ba278162305a496eb7b7562ad8b8ab44.png)
2、在【系统设置 > 小票打印设置】设置中，设置【小票打印机IP】，然后点击测试，如果能够出票则正常，否则检查网络是否联通。
  

![](https://cdn.sunmi.com/public/image/mgt-document/612cede6e4a64eeabc092b8729e8e53d.png)
# **串口连接打印机**
## 1. 转接线购买
商米数据采集云打印机没有串口，但是支持通过转接线实现串口打印。推荐使用优越者Y-105M的转接线，经测试该转接线能够实现正常的串口打印。点击购买串口转Type-C转接线：<https://item.jd.com/100009411306.html>
  

![](https://cdn.sunmi.com/public/image/mgt-document/e6307c9290a94a38a76bb277926a2f4b.png)
## 2. 打印机配置
商米数据采集打印机串口打印的默认设置为：
> 波特率：115200
> 数据位：8
> 停止位：1
> 校验位：无
打印机通过转接线连接之后，请在收银软件中配置上述串口参数。
如果参数无法修改，可通过[网线/Wi-Fi连接打印机](https://www.yuque.com/books/share/4459d3bb-8c62-466e-bea6-79eecea1f05d/wa6rhx)中提到的配置软件（[SUNMI_Cloud_Printer_Setup.zip](https://file.cdn.sunmi.com/SUNMIDOCS/SUNMI_Cloud_Printer_Setup.zip)）手动设置打印机串口参数。
  

![](https://cdn.sunmi.com/public/image/mgt-document/88432dcdf1234af383669531de46ea77.png)
上一篇：5.数采问题FAQ
下一篇：7.收银软件配置打印机
