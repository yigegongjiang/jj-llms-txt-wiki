---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmxqeghjk513
---

# 3、钱箱驱动器Android驱动说明
更新时间：2026-07-13 12:45:57
# 一、概述
本⽂档主要介绍如何通过商米提供的SDK快速使用钱箱驱动器
# 二、SDK Demo介绍
## 1、资源下载
Demo Apk 下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) cashboxdemo-release.apk 
Demo 源码下载：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) demo.zip 
## 2、Demo使用介绍
### 2.1 获取钱箱适配器
根据当前钱箱驱动方式选择不同的通讯方式，如果需要通过蓝牙驱动打开钱箱，请选择【获取蓝牙钱箱适配器】
  

![](https://cdn.sunmi.com/public/image/mgt-document/f818f484b74e4804853f40479024f5cd.jpg)
### 2.2 使用蓝牙钱箱适配器
使用蓝牙连接需要动态申请权限，示例代码中基于Android13版本设备开发，所以在申请权限时包括：
android.permission.ACCESS_FINE_LOCATION、android.permission.BLUETOOTH_CONNECT、android.permission.BLUETOOTH_SCAN
  

![](https://cdn.sunmi.com/public/image/mgt-document/3943961a89e3404ba4f569574ab8b8cf.png)
在授予相应权限后，开始搜索蓝牙钱箱适配器设备，选择指定的适配器名称（名称以CashDrawer_xxxxxx开头）。
连接成功以后，提示【CashDrawer_xxxxxxx】toast信息。
  

![](https://cdn.sunmi.com/public/image/mgt-document/89d6d054ddfa4e8481ee8a299c5ef8da.jpg)
首次使用任意钱箱功能，将会弹出配对PIN输入窗口。**默认PIN码是设备名称最后的6位数字。**
如果设备在上电5分钟内未进行配对绑定，设备将退出配对模式。
如果需要再次配对，请重新拔插USB进行上电操作，并在5分钟内完成绑定。
  

![](https://cdn.sunmi.com/public/image/mgt-document/abe38e2d1c754f0b89f8596eb949c648.jpg)
### 2.3 使用USB钱箱适配器
使用功能前需要授予权限，这之后即可使用钱箱功能
一台设备上的USB设备不区分多个，所以获取USB钱箱适配器将直接返回发现的唯一适配器设备
  

![](https://cdn.sunmi.com/public/image/mgt-document/5791bde87b4a41c4b548c33c7a27abff.png)
# 三、SDK开发说明
## 1、SDK快速入门
### 1.1 AAR下载
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) cashbox-1.0.4-release.aar 
### 1.2 将下载目录中的AAR文件导入到工程的libs目录下，并在gradle配置文件中添加aar的使用
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
android {
  ...
}
dependencies {
    implementation fileTree(dir: 'libs', include: ['*.jar','*.aar'])
    ...
}
  

```

Demo工程中的gradle使用版本为7.4，gradle插件版本为7.3，如果导入SDK项目编译失败可以考虑升级gradle版本
库版本更新后如果替换旧库时最好将IDE的缓存invalidate，防止编译打包时仍使用旧库
### 1.3 根据使用需要申请相应的运行时权限
## 2、SDK详细说明
### 2.1 适配器管理类SunmiCashBoxManager  
| 方法  | 说明  |  
| --- | --- |  
| getUsbAdapter(Context)  | 获取USB适配器  |  
| getBleAdapter(Context , String , AdapterCallback )  | 获取指定名称的蓝牙适配器  |  
| getBleAdapter(Context , AdapterCallback)  | 搜索并获取蓝牙适配器  |  
  * **获取USB适配器**
获取控制钱箱的适配器，此适配器通过USB口连接，将直接返回对应的适配器对象


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
CashBoxAdapter getUsbAdapter(Context context)  
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
  * **获取蓝牙适配器**
获取指定名称的蓝牙适配器，此适配器通过蓝牙连接，将异步返回适配器对象


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getBleAdapter(Context context, String name, AdapterCallback callback)
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| name  | String  | 指定获取的蓝牙适配器名称  |  
| callback  | AdapterCallback  | 适配器回调方法  |  
  * **获取蓝牙适配器**
获取控制钱箱的适配器，此适配器通过蓝牙连接，将异步返回所有可搜索到的蓝牙适配器对象


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getBleAdapter(Context context, AdapterCallback callback)
  

```
  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  | 会话上下文  |  
| callback  | AdapterCallback  | 适配器回调方法  |  
### 2.2 适配器控制类CashBoxAdapter  
| 方法  | 说明  |  
| --- | --- |  
| String getAdapterName()  | 获取钱箱适配器名称  |  
| void getSerialNo(ResultCallback)  | 获取钱箱适配器SN  |  
| void getVersion(ResultCallback)  | 获取钱箱适配器版本  |  
| void openCashBox(long , long , ResultCallback)  | 控制打开钱箱  |  
| void cashBoxStatus(StatusCallback )  | 查询钱箱开启状态  |  
  * **获取钱箱适配器名称**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
String getAdapterName()
  

```

返回当前控制的钱箱适配器名称
  * **获取钱箱适配器SN**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getSerialNo(ResultCallback callback)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| callback  | ResultCallback  | 通过onResult异步返回String类型的SN  |  
  * **获取钱箱适配器版本**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void getVersion(ResultCallback callback)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| callback  | ResultCallback  | 通过onResult异步返回String类型的版本号  |  
  * **控制打开钱箱**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void openCashBox(long openTime, long closeTime, ResultCallback callback)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| openTime  | long  | 钱箱驱动开启时间，范围0-510，单位ms  |  
| closeTime  | long  | 钱箱驱动关断时间，范围0-510，单位ms  |  
| callback  | ResultCallback  | 通过onError返回使用接口中发生的错误  |  
根据使用的不同规格钱箱配置不同的开启关断时间（参考钱箱规格书)
  * **查询钱箱开启状态**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void cashBoxStatus(StatusCallback callback)
  

```


  
| 参数  | 类型  | 说明  |  
| --- | --- | --- |  
| callback  | StatusCallback  | 通过onStatus返回钱箱读取状态   
  
返回值为0x12：表示RJ12 Pin3状态为低   
  
返回值为0x16：表示RJ12 Pin3状态为高  |  
根据使用的不同规格钱箱，可能Pin3状态对应的钱箱开启状态不同
上一篇：2、钱箱驱动器Windows驱动说明
下一篇：4、钱箱驱动器iOS驱动说明
