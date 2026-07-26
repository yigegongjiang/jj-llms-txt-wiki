---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzmxeghjk491
---

# Cordova 打印热敏小票接口
更新时间：2026-07-01 15:30:58
## 1. 功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
<script>
    var lineApi = SunmiPrinterSDK.LineApi;
</script>
  

```

普通热敏小票以行输出方式打印的接口集合，接口将以行为单位渲染打印内容，每个方法都将单独绘制一行的打印内容（文本、图片、条码等单独一行）后立刻打印输出，所以多用于打印POS小票、针式发票的场景。
> initLine(style: Object)  
> printText(content: string, style: Object)  
> addText(content: string, style: Object)  
> printTexts(texts: string[], weights: number[], styles: Object[])  
> printBarCode(data: string, style: Object)  
> printQrCode(data: string, style: Object)  
> printBitmap(base64: string, style: Object)  
> printDividingLine(style: string, offset: number)  
> autoOut()  
> enableTransMode(enable: boolean)  
> printTrans(callback: (resultCode: number, message: string) => void)
## 2. 接口说明
### 2.1 行设置
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
initLine(style: Object)
  

```

初始化打印机的行基础样式，这将生效于之后的每一行，可在必要时重新调用此方法来改变之后的每行样式。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| style  | Object  | 画布基础样式，可用 `BaseStyleBuilder`构建  |  
**BaseStyleBuilder说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setWidth  | 设置行内可打印区域宽度  | 可设置最大范围为纸张宽度减去左边距 当可设置范围超出时仅设置最大值  | 打印机纸张宽度  |  
| setHeight  | 设置每行高度  | 行高范围在0~255像素 当行高小于打印内容时以行内最大高度内容为准  | 默认行高0像素  |  
| setAlign  | 设置之后每行的对齐方式  | 见Align  | 左对齐  |  
| setRenderColor  | 设置颜色  | 见RenderColor  | 默认黑色  |  
| setPosX  | 设置行左边距  | 左边距大小不能超过纸张宽度 改变左边距将可能影响可打印区域宽度  | 默认0像素  |  
> **!** setRenderColor仅在支持双色打印的商米设备上生效，同时双色打印需要打印纸支持
> **!** setPosY在行打印中不可用
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 设置默认初始化
lineApi.initLine({});
// 设置初始化对齐居中、宽度 384
lineApi.initLine(
    SunmiPrinterSDK.BaseStyleBuilder.create()
        .align(SunmiPrinterSDK.Align.CENTER)
        .width(384)
        .build()
);
  

```

### 2.2 打印文本内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printText(content: string, style: Object)
addText(content: string, style: Object)
  

```

向打印机发送文本内容，addText() 不会直接打印，通过多次调用 addText() 并设置不同的样式使一行内容呈现不同效果，而 printText() 将直接打印出发送的文本内容。
> **!** 当最后调用addText()接口需要添加”\”字符才可以保证打印输出  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| content  | string  | 要打印的文本内容  |  
| style  | Object  | 文本样式，可用 `TextStyleBuilder` 构建  |  
**TextStyleBuilder说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setTextSize  | 设置文本字符大小  | 6~96像素  | 24  |  
| setTextWidthRatio  | 横向放大倍数  | 0~7  | 0  |  
| setTextHeightRatio  | 纵向放大倍数  | 0~7  | 0  |  
| setTextSpace  | 文本字间距  | 0~100像素  | 0  |  
| enableBold  | 文本加粗  | 开启文本加粗功能  | 不开启  |  
| enableUnderline  | 文本下划线  | 开启文本下划线功能  | 不开启  |  
| enableStrikethrough  | 文本删除线  | 开启文本删除线功能  | 不开启  |  
| enableItalics  | 文本斜体  | 开启文本斜体功能  | 不开启  |  
| enableInvert  | 文本倒置  | 开启文本倒置功能  | 不开启  |  
| enableAntiColor  | 文本反白  | 开启文本反白功能  | 不开启  |  
| setPosX  | 相对本行已打印内容的距离  | 像素  | 0  |  
> **!** 设置自定义字体需要指定字体的名称，且字体需位于调用app的assets目录中供打印服务使用 外接非商米打印机字由打印机提供无法设置自定义字体
> **!** 由于文本内容大小和位置均可以自由控制，所以setPosY、setWidth、setHeight、setAlign在行打印文本时不可用
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
lineApi.printText("这行内容将直接打印出", {});
lineApi.addText("加粗", SunmiPrinterSDK.TextStyleBuilder.create().bold(true).build());
lineApi.addText("下划线", SunmiPrinterSDK.TextStyleBuilder.create().underline(true).build());
lineApi.addText("删除线", SunmiPrinterSDK.TextStyleBuilder.create().strikethrough(true).build());
lineApi.addText("倾斜", SunmiPrinterSDK.TextStyleBuilder.create().italic(true).build());
  

