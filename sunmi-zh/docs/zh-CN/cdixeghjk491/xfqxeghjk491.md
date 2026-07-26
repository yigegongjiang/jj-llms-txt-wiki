---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfqxeghjk491
---

# 3、收款音箱API对接
更新时间：2026-05-08 12:24:37
## **声明:**
> 商米将采用安全的方法保护合作伙伴的数据隐私，接口调用过程不会涉及实体金额交易，仅实现播报语音信息传递。
# **1、API 公共信息**
## **请求头与签名**  
| 字段名  | 默认值  | 是否必传  | 说明  |  
| --- | --- | --- | --- |  
| Sunmi-Timestamp  |   
 | 是  | 10位数Unix时间戳  |  
| Sunmi-Sign  |   
 | 是  | 计算后的签名  |  
| Sunmi-Nonce  |   
 | 是  | 6位随机数字  |  
| Sunmi-Appid  |   
 | 是  | 应用的appid  |  
| Source  | openapi  | 是  | 固定值“openapi”  |  
  * 签名算法: **hmac256**
  * 签名方式: `Sunmi-Sign = hmac256( json-body + appid + timestamp+ nonce , appkey)`


> 【签名与验证】: [请参考](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmzeghjk557)
  * 签名方式PHP示例


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
var ts = Math.round(new Date() / 1000);
var appid = "";  // 写入应用的appid
var appkey = ""; // 写入应用的appkey
var nonce = "123456";
var jsonBody = pm.request.body.raw;
// console.log("raw:",raw)
var signStr = jsonBody+appid+ts+nonce;
var sign = CryptoJS.HmacSHA256(signStr, appkey).toString();
// console.log("signStr:", signStr);
// console.log("sign:", sign);

pm.request.headers.add({
    key: 'Sunmi-Appid', 
    value: appid
});
pm.request.headers.add({
    key: 'Sunmi-Nonce', 
    value: nonce
});
pm.request.headers.add({
    key: 'Sunmi-Timestamp', 
    value: ts
});
pm.request.headers.add({
    key: 'Sunmi-Sign', 
    value: sign
});
pm.request.headers.add({
    key: 'Source', 
    value: 'openapi'
});
  

```

  * 签名方式Golang示例


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
package main

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"log"
	"strconv"
	"time"
)

func main() {
	appid := "your appid"
	appkey := "your appkey"
	jsonBody := "json body"
	nonce := "123abc"
	ts := time.Now().Unix()
	hash := hmac.New(sha256.New, []byte(appkey))
	hash.Write([]byte(jsonBody + appid + strconv.Itoa(int(ts)) + nonce))
	sign := hex.EncodeToString(hash.Sum(nil))
	log.Println("sign:", sign) // sign:78cb4e4d6b6a16dc427030755ead273105eb7f4ed2e14ca51f95ad25ec8d83e2
}
  

```

## 公共接口 URL
`https://openapi.sunmi.com`
**协议版本说明**   
  
为了数据传输安全考虑，针对公共开放的API接口，我们要求使用HTTPS协议。针对HTTPS协议，我们有以下限制：   
  
1. 对于HTTP协议：支持HTTP1，不支持HTTP2及以上。   
  
2. 对于TLS/SSL协议：支持TLS1.3、TLS1.2、不支持TLS1.1、TLS1.0、SSL3、SSL2。
## **返回公共Header及签名**  
| 字段名  | 是否必传  | 说明  |  
| --- | --- | --- |  
| Sunmi-Timestamp  | 是  | 10位数Unix时间戳  |  
| Sunmi-Sign  | 是  | 计算后的签名  |  
| Sunmi-Nonce  | 是  | 6位随机数字  |  
| Sunmi-Appid  | 是  | 应用的appid  |  
  * 签名方式: **hmac256**
  * 签名算法: `Sunmi-Sign = hmac256( json-body + appid + timestamp+ nonce , appkey)`


