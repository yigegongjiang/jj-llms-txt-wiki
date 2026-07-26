---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzmfeghjk535
---

# 摄像头扫码 Cordova SDK 说明
更新时间：2026-07-01 15:35:03
## 1. 概述
商米提供了带摄像头设备的扫码SDK，并具有以下五个优势：
  * **高识别率** ：相对ZXing等开源扫码方案有更高的识读成功率，污损扭曲条码解码效果更好；
  * **快速扫码** ：百万像素毫秒级解码；
  * **简单便捷** ：几行代码即可集成到客户应用程序；
  * **高适配度** ：与商米的设备完美适配，软硬件结合可以保证功能的高效稳定。
  * **支持多种常用码制** ：已支持EAN-8, EAN-13, UPC-A, UPC-E, Codabar, Code39, Code93, Code128, ISBN10, ISBN13, ISSN, DataBar, DataBar Expanded, Interleaved 2 of 5, QR Code, MicroQR, PDF417, MicroPDF417，DataMatrix，AZTEC, Hanxin.


## 2. 使用说明
### 2.1 如何集成插件
插件已发布到 [npm](https://www.npmjs.com/package/sunmi-scanner-plugin)，直接执行以下命令即可集成：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
cordova plugin add sunmi-scanner-plugin
```

### 2.2 演示示例
![](https://cdn.sunmi.com/public/image/mgt-document/d334acdef1fc47d2adafd865f59623e9.png)
可参考Demo的源码进行开发，点击下方链接跳转下载上图对应的Demo
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiScanner.zip 
### 3.3 如何使用 SDK
  * 开始扫码


调用 ScanSdk.startScan() 方法，通过回调获取扫码结果。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
window.ScanSdk.startScan(function (success, message, data) {
    if (!success) {
        alert(message || '扫码失败');
        return;
    }

    console.log('扫码成功：', data);
});
```

  * 参数说明

  
| 参数  | 描述  |  
| --- | --- |  
| callback  | 扫码结果回调。  |  
  * 回调参数说明

  
| 回调参数  | 描述  |  
| --- | --- |  
| success  | 是否扫码成功。  |  
| message  | 结果提示信息。  |  
| data  | 扫码结果数据。  |  
  * 示例


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
document.addEventListener('deviceready', function () {
    var btn = document.getElementById('scanBtn');
    btn.disabled = false;

    btn.addEventListener('click', function () {
        if (!window.ScanSdk || typeof window.ScanSdk.startScan !== 'function') {
            alert('未检测到扫码插件');
            return;
        }

        window.ScanSdk.startScan(function (success, message, data) {
            if (!success) {
                alert(message || '扫码失败');
                return;
            }

            console.log('扫码结果：', data);
        });
    });
}, false);
```

上一篇：摄像头扫码uni-app SDK说明
下一篇：扫码头引擎（红外线扫码）
