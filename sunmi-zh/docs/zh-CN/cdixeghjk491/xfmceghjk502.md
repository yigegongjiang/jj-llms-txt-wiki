---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmceghjk502
---

# 13、云打印机iOS SDK
更新时间：2025-12-19 22:50:17
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
# 概述
本⽂档主要介绍如何通过商米提供的SDK快速使用商⽶云打印机。
# SDK DEMO介绍
SDK for iOS代码下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SMPrinterSDKProject V1.6.6.zip 
1、DEMO使用介绍
  

![](https://cdn.sunmi.com/public/image/mgt-document/00c621539c684509b659ce8eae193e9f.png)
2、点击【Add Ble printer】进入蓝牙打印机搜索界面
  

![](https://cdn.sunmi.com/public/image/mgt-document/ae6b480e99184184b731feda9c14fb3f.png)
3、点击【Add IP printer】进入局域网打印机搜索界面
  

![](https://cdn.sunmi.com/public/image/mgt-document/b7585ff9d06b42979a93b47c1a62c2f2.png)
4、点击【Wi-Fi Setting】进入wifi配置网络界面
  

![](https://cdn.sunmi.com/public/image/mgt-document/3bf3fcf052a64516af96305eef0c96ea.png)
# SDK快速入门
1.Print SDK for iOS下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiPrinterSDK.xcframework V1.6.6.zip 
2.目前 SDK 只支持手动集成。
3.将 `SunmiPrinterSDK.framework` 导入到 Xcode 工程目录下。
4.在需要使用 API 的类中添加 `#import <SunmiPrinterSDK/SunmiPrinterSDK.h>` 头文件
5.使用蓝牙打印机，需要在 【info.plist】 中添加两个蓝牙权限
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NSBluetoothPeripheralUsageDescription bluetooth usage description 
NSBluetoothAlwaysUsageDescription bluetooth usage description
  

```

  1. 使用 TCP/IP 局域网打印机，需要增加iOS网络发现权限控制。


>   1. 向苹果申请组播权限：<https://developer.apple.com/contact/request/networking-multicast> （大概需要3天左右，成功后会有邮件返回）
>   2. 申请成功后，在开发者账号中，对【appid】进行编辑，拉倒最下面会多一个【Additional Capabilities】的选项，将【Multicast Networking】打钩；
>   
> 
> ![](https://cdn.sunmi.com/public/image/mgt-document/4d1b858fafcf4e2682c65907de5ae608.png)
>   3. 在 app 的【xxxx.entitlements】配置文件中添加 `com.apple.developer.networking.multicast` 的布尔值为 `YES`
>   4. 如果是 iOS14版本，就需要添加本地网络的权限
> bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml
> 
```
<key>NSLocalNetworkUsageDescription</key>
<string>Local Network</string>
  
> 
```

>   5. 在 【info.plist】 中添加如下权限
> 

> bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml
> 
```
<key>NSBonjourServices</key>
<array>
	<string>_afpovertcp._tcp.</string>
</array>
  
> 
```

# SDK 说明
  1. 此 SDK 在真机或模拟器环境中均可使用，建议使用与iOS12或以上系统版本中；
  2. SDK 包含搜索蓝牙打印机、连接、配网模块，局域网打印机搜索、连接模块以及提供了方便开发者调用打印机的指令的接口方法；
  3. 便于开发者使用，提供了 demo 用于开发者调试；
  4. 开发环境 Mac OS、Xcode、iphone设备（需要 iOS 11 以上）。


# 功能模块说明
  1. 蓝牙打印机模块：支持搜索、连接、配网、发送指令数据等；
  2. 局域网打印机模块：支持搜索、连接、发送指令数据等；
  3. 指令模块：对打印相关指令进行封装；


# 蓝牙打印机属性类 SunmiBlePrinterModel  
| 属性名  | 类型  | 说明  |  
| --- | --- | --- |  
| peripheral  | CBPeripheral  | 蓝牙设备  |  
| rssi  | NSNumber  | 发现蓝牙设备时取到的信号强度值  |  
| deviceName  | NSString  | 蓝牙设备名称  |  
| uuidString  | NSString  | 蓝牙设备 UUID  |  
# 蓝牙打印机管理类 SunmiPrinterManager
## **属性列表**  
| 属性名  | 类型  | 说明  |  
| --- | --- | --- |  
| bluetoothDelegate  | id  | 代理协议  |  
## **方法列表**  
| 方法  | 说明  |  
| --- | --- |  
| - (BOOL)getDeviceBluetoothAvailable;  | 蓝牙是否可用  |  
| - (void)setDeviceSearchState:(BOOL)canSearchDevice;  | 控制本机蓝牙状态更新后是否可以扫描连接外设  |  
| - (BOOL)bluetoothIsConnection;  | 当前蓝牙设备连接状态  |  
| - (void)scanPeripheral;  | 扫描蓝牙打印机设备  |  
| - (void)cancelScan;  | 取消扫描蓝牙打印机设备  |  
| - (void)connectPeripheral:(CBPeripheral *_Nonnull)peripheral;  | 建立蓝牙打印机设备的连接  |  
| - (void)disConnectPeripheral;  | 断开和蓝牙打印机设备的连接  |  
| - (void)deviceDisConnectWithBlock:(disConnectionDeviceBlock _Nullable )block;  | 设备连接断开了的回调处理  |  
| -(void)sendPrintData:(NSData *_Nullable)data;  | 向蓝牙打印机发送数据  |  
| - (void)sendSuccess:(sendDeviceDataSuccessBlock _Nullable )block;  | 发送数据成功的回调  |  
| - (void)sendFail:(sendDeviceDataFailBlock _Nullable )block;  | 发送数据失败的回调  |  
| - (void)receivedDeviceData:(receivedDeviceDataBlock _Nullable )block;  | 接收数据的回调  |  
> receivedDeviceDataBlock数据回调的说明
>   1. taskNumber：打印机最近一个打印任务的编号，发送获取打印机最近一个打印任务的编号时返回编号，否则为nil
>   2. deviceSN：打印机sn，发送获取打印机SN指令会返回打印机sn编号，否则为 nil
>   3. printerStatus：打印机状态，发送获取打印机状态会返回打印机状态枚举值，否则返回 -1
> 
  
| 打印机状态枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMPrinterStatus_Normal  | NSInteger  | 1  |  
| SMPrinterStatus_Printing  | NSInteger  | 2  |  
| SMPrinterStatus_NoPaper  | NSInteger  | 3  |  
| SMPrinterStatus_RollIsExhausted  | NSInteger  | 4  |  
| SMPrinterStatus_PaperJam  | NSInteger  | 5  |  
| SMPrinterStatus_NoPaperPickup  | NSInteger  | 6  |  
| SMPrinterStatus_CoverOpened  | NSInteger  | 7  |  
| SMPrinterStatus_HeadOverheating  | NSInteger  | 8  |  
| SMPrinterStatus_MotorOverheating  | NSInteger  | 9  |  
## **协议列表**  
| 方法  | 说明  |  
| --- | --- |  
| - (void)discoveredDevice:(SunmiBlePrinterModel *_Nonnull)device;  | 当搜索到设备时，会执行代理方法，可以在此协议方法中获取扫描到的设备，会执行多次，直到搜索结束  |  
| - (void)didCancelSearching;  | 取消搜索之后，执行代理方法，在此代理方法中做其他逻辑处理  |  
| - (void)didConectPrinter;  | 连接设备成功时，执行代理方法  |  
| - (void)willDisconnectPrinter;  | 断开和蓝牙打印机的连接时，会执行代理方法  |  
# 局域网打印机属性类 SunmiIpPrinterModel  
| 属性名  | 类型  | 说明  |  
| --- | --- | --- |  
| deviceIP  | NSString  | 设备IP  |  
| deviceName  | NSString  | 设备名称  |  
| devicePort  | NSNumber  | 设备端口  |  
# 局域网打印机管理类 SunmiPrinterIPManager
## **属性列表**  
| 属性名  | 类型  | 说明  |  
| --- | --- | --- |  
| delegate  | id  | 代理协议  |  
## **方法列表**  
| 方法  | 说明  |  
| --- | --- |  
| - (void)startSearchPrinterWithIp:(NSString *)searchPrinter;  | 搜索 IP 打印机设备  |  
| - (void)connectSocketWithIP:(NSString *)ip  | 传入设备的 IP 地址，尝试连接设备  |  
| - (void)deviceDisConnectWithBlock:(void(^_Nullable)(NSError * _Nullable error))block;  | 设备连接断开了并返回错误信息  |  
| - (BOOL)IsConnectedIPService;  | 当前 IP 地址打印机是否连接  |  
| - (void)disConnectIPService;  | 断开和 IP 地址打印机的连接  |  
| - (void)controlDevicePrintingData:(NSData *)ipData;  | 向设备发送打印数据  |  
| - (void)controlDevicePrintingData:(NSData *)ipData success:(SuccessBlock)success fail:(FailureBlock)fail;  | 发送数据、成功/失败的回调  |  
| - (void)controlDevicePrintingData:(NSData *)ipData success:(SuccessBlock)success fail:(FailureBlock)fail response:(ResponseBlock)response;  | 发送数据、成功/失败的回调、接收数据的回调  |  
> receivedIPDeviceDataBlock数据回调的说明
>   1. taskNumber：打印机最近一个打印任务的编号，发送获取打印机最近一个打印任务的编号时返回编号，否则为nil
>   2. deviceSN：打印机sn，发送获取打印机SN指令会返回打印机sn编号，否则为 nil
>   3. printerStatus：打印机状态，发送获取打印机状态会返回打印机状态枚举值，否则返回 -1
> 
  
| 打印机状态枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMPrinterStatus_Normal  | NSInteger  | 1  |  
| SMPrinterStatus_Printing  | NSInteger  | 2  |  
| SMPrinterStatus_NoPaper  | NSInteger  | 3  |  
| SMPrinterStatus_RollIsExhausted  | NSInteger  | 4  |  
| SMPrinterStatus_PaperJam  | NSInteger  | 5  |  
| SMPrinterStatus_NoPaperPickup  | NSInteger  | 6  |  
| SMPrinterStatus_CoverOpened  | NSInteger  | 7  |  
| SMPrinterStatus_HeadOverheating  | NSInteger  | 8  |  
| SMPrinterStatus_MotorOverheating  | NSInteger  | 9  |  
## **协议列表**  
| 方法  | 说明  |  
| --- | --- |  
| - (void)discoverIPPrinter:(SunmiIpPrinterModel *_Nullable)printerModel;  | 当搜索到设备时，会执行代理方法，可以在此协议方法中获取扫描到的设备，会执行多次，直到搜索结束  |  
| - (void)finshedSearchPrinter;  | 搜索结束之后，执行代理方法，在此代理方法中做其他逻辑处理  |  
| - (void)didConnectedIPPrinter;  | 成功连接打印机  |  
| - (void)didConnectedIPPrinterWithError:(NSError *)error;  | 连接打印机失败并返回错误信息  |  
# 打印机指令类 SunmiPrinterCommand
## **本地缓存数据接口**
  * **获取将要发送给打印机的本地缓存数据**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (NSData *)getCommandData;
  

```

调用指令类接口时，数据首先缓存在本地，并未真正发送给打印机，通过调用此方法，可以得到本地缓存的打印指令数据，提供此方法给开发人员核对打印指令用。
  * **清除本地缓存数据**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)clearBuffer;
  

```

调用指令类接口时，数据首先缓存在本地，并未真正发送给打印机，通过调用此方法，清除本地即将传给打印机的缓存数据。
## **基础功能接口**
  * **获取sdk信息**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
(NSString *)getSDKVersion;
  

```

返回SDK版权信息；
  * **恢复打印机默认设置**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)restoreDefaultSettings;
  

```

  * **设置打印浓度**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setPrintDensity:(int)density;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| density  | int  | 设置打印浓度，取值范围 70~130，255 代表恢复到默认值，默认值 100  |  
  * **设置打印速度**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setPrintSpeed:(int)speed;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| speed  | int  | 设置打印速度，取值范围 0~250，255 代表恢复到默认值  |  
  * **获取最近一个打印任务的编号**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)getLastPrintTaskNumber;
  

```

打印内部会对每一个票据任务创建一个编号，以此来区分不同的打印任务。通过读取编号，可以识别不同任务的打印状态。
  * **获取打印机状态**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)getDeviceState;
  

```

  * **获取打印机sn**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)getDeviceSN;
  

```

  * **增加元数据**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)appendRawData:(NSData *)data;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | NSData  | 添加打印数据内容  |  
