---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfffeghjk535
---

# 4、回调模式下合作伙伴提供的API接口
更新时间：2026-06-17 17:17:27
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
## **特别声明：**
> 基于合作伙伴数据隐私考虑，【回调模式】商米云不会直接获取打印数据，而是采用消息传递方式，由打印机直连合作伙伴云拉取所需数据进行打印。
> 以下规范了需要合作伙伴云实现的几个基本功能接口，以完成打印机设备获取订单数据进行打印。
> 请求URL地址：为了让打印机设备能够直连合作伙伴云API接口实现订单打印功能，需要事先提供可以正常调用的URL地址给到商米云。
# **1、合作伙伴提供接口URL地址**
此模式下，需要合作伙伴提供打印机可以访问的回调URL地址，并登录“合作伙伴平台”，在关联能力列表选择【云打印】-【设置】中配置【回调模式订单请求地址】。设置说明请参考[云打印能力设置](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xffceghjk502/)
# **2、公共请求sign签名说明**
  * 签名算法：MD5
  * 签名方式：sign=MD5(keyvalue+app_key)


> **签名生成方式举例(此处以打印机绑定接口为例)：  
>   
>  app_id:sm5b9b4daef3463   
>   
> msn:NT1234DF23456   
>   
> timeStamp:1589277365   
>   
> shop_id:1**
> **步骤1：以“key=value”格式，按参数名称ASCII字典顺序排序:**
> **stringA="app_id=sm5b9b4daef3463 &msn=NT1234DF23456&shop_id=1&timeStamp=1589277365"**
> **步骤2：在stringA尾部拼接app_key，假定app_key是“dd3ac24736589ae17d333e362859bf4c”,那么:**
> **stringB=stringA+app_key**
> **stringB="app_id=sm5b9b4daef3463 &msn=NT1234DF23456&shop_id=1&timeStamp=1589277365dd3ac24736589ae17d333e362859bf4c"**
> **步骤3：对stringB进行MD5加密得到最终签名sign，即sign=MD5(stringB)。即sign=“B364A4C3C313EB69F45CB28F445AA4D7** ”
> **以上参数值仅作为签名生成方式举例所用，实际签名生成所需参数值以实际情况为准**
> **基于以上签名生成方式，其中私密参数app_id和app_Key，由商米科技统一分配。**
> **注意：签名sign必须全部32位大写MD5**
# **3、合作伙伴云提供的订单相关接口**
## **1）获取订单列表**
## 接口名：
  * **Path：** `printTicket/getPrintTicketOrderId`
  * **Method：** GET


## 请求参数：
  * **Body**

  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timeStamp  | string  | Unix时间戳（秒）  |  
## 返回数据：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | integer  | 1:代表请求成功，-1:代表请求失败  |  
| data  | object[]  | 返回数据库中订单id集合，每次最多5个id；如果没有订单，则data为null  |  
| msg  | string  | 若请求失败，msg返回error信息，请求成功则msg为””  |  
> 备注：为保证打印不漏单，订单id列表依照时间顺序每次最多返回5条订单id集合列表。打印机会将所有订单id内容依次打印完成后，再请求该接口获取需打印订单id列表，直至该接口返回为空停止。
  * **返回有订单示例一**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,
    "data":["id1","id2","id3","id4","id5"],
    "msg":""
}
  

```

  * **返回无订单示例二**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,
    "data":null,
    "msg":""
}
  

```

  * **返回错误示例三**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":-1,
    "data":null,
    "msg":"error"
}
  

