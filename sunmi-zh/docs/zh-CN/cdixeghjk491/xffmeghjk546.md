---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xffmeghjk546
---

# 5、回调设备信息API接口
更新时间：2025-11-11 15:52:04
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
## **特别声明：**
> 商米云向合作伙伴提供了云打印实时状态信息的回调功能，可以将打印机的在线/离线信息、设备硬件信息、实时动作信息、寿命信息等回传给合作伙伴进行设备管理。
# **1、合作伙伴提供接口URL地址**
此模式下，需要合作伙伴提供打印机可以访问的回调URL地址，并登录“合作伙伴平台”，在关联能力列表选择【云打印】-【设置】中配置【设备状态回调地址】。设置说明请参考[云打印能力设置](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xffceghjk502/)
# **2、接口公共信息**
## 公共Header及签名
当回调开发者的接口时，会使用商米Appkey对回调数据进行签名，请参考文档【[回调通知数据的签名方式](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmzeghjk557)】  
| 字段名  | 是否必传  | 描述  |  
| --- | --- | --- |  
| Sunmi-Timestamp  | 是  | 当前时间戳  |  
| Sunmi-Sign  | 是  | 签名内容  |  
| Sunmi-Nonce  | 是  | 6位随机数  |  
| Sunmi-Appid  | 是  | 申请的APPID  |  
| Sunmi-NotifyType  | 是  | 通知类型。Cloud Printed_1表示打印机基础信息；Cloud Printed_2表示打印机转态信息；Cloud Printed_3表示打印机定时信息；Cloud Printed_4表示打印机在线信息  |  
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

  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| sn  | string  | 必须  |   
 | 设备sn号  |   
 |  
| report_type  | integer  | 必须  |   
 | 设备上报类型，1-打印机基础信息 2-打印机状态信息 3-打印机定时信息 4-打印机在线信息  |   
 |  
| device_base_data  | object  | 非必须  |   
 | 设备基本信息，report_type为1有值，其他时候忽略  |   
 |  
| ├─ model  | string  | 非必须  |   
 | 机型  |   
 |  
| ├─ imei  | string  | 非必须  |   
 | IMEI号  |   
 |  
| ├─ app_version  | string  | 非必须  |   
 | 应用版本  |   
 |  
| ├─ hardware_version  | string  | 非必须  |   
 | 硬件版本  |   
 |  
| ├─ boot_loader_version  | string  | 非必须  |   
 | bootloader版本  |   
 |  
| ├─ firmware_version  | string  | 非必须  |   
 | 固件版本  |   
 |  
| ├─ font_library_version  | string  | 非必须  |   
 | 字库版本  |   
 |  
| ├─ tts_version  | string  | 非必须  |   
 | TTS版本  |   
 |  
| ├─ tts_language  | string  | 非必须  |   
 | TTS语言  |   
 |  
| ├─ iccid  | string  | 非必须  |   
 | ICCID号  |   
 |  
| └─ boot_mac  | string  | 非必须  |   
 | MAC地址  |   
 |  
| device_status_data  | object  | 非必须  |   
 | 设备状态信息，report_type为2有值，其他时候忽略  |   
 |  
| ├─ lack_paper_count  | integer  | 非必须  |   
 | 缺纸次数  |   
 |  
| ├─ hit_knife_count  | integer  | 非必须  |   
 | 撞刀次数  |   
 |  
| ├─ failed_cutting_count  | integer  | 非必须  |   
 | 切纸失败次数  |   
 |  
| ├─ clip_knife_count  | integer  | 非必须  |   
 | 卡刀次数  |   
 |  
| ├─ open_count  | integer  | 非必须  |   
 | 开盖次数  |   
 |  
| ├─ paper_blocking_count  | integer  | 非必须  |   
 | 堵纸次数  |   
 |  
| ├─ not_take_paper_count  | integer  | 非必须  |   
 | 久未取纸次数  |   
 |  
| ├─ paper_will_end_count  | integer  | 非必须  |   
 | 纸将尽次数  |   
 |  
