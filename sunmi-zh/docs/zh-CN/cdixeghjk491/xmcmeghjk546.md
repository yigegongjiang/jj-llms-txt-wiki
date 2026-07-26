---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmcmeghjk546
---

# 22、云打印机Windows DLL接口
更新时间：2025-12-19 22:54:56
  

本功能仅支持以下机型和版本：
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
  

# **概述**
本文档主要介绍在 Windows 系统环境下，如何通过调用 DLL 动态库实现打印机票据打印功能。接口集合主要面向普通热敏小票的行输出打印方式，采用逐行渲染机制，每个接口负责绘制一行打印内容（如文本、条码、图片、分割线等，各元素独立成行）。该方式适用于 POS 小票打印及针式发票打印等场景。
  

# **DLL DEMO介绍**
Demo下载
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SMPrinter_v2.5_20251031.zip 
Python 调用范例
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) Windows动态库Python 调用范例.txt 
  

# **API接口说明**
## **初始化打印机**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
BOOL InitSMPrinter(int paper_height, int print_type)
```

paper_height：初始化纸张的高度（单位：像素）
print_type：打印机类型 1：商米内置打印机 2：商米外置打印机
  

## **关闭打印机**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void CloseSMPrinter();
```

PS：关闭后需要重新 InitSMPrinter 才能打印
  

## **打印文本内容**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef struct {
    const char* text;          // 文本内容(UTF-8)
    double x;                  // X坐标(像素)
    double y;                  // Y坐标(像素)
    const char* font_name;     // 字体名称
    double font_size;          // 字号
    double scale;              // 缩放比例
    int rotation;              // 旋转角度(0-360)
    int halign;                // 水平对齐 0-左 1-中 2-右
    int valign;                // 垂直对齐 0-上 1-中 2-下
} TextParams;

BOOL AddText(const TextParams* params);
```

初始化内置打印机，可选：初始化纸张的高度（单位：像素）
  

## **打印条码/二维码**
目前支持：QR Code、Code 128、Code 39、EAN (European Article Number)
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//条码类型
enum BarcodeType {
    BARCODE_TYPE_QRCODE,   // QR CODE
    BARCODE_TYPE_CODE128,  // Code 128
    BARCODE_TYPE_CODE39,   // Code 39
    BARCODE_TYPE_EAN13,    // EAN-13
};

typedef struct {
	// 编码参数
	const char* data;       // 编码数据
	int barcode_type;       // 条码类型
	int ecc_level;          // 纠错等级
	int version;            // 二维码版本（0=自动）
	int border_width;       // 边界宽度
	BOOL show_text;         // 是否显示可读文本

	// 绘制参数
	double pos_x;           // X坐标（像素）
	double pos_y;           // Y坐标（像素）
	double width;           // 目标宽度（像素）
	double height;          // 目标高度（像素）
	double rotation;        // 旋转角度（度）
	double scale;           // 基础缩放系数

	// 高级选项
	int output_options;     // 输出选项 默认 OUT_BUFFER_INTERMEDIATE
	int input_mode;         // 输入模式 默认 DATA_MODE
} BarcodeParams;

BOOL AddBarcode(const BarcodeParams* params);
```

**二维码配置-容量速查表（字母数字模式）**  
| version  | Level L  | Level M  | Level Q  | Level H  |  
| --- | --- | --- | --- | --- |  
| 1  | 41  | 34  | 27  | 17  |  
| 4  | 154  | 122  | 86  | 64  |  
| 10  | 458  | 361  | 256  | 190  |  
| 20  | 1480  | 1172  | 832  | 613  |  
| 40  | 4296  | 3391  | 2420  | 1773  |  
**根据传入的数据长度选择对应的参数：**
不同版本对应纠错等级参数ecc_level，L = 1；M = 2；Q = 3； H = 4；
  

## **打印图片**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef struct {
    const char* bitmapPath;// 图片路径
    double pos_x;           // X坐标（像素）
    double pos_y;           // Y坐标（像素）
    double width;           // 目标宽度（像素）
    double height;          // 目标高度（像素）
    double rotation;        // 旋转角度（度）
    int output_options;     // 输出选项
    int input_mode;         // 输入模式
} BitmapParams;

BOOL AddPNGImage(const BitmapParams* params);
```

  

## **打印分割线或空行**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
enum DividingLine {
	EMPTY,			// 空行
	DOTTED,			// 虚线
	SOLID			// 实线
};

void AddDividingLine(DividingLine style, int offset, int dash_on_ratio = 5, int dash_off_ratio = 2);
```

注意：若选择 DOTTED 类型，可设置第三、四两个参数控制输出的效果
dash_on_ratio 实线长度
dash_off_ratio 空格长度
  

## **打印内容并切纸**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
BOOL PrintAndCutPaper();
```

  

## **立即切纸**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
typedef enum {
    FULL_CUT,      // 全切
    PARTIAL_CUT,   // 半切
    FEED_AND_CUT   // 走纸后切
} CutMode;
int CutPaper(CutMode mode);
```

  

## **打开钱箱**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
BOOL OpenCashBox();
```

  

## **发送原始指令打印**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
HRESULT SendCommand(BYTE* Data, int DataLength);
```

可通过该接口直接发送 ESC/POS、TSPL等指令给打印机
  

## **获取最新的打印任务编号（PS：只支持外置打印机）**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DWORD GetLatestPrintTaskNumber();
```

  

## **获取打印任务状态（PS：只支持外置打印机）**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 打印任务状态枚举
typedef enum {
    PRINT_TASK_STATUS_UNKNOWN = 0,      // 未知（例如任务编号不存在）
    PRINT_TASK_STATUS_PENDING = 1,      // 尚未打印
    PRINT_TASK_STATUS_PRINTING = 2,     // 正在打印
    PRINT_TASK_STATUS_COMPLETED = 3     // 打印完成
} PrintTaskStatus;

PrintTaskStatus QueryPrintTaskStatus(DWORD taskNumber)
```

PS：需要 GetLatestPrintTaskNumber 函数的返回值作为参数
  

## **清除指定的打印任务（PS：只支持外置打印机）**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 清除任务状态枚举
typedef enum {
    CLEAR_TASK_STATUS_UNKNOWN = 0,      // 未知错误（例如任务编号不存在）
    CLEAR_TASK_STATUS_SUCCESS = 1       // 清除完成
} ClearTaskStatus;
ClearTaskStatus ClearPrintTaskBuffer(DWORD taskNumber)
```

  

## **清除所有打印任务缓冲区（PS：只支持外置打印机）**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// 清除任务状态枚举
typedef enum {
    CLEAR_TASK_STATUS_UNKNOWN = 0,      // 未知错误（例如任务编号不存在）
    CLEAR_TASK_STATUS_SUCCESS = 1       // 清除完成
} ClearTaskStatus;
ClearTaskStatus ClearAllPrintTaskBuffers()
```

  

  

  

上一篇：21、标签打印机Windows驱动
下一篇：23、云打印机局域网HTTP打印
