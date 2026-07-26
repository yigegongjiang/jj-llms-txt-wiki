---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfazeghjk557
---

# 摄像头扫码Flutter SDK说明
更新时间：2025-11-13 12:16:37
# 一. 概述
商米提供了带摄像头设备的扫码SDK，并具有以下五个优势：
  * **高识别率** ：相对ZXing等开源扫码方案有更高的识读成功率，污损扭曲条码解码效果更好；
  * **快速扫码** ：百万像素毫秒级解码；
  * **简单便捷** ：几行代码即可集成到客户应用程序；
  * **高适配度** ：与商米的设备完美适配，软硬件结合可以保证功能的高效稳定。
  * **支持多种常用码制** ：已支持EAN-8, EAN-13, UPC-A, UPC-E, Codabar, Code39, Code93, Code128, ISBN10, ISBN13, ISSN, DataBar, DataBar Expanded, Interleaved 2 of 5, QR Code, MicroQR, PDF417, MicroPDF417，DataMatrix，AZTEC, Hanxin.


# 二. 使用说明
### **1.如何集成SDK**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
flutter pub add sunmi_flutter_plugin_scan
```

这将添加一行这样的包在pubspec.yaml (并运行一个隐式的 'flutter pub get '):
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
dependencies:
  sunmi_flutter_plugin_scan: ^1.0.3+4
```

### **2.演示示例**
![](https://cdn.sunmi.com/public/image/mgt-document/9b7db18ec5964bce9d85ab4142f7c82b.png)
可参考Demo的源码进行开发，点击下方链接跳转下载上图对应的Demo
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) sunmi_flutter_plugin_scan_example.zip 
### 3.如何使用 SDK
##### **3.1 开始扫码**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<void> startScan(ScanResultListener listener)
```
  
| **参数**  | **描述**  |  
| --- | --- |  
| ScanResultListener listener  | 当你传入回调对象时，可以通过回调异步获取扫码结果。  |  
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef ScanResultListener = void Function(
    int code, List<ScanResultBean> resultList, String? msg);


const ScanResultBean({
    this.TYPE,
    this.VALUE,
  });
```
  
| 参数  | **描述**  |  
| --- | --- |  
| TYPE  | 扫码类型  |  
| VALUE  | 扫码结果  |  
###### **示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void _startScan() {
  setState(() {
    scanResultStr = "";
  });
  try {
    ScanSdk.instance.startScan((
      int code,
      List<ScanResultBean> resultList,
      String? msg,
    ) {
      GlobalUtils.logger.d("startScan Result: $code, $resultList, $msg");
      if (code == 0 && resultList.isNotEmpty) {
        setState(() {
          scanResultStr = json.encode(resultList[0]);
        });
      }
    });
  } on PlatformException catch (e) {
    GlobalUtils.logger.d("PlatformException: $e");
  } catch (e) {
    GlobalUtils.logger.d("Exception: $e");
  }
}
```

  

  

上一篇：摄像头扫码SDK说明
下一篇：摄像头扫码uni-app SDK说明
