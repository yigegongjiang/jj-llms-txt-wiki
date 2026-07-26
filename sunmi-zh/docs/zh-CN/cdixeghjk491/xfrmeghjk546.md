---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfrmeghjk546
---

# 8、标签打印机TSPL指令集
更新时间：2026-07-02 20:02:32
本功能仅支持以下机型和版本：
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
本⽂档主要介绍商米打印机在标签模式下的TSPL指令集。需要使用TSPL指令集，打印机需要切换到标签打印模式下才有效。
需要切换打印模式时，按住【配网键】+按下【音量减键】进行模式切换，模式切换时会伴随语音和打印内容提示当前所处的打印模式。
# **系统设置指令**
## **SIZE : 设置标签的宽度和长度**  
| [类型]  | [格式]  |  
| --- | --- |  
| 英制系统（英寸）  | SIZE m,n  |  
| 公制系统（毫米）  | SIZE m mm,n mm  |  
| 点系统（像素点）  | SIZE m dot,n dot  |  
| [参数]  | [说明]  |  
| --- | --- |  
| m  | 标签宽度  |  
| n  | 标签长度  |  
| mm  | 毫米。使用该系统时，必须在参数和"mm"之间加空格。打印头片203 DPI时，1mm=8dots  |  
| dot  | 像素点。使用该系统时，必须在参数和"dot"之间加空格。打印头片203 DPI时，1mm=8dots  |  
  

![](https://cdn.sunmi.com/public/image/mgt-document/d7914fd577a243fd96b4e4decad95816.png)
[举例]
英制系统（英寸）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 1.5, 2.2
  

```

公制系统（毫米）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 38.1 mm, 55.88 mm
  

```

## **GAP : 设置两张标签之间的垂直距离**  
| [类型]  | [格式]  |  
| --- | --- |  
| 英制系统（英寸）  | GAP m,n  |  
| 公制系统（毫米）  | GAP m mm,n mm  |  
| 连续纸  | GAP 0,0  |  
| [参数]  | [说明]  |  
| --- | --- |  
| m  | 两张标签之间的垂直距离，0≤m≤1 (inch)，0≤m≤25.4 (mm)  |  
| n  | 垂直间距的偏移。n≤标签长度（英寸或毫米）  |  
| mm  | 毫米。使用该系统时，必须在参数和"mm"之间加空格。  |  
| 0,0  | m=0和n=0表示使用连续纸  |  
[举例]：一般垂直间距
英制系统（英寸）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
GAP 0.12,0
  

```

公制系统（毫米）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
GAP 3 mm,0 mm
  

```

连续纸
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
GAP 0,0
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/067e3449032540bcbfc093f62f4bc76b.png)
[举例]：特殊垂直间距
英制系统（英寸）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
GAP 0.30,0.10
  

```

公制系统（毫米）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
GAP 7.62 mm,2.54 mm
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/61a4ee6c8814479d9bc4fffd0ee69a62.png)
## **GAPDECTECT : 自动检测纸张和间隔大小**
收到该命令以后，打印机自动走纸，并通过间距传感器自动确定纸张长度和间隔缝隙大小。本命令受到纸张多项因素影响，可能出现自动测量与实际大小不符，此时该命令将不再适用该类型纸张。此校准方法可应用于带有预打印徽标或文本的标签。  
| [类型]  | [格式]  |  
| --- | --- |  
| 点系统（像素点）  | GAPDECTECT [x,y]  |  
| [参数]  | [说明]  |  
| --- | --- |  
| x  | 以点dots为单位的纸宽。如果x、y被忽略时，打印机将自动校准并确定纸张长度和间隙大小。  |  
| y  | 以点dots为单位的间距宽。如果x、y被忽略时，打印机将自动校准并确定纸张长度和间隙大小。  |  
## **OFFSET : 设置纸张停止的位置**
此条指令为每一张标签定义额外的送纸偏移，对于剥纸模式及裁切模式下，通过调整此条指令，可以使打印机将纸张停留在预期的位置上。打印机在每次打印前会将额外设定的距离回卷后再打印。  
| [类型]  | [格式]  |  
| --- | --- |  
| 英制系统（英寸）  | OFFSET m  |  
| 公制系统（毫米）  | OFFSET m mm  |  
| [参数]  | [说明]  |  
| --- | --- |  
| m  | 纸张停止的距离，-1≤m≤1 (inch)。不恰当的偏移量可能会导致卡纸。  |  
| mm  | 毫米。使用该系统时，必须在参数和"mm"之间加空格。  |  
  

![](https://cdn.sunmi.com/public/image/mgt-document/19540e8eb3834b3e86db0b1277598d36.png)
[举例]
英制系统（英寸）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
OFFSET 0.5
  

```

公制系统（毫米）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
OFFSET 12.7 mm
  

```

## **SPEED : 设置打印速度**  
| [类型]  | [格式]  |  
| --- | --- |  
| 英制系统（英寸）  | SPEED n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 打印速度，单位：英寸/秒。最低打印速度为2（ips），最高打印速度请参照产品规格定义。n值如为无效范围，打印机将自动忽略。  |  
[举例]
英制系统（英寸）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SPEED 4
  

```

