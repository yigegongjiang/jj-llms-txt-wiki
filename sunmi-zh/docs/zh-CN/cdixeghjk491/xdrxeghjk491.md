---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdrxeghjk491
---

# LCD客显控制接口
更新时间：2025-09-24 16:50:46
# 一、功能介绍
* * *
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public LcdApi lcdApi()
  

```

LCD液晶显示接口集合，为了方便开发者在打印同时控制部分具有液晶显示屏的商米设备，打印库同时提供了控制接口方法，其中包括如下API：
> void config(Command command)   
>   
> void showText(String text, int size, boolean fill)   
>   
> void showTexts(String[] text, int[] align)   
>   
> void showBitmap(Bitmap bitmap)
> void showDigital(String digital)
# 二、使用限制
因为客显液晶显示屏只在部分指定场景设备上存在，同时不同的客显设备液晶显示屏的类型也不同，所以在使用接口前建议用户根据当前使用的客显屏类型来决定控制客显屏的接口
目前支持客显接口的设备有：T1mini、T2mini、D3mini等
# 三、接口说明
## 1. 控制客显屏的状态
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void config(Command command)
  

```

所有类型的客显屏均支持的初始化方法，只有唤醒后才可以使用
**参数Command说明**  
| 枚举内容  | 枚举说明  |  
| --- | --- |  
| INIT  | 初始化LCD  |  
| WAKE  | 唤醒LCD  |  
| SLEEP  | 休眠LCD  |  
| CLEAR  | 清除LCD屏显内容  |  
## 2. 128x40Dots Lcd屏接口集合
支持此类屏的设备有商米T1mini、T2mini
### 显示指定大小的文本内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void showText(String text, int size, boolean fill)
  

```

**参数说明**  
| 参数  | 类型  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| text  | String  | 需要显示的内容   
  
可显示的内容长度为128/size  | 无  |  
| size  | int  | 指定显示字体大小   
  
可设置范围6~40  | 32  |  
| fill  | boolean  | 是否拉伸显示内容高度整个屏幕  | false  |  
### 显示多行文本内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void showTexts(String[] text, int[] align)
  

```

**参数说明**  
| 参数  | 类型  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| text  | array  | 打印每行文本的内容   
  
当内容为空时表示此行无内容  | 无  |  
| align  | array  | 打印文本内容的行间距比例  | 无  |  
### 显示位图图像
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void showBitmap(Bitmap bitmap）
  

```

**参数说明**  
| 参数  | 类型  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| bitmap  | Bitmap  | 显示的位图内容   
  
**位图可显示128x40像素内容**  | 无  |  
## 3. 段码屏接口集合
支持此类屏的设备有商米D3mini、D3pro配件
### 显示价格内容
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void showDigital(String digital)
  

```

此接口用来显示价格内容，内容将呈现在可支持7位0-9数字和A-Z字符的段码液晶屏上，可在任何字符间插入”.”作为价格分割符
**参数说明**  
| 参数  | 类型  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| digital  | String  | 显示价格内容，字符串可支持7位0-9数字和A-Z字符以及在7位字符中插入的”.”  | 无  |  
上一篇：钱箱控制接口
下一篇：一体机打印文档
