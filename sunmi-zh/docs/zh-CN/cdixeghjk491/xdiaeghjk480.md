---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdiaeghjk480
---

# 商米设备如何操作钱箱
更新时间：2025-10-16 12:43:08
# 关于商米设备操作钱箱
商米部分设备如T1可以连接外部钱箱，App可以通过以下两种方式打开钱箱。
一、通过AIDL文件中封装的方法打开钱箱。
二、通过指令打开钱箱。
## 一、通过AIDL文件中的方法打开钱箱
1.下载相关 [资源文件](https://ota.cdn.sunmi.com/DOC/doczip/%E9%92%B1%E7%AE%B1.zip) ，在项目中新建如下层级的package,将源文件中的AIDL文件放入package中。
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/4083858682747532.jpg)
2.操作钱箱的只有打开钱箱和获取钱箱打开次数这两个方法如下图所示，两个方法在IWoyouService.aidl文件中，开发者可以在自己的代码中调用。
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/4826301609416146.png)
## 二、通过ES/POS指令的方式
开发者可以通过两种方式向服务发送ES/POS指令
1.通过虚拟蓝牙与服务建立连接，发送ES/POS指令，可参照 [打印机驱动](https://developer.sunmi.com/docs/zh-CN/xeghjk491/ciqeghjk513) 文档中的通过蓝牙连接调用发送指令打开钱箱，打开钱箱的ES/POS指令如下：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
byte[] aa = new byte[5];
aa[0] = 0x10;
aa[1] = 0x14;
aa[2] = 0x00;
aa[3] = 0x00;
aa[4] = 0x00;
  

```

2.通过AIDL方法中封装的sendRAWData(bytes [] ,callback )方法发送指令，同样需要用到方式一中的资源文件，可参照 [打印机驱动](https://developer.sunmi.com/docs/zh-CN/xeghjk491/ciqeghjk513)文档中的AIDL打印方式：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
byte[] aa = new byte[5];

aa[0] = 0x10;
aa[1] = 0x14;
aa[2] = 0x00;
aa[3] = 0x00;
aa[4] = 0x00;

try {
    woyouService.sendRAWData(aa, callback);
} catch (RemoteException e1) {
    e1.printStackTrace();
}
  

```

上一篇：RFID uniapp插件使用说明
下一篇：1、钱箱驱动器产品说明
