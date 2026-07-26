---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzffeghjk535
---

# CPad 内置LED指示灯管理
更新时间：2026-06-05 16:57:44
# **功能概述**
商米提供对CPad内置指示灯进行控制的API，支持开发者三方应用对其进行调用，其功能包含以下部分：  
| 功能模块  | 功能说明  |  
| --- | --- |  
| 设置状态灯行为  | 状态灯状态：常亮、熄灭、闪烁  |  
| 设置状态灯颜色  | 红色、绿色、蓝色、黄色、青色、品红、白色(支持R/G/B/RG/GB/RB/RGB七种组合颜色）  |  
# **SunmiCustomerAPI**
## **快速集成SunmiCustomerAPISDK**
  * 本地依赖导入


![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SUNMI_CUSTOMER_API_v1.3.48_release.zip 
将SUNMI_CUSTOMER_API_vx.x.x_release.aar文件放在libs目录下。
  * 在项目根目录的build.gradle中增加以下代码：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
buildscript {
	repositories {
		mavenCentral()
	}
}

allprojects {
    repositories {
        mavenCentral()
    }
}
```

  * 在app/build.gradle中增加以下代码：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
implementation files('libs/SUNMI_CUSTOMER_API_v1.x.x_release.aar')
```

导入aar包完成后，重新build项目。
### **2.2.2 SDK所支持的系统版本以及IDE**
  * 目前SDK支持API-19(Android 4.4+)
  * 目前SDK只支持Android studio、Intellij的集成


##  **2.3** 使用Sunmi Customer API
最佳实践:推荐在Applicaiton的onCreate()初始化API对象，通过进程全局唯一的引用对象调用API。请注意，绑定SunmiCustomerServiced服务为异步回调，请注意在回调连接成功后才能使用API。出现绑定SunmiCustomerServiced失败请先检查设备是否已经安装SunmiCustomerService。其次在项目AndroidManifest清单文件中添加
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 <queries>
        <package android:name="com.sunmi.tmservice"/>
 </queries>
```

然后重新尝试。
### **2.3.1 初始化Sunmi Customer API**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
TMSApi sTmsApi = new TMSApi();
```

### **2.3.2 绑定Sunmi Customer Service**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 /**
     * connect sunmi customer service
     */
    private void connectTmsService() {
        // connect tms service
        sTmsApi.connect(this, new TMSServiceConnection() {
            @Override
            public void onServiceConnected() {
                // tms service is connected
            }

            @Override
            public void onServiceDisconnected() {
                // tms service is disconnected
            }
        });
    }
```

### **使用Sunmi Customer API**
**例** ：获取设备型号
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    /**
     * Get Device Model
     */
    public void getModel() {
        try {
            sTmsApi.getDeviceInfo().getModel();
        } catch (RemoteException | TmsServiceDisconnectedException e) {
            e.printStackTrace();
        }
    }
  

```

  

# **API详情**
## 判断设备是否支持LED指示灯  
| **原型**  | int isSupportRgbLed();  |  
| --- | --- |  
| **功能**  | 判断设备是否支持LED指示灯  |  
| **参数**  | none  |   
 |  
| **返回值**  |  0：支持 -1：不支持 -40：接口在当前设备或ROM不支持 -41：未找到系统服务，请联系设备供应商  |  
| **备注**  | 
  * 仅适用于非金融设备控制系统内置的Rgb LED 指示灯，目前已支持CPad Android14。

 |  
## 打开LED指示灯  
| **原型**  | int openRgbLed(int rbg, int onMs, int offMs, int lightMode, long timeoutMs);  |  
| --- | --- |  
| **功能**  | 打开LED指示灯  |  
| **参数**  | rbg  | 预设颜色索引，仅允许 1-7：1 红(R)、2 绿(G)、3 蓝(B)、4 黄(R+G)、5 青(G+B)、6 品红(R+B)、7 白(R+G+B)  |  
| onMs  | 亮灯时间（毫秒）；仅lightMode == 1 时生效；常亮时可传任意值  |  
| offMs  | 灭灯时间（毫秒）；仅lightMode ==1时生效；常亮时可传任意值  |  
| lightMode  | 0：常亮；1：闪烁  |  
| timeoutMs  | 任务超时后自动释放灯控（毫秒）；仅> 0 时生效；0 表示不自动超时  |  
| **返回值**  |  0：成功 -1:接口调用失败(参数错误等)、非法 rbg（非 1-7）或非法 lightMode -40：接口在当前设备或ROM不支持 -41：未找到系统服务，请联系设备供应商  |  
| **备注**  | 
  * 仅适用于非金融设备控制系统内置的Rgb LED 指示灯，目前已支持CPad Android14。
  * lightMode为0 常亮模式下输入的onMs、offMs不会生效，只有闪烁模式下才生效。

 |  
## 关闭LED指示灯  
| **原型**  | int closeRgbLed();  |  
| --- | --- |  
| **功能**  | 关闭LED指示灯  |  
| **参数**  | none  |   
 |  
| **返回值**  |  0：成功 -1：失败 -40：接口在当前设备或ROM不支持 -41：未找到系统服务，请联系设备供应商  |  
| **备注**  | 
  * 仅适用于非金融设备控制系统内置的Rgb LED 指示灯，目前已支持CPad Android14。

 |  
  

  

上一篇：状态灯服务说明
下一篇：指纹文档
