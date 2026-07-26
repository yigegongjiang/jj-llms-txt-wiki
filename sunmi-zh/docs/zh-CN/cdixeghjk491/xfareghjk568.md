---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfareghjk568
---

# 扫码头引擎（红外线扫码）
更新时间：2026-07-20 14:52:55
扫码头开发请参考：[《 SunmiScanner 开发及使用文档 》](https://sunmi-ota.oss-cn-hangzhou.aliyuncs.com/DOC/resource/re_cn/%E6%89%AB%E7%A0%81%E5%A4%B4/%E6%89%AB%E7%A0%81%E5%A4%B4%E5%BC%80%E5%8F%91%E5%8F%8A%E7%94%A8%E6%88%B7%E6%96%87%E6%A1%A3.pdf)本文件是一份通用机型参考开发文件。
Sunmi L2、L2K、L2-H、L2KS、L2S、L2H、P2 mini、P2Lite、P2Pro、P2H、V2 Pro、M2_MAX、V2S、V2S Plus等扫描码专用设备，可用于商业超级、工业、医疗、农业贸易、执法等
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) ScanHeadDemo.zip 
提示：修改 AndroidManifest.xml：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
<!-- Android target sdk >= 30 add -->
<queries>
  <package android:name="com.sunmi.scanner" />