此接口提供给自定义数据用，可以向本地缓冲区追加任何数据内容。
## **字符控制接口**
  * **黑白翻转**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : int {
    SMPrintModel_Original = 0,
    SMPrintModel_BW_Reverse
} SMPrintMode;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMPrintModel_Original  | int  | 不翻转，默认  |  
| SMPrintModel_BW_Reverse  | int  | 翻转  |  
  * **下划线模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : int {
    SMUnderlineStyle_Empty = 0,
    SMUnderlineStyle_One,
    SMUnderlineStyle_Two 
} SMUnderlineStyle;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMUnderlineStyle_Empty  | int  | 取消下划线，默认  |  
| SMUnderlineStyle_One  | int  | 下划线一点粗  |  
| SMUnderlineStyle_Two  | int  | 下划线两点粗  |  
  * **设置下划线模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setUnderlineMode:(SMUnderlineStyle)mode;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| mode  | SMUnderlineStyle  | 详见 SMUnderlineStyle  |  
  * **设置打印宽度**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setPrintWidth:(int)printWidth;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| dotsPerLine  | int  | 设置打印机打印的宽度，取值范围：384-576  |  
  * **设置打印模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setPrintModesBold:(BOOL)bold
                 double_h:(BOOL)double_h
                 double_w:(BOOL)double_w;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| bold  | BOOL  | 是否设置打印机为加粗模式，YES 加粗  |  
