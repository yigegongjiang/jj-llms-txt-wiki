---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzaceghjk502
---

# Uniapp 打印标签小票接口
更新时间：2026-03-23 12:09:05
# Uniapp 打印标签小票接口
本文档介绍在 **UniApp** / **UniApp-x** 环境下，通过 `sunmi-printersdk` 插件的 **CanvasApi** 调用商米设备内置打印机，实现标签、票据等按画布坐标绘制的接口说明。
* * *
## 1. 功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CanvasApi } from "@/uni_modules/sunmi-printersdk";
  

```

标签小票绘制整张标签后打印的接口集合，通过接口绘制完整的打印内容后打印，但对绘制内容设计要求更高，其更容易实现图文混排的打印效果，同时打印阶段会自动定位标签位置，所以多用于打印标签、黑标的场景
> initCanvasApi(baseStyle: BaseStyle)  
> renderArea(areaStyle : AreaStyle)  
> renderText(text : string, textStyle : TextStyle)  
> renderBarCode(text : string, barcodeStyle : BarcodeStyle)  
> renderBitmap(bitmapBase64 : string, bitmapStyle : BitmapStyle)  
> renderQrCode(qrcode : string, qrStyle : QrStyle)  
> printCanvas(callback : (success : boolean, message : string) => void, count : Int)
## 2. 使用限制
此接口需要使用者将商米内置打印机类型切换为标签或黑标模式才可以正常使用，对于开发者可使用如下代码跳转到类型设置界面：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
PrinterSdk.startSettings(SettingItem.TYPE);
  

```

跳转设置能力需要打印服务6.6.32以上版本；
商米内置打印机也可以在热敏票据模式下使用此接口实现热敏票据打印图文混排内容；
* * *
## 3. 接口说明
### 3.1 画布设置
**initCanvasApi(baseStyle: BaseStyle)**
初始化画布，设定打印区域的宽度、高度、对齐方式、渲染颜色及横向偏移。后续所有绘制都在此画布上按坐标排版。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| baseStyle  | BaseStyle  | 画布基础样式，可用 `BaseStyleBuilder` 构建，包含 `align`、`width`、`height`、`renderColor`、`posX`。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CanvasApi } from "@/uni_modules/sunmi-printersdk";
import { BaseStyleBuilder, Align } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const baseStyle = BaseStyleBuilder.create()
    .align(Align.CENTER)
    .width(384)
    .height(219)
    .build();
CanvasApi.initCanvasApi(baseStyle);
  

```

* * *
### 3.2 绘制文本内容
**renderText(text: string, textStyle: TextStyle)**
在画布指定位置绘制文本。`textStyle` 控制对齐、字体样式、字号、宽高比、坐标、宽高及旋转等。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | string  | 要绘制的文本内容。  |  
| textStyle  | TextStyle  | 文本样式，可用 `TextStyleBuilder` 构建，含 `posX`、`posY`、`width`、`height`、`rotate` 等。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { TextStyleBuilder, FontStyle, Rotate } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const textStyle = TextStyleBuilder.create()
    .align(Align.LEFT)
    .fontStyle(FontStyle.NORMAL)
    .textSize(24)
    .posX(20)
    .posY(30)
    .width(344)
    .height(40)
    .rotate(Rotate.ROTATE_0)
    .build();
CanvasApi.renderText("商品名称：示例标签", textStyle);
  

```

* * *
### 3.3 绘制条形码内容
**renderBarCode(text: string, barcodeStyle: BarcodeStyle)**
在画布指定位置绘制条形码。`barcodeStyle` 可指定对齐、条宽、条高、坐标、宽高、是否显示底部文字及条码类型（Symbology）等。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | string  | 条码数据内容。  |  
| barcodeStyle  | BarcodeStyle  | 条码样式，可用 `BarcodeStyleBuilder` 构建。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { BarcodeStyleBuilder, Symbology, HumanReadable } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const barcodeStyle = BarcodeStyleBuilder.create()
    .align(Align.CENTER)
    .symbology(Symbology.CODE128)
    .barHeight(60)
    .posX(72)
    .posY(80)
    .width(240)
    .readable(HumanReadable.POS_ONE)
    .build();
CanvasApi.renderBarCode("6901234567890", barcodeStyle);
  

```

* * *
### 3.4 绘制二维码内容
**renderQrCode(qrcode: string, qrStyle: QrStyle)**
在画布指定位置绘制二维码。`qrStyle` 可指定对齐、点大小(dot)、宽高、坐标及纠错等级(ErrorLevel)等。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| qrcode  | string  | 二维码内容（如 URL、文本）。  |  
| qrStyle  | QrStyle  | 二维码样式，可用 `QrStyleBuilder` 构建。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { QrStyleBuilder, ErrorLevel } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const qrStyle = QrStyleBuilder.create()
    .align(Align.CENTER)
    .dot(6)
    .width(120)
    .height(120)
    .posX(132)
    .posY(150)
    .errorLevel(ErrorLevel.M)
    .build();
CanvasApi.renderQrCode("https://example.com", qrStyle);
  

```

* * *
### 3.5 绘制图像
**renderBitmap(bitmapBase64: string, bitmapStyle: BitmapStyle)**
在画布指定位置绘制图片。图片需为 Base64 编码字符串；`bitmapStyle` 可指定坐标、宽高及（行式场景下常用的）对齐、二值化算法与阈值。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| bitmapBase64  | string  | 图片的 Base64 编码字符串。  |  
| bitmapStyle  | BitmapStyle  | 图片样式，可用 `BitmapStyleBuilder` 构建，含 `posX`、`posY`、`width`、`height`、`align`、`algorithm`、`value`。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { BitmapStyleBuilder, ImageAlgorithm } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const bitmapStyle = BitmapStyleBuilder.create()
    .posX(20)
    .posY(280)
    .width(200)
    .height(80)
    .algorithm(ImageAlgorithm.BINARIZATION)
    .value(120)
    .build();
