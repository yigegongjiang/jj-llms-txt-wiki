---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzmceghjk502
---

# Cordova 打印标签小票接口
更新时间：2026-07-01 15:30:43
## 1. 功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
<script>
    var canvasApi = SunmiPrinterSDK.CanvasApi;
</script>
  

```

标签小票绘制整张标签后打印的接口集合，通过接口绘制完整的打印内容后打印，但对绘制内容设计要求更高，其更容易实现图文混排的打印效果，同时打印阶段会自动定位标签位置，所以多用于打印标签、黑标的场景 <span style="color: #999;"> initCanvas(style: Object) renderArea(style: Object) renderText(content: string, style: Object) renderBarCode(data: string, style: Object) renderBitmap(base64: string, style: Object) renderQrCode(data: string, style: Object) printCanvas(count: number, callback: Function) </span>
## 2. 使用限制
此接口需要使用者将商米内置打印机类型切换为标签或黑标模式才可以正常使用，对于开发者可使用如下代码跳转到类型设置界面：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterSDK.PrinterApi.startSettings(SunmiPrinterSDK.SettingItem.TYPE);
  

```

跳转设置能力需要打印服务6.6.32以上版本； 商米内置打印机也可以在热敏票据模式下使用此接口实现热敏票据打印图文混排内容；
## 3. 接口说明
### 3.1 画布设置
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
initCanvas(style: Object)
  

```

指定打印标签的画布大小、基础格式 使用画布渲染打印内容必须首先初始化要绘制的画布大小，才能确保最终可打印;  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| style  | Object  | 画布基础样式，可用 `BaseStyleBuilder` 构建  |  
**BaseStyleBuilder 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setWidth  | 初始化预渲染的画布宽度  | 可设置大小不超过打印纸宽度  | 默认0像素  |  
| setHeight  | 初始化预渲染的画布高度  | 可设置大小不超过打印纸高度  | 默认0像素  |  
| setPosX  | **东纳打印机** 绘制区域的相对位置  | 单位像素  | 默认0像素  |  
| setPosY  | **东纳打印机** 绘制区域的相对位置  | 单位像素  | 默认0像素  |  
| setRenderColor  | **东纳打印机** 支持打印内容颜色设置  | 支持黑色和红色墨盒  | 黑色墨盒  |  
| setAlign  | 不可用  | 不可用  | 无  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var canvasStyle = SunmiPrinterSDK.BaseStyleBuilder.create()
    .width(240)
    .height(160)
    .build();
canvasApi.initCanvas(canvasStyle);
  

```

### 3.2 绘制文本内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
renderText(content: string, style: Object)
  

```

向画布内指定区域绘制文本内容  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| content  | string  | 要绘制的文本内容  |  
| style  | Object  | 文本样式，可用 `TextStyleBuilder` 构建  |  
**TextStyleBuilder 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setPosX  | 设置文本内容在画布起始横坐标位置  | 单位像素  | 0  |  
| setPosY  | 设置文本内容在画布起始纵坐标位置  | 单位像素  | 0  |  
| setFont  | 指定打印机的自定义字体  | *.ttf为矢量字体否则为非矢量字体  | 打印机默认设置  |  
| setTextSize  | 字体为矢量时指定文本字符大小 字体为非矢量字体文本字符大小固定  | 有效范围6~96像素  | 24  |  
| setTextWidthRatio  | 指定倍宽大小  | 有效范围0-9  | 0  |  
| setTextHeightRatio  | 指定倍高大小  | 有效范围0-9  | 0  |  
| setWidth  | 设置文本限制宽度  | 若设置宽度限制，超过宽度将自动换行  | 不限制  |  
| setHeight  | 设置文本限制高度  | 若设置高度限制，超过部分将不显示  | 不限制  |  
| setAlign  | 设置文本内容相对起始坐标的位置  | 见Align  | Align.DEFAULT  |  
| setRotate  | 设置文本内容的方向  | 见Rotate  | 水平方向  |  
| setTextSpace  | **内置打印机** 设置文本字间距  | 0~100像素  | 0  |  
| enableBold  | **内置打印机** 设置文本加粗  | 开启文本加粗功能  | 不开启  |  
| enableUnderline  | **内置打印机** 设置文本下划线  | 开启文本下划线功能  | 不开启  |  
| enableStrikethrough  | **内置打印机** 设置文本删除线  | 开启文本删除线功能  | 不开启  |  
| enableItalics  | **内置打印机** 设置文本斜体  | 开启文本斜体功能  | 不开启  |  
| enableInvert  | 不支持设置文本倒置  | 不可用  | 无  |  
| enableAntiColor  | 不支持设置文本反白  | 不可用  | 无  |  
### 3.3 绘制条形码内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
renderBarCode(data: string, style: Object)
  

```