| double_h  | BOOL  | 是否设置打印机为倍高模式，YES 倍高  |  
| double_w  | BOOL  | 是否设置打印机为倍宽模式，YES 倍宽  |  
  * **设置字符大小**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setCharacterWidth:(int)characterHeight
           characterWidth:(int)characterWidth;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| characterHeight  | int  | 设置文本字符串高度缩放，取值范围 1-8  |  
| characterWidth  | int  | 设置文本字符串宽度缩放，取值范围 1-8  |  
  * **设置黑白翻转模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setBlackWhiteReverseMode:(BOOL)enabled;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| enabled  | BOOL  | 设置黑白反转模式，YES 开启黑白反转  |  
  * **设置上下颠倒模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setUpsideDownMode:(BOOL)enabled
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| enabled  | BOOL  | 设置上下颠倒模式，YES 上下颠倒模式  |  
  * **追加单个 unicode 码以及数量**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)appendUnicode:(int)unicode count:(int)count;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| unicode  | int  | 传入现有的 unicode 值  |  
| count  | int  | 重复次数  |  
  * **追加文本字符串**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)appendText:(NSString *)contentText;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| contentText  | NSString  | 在当前数据后面追加文本字符串  |  
> 如果打印内容要实现一行多块，可以添加空格符或者制表符
  * **打印数据并走纸 n 行**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)lineFeed:(int)lines;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| lines  | int  | 打印机走纸  |  
