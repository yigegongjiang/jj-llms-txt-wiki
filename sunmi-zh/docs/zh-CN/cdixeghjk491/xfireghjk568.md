---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfireghjk568
---

# 7.云打印机蓝牙配网SDK（仅适用于使用了SDK_V1接口的旧设备）
更新时间：2025-11-10 16:31:59
## **特别声明**
> 云打印机配网、云绑定过程是通过蓝牙实现的，所以需要利用手机的蓝牙来进行指令的通讯。SDK里对蓝牙配网过程进行了封装，便于APP简单开发实现打印机配对。使用该功能时必须保证蓝牙打开且允许应用获取位置权限。
> 安卓SDK DEMO下载链接：[安卓配网SDK](https://file.cdn.sunmi.com/SUNMIDOCS/CloudPrinter_AndroidSDK_Demo.zip)   
>   
> 微信小程序SDK下载链接：[微信小程序配网SDK](https://file.cdn.sunmi.com/SUNMIDOCS/CloudPrinter_WeChatSDK.rar)   
>   
> IOS SDK DEMO下载链接：[苹果配网SDK](https://file.cdn.sunmi.com/SUNMIDOCS/CloudPrinter_IOSSDK_Demo.rar)
> 到手的云打印机因为没有配置过现场wifi网络，所以在开发的时候联网很不方便。所以我们提供了一个安装在安卓手机上的配网工具给合作伙伴下载使用。同时此DEMO集成了配置wifi网络、基础设置、配置外卖接单等功能，可以用于调试。[点击下载wifi配网工具](https://wifi.cdn.sunmi.com/APK/CP/CloudPrinter_Wifi_Link2.apk)
# **1、配置工具使用方法**
## **1) 首页**
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/DEMO1.jpg)
在输入框中输入商米提供给您的真实APP_ID和APP_KEY，SHOP ID您可以自行定义（小于32位字符），输入完成后点击下方“开始”
## **2) 云打印机设置**
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/DEMO2.jpg)
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/DEMO3.jpg)
A. 配置云打印机wifi
B. 外卖接单绑定、票据及语音设置
C. 打印文字浓度设置
D. 语音播报方言设置、语速设置
E. 打印机提示音设置
# **2、安卓配网SDK类说明**
> Android配网SDK包名：BluetoothBinding.aar
## **1) 类声明**
**接口名：**
  * SunmiPrinterClient


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SunmiPrinterClient sunmiPrinterClicent = SunmiPrinterClient(Context context, IPrinterClient iPrinterClient);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| context  | Context  |   
 |  
| iPrinterClient  | IPrinterClient  | 云打印机蓝牙接口的回调类  |  
## **2) 统一处理回调接口**
> 配置蓝牙打印机的过程中需要通过BLE给蓝牙打印机发送消息，这里采用统一的回调处理蓝牙数据发送情况。
**接口名：**
  * sendDataFail


**回调方法：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void sendDataFail(int code, String msg) {
}
  

```

**回调参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | int  | 错误信息码 0-蓝牙连接失败 1-蓝牙Notify失败  |  
| msg  | String  | 错误信息  |  
# **3、安卓配网SDK接口说明**
## **1) 开始扫描打印机**
**接口名：**
  * startScan


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.startScan();
  

```

**回调方法：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void onPrinterFount(PrinterDevice printerDevice) {

}
  

```

**回调参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| printerDevice  | PrinterDevice  | 蓝牙打印机类 扫描到的打印机数据中只有默认的名称和蓝牙mac地址  |  
## **2) 停止扫描打印机**
> 接口没有回调，建议在当前页面销毁或者开始配置打印机前停止扫描
**接口名：**
  * stopScan


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.stopScan();
  

```

## **3) 获取蓝牙打印机SN**
**接口名：**
  * getPrinterSn


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.getPrinterSn(btAddress);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| btAddress  | String  | 打印机的蓝牙MAC地址  |  
**回调方法：**
  * 方法一：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void getSnRequestSuccess() {

}
  

```

> 说明：该方法表示发送获取SN的命令成功，等待获取SN，该方法内可以加入超时机制
  * 方法二：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void onSnReceived(String sn) {

}
  

```

**回调参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sn  | String  | 打印机的sn  |  
## **4) 获取打印机搜索到的WIFI列表**
**接口名：**
  * getPrinterWifiList


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.getPrinterWifiList(btAddress);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| btAddress  | String  | 打印机的蓝牙MAC地址  |  
**回调方法：**
  * 方法一：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void routerFound(Router router) {

}
  

```

**回调参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| router  | Router  | 打印机搜索到的WIFI类  |  
| Router类参数说明  |   
 |   
 |  
| 参数名  | 类型  | 说明  |  
| —  | —  | —  |  
| name  | String  | WIFI的名字  |  
| hasPwd  | boolean  | WIFI是否有密码  |  
| pwd  | String  | WIFI的密码，此时获取到的WIFI没有该数据  |  
| rssi  | int  | WIFI的信号强度范围0到4，越大信号越强  |  
| essid  | byte[]  | WIFI的essid，配网时需要该参数  |  
  * 方法二：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void onGetWifiListFinish() {

}
  

```

> 说明：蓝牙打印机搜索WIFI结束
  * 方法三：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void onGetWifiListFail() {

}
  

```

> 说明：蓝牙打印机搜索wifi失败
## **5) 给打印机配置wifi**
**接口名：**
  * setPrinterWifi


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.setPrinterWifi(btAddress, ssid, psw);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| btAddress  | String  | 打印机的蓝牙MAC地址  |  
| ssid  | byte[]  | WIFI的essid  |  
| pwd  | String  | WIFI的密码  |  
**回调方法：**
  * 方法一：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void onSetWifiSuccess() {

}
  

```

> 说明：打印机收到需要配置的WIFI信息，开始尝试连接WIFI，这里建议添加超时机制
  * 方法二：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void wifiConfigSuccess() {

}
  

```

> 说明：打印机设置WIFI成功
  * 方法三：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public void onWifiConfigFail() {

}
  

```

> 说明：打印机设置WIFI失败，一般是输入的WIFI密码不正确导致
## **6) 退出配网过程**
> 说明：该方法只有统一的发送蓝牙信息失败的回调，该方法建议在onBackPressed方法中调用
**接口名：**
  * quitConfig


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.quitConfig(btAddress);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| btAddress  | String  | 打印机的蓝牙MAC地址  |  
## **7) 请求删除wifi 配置，并使用移动网络**
> 说明：该方法只有统一的发送蓝牙信息失败的回调，如果打印机使用2G网络，不采用WIFI就可以调用该方法。
**接口名：**
  * deleteWifiInfo


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.deleteWifiInfo(btAddress);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| btAddress  | String  | 打印机的蓝牙MAC地址  |  
## **8) 断开和蓝牙打印机的蓝牙连接**
> 说明：该方法需要在页面销毁的时候进行调用
**接口名：**
  * disconnect


**接口声明：**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
sunmiPrinterClient.disconnect(btAddress);
  

```

**传入参数：**  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| btAddress  | String  | 打印机的蓝牙MAC地址  |  
# **4、微信小程序配网SDK接口说明**
**微信小程序配网SDK引入**
> 说明：使用该功能必须保证已经打开了手机蓝牙
把sdk.js文件放入需要引入接口的文件夹里
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
const sdk = require('../blueConnect/sdk.js');
  

```

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

# **5、IOS配网SDK接口说明**
> 说明：此SDK在真机或模拟器环境中均可使用；建议使用与iOS12或以上系统版本中
## **1) SDK集成**
  1. 目前SDK只支持手动集成。
  2. 将SDK中包含的 libSunmiPrinterManager.a、SunmiPrinterDeviceModel.h、 SunmiPrinterManager.h 三个文件导入到Xcode工程目录下。
  3. 在需要使用API的类中 importSunmiPrinterManager.h头文件，并增加 PrinterManagerDelegate协议。


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
#import "PrinterSettingController.h"
#import "SunmiPrinterManager.h"
@interface PrinterSettingController ()<PrinterManagerDelegate>
@property (nonatomic, strong) SunmiPrinterManager *printerSettingManager;
@end
  

```

## **2) SDK内属性以及主要方法使用方法说明**
为理解SDK各个方法的使用，大致梳理蓝牙打印机正常配置流程的步骤，如下：
  1. 打开手机蓝牙开关
  2. 搜索蓝牙打印机
  3. 连接蓝牙打印机
  4. 向蓝牙打印机获取设备SN（绑定设备时使用）
  5. 向蓝牙打印机获取Wi-Fi列表
  6. 向蓝牙打印机发送ssid和password，为设备配置Wi-Fi网络
  7. 配网步骤完成


## **3) 搜索蓝牙打印机设备**
> 说明：SDK在手机打开蓝牙后会自动搜寻蓝牙打印机设备。用户可以通过SunmiPrinterManager类的connectState属性来获取当前手机的蓝牙状态。当检测到手机蓝牙是开启状态时，用户可通过 shouldSearchDevice属性，来控制是否自动搜索蓝牙打印机，默认false，不会开始自动搜索。
**方法一：开始扫描**  
| 实例方法  | 说明  |  
| --- | --- |  
| -(void)scanPeripheral;  | 扫描蓝牙打印机设备  |  
当搜索到设备时，会执行代理方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| -(void)discoveredDevice : (SunmiPrinterDeviceModel*_Nonnull) device;  | 扫描到打印机设备，可以在此协议方法中获取扫描到的设备  |  
**方法二：取消扫描**  
| 实例方法  | 说明  |  
| --- | --- |  
| -(void)cancelScan;  | 取消扫描蓝牙打印机设备  |  
取消搜索之后，执行代理方法：  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)didCancelSearching;  | 取消了扫描打印机设备的操作，在此代理方法中做其他逻辑处理  |  
## **4) 连接蓝牙打印机设备**
**方法一：连接设备**  
| 实例方法  | 说明  |  
| --- | --- |  
| – (void)connectPeripheral:(CBPeripheral*_Nonnull)peripheral;  | 建立蓝牙打印机设备的连接  |  
连接设备成功时，执行代理方法  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)didConectPrinter;  | 连接上了蓝牙打印机设备  |  
**方法二：断开连接设备**  
| 实例方法  | 说明  |  
| --- | --- |  
| -(void)disConnectPeripheral;  | 断开和蓝牙打印机设备的连接  |  
要断开和蓝牙打印机的连接时，会执行代理方法  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)willDisconnectPrinter;  | 将要断开和蓝牙打印机设备的连接  |  
## **5) 获取蓝牙打印机的SN**
**方法：**  
| 实例方法  | 说明  |  
| --- | --- |  
| -(void)getPrinterSN;  | 向蓝牙打印机发送获取设备SN的指令，此方法获取的SN，将会在蓝牙打印机的绑定操作中使用  |  
**协议方法一：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)willStartReceiveDeviceSn;  | 表明已经开始发送获取SN的指令，可以利用此方法建立超时机制  |  
**协议方法二：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)receiveDeviceSn:(NSString*_Nullable)sn;  | 获取接收到的SN  |  
## **6) 获取打印机搜索到的WIFI列表**
**方法一：**  
| 实例方法  | 说明  |  
| --- | --- |  
| -(void)getWifiList;  | 向蓝牙打印机发送获取Wi-Fi列表的指令  |  
**协议方法一：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)receiveAPInfo:(NSDictionary*_Nullable)apInfo;  | 重写此代理方法，接收获取到的Wi-Fi网络信息。demo中会将接收到的Wi-Fi网络以列表形式展示出来  |  
**协议方法二：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)didReceiveAllApInfo;  | 在接收到蓝牙打印机发送的全部Wi-Fi网络信息时，会执行此代理方法  |  
**协议方法三：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)didFailReceiveApInfo;  | 若过程中接收失败，会触发此代理方法，在此方法内可做其他的自定义处理  |  
## **7) 选择某一个Wi-Fi网络为蓝牙打印机配置网络**
**方法一：**  
| 实例方法  | 说明  |  
| --- | --- |  
| -(void)connectAP:(NSString*_Nonnull)ssid password:(NSString*_Nullable)password;  | 发送Wi-Fi的网络的ssid和password给蓝牙打印机，为蓝牙打印机配置网络  |  
**协议方法一：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)willStartConfigPrinter;  | 表示配置网络的命令已经发给蓝牙打印机，可以在该法内加入超时机制  |  
**协议方法二：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)configPrinterSuccess;  | 配网成功的代理方法，可以在此方法内做后续操作  |  
**协议方法三：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)configPrinterFail;  | 配网失败的代理方法，可以在此方法内做后续其他操作  |  
**方法二：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)quitConnectAP;  | 使用此方法，退出配网流程  |  
## **8) 删除网络配置**
**方法：**  
| 协议方法  | 说明  |  
| --- | --- |  
| – (void)deleteWifiSetting;  | 使用此方法，删除Wi-Fi网络配置  |  
上一篇：6.打印机常用ESC/POS指令集（仅适用于使用了SDK_V1接口的旧设备）
下一篇：摄像头扫码SDK说明