向画布内指定区域绘制条形码内容  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 条码数据内容  |  
| style  | Object  | 条码样式，可用 `BarcodeStyleBuilder` 构建  |  
**BarcodeStyleBuilder 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setPosX  | 设置条码内容在画布起始横坐标位置  | 单位像素  | 0  |  
| setPosY  | 设置条码内容在画布起始纵坐标位置  | 单位像素  | 0  |  
| setDotWidth  | 设置码块宽度  | 1~16像素 将影响最终条码的总宽度  | 2  |  
| setBarHeight  | 设置条码高度  | 1~255像素 将影响最终条码的总高度  | 162  |  
| setReadable  | 设置HRI位置  | 见HumanReadable  | 不展示  |  
| setSymbology  | 设置条码类型  | 见Symbology  | code128  |  
| setAlign  | 设置条码相对起始坐标位置  | 见Align  | Align.DEFAULT  |  
| setRotate  | 设置条码旋转方向  | 见Rotate  | 水平方向  |  
| setWidth  | **内置打印机** 指定条码缩放宽度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
| setHeight  | **内置打印机** 指定条码缩放高度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var barcodeStyle = SunmiPrinterSDK.BarcodeStyleBuilder.create()
    .posX(200)
    .posY(60)
    .dotWidth(2)
    .barHeight(60)
    .width(160)
    .readable(SunmiPrinterSDK.HumanReadable.POS_TWO)
    .build();
canvasApi.renderBarCode("12345678", barcodeStyle);
  

```

### 3.4 绘制二维码内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
renderQrCode(data: string, style: Object)
  

```

向画布内指定区域绘制二维码内容  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 二维码内容（如 URL、文本）  |  
| style  | Object  | 二维码样式，可用 `QrStyleBuilder` 构建  |  
**QrStyleBuilder 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setPosX  | 设置二维码内容在画布起始横坐标位置  | 单位像素  | 0  |  
| setPosY  | 设置二维码内容在画布起始纵坐标位置  | 单位像素  | 0  |  
| setDot  | 设置二维码块大小  | 1~16像素 最终将影响二维码大小  | 4  |  
| setErrorLevel  | 设置二维码纠错等级  | 见ErrorLevel  | ErrorLevel.L  |  
| setRotate  | 设置二维码旋转方向  | 见Rotate  | 水平方向  |  
| setWidth  | **内置打印机** 指定二维码缩放宽度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
| setHeight  | **内置打印机** 指定二维码缩放高度  | 当设置缩放宽度后将强制改变码内容大小  | 不缩放  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var qrStyle = SunmiPrinterSDK.QrStyleBuilder.create()
    .dot(3)
    .posX(20)
    .posY(20)
    .width(120)
    .height(120)
    .build();
canvasApi.renderQrCode("www.sunmi.com", qrStyle);
  

```

### 3.5 绘制图像
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
renderBitmap(base64: string, style: Object)
  

```

向画布内指定区域绘制图像  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| base64  | string  | 图片的 Base64 编码字符串  |  
| style  | Object  | 图片样式，可用 `BitmapStyleBuilder` 构建  |  
**BitmapStyleBuilder 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setPosX  | 设置图片在画布起始横坐标位置  | 超出画布大小将不打印  | 0  |  
| setPosY  | 设置图片在画布起始纵坐标位置  | 超出画布大小将不打印  | 0  |  
| setAlgorithm  | 设置图片转换方式  | 见ImageAlgorithm  | ImageAlgorithm.BINARIZATION  |  
| setValue  | 设置算法浮动值  | 根据具体的算法浮动值不同  | 见ImageAlgorithm  |  
| setWidth  | 内置打印机指定图片缩放宽度  | 当设置缩放宽度后将强制改变图片大小  | 不缩放  |  
| setHeight  | 内置打印机指定图片缩放高度  | 当设置缩放宽度后将强制改变图片大小  | 不缩放  |  
| setAlign  | 图片不支持对齐方式设置  | 不可用  | 无  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var bitmapStyle = SunmiPrinterSDK.BitmapStyleBuilder.create()
    .algorithm(SunmiPrinterSDK.ImageAlgorithm.DITHERING)
    .posX(20)
    .posY(150)
    .width(100)
    .height(60)
    .build();
