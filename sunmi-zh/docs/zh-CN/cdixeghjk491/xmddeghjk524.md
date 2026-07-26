---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmddeghjk524
---

# 指纹文档
更新时间：2025-11-17 18:10:24
## 1. 版本记录  
| 版本  | 日期  | 修改记录  | 修改者  |  
| --- | --- | --- | --- |  
| 1.0.0  | 2019/11/19  | 第一版开发版（仅限debug）  | Bojack  |  
| 1.0.1  | 2019/12/18  | 增加绑定服务修改（仅限debug）  | Bojack  |  
| 1.0.2  | 2020/03/31  | 更新接口DetectCallback onDetectFingerprint(仅限debug)  | Bojack  |  
| 1.0.3  | 2020/04/16  | 更新SPI设备上下电控制逻辑(release)  | Bojack  |  
| 1.0.4  | 2020/09/27  | 增加NFIQ数据支持  | Bojack  |  
| 1.0.5  | 2020/12/18  | 优化录入接口  | Bojack  |  
| 1.0.6  | 2021/02/01  | 新增特性: 检查连续相同指纹注意：a) 为了实现这一特性,服务会临时记录当前指纹信息, releaseFingerprint() 被调用或者客户端与服务丢失连接, 当前指纹信息将会从服务中删除b) 检查相同连续指纹将在服务端耗时400-500ms  | Bojack  |  
## 2. 参考源码
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SunmiFingerprintDemo.zip 
## 3. 接口
### 3.1 简介
Sunmi指纹服务为商米设备上的指纹模块而开发.开发者可以通过商米指纹服务来满足自己的业务场景需求. 当前指纹服务提供 engagement/release, management, detection, enrollment 和identification 功能.
### 3.2 引导
#### 3.2.1 注意
本文档为Sunmi指纹服务集成开发文档. 使用SDK前, 开发者应当对Android编程以及相关IDE有基础认知, 有基础的编码能力. 在使用SDK前, 请认真阅读此文档. 请参照SunmiFingerprintDemo进行功能开发,因为SunmiFingerprintDemo展示了Sunmi指纹服务调用的最佳实践
#### 3.2.2 集成开发
当前SDK仅支持Android Studio 集成开发.
**请注意:**
**1．SunmiFingerprintDemo 展示了 Sunmi指纹服务的最佳实践, 请参照使用 “libsunmifingerprint” module 来连接 Sunmi指纹服务, 这将使得Sunmi指纹服务自动发现来自客户端的断联时间,从而能及时释放相关资源.**
**2． SunmiFingerprintService 可以连接多客户端进程，因为它是一个Android AIDL 服务。 但是为了保证数据的安全性，我们设计了它只能被一个客户端应用访问，这样在任何情况下，指纹传感器数据只能被一个特定的应用访问，请按照SunmiFingerprintDemo的源代码，在任何指纹操作之前尝试调用engageFingerprint () ，并及时调用 releaseFingerprint() 以降低功耗。.**
### 4.1 基础功能
#### 4.1.1 SunmiFingerprintKernel
源码示例: SunmiFingerprintDemo->libsunmifingerprint->SunmiFingerprintKernel
这是一个单例对象类，我们设计用于在客户端进程和SunmiFingerprintService之间建立Android辅助服务连接，它有一个接口调用“ConnectCallback”来通知客户端进程连接或断开连接.
4.1.1.1 SunmiFingerprintKernel.initService()
源码示例:SunmiFingerprintDemo->MyApplication->connectSunmiFingerprintService()  
| 原型  | **boolean initService(Context context, ConnectCallback callback)**  |  
| --- | --- |  
| 功能  | 初始化SunmiFingerprintService, 并绑定服务  |  
| **参数**  | contex ： Android Contextcallback[in] ： Connection status callback, interface in SunmiFingerprintKernel  |  
| 返回  | true: 启动绑定服务成功false: 启动失败,请检查Log  |  
| 注意  | 请通过调用此方法来初始化SunmiFingerprintService，此方法将帮助SunmiFingerprintService 自动检测相关客户端进程的崩溃或退出。  |  
4.1.1.2 SunmiFingerprintKernel.ConectCallback
源码示例: SunmiFingerprintDemo->MyApplication->connectCallback  
| 原型  | **void onConnect()**  |  
| --- | --- |  
| 功能  | 成功绑定服务后回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | **void onDisconnected()**  |  
| --- | --- |  
| 功能  | 与服务丢失连接后回调  |  
| 参数  | Error[in] Fingerprint errors: 参照 FingerprintError  |  
| 返回  | None  |  
| 注意  | 如果调用此方法，则表示 SunmiFingerprintService 崩溃，您应该退出应用程序并尝试重新连接 SunmiFingerprintService  |  
#### 4.1.2 占用指纹
源码示例: SunmiFingerprintDemo->MainActivity->engage()  
| 原型  | **int engageFingerprint(in Bundle settings, in ConnectStatusCallback callback)**  |  
| --- | --- |  
| 功能  | 占用指纹  |  
| 参数  | settings[in] ： Settings for engagementcallback[in] ： Connection status callback. 参照 ConnectStatusCallback  |  
| 返回  | 0: Success-1: api尚不支持   
  
