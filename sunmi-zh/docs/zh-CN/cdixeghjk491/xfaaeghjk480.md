---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfaaeghjk480
---

# JavaScript 指令集打印接口
更新时间：2025-09-24 17:58:47
## 一、功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public CommandApi commandApi()
  

```

指令透传接口集合，用于已经通过指令构建好打印内容的开发者直接使用的接口，目前提供两种指令集ESC/POS和TSPL指令集（会根据需求持续扩展）
> sendEscCommand: (esc: string) => Promise
> sendTsplCommand: (tspl: string) => Promise
## 二、使用限制
商米内置打印机可以支持所有指令集合，其他种类打印机暂时只支持ESC/POS指令集
## 三、接口说明
### 1. 发送ESC命令
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sendEscCommand: (esc: string) => Promise
  

```

向打印机发送ESC指令集，ESC指令集通常用于普通小票打印
常用的ESC指令集可以参考：[ESC指令集](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/commands.html)
  * 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
function stringToHexByteArray(str) {
    return Array.from(str, char => '0x' + char.charCodeAt(0).toString(16).padStart(2, '0'));
}

// 示例
const inputString = "Hello!";
const rv = stringToHexByteArray(inputString);

const hexArray = Array.from(rv, (byte) => {
        return byte.toString(16).padStart(2, "0");
      });

umd_sunmi.printer.commandApi.sendEscCommand(hexArray).then((res) => {
        Toast(res);
      });
  

```



### 2. 发送TSPL命令
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sendTsplCommand: (tspl: string) => Promise
  

```

向打印机发送TSPL指令集，TSPL指令集通常用于绘制标签票据内容
TSPL命令适用于标签打印机，所以使用前需确认打印机是否支持打印标签
对于商米打印机V2s、V2s_Plus等设备支持普通/标签功能切换，使用此命令时请确认已经切换到标签功能模式
  * 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
const content =
        "SIZE 40 mm, 30 mm\r<br></br>" +
        "GAP 3 mm, 0 mm\r<br></br>" +
        "DIRECTION 1,0\r<br></br>" +
        "REFERENCE 0,0\r<br></br>" +
        "SET TEAR 1\r<br></br>" +
        "CLS\r<br></br>" +
        'TEXT 20,10, "test",0,1,1,"first line!"\r<br></br>' +
        'TEXT 20,51, "test",0,1,1,"second line!"\r<br></br>' +
        'TEXT 20,92, "test",0,1,1,"third line!"\r<br></br>' +
        'TEXT 20,133, "test",0,1,1,"firth line!"\r<br></br>' +
        'QRCODE 40,165,L, 4, A, 0,"abcdefg"\r<br></br>' +
        "PRINT 1,2\r<br></br>";
      umd_sunmi.printer.commandApi.sendTsplCommand(content).then((res) => {
        Toast(JSON.stringify(res));
      });
  

```



上一篇：JavaScrip 打印标签小票接口
下一篇：JavaScript 打印机查询接口
