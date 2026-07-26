---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzfxeghjk491
---

# 安卓SDK对接
更新时间：2026-07-09 12:47:36
金融客户需要参数下发功能来实现对应用和设备的管理。
**SUNMI** 提供 **SunmiParamsSDK** ，以 **aar** 的形式提供给客户。
客户在云端配置参数后，可通过 **aar** 调用获取云端配置的参数。
# **使用限制**
  * 使用该**aar** 需要先完成[开发者能力接入流程](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmmeghjk546/)。
  * 完成**Android** 应用的创建并添加`参数管理SDK`能力。


# **配置描述**
## **引入aar库**
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SUNMI_Params_SDK_v1.2.4.aar 
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SUNMI_Params_SDK_recombine_v1.2.4.aar 
### **已引入SUNMI_CUSTOMER_API aar**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
implementation 'com.squareup.okhttp3:okhttp:3.10.0' //network lib
implementation files('libs/SUNMI_CUSTOMER_API_vx.x.x_release.aar') //TODO 替换为相应版本的aar文件
implementation files('libs/SUNMI_Params_SDK_recombine_v1.2.4.aar')
```

### **未引入SUNMI_CUSTOMER_API aar**
导入aar到项目的libs文件夹中
![](https://cdn.sunmi.com/public/image/mgt-document/9c44becec46a4b52acc0d7f3d58d6273.png)
在build.gradle文件中引入aar库，并添加相应的依赖库
![](https://cdn.sunmi.com/public/image/mgt-document/63927bfd03e542619833e8732bac69ce.png)
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
implementation 'com.squareup.okhttp3:okhttp:3.10.0' // network lib
implementation files('libs/SUNMI_Params_SDK_v1.2.4.aar')
```

## **使用SUNMI Params SDK**
### **初始化SUNMI Params SDK**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
TMSParamsSDK.init(this, appId, appKey, new InitCallback() {
            @Override
            public void onInitSuccess() {
                Log.d("TAG", "TMS Params SDK init success!");
            }
        });
```

### **使用SUNMI Params SDK**
**例：** 获取全部参数
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
private void getSunmiParams(String packageName, int versionCode) {
    TMSParamsSDK.getTMSParams(packageName, versionCode, new IRequestCallback() {
        @Override
        public void onSuccess(String json) {
            Log.d("TAG", "onSuccess: " + json);
        }

        @Override
        public void onError(int code, String message) {
            Log.d("TAG", "onError: code=" + code + ", message=" + message);
        }
    });
}
```