## **DENSITY : 设置打印浓度**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | DENSITY n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 打印浓度，参数范围0≤n≤15。0表示最淡的打印浓度；15表示最深的打印浓度。默认n=8  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DENSITY 8
  

```

## **DIRECTION : 设置出纸方向和镜像图像**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | DIRECTION n[,m]  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 0或1。打印内容在走纸方向上的打印顺序  |  
| m  | 0：表示打印正常图像；1：表示打印镜像图像  |  
  

![](https://cdn.sunmi.com/public/image/mgt-document/897ea405dd64418297973d5069241b8f.png)
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DIRECTION 0
  

```

bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DIRECTION 0,1
  

```

## **REFERENCE : 设置标签打印的坐标原点**
参考原点的位置将会受到打印方向的影响  
| [类型]  | [格式]  |  
| --- | --- |  
| 点系统（像素点）  | REFERENCE x,y  |  
| [参数]  | [说明]  |  
| --- | --- |  
| x  | 水平方向的坐标位置，以点（dots）为单位。参考点将根据打印方向设置而变化。打印头片203 DPI时，1mm=8dots  |  
| y  | 垂直方向的坐标位置，以点（dots）为单位。参考点将根据打印方向设置而变化。打印头片203 DPI时，1mm=8dots  |  
  

![](https://cdn.sunmi.com/public/image/mgt-document/41b1efe6a4df4b8480df69db82c2561c.png)
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
REFERENCE 10,10
  

```

## **SHIFT : 设置进纸/退纸**
该命令用于移动标签的垂直位置。正值将标签从打印方向进一步移动，负值将标签反方向移动。  
| [类型]  | [格式]  |  
| --- | --- |  
| 点系统（像素点）  | SHIFT n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | -90≤n≤90，以点（dots）为单位。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 60 mm,45 mm
GAP 2 mm,0
DIRECTION 0
SHIFT 0
OFFSET 0
CLS
TEXT 40,20,"0",0,1,1,"方向 0"
TEXT 40,50,"0",0,1,1,"移动 0"
BOX 20,10,460,320,8
PRINT 2,1

SIZE 60 mm,45 mm
GAP 2 mm,0
DIRECTION 1
SHIFT 0
OFFSET 0
CLS
TEXT 40,20,"0",0,1,1,"方向 1"
TEXT 40,50,"0",0,1,1," 移动 0"
BOX 20,10,460,320,8
PRINT 2,1
  
SIZE 60 mm,45 mm
GAP 2 mm,0
DIRECTION 0
SHIFT 36
OFFSET 0
CLS
TEXT 40,20,"0",0,1,1,"方向 0"
TEXT 40,50,"0",0,1,1,"移动 36"
BOX 20,10,460,320,8
PRINT 1,1
  
SIZE 60 mm,45 mm
GAP 2 mm,0
DIRECTION 1
SHIFT 36
OFFSET 0
CLS
TEXT 40,20,"0",0,1,1,"方向 1"
TEXT 40,50,"0",0,1,1,"移动 36"
BOX 20,10,460,320,8
PRINT 1,1
  
SIZE 60 mm,45 mm
GAP 2 mm,0
DIRECTION 0
SHIFT -36
OFFSET 0
CLS
TEXT 40,20,"0",0,1,1,"方向 0"
TEXT 40,50,"0",0,1,1,"移动 -36"
BOX 20,10,460,320,8
PRINT 1,1
  
SIZE 60 mm,45 mm
GAP 2 mm,0
DIRECTION 1
SHIFT -36
OFFSET 0
CLS
TEXT 40,20,"0",0,1,1,"方向 1"
TEXT 40,50,"0",0,1,1,"移动 -36"
BOX 20,10,460,320,8
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/fe16d3e1c6794efb877a7c34f2025f81.png)
## **CODEPAGE : 设置国际字符集的代码页**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | CODEPAGE n  |  
|   
 | CODEPAG “n“  |  
|   
 | CODEPAG name  |  
|   
 | CODEPAG “name”  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 字符集的名称或代码号，可进一步区分7-bit或8-bit。数据长度决定7位或8位通讯参数。  |  
| 字符编码  | 参数值 n  | 名称 Name  | 带前缀写法  | 适用语言  |  
| --- | --- | --- | --- | --- |  
| UTF-8  | 65001、0、65279  | utf8、utf  | cp65001、windows65001、cp0、windows0  | 全球Unicode  |  
| GB18030  | 936、54936  | gb18030、gbk  | cp930、windows936、cp54936、windows54936  | 简体中文  |  
| Big5  | 950  | big5  | cp950、windows950  | 繁体中文  |  
| Shift_JIS  | 932  | shiftjis、sjis  | cp932、windows932  | 日文  |  
| KSC5601  | 949、1361  | ksc5601、cp949、euckr  | cp949、windows949、cp1361、windows1361  | 韩文  |  
| JIS0208  | 20932  | jis0208、eucjp  | cp20932、windows20932  | 日文  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
CODEPAGE "936"
CODEPAGE 936
```

## **CLS : 清除图像缓冲区**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | CLS  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 | 该命令必须放置在SIZE命令后  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
CLS
  

```

## **FEED : 控制标签进纸距离**  
| [类型]  | [格式]  |  
| --- | --- |  
| 点系统（像素点）  | FEED n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 1≤n≤9999，以点（dots）为单位。打印头片203 DPI时，1mm=8dots  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//203dpi：80 dots=10mm
FEED 80
  