-2: 未知错误-3: 已占用  |  
| 注意  | 使用指纹功能之前, 请调用该接口让指纹服务开启指纹传感器并占用.  |  
#### 4.1.3 释放指纹
源码示例: SunmiFingerprintDemo->MainActivity->release()  
| 原型  | int releaseFingerprint()  |  
| --- | --- |  
| 功能  | 释放指纹设备  |  
| 参数  | None  |  
| 返回  | 0: Success-1: api尚不支持   
  
-2: 未知错误  |  
| 注意  | 请调用该接口释放指纹设备，SunmiFingerprintService 将关闭指纹传感器。如果没有调用该接口，除非绑定指纹服务的应用程序崩溃或退出，否则指纹服务将无法绑定到任何其他进程，指纹传感器将保持上电。  |  
### 4.2 检测功能
4.2.1 检测功能
源码示例: SunmiFingerprintDemo->FingerprintDetectActivity->detectFingerprint()  
| 原型  | int detectFingerprint(in Bundle settings, in DetectCallback callback)  |  
| --- | --- |  
| 功能  | 获取sensor上的指纹数据  |  
| 参数  | settings [in] ： Contains the following data: FingerprintKeywords.TIME_OUT: int, default value is 5 (seconds) FingerprintKeywords.FINGERPRINT_IMAGE_FORMAT: String, “default”or “raw”, default value is“default” FingerprintKeywords.FINGERPRINT_FEATURE_FORMAT: String, “iso_fmr”or None FingerprintKeywords.FINGERPRINT_ISO_IMAGE_JPEG2000: String, “yes”or None FingerprintKeywords.FINGERPRINT_ISO_IMAGE_WSQ: String, “yes”or None FingerprintKeywords.FINGERPRINT_NFIQ，String, “yes”or None FingerprintKeywords.DetectParams.SAME_FINGER_CHECK: String, “yes”or “no” FingerprintKeywords.DetectParams.IMAGE_SIZE_500X500: String, “yes”or “no”, mean all data will be transformed from 500X500 raw image data callback [in] ： Detection status callback: 参照 DetectCallback  |  
| 返回  | 0: Success-1: api尚不支持other value: 未知错误  |  
| 注意  | 该接口旨在从传感器获取指纹图像数据和指纹特征数据。要检测指纹，需要进行一些设置，至少您必须定义 FINGERPRINT_IMAGE_FORMAT 设置。  |  
4.2.2 取消检测
源码示例: SunmiFingerprintDemo->FingerprintDetectActivity->cancelDetect()  
| 原型  | **int cancelDetect()**  |  
| --- | --- |  
| 功能  | 取消检测功能  |  
| 参数  | None  |  
| 返回  | 0: Success-1: api尚不支持Other value: Fail  |  
| 注意  | 取消指纹检测任务请注意，并非所有类型的指纹传感器都支持检测取消功能，某些指纹传感器在检测被触发后会在超时内阻塞。请记住检查此功能的返回值以制作正确的 UI，尤其是在使用进度对话框时。  |  
### 4.3 录入功能
4.3.1 录入功能
源码示例: SunmiFingerprintDemo->FingerprintEnrollActivity->enrollFingerprint()  
| 原型  | int enrollFingerprint(in Bundle settings, in EnrollCallback callback)  |  
| --- | --- |  
| 功能  | 录入指纹  |  
| 参数  | settings [in] ： Contains the following data:  |  
| FingerprintKeywords.TIMEOUT: int, default value is 10(seconds)  |   
 |  
