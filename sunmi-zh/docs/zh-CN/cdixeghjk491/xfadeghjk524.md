---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfadeghjk524
---

# Flutter SDK概览
更新时间：2025-10-20 17:44:39
商米推出了一款 Flutter 插件，极大地简化并加速了开发者对商米打印机的适配流程。第三方软件开发者通过调用 Flutter SDK，开发者能够便捷地调用商米设备的内置打印机，轻松实现打印功能，有效提升开发效率，降低对接成本 。
### **1.如何集成SDK**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 $ flutter pub add sunmi_flutter_plugin_printer
```

这将添加一行这样的包在pubspec.yaml (并运行一个隐式的 'flutter pub get '):
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
dependencies:
  sunmi_flutter_plugin_printer: ^1.0.7+7
```

### **2.演示示例**
![](https://cdn.sunmi.com/public/image/mgt-document/bccee5dd0da34db197033b2f75275702.png)
可参考Demo的源码进行开发，点击下方链接跳转下载上图对应的Demo
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) FlutterPrinterSample.zip 
### 3.如何使用 SDK
集成SDK后，首先可以使用getPrinter()方法获取打印机，这是一个异步过程。您可以使用PrinterListen回调onDefPrinter()方法返回的默认打印机对象来控制设备内置的打印服务。
打印机对象可以使用以下API来管理打印机，满足不同的打印需求，包括：
  * QueryApi 打印机查询接口
  * CommandApi 指令集打印接口
  * LineApi 小票打印接口
  * CanvasApi 标签打印接口
  * CashDrawerApi 钱箱控制接口


##### **3.1 获取打印机**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<void> getPrinter(PrinterListener listener)
```
  
| **参数**  | **描述**  |  
| --- | --- |  
| PrinterListen printerListen  | 当你传入回调对象时，可以通过回调异步获取可用的打印机。  |  
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
abstract class PrinterListener {
  void onDefPrinter(Printer var1);
}
```
  
| 参数  | **描述**  |  
| --- | --- |  
| Printer printer  | 默认打印机对象  |  
##### **3.2 设置日志输出的API**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<void> log(bool enable, String? tag)
```

  

在打印机正常使用过程中，为了不中断整个打印流程，API 不会针对打印过程中出现的参数传递错误、方法使用错误等问题直接返回失败。因此，为了方便适配，您可以开启此日志开关，以便获取开发过程中出现的问题（建议发布时关闭）。默认标签为：[PrinterX]
##### **3.3 释放SDK**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<void> destroy()
```

当SDK不再使用时，可以调用该方法，释放应用程序占用的打印资源。
##### **3.4 跳转到打印机配置页面**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<bool?> startSettings(SettingItem item)
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
##### **3.5 SDK异常**
商米打印SDK是一套针对各种类型打印机的API。不同类型的API对应不同类型的打印机调用。如果调用某些特定打印机不支持的API，可能会引发SDK异常。
例如，调用API构建激光打印机热敏收据的内容时，就会引发异常。
商米打印SDK基本适用于所有商米设备。部分机型由于系统版本过低，可能不支持API，请联系商米技术支持将设备升级到最新版本。
上一篇：JavaScript 打印机查询接口
下一篇：Flutter 打印热敏小票接口