## **行间距控制接口**
  * **恢复默认行间距**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)restoreDefaultLineSpacing;
  

```

  * **设置行间距**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setLineSpacing:(int)lineSpace;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| lineSpace  | int  | 设置打印机行间距，取值范围0-255，默认 30  |  
## **打印定位接口**
  * 中间加制表符的代码范例：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterCommand *command = [[SunmiPrinterCommand alloc] init];
[command setUtf8Mode:1];
[command appendText:@"测试文本1"];
[command horizontalTab:1];
[command appendText:@"测试文本2"];
[command lineFeed:1];
[[SunmiPrinterManager shareInstance] sendPrintData:[command getCommandData]];
  

```

  * 分列打印的代码范例：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterCommand *command = [[SunmiPrinterCommand alloc] init];
[command setUtf8Mode:1];
SunmiColumnSetting *set1 = [[SunmiColumnSetting alloc] init];
set1.width = 192;
set1.alignment = SMAlignStyle_Left;
set1.mode = SMPrintModel_Original;
SunmiColumnSetting *set2 = [[SunmiColumnSetting alloc] init];
set2.width = 192;
set2.alignment = SMAlignStyle_Center;
set2.mode = SMPrintModel_Original;
SunmiColumnSetting *set3 = [[SunmiColumnSetting alloc] init];
set3.width = 192;
set3.alignment = SMAlignStyle_Right;
set3.mode = SMPrintModel_Original;
[command setupColumns:@[set1, set2, set3]];
[command printInColumns:@[@"测试文本1", @"测试文本2", @"测试文本3"]];
[command lineFeed:1];
[[SunmiPrinterManager shareInstance] sendPrintData:[command getCommandData]];
  