| FingerprintKeywords.FINGERPRINT_USER_INDEX: int, the index for enrollmentcallback [in] ： Enroll status callback: 参考 EnrollCallback  |   
 |  
| 返回  | 0: Success-1: api尚不支持Other value: Fail  |  
| 注意  | 接口旨在将指纹注册到设备中  |  
4.3.2 取消录入
源码示例: SunmiFingerprintDemo->FingerprintEnrollActivity->cancelEnroll()  
| 原型  | int cancelEnroll()  |  
| --- | --- |  
| 功能  | Cancel enroll task  |  
| 参数  | None  |  
| 返回  | 0: Success-1: api尚不支持Other value: Fail  |  
| 注意  | 取消指纹登记任务。请注意，并非所有类型的指纹传感器都支持取消注册功能，某些指纹传感器会在注册后在超时时间内阻塞  |  
### 4.4 识别功能
4.4.1 识别功能
源码示例: SunmiFingerprintDemo->FingerprintIdentifyActivity->identifyFingerprint()  
| 原型  | int identify(in Bundle settings, in IdentifyCallback callback)  |  
| --- | --- |  
| 功能  | 识别指纹  |  
| 参数  | settings [in] ： Contains the following data:  |  
| FingerprintKeywords.TIMEOUT: int, default value is 10(seconds)callback [in] ： Idenfication status callback: Refer to IdentifyCallback  |   
 |  
| 返回  | 0: Success-1: api尚不支持Other value: Fail  |  
| 注意  | 用于识别当前指纹，查看当前指纹是否已注册到设备中，是哪个索引  |  
4.4.2 取消识别
源码示例: SunmiFingerprintDemo->FingerprintIdentifyActivity->cancelIdentify  
| 原型  | int cancelIdentify()  |  
| --- | --- |  
| 功能  | 取消识别  |  
| 参数  | None  |  
| 返回  | 0: Success-1: api尚不支持Other value: Fail  |  
| 注意  | 取消指纹识别任务请注意，并非所有类型的指纹传感器都支持注册识别功能，某些指纹传感器在识别被触发后会在 超时内阻塞。请记住检查此功能的返回值以制作正确的 UI，尤其是在使用进度对话框时。  |  
### 4.5 指纹管理功能
4.5.1 获取可注册的最大指纹数
源码示例: SunmiFingerprintDemo->FingerprintManageActivity->getDeviceCapacity()  
| 原型  | **int getDeviceCapacity()**  |  
| --- | --- |  
| 功能  | 获取最大的可录入指纹数  |  
| 参数  | None  |  
| 返回  | >=0: 最大容量Other value: Failed  |  
| 注意  | 返回可录入最大指纹数量  |  
4.5.2 获取指纹设备中已经输入的指纹数量
源码示例: SunmiFingerprintDemo->FingerprintManageActivity->getEnrolledNumber()  
| 原型  | **int getEnrolledNumber()**  |  
| --- | --- |  
| 功能  | 获取已经录入的指纹数量  |  
| 参数  | None  |  
| 返回  | >=: 已录入指纹数量Other value: Failed  |  
| 注意  | 返回已录入的指纹数量  |  
4.5.3 清除所有指纹数据
源码示例: SunmiFingerprintDemo->FingerprintManageActivity->deleteAll()  
| 原型  | int deleteAll(in DeleteCallback callback)  |  
| --- | --- |  
| 功能  | 删除所有指纹数据  |  
| 参数  | callback [in] ： Deletion status callback: 参考 DeleteCallback  |  
| 返回  | 0: SuccessOther value: Fail  |  
| 注意  | 会删除当前指纹设备所有存储的指纹  |  
4.5.4 删除特定指纹数据 (删除设置指定)
源码示例: SunmiFingerprintDemo->FingeprintManageActivity->deleteOne  
| 原型  | int deleteOne(in Bundle settings, in DeleteCallback callback)  |  
| --- | --- |  
| 功能  | 删除特定指纹数据  |  
| 参数  | settings[in] : Contains the following data:  |  
| FingerprintKeywords.FINGERPRINT_USER_INDEX: int, the index when this specific fingerprint used when enrolledcallback[in] : Deletion status callback: Refer to DeleteCallback  |   
 |  