> 【签名与验证】: [请参考](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmzeghjk557)
## 返回能力接入错误代码  
| Code  | 错误说明  |  
| --- | --- |  
| 10000  | 请求成功  |  
| 20000  | 网关校验缺少必要参数  |  
| 20001  | 请求超过有效期  |  
| 30000  | 开发者验证失败 (APPID 错误; IP地址不在IP白名单里)  |  
| 30001  | 用户缺失权限（能力未联接）  |  
| 40000  | 签名验证失败  |  
| 50000  | 服务器错误  |  
| 50001  | 网关错误  |  
## 返回接口错误代码  
| code  | 描述  | 错误说明  |  
| --- | --- | --- |  
| 1  | success  | 请求成功  |  
| 10071400  | parameter error  | 参数错误，请注意参数类型是否错误  |  
| 10071401  | header verify failed  | 请求头验证失败  |  
| 10071500  | server error  | 服务器错误  |  
| 10071701  | device is unknown  | 未知设备，需要检查SN号是否错误  |  
| 10071702  | device is already bound  | 设备已绑定  |  
| 10071703  | device hardware not support  | 设备不支持  |  
| 10071704  | not belong to this channel  | 该SN号设备不属于本渠道，需要联系销售绑定到贵司渠道下  |  
| 10071705  | order has been push  | 消息已推送，需要确认是否重复  |  
| 10071707  | no device or not this channel  | 没有设备或不是此通道  |  
| 10071710  | payment box is unbind  | 设备未绑定  |  
| 10071711  | out of max amount  | 超出最大金额  |  
# **２、设备相关接口**
## **1）绑定收款音箱**
## 接口名：
  * **Path：** `/v2/iot/paymentbox/device/enablePaymentBox`
  * **Method：** POST


## 请求参数：
  * **Headers**

  
| 参数名称  | 参数值  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| Content-Type  | application/json  | 是  |   
 |   
 |  
  * **Body**

  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| sn  | string  | 必须  | 设备SN  |   
 |  
  * **请求示例**   
  
请求body参数：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N302LDY000353"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| code  | integer  | 必须  | 状态码  |   
 |  
| msg  | string  | 必须  | 消息  |   
 |  
| data  | object  | 非必须  | 返回数据  |   
 |  
  * **返回示例**   
  
绑定成功：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success"
}
  

```

绑定失败：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071704,
    "msg": "not belong to this channel"
}
  

```

## **2）解绑收款音箱**
## 接口名：
  * **Path：** `/v2/iot/paymentbox/device/disablePaymentBox`
  * **Method：** POST


## 请求参数：
  * **Headers**

  
| 参数名称  | 参数值  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| Content-Type  | application/json  | 是  |   
 |   
 |  
  * **Body**

  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| sn  | string  | 必须  | 设备SN  |   
 |  
  * **请求示例**   
  
请求body参数：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N302LDY000353"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| code  | integer  | 必须  | 状态码  |   
 |  
| msg  | string  | 必须  | 消息  |   
 |  
| data  | object  | 非必须  | 返回数据  |   
 |  
  * **返回示例**
绑定成功：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success"
}
  

```

绑定失败：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071704,
    "msg": "not belong to this channel"
}
  

```

## **3）推送播报语音**
## 接口名：
  * **Path：** `/v2/iot/paymentbox/device/pushPaymentVoice`
  * **Method：** POST


## 请求参数：
  * **Headers**

  
| 参数名称  | 参数值  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| Content-Type  | application/json  | 是  |   
 |   
 |  
  * **Body**

  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| sn  | string  | 必须  | 设备SN  |   
 |  
| trade_no  | string  | 必须  | 播报序号（最大32位）  | 当播报序号不同时，就会产生一次全新播报内容。播报序号与实际订单号无关，即同一个订单号需要播报两次时也需要变更trade_no。  |  
| broadcast_type  | integer  | 必须  | 播报类型。1、收款；2、退款；3、取消支付；4、准备支付  | 当播报类型为3和4时，仅播报蜂鸣提示音，没有金额播报。  |  
| currency_code  | string  | 必须  | 播报货币代码（CNY、USD、VND、SGD）  | 播报金额时，将金额数字转换为对应的货币单位语法。比如：(1)设备设置为“普通话”播报时，CNY 1.23，读“壹点贰叁元”；USD 1.23，读“壹点贰叁美元；SGD 1.23，读“壹新币二十三仙” ；  
  
(2)设置为“英语”播报时，CNY 1.23，读“one point two three yuan”；USD 1.23，读“one dollars twenty Three cents”  |  
| amount  | integer  | 必须  | 播放金额（CNY、USD最小单位为分；VND最小单位为盾）  | 播放金额不能带有小数点，云端会自动添加金额单位进行播报。当broadcast_type=3或4时，仅播报蜂鸣提示音，没有金额播报，所以amount=0即可。  |  
| cycle  | integer  | 必须  | 语音播放次数，999 代表无限播放，默认值1  | 当broadcast_type=3或4时，仅播报蜂鸣提示音，没有金额播报，所以cycle=1即可。  |  
| interval  | integer  | 必须  | 播放间隔时间（毫秒）  | 当broadcast_type=3或4时，仅播报蜂鸣提示音，没有金额播报，所以interval=0即可。  |  
| led_status  | integer  | 必须  | 交易指示灯类型。0 所有灯灭，1 蓝灯长亮，2蓝灯闪烁，3 红灯长亮，4 红灯闪烁  | 部分型号设备不支持交易灯功能，该值可默认传0。  |  
| led_time  | integer  | 必须  | 交易指示灯亮灯持续时间（毫秒）  | 部分型号设备不支持交易灯功能，该值可默认传0。  |  
|   
 |   
 |   
 |   
 |   
 |  
  * **请求示例**


