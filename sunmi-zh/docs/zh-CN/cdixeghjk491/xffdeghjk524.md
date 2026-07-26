---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xffdeghjk524
---

# 3、开发对接API接口
更新时间：2026-04-10 17:00:08
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
## **特别声明：**
> 直推方式合作伙伴云的数据会推送到商米云，再由商米云推送给打印机。商米将采用安全的方法保护合作伙伴的数据隐私。
> 如果希望让数据隐私得到更好的保护，可以采用【回调模式】，让打印机从合作伙伴云直接获取数据，避免数据中转。
# **１、接口公共信息**
## 公共Header及签名  
| 字段名  | 默认值  | 是否必传  | 描述  |  
| --- | --- | --- | --- |  
| Sunmi-Timestamp  |   
 | 是  | 10位Unix时间戳  |  
| Sunmi-Sign  |   
 | 是  | 计算后的签名  |  
| Sunmi-Nonce  |   
 | 是  | 6位随机数  |  
| Sunmi-Appid  |   
 | 是  | 分配的appid  |  
| Source  | openapi  | 是  | 传固定值 openapi  |  
  * 签名算法: **hmac256**
  * 签名方式: `Sunmi-Sign = hmac256( json-body + appid + timestamp+ nonce , appkey)`


> 【签名和验签】具体说明[请参考](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmzeghjk557)
  * 签名方式示例


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

## 公共接口URL地址
`https://openapi.sunmi.com`
**协议版本说明**   
为了数据传输安全考虑，针对公共开放的API接口，我们要求使用HTTPS协议。针对HTTPS协议，我们有以下限制：   
1. 对于HTTP协议：支持HTTP1，不支持HTTP2及以上。   
2. 对于TLS/SSL协议：支持TLS1.3、TLS1.2、不支持TLS1.1、TLS1.0、SSL3、SSL2。
## 能力接入错误代码  
| code  | 描述  |  
| --- | --- |  
| 10000  | 请求成功  |  
| 20000  | 网关校验缺少必要参数  |  
| 20001  | 请求超过有效期  |  
| 30000  | 开发者身份验证失败（APPID无效；访问IP不在配置的IP白名单列表）  |  
| 30001  | 用户缺少相关权限（能力未关联）  |  
| 40000  | 签名验证失败  |  
| 50000  | 服务器异常  |  
| 50001  | 网关异常  |  
## 接口错误码  
| code  | 描述  | 错误说明  |  
| --- | --- | --- |  
| 10071400  | parameter error  | 参数错误，请注意参数类型是否错误  |  
| 10071500  | server error  | 服务器错误  |  
| 10071701  | device is unknown  | 未知设备，需要检查SN号是否错误  |  
| 10071702  | device is bound  | 设备已绑定  |  
| 10071703  | device binding exception  | 设备绑定异常  |  
| 10071704  | not belong to this channel  | 该SN号设备不属于本渠道，需要联系销售绑定到贵司渠道下  |  
| 10071705  | order has been push  | 订单已推送，需要确认订单号是否重复  |  
| 10071706  | order is unknown  | 订单未知  |  
| 10071707  | no device or not this channel  | 没有设备或不是此通道  |  
# **２、设备相关接口**
【[Python](https://developer.sunmi.com/docs/preview/zh-CN/fimeghjk546)】、【[JAVA](https://developer.sunmi.com/docs/preview/zh-CN/fizeghjk557)】、【[PHP](https://developer.sunmi.com/docs/read/zh-CN/fideghjk524#h-java-of-the-sample-code)】 DEMO开发示例
## **1）打印机绑定店铺**
## 接口名：
  * **Path：** `/v2/printer/open/open/device/bindShop`
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
 | 设备SN  |   
 |  
| shop_id  | integer  | 必须  |   
 | 店铺id(不超过int64最大值)  | 此处的shop_id是相对于合作伙伴APP中店铺管理而言的店铺id号，并非商米的店铺。因为打印机一般需要绑定到店铺下，才能建立店铺和设备的对应关系，设备查询时也要依据店铺来查询设备的联网状态。这个shop_id由合作伙伴进行管理，传什么值商米就保存什么值。   
  
shop_id是对合作伙伴名下设备进行分组管理的唯一条件，查询店铺设备状态列表时使用。若无店铺需关联可传””，那么设备状态列表结果集将只返回shop_id为空的设备，不会返回shop_id不为空的设备。  |  
  * **请求示例**   
  
请求body参数：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "shop_id":2441,
    "sn":"N302LDY000353"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | null  | 必须  |   
 | 数据  |   
 |  
  * **返回示例**   
  
绑定成功：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": null
}
  