```

  * **跳转 n 个制表符**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)horizontalTab:(int)tabs;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| tabs  | int  | 跳到下 tabs 个制表符位置  |  
  * **设置横向绝对位置**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setAbsolutePrintPosition:(int)horizontalPosition;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| horizontalPosition  | int  | 跳到横向绝对位置  |  
  * **设置横向相对位置**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setRelativePrintPosition:(int)horizontalPosition;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| horizontalPosition  | int  | 跳到横向相对位置  |  
  * **对齐方式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : NSInteger {
    SMAlignStyle_Left = 0,
    SMAlignStyle_Center,
    SMAlignStyle_Right
} SMAlignStyle;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMAlignStyle_Left  | NSInteger  | 将一行数据按照指定的位置对齐，居左对齐，默认  |  
| SMAlignStyle_Center  | NSInteger  | 将一行数据按照指定的位置对齐，居中对齐  |  
| SMAlignStyle_Right  | NSInteger  | 将一行数据按照指定的位置对齐，居右对齐  |  
  * **设置对齐方式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setAlignment:(SMAlignStyle)alignment;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| alignment  | SMAlignStyle  | 详见 SMAlignStyle  |  
  * **设置分列打印配置**


SunmiColumnSetting 对象
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
@interface SunmiColumnSetting : NSObject

@property (nonatomic, assign) int width;
@property (nonatomic, assign) SMPrintMode mode;
@property (nonatomic, assign) SMAlignStyle alignment;

@end
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| width  | int  | 列打印的宽度  |  
| mode  | SMPrintMode  | 是否黑白翻转  |  
| alignment  | SMAlignStyle  | 对齐方式  |  
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setupColumns:(NSArray <SunmiColumnSetting*>*)settings;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| settings  | NSArray  | 设置分列打印的配置，数组的元素为 SunmiColumnSetting，详见 SunmiColumnSetting  |  
  * **设置分列打印文本**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)printInColumns:(NSArray *)texts;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| texts  | NSArray  | 设置分列打印文本，同 setupColumns 方法，数组的长度与其保持一致  |  
> 使用按列打印文本内容会计算文本大小，所以会将字体的缩放默认变为原始大小，同时文本内容数组、字符长度数组和对齐方式数组的长度必须一致
## **图像打印接口**
  * **图片转换算法**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : NSInteger {
    SMImageAlgorithm_BINARIZATION = 0,
    SMImageAlgorithm_DITHERING
} SMImageAlgorithm;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMImageAlgorithm_BINARIZATION  | NSInteger  | 二值化算法通过调整浮动值将转换不同彩色值为黑色，可根据图片颜色信息调整一般浮动值，浮动值默认200  |  
| SMImageAlgorithm_DITHERING  | NSInteger  | 抖动灰度算法不用考虑浮动值变化  |  
  * **添加图片**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)appendImage:(UIImage *)img
               mode:(SMImageAlgorithm)mode;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| img  | UIImage  | 图片对象  |  
| mode  | SMImageAlgorithm  | 设置图片转换算法，详见 SMImageAlgorithm  |  
## **机械控制接口**
  * **设置切纸模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)cutPaper:(BOOL)fullCut
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| fullCut  | BOOL  | 设置切纸模式，YES 全切，NO 为 半切  |  
  * **设置延迟切纸**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)postponedCutPaper:(BOOL)fullCut delay:(int)delay
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| fullCut  | BOOL  | 设置切纸模式，YES 全切，NO 为 半切  |  
| delay  | int  | 发送该指令后，打印机不会马上切纸，而是等到后续走纸或打印了 (d + delay) 点的距离后再切纸，其中 d 为切刀与打印头之间的距离，delay 为 0~255  |  
  * **设置切刀模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setCutterMode:(int)mode;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| mode  | int  | 设置切刀模式，0为正常模式，1为半切模式，2为全切模式，3为不切模式，默认为0  |  
  * **清除未取纸状态**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)clearPaperNotTakenAlarm
  

```

  * **开钱箱**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)openCashBox;
  