| ├─ lng  | float64  | 非必须  |   
 | 经度  |   
 |  
| ├─ lat  | float64  | 非必须  |   
 | 纬度  |   
 |  
| ├─ new_order_play_time  | integer  | 非必须  |   
 | 新订单播报次数 。默认值：1  |   
 |  
| ├─ new_order_contents  | string  | 非必须  |   
 | 新订单播报内容 ：0-来源播报 ，1-内容播报 ，2-金额播报 ，3-备注播报。默认值：“0,3”  |   
 |  
| ├─ cancel_order_play_time  | integer  | 非必须  |   
 | 取消单播报次数。默认值：1  |   
 |  
| ├─ cancel_order_contents  | string  | 非必须  |   
 | 取消单播报内容 ：0-来源播报，1-内容播报 ，2-金额播报 ，3-备注播报。默认值：“0,3"  |   
 |  
| ├─ rush_order_play_time  | integer  | 非必须  |   
 | 催单播报次数。默认值：1  |   
 |  
| ├─ voice_connect  | integer  | 非必须  |   
 | 网络连接语音播报次数。默认值：1  |   
 |  
| ├─ voice_disconnect  | integer  | 非必须  |   
 | 网络断开语音播报次数。默认值：0  |   
 |  
| ├─ inspection_page_language  | integer  | 非必须  |   
 | 自检页语言 0 -简体中文 ，1-英文。默认值：0  |   
 |  
| ├─ density  | integer  | 非必须  |   
 | 打印浓度（百分制）。默认值：100  |   
 |  
| ├─ new_order_copy  | integer  | 非必须  |   
 | 新订单打印联数。默认值：1  |   
 |  
| ├─ cancel_order_copy  | integer  | 非必须  |   
 | 取消单打印联数。默认值：1  |   
 |  
| ├─ dialect  | integer  | 非必须  |   
 | 语音方言：0-普通话男声 ，1-普通话女声 ，2-四川话女声 ，3-粤语女声,4-英语。默认值：1  |   
 |  
| └─ voice_speed  | integer  | 非必须  |   
 | 语音播报速度（百分制）。默认值：50  |   
 |  
| device_timing_data  | object  | 非必须  |   
 | 设备定时信息report_type为3有值，其他时候忽略  |   
 |  
| ├─ print_head_broken_count  | string  | 非必须  |   
 | 打印头坏点数  |   
 |  
| ├─ printed_kilometre  | integer  | 非必须  |   
 | 打印公里数  |   
 |  
| ├─ open_money_box_count  | integer  | 非必须  |   
 | 打开钱箱次数  |   
 |  
| ├─ wifi_strength  | integer  | 非必须  |   
 | wifi信号强度  |   
 |  
| ├─ gprs_strength  | integer  | 非必须  |   
 | gprs信号强度（蜂窝数据信号强度）  |   
 |  
| └─ network_type  | integer  | 非必须  |   
 | 网络类型，0-表示未连接， 1-wifi, 2-lan，3-gprs，4-mobile(lan表有线网络，gprs表示2g网络，mobile表示移动网络)  |   
 |  
| device_online_data  | object  | 非必须  |   
 | 设备在线信息report_type为4有值，其他时候忽略  |   
 |  
| ├─ action  | integer  | 非必须  |   
 | 在线状态变更动作：1-上线，2-离线  |   
 |  
| └─ update_at  | integer  | 非必须  |   
 | 状态变更时间戳：上线为上线时间戳，离线为离线时间戳（精确到秒）  |   
 |  
  * **请求示例**   
  
请求body参数：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N301P9WZC1005",
    "report_type":3,
    "device_timing_data":{
        "print_head_broken_count":"00000000000000000000000000000000",
        "printed_kilometre":0,
        "open_money_box_count":1,
        "wifi_strength":0,
        "gprs_strength":0,
        "network_type":2
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

上一篇：4、回调模式下合作伙伴提供的API接口
下一篇：6、远程管理云打印机