```

绑定失败：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071704,
    "msg": "not belong to this channel",
    "data": null
}
  

```

## **2）打印机解绑店铺**
## 接口名：
  * **Path：** `/v2/printer/open/open/device/unbindShop`
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
 | 设备SN  |   
 |  
| shop_id  | integer  | 必须  |   
 | 店铺id(不超过int64最大值)  | 合作伙伴名下的商户Id ，需要与绑定时一致  |  
  * **请求示例**   
  
请求body参数：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "shop_id":2441,
    "sn":"N302LDY000353"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | null  | 必须  |   
 |   
 |   
 |  
  * **返回示例**
绑定成功：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": null
}
  

```

绑定失败：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071704,
    "msg": "not belong to this channel",
    "data": null
}
  

```

## **3）查询渠道下所有设备或者单台设备在线状态**
## 接口名：
  * **Path：** `/v2/printer/open/open/device/onlineStatus`
  * **Method：** POST


## 请求参数：
  * **Headers**

  
| 参数名称  | 参数值  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| Content-Type  | application/json  | 是  |   
 |   
 |  
  * **Body**

  
| 名称  | 类型  | 是否必须  | 默认值  | **备注**  |  
| --- | --- | --- | --- | --- |  
| page_no  | integer  | 非必须  | 1  | 页码，正整数，数字非必须，可不传或者0。sn和shop_id为空时必传。  |  
| page_size  | integer  | 非必须  | 100  | 每页数量，正整数，数字非必须，可不传或者0。sn和shop_id为空时必传。  |  
| shop_id  | integer  | 非必须  |   
 | 店铺id(不超过int64最大值)，数字非必须。sn为空时，查询shop_id下的所有sn在线状态。  |  
| sn  | string  | 非必须  |   
 | 设备SN，字符串非必须，可不传或者空字符串。page_no和page_size为空时必传。  |  
  * **请求示例一**


请求body参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "page_no":1,
    "page_size":3
}
  

```

  * **请求示例二**


请求body参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N302LDY000353"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| message  | string  | 必须  |   
 |   
 |   
 |  
| data  | object  | 必须  |   
 | 数据  |   
 |  
| ├┬ list  | object []  | 必须  |   
 |   
 | item 类型: object  |  
| │├─ sn  | string  | 必须  |   
 | 设备SN  |   
 |  
| │└─ is_online  | integer  | 必须  |   
 | 是否在线：0-离线，1-在线  |   
 |  
| └┬ page  | object  | 必须  |   
 |   
 |   
 |  
|  ├─ total  | integer  | 必须  |   
 | 符合条件的设备总数  |   
 |  
|  ├─ page_no  | integer  | 必须  |   
 | 当前页码  |   
 |  
|  └─ page_size  | integer  | 必须  |   
 | 每页数量  |   
 |  
  * **示例一返回示例**
请求成功：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": {
        "list": [
            {
                "sn": "N301203540661",
                "is_online": 0
            },
            {
                "sn": "N301203UU0024",
                "is_online": 0
            },
            {
                "sn": "N301P9WZC1005",
                "is_online": 0
            }
        ],
        "page": {
            "total": 35,
            "page_no": 1,
            "page_size": 3
        }
    }
}
  

```

  * **示例二返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": {
        "list": [
            {
                "sn": "N302LDY000353",
                "is_online": 0
            }
        ],
        "page": {
            "total": 1,
            "page_no": 1,
            "page_size": 1
        }
    }
}
  

```

## **4）清除云端缓存的打印队列**
## 接口名：
  * **Path：** `/v2/printer/open/open/device/clearPrintJob`
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
 | 设备SN  |   
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
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | null  | 必须  |   
 | 数据  |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": null
}
  

```

请求失败：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071701,
    "msg": "device is unknown",
    "data": null
}
  

