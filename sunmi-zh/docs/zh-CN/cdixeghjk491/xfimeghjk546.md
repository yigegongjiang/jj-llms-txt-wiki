---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfimeghjk546
---

# 5.开发对接API接口（直推方式）（仅适用于使用了SDK_V1接口的旧设备）
更新时间：2025-11-10 16:33:55
5. 开发对接API接口（方案二：直推方式）
特别声明：
> 直推方式合作伙伴云的数据会推送到商米云，再由商米云推送给打印机。商米将采用安全的方法保护合作伙伴的数据隐私。如果希望让数据隐私得到更好的保护，可以采用“方法一：回调方式”让打印机从合作伙伴云直接获取数据，避免数据中转。
#### 参数说明：
  1. 每个请求的参数包含两部分：公共参数和私有参数，详细请看以下对应说明。
  2. 传参方式为key-value方式，请求头加上“application/x-www-form-urlencoded”。
  3. 以下每个接口只列出私有参数和返回结果，其他请看请求和公共参数说明。
  4. 返回结果皆为json格式数据。


#### 请求公共参数说明（所有请求都必须传递的参数）：  
| 请求参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考下面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id，申请开通通道时获得  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 返回公共参数说明：  
| 返回参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | int  | 返回码，参见常见错误码列表  |  
| data  | 不指定  | 数据类型和内容详见私有返回参数data，如果有错误，返回null  |  
| msg  | string  | 结果提示信息，如果有错误返回错误信息描述  |  
#### 公共请求参数sign生成说明：
#### 签名算法：MD5
> 签名生成方式举例(此处以打印机绑定接口为例)：   
>   
> app_id:sm5b9b4daef3463   
>   
> msn:NT1234DF23456   
>   
> timestamp:1589277365   
>   
> shop_id:1
> 步骤1：以“key=value”格式，按参数名称ASCII字典顺序排序:   
>   
> stringA=“app_id=sm5b9b4daef3463&msn=NT1234DF23456&shop_id=1&timestamp=1589277365”;
> 步骤2：拼接的关键app_key，假定app_key是“dd3ac24736589ae17d333e362859bf4c”,那么   
>   
> stringB=“app_id=sm5b9b4daef3463&msn=NT1234DF23456&shop_id=1&timestamp=1589277365dd3ac24736589ae17d333e362859bf4c”
> 步骤3：对stringB进行MD5加密得到最终签名sign，即sign=MD5(stringB)。即sign=“946720303FEFF4516626A4431D2753CA”
> 以上参数值仅作为签名生成方式举例所用，实际签名生成所需参数值以实际情况为准
> 基于以上签名生成方式，其中私密参数app_id和app_Key，由商米科技统一分配。注意：签名sign必须全部32位大写MD5
#### 常见错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 10000  | 接口调用成功  |  
| 20000  | 服务不可用  |  
| 20001  | 授权失败  |  
| 40001  | 缺少所需参数  |  
| 40002  | 非法参数  |  
> 此上为全局常见错误码说明，后续会继续补充。除公共错误码以外，具体业务错误码参见具体业务接口内容。
#### 推送接口URL地址：
> 请求URL地址：https://openapi.sunmi.com/
### 1)打印机绑定接口
#### 接口名：
  * /v1/printer/printerAdd


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| shop_id  | string  | 合作伙伴名下的商户Id，最大长度小于32位字符。   
  
此处的shop_id是相对于合作伙伴APP中店铺管理而言的店铺id号，并非商米的店铺。因为打印机一般需要绑定到店铺下，才能建立店铺和设备的对应关系，设备查询时也要依据店铺来查询设备的联网状态。这个shop_id由合作伙伴进行管理，传什么值商米就保存什么值。   
  