```

## **BACKFEED : 控制标签退纸距离**  
| [类型]  | [格式]  |  
| --- | --- |  
| 点系统（像素点）  | BACKFEED n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 1≤n≤9999，以点（dots）为单位。打印头片203 DPI时，1mm=8dots。不恰当的退纸可能会导致卡纸。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
BACKFEED 40
  

```

## **FORMFEED : 走纸到下一张标签的起始位置**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | FORMFEED  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 |   
 |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,2.5
GAP 2 mm,0
DIRECTION 1
FORMFEED
CLS
TEXT 25,25,"0",0,1,1,"FORMFEED COMMAND TEST"
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/80d89746ca3340138da72eebcda8d566.png)
## **HOME : 标签走纸到原点**
在使用含有间隙或黑标的标签纸时，若不能确定第一张标签纸是否在正确打印位置时，此指令可将标签纸向前推送至下一张标签纸的开始打印起点。  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | HOME  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 | 使用该命令之前，应定义标签的大小和间隙  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 60 mm,45 mm
GAP 2 mm,0
HOME
CLS
BOX 1,1,360,65,12
TEXT 25,25,"0",0,1,1,"HOME PRINT 2,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/d64cea1f90f64dcc8005b10aa6c7c0b7.png)
## **PRINT : 打印当前缓冲区中的数据**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | PRINT m[,n]  |  
| [参数]  | [说明]  |  
| --- | --- |  
| m  | 指定打印的份数，1≤m≤999999999。如果m=1，打印机将打印最后一个标签内容n份。  |  
| n  | 指定每份打印的份数，1≤n≤999999999。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 60 mm,45 mm
GAP 3 mm,0
DIRECTION 1
SET COUNTER @1 1
@1="0001"
CLS
TEXT 10,10,"1",0,1,1,@1
PRINT 3,2
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/90a1dd3f6de94b1aa06dd62a95a88c09.png)
## **SELFTEST : 打印自检页信息**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | SELFTEST  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 | 打印自检页，读取打印机的信息  |  
## **BOLD : 设置加粗字体**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | BOLD n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | n=0，关闭加粗字体设置；n=1，开启字体加粗设置。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 60 mm,45 mm
CLS
BOLD 0
TEXT 200,100,"0",0,1,1,"font"
BOLD 1
TEXT 200,150,"0",0,1,1,"font bold"
PRINT 1,1
  

```

## **CUT : 立即执行切刀动作**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | CUT  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 | 立即执行切刀动作。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 3,3
GAP 0,0
CLS
BOX 0,0,866,866,5
TEXT 100,100, "5",0,1,1, "FEED & CUT"
TEXT 100,200, "5",0,1,1, "300 DPI"
PRINT 1,1
FEED 260
CUT
```

# **标签格式指令**
## **BAR : 绘制条形图**  
| [类型]  | [格式]  |  
| --- | --- |  
| 点系统（像素点）  | BAR x, y, width, height  |  
| [参数]  | [说明]  |  
| --- | --- |  
| x  | 左上角开始的x坐标，以点（dots）为单位  |  
| y  | 左上角开始的y坐标，以点（dots）为单位  |  
| width  | 条形图宽，以点（dots）为单位  |  
| height  | 条形图高，以点（dots）为单位  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 60 mm,45 mm
GAP 3 mm,0
DIRECTION 1
CLS
BAR 80,80,300,100
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/80034c36b76a4db2b07bdffc77dcd384.png)
## **BARCODE : 打印一维条形码**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | BARCODE x, y, "code type", height, human readable, rotation, narrow, wide, "code"  |  
| [参数]  | [说明]  |  
| --- | --- |  
| x  | 指定条码打印位置的x坐标  |  
| y  | 指定条码打印位置的y坐标  |  
| code type  | 可用的条形码类型：   
  
Code 39；   
  
Code 128，自动切换代码子集A、B、C。   
  
EAN 8；   
  
EAN 13；   
  
Interleaved 2 of 5；   
  
Code 93；   
  
UPC-A；   
  
UPC-E；   
  
Codebar；   
  
EAN18；   
  
CPOST；   
  
  
  