```

## **5）给打印机推送播报语音**
本功能仅支持以下机型：
58票据云打印机，Model：NT21x；
80后厨云打印机，Model：NT31x；
如需升级请联系客服！
## 接口名：
  * **Path：** `/v2/printer/open/open/device/pushVoice`
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
 | 设备SN  |   
 |  
| content  | string  | 非必须  |   
 | 播放的语音字符串内容，content和media_url二选一  |   
 |  
| media_url  | string  | 非必须  |   
 | 播放的语音文件下载路径，content和media_url二选一  | 文件格式只能为MP3或WAV格式（MP3要求16bit单声道8000Hz采样率。WAV要求16bit单声道8000Hz采样率）  |  
| expire_in  | integer  | 非必须  |   
 | 有效时长（秒）,默认值300  | 当设备网络故障无法获取本推送时，让语音消息在服务上保存的有效时间，避免超时语音继续被推送到设备上。  |  
| cycle  | integer  | 非必须  |   
 | 语音播放次数，999 代表无限播放，默认值1  |   
 |  
| interval  | integer  | 非必须  |   
 | 播放间隔时间（秒），默认 2  |   
 |  
  * **请求示例**


请求body参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N302LDY000353",
    "content":"hello",
    "media_url":"",
    "expire_in":300,
    "cycle": 2,
    "interval":3
}
  

```

请求body参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N302LDY000353",
    "content":"",
    "media_url":"http://www.xxx.com/aaa/a.wav",
    "expire_in":300,
    "cycle": 2,
    "interval":3
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | null  | 必须  |   
 |   
 |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": null
}
  

```

请求异常：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071701,
    "msg": "device is unknown",
    "data": null
}
  

```

## **6）查询订单打印状态**
## 接口名：
  * **Path：** `/v2/printer/open/open/ticket/printStatus`
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
| trade_no  | string  | 必须  |   
 | 商户单号，最大32位，商户下唯一  | 推送订单唯一ID，每张打印票据ID不同就认为是不同的打印内容  |  
  * **请求示例 ：**


请求body参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "trade_no":"3433134"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | object  | 必须  |   
 |   
 |   
 |  
| ├─ sn  | string  | 必须  |   
 | 打印设备SN  |   
 |  
| ├─ is_print  | integer  | 必须  |   
 | 是否打印：0-未打印，1-已打印，2-已删除  |   
 |  
| └─ print_time  | integer  | 必须  |   
 | 打印成功的时间戳，如果未打印或者已删除返回0  |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": {
        "sn": "",
        "is_print": 2,
        "print_time": 1639621476
    }
}
  

```

请求异常：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071706,
    "msg": "order is unknown",
    "data": null
}
  

```

# **３、直推模式接口**
## **1）给打印机推送订单详情**
## 接口名：
  * **Path：** `/v2/printer/open/open/device/pushContent`
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
| trade_no  | string  | 必须  |   
 | 商户单号，最大32位，商户下唯一  | 推送订单唯一ID，每张打印票据ID不同就认为是不同的打印内容  |  
| sn  | string  | 必须  |   
 | 设备SN  |   
 |  
| order_type  | integer  | 非必须  |   
 | 打印票据类型，1 新订单，2 取消订单，3 催单，4 退单，5 其它  |   
 |  
| content  | string  | 必须  |   
 | 打印内容  | 打印票据内容。所有文本内容采用UTF8编码，再转换成十六进制格式传输。在内容中可以直接使用ESC/POS指令集控制打印，以获得所需的票据排版格式。  |  
| count  | integer  | 必须  |   
 | 打印票据张数  |   
 |  
| media_text  | string  | 非必须  |   
 | 播放的语音字符串内容，media_text和media_url二选一  |   
 |  
| media_url  | string  | 非必须  |   
 | 播放的语音文件下载路径，media_text和media_url二选一  | 文件格式只能为MP3或WAV格式（MP3要求16bit单声道8000Hz采样率。WAV要求16bit单声道8000Hz采样率）  |  
| cycle  | integer  | 非必须  |   
 | 语音播放次数，999 代表无限播放，默认值1  |   
 |  
打印内容“content”转换示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//PHP代码示例：  