</queries>
```

## 1. 可识别的码
不同型号的扫码头支持的码类型不同，列举部分可支持的码类型，如下表所示：  
| 扫码头  | 编码类型  |  
| --- | --- |  
| 商米  | Code 128, UCC/EAN 128, ISBT 128, EAN 8, EAN 13, UPC-A, UPC-E, UPA-E1, Interleaved 2 of 5, Matrix 2 of 5, Code 39, Code 93, Codabar, GS1-Databar, Code 11, ISBN, MSI-Plessey, ISSN-EAN, PDF417, QR Code, Aztec, Data Matrix, HanXin Code  |  
| 新大陆二维扫码头  | code128, uccean128, ean8, ean13, upce, upca, itf, matrix, code39, codabar, code93, isbn, industrial25, standard25, Plessey, code11, msiplessey, pdf417, qr, datamatrix, hanxin  |  
| 斑马二维扫码头  | auspostal, Aztec, chinese25, codabar, code11, code128, code39, code93, itf, pdf417, compositeab, composite, uccean128, ean8, ean13, issnean, isbn, upca, upce, upce1, matrix, discrete25, msiplessey, gs1databar, qr, datamatrix, hanxin, maxi, isbt128, korea35, uspostnet, usplanet, ukpostal, japanpostal  |  
| 新大陆/鹰捷  | ISSN EAN, MIS-Plessey, Standard 2 of 5, Industrial 2 of 5, ISBN, Code 11, Code 93, Codabar, Code 39, Matrix 2 of 5, Interleaved 2 of 5(ITF), UPC-A, UPC-E, EAN13, EAN8, UCC-EAN128, Code128  |  
用户可根据需求选择可识别的码，具体操作步骤如下：
  1. 在“**扫码头设置** ”中点击“**选择可识别码** ”；
  2. 选择开启或关闭制定识别码，**默认开启全部识别码** 。


  

![](https://file.cdn.sunmi.com/SUNMIDOCS/select_images.png)
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/select_code.png)
## 2. 编码方式
用户可在“**输出编码设置** ”中对输出编码进行设置，可选择 **UTF-8，GBK，ISO-8859-1和SHITF-JIS** 四种编码格式，其中，**默认编码格式为 UTF-8** 。
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/5486881535226034.png)
## 3. 提示方式
用户可在“**提示方式设置** ”中对提示方式进行设置，**默认打开声音提示和震动提示** 。
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/5957431351551776.png)
## 4. 输出方式
用户可在“**输出方式设置** ”中对扫码结果的输出方式进行设置，**默认选中模拟键盘方式输出、自动补回车、开启广播输出** 。
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/18804965628460013.png)
## 5. 触发模式
用户可在“**触发模式设置** ”中选择触发模式，**默认“短按触发** ”，扫码成功后扫码头自动熄灭。
_注：新大陆扫码头和斑马扫码头在“_** _短按触发，连续扫码_** _”选项表现的现象不同：_
  1. _当机器为新大陆扫码头，并且选中了“短按触发，连续扫码”后，扫码头会自动进入连续扫码模式，扫码头会常亮一段时间后熄灭，然后又重新亮起开始扫描，如此循环。_
  2. _当机器为斑马扫码头，并且选中了“短按触发，连续扫码”后，机器会自动进入连续扫码模式，扫码头会常亮，除非再次按下扫码键，否则扫码头灯光不会熄灭，且相同的码只会输出一次。_


  

![](https://file.cdn.sunmi.com/SUNMIDOCS/28911467517499.png)
## 6. Q&A
常见问题：
**1. 问：如何对接扫码头？**
答：为了方便开发者，商米把扫码头做成了一个免开发的组件。一般用户无需针对扫码头做开发即可获取扫码内容。
**2.问：如何获取扫码结果？**
答：商米提供了3种数据输出方式 ： 1）模拟键盘：会把扫码结果模拟成按键事件，会自动输出到焦点框文本，用户可以通过获取焦点框内容或者监听按键事件来获取扫码结果； 2）直接填充：会把扫码结果拷贝到粘贴版，会自动输出到焦点框文本，相比较与模拟键盘输出方式，直接填充的输出速度更快； 3）广播输出：会把扫码结果作为广播发送，用户可以监听广播来获取扫码结果。
**3.问：连续扫码结果不一致？**
答：输入法会影响扫码输入结果，已知百度小米版输入法会影响扫码结果输出，可更换谷歌拼音输入法。
**4.问：扫码结果比较慢？**
答：如果选择了模拟按键输出模式，按键间隔时间会影响输出速度，设置为0为最快速度 （默认0） 。
**5.问：需要兼容摄像头扫码和扫码头扫码？**
答：L2斑马扫码头与摄像头不可同时开启，用户可以判断机型来区分商米设备和其他设备，在商米设备上使用侧键扫码。
**6.问：软件如何触发扫码？**
答：可参考[L2用户指南](https://sunmi-ota.oss-cn-hangzhou.aliyuncs.com/DOC/resource/re_cn/%E6%89%AB%E7%A0%81%E5%A4%B4/%E6%89%AB%E7%A0%81%E5%A4%B4%E5%BC%80%E5%8F%91%E5%8F%8A%E7%94%A8%E6%88%B7%E6%96%87%E6%A1%A3.pdf) 中的aidl接口，扫码服务提供了软触发的接口。
**7.问：扫描中文二维码出现乱码怎么解决？**
答：解析带中文的二维码需要把_输出编码匹配二维码中中文的编码格式_，比如二维码的中文“你好”的编号格式为UTF-8，扫码头输出编码也需要设置UTF-8才能正确输出“你好”，否则将会输出乱码。
**8.问： 是否支持OCR** ？
新增身份证号码OCR识别功能，暂时仅支持扫码头SS1100，该功能需另外开通，请参考文档[《身份证号码识别使用说明》](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/fddeghjk524) 。
**9. 问： OCR的调用方式和输出方式？**
OCR的调用方式和输出方式与扫码的条码输出一致，请参考：[《 SunmiScanner 开发及使用文档 》](https://sunmi-ota.oss-cn-hangzhou.aliyuncs.com/DOC/resource/re_cn/%E6%89%AB%E7%A0%81%E5%A4%B4/%E6%89%AB%E7%A0%81%E5%A4%B4%E5%BC%80%E5%8F%91%E5%8F%8A%E7%94%A8%E6%88%B7%E6%96%87%E6%A1%A3.pdf) 。
上一篇：摄像头扫码 Cordova SDK 说明
下一篇：扫码底座