Code 128M，手动切换代码子集A、B、C。  |  
| height  | 条码高度，以点（dots）为单位  |  
| human readable  | 条码下方显示码值位置。0：字符不打印；1：字符左对齐打印；2：字符居中打印；3：字符右对齐打印  |  
| rotation  | 条码旋转角度，顺时针方向。0：不旋转；90：顺时针方向旋转90度；180：顺时针方向旋转180度；270：顺时针方向旋转270度。  |  
| narrow  | 窄条码比例因子，以点（dots）为单位  |  
| wide  | 宽条码比例因子，以点（dots）为单位  |  
| code  | 打印的字符串内容  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,1
GAP 0,0
DIRECTION 1
CLS
TEXT 10,10, "2",0,1,1, "Human readable alignment"
BARCODE 10,50, "128",100,1,0,2,2,"left"
BARCODE 310,50, "128",100,2,0,2,2,"center"
BARCODE 610,50, "128",100,3,0,2,2,"right"
PRINT 1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/195b3ea5cb48443a81fc9ec78af5875a.png)
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,1
GAP 0,0
DIRECTION 1
CLS
TEXT 10,10, "2",0,1,1, "Code 128, switch code subset automatically. "
BARCODE 10,50, "128",100,1,0,2,2, "123456abcd123456"
PRINT 1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/66b86072325c4a0fadcf394ec0a22f6c.png)
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,1
GAP 0,0
DIRECTION 1
CLS
TEXT 10,10, "2",0,1,1, "Code 128, switch code subset manually."
BARCODE 10,50, "128M",100,1,0,2,2, "!104!096ABCD!101EFGH"
PRINT 1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/544862a1a729477b9e16d1efc631f295.png)
## **BITMAP : 打印位图图像**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | BITMAP X,Y, width, height, mode, bitmap data...  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X  | 点阵图像的水平起始位置  |  
| Y  | 点阵图像的垂直起始位置  |  
| width  | 图像宽度，以字节（Byte）为单位  |  
| height  | 图像高度，以字节（Byte）为单位  |  
| mode  | 图像绘制模式。0:OVERWRITE；1:OR；2:XOR；3:Mini LZO  |  
| bitmap data...  | 位图数据。使用LZO算法时，前4个字节表示压缩数据的总数。总数据（4字节）不压缩，并且低位在前。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,2
GAP 0,0
CLS
BITMAP 200,200,2,16,0,  -?-????
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/5b037ac7d7b0431ab96678fb8a6ba317.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/24e1b8eaef5a4790b127dffdaaf64834.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/b51bace458ea44f88b3858f92cbdfe0e.png)
## **BOX : 绘制矩形框**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | BOX X_start, Y_start, X_end, Y_end, line thickness  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X_start  | 矩形水平方向左上角起始位置，以点（dots）为单位。  |  
| Y_start  | 矩形垂直方向左上角起始位置，以点（dots）为单位。  |  
| X_end  | 矩形水平方向右上角终点位置，以点（dots）为单位。  |  
| Y_end  | 矩形垂直方向右上角终点位置，以点（dots）为单位。  |  
| line thickness  | 矩形框线条粗细度，以点（dots）为单位。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,1.1
CLS
BOX 60,60,610,210,4
BOX 80,80,590,190,4
BOX 100,100,570,170,4,20
BOX 120,120,550,150,4,20
PRINT 1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/fd8006752d97491e9bb3525d98b7c1a7.png)
## **CIRCLE : 绘制圆圈**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | CIRCLE X_start, Y_start, diam eter, circle thickness  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X_start  | 圆形水平方向左上角起始位置，以点（dots）为单位。  |  
| Y_start  | 圆形垂直方向左上角起始位置，以点（dots）为单位。  |  
| diam eter  | 指定圆的直径，以点（dots）为单位。  |  
| circle thickness  | 圆形框线条粗细度，以点（dots）为单位。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 80 mm,30 mm
GAP 0,0
DIRECTION 1
CLS
BAR 250,20,100,1
BAR 250,20,1,100
CIRCLE 250,20,100,5
PRINT 1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/9977229ebb3f42e89a906efe128efd95.png)
## **ERASE : 清除图像缓冲区中指定区域的数据**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | ERASE X_start, Y_start, X_width, Y_height  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X_start  | 水平方向的起始位置，以点（dots）为单位。  |  
| Y_start  | 垂直方向的起始位置，以点（dots）为单位。  |  
| X_width  | 水平方向宽度，以点（dots）为单位。  |  
| Y_height  | 垂直方向宽度，以点（dots）为单位。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,2.5
GAP 0,0
DIRECTION 1
CLS
BAR 100,100,300,300
ERASE 150,150,200,200
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/c6d1df2703434bef8dbdc4baefa4a953.png)
## **QRCODE : 打印二维QR码**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | QRCODE X, Y, ECC Level, cell width, mode, rotation, [model, mask] "Data string"  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X  | QR条码的左上角X坐标  |  
| Y  | QR条码的左上角Y坐标  |  
| ECC Level  | 纠错恢复等级。L:7%；M:15%；Q:25%；H:30%  |  
| cell width  | 1,3,5,7,10,12  |  
| mode  | 自动生成编码/手动生成编码。A:Auto；M:Manual  |  
| rotation  | 顺时针旋转角度。0:不旋转；90:顺时针旋转90度；180:顺时针旋转180度；270:顺时针旋转270度；  |  
| model  | 条码生产样式。1:默认版本；2:扩大版本；  |  
| mask  | 范围0~8，默认值=7  |  
| Data string  | 编码字符集数据  |  
编码字符集：
1）数字数据：0~9
2）字母数字式：数字0~9；大写字母A~Z；九种其他字符：space，$%*+-./:
3）8位字节数据：JIS 8位字符集（拉丁文和假名）符合JIS X 0201
4）日文汉字数据：JIS值包含8140 HEX~9FFC HEX 和 E040 HEX~EAA4 HEX
条码的最大资料长度：  
|   
 | Model 1 (Version 14-L)  | Model 2 (Version 40-L)  |  
