---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfraeghjk480
---

# 12、云打印机微信小程序SDK
更新时间：2025-12-19 22:49:49
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
# 方案一：客户H5唤起商米“Wifi配网服务”小程序
## 1、云开发说明
客户端H5 APP可以通过云函数调用，获取到微信目标小程序的URL Scheme，然后跳转到微信打开wifi配网小程序。
调用微信小程序需要依赖云开发web SDK：https://res.wx.qq.com/open/js/cloudbase/1.1.0/cloud.js
## 2、客户端示例代码
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var c = new cloud.Cloud({
      // 必填，表示是未登录模式
      identityless: true,
      // 资源方 AppID，请联系商米售后人员获得
      resourceAppid: 'xxxxxx', // <!-- 商米服务小程序appid -->
      // 资源方 环境ID
      resourceEnv: 'sunmi-9gq8dzc93d7bab6d', // <!-- 商米服务云函数环境id -->
    })

    await c.init({ env: 'sunmi-9gq8dzc93d7bab6d' }) // <!-- replace --> 云开发环境ID 
    window.c = c

    try {
      await openWeapp()
    } catch (e) {
      throw e
    }
  }
})

async function openWeapp(onBeforeJump) {
  // 调用云函数, 获取目标小程序 URLScheme
  var c = window.c
  const res = await c.callFunction({
    name: 'public-iot',  // 云函数名  <!-- replace -->
    data: {
      action: 'getUrlScheme',  // 开放action  <!-- replace -->
    },
  });
  if (onBeforeJump) {
    onBeforeJump()
  }
  if(res.result.openlink){
    location.href = res.result.openlink
  }else{
    console.warn('未获取到openLink',res.result)
  }
}
  

```

# 方案二：小程序引入配网SDK
**特别说明**
云打印机配网、云绑定过程是通过蓝牙实现的，所以需要利用手机的蓝牙来进行指令的通讯。SDK里对蓝牙配网过程进行了封装，便于简单开发实现打印机配对。使用该功能时必须保证蓝牙打开且允许应用获取位置权限。
**微信小程序SDK下载：**
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) CloudPrinter_WeChatSDK.rar 
**微信小程序SDK引入：**
> 说明：使用该功能必须保证已经打开了手机蓝牙
把sdk.js文件放入需要引入接口的文件夹里
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
const sdk = require('../blueConnect/sdk.js');
  

```

# **配网SDK接口说明**
## **1) 初始化蓝牙模块，开始扫描周围的设备**
**接口名：**
  * openBluetoothAdapter


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//接口的参数是一个回调函数status
sdk.openBluetoothAdapter((res) => {
    console.log('蓝牙打开结果', res);
}); 
  

```

**回调参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| status  | 0  | 蓝牙模块初始化成功  |  
| status  | 10001  | 蓝牙模块初始化失败，用户没有打开手机蓝牙  |  
## **2) 获取周围的蓝牙设备列表**
**接口名：**
  * onBluetoothDeviceFound


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//接口的参数是一个回调函数devicesList
sdk.onBluetoothDeviceFound((devicesList) => {
    console.log('devicesList', devicesList);
})
  

```

**回调参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| devicesList  | [ ]  | 数组  |  
## **3) 停止扫描周围的蓝牙设备**
**接口名：**
  * stopScanDevices


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.startScanDevices();
  

```

## **4) 关闭蓝牙模块，调用该方法将断开所有已建立的连接并释放系统资源**
**接口名：**
  * closeBluetoothAdapter


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.startScanDevices();
  

```

## **5) 连接蓝牙设备**
**接口名：**
  * createBLEConnection


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.createBLEConnection(deviceId, (res) => {
    this.setData({
        canWrite: res.canWrite,
        deviceId: res.deviceId,
        characteristicId: res.characteristicId,
        connected: res.connected,
    })
    console.log('设备连接成功', res)
});
  

```

**接口参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| deviceId  | string  | 可以通过onBluetoothDeviceFound 接口获得设备列表，每个设备信息里面有deviceId  |  
**回调参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| status  | 0  | 连接成功特征值获取成功，并且有下面的参数  |  
|   
 | 10001  | 连接失败  |  
|   
 | 10002  | 连接成功，获取特征值失败（设备不支持特征值读写）  |  
| characteristicId  | string  | 特征值ID  |  
| deviceId  | string  | 设备ID  |  
| serviceId  | string  | 服务ID  |  
## **6) 获取云打印机SN号**
**接口名：**
  * getSN


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//接口的参数是一个回调函数
sdk.getSN((data) => {
    console.log('sn', data);
    this.setData({
        sn: data
    });
});
  

```

**回调参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| data  | string  | SN号字符串  |  
## **7) 获取云打印机设备搜索到的Wi-Fi列表**
**接口名：**
  * getWifiList


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//接口的参数是一个回调函数
sdk.getWifiList((data) => {
    this.setData({
        wifiList: data
    })
    console.log('wifiList', data);
});
  

```

**回调参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| data  | [ ]  | 数组，每个元素都是Wi-Fi信息，含有ssid(Wi-Fi名称)、mode（Wi-Fi模式）、rssi（Wi-Fi强度）、complete（完整的回复指令信息）  |  
## **8) 连接指定Wi-Fi**
**接口名：**
  * connectWifi


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.connectWifi(essid, password, (res) => {
    console.log('Wi-Fi连接结果打印', res);
});
  

```

**接口参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| essid  | string  | Wi-Fi名称  |  
| password  | string  | Wi-Fi密码  |  
**回调参数：**  
| 参数名  | 参数值  | 说明  |  
| --- | --- | --- |  
| status  | 0  | Wi-Fi连接成功  |  
|   
 | 10001  | Wi-Fi连接出错,请检查Wi-Fi账号、密码  |  
## **9) 退出配网设置**
**接口名：**
  * cancelConnect


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.cancelConnect();
  

```

## **10) 删除Wi-Fi配置**
**接口名：**
  * deleteWifi


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.cancelConnect();
  

```

## **11) 断开蓝牙连接**
**接口名：**
  * closeBLEConnection


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sdk.closeBLEConnection();
  

```

上一篇：11、云打印机微信WIFI配网说明
下一篇：13、云打印机iOS SDK
