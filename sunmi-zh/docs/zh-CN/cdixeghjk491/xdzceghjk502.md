---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdzceghjk502
---

# SDK概览
更新时间：2026-06-24 11:38:35
# 1. 如何集成SDK
商米打印SDK可以通过远程仓库获取，只需要在模块的build.gradle中添加依赖即可。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
android {
    ...
}

dependencies {
    implementation 'com.sunmi:printerx:1.0.20'
}
  

```

[版本更新说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdzqeghjk513)
# 2. 演示示例
您可以参考SDK使用Demo源码进行开发。点击下方链接下载Demo。
[Github地址](https://github.com/shangmisunmi/SunmiPrinterXSample)
[Gitee地址](https://gitee.com/kaltin/SunmiPrinterXSample)
# 3. 如何使用 SDK
集成SDK后，首先可以使用getPrinter()方法获取打印机，这是一个异步过程。通常，您可以使用PrinterListen回调onDefPrinter()方法返回的默认打印机对象来控制设备内置的打印服务。
当多台打印机（无论是商米打印机还是其他打印机）根据需要连接到商米设备时，也会通过onPrinters()回调返回一组打印机对象供您选择。
打印机对象可以使用以下API来管理打印机，满足不同的打印需求，包括：
  * QueryApi: Printer query API
  * CommandApi: Command set printing API
  * LineApi: Receipt printing API
  * CanvasApi: Label printing API
  * FileApi: File printing API
  * CashDrawerApi: Cash drawer control API
  * LcdApi LCD: LCD customer display control API


## 1. 获取打印机
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getPrinter(Context context, PrinterListen printerListen)
  

```
  
| **参数**  | **描述**  |  
| --- | --- |  
| Context context  | 传入的会话上下文和应用程序的上下文将被使用。  |  
| PrinterListen printerListen  | 当你传入回调对象时，可以通过回调异步获取可用的打印机。  |  
PrinterListen可以持续保持，商米设备会实时动态更新发现或添加的新打印机。您可以根据业务需求，动态更新本地可用的打印机。
  * **PrinterListen**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void onDefPrinter(Printer printer)

void onPrinters(List<Printer> printers)
  

```
  
| 参数  | **描述**  |  
| --- | --- |  
| Printer printer  | 默认打印机对象  |  
| List printers  | 商米设备集成的打印机对象列表  |  
onDefrinter 返回的打印机对象一般是商米内置的打印机。当商米设备没有打印配置时，也可以将通过商米打印设备添加的自定义打印机设置为默认打印机。此时，打印机对象会更改为自定义打印机。
调用 onPrinters 方法会返回商米设备所有支持和已添加的打印机列表。
  * **Printer**


打印机实例对象，用于完成打印任务。
当多台打印机连接到商米设备时，可以通过调用printer.toString获取打印机的唯一ID。您可以保存此ID，并使用指定的ID配置打印机，以实现不同的业务场景。
打印机唯一ID仅对当前商米设备有效，同一台打印机在不同商米设备上的唯一ID不同，因此不能用于不同的设备。
## 2. 设置日志输出的API
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void log(boolean enable, String tag)
  

```

在打印机正常使用过程中，为了不中断整个打印流程，API 不会针对打印过程中出现的参数传递错误、方法使用错误等问题直接返回失败。因此，为了方便适配，您可以开启此日志开关，以便获取开发过程中出现的问题（建议发布时关闭）。默认标签为：[PrinterX]
## 3. 释放SDK
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void destroy()
  

```

当SDK不再使用时，可以调用该方法，释放应用程序占用的打印资源。
## 4. 跳转到打印机配置页面
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
boolean startSettings(Activity activity, SettingItem item) 
  

```

由于部分商米打印机的全局属性只能在系统设置中配置，因此我们提供了接口方式，可以跳转到相应的配置界面。
**SettingItem**  
| 枚举  | **描述**  |  
| --- | --- |  
| TYPE  | 跳转到切换打印机类型的配置项，允许用户将当前打印机切换为热敏/标签等。  |  
| DENSITY  | 跳转到打印机浓度配置项，供用户设置打印浓度值。  |  
| PAPER  | 跳转到打印机纸张规格配置项，允许用户切换当前打印机的纸张规格  |  
| FONT  | 跳转到打印机字体配置项，允许用户切换当前打印机字体  |  
| ALL  | 跳转到其他配置项  |  
跳转打印机配置用于商米打印机，要求打印服务版本在6.6.32以上，如果打印服务版本不支持跳转配置，则此方法返回失败。
## 5. **PrinterSdk 全局设置接口文档**
全局设置商米打印机的参数，会直接覆盖系统设置中用户通过手动配置的对应打印机参数，因此将影响用户的自定义配置。
此功能需要打印服务版本 6.11.23 或更高版本支持，使用前请先检查打印组件的版本。
### **1. setSunmiPrinterDensity**
  * **接口定义**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1public void setSunmiPrinterDensity(Context context, Density density)
