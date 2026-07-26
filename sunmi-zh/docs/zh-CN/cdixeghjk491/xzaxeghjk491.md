---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzaxeghjk491
---

# Uniapp 打印热敏小票接口
更新时间：2026-03-23 12:07:03
# Uniapp 打印热敏小票接口
本文档介绍在 **UniApp** / **UniApp-x** 环境下，通过 `sunmi-printersdk` 插件的 **LineApi** 调用商米设备内置热敏打印机，实现小票、流水单等按行打印的接口说明。
* * *
## 1. 功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { LineApi } from "@/uni_modules/sunmi-printersdk";
  

```

普通热敏小票以行输出方式打印的接口集合，接口将以行为单位渲染打印内容，每个方法都将单独绘制一行的打印内容（文本、图片、条码等单独一行）后立刻打印输出，所以多用于打印POS小票、针式发票的场景
>initLine(baseStyle: BaseStyle)  
>printText(text : string, textStyle : TextStyle)  
>addText(text : string, textStyle : TextStyle)  
>printTexts(textArray : Array<string>, colsWidthArr : Array<number>, textStyles : Array<TextStyle>)  
>printBarCode(text : string, barcodeStyle : BarcodeStyle)  
>printQrCode(qrcode : string, qrStyle : QrStyle)  
>printBitmap(bitmapBase64 : string, bitmapStyle : BitmapStyle)  
>printDividingLine(lineType : Int, lineHeight : Int)  
>autoOut()  
>enableTransMode(enable : boolean)  
>printTrans(callback : (success : boolean, message : string) => void)
## 2. 接口说明
### 2.1 行设置
**initLine(baseStyle: BaseStyle)**
初始化行式画布，设定打印区域的宽度、对齐方式、高度、渲染颜色及横向偏移。后续所有行式打印都在此画布上排版。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| baseStyle  | BaseStyle  | 画布基础样式，可用 `BaseStyleBuilder` 构建，包含 `align`、`width`、`height`、`renderColor`、`posX`。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { LineApi } from "@/uni_modules/sunmi-printersdk";
import { BaseStyleBuilder, Align } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const baseStyle = BaseStyleBuilder.create()
    .align(Align.CENTER)
    .width(384)
    .height(0)
    .build();
LineApi.initLine(baseStyle);
  

```

* * *
### 2.2 打印文本内容
**printText(text: string, textStyle: TextStyle)**
打印一行文本并换行。`textStyle` 控制对齐、字体样式、字号、宽高比、字间距等。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | string  | 要打印的文本内容。  |  
| textStyle  | TextStyle  | 文本样式，可用 `TextStyleBuilder` 构建。  |  
**addText(text: string, textStyle: TextStyle)**
在当前行追加文本，不换行。可与 `printText` 搭配，实现同一行多段不同样式。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| text  | string  | 追加的文本内容。  |  
| textStyle  | TextStyle  | 文本样式。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { TextStyleBuilder, FontStyle } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const textStyle = TextStyleBuilder.create()
    .align(Align.LEFT)
    .fontStyle(FontStyle.NORMAL)
    .textSize(24)
    .build();
LineApi.printText("订单号：202502100001", textStyle);
LineApi.addText("  |  金额：￥99.00", textStyle);
  

```

* * *
### 2.3 按列排列打印内容
**printTexts(textArray: string[], colsWidthArr: number[], textStyles: TextStyle[])**
多列对齐打印。按列宽数组将多段文本排列在同一行，每列可指定样式（对齐、字体等）。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| textArray  | string[]  | 各列文本内容数组。  |  
| colsWidthArr  | number[]  | 各列宽度数组，与 textArray 长度一致。  |  
| textStyles  | TextStyle[]  | 各列文本样式数组，与 textArray 长度一致。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
LineApi.printTexts(
    ["商品名称", "数量", "金额"],
    [200, 80, 104],
    [textStyleLeft, textStyleCenter, textStyleRight]
);
  

```

* * *
### 2.4 打印条形码
**printBarCode(text: string, barcodeStyle: BarcodeStyle)**
打印条形码。`barcodeStyle` 可指定对齐、条宽、条高、是否显示底部文字、条码类型（Symbology）等。  
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
    .barHeight(80)
    .readable(HumanReadable.POS_ONE)
    .build();
LineApi.printBarCode("6901234567890", barcodeStyle);
  

```

* * *
### 2.5 打印二维码
**printQrCode(qrcode: string, qrStyle: QrStyle)**
打印二维码。`qrStyle` 可指定对齐、点大小(dot)、宽高、纠错等级(ErrorLevel)等。  
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
    .width(200)
    .height(200)
    .errorLevel(ErrorLevel.M)
    .build();
LineApi.printQrCode("https://example.com", qrStyle);
  

```

* * *
### 2.6 打印图片
**printBitmap(bitmapBase64: string, bitmapStyle: BitmapStyle)**
打印图片。图片需为 Base64 编码字符串；`bitmapStyle` 可指定对齐、宽高、坐标、二值化算法及阈值。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| bitmapBase64  | string  | 图片的 Base64 编码字符串。  |  
| bitmapStyle  | BitmapStyle  | 图片样式，可用 `BitmapStyleBuilder` 构建，含 `align`、`width`、`height`、`posX`、`posY`、`algorithm`、`value`。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { BitmapStyleBuilder, ImageAlgorithm } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

const bitmapStyle = BitmapStyleBuilder.create()
    .align(Align.CENTER)
    .width(384)
    .height(150)
    .algorithm(ImageAlgorithm.BINARIZATION)
    .value(120)
    .build();
LineApi.printBitmap(base64String, bitmapStyle);
  

