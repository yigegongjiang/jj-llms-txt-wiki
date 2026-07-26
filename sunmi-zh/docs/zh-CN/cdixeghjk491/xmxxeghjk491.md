---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmxxeghjk491
---

# 1、钱箱驱动器产品说明
更新时间：2025-10-16 11:53:31
# 一、产品介绍
钱箱驱动器是一款可以通过USB和Bluetooth连接，驱动现金抽屉打开的设备。当POS系统中没有使用打印机时，钱箱可以连接到计算机、安卓平板或iPad平板，通过软件驱动钱箱打开。此驱动器使用Type-A USB接口进行供电，可通过PC供电或5V/2A电源适配器供电。
使用USB连接到PC计算机时，钱箱驱动器使用虚拟串口驱动程序，使其看起来像标准RS232串口。现有的POS软件可以与钱箱驱动器进行通讯，就像连接到串行端口一样。系统会分配每个USB驱动程序一个唯一的端口号，钱箱驱动器软件将为识别到的每个设备分配一个可用的串口端口（即多个钱箱驱动器可以在同一计算机上使用）。
使用Bluetooth连接到计算机时，钱箱驱动器使用BT BLE驱动程序，POS软件通过专用的SDK与钱箱驱动器进行通讯。
钱箱驱动器除了能够通过USB和Bluetooth驱动打开钱箱以外，还能实时获取钱箱的状态，提供了比典型钱箱接口更高的安全性。通过LED指示灯，还能实时获得钱箱故障状态，使故障排除变得容易。
钱箱驱动器还兼容了Square Register、Square Stand的软件打开钱箱功能，可以替换【APG VB554A-BL1616】USB钱箱的功能。
  