```

### 2.3 按列排列打印内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printTexts(texts: string[], weights: number[], styles: Object[])
  

```

向打印机按列排列发送打印内容，打印的内容为固定大小固定样式内容  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| texts  | string[]  | 各列文本内容数组  |  
| weights  | number[]  | 每一列占一行的比例大小  |  
| styles  | Object[]  | 每列打印内容的样式设置  |  
> **!** 目前打印服务6.5.0以上版本已支持列内的样式设置（打印服务版本可通过设备中硬件管家查看） >样式属性将改变全局的打印样式 >样式属性设置仅适用于商米打印机，外接非商米打印机则不适用除对齐方式以外的样式
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var style = SunmiPrinterSDK.TextStyleBuilder.create()
    .align(SunmiPrinterSDK.Align.CENTER)
    .build();
lineApi.printTexts(
    ["第一列", "第二列", "第三列"],
    [1, 1, 1],
    [style, style, style]
);
  

```

### 2.4 打印条形码
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printBarCode(data: string, style: Object)
  

```

使打印机打印一枚条形码  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 条码数据内容  |  
| style  | Object  | 条码样式对象，可用 `BarcodeStyleBuilder` 构建  |  
> **!** 不同类型的条形码对码内容有严格要求，如果码内容不符合对应码制规则将不会打印，在使用前请确认码内容无误
**BarcodeStyleBuilder说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setDotWidth  | 设置码块宽度  | 1~16像素 将影响最终条码的总宽度  | 2  |  
| setBarHeight  | 设置条码高度  | 1~255像素 将影响最终条码的总高度  | 162  |  
| setReadable  | 设置HRI位置  | 见HumanReadable  | 不展示  |  
| setSymbology  | 设置条码类型  | 见Symbology  | code128  |  
| setAlign  | 设置条码行内对齐方式  | 见Align  | 居左  |  
| setWidth  | 生成条码指定缩放宽度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
| setHeight  | 生成条码指定缩放高度  | 当设置缩放高度后将强制改变码内容大小  | 不缩放  |  
> **!** setWidth()和setHeight()自定义条码大小需要打印服务6.0.0以上版本支持（打印服务版本可通过设备中硬件管家查看） >缩放后的内容会有部分失真，可能会造成识别影响，请谨慎使用！ >外接非商米打印机则不适用自定义条码大小
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var style = SunmiPrinterSDK.BarcodeStyleBuilder.create()
    .align(SunmiPrinterSDK.Align.CENTER)
    .dotWidth(2)
    .barHeight(100)
    .readable(SunmiPrinterSDK.HumanReadable.POS_TWO)
    .build();
lineApi.printBarCode("0123456789", style);
  

```

### 2.5 打印二维码
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printQrCode(data: string, style: Object)
  

```

使打印机打印一枚条形码  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 二维码内容（如 URL、文本）  |  
| style  | Object  | 二维码样式对象，可用 `QrStyleBuilder` 构建  |  
**QrStyleBuilder说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setDot  | 设置二维码块大小  | 1~16像素 最终将影响二维码大小  | 4  |  
| setErrorLevel  | 设置二维码纠错等级  | 见ErrorLevel  | ErrorLevel.L  |  
| setAlign  | 设置二维码行内对齐方式  | 见Align  | 居左  |  
| setWidth  | 生成二维码指定缩放宽度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
| setHeight  | 生成二维码指定缩放高度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
> **!** setWidth()和setHeight()自定义条码大小需要打印服务6.0.0以上版本支持（打印服务版本可通过设备中硬件管家查看） >缩放后的内容会有部分失真，可能会造成识别影响，请谨慎使用！ >外接非商米打印机则不适用自定义条码大小
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var qrStyle = SunmiPrinterSDK.QrStyleBuilder.create()
    .align(SunmiPrinterSDK.Align.CENTER)
    .dot(9)
    .errorLevel(SunmiPrinterSDK.ErrorLevel.L)
    .build();