| --- | --- | --- |  
| 1）Numeric data  | 1,167 characters  | 7,089 characters  |  
| 2）Alphanumeric data  | 707 characters  | 4,296 characters  |  
| 3）8-bit byte data  | 486 characters  | 2,953 characters  |  
| 4）Kanji data  | 299 characters  | 1,817 characters  |  
手动生成编码注意：
1）如条码内容的第一个字符为”A“，则后续的数据为”文字数字“形态。
2）如条码内容的第一个字符为”N“，则后续的数据为”数字“形态。
3）如条码内容的第一个字符为”B“，则后续的4位数字表示紧接二进制数据的长度（单位为byte），且后续的资料为”二进制资料“形态。
4）如条码内容的第一个字符为”K“，则后续的数据为”日文汉字“形态。
5）”!“则是用来转换资料的格式，”N“、”A“、“B”、“K”等数据形态可透过"!"的转换组成一组条码内容。
[举例]
自动模式，通用数据串
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,2.5
GAP 0,0
DIRECTION 1
CLS
QRCODE 10,10,H,4,A,0,"ABCabc123"
QRCODE 160,160,H,4,A,0,"123ABCabc"
QRC
ODE 310,310,H,4,A,0," 印表機 ABCabc123"
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/4795f76180d047ddb64e32bbf28b9463.png)
自动模式，数据字符串，包括双引号 (“) 字符，请在程序内使用 \\[“]格式打印双引号
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,2.5
GAP 0,0
DIRECTN 1 CLS
QRCODE 10,10,H,4,A,0,"ABC\["]abc\["]123"
QRCODE 160,160,H,4,A,0,"123\["]ABC\["]abc"
QRCODE 310,310,H,4,A,0,"\["]印表機\["]ABCabc123"
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/a2ab83f396074368879d2a83cb4b58d3.png)
## **REVERSE : 设置黑白反转打印**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | REVERSE X_start, Y_start, X_width,Y_height  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X_start  | X坐标的起始位置，以点（dots）为单位。  |  
| Y_start  | Y坐标的起始位置，以点（dots）为单位。  |  
| X_width  | X反向反白区域宽度，以点（dots）为单位。  |  
| Y_height  | Y反向反白区域宽度，以点（dots）为单位。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 4,2.5
GAP 0,0
SPEED 6
DENSITY 8
DIRECTION 0
CLS
TEXT 100,100,"3",0,1,1,"REVERSE"
REVERSE 90,90,128,40
PRINT 1,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/a3e0de702c6948dcb4bb8b3966a55930.png)
## **TEXT : 打印文字**  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | TEXT X, Y, ”font”, rotation, x-multiplication, y-multiplication, [alignment,] “content”  |  
| [参数]  | [说明]  |  
| --- | --- |  
| X  | 打印文字左上角X坐标  |  
| Y  | 打印文字左上角Y坐标  |  
| font  | 字形名称。0:12x24 ASCII, 24x24 GBK；1:8x16 ASCII, 16x16 GBK  |  
| rotation  | 文字旋转角度，顺时针方向。0：不旋转；90：顺时针方向旋转90度；180：顺时针方向旋转180度；270：顺时针方向旋转270度。  |  
| x-multiplication  | 水平放大值，最大可放大至10倍。值范围1~10  |  
| y-multiplication  | 垂直放大值，最大可放大至10倍。值范围1~10  |  
| alignment  | 指定文本的对齐方式。0:默认（居左）；1:居左；2:居中；3:居右；  |  
| content  | 打印文本内容  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 44mm,56 mm
CLS
TEXT 20,10,"0",0,2,2,"中文字体 FONT 0"
TEXT 20,120,"1",0,2,2,"中文字体 FONT 1"
PRINT 1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/de93c2f616a94412b3750c20d2a73138.png)
# **状态命令**
## **< ESC>!? : 获取打印机状态**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | ESC ! ?  |  
| HEX  | 1B 21 3F  |  
| Decimal  | 27 33 63  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 | 通过端口送出指令用来获得打印机目前状态，其中<ESC>符号表示ASCII 27 (Hex1B)。在打印机发生错误状态时，此项指令也可获取打印机状态。若返回值为0表示打印机已准备好打印标签。  |  
返回数据：  
| 位  | 状态  |  
| --- | --- |  
| 0  | 就绪  |  
| 1  | 卡纸  |  
| 2  | 缺纸  |  
| 3  | 无碳带  |  
| 4  | 暂停打印  |  
| 5  | 打印中  |  
| 6  | 上盖开启  |  
| 7  | 过热  |  
## **< ESC>!R : 重启打印机**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | ESC ! R  |  
| HEX  | 1B 21 52  |  
| Decimal  | 27 33 82  |  
| [参数]  | [说明]  |  
| --- | --- |  
|   
 | 通过端口送出指令使打印机重新开机，其中<ESC>符号表示ASCII 27 (Hex1B)，此项指令可随时被送出，重新开机后DRAM中的资料将被清除。  |  