![](https://cdn.sunmi.com/public/image/mgt-document/3206ffe9e09f438ea7d48abe6ba81658.jpg)  
| 类型  | 参数  |  
| --- | --- |  
| 输入  | Type-A USB，5V/2A  |  
| 蓝牙通讯  | BLE 5.0  |  
| 指示灯  | 电源灯、蓝牙灯  |  
| 钱箱输出  | RJ12，24V/1A，100ms  |  
| 短路保护  | 上电后、开启钱箱前，会自动对钱箱进行短路检测，检测正常才驱动钱箱开启  |  
| 操作系统  | Windows（支持USB CDC和BT BLE模式）；   
  
Android（支持USB CDC和BT BLE模式）；   
  
iOS（仅支持BT BLE模式）；   
  
macOS（仅支持USB CDC模式）；  |  
| 兼容应用程序  | Windows：快捷图标方式打开、OPOS驱动方式打开、串口指令驱动打开；   
  
Android：SDK驱动打开；   
  
iOS：SDK驱动打开；   
  
macOS：串口指令驱动打开；   
  
Square：兼容Square Register、 Square Stand；  |  
# 二、连接方式
## USB驱动方式
① 使用RJ12接口连接钱箱线，输出脉冲驱动信号打开钱箱；
② 使用USB接口连接PC、POS机，可以让驱动程序发送指令打开钱箱；
  

![](https://cdn.sunmi.com/public/image/mgt-document/d04a44dd347c4d7e84fbf5ea3138a82d.png)
## Bluetooth驱动方式
① 使用RJ12接口连接钱箱线，输出脉冲驱动信号打开钱箱；
② 使用Bluetooth驱动钱箱，就需要将USB接口连接一个5V/2A电源适配器，使其给设备供电；
5V/2A电源适配器需要额外购买
③ USB供电后，Bluetooth处于工作状态，只要符合BT BLE标准的设备都可以发送指令驱动钱箱；
  

![](https://cdn.sunmi.com/public/image/mgt-document/2ce62cb9e56e464d94ceae5e13d11dd1.png)
# 三、指示灯说明  
|   
 | 指示灯  | 不亮  | 常亮  | 闪烁  |  
| --- | --- | --- | --- | --- |  
| ①  | 绿色灯  | 设备USB口没有接电源  | 工作正常  | 钱箱短路检测中，跟蓝色灯同步一直闪烁表示钱箱故障  |  
| ②  | 蓝色灯  | 蓝牙未连接  | 蓝牙连接并通讯中  | 钱箱短路检测中，跟绿色灯同步一直闪烁表示钱箱故障  |  
  

![](https://cdn.sunmi.com/public/image/mgt-document/7df86c5dd05f4f6ba71aefe1d5ac8733.png)
# 四、驱动指令集
## ESC p 驱动打开钱箱  
| [格式]  | ASCII  | ESC  | p  | m  | t1  | t2  |  
| --- | --- | --- | --- | --- | --- | --- |  
|   
 | Hex  | 1B  | 70  | 00  | t1  | t2  |  
|   
 | Decimal  | 27  | 112  | 0  | t1  | t2  |  
| 类型  | 说明  |  
| --- | --- |  
| [范围]  | m=0  |  
| [默认]  |   
 |  
| [描述]  | 在钱箱口输出脉冲，高电平时间为(t1×2 ms)，低电平时间为(t2×2 ms)。  |  
| [注释]  | 这个指令属于打印机命令，意味着应用程序支持打印机开钱箱就可以使用相同指令驱动钱箱  |  
| [举例]  |   
 |  
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
string send = "" + (char)(27) + (char)(112) + (char)(0) + (char)(60) + (char)(255);
System.IO.Ports.SerialPort com = new System.IO.Ports.SerialPort("COM7");
com.Open();
com.WriteLine(send);
com.Close();
  

```

## DLE EOT 读取钱箱开启状态  
| [格式]  | ASCII  | DLE  | EOT  | n  |  
| --- | --- | --- | --- | --- |  
|   
 | Hex  | 10  | 04  | 01  |  
|   
 | Decimal  | 16  | 4  | 1  |  
| 类型  | 说明  |  
| --- | --- |  
| [范围]  | n=1  |  
| [默认]  |   
 |  
| [描述]  | 获取钱箱打开/关闭实时状态。  |  
| [注释]  | 这个指令属于打印机命令，意味着应用程序支持打印机开钱箱就可以使用相同指令驱动钱箱  |  
| [举例]  |   
 |  
**收到指令后，响应返回1个字节消息，指示钱箱当前状态：**
  * 0x12 → 钱箱处于打开状态；
  * 0x16 → 钱箱处于关闭状态；


## GS I D 读取钱箱驱动器SN号  
| [格式]  | ASCII  | GS  | I  | D  |  
| --- | --- | --- | --- | --- |  
|   
 | Hex  | 1D  | 49  | 44  |  
|   
 | Decimal  | 29  | 73  | 68  |  
| 类型  | 说明  |  
| --- | --- |  
| [范围]  |   
 |  
| [默认]  |   
 |  
| [描述]  | 读取钱箱驱动器SN号  |  
| [注释]  |   
 |  
| [举例]  |   
 |  
**收到指令后，响应返回多个字节消息，包含起始符0x5F+SN号+结束符0x00：**
  * 0x5F B1 B2 B3 B4 B5 B6 B7 B8 B9 B10 B11 B12 B13 0x00 → B1…B13表示13字节的SN序列号。


## GS I A 读取钱箱驱动器固件版本号  
| [格式]  | ASCII  | GS  | I  | A  |  
| --- | --- | --- | --- | --- |  
|   
 | Hex  | 1D  | 49  | 41  |  
|   
 | Decimal  | 29  | 73  | 65  |  
| 类型  | 说明  |  
| --- | --- |  
| [范围]  |   
 |  
| [默认]  |   
 |  
| [描述]  | 读取钱箱驱动器固件版本号  |  
| [注释]  |   
 |  
| [举例]  |   
 |  
**收到指令后，响应返回多个字节消息，包含起始符0x5F+版本号+结束符0x00：**
  * 0x5F B1 B2 B3 0x00 → B1 B2 B3表示固件版本号。


## GS I B 读取钱箱驱动器制造商名称  
| [格式]  | ASCII  | GS  | I  | B  |  
| --- | --- | --- | --- | --- |  
|   
 | Hex  | 1D  | 49  | 42  |  
|   
 | Decimal  | 29  | 73  | 66  |  
| 类型  | 说明  |  
| --- | --- |  
| [范围]  |   
 |  
| [默认]  |   
 |  
| [描述]  | 读取钱箱驱动器制造商名称  |  
| [注释]  |   
 |  
| [举例]  |   
 |  
**收到指令后，响应返回多个字节消息，包含起始符0x5F+SN号+结束符0x00：**
  * 0x5F 0x53 0x55 0x4E 0x4D 0x49 0x00 → 返回制造名称0x53 0x55 0x4E 0x4D 0x49表示“SUNMI”


## US C D 进行钱箱故障检测  
| [格式]  | ASCII  | US  | C  | D  | MD1  | Y  |  
| --- | --- | --- | --- | --- | --- | --- |  
|   
 | Hex  | 1F  | 43  | 44  | 01  | 59  |  
|   
 | Decimal  | 31  | 67  | 68  | 1  | 89  |  
| 类型  | 说明  |  
| --- | --- |  
| [范围]  |   
 |  
| [默认]  |   
 |  
| [描述]  | 对钱箱进行故障检测，如果检测后钱箱管脚出现短路，则打开钱箱指令无法执行  |  
| [注释]  |   
 |  
| [举例]  |   
 |  
**收到指令后，响应返回1个字节消息，对应位为1表示对应管脚短路：**  
| 位  | 0/1  | 状态  |  
| --- | --- | --- |  
| 0  | 0  | Pin 1-2正常  |  
|   
 | 1  | Pin 1-2短路  |  
| 1  | 0  | Pin 1-3正常  |  
|   
 | 1  | Pin 1-3短路  |  
| 2  | 0  | Pin 1-4正常  |  
|   
 | 1  | Pin 1-4短路  |  
| 3  | 0  | Pin 2-3正常  |  
|   
 | 1  | Pin 2-3短路  |  
| 4  | 0  | Pin 2-4正常  |  
|   
 | 1  | Pin 2-4短路  |  
| 5  | 0  | Pin 3-4正常  |  
|   
 | 1  | Pin 3-4短路  |  
  

![](https://cdn.sunmi.com/public/image/mgt-document/c65e6cf01cdf4a05bb144bc98b792036.png)
上一篇：商米设备如何操作钱箱
下一篇：2、钱箱驱动器Windows驱动说明
