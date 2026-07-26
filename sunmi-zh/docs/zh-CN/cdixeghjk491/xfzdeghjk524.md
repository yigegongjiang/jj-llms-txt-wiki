---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzdeghjk524
---

# 自定义客显程序开发说明（仅适用于T1设备）
更新时间：2025-11-11 16:12:20
## 更新内容
##### 2018-01-15
  * 去掉常用方法、关键类说明；
  * 更新Demo源码；
  * 更新双屏应用开发调试的方法；


* * *
## 一、简介
T1双屏机器有三种组合：主机、主机+7寸副屏、主机+14寸副屏。主副屏都是运行SUNMI OS定制系统，通过商米已封装好的接口实现通信。主屏主要用来运行业务APP，例如：收银系统。副屏主要面向顾客显示结算、广告内容。
双屏通信原理：
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/5675451750701686.png)
从下到上依次为：
  * Driver：该层是最底层的通信协议，开发者无需关注。
  * Service：该层是商米封装的一个通信服务，开发者也无需关注。
  * SDK：该层是T1自带的副显程序使用的通信接口，自定义的客显程序调用的是该层的接口。
  * APP：指的是开发者自己的业务app和内置的副显程序。


开发者有两种方式实现副屏显示：
  * T1副屏系统内置了默认副屏显示APP，内置多个常用模板。开发者仅需通过商米的SDK向副屏发送正确格式的数据，即可实现副屏显示内容。
  * 开发自己副屏显示APP，需要自行处理数据的收发、内容显示等动作；


虽然商米封装的内置客显程序能满足绝大部分开发者的使用需求，但是考虑业务的多样性，商米秉持开放的原则，支持开发者自己定义副屏显示。
下文将对开发自定义双屏应用进行说明。
## 二、如何调试应用
由于主副屏是通过USB通信的，当主/副屏插入USB线时，主副屏的连接会断开，导致无法调试设备。商米提供的解决方案是：将开发电脑与将要调试的T1主机处于同一个局域网络环境下，通过网络对设备进行调试。
如何调试：
1、开启USB调试，及调试权限；
[操作说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrzeghjk557/)
2、通过网络对设备进行ADB调试；
[操作说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfmmeghjk546/)
## 三、如何开发自定义双屏应用
（强调：Sunmi OS是基于Android6.0定制，Android6.0+ 要求部分敏感权限需要动态申请）
### 1、初始化配置
以下将基于Demo讲解自定义双屏应用如何实现。
步骤1：
[下载DoubleAPP资源文件](https://ota.cdn.sunmi.com/DOC/resource/re_cn/DoubleScreenApp.zip)。（源码使用AndroidStudio编写）
步骤2：
参照Demo源码，直接在Android Studio的app module下的build.gradle文件中声明以下代码。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
compile 'com.sunmi:DS_Lib:1.0.9'  //商米提供的lib库，包含已封装好的接口
compile 'com.google.code.gson:gson:2.6.2'  //gson任意版本
  

```

步骤3：
在清单文件AndroidMainfest.xml的节点下注册广播（开发者可参考Demo源码，自行实现广播类）。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
"...广播类...">  //接收数据的广播
"com.sunmi.hcservice"/>
"com.sunmi.hcservice.status"/>
  

```

步骤4：
在适当的位置初始化SDK代码，可以参考Demo源码。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
private void initSdk() {
mDSKernel = DSKernel.newInstance();
mDSKernel.init(this, mIConnectionCallback);  //绑定服务的回调
mDSKernel.addReceiveCallback(mIReceiveCallback);  //双屏通信接收数据回调
}
  

```

### 2、开发双屏应用
参考Demo源码，实现双屏数据交互。具体方法可以自行定义，此处不做示例。
当主屏与副屏的业务逻辑都已经实现了，需要将主屏与副屏的业务代码合在一个apk里（因为应用市场的安装逻辑是将一个应用分别安装到主副屏，故双屏应用只需提交一个APK）。
## 四、发布应用前的准备
自定义双屏应用已经开发好了，那接下来就要将应用发布至商米应用市场，分发出去。在此之前，需要再做一件重要的步骤，就是让应用市场能够识别该应用是双屏应用。
在应用的AndroidManifest.xml中添加一行标识代码，这样，该应用在应用市场上将显示为双屏应用，安装时，应用市场会将该应用分别安装在主屏和副屏。 `"sunmi_dual" android:value="open"/>`
上一篇：T1双屏通信接口文档（仅适用于T1设备）
下一篇：ClientView 开源-副屏异显解决方案
