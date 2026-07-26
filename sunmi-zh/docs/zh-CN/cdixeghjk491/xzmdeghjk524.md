---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzmdeghjk524
---

# Cordova 钱箱控制接口

更新时间：2026-07-01 15:32:09

## 1\. 功能介绍

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
var cashDrawerApi = SunmiPrinterSDK.CashDrawerApi;
```

商米台式设备一般有钱箱端口，为了方便开发者在打印同时控制钱箱，打印库同时提供了钱箱控制方法

> open(callback?: (success: boolean, message: string) => void)
>
> isOpen(callback: (status: boolean | null) => void)

## 2\. 使用限制

> **!** 如果在不支持的设备上调用钱箱接口，接口将抛出异常提示

## 3\. 接口说明

### 3.1 开启钱箱

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
open(callback?: (success: boolean, message: string) => void)
```

打开连接在设备上的钱箱

**示例**

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
// 不关心结果
CashDrawerApi.open();

// 获取开启结果
CashDrawerApi.open(function(success, message) {
    if (success) {
        console.log("打开成功:", message);
    } else {
        console.error("打开失败:", message);
    }
});
```

### 3.2 获取钱箱状态

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
isOpen(callback: (status: boolean | null) => void)
```

获取连接在设备上的钱箱状态 **返回值**

-   true 钱箱状态打开

-   false 钱箱状态关闭


**示例**

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
CashDrawerApi.isOpen(function(status) {
    console.log(status ? "钱箱开启" : "钱箱关闭");
});
```

---

上一篇：Cordova 指令集打印接口
下一篇：1.了解一下对接流程