| 返回  | 0: Success Other value: Fail  |  
| 注意  | 按照删除设置,删除特定指纹数据  |  
4.5.5 获取指纹设备信息
源码示例: SunmiFingerprintDemo->FingerprintManageActivity->getFingerprintInfo()  
| 原型  | Bundle getFingerprintDeviceInfo()  |  
| --- | --- |  
| 功能  | 获取指纹设备信息  |  
| 参数  | None  |  
| 返回  | Bundle : Contains the following data:FingerprintKeywords.DEVICE _MODEL: String, sensor device modelFingerprintKeywords.DEVICE_FIRMWARE_VERSION: String, firmware versionFingerprintKeywords.DEVICE_ID: String, device IDFingerprintKeywords.DEVICE_PRODUCT: String, device productFingerprintKeywords.DEVICE_SERIALNUMBER: String, sensor serial numberFingerprintKeywords.DEVICE_MANUFACTURER: String, sensor manufacturer  |  
| 注意  | 接口返回Bundle数据, 展示所有指纹数据  |  
4.5.6 转换WSQ数据为bitmap数据
源码示例: SunmiFingerprintDemo->FingerprintManageActivity->wsqToBitmapByte ()  
| 原型  | byte[] wsqToBitmapByte(in byte[] wsqData)  |   
 |  
| --- | --- | --- |  
| 功能  | 转换wsq数据为Bitmap数据  |   
 |  
| 参数  | byte[] wsqData  | byte[], wsq data that get from sensor  |  
| 返回  | byte[]  | bitmap byte array[] converted from wsq byte[] array data  |  
| 注意  | 此接口将 wsq 字节数组数据转换为位图数据数组  |   
 |  
### 4.6 指纹回调接口
4.6.1 ConnectStatusCallback
源码示例: SunmiFingerprintDemo->MainActivity->connectStatusCallback  
| 原型  | void onConnectSuccess()  |  
| --- | --- |  
| 功能  | 成功占用指纹时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onConnectFailed ()  |  
| --- | --- |  
| 功能  | 断开连接时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
4.6.2 DeleteCallback
源码示例: SunmiFingerprintDemo->FingerprintManageActivity->deleteCallback  
| 原型  | void onDeleteSucess()  |   
 |  
| --- | --- | --- |  
| 功能  | 成功删除数据时回调  |   
 |  
| 参数  | None  |   
 |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
| 原型  | void onDeleteFailed(in FingerprintError error)  |   
 |  
| 功能  | 删除数据失败时回调  |   
 |  
| 参数  | Error[in]  | Fingerprint errors: Refer to FingerprintError  |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
4.6.3 DetectCallback
源码示例: SunmiFingerprintDemo->FingeprintDetectActivity->detectCallback  
| 原型  | void onDetectStart()  |  
| --- | --- |  
| 功能  | 开始检测时 回调  |  
| 参数  |   
 |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onDetectFingerprint(in Bundle fingerprint)  |   
 |  
| --- | --- | --- |  
| 功能  | 检测到指纹数据时回调  |   
 |  
| 参数  | Fingerprint[in]  | Contains following data:FingerprintKeywords.FINGERPRINT_IMAGE_DATA: byte[], data which indicate by settings, “default”or “raw”FingerprintKeywords.FINGERPRINT_FEATURE_DATA: byte[], data which indicate by settings, “iso_fmr”Fingerprintkeywords.FINGERPRINT_ISO_IMAGE_JPEG2000: byte[], data which indicate by settings, “yes”FingerprintKeywords.FINGERPRINT_ISO_IMAGE_WSQ: byte[], data which indicate by settings, “yes”FingerprintKeywords.DetetcParams.SAME_FINGER_CHECK_SCORE: int, 0 ~ 100(usually, when this data equals 0 , we think this is not the same finger)  |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
| 原型  | void onDetectFailed(in FingerprintError error)  |   
 |  
| --- | --- | --- |  
| 功能  | 检测失败时回调  |   
 |  
| 参数  | Error[in]  | Fingerprint errors: Refer to FingerprintError  |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
| 原型  | void onDetecting()  |  
| --- | --- |  
| 功能  | 正在检测时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onTimeout()  |  
| --- | --- |  
| 功能  | 检测超时时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onDetectStop()  |  
| --- | --- |  
| 功能  | 检测中止时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onDetectCanceled()  |  
| --- | --- |  
| 功能  | 检测被取消时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
4.6.4 EnrollCallback
源码示例: SunmiFingerprintDemo->FingerprintEnrollActivity->enrollCallback  
| 原型  | void onEnrollSuccess()  |  
| --- | --- |  
| 功能  | 成功录入时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onEnrollFailed(in FingerprintError error)  |   
 |  
