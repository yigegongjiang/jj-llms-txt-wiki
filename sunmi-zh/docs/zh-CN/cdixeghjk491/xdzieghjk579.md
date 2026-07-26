---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdzieghjk579
---

# 指令集打印接口
更新时间：2025-09-24 16:50:20
## 一、功能介绍
* * *
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public CommandApi commandApi()
  

```

指令透传接口集合，用于已经通过指令构建好打印内容的开发者直接使用的接口，目前提供两种指令集ESC/POS和TSPL指令集（会根据需求持续扩展）
> void sendEscCommand(byte[] esc)
> void sendTsplCommand(byte[] tspl)
## 二、使用限制
商米内置打印机可以支持所有指令集合，其他种类打印机暂时只支持ESC/POS指令集
## 三、接口说明
### 1. 发送ESC命令
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void sendEscCommand(byte[] esc)
  

```

向打印机发送ESC指令集，ESC指令集通常用于普通小票打印
常用的ESC指令集可以参考：[ESC指令集](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/commands.html)
使用此接口除执行ESC命令外也可直接发送打印文本的十六进制数据
商米打印机默认使用的字符集为**GB18030（即传输的内容为GB18030编码）** ，非商米打印机根据厂商配置决定
若需要打印指定语言的文本内容，可根据ESC标准指令集设置打印机的字符集将打印机可接收的编码切换为需要的
例如需要打印CP437可在发送打印内容前设置
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
HEX:1C 2E     —— 单字节编码
HEX:1B 74 00  —— CP437代码页设置
  

```

对于商米打印机支持全语种文本打印的通用方式可设置(非商米打印机不支持）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
HEX:1C 26     —— 多字节编码
HEX:1C 43 FF  —— UTF8编码设置
  

```

  * **示例**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
byte[] esc = new byte[]{0x1B，0x61, 0x01};
printer.commandApi().sendEscCommand(esc);
byte[] content = "测试打印内容<br></br>".getBytes("gb18030");
printer.commandApi().sendEscCommand(content);
  

```

### 2. 发送TSPL命令
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void sendTsplCommand(byte[] tspl)
  

```

向打印机发送TSPL指令集，TSPL指令集通常用于绘制标签票据内容
TSPL命令适用于标签打印机，所以使用前需确认打印机是否支持打印标签
对于商米打印机V2s、V2s_Plus等设备支持普通/标签功能切换，使用此命令时请确认已经切换到标签功能模式
  * **示例**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
byte[] content = "SIZE 100 100".getBytes(StandardCharsets.US_ASCII);
printer.commandApi().sendTsplCommand(content);
  

```

上一篇：打印机查询接口
下一篇：钱箱控制接口
