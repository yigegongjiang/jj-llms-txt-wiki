---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmxdeghjk524
---

# 4、钱箱驱动器iOS驱动说明
更新时间：2025-10-31 17:31:09
# 一、概述
本⽂档主要介绍如何通过商米提供的SDK快速使用钱箱驱动器
# 二、SDK DEMO 介绍
## 资源下载
钱箱驱动器 DEMO for iOS 代码下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiCashBoxDemo.zip 
## Demo使用介绍
1.DEMO使用介绍；
  

![](https://cdn.sunmi.com/public/image/mgt-document/b4ef668238ea4383ad7b41c2e3a1cd64.PNG)
2.点击【Search CashBox】进入蓝牙扫描界面；
  

![](https://cdn.sunmi.com/public/image/mgt-document/96ec4414677f4ddfb279722b3ff96201.PNG)
3.选择一台钱箱设备【CashDrawer_00xxxx】，xxxx为对应设备的SN号后4位。PIN码为设备名称”CashDrawer_00xxxx”后6位数字【00xxxx】；
配对时，需要在USB插上后通电5分钟内完成，如果超时配对请重新拔插USB接口
  

![](https://cdn.sunmi.com/public/image/mgt-document/96d6293b366a4b46ba8ca04a4d9bbb87.JPG)
4.点击对应功能进行测试；
  

![](https://cdn.sunmi.com/public/image/mgt-document/9d4d58ea2357416d8eb42b891a18814a.PNG)
# 三、SDK开发说明
## SDK 快速入门
1.SDK for iOS 下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiCashBox.xcframework.zip 
2.目前 SDK 只支持手动集成；
3.将 `SunmiCashBox.xcframework` 导入到 Xcode 工程目录下；
4.在需要使用 API 的类中添加 `#import <SunmiCashBox/SunmiCashBox.h>` 头文件；
5.使用蓝牙钱箱，需要在 `info.plist` 中添加两个蓝牙权限；
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Privacy - Bluetooth Always Usage Description
Privacy - Bluetooth Peripheral Usage Description
  

```

## SDK 详细说明
1.此 SDK 在真机或模拟器环境中均可使用，建议使用与 iOS11 或以上系统版本；
2.SDK 包含搜索蓝牙搜索、连接、关闭以及提供了方便开发者调用钱箱指令的接口方法；
3.便于开发者使用，提供了 DEMO 用于开发者调试；
4.开发环境 Mac OS、Xcode、iPhone设备（需要 iOS 11 以上）。
## 钱箱设备类 SMCashBoxModel  
| 属性名  | 类型  | 说明  |  
| --- | --- | --- |  
| peripheral  | CBPeripheral  | 蓝牙设备  |  
| rssi  | NSNumber  | 发现蓝牙设备时取到的信号强度值  |  
| deviceName  | NSString  | 蓝牙设备名称  |  
| uuidString  | NSString  | 蓝牙设备 UUID  |  
## 钱箱管理类 SMCashBoxBleManager
  * **属性列表**

  
| 属性名  | 类型  | 说明  |  
| --- | --- | --- |  
| bluetoothDelegate  | id  | 代理协议  |  
  * **方法列表**

  
| 方法  | 说明  |  
| --- | --- |  
| + (instancetype)sharedInstance;  | SDK 初始化  |  
| + (void)instanceDealloc;  | SDK 销毁  |  
| - (void)startScanPeripheral;  | 开始扫描蓝牙设备  |  
| - (void)stopScan;  | 停止扫描蓝牙设备  |  
| - (void)chooseDevice:(SMCashBoxModel *)model;  | 选择扫描到的设备  |  
| - (void)openCashBox;  | 发送打开钱箱指令  |  
| - (void)sendGetCashBoxStateWithReceived:(receivedStateBlock)stateBlock;  | 发送钱箱开启状态指令并返回状态枚举  |  
| - (void)sendGetCashBoxSNNumberWithReceived:(receivedSNBlock)snBlock;  | 发送获取设备 SN 指令并返回 SN 字串  |  
| - (void)sendGetCashBoxFirmwareVersionWithReceived:(receivedVersionBlock)versionBlock;  | 发送获取固件版本号指令并返回版本号字串  |  
| - (void)sendSuccess:(sendDataSuccessBlock)successBlock;  | 发送指令成功的回调  |  
| - (void)sendFail:(sendDataFailBlock)failBlock;  | 发送指令失败的回调  |  
> 数据回调的说明
> 1.receivedVersionBlock：固件版本号回调，返回版本号字符串
> 2.receivedStateBlock：钱箱开启状态回调，返回状态枚举值
> 3.receivedSNBlock：钱箱 sn，返回 SN 字符串  
| 枚举值  | 类型  | 说明  |  
| --- | --- | --- |  
| SMCashBoxOpenStatus_low  | NSInteger  | 低电平  |  
| SMCashBoxOpenStatus_high  | NSInteger  | 高电平  |  
  * **协议列表**

  
| 方法  | 说明  |  
| --- | --- |  
| - (void)discoveredDevice:(SMCashBoxModel *)device;  | 当搜索到设备时，会执行代理方法，可以在此协议方法中获取扫描到的设备，会执行多次，直到搜索结束  |  
| - (void)didStopSearching;  | 停止搜索设备  |  
| - (void)connectCashBoxFail:(NSError *)error;  | 连接设备失败返回失败信息  |  
上一篇：3、钱箱驱动器Android驱动说明
下一篇：5、钱箱驱动器macOS驱动说明
