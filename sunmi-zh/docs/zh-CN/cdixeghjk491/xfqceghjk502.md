---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfqceghjk502
---

# 4、收款音箱状态回调API接口
更新时间：2026-03-18 18:53:48
## **特别声明：**
> 商米云向合作伙伴提供了收款音箱实时状态信息的回调功能，可以将收款音箱的在线/离线信息、设备硬件信息、实时动作信息、寿命信息等回传给合作伙伴进行设备管理。
# **1、合作伙伴提供接口URL地址**
此模式下，需要合作伙伴提供可以访问的回调URL地址，并登录“合作伙伴平台”，在关联能力列表选择【收款音箱】-【设置】中配置【设备状态回调地址】。设置说明请参考[收款音箱能力设置](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfqaeghjk480)
# **2、接口公共信息**
## 公共Header及签名
当回调开发者的接口时，会使用商米Appkey对回调数据进行签名，请参考文档【[回调通知数据的签名方式](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmzeghjk557)】  
| 字段名  | 是否必传  | 描述  |  
| --- | --- | --- |  
| Sunmi-Timestamp  | 是  | 当前时间戳  |  
| Sunmi-Sign  | 是  | 签名内容  |  
| Sunmi-Nonce  | 是  | 6位随机数  |  
| Sunmi-Appid  | 是  | 申请的APPID  |  
| Sunmi-NotifyType  | 是  | 通知类型。CloudSoundBox_1表示设备基础信息；CloudSoundBox_2表示设备状态信息；CloudSoundBox_3表示设备定时信息；CloudSoundBox_4表示设备在线信息  |  
  * 签名算法: **hmac256**
  * 签名方式: `Sunmi-Sign = hmac256( json-body + Sunmi-Appid + Sunmi-Timestamp + Sunmi-Nonce + Sunmi-NotifyType, APPKEY )`


如果结果不是十六进制字符，则需要转成十六进制字符。
# **3、设备状态信息回调接口**
## 接口名：
  * **Path：** `自定义，并在【设备状态回调地址】中填写`
  * **Method：** POST


## 请求参数：
  * **Headers**

  
| 参数名称  | 参数值  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| Content-Type  | application/json  | 是  |   
 |   
 |  
  * **Body**

  
| **名称**  | **类型**  | **是否必须**  | **备注**  |  
| --- | --- | --- | --- |  
| sn  | string  | 必须  | 设备sn号  |  
| report_type  | integer  | 必须  | 设备上报类型。1-设备基础信息；2-设备状态信息；3-设备定时信息；4-设备在线信息；  |  
| **device_base_data**  | **object**  | **非必须**  | **设备基本信息，report_type为1有值，其他时候忽略**  |  
| ├─ model  | string  | 非必须  | 机型  |  
| ├─ hardware_version  | string  | 非必须  | 硬件版本  |  
| ├─ firmware_version  | string  | 非必须  | 固件SDK版本  |  
| ├─ app_version  | string  | 非必须  | 应用版本  |  
| ├─ voice_code  | string  | 非必须  | 语音发音人  |  
| ├─ voice_version  | string  | 非必须  | 语音版本  |  
| └─ voice_language  | integer  | 非必须  | 语音语种。1:普通话；2:英语；  |  
| **device_status_data**  | **object**  | **非必须**  | **设备状态信息，report_type为2有值，其他时候忽略**  |  
| ├─ trade_no  | string  | 非必须  | 播报完毕的播报序号，最大32位，商户下唯一  |  
| ├─ task_id  | string  | 非必须  | 播报完毕“唯一标识id”  |  
| ├─ receive_time  | integer  | 非必须  | 商米云通道接收时间  |  
| ├─ generate_time  | integer  | 非必须  | 商米云数据生成时间  |  
| ├─ push_time  | integer  | 非必须  | 商米云消息推送时间  |  
| ├─ play_time  | integer  | 非必须  | 播报完成时间戳  |  
| ├─ volume_level  | integer  | 非必须  | 音量等级  |  
| ├─ charge_start  | integer  | 非必须  | 充电开始时间戳  |  
| ├─ charge_stop  | integer  | 非必须  | 充电停止时间戳  |  
| ├─ battery_voltage  | string  | 非必须  | 电池电量  |  
| ├─ battery_temp  | string  | 非必须  | 电池温度  |  
| ├─ lng  | string  | 非必须  | 经度  |  
| ├─ lat  | string  | 非必须  | 纬度  |  
| └─ network_type  | integer  | 非必须  | 网络类型。1:Wifi；2:4G  |  
| **device_timing_data**  | **object**  | **非必须**  | **设备定时信息report_type为3有值，其他时候忽略**  |  
| ├─ wifi_mac  | string  | 非必须  | 无线网络mac地址  |  
| ├─ wifi_dhcp_ip  | string  | 非必须  | 无线网络IP地址-动态分配  |  
| ├─ wifi_dhcp_netmask  | string  | 非必须  | 无线网络子网掩码-动态分配  |  
| ├─ wifi_static_ip  | string  | 非必须  | 无线网络IP地址-静态分配  |  
| ├─ wifi_static_netmask  | string  | 非必须  | 无线网络子网掩码-静态分配  |  
| ├─ wifi_static_gateway  | string  | 非必须  | 无线网络网关-静态分配  |  
| ├─ wifi_strength  | integer  | 非必须  | wifi信号强度  |  
| ├─ wifi_ssid  | string  | 非必须  | Wi-Fi名称  |  
| ├─ wifi_password  | string  | 非必须  | Wi-Fi密码  |  
| ├─ strength  | integer  | 非必须  | 移动4G信号强度  |  
| ├─ mnc  | string  | 非必须  | 移动网络号码，用于识别移动用户所归属的移动通信网  |  
| ├─ mcc  | string  | 非必须  | 移动国家编码  |  
| ├─ isp  | string  | 非必须  | 运营商名称  |  
| ├─ apn  | string  | 非必须  | 接入点名称  |  
| ├─ access_type  | string  | 非必须  | 接入网络类型  |  
| ├─ imei  | string  | 非必须  | 国际移动设备身份码IMEI号  |  
| ├─ imsi  | string  | 非必须  | 国际移动用户识别码IMSI号  |  
| ├─ cpsi  | string  | 非必须  | 注册网络信息  |  
| └─ iccid  | string  | 非必须  | SIM卡ICCID号  |  
| **device_online_data**  | **object**  | **非必须**  | **设备在线信息report_type为4有值，其他时候忽略**  |  
| ├─ action  | integer  | 非必须  | 在线状态变更动作：1-上线，2-离线  |  
| ├─ update_at  | integer  | 非必须  | 状态变更时间戳：上线为上线时间戳，离线为离线时间戳（精确到秒）  |  
| ├─ TID  | string  | 非必须  | 受卡机终端标识码  |  
| └─ MID  | string  | 非必须  | 卡接受方识别码  |  
  * **请求示例**   
  
请求body参数：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"ZG03D4B700025",
    "report_type":1,
    "device_timing_data":{
        "model":"NJ021_CN",
        "hardware_version":"EC600G",
        "firmware_version":"0.1.10",
        "app_version":"1.0.2",
        "voice_code":"xiaoli",
        "voice_version":"1.0.2",
        "voice_language":2
    }
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
|   
 | string  | 是  |   
 | 返回"SUCCESS"表示接受成功，返回其他内容全都表示接受失败，会进行重试。  |   
 |  
  * **返回示例**   
  
成功：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
SUCCESS
  

```

上一篇：3、收款音箱API对接
下一篇：5、远程管理收款音箱
