---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzaqeghjk513
---

# Uniapp 指令集打印接口
更新时间：2026-03-23 12:11:54
# Uniapp 指令集打印接口
本文档介绍在 **UniApp** / **UniApp-x** 环境下，通过 `sunmi-printersdk` 插件的 **CommandApi** 直接向商米设备内置打印机发送 **ESC/POS** 或 **TSPL** 原始指令，实现完全自定义的指令集打印能力。
* * *
## 1. 功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CommandApi } from "@/uni_modules/sunmi-printersdk";
  

```

**CommandApi** 指令透传接口集合，用于已经通过指令构建好打印内容的开发者直接使用的接口，目前提供两种指令集ESC/POS和TSPL指令集（会根据需求持续扩展）  
| 方法  | 说明  |  
| --- | --- |  
| `sendEscCommand(data)`  | 发送 ESC/POS 指令。`data` 可为 `string` 或 `Array<Byte>`（UTS 中）。  |  
| `sendTsplCommand(data)`  | 发送 TSPL 指令。`data` 可为 `string` 或 `Array<Byte>`。  |  
* * *
## 2. 使用限制
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
商米内置打印机可以支持所有指令集合，其他种类打印机暂时只支持 ESC/POS 指令集。
  

```

* * *
## 3. 接口说明
### 3.1 发送 ESC/POS 指令
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sendEscCommand(data: string | Array<Byte>)**
  

```

bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
向打印机发送ESC指令集，ESC指令集通常用于普通小票打印
常用的ESC指令集可以参考：ESC指令集(https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/commands.html)
  

```

bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
使用此接口除执行ESC命令外也可直接发送打印文本的十六进制数据
商米打印机默认使用的字符集为GB18030（即传输的内容为GB18030编码），非商米打印机根据厂商配置决定
若需要打印指定语言的文本内容，可根据ESC标准指令集设置打印机的字符集将打印机可接收的编码切换为需要的
例如需要打印CP437可在发送打印内容前设置
  

```

示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CommandApi } from "@/uni_modules/sunmi-printersdk";

// 字符串形式（转义字符）
CommandApi.sendEscCommand("\x1B\x40");           // 初始化打印机
CommandApi.sendEscCommand("\x1B\x61\x01");       // 居中
CommandApi.sendEscCommand("Hello ESC<br></br>");
CommandApi.sendEscCommand("\x1D\x56\x00");      // 切纸

// 若在 UTS 中构造 Byte 数组，也可传入 Array<Byte>
// CommandApi.sendEscCommand(byteArray);
  

```

* * *
### 3.2 发送 TSPL 指令
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sendTsplCommand(data: string | Array<Byte>)**
  

```

向打印机发送TSPL指令集，TSPL指令集通常用于绘制标签票据内容。`data` 可为字符串（UTF-8 编码后发送）或 UTS 中的 `Array<Byte>`。  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string | Array<Byte>  | TSPL 指令文本或字节数组。字符串会按 UTF-8 转为字节后发送。  |  
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
TSPL命令适用于标签打印机，所以使用前需确认打印机是否支持打印标签
对于商米打印机V2s、V2s_Plus等设备支持普通/标签功能切换，使用此命令时请确认已经切换到标签功能模式
  

```

示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CommandApi } from "@/uni_modules/sunmi-printersdk";

const tspl = [
  "SIZE 40 mm, 30 mm",
  "GAP 2 mm, 0 mm",
  "DIRECTION 1",
  "CLS",
  "TEXT 10,10,\"3\",0,1,1,\"Label\"",
  "PRINT 1,1"
].join("<br></br>");

CommandApi.sendTsplCommand(tspl);
  

```

上一篇：Uniapp 打印标签小票接口
下一篇：Uniapp 钱箱控制接口