canvasApi.renderBitmap(base64String, bitmapStyle);
  

```

### 3.6 绘制特殊图形
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
renderArea(style: Object)
  

```

向画布指定区域内绘制图形  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| style  | Object  | 区域样式，可用 `AreaStyleBuilder` 构建  |  
**AreaStyleBuilder 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setShape  | 设置绘制形状  | 见Shape  | Shape.RECT_FILL  |  
| setWidth  | 设置图形的宽度 形状为线段时无效 形状为圆形时表示圆形直径  | 单位像素  | 50  |  
| setHeight  | 设置图形的高度 形状为线段时无效 形状为圆形时无效  | 单位像素  | 50  |  
| setPosX  | 设置起始x坐标  | 单位像素  | 0  |  
| setPosY  | 设置起始y坐标  | 单位像素  | 0  |  
| setEndX  | 设置线段的终点x坐标  | 单位像素  | 50  |  
| setEndY  | 设置线段的终点y坐标  | 单位像素  | 50  |  
| setThick  | 设置描边  | 描边宽度  | 1  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var boxStyle = SunmiPrinterSDK.AreaStyleBuilder.create()
    .shape(SunmiPrinterSDK.Shape.BOX)
    .posX(0)
    .posY(0)
    .width(240)
    .height(159)
    .build();
canvasApi.renderArea(boxStyle);
  

```

### 3.7 打印绘制的内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
printCanvas(count: number, callback: Function)
  

```

如果画布内容有效将指定打印机打印画布内容  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| count  | number  | 欲打印的次数，如果是标签打印机，将分别打印到每张标签上, 打印数量大于0张  |  
| callback  | Function  | 回调函数：function(success, message),`success` 为是否成功，`message` 为结果描述  |  
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 不关心结果
CanvasApi.printCanvas(getCount());
// 获取打印结果
CanvasApi.printCanvas(getCount(), function(success, message) {
    if (success) {
        console.log("打印成功");
    } else {
        console.error("打印失败:", message);
    }
});
  

```

## 4. 枚举参数说明
**Align 对齐方式**  
| 枚举内容  | 说明  |  
| --- | --- |  
| DEFAULT  | 默认居左  |  
| LEFT  | 居左对齐  |  
| CENTER  | 居中对齐  |  
| RIGHT  | 居右对齐  |  
**Rotate 绘制方向**  
| 枚举内容  | 说明  |  
| --- | --- |  
| ROTATE_0  | 水平0度  |  
| ROTATE_90  | 90度方向  |  
| ROTATE_180  | 180度方向  |  
| ROTATE_270  | 270度方向  |  
**HumanReadable 条码内容可读位置**  
| 枚举内容  | 说明  |  
| --- | --- |  
| HIDE  | 隐藏  |  
| POS_ONE  | 底部居左  |  
| POS_TWO  | 底部居中  |  
| POS_THREE  | 底部居右  |  
标签打印的条码可读位置与热敏打印的条码可读位置是有差异的！ **Symbology 条码类型**  
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
**Shape 特殊图形**  
| 枚举内容  | 说明  |  
| --- | --- |  
| RECT_FILL  | 填充矩形 矩形区域为黑块  |  
| RECT_WHITE  | 擦除矩形 矩形区域为白块  |  
| RECT_REVERSE  | 反白矩形 矩形区域将反白  |  
| BOX  | 空心矩形  |  
| CIRCLE  | 空心圆形  |  
| OVAL  | 空心椭圆  |  
| PATH  | 线段  |  
上一篇：Cordova 打印热敏小票接口
下一篇：Cordova 指令集打印接口