```

  * **方法说明**


全局设置商米打印机浓度。
备注：S2LCC暂不支持设置
  

  * **参数说明**

  
| **参数名**  | **类型**  | **必填**  | **说明**  |  
| --- | --- | --- | --- |  
| context  | Context  | 是  | 上下文对象，用于调用 ContentResolver  |  
| density  | Density  | 是  | 浓度枚举  |  
### **2. setSunmiPrinterFontType**
  * **接口定义**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1public void setSunmiPrinterFontType(Context context, FontType fontType)
```

**方法说明**
全局设置商米打印机字体类型。
  

**参数说明**  
| **参数名**  | **类型**  | **必填**  | **说明**  |  
| --- | --- | --- | --- |  
| context  | Context  | 是  | 上下文对象，用于调用 ContentResolver  |  
| fontType  | FontType  | 是  | 字体枚举  |  
### **3. setSunmiPrinterSpeed**
**接口定义**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1public void setSunmiPrinterSpeed(Context context, Speed speed)
```

  * **方法说明**


全局设置商米打印机打印速度。
备注：目前仅支持金融设备。
  

  * **参数说明**

  
| **参数名**  | **类型**  | **必填**  | **说明**  |  
| --- | --- | --- | --- |  
| context  | Context  | 是  | 上下文对象，用于调用 ContentResolver  |  
| speed  | Speed  | 是  | 速度枚举  |  
### **4. setSunmiPrinterAlert**
  * **接口定义**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1public void setSunmiPrinterAlert(Context context, boolean isAlert)
```

  * **方法说明**


全局设置商米打印机异常报警弹窗开关。
  

  * **参数说明**

  
| **参数名**  | **类型**  | **必填**  | **说明**  |  
| --- | --- | --- | --- |  
| context  | Context  | 是  | 全局上下文，用于调用 ContentResolver。  |  
| isAlert  | boolean  | 是  | 是否开启打印机异常报警弹窗，`true` 为开启，`false` 为关闭。  |  
### **5. setSunmiPrinterBootAlert**
  * **接口定义**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1public void setSunmiPrinterBootAlert(Context context, boolean isAlert)
```

  * **方法说明**


全局设置商米打印机开机报警提示开关。
  

  * **参数说明**

  
| **参数名**  | **类型**  | **必填**  | **说明**  |  
| --- | --- | --- | --- |  
| context  | Context  | 是  | 全局上下文，用于调用 ContentResolver。  |  
| isAlert  | boolean  | 是  | 是否开启开机打印机报警提示，`true` 为开启，`false` 为关闭。  |  
### **6. 枚举说明**
**6.1 Density（打印浓度）**
说明：用于`setSunmiPrinterDensity` 方法设置打印机的浓度，浓度范围为70%-130%
  

**6.2 FontType（字体类型）**
说明：用于`setSunmiPrinterFontType` 方法设置打印机的字体  
| **枚举值**  | **ordinal**  | **含义说明**  |  
| --- | --- | --- |  
| Default  | 0  | 默认字体（根据不同项目使用下面1、2、3之一）  |  
| SunmiFont1  | 1  | 商米字体 1： 商米自定义字体库主要支持拉丁文  |  
| SunmiFont2  | 2  | 商米字体 2： 商米自定义字体库主要支持绝大部分unicode字符  |  
| SunmiFont3  | 3  | 商米字体 3： 商米自定义字体库支持更多unicode字符且表现更细致  |  
**6.3 Speed（打印速度）**
说明：用于`setSunmiPrinterSpeed` 方法设置速度  
| **枚举值**  | **ordinal**  | **含义说明**  |  
| --- | --- | --- |  
| LOW  | 0  | 低速  |  
| MEDIUM  | 1  | 中速  |  
| HIGH  | 2  | 高速  |  
# 4. SDK异常
商米打印SDK是一套针对各种类型打印机的API。不同类型的API对应不同类型的打印机调用。如果调用某些特定打印机不支持的API，可能会引发SDK异常。
例如，调用API构建激光打印机热敏收据的内容时，就会引发异常。
商米打印SDK基本适用于所有商米设备。部分机型由于系统版本过低，可能不支持API，请联系商米技术支持将设备升级到最新版本。
上一篇：SDK升级说明
下一篇：SDK版本说明