# **API 详情**
## **API列表**  
| **接口名**  | **接口描述**  |  
| --- | --- |  
| void init(Context context, String appId, String secret, InitCallback callback)  | 初始化参数管理SDK  |  
| void init(Context context, String appId, String secret,boolean isPublicCloud, InitCallback callback)  | 初始化参数管理SDK  |  
| void initDomain(String domain)  | 修改参数管理SDK默认域名  |  
| void checkDomain(IRequestCallback callback)  | 检查参数管理SDK域名配置  |  
| void getTMSParams(String packageName, int versionCode, IRequestCallback callback)  | 获取全部参数  |  
| void getTMSSingleParams(String type, String packageName, int versionCode, IRequestCallback callback)  | 获取单个参数  |  
| void checkTMSSingleParamsUpdate(String type, String packageName, int versionCode, IRequestCallback callback)  | 检查单个参数更新  |  
| void updateTMSDeviceParams(String key, String value, IRequestCallback callback)  | 修改设备参数（只支持string类型）  |  
| void initPushConfig(SunmiParamsPushInitCallback callback)  | 初始化参数管理SDK推送模块 (初始化推送配置时，会静默安装SunmiParamsPushForwarder应用，用于支持参数管理SDK接收**SUNMI** 云推送消息)  |  
| void setOnParamsUpdateListener(OnParamsUpdateListener listener)  | 设置参数变更推送回调  |  
## **接口详情**
### **初始化参数管理SDK**
#### **接口详情**  
| **接口原型**  | void init(Context context, String appId, String secret, InitCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | Initialization Parameter Management SDK  |   
 |  
| **参数**  | Context context  | 应用程序上下文  |  
|   
 | String appId  | appId，在云端生成  |  
|   
 | String secret  | secret，在云端生效  |  
|   
 | InitCallback callback  | 参数管理SDK初始化回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void init(String appId, String appSecret) {
        TMSParamsSDK.init(this, appId, appSecret, new InitCallback() {
            @Override
            public void onInitSuccess() {
                Log.d("TAG", "TMS Params SDK init success!");
            }
        });
    }
```

  

### **初始化参数管理SDK**
#### **接口详情**  
| **接口原型**  | void init(Context context, String appId, String secret, boolean isPublicCloud, InitCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | Initialization Parameter Management SDK with public cloud  |   
 |  
| **参数**  | Context context  | 应用程序上下文  |  
|   
 | String appId  | appId，你的appId  |  
|   
 | String secret  | secret，你的app secret  |  
|   
 | boolean isPublicCloud  | true 访问公有云，固定true  |  
|   
 | InitCallback callback  | 参数管理SDK初始化回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void init(String appId, String appSecret) {
        TMSParamsSDK.init(this, appId, appSecret, true, new InitCallback() {
            @Override
            public void onInitSuccess() {
                Log.d("TAG", "TMS Params SDK init success!");
            }
        });
    }
```

  

### **修改参数管理SDK默认域名**
#### **接口详情**  
| **接口原型**  | void initDomain(String domain)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 修改参数管理SDK默认域名  |   
 |  
| **参数**  | String domain  |  域名，例： [https://openapi.sunmi.com/](https://openapi.sunmi.com/) [https://openapi.us.sunmi.com/](https://openapi.us.sunmi.com/) [https://openapi.eu.sunmi.com/](https://openapi.eu.sunmi.com/)  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void initDomain(String domain) {
        TMSParamsSDK.initDomain(domain);
    }
```

  

### **检查参数管理SDK域名配置**
#### **接口详情**  
| **接口原型**  | void checkDomain(IRequestCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 检查参数管理SDK域名配置  |   
 |  
| **参数**  | IRequestCallback callback  | 接口回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void checkDomain() {
        TMSParamsSDK.checkDomain(new IRequestCallback() {
            @Override
            public void onSuccess(String json) {
                Log.d("TAG", "checkAutoDomainEnabled onSuccess: " + json);
            }

            @Override
            public void onError(int code, String message) {
                Log.d("TAG", "checkAutoDomainEnabled onError: code=" + code + ", message=" + message);
            }
        });
    }
```

#### **返回值示例**
**auto_domain_enabled** : 是否支持自动获取参数管理域名
**domain** : 当前参数管理访问域名
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "auto_domain_enabled":true,
    "domain":"https://openapi.sunmi.com/"
}
```

  

### **获取全部参数**
#### **接口详情**  
| **接口原型**  | void getTMSParams(String packageName, int versionCode, IRequestCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 获取全部参数  |   
 |  
| **参数**  | String packageName  | 应用包名  |  
|   
 | int versionCode  | 应用版本号  |  
|   
 | IRequestCallback callback  | 接口请求回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
private void getSunmiParams(String packageName, int versionCode) {
    TMSParamsSDK.getTMSParams(packageName, versionCode, new IRequestCallback() {
        @Override
        public void onSuccess(String json) {
            Log.d("TAG", "onSuccess: " + json);
        }

        @Override
        public void onError(int code, String message) {
            Log.d("TAG", "onError: code=" + code + ", message=" + message);
        }
    });
}
```

#### **返回值示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "app_params": {},
    "global_params": {},
    "device_params": {
	    "device_level1": {
	        "device_level1_key": "device_level1_value"
	    },
	    "device_key1": "device_value1",
	    "device_key2": "device_value2"
	}
}
```

  

### **获取单个参数**
#### **接口详情**  
| **接口原型**  | void getTMSSingleParams(String type, String packageName, int versionCode, IRequestCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 获取单个参数  |   
 |  
| **参数**  | String type  |  参数类型： device - 设备参数  |  
|   
 | String packageName  | 应用包名  |  
|   
 | int versionCode  | 应用版本号  |  
|   
 | IRequestCallback callback  | 接口请求回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void getSunmiSingleParams(String type, String packageName, int versionCode) {
        TMSParamsSDK.getTMSSingleParams(type, packageName, versionCode, new IRequestCallback() {
            @Override
            public void onSuccess(String json) {
                Log.d("TAG", "onSuccess: " + json);
            }

            @Override
            public void onError(int code, String message) {
                Log.d("TAG", "onError: code=" + code + ", message=" + message);
            }
        });
    }
```