lineApi.printQrCode("http://www.sunmi.com", qrStyle);
  

```

### 2.6 打印图片
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printBitmap(base64: string, style: Object)
  

```

使打印机打印一枚图片  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| base64  | string  | 图片的 Base64 编码字符串  |  
| style  | Object  | 图片样式，可用 `BitmapStyleBuilder` 构建  |  
**BitmapStyleBuilder说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setAlgorithm  | 设置图片转换方式  | 见ImageAlgorithm  | ImageAlgorithm.BINARIZATION  |  
| setValue  | 设置额外参数  | 根据图片转换算法可能需要设置相应的参数  | 见ImageAlgorithm  |  
| setAlign  | 设置图片行内对齐方式  | 见Align  | 居左  |  
| setWidth  | 图片缩放宽度  | 当设置缩放宽度后将强制改变图片大小  | 不缩放  |  
| setHeight  | 图片缩放高度  | 当设置缩放宽度后将强制改变图片大小  | 不缩放  |  
> **!** setWidth()和setHeight()自定义条码大小需要打印服务6.0.0以上版本支持（打印服务版本可通过设备中硬件管家查看） >缩放后的内容会有部分失真，可能会造成识别影响，请谨慎使用！
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
getImageBase64('img/sunmi.png', function(base64) {
    var bitmapStyle = SunmiPrinterSDK.BitmapStyleBuilder.create()
        .align(SunmiPrinterSDK.Align.CENTER)
        .algorithm(SunmiPrinterSDK.ImageAlgorithm.BINARIZATION)
        .value(120)
        .width(384)
        .height(150)
        .build();
    lineApi.printBitmap(base64, bitmapStyle);
});
  

```

### 2.7 打印分割线
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printDividingLine(style: string, offset: number)
  

```

使打印机打印不同样式的分割线  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| style  | string  | 分割线类型`DividingLine`  |  
| offset  | number  | 分割线高度（单位：像素）  |  
**DividingLine 说明**  
| 参数  | 类型  |  
| --- | --- |  
| EMPTY  | 空白线（即空行  |  
| SOLID  | 实线  |  
| DOTTED  | 点线  |  
> **!** 由于老版本服务兼容性问题，分割线方法目前不能很好支持打印实线和点线，若需要请联系技术支持更新到最新版本服务
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
lineApi.printDividingLine(SunmiPrinterSDK.DividingLine.DOTTED, 5);
lineApi.printDividingLine(SunmiPrinterSDK.DividingLine.EMPTY, 20);
lineApi.printDividingLine(SunmiPrinterSDK.DividingLine.SOLID, 10);
  

```

### 2.8 打印结束输出
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
autoOut()
  

```

使打印机在打印内容结束后自动走纸到纸仓口，当打印机有切刀时会自动切纸 **示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var ditheringStyle = SunmiPrinterSDK.BitmapStyleBuilder.create()
    .align(SunmiPrinterSDK.Align.CENTER)
    .algorithm(SunmiPrinterSDK.ImageAlgorithm.DITHERING)
    .width(384)
    .height(150)
    .build();
lineApi.printBitmap(base64, ditheringStyle);
lineApi.autoOut();
  

```

### 2.9 事务模式-开关
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
enableTransMode(enable: boolean)
  

```
  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| enable  | boolean  | true 开启事务模式；false 关闭  |  
| 使打印机开启或关闭事务模式  |   
 |   
 |  
| 在开启事务模式后所有行打印命令将缓存执行直到执行事务模式提交  |   
 |   
 |  
> **!** 事务模式是商米打印机特殊支持的能力，对于外接非商米打印机没有效果
### 2.10 事务模式-提交
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printTrans(callback: (resultCode: number, message: string) => void)
  