# **设备配置指令**
## **SET COUNTER : 设置计数器**
计数器可以是实数计数器或是变量，此指令用于设置程序中的计数器编号及其增量。计数器包含三种形态：数位（0~9~0）、小写字母（a~z~a）、大写字母（A~Z~A）  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | SET COUNTER @n step@n = "Expression"  |  
| [参数]  | [说明]  |  
| --- | --- |  
| @n  | n:计数器编号。打印机中有51个计数器可用（@0~@50）  |  
| step  | 计数器的增量，可以是正数或负数。-999999999≤step≤999999999。如果计数器用作固定变量，请将增量设置为0  |  
| Expression  | 初始字符串，字符串长度为101字节  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 60 mm,45 mm
GAP 3 mm,0
DIRECTION 1
SET COUNTER @1 1
@1="0001"
CLS
TEXT 10,10,"1",0,1,1,@1
PRINT 3,1
  

```

  

![](https://cdn.sunmi.com/public/image/mgt-document/923852a71567453e9fb7e6811164e546.png)
## **SET TEAR : 设置定位到撕纸位开关**
该设置用于启用或禁止将标签纸缝隙定位到撕纸的位置。  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | SET TEAR ON/OFF  |  
| [参数]  | [说明]  |  
| --- | --- |  
| ON  | 打印完成后，标签间隔将停止在撕纸位置。  |  
| OFF  | 打印后，标签间隔不会在撕纸位置停止，并且标签的开头将与打印头对齐。  |  
[举例]
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 3,3
GAP 0.08,0
DIRECTION 0
REFERENCE 0,0
SET TEAR ON
CLS
TEXT 50,100,"0",0,1,1,"test"
PRINT 1
  

```

## **SET CUTTER : 设置切刀**
此设置可激活或关闭切刀，并确定一次能切割多少个打印标签。该设置在关闭电源后会保存在打印机内存中。  
| [类型]  | [格式]  |  
| --- | --- |  
|   
 | SET CUTTER OFF/BATCH/Pieces  |  
| [参数]  | [说明]  |  
| --- | --- |  
| OFF  | 禁用切割功能。  |  
| BATCH  | 将打印机设置为在打印作业结束时裁剪标签。  |  
| Pieces  | 设置每次切割的打印标签数量。0 ≤ Pieces ≤ 65535  |  
[举例]
禁用切刀功能。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SIZE 3,3
GAP 0,0
SET CUTTER OFF
SET PEEL OFF
CLS
TEXT 50,50, "3",0,1,1, "SET CUTTER OFF"
PRINT 3
```

全部打印完以后（范例中打印6个标签），执行一次切刀。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SET CUTTER BATCH
CLS
TEXT 50,50, "3",0,1,1, "SET CUTTER BATCH"
PRINT 3,2
```

每一张标签都执行一次切刀。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SET CUTTER 1
CLS
TEXT 50,50, "3",0,1,1, "SET CUTTER 1"
PRINT 3,2
```

每两张标签执行一次切刀。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SET CUTTER 2
CLS
TEXT 50,50, "3",0,1,1, "SET CUTTER 2"
PRINT 3,2
```