请求body参数：
播报类型：收款
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
	"sn": "N302LDY000353",
	"trade_no": "SP2023111200001",
	"broadcast_type": 1,
    "currency_code": "USD",
    "amount": 1250,
	"cycle": 1,
	"interval": 500,
	"led_status": 0,
	"led_time": 0
}
  

```

播报类型：准备支付
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
	"sn": "N302LDY000353",
	"trade_no": "SP2023111200002",
	"broadcast_type": 4,
    "currency_code": "USD",
    "amount": 0,
	"cycle": 1,
	"interval": 0,
	"led_status": 0,
	"led_time": 0
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| code  | integer  | 必须  | 状态码  |   
 |  
| msg  | string  | 必须  | 消息  |   
 |  
| data  | object  | 非必须  | 返回数据  |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success"
}
  

```

请求异常：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071701,
    "msg": "device is unknown"
}
  

```

## **4）推送语音文件**
## 接口名：
  * **Path：** `/v2/iot/paymentbox/device/sendAudioMedia`
  * **Method：** POST


## 请求参数：
  * **Headers**

  
| 参数名称  | 参数值  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| Content-Type  | application/json  | 是  |   
 |   
 |  
  * **Body**

  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| sn  | string  | 必须  | 设备SN  |   
 |  
| trade_no  | string  | 必须  | 播报序号（最大32位）  | 当播报序号不同时，就会产生一次全新播报内容。播报序号与实际订单号无关，即同一个订单号需要播报两次时也需要变更trade_no。  |  
| media_url  | string  | 必须  | 播放的语音文件下载路径  | 文件格式只能为MP3格式（MP3要求16bit单声道16000Hz采样率），文件大小要求小于500KB  |  
| audio_md5  | string  | 必须  | 语音文件的md5  | 播放的语音文件MD5哈希值  |  
| start_date  | date  | 必须  | 循环播报开始日期  | 指定日期范围，时间按格林威治时间计算，不区分时区  |  
| end_date  | date  | 必须  | 循环播报结束日期  | 指定日期范围，时间按格林威治时间计算，不区分时区  |  
| start_time  | time  | 必须  | 循环播报开始时间  | 在指定日期范围内，每天计算任务执行的时间点，时间按格林威治时间计算，不区分时区  |  
| end_time  | time  | 必须  | 循环播报结束时间  | 在指定日期范围内，每天计算任务执行的时间点，时间按格林威治时间计算，不区分时区  |  
| interval_second  | integer  | 必须  | 循环播报频率的间隔时间  | 每次播报的时间间隔，按照秒计算  |  
| status_type  | integer  | 必须  | 指示灯类型。0 所有灯灭，1 蓝灯长亮，2蓝灯闪烁，3 红灯长亮，4 红灯闪烁  | 通过蓝灯，红灯表示客户支付的状态是否成功  |  
| time_sec  | integer  | 必须  | 指示灯亮灯持续时间（毫秒）  |   
 |  
  * **请求示例**


请求body参数：播报声音文件示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "trade_no": "SP2023111200002",
    "sn": "N302LDY000353",
    "media_url": "http://www.xxx.com/aaa/a.mp3",
    "audio_md5": "xxxxxxxxxxxxxxxx",
    "start_date": "2024-10-01",
    "end_date": "2024-10-31",
    "start_time": "09:00",
    "end_time": "17:00",
    "interval_second": 600,
    "status_type": 1,
    "time_sec": 5000
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- |  
| code  | integer  | 必须  | 状态码。1表示成功，其他错误请接口错误码表  |   
 |  
| msg  | string  | 必须  | 消息  |   
 |  
| data  | object  | 非必须  | 返回数据  |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success"
}
  

```

请求异常：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071701,
    "msg": "device is unknown"
}
  

```

上一篇：2、开通收款音箱对接能力
下一篇：4、收款音箱状态回调API接口
