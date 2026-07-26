---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdifeghjk535
---

# 一体机SDK版本说明
更新时间：2025-09-24 16:58:37
# 声明依赖项
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
dependencies {
    implementation 'com.sunmi:printerlibrary:1.0.24'
}
  

```

注意当Android系统版本在11或以上时系统会限制应用可见性导致sdk无法使用，所以需要在AndroidManifest.xml中添加如下内容： 
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
<manifest package="开发者应用包名">    
  <queries>        
  	<package android:name="woyou.aidlservice.jiuiv5" />        
  </queries>    
  ...
</manifest>    
  

```

# 版本 1.0
## 版本 1.0.24
2025年5月6日
更新打印库的配置
## 版本 1.0.23
2024年4月1日
修复MIX机型上不支持开启钱箱接口的问题
## 版本 1.0.22
2023年8月4日
  1. 支持D3MINI机型
  2. 新增在D3MINI设备上支持控制段码屏的接口:sendLCDDigital


## 版本 1.0.21
2023年3月31日
  1. 支持V3 MIX机型
  2. autoOutPaper和定位接口可在台式设备上使用


## 版本 1.0.20
2023年2月1日
  1. 支持X30TR机型


## 版本 1.0.19
2022年6月8日
  1. 支持QBao H1机型


上一篇：一体机打印文档
下一篇：自助机打印文档