```

## **字库控制接口**
  * **设置 CJK 编码**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setCjkEncoding:(int)cjkEncoding;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| cjkEncoding  | int  | 设置 CJK(Chinese/Japanese/Korean:中日韩文字)双字节字符。0 代表 GB18030，1 代表 BIG5，11 代表 Shift_JIS，12 代表 JIS 0208，21 代表 KS C 5601，128 代表 Disable CJK mode，255 代表默认设置  |  
  * **设置 CJK 字体**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)selectCjkCharFont:(int)charFont;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| charFont  | int  | 设置中日韩字符使用的字库，0 代表默认使用点阵字库，1 代表使用内置矢量字库，大于等于 128 代表使用第(charFont-127)种第三方矢量字库  |  
  * **设置 CJK 字体大小**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setHarfBuzzCjkCharSize:(int)charSize;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| charSize  | int  | 设置矢量字库中日韩字符大小，取值 4~255，默认 24  |  
  * **设置 utf8 模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setUtf8Mode:(int)utf8Mode
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| utf8Mode  | int  | 设置 UTF-8 字符集。0 代表禁用，1 代表可用，255 代表默认设置  |  
  * **设置 Latin 字体**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)selectAsciiCharFont:(int)charFont;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| charFont  | int  | 设置Latin字符使用的字库，0 代表默认使用点阵字库，1 代表使用内置矢量字库，大于等于 128 代表使用第(charFont-127)种第三方矢量字库  |  
  * **设置 Latin 字体大小**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setHarfBuzzAsciiCharSize:(int)charSize;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| charSize  | int  | 设置矢量字库 Latin 字符大小，取值 4~255，默认 12  |  
  * **设置其他字体**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)selectOtherCharFont:(int)charFont;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| charFont  | int  | 设置其他字符使用的字库，0 代表使用内置矢量字库，大于等于 128 代表使用第(charFont-127)种第三方矢量字库  |  
  * **设置其他字体大小**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setHarfBuzzOtherCharSize:(int)charSize;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| charSize  | int  | 设置矢量字库其他字符大小，取值 4~255，默认 24  |  
## **条码打印接口**
  * **条码 HRI 字符的打印位置**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : NSInteger {
    SMBarcodeReadable_Hide = 0,
    SMBarcodeReadable_Above,
    SMBarcodeReadable_Below,
    SMBarcodeReadable_Both
} SMBarcodeReadable;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMBarcodeReadable_Hide  | NSInteger  | 打印条码时，为 HRI 字符选择打印位置，不打印，默认  |  
| SMBarcodeReadable_Above  | NSInteger  | 打印条码时，为 HRI 字符选择打印位置，在条码上方  |  
| SMBarcodeReadable_Below  | NSInteger  | 打印条码时，为 HRI 字符选择打印位置，在条码下方  |  
| SMBarcodeReadable_Both  | NSInteger  | 打印条码时，为 HRI 字符选择打印位置，在条码上方及下方  |  
  * **选定条形码系统并打印**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : NSInteger {
    SMBarcodeType_UPCA = 65,
    SMBarcodeType_UPCE,
    SMBarcodeType_EAN13,
    SMBarcodeType_EAN8,
    SMBarcodeType_CODE39,
    SMBarcodeType_ITF,
    SMBarcodeType_CODABAR,
    SMBarcodeType_CODE93,
    SMBarcodeType_CODE128
} SMBarcodeType;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMBarcodeType_UPCA  | NSInteger  | UPC-A条码商品条码是纯数字,位数是11位在编码过后外加一位校验码，组成12位数字  |  
| SMBarcodeType_UPCE  | NSInteger  | UPC-E条码商品条码是纯数字，是由UPC-A缩减而成，位数是7位，而且首位必须为0在编码过后外加一位校验码，组成8位数字  |  
| SMBarcodeType_EAN13  | NSInteger  | EAN13商品条码是纯数字，而且位数是12位在编码过后外加一位校验码，组成13位数字  |  
| SMBarcodeType_EAN8  | NSInteger  | EAN8商品条码是纯数字，而且位数是7位在编码过后外加一位校验码，组成8位数字  |  
| SMBarcodeType_CODE39  | NSInteger  | Code39条码生成字符集包括数字 、大写字母以及- . $ / + % * 空格等字符其中"*"只用于标记开始和结束  |  
| SMBarcodeType_ITF  | NSInteger  | 交叉25码（Interleaved 2 of 5）条码生成，常用于物流管理字符集仅为数字且个数为偶数,为奇数将自动在前面加"0"  |  
| SMBarcodeType_CODABAR  | NSInteger  | 库德巴码（Codabar）条码生成，字符集包括数字和- $ : /. + 以及ABCD等字符其中ABCD只用于开始或者结尾，作为标识符使用  |  
| SMBarcodeType_CODE93  | NSInteger  | Code93条码生成是 full ASCII 模式，可使用ASCII全部128个字符  |  
| SMBarcodeType_CODE128  | NSInteger  | 组合code128a、code128b、code128c，需根据码内容动态切换  |  
  * **添加条形码**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)appendBarcode:(SMBarcodeReadable)hri_pos
               height:(int)height
          module_size:(int)module_size
         barcode_type:(SMBarcodeType)barcode_type
                 text:(NSString *)text;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| hri_pos  | SMBarcodeReadable  | HRI 字符打印位置，详见 SMBarcodeReadable  |  
| height  | int  | 设置条码高度，取值范围 1~255，默认 162  |  
| module_size  | int  | 设置条码的模组宽度，取值范围 2~6，默认 3  |  
| barcode_type  | SMBarcodeType  | 设置条码类型，详见 SMBarcodeType  |  
| text  | NSString  | 条码文本  |  
## **二维码打印接口**
  * **二维码纠错等级**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : NSInteger {
    SMErrorLevel_L = 0,
    SMErrorLevel_M,
    SMErrorLevel_Q,
    SMErrorLevel_H
} SMErrorLevel;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMErrorLevel_L  | NSInteger  | 选择纠错等级 L，可恢复字码比例 7%，默认  |  
| SMErrorLevel_M  | NSInteger  | 选择纠错等级 M，可恢复字码比例 15%  |  
| SMErrorLevel_Q  | NSInteger  | 选择纠错等级 Q，可恢复字码比例 25%  |  
| SMErrorLevel_H  | NSInteger  | 选择纠错等级 R，可恢复字码比例 30%  |  
  * **添加二维码**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)appendQRcode:(int)module_size
            ec_level:(SMErrorLevel)ec_level
                text:(NSString *)text;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| module_size  | int  | 设置二维码的模块大小，取值范围 1~16，默认 3  |  
| ec_level  | SMErrorLevel  | 设置纠错等级，详见 SMErrorLevel  |  
| text  | NSString  | 二维码文本  |  
## **页模式接口**
  * **进入页模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)enterPageMode;
  

```