```

使打印机提交事务 打印机开启事务模式时使用，调用后将顺序执行缓存的行打印命令，这个时候可以通过PrintResult获取这些命令的执行结果  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| callback  | Function  | 回调函数：function(resultCode, message)，resultCode 为 0 表示成功，非 0 表示失败；message 为结果描述  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 在进入打印模块后可开启事务
lineApi.enableTransMode(true);
lineApi.addText("测试内容1", {});
lineApi.addText("测试内容2", {});
lineApi.addText("测试内容3", {});
// 最后使用提交事务执行打印，通过回调确认打印执行情况
lineApi.printTrans(function(resultCode, message) {
    if (resultCode === 0) {
        console.log("打印成功:", message);
    } else {
        console.error("打印失败:", message);
    }
});
lineApi.enableTransMode(false);
  

```

> **!** 如果业务有事务模式和非事务模式同时使用的场景，请注意enableTransMode方法的调用情况，若当进入事务模式后忘记退出或app异常退出时，使用非事务打印同时没有调用printTrans方法，会造成不打印的情况，所以此种场景下建议非事务打印前也调用enableTransMode(false)，关闭事务模式防止以上遗漏情况
## 3. 枚举参数说明
**Align 行内对齐方式**  
| 枚举内容  | 说明  |  
| --- | --- |  
| DEFAULT  | 默认居左  |  
| LEFT  | 居左对齐  |  
| CENTER  | 居中对齐  |  
| RIGHT  | 居右对齐  |  
**HumanReadable 条码内容可读位置**  
| 枚举内容  | 说明  |  
| --- | --- |  
| HIDE  | 隐藏  |  
| POS_ONE  | 条码上方  |  
| POS_TWO  | 条码下方  |  
| POS_THREE  | 条码上下  |  
**Symbology 条码类型**  
| 枚举内容  | 说明  |  
| --- | --- |  
| UPCA  | UPC-A条码商品条码是纯数字,位数是11位 在编码过后外加一位校验码，组成12位数字,主要在美国和加拿大使用  |  
| UPCE  | UPC-E条码商品条码是纯数字，是由UPC-A缩减而成，位数是7位，而且首位必须为0 在编码过后外加一位校验码，组成8位数字  |  
| EAN13  | EAN13商品条码是纯数字，而且位数是12位 在编码过后外加一位校验码，组成13位数字  |  
| EAN8  | EAN8商品条码是纯数字，而且位数是7位 在编码过后外加一位校验码，组成8位数字  |  
| CODE39  | Code39条码生成字符集包括数字 、大写字母以及- . $ / + % * 空格等字符 其中"*"只用于标记开始和结束  |  
| ITF  | 交叉25码（Interleaved 2 of 5）条码生成，常用于物流管理 字符集仅为数字且个数为偶数,为奇数将自动在前面加"0"  |  
| CODABAR  | 库德巴码（Codabar）条码生成，字符集包括数字和- $ : /. + 以及ABCD等字符 其中ABCD只用于开始或者结尾，作为标识符使用  |  
| CODE93  | Code93条码生成是 full ASCII 模式，可使用ASCII全部128个字符  |  
| CODE128  | 组合code128a、code128b、code128c，需根据码内容动态切换  |  
**ErrorLevel 二维码纠错等级**  
| 枚举内容  | 说明  |  
| --- | --- |  
| L  | 纠错等级L 7%  |  
| M  | 纠错等级M 15%  |  
| Q  | 纠错等级Q 25%  |  
| H  | 纠错等级H 30%  |  
**ImageAlgorithm 图片转换算法**  
| 枚举内容  | 说明  | 参考浮动值  |  
| --- | --- | --- |  
| BINARIZATION  | 二值化算法  | 二值化算法通过调整浮动值将转换不同彩色值为黑色 可根据图片颜色信息调整参数设置浮动值（默认浮动值200）  |  
| DITHERING  | 抖动灰度算法  | 抖动灰度算法不用考虑浮动值变化  |  
**RenderColor 颜色**  
| 枚举内容  | 说明  |  
| --- | --- |  
| BLACK  | 打印默认颜色，所有打印内容将呈现黑色效果  |  
| RED  | 当打印机支持双色打印时可设置生效，这将在特殊的打印纸上呈现红色（纸张支持的颜色）  |  
上一篇：Cordova SDK 概览
下一篇：Cordova 打印标签小票接口