CanvasApi.renderBitmap(base64String, bitmapStyle);
  

```

* * *
### 3.6 绘制特殊图形
**renderArea(areaStyle: AreaStyle)**
在画布上绘制矩形、圆、椭圆等区域图形，用于边框、分割块或装饰。`areaStyle` 指定位置、尺寸、线宽及形状类型（Shape）。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| areaStyle  | AreaStyle  | 区域样式，可用 `AreaStyleBuilder` 构建，含 `width`、`height`、`posX`、`posY`、`endX`、`endY`、`thick`、`shape`。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { AreaStyleBuilder, Shape } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

// 绘制矩形边框
CanvasApi.renderArea(AreaStyleBuilder.create()
    .width(344)
    .height(200)
    .posX(20)
    .posY(10)
    .thick(1)
    .shape(Shape.BOX)
    .build());

// 绘制实心矩形
CanvasApi.renderArea(AreaStyleBuilder.create()
    .width(100)
    .height(30)
    .posX(242)
    .posY(15)
    .shape(Shape.RECT_FILL)
    .build());
  

```

* * *
### 3.7 打印绘制的内容
**printCanvas(callback: (success: boolean, message: string) = > void, count: number)**
将当前画布上已绘制的全部内容发送打印机并走纸。可指定打印联数 `count`（如 1 表示打印 1 张，2 表示相同内容打印 2 张）。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| callback  | (success, message) => void  | 回调：`success` 为是否成功，`message` 为结果描述。  |  
| count  | number  | 打印联数，通常为 1 或更大正整数。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
CanvasApi.initCanvasApi(baseStyle);
CanvasApi.renderArea(AreaStyleBuilder.create().width(344).height(200).posX(20).posY(10).shape(Shape.BOX).thick(1).build());
CanvasApi.renderText("标签标题", TextStyleBuilder.create().posX(20).posY(20).textSize(28).build());
CanvasApi.renderBarCode("6901234567890", barcodeStyle);
CanvasApi.renderQrCode("https://example.com", qrStyle);

CanvasApi.printCanvas((success, message) => {
    if (success) {
        console.log("打印成功");
    } else {
        console.error("打印失败:", message);
    }
}, 1);  // 打印 1 联
  

```

* * *
## 4. 枚举参数说明
以下枚举与样式类型定义在 `@/uni_modules/sunmi-printersdk/utssdk/interface.uts`，按需引入。
### 4.1 画布基础样式 BaseStyle（画布设置）  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **BaseStyleBuilder**  | `align`、`width`、`height`、`renderColor`、`posX`。  |  
| **Align**  | 对齐：`DEFAULT(0)`、`LEFT(1)`、`CENTER(2)`、`RIGHT(3)`。  |  
| **RenderColor**  | 渲染颜色（双色机且纸张支持时生效）：`BLACK(0)`、`RED(1)`。  |  
### 4.2 文本样式 TextStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **TextStyleBuilder**  | `align`、`fontStyle`、`textSize`、`textWidthRatio`、`textHeightRatio`、`posX`、`posY`、`width`、`height`、`rotate`、`textSpace`。  |  
| **FontStyle**  | 字体样式：`NORMAL(0)`、`BOLD(1)`、`UNDERLINE(2)`、`STRIKETHROUGH(3)`、`ITALIC(4)`、`Invert(5)`、`AntiColor(6)`。  |  
| **Rotate**  | 旋转：`ROTATE_0(0)`、`ROTATE_90(90)`、`ROTATE_180(180)`、`ROTATE_270(270)`。  |  
### 4.3 条码样式 BarcodeStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **BarcodeStyleBuilder**  | `align`、`dotWidth`、`barWidth`、`barHeight`、`readable`、`posX`、`posY`、`width`、`height`、`symbology`。  |  
| **HumanReadable**  | 条码底部文字：`HIDE(0)`、`POS_ONE(1)`、`POS_TWO(2)`、`POS_THREE(3)`。  |  
| **Symbology**  | 条码类型：如 `UPCA(0)`、`UPCE(1)`、`EAN13(2)`、`EAN8(3)`、`CODE39(4)`、`ITF(5)`、`CODABAR(6)`、`CODE93(7)`、`CODE128(8)`、`EAN128(9)` 等。  |  
### 4.4 二维码样式 QrStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **QrStyleBuilder**  | `align`、`dot`、`width`、`height`、`posX`、`posY`、`errorLevel`。  |  
| **ErrorLevel**  | 纠错等级：`L(0)`、`M(1)`、`Q(2)`、`H(3)`。  |  
### 4.5 图片样式 BitmapStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **BitmapStyleBuilder**  | `align`、`width`、`height`、`posX`、`posY`、`algorithm`、`value`。  |  
| **ImageAlgorithm**  | 二值化算法：`BINARIZATION(0)`、`DITHERING(1)`。`value` 为阈值等参数。  |  
### 4.6 区域样式 AreaStyle 与形状 Shape  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **AreaStyleBuilder**  | `width`、`height`、`posX`、`posY`、`endX`、`endY`、`thick`、`shape`。  |  
| **Shape**  | 区域形状：`RECT_FILL(0)` 实心矩形、`RECT_WHITE(1)` 白底矩形、`RECT_REVERSE(2)` 反色矩形、`BOX(3)` 矩形框、`CIRCLE(4)` 圆、`OVAL(5)` 椭圆、`PATH(6)` 路径。  |  
* * *
上一篇：Uniapp 打印热敏小票接口
下一篇：Uniapp 指令集打印接口
