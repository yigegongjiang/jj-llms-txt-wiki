---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzieghjk579
---

# 18、云打印机OPOS驱动
更新时间：2025-12-19 22:53:10
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
在Windows电脑下安装OPOS打印机驱动，请下载驱动程序：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SUNMI Printer OPOS Driver.rar 
# OPOS安装
## 1、注册OCX控件
OPOS安装目录请不要包含中文，否则将会导致某些功能不可用。
右键选择【SetupOCX.bat】文件，菜单中选择【以管理员身份运行】注册OPOS控件。
  

![](https://cdn.sunmi.com/public/image/mgt-document/815bb97db76d4f5fac96ae830be44632.png)
注册成功如下图：
  

![](https://cdn.sunmi.com/public/image/mgt-document/620258fe82644bed96bf3a8f2daeba5c.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/95ea47e6fa4345f59d2b306eb18b1eb8.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/54988030d33f4724b9249c7f7767af9d.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/d6d08d0614bf4b69a78c521a9e1fe867.png)
共四个文件注册成功!
## 2、添加打印机
右键选择【SetupPOS_CN.exe】文件，菜单中选择【以管理员身份运行】运行程序。
选择设备类型【POSPrinter】，选择机型【POS80】，设备名称输入【CloudPrinter】，点击【增加新设备】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/8da0d15e18764de097a195ecc4a4b52a.png)
端口选择【USB】，点击【保存】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/dee5dc35232d424b88f678f41c3ab66d.png)
保存后列表出现新增的打印机名称；
  

![](https://cdn.sunmi.com/public/image/mgt-document/7e4cbbba69f844c1b21c2d17c3b11723.png)
## 3、添加钱箱
选择设备类型【CashDrawer】，选择机型【POS80】，设备名称输入【CashDraw】，点击【增加新设备】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/3d3a1e49b22046288314b73436d5ad09.png)
端口选择【USB】，点击【保存】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/dee5dc35232d424b88f678f41c3ab66d.png)
保存后列表出现新增的钱箱名称，退出设置程序；
  

![](https://cdn.sunmi.com/public/image/mgt-document/5187a6c72dd1486599cb87501963b884.png)
# 测试设备驱动
1、右键选择【Printer_OPOS_Test.exe】程序，菜单中选择【以管理员身份运行】运行程序。选择【Printer】，OPOS System选择【Generic(NCR/RCS)[GENERIC]】，点击【Inisialize】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/33de290907704ad5bc8bc6a5f83758f0.png)
2、Device选择【POS80】，点击【Open】，下方窗口显示【Printer: OPEN(POS80)=OPOS_SUCCES】;
点击【Claim】，下方窗口显示【Printer: CLAIM=OPOS_SUCCES】;
点击【Click to Enable】，按键变为【Click to Disable】，下方窗口显示【Printer: ENABEL=OPOS_SUCCES】;
  

![](https://cdn.sunmi.com/public/image/mgt-document/b1544b4fdad84358847cabe134c28a25.png)
3、Line文本框输入需要打印的文本内容，点击【Print】，打印会打印出来对应的内容；
  

![](https://cdn.sunmi.com/public/image/mgt-document/e2e0b84a459745da9650fd4c9cc9422b.png)
4、钱箱的测试方法相同
# 删除设备
## 1、删除打印机
列表选择要删除的打印机名称，设备类型选择【POSPrinter】，点击【删除已安装的设备】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/e71e945dd2d7416db8b842785de82e44.png)
## 2、删除钱箱
列表选择要删除的钱箱名称，设备类型选择【CashDrawer】，点击【删除已安装的设备】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/80dfa40bdff7437fbe6504b73d47532d.png)
上一篇：17、云打印机macOS驱动
下一篇：19、云打印机Windows蓝牙驱动