# **商米私有命令**
## **< ESC>p<m><t1><t2> : 驱动钱箱**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | ESC p m t1 t2  |  
| HEX  | 1B 70 m t1 t2  |  
| Decimal  | 27 112 m t1 t2  |  
| [参数]  | [说明]  |  
| --- | --- |  
| m  | 默认值=0  |  
| t1  | 在钱箱口输出脉冲，高电平时间为(t1×2 ms)。  |  
| t2  | 在钱箱口输出脉冲，低电平时间为(t2×2 ms)。  |  
## **< GS>I<n> : 获取打印机信息**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS I n  |  
| HEX  | 1D 49 n  |  
| Decimal  | 29 73 n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| n  | 获取打印机信息。65：打印机应用版本号；66：打印机生产厂商，"SUNMI"；67：打印机型号；68：打印机序列号；69：打印机字符编码；  |  
返回数据的格式如下：  
| 内容  | 数据  | 长度  |  
| --- | --- | --- |  
| 首字节  | 0x5F (十进制95，'_'字符)  | 1 byte  |  
| 打印机信息  | ASCII 32~126  | 0-80 bytes  |  
| 尾字节  | 0x00 (NUL字符)  | 1 byte  |  
## **< GS>(T<pL><pH><fn> : <function 1>查询打印机状态**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn  |  
| HEX  | 1D 28 54 01 00 01  |  
| Decimal  | 29 40 84 1 0 1  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=1  |  
| pH  | 默认值=0  |  
| fn  | 默认值=1  |  
向打印机发送数据或命令后，至少等待600ms才能发送这条命令，打印机返回1字节状态值。
返回数据：  
| 位  | 状态  |  
| --- | --- |  
| 0  | 1：打印头正在打印  |  
| 1  | 1：缺纸  |  
| 2  | 1：纸卷即将耗尽 NOTE1  |  
| 3  | 1：发生堵纸NOTE1  |  
| 4  | 1：切纸后未取纸 NOTE1  |  
| 5  | 1：纸仓盖已打开 NOTE1  |  
| 6  | 1：打印头过热  |  
| 7  | 1：走纸马达过热 NOTE1  |  
## **< GS>(T<pL><pH><fn> : <function 2>获取最近一个打印任务的编号**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn  |  
| HEX  | 1D 28 54 01 00 02  |  
| Decimal  | 29 40 84 1 0 2  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=1  |  
| pH  | 默认值=0  |  
| fn  | 默认值=2  |  
向打印机发送打印数据后，在打印机内部会形成一个打印任务并被放到打印队列中等待打印，并分配给每个打印任务一个唯一的任务编号。这条命令用于获取当前通道最近一个打印任务的编号，打印机返回4字节任务编号。向打印机发送数据或命令后，至少等待600ms才能发送这条命令。
返回数据：  
| 位  | 状态  |  
| --- | --- |  
| d1…d4  | 获取最近一个打印任务的编号。  |  
## **< GS>(T<pL><pH><fn><d1...d4> : <function 3>查询打印任务编号状态**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn d1 d2 d3 d4  |  
| HEX  | 1D 28 54 05 00 03 d1 d2 d3 d4  |  
| Decimal  | 29 40 84 5 0 3 d1 d2 d3 d4  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=5  |  
| pH  | 默认值=0  |  
| fn  | 默认值=3  |  
| d1~d4  | d1~d4为4字节任务编号。  |  
向打印机发送数据或命令后，至少等待600ms才能发送这条命令。
返回数据：  
| 值  | 任务状态  |  
| --- | --- |  
| 0  | 未知（例如任务编号不存在）  |  
| 1  | 尚未打印  |  
| 2  | 正在打印  |  
| 3  | 打印完成  |  
## **< GS>(T<pL><pH><fn><d1…d4> : <function 5>清除打印任务缓冲数据**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn d1 d2 d3 d4  |  
| HEX  | 1D 28 54 05 00 05 d1 d2 d3 d4  |  
| Decimal  | 29 40 84 5 0 5 d1 d2 d3 d4  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=5  |  
| pH  | 默认值=0  |  
| fn  | 默认值=5  |  
| d1~d4  | 清除指定的打印任务缓冲区，正在打印阻塞的任务也可以清除。d1~d4为4字节任务编号，”0000”表示所有打印任务。  |  
向打印机发送数据或命令后，至少等待600ms才能发送这条命令。
返回数据：  
| 值  | 任务状态  |  
| --- | --- |  
| 0  | 未知错误（例如任务编号不存在）  |  
| 1  | 清除完成  |  
## **< GS>(E<pL><pH><fn><n> : <function 7>设置打印浓度**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( E pL pH fn n  |  
| HEX  | 1D 28 45 02 00 07 n  |  
| Decimal  | 29 40 69 2 0 7 n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=2  |  
| pH  | 默认值=0  |  
| fn  | 默认值=7  |  
| n  | 设置打印浓度为n%。70≤n≤130，或者n=255恢复到默认值100。  |  
## **< GS>(E<pL><pH><fn><n> : <function 8>设置打印速度**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( E pL pH fn n  |  
| HEX  | 1D 28 45 02 00 08 n  |  
| Decimal  | 29 40 69 2 0 8 n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=2  |  
| pH  | 默认值=0  |  
| fn  | 默认值=8  |  
| n  | 设置打印速度，降低打印速度可以提高打印质量。打印质量与打印速度并不是线性的反比关系。当打印速度降到一定程度后继续降低打印速度，打印质量不会再有明显的提升。0≤n≤250，或者n=255恢复到默认值。  |  
## **< GS>(T<pL><pH><fn> : <function 9>查询打印机状态**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn  |  
| HEX  | 1D 28 54 01 00 09  |  
| Decimal  | 29 40 84 1 0 9  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=1  |  
| pH  | 默认值=0  |  
| fn  | 默认值=9  |  
向打印机发送数据或命令后，至少等待600ms才能发送这条命令，打印机返回1字节状态值。
**接收返回数据格式：**  
| **内容**  | **Hex**  | **Dec**  | **数据长度**  |  
| --- | --- | --- | --- |  
| 首字节  | 37H  | 55  | 1 byte  |  
| 标识符  | 09H  | 9  | 1 byte  |  
| 状态信息  | n  | n  | 1 bytes  |  
| 结束符  | 00H  | 0  | 1 byte  |  
n返回数据：  
| 位  | 状态  |  
| --- | --- |  
| 0  | 1：打印头正在打印  |  
| 1  | 1：缺纸  |  
| 2  | 1：纸卷即将耗尽 NOTE1  |  
| 3  | 1：发生堵纸NOTE1  |  
| 4  | 1：切纸后未取纸 NOTE1  |  
| 5  | 1：纸仓盖已打开 NOTE1  |  
| 6  | 1：打印头过热  |  
| 7  | 1：走纸马达过热 NOTE1  |  
## **< GS>(T<pL><pH><fn> : <function 11>获取最近一个打印任务的编号**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn  |  
| HEX  | 1D 28 54 01 00 0B  |  
| Decimal  | 29 40 84 1 0 11  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=1  |  
| pH  | 默认值=0  |  
| fn  | 默认值=11  |  
向打印机发送打印数据后，在打印机内部会形成一个打印任务并被放到打印队列中等待打印，并分配给每个打印任务一个唯一的任务编号。这条命令用于获取当前通道最近一个打印任务的编号，打印机返回4字节任务编号。向打印机发送数据或命令后，至少等待600ms才能发送这条命令。
**接收返回数据格式：**  
| **内容**  | **Hex**  | **Dec**  | **数据长度**  |  
| --- | --- | --- | --- |  
| 首字节  | 37H  | 55  | 1 byte  |  
| 标识符  | 0BH  | 11  | 1 byte  |  
| 任务编号  | 00H~FFH  | 0~255  | 4 bytes  |  
| 结束符  | 00H  | 0  | 1 byte  |  
## **< GS>(T<pL><pH><fn><d1...d4> : <function 12>查询打印任务编号状态**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn d1 d2 d3 d4  |  
| HEX  | 1D 28 54 05 00 0C d1 d2 d3 d4  |  
| Decimal  | 29 40 84 5 0 12 d1 d2 d3 d4  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=5  |  
| pH  | 默认值=0  |  
| fn  | 默认值=12  |  
| d1~d4  | d1~d4为4字节任务编号。  |  
向打印机发送数据或命令后，至少等待600ms才能发送这条命令。
**接收返回数据格式：**  
| **内容**  | **Hex**  | **Dec**  | **数据长度**  |  
| --- | --- | --- | --- |  
| 首字节  | 37H  | 55  | 1 byte  |  
| 标识符  | 0CH  | 12  | 1 byte  |  
| 状态信息  | n  | n  | 1 bytes  |  
| 结束符  | 00H  | 0  | 1 byte  |  
n返回数据：  
| 值  | 任务状态  |  
| --- | --- |  
| 0  | 未知（例如任务编号不存在）  |  
| 1  | 尚未打印  |  
| 2  | 正在打印  |  
| 3  | 打印完成  |  
## **< GS>(T<pL><pH><fn><d1…d4> : <function 13>清除打印任务缓冲数据**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( T pL pH fn d1 d2 d3 d4  |  
| HEX  | 1D 28 54 05 00 0D d1 d2 d3 d4  |  
| Decimal  | 29 40 84 5 0 13 d1 d2 d3 d4  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=5  |  
| pH  | 默认值=0  |  
| fn  | 默认值=13  |  
| d1~d4  | 清除指定的打印任务缓冲区，正在打印阻塞的任务也可以清除。d1~d4为4字节任务编号，”0000”表示所有打印任务。  |  
向打印机发送数据或命令后，至少等待600ms才能发送这条命令。
**接收返回数据格式：**  
| **内容**  | **Hex**  | **Dec**  | **数据长度**  |  
| --- | --- | --- | --- |  
| 首字节  | 37H  | 55  | 1 byte  |  
| 标识符  | 0DH  | 13  | 1 byte  |  
| 任务状态  | n  | n  | 1 bytes  |  
| 结束符  | 00H  | 0  | 1 byte  |  
n返回数据：  
| 值  | 任务状态  |  
| --- | --- |  
| 0  | 未知错误（例如任务编号不存在）  |  
| 1  | 清除完成  |  
## **< GS>(E<pL><pH><fn><n> : <function 20>设置打印模式**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( E pL pH fn n  |  
| HEX  | 1D 28 45 02 00 14 n  |  
| Decimal  | 29 40 69 2 0 20 n  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=2  |  
| pH  | 默认值=0  |  
| fn  | 默认值=20  |  
| n  | 设置打印模式，1≤n≤3。1：票据纸打印模式，使用ESC/POS指令进行通信。2：模切标签打印模式，使用TSPL指令进行通信。3：无底纸打印模式，使用ESC/POS指令进行通信。  |  
打印机重启后，打印模式不会改变。**本指令仅NT320打印机支持。**
## **< GS>(E<pL><pH><fn> : <function 21>获取打印模式**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( E pL pH fn  |  
| HEX  | 1D 28 45 01 00 15  |  
| Decimal  | 29 40 69 1 0 21  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=1  |  
| pH  | 默认值=0  |  
| fn  | 默认值=21  |  
获取打印模式，打印机返回1字节打印模式。**本指令仅NT320打印机支持。**
返回数据：  
| n  | 打印模式  |  
| --- | --- |  
| 1  | 票据纸打印模式。  |  
| 2  | 模切标签打印模式。  |  
| 3  | 无底纸打印模式。  |  
## **< GS>(E<pL><pH><fn> : <function 22>获取打印模式**  
| [类型]  | [格式]  |  
| --- | --- |  
| ASCII  | GS ( E pL pH fn  |  
| HEX  | 1D 28 45 01 00 16  |  
| Decimal  | 29 40 69 1 0 22  |  
| [参数]  | [说明]  |  
| --- | --- |  
| pL  | 默认值=1  |  
| pH  | 默认值=0  |  
| fn  | 默认值=22  |  
获取打印模式，打印机返回1字节打印模式。**本指令仅NT320打印机支持。**
**接收返回数据格式：**  
| **内容**  | **Hex**  | **Dec**  | **数据长度**  |  
| --- | --- | --- | --- |  
| 首字节  | 37H  | 55  | 1 byte  |  
| 标识符  | 16H  | 22  | 1 byte  |  
| 打印模式  | n  | n  | 1 bytes  |  
| 结束符  | 00H  | 0  | 1 byte  |  
n返回数据：  
| n  | 打印模式  |  
| --- | --- |  
| 1  | 票据纸打印模式。  |  
| 2  | 模切标签打印模式。  |  
| 3  | 无底纸打印模式。  |  
  

  

上一篇：7、云打印机ESC/POS指令集
下一篇：9、云打印机WEB配置使用说明