```

## **2）获取订单详情**
## 接口名：
  * **Path：** `printTicket/getPrintTicketInfo`
  * **Method：** GET


## 请求参数：
  * **Body**

  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timeStamp  | string  | Unix时间戳（秒）  |  
| orderId  | string  | 订单id  |  
## 返回数据：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | integer  | 1 :代表请求成功，-1:代表请求失败  |  
| data  | object  | 返回订单内容，若请求失败data为null；请求成功则返回：voiceCnt为语音播报次数，voice为语音播报内容，orderCnt为打印票据张数，data为订单打印数据  |  
| ├─voiceCnt  | integer  | 为语音播报次数  |  
| ├─voice  | string  | 为语音播报内容，voice与voiceUrl内容只能使用一个，无数据时传""  |  
| ├─voiceUrl  | string  | 语音播报文件下载地址，voice与voiceUrl内容只能使用一个，无数据时传""  |  
| ├─orderCnt  | integer  | 为打印票据张数  |  
| └─data  | string  | 为订单打印数据  |  
| msg  | string  | 若请求失败，msg返回error信息，请求成功则msg为””  |  
> 云打印机因为具有重打订单功能，该接口需要允许状态已经标记为打印成功的订单，仍然允许打印机根据订单ID请求订单详情，以完成重打动作。
  * **返回成功示例一**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,
    "data":{
      "voiceCnt":1,  //语音播报次数
      "voice":"语音内容",  //语音播报内容，voice与voiceUrl内容只能使用一个，无数据时传""
      "voiceUrl":"",  //语音播报文件下载地址，voice与voiceUrl内容只能使用一个，无数据时传""。voice是内置语音播报的内容文本。voiceUrl为需要设备播放的语音文件，文件格式只能为MP3或WAV格式（MP3要求16bit单声道8000Hz采样率。WAV要求16bit单声道8000Hz采样率）
      "orderCnt":2,  //打印票据张数
      "data":"1B21306D4B8BD5310A",//使用ESCPOS指令字体倍高宽模式打印“测试1”内容，末尾还有一个回车指令
    },
    "msg":""
}
  

```

  * **返回成功示例二**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,
    "data":{
      "voiceCnt":1,  //语音播报次数
      "voice":"",  //语音播报内容，voice与voiceUrl内容只能使用一个，无数据时传""
      "voiceUrl":"http://xxx.xxx.xxx/playaudio.MP3",  //语音播报文件下载地址，voice与voiceUrl内容只能使用一个，无数据时传""。voice是内置语音播报的内容文本。voiceUrl为需要设备播放的语音文件，文件格式只能为MP3或WAV格式（MP3要求16bit单声道8000Hz采样率。WAV要求16bit单声道8000Hz采样率）
      "orderCnt":2,  //打印票据张数
      "data":"1B21306D4B8BD5310A",//使用ESCPOS指令字体倍高宽模式打印“测试1”内容，末尾还有一个回车指令
    },
    "msg":""
}
  

```

> data说明：打印内容采用十六进制格式传输，所有文本内容采用UTF8编码。在内容中可以直接使用ESC/POS指令集控制打印，以获得所需的票据排版格式。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//data数据PHP代码示例：

$str = chr（27）. chr（33）. chr（48）. "南国超市". chr（27）. chr（33）. chr（0）. chr（0x0A）;  //chr（27）. chr（33）. chr（48）是ESC/POS指令的字体效果倍高宽；chr（27）. chr（33）. chr（0）是ESC/POS指令的字体效果标准大小；0A是换行
$str = $str. "台号:01       工号:01". chr（0x0A）; 
$str = $str. "时间:12:45:28". chr（0x0A）; 
$str = $str. "单号:00554789713585". chr（0x0A）; 
$str = $str. "商品名称     单价数量       金额". chr（0x0A）; 
$str = $str. "--------------------------------". chr（0x0A）; 
$str = $str. "百事利精装糖果  7. 001      7. 00". chr（0x0A）; 
$str = $str. "旺旺雪饼        8. 001      8. 00". chr（0x0A）; 
$str = $str. "可口可乐        2. 501      2. 50". chr（0x0A）; 
$str = $str. "喜之朗果冻     10. 001     10. 00". chr（0x0A）; 
$str = $str. "巧克力饼干     10. 001     10. 00". chr（0x0A）; 
$str = $str. "--------------------------------". chr（0x0A）; 
$str = $str. "总件数:5          总计RMB: 27. 00". chr（0x0A）; 
$str = $str. chr（0x0A）; 
$str = $str. chr（27）. chr（33）. chr（16）. "      谢谢惠顾欢迎下次光临". chr（27）. chr（33）. chr（0）. chr（0x0A）;//显示效果是字体倍高；0A是换行

$data = bin2hex（strToUtf8（$str））;  //$data为：1b2130e58d97e59bbde8b685e5b8821b21000ae58fb0e58fb73a303120202020202020e5b7a5e58fb73a30310ae697b6e997b43a31323a34353a32380ae58d95e58fb73a30303535343738393731333538350ae59586e59381e5908de7a7b020202020202020202020202020e58d95e4bbb72ae695b0e9878f202020202020e98791e9a29d0a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0ae799bee4ba8be588a9e7b2bee8a385e7b396e69e9c2020202020202020372e30302a312020202020202020372e30300ae697bae697bae99baae9a5bc2020202020202020202020202020382e30302a312020202020202020382e30300ae58fafe58fa3e58fafe4b9902020202020202020202020202020322e35302a312020202020202020322e35300ae5969ce4b98be69c97e69e9ce586bb202020202020202020202031302e30302a312020202020202031302e30300ae5b7a7e5858be58a9be9a5bce5b9b2202020202020202020202031302e30302a312020202020202031302e30300a2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d2d0ae680bbe4bbb6e695b02020342e303020202020202020202020e680bbe8aea1524d4220202020202032372e30300a0a1b2110202020202020e8b0a2e8b0a2e683a0e9a1bee6aca2e8bf8ee4b88be6aca1e58589e4b8b41b21000a

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

  * **返回失败示例三**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":-1,
    "data":null,
    "msg":"error"
}
  

```