```

* * *
### 2.7 打印分割线
**printDividingLine(lineType: number, lineHeight: number)**
打印分割线，用于小票中的视觉分隔。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| lineType  | number  | 线型：0 空 / 1 实线 / 2 虚线，对应枚举 `DividingLine.EMPTY` / `SOLID` / `DOTTED`。  |  
| lineHeight  | number  | 分割线高度（单位：点）。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { DividingLine } from "@/uni_modules/sunmi-printersdk/utssdk/interface.uts";

LineApi.printDividingLine(DividingLine.SOLID, 2);   // 实线，高度 2
LineApi.printDividingLine(DividingLine.DOTTED, 1); // 虚线，高度 1
  

```

* * *
### 2.8 打印结束输出
**autoOut()**
在不开启事务模式时，组版完成后调用本方法，立即将当前缓冲区内容送打印机并走纸。若已开启事务模式，需使用 `printTrans(callback)` 提交，而非 `autoOut()`。
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
LineApi.initLine(baseStyle);
LineApi.printText("Hello Sunmi", TextStyleBuilder.create().build());
LineApi.autoOut();  // 立即打印并出纸
  

```

* * *
### 2.9 事务模式-开关
**enableTransMode(enable: boolean)**
开启或关闭事务模式。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| enable  | boolean  | `true` 开启事务模式（组版仅缓存，不立即出纸）；`false` 关闭。  |  
  * 开启后，所有 `print*` / `addText` 仅写入缓冲区，需配合 **2.10 事务模式-提交** `printTrans` 一次性打印。
  * 关闭后，若需出纸可调用 `autoOut()`。


示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
LineApi.enableTransMode(true);   // 开启事务，后续组版不立即出纸
// ... 组版多行内容 ...
LineApi.printTrans((success, message) => {
    // 提交后再关闭事务（可选）
    LineApi.enableTransMode(false);
});
  

```

* * *
### 2.10 事务模式-提交
**printTrans(callback: (success: boolean, message: string) = > void)**
提交事务：将当前事务模式下已组版的内容一次性发送打印机并走纸。回调返回本次打印是否成功及消息。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| callback  | (success, message) => void  | 回调：`success` 为是否成功，`message` 为结果描述。  |  
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
LineApi.enableTransMode(true);
LineApi.initLine(baseStyle);
LineApi.printText("订单号：202502100001", textStyle);
LineApi.printDividingLine(DividingLine.SOLID, 2);

LineApi.printTrans((success, message) => {
    if (success) {
        console.log("打印成功");
    } else {
        console.error("打印失败:", message);
    }
    LineApi.enableTransMode(false);
});
  

```

* * *
## 3. 枚举参数说明
以下枚举与样式类型定义在 `@/uni_modules/sunmi-printersdk/utssdk/interface.uts`，按需引入。
### 3.1 画布基础样式 BaseStyle（行设置）  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **BaseStyleBuilder**  | `align`、`width`、`height`、`renderColor`、`posX`。  |  
| **Align**  | 对齐：`DEFAULT(0)`、`LEFT(1)`、`CENTER(2)`、`RIGHT(3)`。  |  
| **RenderColor**  | 渲染颜色（双色机且纸张支持时生效）：`BLACK(0)`、`RED(1)`。  |  
### 3.2 文本样式 TextStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **TextStyleBuilder**  | `align`、`fontStyle`、`textSize`、`textWidthRatio`、`textHeightRatio`、`posX`、`posY`、`width`、`height`、`rotate`、`textSpace`。  |  
| **FontStyle**  | 字体样式：`NORMAL(0)`、`BOLD(1)`、`UNDERLINE(2)`、`STRIKETHROUGH(3)`、`ITALIC(4)`、`Invert(5)`、`AntiColor(6)`。  |  
| **Rotate**  | 旋转：`ROTATE_0(0)`、`ROTATE_90(90)`、`ROTATE_180(180)`、`ROTATE_270(270)`。  |  
### 3.3 条码样式 BarcodeStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **BarcodeStyleBuilder**  | `align`、`dotWidth`、`barWidth`、`barHeight`、`readable`、`posX`、`posY`、`width`、`height`、`symbology`。  |  
| **HumanReadable**  | 条码底部文字：`HIDE(0)`、`POS_ONE(1)`、`POS_TWO(2)`、`POS_THREE(3)`。  |  
| **Symbology**  | 条码类型：如 `UPCA(0)`、`UPCE(1)`、`EAN13(2)`、`EAN8(3)`、`CODE39(4)`、`ITF(5)`、`CODABAR(6)`、`CODE93(7)`、`CODE128(8)`、`EAN128(9)` 等。  |  
### 3.4 二维码样式 QrStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **QrStyleBuilder**  | `align`、`dot`、`width`、`height`、`posX`、`posY`、`errorLevel`。  |  
| **ErrorLevel**  | 纠错等级：`L(0)`、`M(1)`、`Q(2)`、`H(3)`。  |  
### 3.5 图片样式 BitmapStyle  
| 构建器/枚举  | 说明  |  
| --- | --- |  
| **BitmapStyleBuilder**  | `align`、`width`、`height`、`posX`、`posY`、`algorithm`、`value`。  |  
| **ImageAlgorithm**  | 二值化算法：`BINARIZATION(0)`、`DITHERING(1)`。`value` 为阈值等参数。  |  
### 3.6 分割线 DividingLine  
| 枚举  | 说明  |  
| --- | --- |  
| **DividingLine**  | `EMPTY(0)` 空、`SOLID(1)` 实线、`DOTTED(2)` 虚线。  |  
上一篇：Uniapp SDK概述
下一篇：Uniapp 打印标签小票接口