$str = chr（27）. chr（33）. chr（48）."南国超市".chr（27）. chr（33）. chr（0）. chr（0x0A）;  //chr（27）. chr（33）. chr（48）是ESC/POS指令的字体效果倍高宽；chr（27）. chr（33）. chr（0）是ESC/POS指令的字体效果标准大小；0A是换行
$str = $str."台号:01       工号:01".chr（0x0A）;
$str = $str."时间:12:45:28".chr（0x0A）;
$str = $str."单号:00554789713585".chr（0x0A）;
$str = $str."商品名称     单价*数量       金额".chr（0x0A）;
$str = $str."--------------------------------".chr（0x0A）;
$str = $str."百事利精装糖果  7.00*1      7.00".chr（0x0A）;
$str = $str."旺旺雪饼        8.00*1      8.00".chr（0x0A）;
$str = $str."可口可乐        2.50*1      2.50".chr（0x0A）;
$str = $str."喜之朗果冻     10.00*1     10.00".chr（0x0A）;
$str = $str."巧克力饼干     10.00*1     10.00".chr（0x0A）;
$str = $str."--------------------------------".chr（0x0A）;
$str = $str."总件数:5          总计RMB: 27.00".chr（0x0A）;
$str = $str.chr（0x0A）;
$str = $str.chr（27）. chr（33）. chr（16）. "      谢谢惠顾欢迎下次光临". chr（27）. chr（33）. chr（0）. chr（0x0A）;//显示效果是字体倍高；0A是换行

$orderData = bin2hex（strToUtf8（$str））;  //$orderData为：1b2130e58d97e59bbde8b685e5b8821b21000ae58fb0e58fb73a303120202020202020e5b7a5e58fb73a30310ae697b6e997b43a31323a34353a32380ae58d95e58fb73a30303535343738393731333538350ae59586e59381e5908de7a7b020202020202020202020202020e58d95e4bbb72ae695b0e9878f202020202020e98791e9a29d0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0ae799bee4ba8be588a9e7b2bee8a385e7b396e69e9c2020202020202020372e30302a312020202020202020372e30300ae697bae697bae99baae9a5bc2020202020202020202020202020382e30302a312020202020202020382e30300ae58fafe58fa3e58fafe4b9902020202020202020202020202020322e35302a312020202020202020322e35300ae5969ce4b98be69c97e69e9ce586bb202020202020202020202031302e30302a312020202020202031302e30300ae5b7a7e5858be58a9be9a5bce5b9b2202020202020202020202031302e30302a312020202020202031302e30300a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0ae680bbe4bbb6e695b02020342e303020202020202020202020e680bbe8aea1524d4220202020202032372e30300a0a1b2110202020202020e8b0a2e8b0a2e683a0e9a1bee6aca2e8bf8ee4b88be6aca1e58589e4b8b41b21000a

//该方法判断字符串内容是否是UTF8编码，如果是直接返回，如果不是则将字符串转换为UTF8编码
function strToUtf8（$str）{
    $encode = mb_detect_encoding（$str, array（"ASCII",'UTF-8',"GB2312","GBK",'BIG5'））;
    if（$encode == 'UTF-8'）{
        return $str;
    }else{
        return mb_convert_encoding（$str, 'UTF-8', $encode）;
    }
}
  

```

  * **请求示例**


请求body参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "sn":"N302LDY000353",
    "trade_no":"3433135",
    "count":2,
    "content":"1b2130e58d97e59bbde8b685e5b882"
}
  

```

## 返回数据：  
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | object  | 必须  |   
 |   
 |   
 |  
| └ trade_no  | string  | 必须  |   
 | 商户单号  |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": {
        "trade_no": "3433135"
    }
}
  

```

请求异常：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071500,
    "msg": "server error",
    "data": null
}
  

```

# **４、回调模式接口**
## **1）给打印机推送有新订单消息**
## 接口名：
  * **Path：** `/v2/printer/open/open/ticket/newTicketNotify`
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
 | 设备SN  |   
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
| 名称  | 类型  | 是否必须  | 默认值  | 备注  | 其他信息  |  
| --- | --- | --- | --- | --- | --- |  
| code  | integer  | 必须  |   
 | 状态码  |   
 |  
| msg  | string  | 必须  |   
 | 消息  |   
 |  
| data  | null  | 必须  |   
 | 数据  |   
 |  
  * **返回示例**


请求成功：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": null
}
  

```

请求异常：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 10071701,
    "msg": "device is unknown",
    "data": null
}
  

```

  
  

上一篇：2、开通云打印对接能力
下一篇：4、回调模式下合作伙伴提供的API接口
