---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzmaeghjk480
---

# Cordova SDK 概览
更新时间：2026-07-01 15:32:46
  

商米推出了 Cordova 打印机 SDK 插件，用于在 **Cordova** 环境下开发针对商米 V2sPlus、V3 等 Android 设备的打印功能。第三方开发者通过调用本 SDK，可便捷使用商米设备内置打印机，轻松实现打印功能，有效提升开发效率、降低对接成本。
## 1. 如何集成SDK
插件已发布到 [npm](https://www.npmjs.com/package/sunmi-printer-plugin)，直接执行以下命令即可集成：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cordova plugin add sunmi-printer-plugin
```

安装插件后，可通过全局对象 SunmiPrinterSDK 调用打印接口。
## 2. 演示示例
以下为基于本 SDK 开发的 Cordova 打印 Demo 主界面，展示了打印机详情、打印票据、打印标签、打印指令、钱箱控制、打印设置等能力入口：
![](https://cdn.sunmi.com/public/image/mgt-document/bd0ce6149c914de69b897c192df2e2bf.png)
可参考Demo的源码进行开发，点击下方链接跳转下载上图对应的Demo
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiPrinter.zip 
## 3. 如何使用 SDK
使用打印能力前，需先初始化并获取打印机实例。在页面加载时调用 PrinterApi.initPrinter()，在回调中根据结果判断是否获取到默认打印机。
打印机对象可以使用以下API来管理打印机，满足不同的打印需求，包括：
  * PrinterApi 打印机管理与查询接口
  * CommandApi 指令集打印接口
  * LineApi 小票打印接口
  * CanvasApi 标签打印接口
  * CashDrawerApi 钱箱控制接口


### 3.1 获取打印机
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterSDK.PrinterApi.initPrinter(
    function() { console.log("打印机就绪"); },
    function(err) { console.error("初始化失败:", err); }
);
```

获取打印机信息可使用：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterSDK.PrinterApi.getPrinterInfo(
    function(info) { console.log("打印机信息:", info); },
    function(err) { console.error("获取失败:", err); }
);
  

```

### 3.2 设置日志输出的 API
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 开启日志
SunmiPrinterSDK.PrinterApi.log(true, "YourTag");
// 关闭日志
SunmiPrinterSDK.PrinterApi.log(false);
```

在打印机正常使用过程中，为了不中断整个打印流程，API 不会针对打印过程中出现的参数传递错误、方法使用错误等问题直接返回失败。因此，为了方便适配，您可以开启此日志开关，以便获取开发过程中出现的问题（建议发布时关闭）。默认标签为：[PrinterX]
### 3.3 释放 SDK
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterSDK.PrinterApi.destroy();
```

当SDK不再使用时，可以调用该方法，释放应用程序占用的打印资源。
### 3.4 跳转到打印机配置页面
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterSDK.PrinterApi.startSettings(SunmiPrinterSDK.SettingItem.TYPE);
```

由于部分商米打印机的全局属性只能在系统设置中配置，因此我们提供了接口方式，可以跳转到相应的配置界面。 **SettingItem**  
| 枚举  | 描述  |  
| --- | --- |  
| TYPE  | 跳转到切换打印机类型的配置项，允许用户将当前打印机切换为热敏/标签等。  |  
| DENSITY  | 跳转到打印机浓度配置项，供用户设置打印浓度值。  |  
| PAPER  | 跳转到打印机纸张规格配置项，允许用户切换当前打印机的纸张规格。  |  
| FONT  | 跳转到打印机字体配置项，允许用户切换当前打印机字体。  |  
| ALL  | 跳转到其他配置项。  |  
> 跳转打印机配置用于商米打印机，要求打印服务版本在 **6.6.32** 以上。如果打印服务版本不支持跳转配置，则此方法返回失败。
### 3.5 SDK异常
商米打印SDK是一套针对各种类型打印机的API。不同类型的API对应不同类型的打印机调用。如果调用某些特定打印机不支持的API，可能会引发SDK异常。例如，调用API构建激光打印机热敏收据的内容时，就会引发异常。商米打印SDK基本适用于所有商米设备。部分机型由于系统版本过低，可能不支持API，请联系商米技术支持将设备升级到最新版本。
上一篇：Uniapp 钱箱控制接口
下一篇：Cordova 打印热敏小票接口
