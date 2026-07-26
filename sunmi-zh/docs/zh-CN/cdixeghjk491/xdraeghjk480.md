---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdraeghjk480
---

# 钱箱控制接口
更新时间：2025-09-24 16:50:35
## 一、功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public CashDrawerApi cashDrawerApi()
  

```

商米台式设备一般有钱箱端口，为了方便开发者在打印同时控制钱箱，打印库同时提供了钱箱控制方法，其中包括如下API：
> void open(PrintResult resultListener)
> boolean isOpen()
## 二、使用限制
如果在不支持的设备上调用钱箱接口，接口将抛出异常提示
## 三、接口说明
### 1. 开启钱箱
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void open(PrintResult resultListener)
  

```

打开连接在设备上的钱箱
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 //不关心结果
printer.cashDrawerApi().open(null);
 //需要获取开启结果
printer.cashDrawerApi().open(new PrintResult() {
            @Override
            public void onResult(int resultCode, String message)  {
                
            }
        });
  

```

### 2. 获取钱箱状态
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
boolean isOpen()
  

```

获取连接在设备上的钱箱状态
**返回值**
true 钱箱状态打开
false 钱箱状态关闭
**示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
boolean result = printer.cashDrawerApi().isOpen();
Toast.makeText(this, result?"钱箱开启":"钱箱关闭", Toast.LENGTH_LONG).show();
  

```

上一篇：指令集打印接口
下一篇：LCD客显控制接口