## **3）更新订单状态**
## 接口名：
  * **Path：** `printTicket/updatePrintTicketStatus`
  * **Method：** GET


## 请求参数：
  * **Body**

  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timeStamp  | string  | Unix时间戳（秒）  |  
| orderId  | string  | 订单id  |  
| status  | int  | 状态 1:订单打印成功、0:订单打印失败、-1:订单json格式错误、-2:data字段内容为空、-3:code或msg字段错误  |  
## 返回数据：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | integer  | 1: 代表请求成功，-1: 代表请求失败  |  
| data  | string  | 成功返回“success”，失败返回“fail”  |  
| msg  | string  | 若请求失败，msg返回error信息，请求成功则msg为””  |  
  * **返回成功示例一**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,
    "data":"success",
    "msg":""
}
  

```

  * **返回失败示例二**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":-1,
    "data":"fail",
    "msg":"error"
}
  

```

##  **4）****扫码核销**
本接口仅在设备具有“扫码模块”且启用“回调模式”时产生回调。在此模式下，扫码窗获取的条码信息将不再通过USB/BT传输至上位机，而仅通过云端传输。
## 接口名：
  * **Path：** `scanner/reportBarcode`
  * **Method：** GET


## 请求参数：
  * **Body**

  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timeStamp  | string  | Unix时间戳（秒）  |  
| barcode  | string  | 扫码器读到UTF-8格式的Base64Url编码条码信息  |  
## 返回数据：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | integer  | 1: 代表请求成功，-1: 代表请求失败。当voiceUrl为“”，code为1请求成功时，默认播报“核销成功”；code为-1请求失败时，默认播报“核销失败”  |  
| data  | object  | 返回核销结果，若请求失败data为null；请求成功则返回如下信息  |  
| ├─voiceCnt  | integer  | 为语音播报次数  |  
| └─voiceUrl  | string  | 语音播报文件下载地址，无数据时传""。文件格式为WAV格式（WAV要求16bit单声道8000Hz采样率）  |  
| msg  | string  | 若请求失败，msg返回error信息，请求成功则msg为””  |  
**说明：同一时间只能处理一条扫码核销任务，在网络没有返回数据前，再次扫码会提示语音“操作太频繁，稍候再试”。如果网络请求异常，会播报“核销失败”。**
  

  * **返回成功示例一 - 语音播报**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,  //播报voiceUrl语音文件
    "data":{
        "voiceCnt":1,  //语音播报次数
        "voiceUrl":"http://xxx.xxx.com/xxx/success1.wav",  //语音文件下载地址
    },
    "msg":""
}
```

  * **返回成功示例二 - 无播报**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":1,   //播报“核销成功”
    "data":{
        "voiceCnt":0,  //语音播报次数
        "voiceUrl":"",  //语音文件下载地址
    },
    "msg":""
}
```

  * **返回失败示例三**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":-1,   //播报“核销失败”
    "data":null,
    "msg":"error"
}
```

  

上一篇：3、开发对接API接口
下一篇：5、回调设备信息API接口
