---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmxceghjk502
---

# 2、钱箱驱动器Windows驱动说明
更新时间：2025-10-31 17:26:22
# 一、安装USB驱动
钱箱驱动器无需安装驱动程序，只需将其连接到Windows系统的USB接口上即可自动识别为USB CDC串行设备。在Windows的【设备管理器】中可以看到为其分配的串口号，例如“COM3”。
  

![](https://cdn.sunmi.com/public/image/mgt-document/b8ece57f740743d99d4b442a934f589f.png)
# 二、快捷方式图标打开USB钱箱
2.1 桌面单击右键→新建→快捷方式
  

![](https://cdn.sunmi.com/public/image/mgt-document/900588cddc05451fbfb00bdf4a2462ea.png)
2.2 在创建快捷方式窗口中，先点击【请键入对象的位置】，然后输入以下命令：【C:\WINDOWS\System32\cmd.exe /c echo OpenDoor>\\\\.\COMxx】。请将“COMxx”替换为实际设备管理器中的串口号，例如“COM3”。接下来，点击【下一步】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/b79e13f116b6498898ad539de951408d.png)
2.3 在【键入该快捷方式的名称】输入快捷方式名称，比如【Open Cash Drawer】，点击【完成】。
  

![](https://cdn.sunmi.com/public/image/mgt-document/333cb40833d9410498836b45f6658446.png)
2.4 现在桌面上将看到一个【Open Cash Drawer】快捷方式，双击时，它将弹出抽屉。
  

![](https://cdn.sunmi.com/public/image/mgt-document/9da56b926f774a8d817a20d68e733f15.png)
2.5 还可以为这个开钱箱动作设置一个键盘热键。
1)-右键单击快捷方式；2)-选择快捷键选项卡；3)-选择快捷键字段；4)-输入要使用的键盘键；5)-单击“确定”
  

![](https://cdn.sunmi.com/public/image/mgt-document/c0f7449e44d649689833ab5238232fee.png)
# 三、OPOS驱动打开USB钱箱
下载钱箱OPOS驱动
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SUNMI_OPOS_Driver.zip 
## 3.1 注册OCX控件
OPOS安装目录请不要包含中文，否则将会导致某些功能不可用。
右键选择【OCX_Install.bat】文件，菜单中选择【以管理员身份运行】注册OPOS控件。
  

![](https://cdn.sunmi.com/public/image/mgt-document/bbc7d7af6bae4531bb8b123814c522de.png)
控件注册成功如下图：
  

![](https://cdn.sunmi.com/public/image/mgt-document/d9ef0f6b26754099a8bb04bbc9c0aff3.png)
提示4次文件注册成功!
## 3.2 添加钱箱驱动
1、右键选择【Setup_OPOS_EN.exe】文件，菜单中选择【以管理员身份运行】运行程序。
2、选择设备类型【CashDrawer】，选择机型【POS58】，设备名称输入【CashDrawerTrigger】，点击【增加新设备】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/9fee208418844994afae06c287fd1d89.png)
3、端口选择【COMx】例如”COM3”，校验选择【NONE】，点击【保存】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/6342d673aa1f4b86af72f7efeacf2cc6.png)
4、保存后列表出现新增的钱箱名称，退出设置程序；
  

![](https://cdn.sunmi.com/public/image/mgt-document/e47e92089e9141eb8845bf71ea4fda88.png)
## 3.3 测试钱箱
1、右键选择【Test_OPOS.exe】文件，菜单中选择【以管理员身份运行】运行程序。
2、选择【Cash Drawer】，OPOS System选择【Generic(NCR/RCS)[GENERIC]】，点击【Inisialize】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/ffc29651d1bb4039bc143ce99bdc17f3.png)
3、Device选择【CashDrawerTrigger】，点击【Open】，下方窗口显示【Cash Drawer: OPEN(CashDrawerTrigger)=OPOS_SUCCES】;
点击【Claim】，下方窗口显示【Cash Drawer: CLAIM=OPOS_SUCCES】;
点击【Click to Enable】，按键变为【Click to Disable】，下方窗口显示【Cash Drawer: ENABEL=OPOS_SUCCES】;
如果显示【Cash Drawer: ENABEL=OPOS_E_ILLEGAL】或者其他信息时，请检查设备端口号是否正确，设备是否连接正确。
软件程序是否有选择【以管理员身份运行】权限。
  

![](https://cdn.sunmi.com/public/image/mgt-document/0a8e312a138b48ba9a7e350d500f8b3f.png)
3、Cash Drawer Operation选择【Open Drawer】，钱箱将会打开；点击【Drawer Status】，显示钱箱状态【Drawer is Opend】或者【Drawer is Closed】；
  

![](https://cdn.sunmi.com/public/image/mgt-document/85541f510425416daa3825f5fecddc4e.png)
## 3.4 删除钱箱
列表选择要删除的钱箱名称，设备类型选择【CashDrawer】，点击【删除已安装的设备】，钱箱驱动删除完成。
  

![](https://cdn.sunmi.com/public/image/mgt-document/30991b49116d45dc84d216e9868118b0.png)
上一篇：1、钱箱驱动器产品说明
下一篇：3、钱箱驱动器Android驱动说明