shop_id是对合作伙伴名下设备进行分组管理的唯一条件，查询店铺设备状态列表时使用。若无店铺需关联可传””，那么设备状态列表结果集将只返回shop_id为空的设备，不会返回shop_id不为空的设备。  |  
#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": null,
    "msg" : ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 包含成功和失败信息  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
  "code": "10000",
  "data": {
     "subCode": "60001",
     "subMessage": "",
  },
  "msg":  "string"
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
| 60004  | 绑定失败  |  
| 60007  | 绑定异常  |  
| 60008  | 设备已绑定，请检查是否已在商米助手绑定过，需在商米助手中解绑后才能执行本渠道绑定  |  
| 60012  | shop_id超长，请限制在32位字符以内  |  
### 2)打印机解绑接口
#### 接口名：
  * /v1/printer/printerUnBind


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| shop_id  | string  | 合作伙伴名下的商户Id ，需要与绑定时一致  |  
> shop_id必传，否则无法将该打印机从该店铺下解绑
#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": null,
    "msg" : ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 包含成功和失败信息  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
  "code": "10000",
  "data": {
     "subCode": "60001",
     "subMessage": "",
  },
  "msg":  "string"
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
| 60005  | 解绑失败  |  
### 3)查询店铺下已绑定设备状态
#### 接口名：
  * /v1/machine/queryBindMachine


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| shop_id  | string  | 合作伙伴商户Id ，需要与绑定时一致  |  
#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": [
        {
            "is_online": "1",//1表示设备在线，0表示离线
            "msn": "N302D94D40068"//设备SN号
        }
    ],
    "msg": ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | Object  | msn:设备sn号；is_online:设备是否在线 1-在线 0-不在线  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": {
       "subCode": "60001",
       "subMessage": "",
    },
    "msg": ""
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
| 60006  | 商户Id不存在  |  
### 4)语音内容推送
#### 接口名：
  * /v1/printer/pushVoice


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| call_content  | string  | 需要语音播报的字符串，与call_url只能使用一个，无数据时传””  |  
| call_url  | string  | 需要设备播放的语音文件下载路径，与call_content只能使用一个，无数据时传””。文件格式只能为MP3或WAV格式（MP3要求16bit单声道8000Hz采样率。WAV要求16bit单声道8000Hz采样率）  |  
| expTimestamp  | int  | 语音消息有效截止Unix系统时间戳，一般为当前时间延后300s  |  
| cycle  | int  | 语音播放次数，0 不提示，1 播报1次，3 播报3次，999 循环播放  |  
| delay  | int  | 播报中间间隔时间，单位秒，一般间隔2秒  |  
#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": null,
    "msg" : ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 包含成功和失败信息  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": {
       "subCode": "60001",
       "subMessage": "",
    },
    "msg": ""
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
### 5)推送订单详情
#### 接口名：
  * /v1/printer/pushContent


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| pushId  | string  | 推送唯一ID，每张打印票据ID不同就认为是不同的打印内容  |  
| voiceCnt  | int  | 票据语音播放次数，0 不提示，1 播报1次，3 播报3次，999 循环播放  |  
| voice  | string  | 需要语音播报的字符串，voice与voiceUrl内容只能使用一个，无数据时传””  |  
| voiceUrl  | string  | 需要设备播放的语音文件http下载路径，voice与voiceUrl内容只能使用一个，无数据时传””。文件格式只能为MP3或WAV格式（MP3要求16bit单声道8000Hz采样率。WAV要求16bit单声道8000Hz采样率）  |  
| orderCnt  | int  | 打印票据张数  |  
| orderType  | int  | 打印票据类型，1 新订单，2 取消订单，3 催单，4 退单，5 其它  |  
| orderData  | string  | 打印票据内容。所有文本内容采用UTF8编码，再转换成十六进制格式传输。在内容中可以直接使用ESC/POS指令集控制打印，以获得所需的票据排版格式。  |  
#### 内容转换示例：
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

#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": null,
    "msg" : ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | string  | 包含成功和失败信息  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": {
       "subCode": "60001",
       "subMessage": "",
    },
    "msg": ""
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
| 60009  | 推送唯一ID不能为空  |  
| 60010  | 推送唯一ID已存在  |  
| 60011  | 推送失败  |  
### 6)清空待打印队列
#### 接口名：
  * /v1/printer/clearPrintList


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| 无  |   
 |   
 |  
#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data":{
       "count":2
    },
    "msg" : ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | Object  | 删除完成返回成功单数count；没有需要删除的订单时返回null  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": {
       "subCode": "60001",
       "subMessage": "",
    },
    "msg": ""
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
### 7)查询订单打印状态
#### 接口名：
  * /v1/printer/getPrintStatus


#### 请求方式：
  * POST


#### 公共参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| sign  | string  | 签名，参考上面签名生成方式。注意：签名sign必须全部32位大写MD5  |  
| app_id  | string  | 商米提供的app_id  |  
| msn  | string  | 设备SN号  |  
| timestamp  | string  | Unix时间戳（秒）  |  
#### 私有参数：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| pushId  | string  | 查询打印票据ID。为避免造成服务器压力，请勿频繁调用此接口，相同ID查询建议间隔1分钟以上  |  
#### 成功返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//订单打印完成
{
    "code": "10000",
    "data": {
        "msn"    : "NT210XXXXXXXX", 
        "isPrint": "1", //1表示已打印，0表示未打印，-1表示打印异常
        "unixTime": "1577425708" //打印成功的Unix时间
    },
    "msg" : ""
}
  

```

bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//订单未打印
{
    "code": "10000",
    "data": {
        "msn"    : "NT210XXXXXXXX", 
        "isPrint": "0", //1表示已打印，0表示未打印，-1表示打印异常
        "unixTime":  null //打印成功的Unix时间
    },
    "msg" : ""
}
  

```

#### 成功返回参数说明：  
| 参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| data  | Object  | 无可查询数据时返回null；查询成功返回打印信息：   
  
“status”1 订单打印成功、0 订单打印失败、-1 订单打印异常；   
  
“unixTime”已打印成功返回打印Unix时间”、未打印或异常的返回null  |  
#### 失败返回示例：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": "10000",
    "data": {
       "subCode": "60001",
       "subMessage": "",
    },
    "msg": ""
}
  

```

#### 失败错误码说明：  
| 错误码  | 说明  |  
| --- | --- |  
| 60001  | 超出时间范围请求无效  |  
| 60002  | sn号不存在  |  
| 60003  | 不属于该渠道设备  |  
上一篇：4. 开发对接API接口（回调方式）（仅适用于使用了SDK_V1接口的旧设备）
下一篇：6.打印机常用ESC/POS指令集（仅适用于使用了SDK_V1接口的旧设备）