进入页模式，打印机内置缓存区开启，开始缓存打印指令内容，等待打印。
  * **退出页模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)exitPageMode;
  

```

退出页模式，打印内置缓存区关闭，此时打印机内缓存数据清空，不会打印。
  * **清空页模式缓存区数据**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)clearInPageMode;
  

```

调用此方法，清空打印机页模式下的缓存数据，不会打印。
  * **打印页模式数据**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)printInPageMode;
  

```

页模式下缓存的数据将打印出来。
  * **打印并退出页模式**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)printAndExitPageMode;
  

```

页模式下缓存的数据将打印出来，此时打印机内缓存数据清空，并退出页模式。
  * **页模式打印旋转方向**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum : NSInteger {
    SMPageModeDirection_Original = 0,
    SMPageModeDirection_1,
    SMPageModeDirection_2,
    SMPageModeDirection_3
} SMPageModeDirection;
  

```
  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMPageModeDirection_Original  | NSInteger  | 不旋转，默认  |  
| SMPageModeDirection_1  | NSInteger  | 顺时针旋转90度  |  
| SMPageModeDirection_2  | NSInteger  | 顺时针旋转180度  |  
| SMPageModeDirection_3  | NSInteger  | 顺时针旋转270度  |  
  * **设置页模式区域**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setPrintAreaInPageModeOriginalX:(int)originalX
                              originalY:(int)originalY
                                  width:(int)width
                                 height:(int)height;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| originalX  | int  | 打印区域原点距离左间距  |  
| originalY  | int  | 打印区域原点距离下间距  |  
| width  | int  | 打印区域的宽度  |  
| height  | int  | 打印区域的高度  |  
  * **设置页模块打印方向**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setPrintDirectionInPageMode:(SMPageModeDirection)printDirection;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| printDirection  | SMPageModeDirection  | 详见 SMPageModeDirection  |  
  * **设置页模式下纵向绝对位置**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setAbsolutePrintPositionInPageMode:(int)verticalPosition;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| verticalPosition  | int  | 设置页模式下的绝对垂直打印位置，取值为 0~65535  |  
  * **设置页模式下纵向相对位置**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
- (void)setRelativePrintPositionInPageMode:(int)verticalPosition;
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| verticalPosition  | int  | 设置页模式下的相对垂直打印位置，取值为 0~65535  |  
# WIFI配网模块
Wifi配网时，务必需要长按配网键进入配网模式，否则设置无法成功。
## **开始扫描**
  * **扫描设备**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)scanPeripheral;  | 扫描蓝牙打印机设备  |  
当扫描到设备时，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)discoveredDevice:(SunmiBlePrinterModel *_Nonnull)device;  | 当扫描到设备时，可以在此协议方法中获取扫描到的设备，可能会执行多次，直到扫描结束，SunmiBlePrinterModel 详见 蓝牙打印机属性类  |  
  * **取消扫描**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)cancelScan;  | 取消扫描蓝牙打印机设备  |  
取消扫描之后，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)didCancelSearching;  | 取消扫描之后，可以在此协议方法中做其他逻辑处理  |  
## **连接打印机设备**
  * **连接设备**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)connectPeripheral:(CBPeripheral *_Nonnull)peripheral;  | 与指定蓝牙打印机设备建立连接，CBPeripheral 为系统类  |  
连接成功后，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)didConectPrinter;  | 成功连接到设备，可以在此协议方法中做连接后的逻辑  |  
  * **断开设备连接**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)disConnectPeripheral;  | 主动断开当前连接的蓝牙设备  |  
与设备断开连接后，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)willDisconnectPrinter;  | 已断开和蓝牙打印机的连接，可以在此协议方法中做断接后的逻辑  |  
## **获取打印机的SN**
  * **获取SN**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)getPrinterSN;  | 向蓝牙打印机发送获取设备SN的指令，此方法获取的SN，将会在蓝牙打印机的绑定操作中使用  |  
发送指令后，等待获取SN的过程，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)willStartReceiveDeviceSn;  | 已经开始发送获取SN的指令，可以在此协议方法中建立超时机制  |  
设备端发送SN，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)receiveDeviceSn:(NSString *_Nullable)sn;  | 接收设备发送的SN  |  
## **自动进入配网模式**  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)enterNetworkMode:(NSString *)deviceSN;  | 通过SN，自动进入配网模式。自动和手动进入配网同时存在，两种模式选择一种即可  |  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| deviceSN  | NSString  | 打印机的 SN 编号  |  
## **获取打印机搜索到的WIFI列表**
  * **获取Wi-Fi列表**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)getWifiList;  | 向蓝牙打印机发送获取Wi-Fi列表的指令  |  
接收到Wi-Fi信息后，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)receiveAPInfo:(NSDictionary *_Nullable)apInfo;  | 接收获取到的Wi-Fi网络信息。将接收到的Wi-Fi网络以列表形式展示出来，会执行多次  |  
接收到Wi-Fi信息发送完毕，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)didReceiveAllApInfo;  | Wi-Fi网络信息接收完毕，会执行此协议方法  |  
如果接收Wi-Fi信息失败，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)didFailReceiveApInfo;  | 接收失败，在此协议方法内可做其他的自定义处理  |  
## **选择某一个Wi-Fi网络为打印机配置网络**
  * **配置网络**


发送Wi-Fi名以及密码给蓝牙打印机，请求配网  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)connectAP:(NSString *_Nonnull)ssid password:(NSString * _Nullable)password;  | 发送Wi-Fi的网络的ssid和password给蓝牙打印机，为蓝牙打印机配置网络  |  
等待配网结果，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)willStartConfigPrinter;  | 可配置网络的命令已经发给蓝牙打印机，以在此协议方法内加入超时机制  |  
蓝牙打印机配网成功，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)configPrinterSuccess;  | 配网成功，可以在此协议方法内做后续操作  |  
配网成功，向蓝牙打印机发送通知  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)connectAPSuccess;  | 向蓝牙打印机发送配网成功的指令  |  
如果蓝牙打印机配网失败，会执行协议方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| - (void)configPrinterFail;  | 配网失败，可以在此协议方法内做后续其他操作  |  
  * **删除网络配置**

  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)deleteWifiSetting;  | 向蓝牙打印机发送删除 wifi 配置的指令  |  
## **退出配网模式**  
| 实例方法  | 说明  |  
| --- | --- |  
| - (void)quitConnectAP;  | 向蓝牙打印机发送退出配网模式的指令  |  
上一篇：12、云打印机微信小程序SDK
下一篇：14、云打印机iOS SDK多台同时连接
