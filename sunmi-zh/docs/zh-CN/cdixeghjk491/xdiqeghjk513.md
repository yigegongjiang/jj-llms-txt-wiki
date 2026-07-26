---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdiqeghjk513
---

# PSAM、ETC、M112读卡开发
更新时间：2025-11-20 11:49:58
# **1. 概述**
非金融SunmiPaySDK是基于SP（secure processor）或MCU（Microcontroller Unit）封装的SDK。
SDK通过Android AIDL的方式对客户端提供接口，主要包含：卡操作模块、ETC模块、RFID模块等。客户端集成SKD后可快速调用商米金融服务的接口，实现各种金融功能。
## **1.1 功能介绍**
商米SunmiPaySDK各个模块功能如下：
  * **卡模块** ：提供检卡、卡片APDU交互、卡片下电等接口，目前仅支持SAM卡
  * **ETC模块** ：提供搜索OBU、ETC扣费等接口
  * **RFID模块** ：提供M112读卡芯片的数据交互接口


## **1.2 SDK文件说明**  
| 名称  | 功能  | 备注  |  
| --- | --- | --- |  
| PayLib-release-x.x.x.aar  | AIDL接口编译后的aar包  | 客户端App集成此包后可调用SDK的各个接口  |  
| PayLib-release-x.x.x-sources.jar  | 提供PayLib-release-x.x.x.aar中类的源码  | 客户端可以不集成  |  
| SUNMI PAY SDK V2 开发文档_x.x.x.docx  | 中文版SDK接口文档，提供各个接口的功能说明  |   
 |  
| SUNMI PAY SDK V2 Development Document_x.x.x.docx  | 英文版SDK接口文档，提供各个接口的功能说明  |   
 |  
| SunmiPaySdkTestDemo.rar  | SDKTestDemo源码，提供各SDK接口的使用示例  |   
 |  
| SunmiSDKTestDemo_x.x.x_debug.apk  | SDK源码编译后生成的apk  |   
 |  
# **2. 环境信息**  
| 系统环境  | 平台  | 编译环境  |  
| --- | --- | --- |  
| Android 6.0及以上  | arm64， arm32  | Android studio  |  
# 3. SDK整包获取链接
说明：在sdk整包中包含aar包、接口文档、演示demo和demo源码。
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiPaySDKV2_v1.0.60_NP_20230908_01.rar 
## **3.1 导入SDK包**
在Android Studio项目中将PayLib-release-x.x.x.aar放在libs目录中。
## **3.2 配置build.gradle文件**
在build.gradle中文件中添加以下代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
repositories {
  flatDir {
    dirs 'libs'
  }
}
dependencies {
  ......
  compile(name: 'PayLib-release-x.x.x', ext: 'aar')
}
  

```

## **3.3 初始化SDK**
参考以下方法绑定金融服务：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
/** bind PaySDK service */
public void bindPaySDKService() {
    final SunmiPayKernel payKernel = SunmiPayKernel.getInstance();
    payKernel.initPaySDK(this, new SunmiPayKernel.ConnectCallback() {
        @Override
        public void onConnectPaySDK() {
            LogUtil.e(Constant.TAG, "onConnectPaySDK...");
            readCardOptV2 = payKernel.mReadCardOptV2;
            etcOptV2 = payKernel.mETCOptV2;
            rfidOptV2 = payKernel.mRFIDOptV2;
            connectPaySDK = true;
        }

        @Override
        public void onDisconnectPaySDK() {
            LogUtil.e(Constant.TAG, "onDisconnectPaySDK...");
            connectPaySDK = false;
            readCardOptV2 = null;
            etcOptV2 = null;
            rfidOptV2 = null;
            Utility.showToast(R.string.connect_fail);
        }
    });
}
  

```

# 4.注意：
1.Android 11.0+系统上绑定SPHS时需要增加以下权限声明，否则会出现报错：“bind PayHardwareService failed: service not found”：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
<uses-permission 

  android:name="android.permission.QUERY_ALL_PACKAGES"

  tools:ignore="QueryAllPackagesPermission" />
  

```

上一篇：磁条卡服务说明
下一篇：RFID SDK集成说明
