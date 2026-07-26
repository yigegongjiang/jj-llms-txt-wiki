---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmaqeghjk513
---

# NFC相关SDK说明
更新时间：2026-03-20 19:58:42
# **1. 功能概述**
商米提供对例如FLEX 3等设备屏下NFC和外接NFC模块的切换控制以及屏下NFC水印控制等
# **2. 快速开始**
与StatusLightService使用相同SDK及相同的引入方式
# **3. 接口说明**
## **3.1 初始化**
**在Activity onCreate生命周期中初始化，并且打开设备**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NfcManager.init(this) { success ->
    ...
}
```

## **3.2 注册NFC模块监听的变化**
注册监听NFC模块变化的回调，以实时获取当前设备可用的NFC模块列表
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NfcControlManager.registerNfcListener(object : INfcListener.Stub() {
            override fun onNfcListChanged(nfcList: MutableList<Nfc>?) {
                
            }
        })
```

## **3.3 切换指定的NFC模块**
通过使用获取到NFC列表中NFC模块的SN，指定切换到对应的NFC模块
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NfcControlManager.switchNfc(nfcList!![0].sn)
```

## **3.4 设置当前NFC模块的水印**
主动设置当前NFC模块的水印透明度，范围在0-100
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NfcControlManager.setNfcWaterMarkAlpha(100)
```

## **3.5 结束注册的NFC模块监听器**
当不需要使用并关注NFC模块时可结束注册
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NfcControlManager.unregisterNfcListener(‘注册的匿名类’)
```

## **3. 6 销毁**
也可以直接对SDK销毁结束操作
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
NfcControlManager.destroy(this)
```

上一篇：电子秤 SDK 开发
下一篇：磁条卡服务说明
