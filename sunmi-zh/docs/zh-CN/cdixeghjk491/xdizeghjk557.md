---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdizeghjk557
---

# JavaScript SDK概览
更新时间：2026-05-27 10:39:11
商米推出了一款 JavaScript 打印插件，极大地简化并加速了开发者对商米打印机的适配流程。第三方软件开发者只需在商米应用市场中搜索 “JS USDK”，即可获取该插件。通过运用 JavaScript API，开发者能够便捷地调用商米设备的内置打印机，轻松实现打印功能，有效提升开发效率，降低对接成本 。
# 1. 环境准备
进入系统桌面，打开应用市场 App，搜索“JS USDK”并进行安装。
![](https://cdn.sunmi.com/public/image/mgt-document/f1f6c55d346a4a4890c62dc1731fc3dd.png)
# 2. 接入SDK
商米JS-SDK可以直接使用npm进行安装使用，也可以通过UMD方式直接在html中引用后使用。
## 安装
### npm
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
npm i sunmi-js-sdk
  

```

### umd
[下载地址](https://sunmi-static.oss-cn-hangzhou.aliyuncs.com/h5/printer-sdk/dist/bundle.umd.js)
# 3. 参考Demo
  

![](https://cdn.sunmi.com/public/image/mgt-document/0f3c110154594a8e91eb65eb35d2433c.png)
可参考Demo的源码进行开发，点击下方链接跳转下载上图对应的Demo
[Demo地址](https://h5.sunmi.com/printer-sdk/demo.html)
# 4. 使用SDK
## 4.1 初始化
在使用JS-SDK的打印服务之前需要对设备的打印服务进行初始化，使用launchPrinterService进行服务的唤起。唤起是一个异步过程，用户可以通过回调在完成唤起后进行后续操作
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
let umd_sunmi = new SUNMI();
umd_sunmi.launchPrinterService().then((res) => {
        umd_sunmi.init();
        //Toast("success");
      });
```

## 4.2 打印机对象
打印机对象可使用以下API完成打印机的管理和不同打印场景内容的构造，具体的API包括：
  * QueryApi 打印机查询接口
  * CommandApi 指令集打印接口
  * LineApi 小票打印接口
  * CanvasApi 标签打印接口


上一篇：自助机打印文档
下一篇：JavaScript 打印热敏小票接口
