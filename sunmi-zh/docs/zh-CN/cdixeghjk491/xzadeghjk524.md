---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzadeghjk524
---

# Uniapp 钱箱控制接口
更新时间：2026-03-23 12:16:13
# Uniapp 指令集打印接口
本文档介绍在 **UniApp** / **UniApp-x** 环境下，通过 `sunmi-printersdk` 插件的CashDrawerApi接口控制钱箱。
* * *
## 1. 功能介绍
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CashDrawerApi } from "@/uni_modules/sunmi-printersdk"
  

```

商米台式设备一般有钱箱端口，为了方便开发者在打印同时控制钱箱，打印库同时提供了钱箱控制方法
>openCashDrawer(callback) : void // 异步回调，可获知打开成功或失败  
>isCashDrawerOpen() : boolean
## 2. 使用限制
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
在不支持的设备上调用 openCashDrawer 时，会通过回调的 message 返回失败原因（如「该设备不支持钱箱功能」），不再抛异常
  

```

* * *
## 3. 接口说明
### 3.1 开启钱箱
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 openCashDrawer(callback: (success: boolean, message: string) => void)
  

```

打开连接在设备上的钱箱，通过异步回调返回结果。
**参数：**
  * `callback`：回调函数
    * `success`：是否打开成功
    * `message`：说明信息（成功时为「打开成功」，失败时为具体原因，如「该设备不支持钱箱功能」）


示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import { CashDrawerApi } from "@/uni_modules/sunmi-printersdk";
CashDrawerApi.openCashDrawer((success, message) => {
	if (success) {
		uni.showToast({ title: '打开钱箱成功', icon: 'success' });
	} else {
		uni.showToast({ title: message, icon: 'none' });
	}
});
  

```

* * *
### 3.2 获取钱箱状态
获取连接在设备上的钱箱状态
### 返回值
true 钱箱状态打开  
false 钱箱状态关闭
示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
const isOpen = CashDrawerApi.isCashDrawerOpen()
this.cashStatusText = isOpen ? '钱箱状态：已打开' : '钱箱状态：已关闭'
  

```

上一篇：Uniapp 指令集打印接口
下一篇：Cordova SDK 概览
