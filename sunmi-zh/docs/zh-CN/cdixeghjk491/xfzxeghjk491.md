---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzxeghjk491
---

# 15、云打印机Android SDK
更新时间：2026-07-07 16:54:06
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
# 概述
本⽂档主要介绍如何通过商米提供的SDK快速使用商⽶云打印机。
# SDK DEMO介绍
Demo APK 下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiCloudPrinterDemo.apk 
SDK Demo for Android 源代码下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiCloudPrinterDemo.zip 
  

1、DEMO使用介绍
![](https://cdn.sunmi.com/public/image/mgt-document/7cd7f79fd6684dcba69e78fbd6b0710f.png)
  

2、点击【Add Ble printer】、【Add BT Printer】、【Add USB Printer】进入打印机搜索界面
![](https://cdn.sunmi.com/public/image/mgt-document/f1090197319646df9301ac742be5f050.png)
  

3、通过蓝牙搜索到打印机后，点击【Wi-Fi Setting】进入wifi配置网络界面
![](https://cdn.sunmi.com/public/image/mgt-document/b83a31f70d754d30b17733e59d4b8047.png)
  

4、搜索wifi配置并使用对应的局域网配置
![](https://cdn.sunmi.com/public/image/mgt-document/5b9484266f7e4dbe806c5eeda0affffa.png)
# SDK快速入门
## 远程依赖
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
android {
    ...
}

dependencies {
    implementation 'com.sunmi:external-printerlibrary2:1.0.16'
}
  

```

## 本地依赖
1、打印SDK下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) externalprinterlibrary2-1.0.16-release.aar 
SDK最高已适配到 Android 14
2、将下载目录中的AAR文件导入到工程的libs目录下，并在gradle配置文件中添加aar的使用：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
android {
  ...
}
dependencies {
    implementation fileTree(dir: 'libs', include: ['*.jar','*.aar'])
    ...
}
  

```

Demo工程中的gradle使用版本为7.4，gradle插件版本为7.3,如果导入SDK项目编译失败可以考虑升级gradle版本
3、SDK中已经申请了大部分非运行时权限，但对于一些特殊权限需要app提前申请，例如使用蓝牙方式连接和控制打印机、给打印机配网：需要申请Android 定位权限（同时定位功能开启）
# SDK详细说明
SDK通过单例模式调用接口，主要包括以下几个部分：
  * 打印机获取：通过多种方式搜索云打印机；
  * 打印机使用：通过搜到的云打印机对象，直接控制打印机连接和打印；
  * 打印机配网：通过蓝牙方式搜索到打印机后，对打印机进行wifi账号信息配置操作；


# 获取打印机的方法
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| searchCloudPrinter  | 搜索打印机  |  
| stopSearch  | 停止搜索  |  
| createCloudPrinter  | 通过IP地址获取打印机  |  
  * **搜索打印机**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void searchCloudPrinter(Context context, @SearchMethod int method, SearchCallback callback) 
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| method  | int  | 搜索方式：   
  
SearchMethod.USB   
  
SearchMethod.LAN   
  
SearchMethod.BT  |  
| callback  | SearchCallback  | 新搜索到的打印机回调，将返回新发现的打印机对象  |  
注意使用BT方式搜索打印机要申请定位相关权限，并打开设备定位
使用LAN方式搜索打印机需要打印机已经加入所在局域网
  * **停止搜索打印机**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void stopSearch(Context context, @SearchMethod int method)
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| method  | int  | 需要停止搜索的方式  |  
开始搜索打印机后将持续发现打印机，需要在合适时机停止搜索释放系统资源
  * **使用静态IP地址**


如果对打印机已经配置好当前局域网的IP地址及端口号，可以通过如下方法直接获取打印机对象操作打印机
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
CloudPrinter createCloudPrinter(String staticIp, int port)
  

```
  
| 属性  | 类型  | 说明  |  
| --- | --- | --- |  
| staticIp  | String  | 指定连接的云打印机静态IP地址  |  
| port  | port  | 指定连接的云打印机端口号  |  
# 打印机使用方法
## **获取连接信息**
通过SearchCallback回调搜索打印机实例CloudPrinter，获取实例后即可操作打印机完成打印工作，实例支持以下方法
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
CloudPrinterInfo getCloudPrinterInfo()
  

```

使用此方法获取打印机连接配置信息，如当前打印机的IP地址或蓝牙mac地址，CloudPrinterInfo如下  
| 属性  | 类型  | 说明  |  
| --- | --- | --- |  
| name  | String  | 云打印机名称  |  
| vid  | int  | 云打印机通过USB搜索到时的设备VID  |  
| pid  | int  | 云打印机通过USB搜索到时的设备PID  |  
| mac  | String  | 云打印机通过蓝牙搜索到时的设备蓝牙地址  |  
| address  | String  | 云打印机通过局域网搜索到的IP地址  |  
| port  | int  | 云打印机通过局域网搜索到的端口地址  |  
## **设备连接接口**
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| void connect(final Context context, final ConnectCallback callback)  | 连接当前打印机  |  
| void release(Context context)  | 当不再使用打印机时释放打印机所占用系统资源  |  
| boolean isConnected()  | 返回当前打印机的连接状态  |  
  * **连接打印机**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void connect(final Context context, final ConnectCallback callback)
```


  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| callback  | ConnectCallback  | 打印机异步连接结果回调：   
  
onConnect() 打印机已连接上   
  
onFailed(String error) 打印机连接失败   
  
onDisConnect() 打印机已断开连接  |  
  * **释放打印机**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void release(Context context)
```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
  * **打印机是否连接**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
boolean isConnected()
  

```

返回值说明：如果已经成功连接打印机返回true 否则返回false
## **本地缓存数据接口**
在构建小票内容后需要使用触发命令将小票数据发给打印机
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| void commitTransBuffer(ResultCallback callback)  | 将当前已经缓存的内容立刻发给打印机打印  |  
| void clearTransBuffer()  | 清除未打印的打印内容  |  
  * **提交打印内容打印**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void commitTransBuffer(ResultCallback callback)
```

参数：callback 打印结果的回调
void onComplete() 返回：提交打印内容已经打印完成
void onFailed(CloudPrinterStatus status) 返回：提交的打印内容打印失败，失败返回status状态值
  * **清除待提交打印的内容**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void clearTransBuffer()
```

可清除待提交的打印内容;
当打印机异常时可清除已提交但未打印的数据,使打印机不会重打印;
## **基础功能接口**
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| void setPrintDensity(int density)  | 设置打印机的浓度，取值范围 70~130，255 代表恢复到默认值，默认值 100  |  
| void setPrintSpeed(int speed)  | 设置打印速度，取值范围 0~250，255 代表恢复到默认值  |  
| void setPrintCutter(CutterMode mode)  | 设置打印机的切刀模式，见CutterMode  |  
| void setPrintMode(int mode)  | 设置打印机的打印模式，取值范围1~3  |  
| void selectAsciiCharFont(int select)  | 设置Latin字符使用的字库，0 代表默认使用点阵字库，1 代表使用内置矢量字库，   
大于等于 128 代表使用第(charFont-127)种第三方矢量字库  |  
| void selectCjkCharFont(int select)  | 设置中日韩字符使用的字库，0 代表默认使用点阵字库，1 代表使用内置矢量字库，   
大于等于 128 代表使用第(charFont-127)种第三方矢量字库  |  
| void selectOtherCharFont(int select)  | 设置其他字符使用的字库，0 代表使用内置矢量字库，   
大于等于 128 代表使用第(charFont-127)种第三方矢量字库  |  
| void setEncodeMode(EncodeType type)  | 设置传输文本的方法，将影响appendRawData方法的解码方式，解码见EncodeType  |  
| void restoreDefaultSettings()  | 恢复默认配置  |  
  * **设置打印机的浓度**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setPrintDensity(int density)
  

```

参数：density表示可设置的浓度范围，浓度范围在70-130之间，默认打印机浓度100
  * **设置打印机的打印速度**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setPrintSpeed(int speed)
  

```

参数：speed表示可设置的速度范围，速度范围在0-250，速度设置越快相应的打印质量也会下降
  * **设置打印机的切刀模式**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setPrintCutter(CutterMode mode)
  

```

参数：CutterMode

  
| 枚举  | 说明  |  
| --- | --- |  
| CutterMode.NORMAL  | 正常模式，根据指令进行全切或半切  |  
| CutterMode.HALF  | 半切模式，所有切刀均执行半切  |  
| CutterMode.ALL  | 全切模式，所有切刀均执行全切  |  
| CutterMode.NULL  | 不切模式，所有切刀均不执行  |  
  * **设置打印模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setPrintMode(int mode)
```

参数：mode 可设置的打印机模式，1: 票据模式；2: 标签模式；3: 无底纸模式
  * **设置字体字库类型**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void selectAsciiCharFont(int select)
void selectCjkCharFont(int select)
void selectOtherCharFont(int select)
  

```

参数：select表示选择当前使用的字库，默认0使用点阵字库（点阵字库比较清晰但不能自由调整大小），1使用内置矢量字库，大于127表示使用其他第三方字库，需要预置进打印设备
  * **设置传输文本内容的编码方式**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setEncodeMode(EncodeType type)
  

```

参数：EncodeType

  
| 枚举  | 说明  |  
| --- | --- |  
| EncodeType.ASCII  | 打印机对传输指令数据按照ASCII单字符解码  |  
| EncodeType.GB18030  | 打印机按GB8030解码  |  
| EncodeType.BIG5  | 打印机按BIG5解码  |  
| EncodeType.SHIFT_JIS  | 打印机按SHIFT_JIS解码  |  
| EncodeType.JIS_0208  | 打印机按JIS_0208解码  |  
| EncodeType.KSC_5601  | 打印机按KSC_5601解码  |  
| EncodeType.UTF_8  | 打印机按UTF8解码  |  
”传输文本内容“指通过appendRawData方法传输指令内容时，由于打印机处于不同的编码类型，当编码类型不匹配时，会对文本内容打印效果产生差异，所以可通过此方法，将打印机设置成与传输文本内容符合的编码类型，让打印机可以正常进行打印。
  * **恢复打印机的默认配置**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void restoreDefaultSettings()
  

```



## **设备信息接口**
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| void getDeviceState(StatusCallback statusCallback)  | 获取打印机当前的状态  |  
| void getDeviceSN(PropCallback propCallback)  | 获取打印设备的sn  |  
| void getDeviceModel(PropCallback propCallback)  | 获取打印设备的Model值  |  
| void getPrintMode(PropCallback propCallback)  | 获取打印模式  |  
  * **获取打印机状态**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getDeviceState(StatusCallback statusCallback)
  

```

参数：statusCallback 状态回调方法 void onResult(CloudPrinterStatus status)

  
| 枚举  | 说明  |  
| --- | --- |  
| CloudPrinterStatus.OFFLINE  | 打印机离线不可用  |  
| CloudPrinterStatus.UNKNOWN  | 打印机未知状态  |  
| CloudPrinterStatus.RUNNING  | 打印机可以打印  |  
| CloudPrinterStatus.NEAR_OUT_PAPER  | 打印机纸即将用完  |  
| CloudPrinterStatus.OUT_PAPER  | 打印机缺纸  |  
| CloudPrinterStatus.JAM_PAPER  | 打印机堵纸  |  
| CloudPrinterStatus.PICK_PAPER  | 打印机待取纸  |  
| CloudPrinterStatus.COVER  | 打印机开盖  |  
| CloudPrinterStatus.OVER_HOT  | 打印机过热  |  
| CloudPrinterStatus.MOTOR_HOT  | 打印机马达过热  |  
  * **获取打印机SN**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getDeviceSN(PropCallback propCallback)
  

```

参数：propCallback 属性的回调方法参见 void onProperty(String result) 返回SN
  * **获取打印机Model值**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getDeviceModel(PropCallback propCallback)
  

```

参数：propCallback 属性的回调方法参见 void onProperty(String result) 返回 Model值
  * **获取打印模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getPrintMode(PropCallback propCallback)
```

参数：propCallback 属性的回调方法参见 void onProperty(String result) 返回打印机类型
## **打印内容接口**
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| void appendText(String text)  | 追加打印文本内容  |  
| void printText(String text)  | 将打印内容按一行直接打印  |  
| void printColumnsText(String[] colsTextArr, int[] colsWidthArr, AlignStyle[] colsAlign)  | 一行内按列打印内容  |  
| void printImage(Bitmap bitmap, ImageAlgorithm mode)  | 打印图片  |  
| void printBarcode(String text, BarcodeType type, int height, int size, HriStyle style)  | 打印条形码  |  
| void printQrcode(String text, int size, ErrorLevel level)  | 打印二维码  |  
| void appendRawData(byte[] data)  | 发送原始ESC/POS指令  |  
  * **打印文本内容**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void appendText(String text)
void printText(String text)
  

```

参数：text 要打印的文本内容，可以传入任意字符


1、appendText和printText区别主要在于后者将直接将内容打印在一行中，而前者通过打印布局命令可以在一行里打印多种样式的内容
2、传入的字符受setEncodeMode方法影响转换为对应字符编码发送给打印机，所以针对一般场景默认GB18030即可，如果是日本建议使用SHIFT_JIS字符集编码，韩国使用KSC_5601编码，其他国家使用utf-8编码
  * **按列打印文本内容**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printColumnsText(String[] colsTextArr, int[] colsWidthArr, AlignStyle[] colsAlign)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| colsTextArr  | array  | 在本行中每一列要打印的文本内容，可以为空字符串  |  
| colsWidthArr  | array  | 在本行中每一列打印文本内容字符长度  |  
| colsAlign  | array  | 在本行中每一列中打印文本内容的对齐方式  |  
使用按列打印文本内容会计算文本大小，所以会将字体的缩放默认变为原始大小，同时文本内容数组、字符长度数组和对齐方式数组的长度必须一致
  * **打印图片**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printImage(Bitmap bitmap, ImageAlgorithm mode)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| bitmap  | Bitmap  | 打印图片的Bitmap对象  |  
| mode  | ImageAlogrithm  | 图片的转换算法，说明见下  |  
参数：ImageAlogrithm  
| 枚举  | 说明  |  
| --- | --- |  
| ImageAlogrithm.BINARIZATION  | 默认方式，二值化算法，将图片内容二值化处理打印黑白效果  |  
| ImageAlogrithm.DITHERING  | 抖动灰度算法，将图片抖动处理，呈现出有灰度的区域效果  |  
  * **打印条形码**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printBarcode(String text, BarcodeType type, int height, int size, HriStyle style)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | String  | 条形码内容那个  |  
| type  | BarcodeType  | 条形码类型，说明见下  |  
| height  | int  | 条形码高度  |  
| size  | int  | 条形码大小  |  
| style  | HriStyle  | Hri样式，说明见下  |  
参数：BarcodeType  
| 枚举  | 说明  |  
| --- | --- |  
| BarcodeType.UPCA  | 固定字符内容0～9，可打印11-12个字符  |  
| BarcodeType.UPCE  | 固定字符内容0～9，可打印6-7个字符  |  
| BarcodeType.EAN13  | 固定字符内容0～9，可打印12-13个字符  |  
| BarcodeType.EAN8  | 固定字符内容0～9，可打印7-8个字符  |  
| BarcodeType.CODE39  | 固定字符内容0~9,A~Z,SP,$,%,*,+,-,.,/   
  
可打印1-64个字符  |  
| BarcodeType.ITF  | 固定字符内容0～9，可打印2-64个字符（偶数个）  |  
| BarcodeType.CODABAR  | 固定字符内容0~9,A~D,a~d,$,+,-,.,/,:   
  
可变的可打印字符数  |  
| BarcodeType.CODE93  | 可打印1-64个字符  |  
| BarcodeType.CODE128  | 可打印2-64个字符  |  
参数：HriStyle  
| 枚举  | 说明  |  
| --- | --- |  
| HriStyle.HIDE  | 隐藏HRI字符  |  
| HriStyle.ABOVE  | HRI字符在条码上方  |  
| HriStyle.BELOW  | HRI字符在条码下方  |  
| HriStyle.BOTH  | HRI字符在条码上下  |  
  * **打印二维码**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printQrcode(String text, int size, ErrorLevel level)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | String  | 要打印二维码的内容  |  
| size  | int  | 二维码单位块大小  |  
| level  | ErroLevel  | 二维码的纠错等级  |  
参数：ErroLevel  
| 枚举  | 说明  |  
| --- | --- |  
| ErroLevel.L  | 纠错等级7%  |  
| ErroLevel.M  | 纠错等级15%  |  
| ErroLevel.Q  | 纠错等级25%  |  
| ErroLevel.H  | 纠错等级30%  |  
  * **打印原始指令**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void appendRawData(byte[] data)
  

```

参数：data 表示要发送的打印内容十六进制数组或ESC/POS指令


## **布局控制接口**
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| void initStyle()  | 重新初始化布局样式  |  
| void setPrintWidth(int printWidth)  | 设置可打印区域宽度  |  
| void setLeftSpace(int leftSpace)  | 设置左边距的宽度  |  
| void setLineSpacing(int lineSpacing)  | 设置行间距  |  
| void setBlackWhiteReverseMode(boolean enable)  | 设置反白  |  
| void setUnderlineMode(UnderlineStyle mode)  | 设置下划线  |  
| void setBoldMode(boolean enable)  | 设置加粗  |  
| void setUpsideDownMode(boolean enable)  | 设置倒置  |  
| void setCharacterSize(int characterWidth, int characterHeight)  | 设置打印文本倍高和倍宽  |  
| void setAsciiSize(int size)  | 设置Latin字符的大小（字库为矢量字库有效）  |  
| void setCjkSize(int size)  | 设置中日韩字符的大小（字库为矢量字库有效）  |  
| void setOtherSize(int size)  | 设置其他字符的大小（字库为矢量字库有效）  |  
| void dotsFeed(int dots)  | 按点行走纸  |  
| void lineFeed(int lines)  | 按行间距走纸  |  
| void horizontalTab(int n)  | 横向跳格数（即tab数）  |  
| void setAbsolutePrintPosition(int horizontalPosition)  | 设置绝对打印位置  |  
| void setRelativePrintPosition(int horizontalPosition)  | 设置相对打印位置  |  
| void setAlignment(AlignStyle alignment)  | 设置对齐方式  |  
| void restoreDefaultLineSpacing()  | 恢复默认行间距  |  
  * **初始化样式**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void initStyle()
```



这个方法将恢复设置的字体大小、加粗、下划线等所有样式设置，同时如果使用appendText方法添加的打印内容也将清除不再打印
  * **设置打印区域宽度**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setPrintWidth(int printWidth)
```

参数：printWidth 可打印区域的像素宽度，大小不可超过打印机纸张宽度
  * **设置打印左边距**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setLeftSpace(int leftSpace)
```

参数：leftSpace 打印的左边距像素padding，大小不可超过当前打印机纸张宽度，会影响设置的可打印区域宽度大小
  * **设置打印机行间距**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setLineSpacing(int lineSpacing)
```

参数：lineSpacing 打印机行间距大小，如果行间距小于当前最大高度字体或图像，此行将以最大高度为准
  * **设置打印机文本内容是否反白**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setBlackWhiteReverseMode(boolean enable)
  

```

参数：enable true 将之后打印内容设置为反白 false 将之后打印内容设置为正常
  * **设置打印机文本内容是否添加下划线**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setUnderlineMode(UnderlineStyle mode)
```

参数：mode

  
| 枚举  | 说明  |  
| --- | --- |  
| UnderlineStyle.EMPTY  | 无下划线  |  
| UnderlineStyle.ONE  | 一点行下划线  |  
| UnderlineStyle.TWO  | 两点行下划线  |  
  * **设置打印机文本内容是否加粗**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setBoldMode(boolean enable)
```

参数：enable true 之后打印内容将加粗 false 之后打印内容将取消加粗
  * **设置打印机文本内容是否倒置**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setUpsideDownMode(boolean enable)
```

参数：enable true 之后打印文本内容会产生颠倒效果 false 之后打印文本内容恢复非颠倒效果
  * **设置打印机文本内容倍高和倍宽**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setCharacterSize(int characterWidth, int characterHeight)
  

```

参数：characterWidth 打印字符横向放大系数 1-8；characterHeight 打印字符纵向放大系数 1-8
  * **设置打印机文本字符大小**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setAsciiSize(int size)
void setCjkSize(int size)
void setOtherSize(int size)
```

参数：size 各个字符下的字体大小，4-255像素点，默认Latine字母12像素，其他字符24像素


必须设置打印机为矢量字库时，字符大小设置才有效
  * **按像素点走纸**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void dotsFeed(int dots)
```

参数：dots 打印机向前进纸的像素点数
  * **按行间距走纸**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void lineFeed(int lines)
```

参数：lines 打印机向前进纸指定的行数


受行间距影响，如果行间距设置为0，此方法将无效
  * **横向跳格**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void horizontalTab(int n)
```

参数： n 移动n个横向跳格位置
  * **设置当前距离行首的打印位置**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setAbsolutePrintPosition(int horizontalPosition)
```

参数：horizontalPostiion 距离打印行首的位置距离，单位像素
  * **设置当前距离上一个打印结束位置**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setRelativePrintPosition(int horizontalPosition)
```

参数：horizontalPosition 距离打印结束位置的距离，单位像素
  * **设置之后打印内容的对齐方式**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setAlignment(AlignStyle alignment)
```


  
| 枚举  | 说明  |  
| --- | --- |  
| AlignStyle.LEFT  | 居左  |  
| AlignStyle.CENTER  | 居中  |  
| AlignStyle.RIGHT  | 居右  |  
  * **恢复默认行间距**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void restoreDefaultLineSpacing()
```

默认行间距为30像素
## **机械控制接口**
  * **方法列表**

  
| 方法  |   
 |  
| --- | --- |  
| void cutPaper(boolean full)  | 在当前位置切刀  |  
| void postCutPaper(boolean full, int dis)  | 进纸切刀  |  
| void openCashBox()  | 开启钱箱  |  
  * **打印机切纸**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void cutPaper(boolean full)
```

参数：full=true 全切；=false 半切
  * **打印机进纸切纸**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void postCutPaper(boolean full, int dis)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| full  | bool  | true 全切 false 半切  |  
| dis  | int  | 打印机从当前位置继续走纸的距离，之后再执行切纸  |  
两种切纸方式中的半切和全切均受setPrintCutter方法影响
## **标签打印接口**
注意标签打印接口需要打印机支持标签打印同时打印机设置为标签模式才可以正常使用
  * **方法列表**

  
| **方法**  | **说明**  |  
| --- | --- |  
| void sendTSPLData(String data)   | 发送TSPL指令内容  |  
| void initLabelStyle(int width, int height, int gap, int offset, boolean direction, boolean mirror)  | 初始化标签布局  |  
| void renderText(String text, int x, int y, FontType fontType, RotationStyle rotation, int ratioX, int ratioY, AlignStyle align, boolean isBold)  | 绘制标签文本  |  
| void renderBarCode(String code, int x, int y, BarcodeType type, int height, int dots, HriStyle2 style, RotationStyle rotation)   | 绘制标签条码  |  
| void renderQrCode(String text, int x, int y, ErrorLevel errorCode, int size, RotationStyle rotation)  | 绘制标签二维码  |  
| void renderBitmap(Bitmap bitmap, int x, int y, int threshold)   | 绘制标签位图  |  
| void renderArea(ShapeStyle style, int x, int y, int w, int h, int thickness)  | 绘制标签图形区域  |  
| void printLabel(int count, boolean cut)  | 打印已绘制的标签  |  
  * **发送原始TSPL指令**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void sendTSPLData(String data) 
```

参考[打印指令集](https://docs.sunmi.com/zh-CN/cdixeghjk491/xfrmeghjk546)直接给打印机发送TSPL指令
  * **初始化标签布局**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void initLabelStyle(int width, int height, int gap, int offset, boolean direction, boolean mirror) 
```

绘制标签的初始布局参数，打印标签前必须设置  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| width  | int  | 标签宽度，单位 mm  |  
| height  | int  | 标签高度，单位 mm  |  
| gap  | int  | 标签间距，单位 mm  |  
| offset  | int  | 每张标签额外送纸偏移，单位mm  |  
| direction  | boolean  | 标签方向是否与打印方向一致  |  
| mirror  | boolean  | 是否镜像打印  |  
  * **绘制文本**


绘制文本内容在已初始化的标签布局中
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void renderText(String text, int x, int y, FontType fontType, RotationStyle rotation, int ratioX,
                int ratioY, AlignStyle align, boolean isBold)
```

绘制文本不会立即打印，只是将内容绘制在打印机内的图像缓冲区中  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | String  | 文本内容  |  
| x  | int  | 相对标签的 x 坐标，单位像素  |  
| y  | int  | 相对标签的 y 坐标，单位像素  |  
| fontType  | FontType  | 字体类型  |  
| rotation  | RotationStyle  | 相对坐标的旋转角度  |  
| ratioX  | int  | 横向放大倍率，1-10  |  
| ratioY  | int  | 纵向放大倍率，1-10  |  
| align  | AlignStyle  | 文本对齐方式  |  
| isBold  | boolean  | 是否加粗  |  
FontType  
| 枚举  | 说明  |  
| --- | --- |  
| ASCII_12x24  | 12×24的英文字符  |  
| GBK_24x24  | 24×24的中文字符  |  
| ASCII_8x16  | 8×16的英文字符  |  
| GBK_16x16  | 16×16的中文字符  |  
RotationStyle  
| 枚举  | 说明  |  
| --- | --- |  
| ROTATION_0  | 不旋转  |  
| ROTATION_90  | 90度旋转  |  
| ROTATION_180  | 180度旋转  |  
| ROTATION_270  | 270度旋转  |  
AlignStyle  
| 枚举  | 说明  |  
| --- | --- |  
| AlignStyle.LEFT  | 居左  |  
| AlignStyle.CENTER  | 居中  |  
| AlignStyle.RIGHT  | 居右  |  
  * **绘制条码**


绘制条码内容在已初始化的标签布局中
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void renderBarCode(String code, int x, int y, BarcodeType type, int height, int dots, HriStyle2 style, RotationStyle rotation)
```

绘制条码不会立即打印，只是将内容绘制在打印机内的图像缓冲区中  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | String  | 条码内容  |  
| x  | int  | 相对标签的 x 坐标，单位像素  |  
| y  | int  | 相对标签的 y 坐标，单位像素  |  
| type  | BarcodeType  | 条码类型，同printBarcode参数  |  
| height  | int  | 条码高度  |  
| dots  | int  | 条码码宽  |  
| style  | HriStyle2  | 标签的HRI 显示位置  |  
| rotation  | RotationStyle  | 旋转角度，同renderText参数  |  
HriStyle2  
| 枚举  | 说明  |  
| --- | --- |  
| HIDE  | 隐藏  |  
| LEFT  | 左下  |  
| MID  | 中下  |  
| RIGHT  | 右下  |  
  * **绘制二维码**


绘制二维码内容在已初始化的标签布局中
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void renderQrCode(String text, int x, int y, ErrorLevel errorCode, int size, RotationStyle rotation)
```

同样绘制内容不会直接打印而是缓存在打印机缓冲区中  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | String  | 二维码内容  |  
| x  | int  | 相对标签的 x 坐标，单位像素  |  
| y  | int  | 相对标签的 y 坐标，单位像素  |  
| errorCode  | ErrorLevel  | 纠错等级，同printQrcode参数  |  
| size  | int  | 二维码尺寸  |  
| rotation  | RotationStyle  | 旋转角度，同renderText参数  |  
  * **绘制位图**


绘制图片内容在已初始化的标签布局中
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void renderBitmap(Bitmap bitmap, int x, int y, int threshold)
```

同样绘制内容不会直接打印而是缓存在打印机缓冲区中  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| bitmap  | Bitmap  | 位图对象  |  
| x  | int  | 相对标签的 x 坐标，单位像素  |  
| y  | int  | 相对标签的 y 坐标，单位像素  |  
| threshold  | int  | 二值化阈值，0~255,用于调整图片二值化的显示效果，值越大细节显示更多  |  
  * **绘制图形区域**


绘制图形内容在已初始化的标签布局中
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void renderArea(ShapeStyle style, int x, int y, int w, int h, int thickness)
```

同样绘制内容不会直接打印而是缓存在打印机缓冲区中  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| style  | ShapeStyle  | 图形形状枚举  |  
| x  | int  | 相对标签的 x 坐标，单位像素  |  
| y  | int  | 相对标签的 y 坐标，单位像素  |  
| w  | int  | 图形宽度或直径（当选择圆形时）  |  
| h  | int  | 图形高度  |  
| thickness  | int  | 线框线宽  |  
ShapeStyle  
| 枚举  | 说明  |  
| --- | --- |  
| RECT_FILL  | 矩形区域填充  |  
| RECT_WHITE  | 矩形区域擦除  |  
| RECT_REVERSE  | 矩形区域反色  |  
| CIRCLE  | 绘制圆形  |  
| BOX  | 绘制矩形框  |  
  * **打印标签**


将当前打印机缓冲区中的标签内容立即打印
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printLabel(int count, boolean cut)
```

  
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| count  | int  | 打印数量  |  
| cut  | boolean  | 是否在打印结束后切纸  |  
# WIFI配网方法
  * **方法列表**


可通过如下方法为打印机配置局域网参数。
因为配网方法基于蓝牙实现，所以需要获取系统蓝牙权限
同时要对打印机进行配网，需要打印机长按配网键【进入配网模式】！  
| 方法  | 说明  |  
| --- | --- |  
| startPrinterWifi  | 进入打印机配网模式  |  
| searchPrinterWifiList  | 获取打印机搜索到的WIFI信息列表  |  
| setPrinterWifi  | 配置打印机的WIFI信息  |  
| deletePrinterWifi  | 删除当前打印机存储的WIFI信息  |  
| exitPrinterWifi  | 结束打印机WIFI配置，并退出打印机配网  |  
## **自动进入打印机配网模式**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void startPrinterWifi(Context context, CloudPrinter cloudPrinter, String sn)
```
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| cloudPrinter  | CloudPrinter  | 通过蓝牙方式搜索到的打印机名  |  
| sn  | String  | 指定打印机的SN  |  
打印机提供自动进入和手动进入两种配网模式，长按打印机底部【配网键】也可以进入配网模式。
## **获取打印机搜索到的WIFI列表**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void searchPrinterWifiList(Context context, CloudPrinter cloudPrinter, WifiResult result)
  

```
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| cloudPrinter  | CloudPrinter  | 通过蓝牙方式搜索到的打印机  |  
| result  | WifiResult  | 搜索结果回调  |  
参数：WifiResult
  * onRouterFound(Router router) 返回当前打印机搜索到的wifi信息

  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| name  | String  | 此wifi名称  |  
| hasPwd  | bool  | 此wifi是否有密码  |  
| pwd  | String  | 当前wifi的密码（未配置成打印机wifi时为空）  |  
| rssi  | int  | wifi信号的强度 0-4  |  
| essid  | byte[]  | wifi的essid，用于配网  |  
  * onFinishi 搜索结束
  * onFailed 此次搜索失败


## **选择某一个Wi-Fi网络为打印机配置网络**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void setPrinterWifi(Context context, CloudPrinter cloudPrinter, byte[] ssid, String password, SetWifiCallback callback)
  

```
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| cloudPrinter  | CloudPrinter  | 通过蓝牙方式搜索到的打印机  |  
| ssid  | byte[]  | wifi的essid，通过搜索到的wifi信息获取  |  
| password  | String  | 指定wifi的密码  |  
| callback  | SetWifiCallback  | 设置打印机wifi的过程回调  |  
参数：SetWifiCallback
  * onSetWifiSuccess 回调当前配置信息已保存到打印机中
  * onConnectWifiSuccess 回调当前配置信息已连接到局域网中
  * onConnectWifiFailed 回调当前配置信息连接局域网失败


## **删除打印机的WIFI配置信息**
删除打印机内部存储的WIFI配置信息后，打印机将断开所有wifi连接；
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void deletePrinterWifi(Context context, CloudPrinter cloudPrinter)
  

```
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| cloudPrinter  | CloudPrinter  | 通过蓝牙方式搜索到的打印机  |  
## **退出配网模式**
结束并退出打印机的配网过程，如果操作过搜索和配置打印机网络最好结束时调用
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void exitPrinterWifi(Context context, CloudPrinter cloudPrinter)
  

```
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| cloudPrinter  | CloudPrinter  | 通过蓝牙方式搜索到的打印机  |  
上一篇：14、云打印机iOS SDK多台同时连接
下一篇：16、云打印机macOS SDK