#### **返回值示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "first":{
        "params":"388"
    },
    "test":"testValue"
}
```

  

### **检查单个参数更新**
#### **接口详情**  
| **接口原型**  | void checkTMSSingleParamsUpdate(String type, String packageName, int versionCode, IRequestCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 检查单个参数更新  |   
 |  
| **参数**  | String type  |  参数类型： device - 设备参数  |  
|   
 | String packageName  | 应用包名  |  
|   
 | int versionCode  | 应用版本号  |  
|   
 | IRequestCallback callback  | 接口请求回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void checkSunmiSingleParamsUpdate(String packageName, int versionCode, String type) {
        TMSParamsSDK.checkTMSSingleParamsUpdate(type, packageName, versionCode, new IRequestCallback() {
            @Override
            public void onSuccess(String json) {
                Log.d("TAG", "onSuccess: " + json);
            }

            @Override
            public void onError(int code, String message) {
                Log.d("TAG", "onError: code=" + code + ", message=" + message);
            }
        });
    }
```

#### **返回值示例**
must_update: 参数是否已更新
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "must_update":false
}
```

  

### **修改设备参数**
#### **接口详情**  
| **接口原型**  | void updateTMSDeviceParams(String key, String value, IRequestCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 修改设备参数  |   
 |  
| **参数**  | String packageName  | 应用包名  |  
|   
 | int versionCode  | 应用版本号  |  
|   
 | String key  | 参数名  |  
|   
 | String value  | 参数值  |  
|   
 | IRequestCallback callback  | 接口请求回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void updateSunmiDeviceParams(String key, String value) {
        TMSParamsSDK.updateTMSDeviceParams(key, value, new IRequestCallback() {
            @Override
			//Distinguish multi-level parameters with '/'
            public void onSuccess(String json) {
                Log.d("TAG", "onSuccess: " + json);
            }

            @Override
            public void onError(int code, String message) {
                Log.d("TAG", "onError: code=" + code + ", message=" + message);
            }
        });
    }
```

  

### 初始化参数管理SDK推送模块
#### **接口详情**  
| **接口原型**  | void initPushConfig(SunmiParamsPushInitCallback callback)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 初始化参数管理SDK推送模块 (初始化推送配置时，会静默安装SunmiParamsPushForwarder应用，用于支持参数管理SDK接收**SUNMI** 云推送消息)  |   
 |  
| **参数**  | SunmiParamsPushInitCallback callback  | 参数管理SDK推送模块初始化回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void initPushConfig() {
         TMSParamsSDK.initPushConfig(new SunmiParamsPushInitCallback() {
                @Override
                public void onInitResult(boolean success) {
                    Log.d("TAG", "onInitResult: " + success);
                }
            });
    }
```

#### **备注**
如果推送模块初始化失败，可以检查设备上是否已安装SunmiParamsPushForwarder组件，应用包名为com.sunmi.params_push_forwarder。
若设备未安装上述组件，则可以通过手动安装或者云端定向推送组件的方式，为设备安装组件。
部分金融客户存在定制的金融签名，如果需要安装上述组件，需要先对该组件进行加签操作。
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiParamsPushForwarder.apk 
### 设置参数变更推送回调
#### **接口详情**  
| **接口原型**  | void setOnParamsUpdateListener(OnParamsUpdateListener listener)  |   
 |  
| --- | --- | --- |  
| **接口描述**  | 设置参数变更推送回调  |   
 |  
| **参数**  | OnParamsUpdateListener listener  | 参数变更回调  |  
| **返回值**  | 无  |   
 |  
#### **调用示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
    private void setOnParamsUpdateListener() {
         TMSParamsSDK.setOnParamsUpdateListener(new OnParamsUpdateListener() {
                @Override
                public void onParamsUpdate(List<String> paramsTypes) {
                    Log.d("TAG", "onParamsUpdate: " + paramsTypes);
                }
            });
    }
```

  

## **错误码**  
| **错误码**  | **错误提示**  |  
| --- | --- |  
| -1  | **SUNMI** Params SDK初始化异常！  |  
| -2  | 设备网络不可用！  |  
| -3  | AppID校验失败，请检查您的AppID！  |  
| -4  | AppKey校验失败，请检查您的AppKey！  |  
| -5  | 设备未备案！  |  
| -6  | 设备和子机构不匹配！  |  
| -7  | 请求参数错误，请检查您的请求参数！  |  
| -8  | 该应用未上架到应用市场！  |  
| -9  | 服务器异常！  |  
| -10  | 未找到该应用的对应版本！  |  
| -11  | 请检查您的应用版本号！  |  
| -12  | 请检查设备是否安装了SunmiCustomerService组件！  |  
| -13  | 未找到此设备参数，请检查是否输入正确！  |  
| -14  | 此设备参数不支持修改！  |  
  

  

  

  

  

  

  

  

上一篇：设备应用参数推送参数详情
下一篇：商米打印服务介绍
