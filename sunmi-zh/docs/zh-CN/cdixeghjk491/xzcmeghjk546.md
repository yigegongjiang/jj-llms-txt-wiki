---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzcmeghjk546
---

# 摄像头扫码uni-app SDK说明
更新时间：2026-05-20 16:50:30
# 1.概述
商米提供了带摄像头设备的扫码SDK，并具有以下五个优势：
  * **高识别率** ：相对ZXing等开源扫码方案有更高的识读成功率，污损扭曲条码解码效果更好；
  * **快速扫码** ：百万像素毫秒级解码；
  * **简单便捷** ：几行代码即可集成到客户应用程序；
  * **高适配度** ：与商米的设备完美适配，软硬件结合可以保证功能的高效稳定。
  * **支持多种常用码制** ：已支持EAN-8, EAN-13, UPC-A, UPC-E, Codabar, Code39, Code93, Code128, ISBN10, ISBN13, ISSN, DataBar, DataBar Expanded, Interleaved 2 of 5, QR Code, MicroQR, PDF417, MicroPDF417，DataMatrix，AZTEC, Hanxin.


# 2 使用说明
### 2.1 如何集成SDK
在项目中通过 uni_modules 引入 sunmi-scanner-sdk 插件（从DCloud插件市场安装[sunmi-scanner-sdk](https://ext.dcloud.net.cn/plugin?name=sunmi-scanner-sdk)或拷贝至 uni_modules 目录）。
  

在页面的 `script` 中按需引入打印相关 API：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { ScanSdk } from "@/uni_modules/sunmi-scanner-sdk";
```

## 2.2 演示示例
![](https://cdn.sunmi.com/public/image/mgt-document/573c2c94f921466eacb94e943f550951.png)
可参考Demo的源码进行开发，点击下方链接跳转下载上图对应的Demo
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiScannerUniPlugin.zip 
## 3.如何使用 SDK
### 3.1开始扫码
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
ScanSdk.startScan(callback)
  

```
  
| 参数  | 描述  |  
| --- | --- |  
| callback  | 当你传入回调函数时，可以通过回调异步获取扫码结果。  |  
| 回调参数  | 描述  |  
| --- | --- |  
| success  | 扫码是否成功  |  
| message  | 扫码结果  |  
| data  | 扫码数据  |  
### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
ScanSdk.startScan((success, message, data) => {
    if (!success) {
        uni.showToast({
            title: message || '扫码失败',
            icon: 'none'
        })
        return
    }
    this.scanResult = data
    uni.showToast({
        title: '扫码成功',
        icon: 'none'
    })
})
  

```

上一篇：摄像头扫码Flutter SDK说明
下一篇：摄像头扫码 Cordova SDK 说明