| --- | --- | --- |  
| 功能  | 录入失败时回调  |   
 |  
| 参数  | Error[in]  | Fingerprint errors: Refer to FingerprintError  |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
| 原型  | void onEnrollCanceled()  |  
| --- | --- |  
| 功能  | 取消录入时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onEnrollTimeout()  |  
| --- | --- |  
| 功能  | 录入超时时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onPressFingerHint()  |  
| --- | --- |  
| 功能  | 提示手指按压时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onRaiseFingerHint()  |  
| --- | --- |  
| 功能  | 提示手指移开时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
4.6.4 IdentifyCallback
源码示例: SunmiFingerprintDemo->FingerprintIdentifyActivity->identifyCallback  
| 原型  | void onIdentifySuccess(in Bundle info)  |   
 |  
| --- | --- | --- |  
| 功能  | 识别成功时回调  |   
 |  
| 参数  | Info[in]  | Contains following data:FingerprintKeywords.FINGERPRINT_USER_INDEX: int, id which used when this fingerprint is enrolled into the device  |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
| 原型  | void onIdentifyFailed(in FingerprintError error)  |   
 |  
| --- | --- | --- |  
| 功能  | 识别失败时回调  |   
 |  
| 参数  | Error[in]  | Fingerprint errors: Refer to FingerprintError  |  
| 返回  | None  |   
 |  
| 注意  |   
 |   
 |  
| 原型  | void onIdentifyTimeout()  |  
| --- | --- |  
| 功能  | 识别超时回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onCancelIdentify ()  |  
| --- | --- |  
| 功能  | 识别取消回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onIdentifyHintPress()  |  
| --- | --- |  
| 功能  | 当提示手指按压时 回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
| 原型  | void onIdentifyHintRaise()  |  
| --- | --- |  
| 功能  | 当提示手指移开时 回调  |  
| 参数  | None  |  
| 返回  | None  |  
| 注意  |   
 |  
## 5. 实体类
### 5.1 FingerprintError
Constructor：
public FingerprintError(SupportedFingerprintDevices devices,String reason) {   
  
this.devices = devices;   
  
this.reason = reason;   
  
}
功能：负责跨进程传递指纹错误信息
### 5.2 SupportedFingerprintDevices
Constructor：
private SupportedFingerprintDevices(String manufacturer, String model, String connectType){   
  
this.manufacturer = manufacturer;   
  
this.model = model;   
  
this.connectType = connectType;   
  
}
功能：枚举所有支持的指纹设备信息
### 5.3 FingerprintKeywords
Code：
public class FingerprintKeywords {
//fingerprint
public static final String FINGERPRINT_IMAGE_DATA = "image_data";
public static final String FINGERPRINT_IMAGE_BASE64 = "image_base64";
public static final String FINGERPRINT_IMAGE_FORMAT = "image_format";
public static final String FINGERPRINT_IMAGE_QUALITY = "image_quality";
public static final String FINGERPRINT_USER_INDEX = "user_index";
public static final String FINGERPRINT_USER_NAME = "user_name";
public static final String FINGERPRINT_FEATURE_FORMAT = "feature_format";
public static final String FINGERPRINT_FEATURE_DATA = "feature_data";
public static final String FINGERPRINT_ISO_IMAGE_JPEG2000 = "iso_image_jpeg";
public static final String FINGERPRINT_ISO_IMAGE_WSQ = "iso_image_wsq";
public static final String FINGERPRINT_POSITION = "finger_position";
//device
public static final String DEVICE_ID = "device_id";
public static final String DEVICE_MODEL= "device_model";
public static final String DEVICE_MANUFACTURER = "device_manufacturer";
public static final String DEVICE_SERIALNUMBER = "device_serialnumber";
public static final String DEVICE_PRODUCT = "device_product";
public static final String DEVICE_FIRMWARE_VERSION = "device_firmware_version";
//settings
public static final String TIMEOUT = "timeout";
//api
public static final int API_NOT_SUPPORT = -1;
public static final int UNKNOWN_ERROR = -2;
public st atic final int ALREADY_ENGAGED = -3;
}
功能：定义指纹信息关键字
上一篇：CPad 内置LED指示灯管理
下一篇：商米Android设备间通信连接服务 (ECR) SDK 开发指南
