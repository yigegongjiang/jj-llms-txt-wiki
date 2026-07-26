---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfxieghjk579
---

# 4.开发数据回调API接口
更新时间：2025-09-24 19:10:15
> 特别说明：商米数据云打印机的数据采集服务集成在【商米数字店铺】和【商米助手】APP中。所以只有执行了【商米助手】APP将打印机与门店进行设备绑定，打印机的打印数据才会上报到对应的门店下，然后通过本API接口将合作伙伴云与门店进行绑定，才能接收到打印机数据。
### **绑定前获取门店对接凭证**
登录【商米数字店铺】，在【基础数据>组织管理>组织详情>获取对接凭证】中获取【对接店铺编号sunmi_shop_no】和【对接店铺密钥sunmi_shop_key】，记录下来为后续绑定接口提供参数（为了保证信息的有效性和安全性，对接凭证的有效期为24小时，过期之后可以重新获取）。
### **协议说明**
  1. 对接的接口目前只开放HTTPS方式推送消息，所有的消息一律采用POST方式。
  2. 商米接口请求URL地址：https://store.sunmi.com/openapi
  3. 传参请求头Content-Type需要加上”application/x-www-form-urlencoded”。
  4. 以下每个接口只列出私有参数和返回结果，其他参数请求请看公共参数说明。
  5. 请求和返回内容皆为json格式数据。
  6. 字符编码均为UTF-8。


### **请求公共参数说明（所有请求都必须传递的参数）**  
| 参数名  | 必填  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| app_id  | 是  | string  | 软件合作伙伴的唯一身份标识，同步获取的还有签名校验参数secret key  |  
| random  | 是  | string  | 随机字符串，由数字和大小写字母组成，长度范围为6-10位  |  
| timestamp  | 是  | int  | 当前的unix时间戳，精度到秒级，10位数字，格式参考<https://tool.chinaz.com/tools/unixtime.aspx>  |  
| sign  | 是  | string  | 签名信息，详见下文签名规则的说明  |  
### **请求公共参数sign签名说明**
  1. 对所有包含key的请求参数按照ASCII码顺序从小到大排序，将key value键值对依此拼接成字符串（文件参数不参与验签）
  2. 在字符串尾部拼接该软件合作伙伴独有的签名校验secret key
  3. 对字符串进行MD5加密，再将生成的MD5加密转化为32位全大写字符


> 示例：如果接口包含下列参数：   
>   
> app_id：87F380B846C51FF7   
>   
> random：289192   
>   
> timestamp：1603718252   
>   
> shop_id：100001   
>   
> company_id：10000   
>   
> shop_nam：myShop   
>   
> company_name：myCompany   
>   
> sunmi_shop_no：470826116009   
>   
> sunmi_shop_key：MG4LJ5ERHUBDMF5XOOU8   
>   
> contact_person：LiLei   
>   
> phone：13166668888
> 则签名生成步骤为：   
>   
> 步骤1：按照key的ASCII码顺序从小到大排序并拼接成字符串   
>   
> str1 = “app_id=87F380B846C51FF7&company_id=10000&company_name=myCompany&contact_person=LiLei&phone=13166668888&random=289192&shop_id=100001&shop_nam=myShop&sunmi_shop_key=MG4LJ5ERHUBDMF5XOOU8&sunmi_shop_no=470826116009&timestamp=1603718252”
> 步骤2：在字符串尾部拼接secret key   
>   
> str2 = str1 + “&key=kdsofkdsnflke9382938k”
> 步骤3：对str2进行MD5加密，并转为32位全大写字符，就可以获得最终签名sign   
>   
> sign = MD5(str2).upper() = 6413A7275DBE256D936AAEF5F1D958B9
### **返回公共参数说明**  
| 返回参数名  | 类型  | 说明  |  
| --- | --- | --- |  
| code  | int  | 返回码，参见下面常见错误代码列表  |  
| data  | 不指定  | 数据类型和内容详见私有返回参数data，如果有错误，返回null  |  
| msg  | string  | 结果提示信息，如果有错误返回错误信息描述  |  
> 请求后，返回结果作为HTTP消息返回，如果消息签名认证失败，HTTP消息会返回 401等作为提示。
> 如果消息签名认证通过，则进入业务层处理，此次HTTP请求返回200 OK。
> 具体业务层面的操作返回JSON值在消息体中。
### **返回常见错误代码说明**  
| 错误码  | 说明  |  
| --- | --- |  
| 0  | 成功  |  
| 5090  | sign校验不正确  |  
| 5091  | timestamp的请求过期  |  
## 4.1、商米云API接口-门店绑定接口
###  **接口描述** ：
通过本接口调用，用户可以将第三方对接软件上的某门店与【商米数字店铺】中的门店进行绑定。
### **接口名：**
/shop/bind
###  **接口私有参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| shop_id  | 是  | string  | **第三方对接软件中门店的标识** ，对接的软件提供shop_id 与 sunmi_shop_no 具有一一对应关系。对于v2.0 及以后版本的所有接口，建议采用sunmi_shop_no作为店铺标识。对于已经使用shop_id作为参数的接口保持后向兼容，不需要进行修改。  |  
| company_id  | 是  | string  | **第三方对接软件中商户的标识** ，对接的软件提供商户是门店的上一级组织。记录商户与门店对应关系，可以据此处理后续商户级别业务，以及商户内跨门店业务。对于单商户-多门店模式的用户，建议商户-门店的对应关系在第三方对接软件和【商米数字店铺】体系中保持一致。  |  
| shop_name  | 是  | string  | 第三方对接软件中门店的门店名称  |  
| company_name  | 是  | string  | 商户名称  |  
| sunmi_shop_no  | 是  | string  | 【商米数字店铺】的门店【对接店铺编号】，可在【商米数字店铺】的【基础管理>组织管理>组织详情】中获取  |  
| sunmi_shop_key  | 是  | string  | 【商米数字店铺】的门店【对接店铺密钥】，可在【商米数字店铺】的【基础管理>组织管理>组织详情】中获取  |  
| contact_person  | 是  | string  | 联系人姓名  |  
| phone  | 是  | string  | 联系人电话  |  
| industry_id  | 否  | int  | 门店行业类别编号  |  
| industry_name  | 否  | string  | 门店行业类别名称  |  
| province  | 否  | string  | 所在省份  |  
| city  | 否  | string  | 所在市  |  
| district  | 否  | string  | 所在区县  |  
| address  | 否  | string  | 商户门店地址  |  
| mail  | 否  | string  | 联系人邮箱  |  
| format  | 否  | int  | 门店业态 0-无 1 – 餐前付款， 2 – 餐后付款，默认0  |  
| status  | 否  | int  | 门店状态 1 – 启用， 2 – 停用， 默认1  |  
**请求示例** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
  "method": "POST",
  "url": "https://store.uat.sunmi.com/openapi/shop/bind",
  "headers": {
    "Content-Type": "application/x-www-form-urlencoded"
  },
  formData: {
    "shop_id": "10096",
    "shop_name": "myShop",
    "sunmi_shop_no": "560279010307",
    "sunmi_shop_key": "MG4LJ5ERHUBDMF5XOOU8",
    "company_id": "10000",
    "company_name": "myCompany",
    "contact_person": "LiLei",
    "phone": "13166668888",
    "app_id": "LMWWQVTW4QGCC",
    "timestamp": 1581383983,
    "random": "5dsf6698",
    "sign": "33C18A18282733A71F998BB5A5E4319D"
  }
  

```

**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 0,   /* 其他错误参考错误列表 */
    "msg": "succeed",
    "data": {}
}
  

```

**错误编号列表：**  
| 错误编号  | 错误原因  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5032  | 非法店铺  |  
| 5041  | 非法对接软件  |  
| 5042  | 对接软件中的门店已绑定  |  
| 5043  | 对接商米店铺密钥错误  |  
## 4.2、商米云API接口-门店解绑接口
###  **接口描述** ：
通过本接口调用，用户可以将SaaS上的某门店与【商米数字店铺】中的门店解除绑定。
### **接口名：**
/shop/unbind
###  **接口私有参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| shop_id  | 是  | string  | 第三方对接软件中门店的标识，对接的软件提供  |  
| company_id  | 否  | string  | 第三方对接软件中商户的标识，对接的软件提供  |  
| sunmi_shop_no  | 否  | string  | 【商米数字店铺】的门店【对接店铺编号】  |  
**请求示例** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
  "method": "POST",
  "url": "https://store.uat.sunmi.com/openapi/shop/unbind",
  "headers": {
    "Content-Type": "application/x-www-form-urlencoded"
  },
  formData: {
    "shop_id": "10096",
    "app_id": "LMWWQVTW4QGCC",
    "timestamp": 1581383983,
    "random": "5dsf6698",
    "sign": "33C18A18282733A71F998BB5A5E4319D"
  }
  

```

**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 0,   /* 其他错误参考错误列表 */
    "msg": "succeed",
    "data": {}
}
  

```

**错误列表：**  
| 错误编号  | 错误原因  |  
| --- | --- |  
| 5032  | 非法店铺  |  
| 5041  | 非法对接软件  |  
| 5000  | 数据库错误  |  
## 4.3、合作伙伴云API接口-数据接收接口
### **接口描述：**
当商米云收到打印机的小票数据并完成解析后，会调用软件合作方提供的一个接口发送数据。在合作伙伴云端开发完成了本数据接收接口以后，还需要将本接口的【回调URL地址】和【消息类型event】告诉商米对接技术人员；商米后台设置以后才会回传数据到合作伙伴云。
### **接口名：**
/uploaddata
### **接口私有参数：**  
| 参数名  | 必填  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 是  | string  | 【商米数字店铺】平台门店的唯一编号  |  
| shop_id  | 是  | string  | 店铺在对接软件体系下的唯一标识  |  
| event  | 是  | int  | 触发消息的类型，具体请参考下文  |  
| payload  | 是  | string  | 消息的具体格式（根据消息不同会有不同的json），具体请参考下文  |  
注意：如果回调消息发送失败，会触发重传机制。一共会触发4次重传，时间间隔分别为15秒, 30 秒, 1分钟和2分钟。
合作伙伴云接口返回状态码为2xx时，表示数据推送成功。
## **4.4 数据事件和消息event**
**1、event=6001 未经处理的文字小票上传消息**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
报文格式： application/x-www-form-urlencoded;param=value;charset=UTF-8

消息体格式：
{ 
  "app_id": 'CSJGYI6T8P237',                //唯一标识接入身份，联系商米数字店铺提供
  "event": '6002',                          // 触发消息的类型
  "payload": '{
       "id":"70dcd2f600e45736e1",           // 系统生成的小票ID
       "sn":"N302D9WZC0064",                // 打印设备SN
       "text": "abc"                        // 文字小票内容
  }',
  "random": 'NDL8GXR',                          // 随机字符串，由数字和字母组成，长度范围为6-10位
  "shop_id": '29203',                           // 店铺在SaaS软件体系下的唯一标识, 没有或者不需要则为空
  "sign": '738D9FF2482D59E5DC1FB32B6F445464',   //签名校验
  "sunmi_shop_no": '28393437387',               // 商米数字店铺平台门店唯一编号, 没有或者不需要则为空
  "timestamp": '1604567375'                     //当前的unix timestamp，精度到秒级，10位数字
}

备注：payload字段对应的值为string类型，解析对此string类型内容进行json解析
  

```

**2、event=6002 未经处理的图片小票上传消息**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
报文格式： application/x-www-form-urlencoded;param=value;charset=UTF-8

消息体格式：
{ 
  "app_id": 'CSJGYI6T8P237',                //唯一标识接入身份，联系商米数字店铺提供
  "event": '6002',                          // 触发消息的类型
  "payload": '{
       "id":"70dcd2f600e45736e1",           // 系统生成的小票ID
       "sn":"N302D9WZC0064",                // 打印设备SN
       "base64_image":"DKJOIFDNLKSDJLSJDISDNLKFJDLSLDJF==",  // 图片小票的base64 encoded string
  }',
  "random": 'NDL8GXR',                          // 随机字符串，由数字和字母组成，长度范围为6-10位
  "shop_id": '29203',                           // 店铺在SaaS软件体系下的唯一标识, 没有或者不需要则为空
  "sign": '738D9FF2482D59E5DC1FB32B6F445464',   //签名校验
  "sunmi_shop_no": '28393437387',               // 商米数字店铺平台门店唯一编号, 没有或者不需要则为空
  "timestamp": '1604567375'                     //当前的unix timestamp，精度到秒级，10位数字
}

备注：payload字段对应的值为string类型，解析对此string类型内容进行json解析
  

```

**3、event=6003 经过处理的小票消息**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
报文格式： application/x-www-form-urlencoded;param=value;charset=UTF-8

消息体格式：
{ 
  "app_id": 'CSJGYI6T8P237',                //唯一标识接入身份，联系商米数字店铺提供
  "event": '6003',                          // 触发消息的类型
  "payload": '{
       "id":"70dcd2f600e45736e1",           // 系统生成的小票ID
       "sn":"N302D9WZC0064",                // 打印设备SN
       "order_time":"2020-11-05 17:45:00",  // 小票生成时间 
       "order_id":"202011051707",           // 小票上的订单ID 
       "order_received_amount":"20",          // 小票金额 
       "sales_detail_list":[                // 明细列表
            { "amount":"4",                   // 商品单价
              "item":"维他柠檬茶",           // 商品名字
              "quantity":"1"                  // 商品数量
            }, 
            {"amount":"7","item":"梅汤","quantity":"1"},
            {"amount":"6","item":"旺仔牛奶","quantity":"1"}, 
            {"amount":"3","item":"罐","quantity":"1"}
      ]
  }',
  "random": 'NDL8GXR',                          // 随机字符串，由数字和字母组成，长度范围为6-10位
  "shop_id": '29203',                           // 店铺在SaaS软件体系下的唯一标识, 没有或者不需要则为空
  "sign": '738D9FF2482D59E5DC1FB32B6F445464',   //签名校验
  "sunmi_shop_no": '28393437387',               // 商米数字店铺平台门店唯一编号, 没有或者不需要则为空
  "timestamp": '1604567375'                     //当前的unix timestamp，精度到秒级，10位数字
}

备注：payload字段对应的值为string类型，解析对此string类型内容进行json解析
  

```

**4、event=6004 经过处理的小票携带二次计算实收金额信息消息**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
报文格式： application/x-www-form-urlencoded;param=value;charset=UTF-8

消息体格式：
{ 
  "app_id": 'CSJGYI6T8P237',               	//唯一标识接入身份，联系商米数字店铺提供
  "event": '6004',                          // 触发消息的类型
  "payload": '{
       "id":"70dcd2f600e45736e1",           // 系统生成的小票ID
       "sn":"N302D9WZC0064",                // 打印设备SN
       "seq": "16354077750",                // 设备上传唯一标识
       "order_time":"2020-11-05 17:45:00",  // 小票生成时间 
       "order_id":"202011051707",           // 小票上的订单ID 				
       "receipt_type":"1",			        //小票类型：1： 收银小票，2： 退款单， 3：交接单，4：其它
       "order_received_amount":"20",        // 小票金额 
       "sales_detail_list":[                // 明细列表
            { "amount":"4",                 // 商品单价
              "item":"维他柠檬茶",          	// 商品名字
              "quantity":"1",               // 商品数量
              "barcode": "38293823"         // 商品条码
            }, 
            {"amount":"7","item":"梅汤","quantity":"1", "barcode":"29300239"}, 
            {"amount":"3","item":"罐","quantity":"1", "barcode":"29300229"}
                           ],
      "shop_defined_pay_method_list":[		//门店定义支付计算方式列表
            { "pay_method":"储值卡",      			// 支付方式名称
              "pay_discount_factor":"100",  	// 实收百分因子，取值1~100整数
              "pay_calculation_type":"1"    	// 支付计算类型，1：加，2：减
            },
            { "pay_method":"支付宝", "pay_discount_factor":"20","pay_calculation_type":"2"}，
            { "pay_method":"微信", "pay_discount_factor":"80","pay_calculation_type":"1"}
                                      ],
      "calculated_pay_amount":"17.38",		//根据门店配置，AI解析后计算的实收金额
      "ai_recognized_pay_method_list":[
            { "key":"微信",        		//AI识别出来的支付方式名称
              "value":"22.3"			//AI识别出来的该支付方式对应的金额
            },
            { "key":"支付宝", "value":"2.3"},
            { "key":"储蓄卡","value":""}
     ]
   }',
  "shop_id": '29203',                          // 第三方对接软件中门店的标识, 没有或者不需要则为空
  "sunmi_shop_no": '28393437387',           // 商米数字店铺平台的门店组织编号, 没有或者不需要则为空
  "random": 'NDL8GXR',                       // 随机字符串，由数字和字母组成，长度范围为6-10位
  "sign": '738D9FF2482D59E5DC1FB32B6F445464',   //签名校验
  "timestamp": '1604567375'                  //当前的unix timestamp，精度到秒级，10位数字
}
备注：payload字段对应的值为string类型，解析对此string类型内容进行json解析
实收金额计算公式如下：
"calculated_pay_amount"= SUM（"value"*对应"pay_discount_factor"/100）
加上还是减去由"pay_calculation_type"决定
  

```

支付方式关键字的配置要在数字店铺操作，操作手册可见：《[3.2.1 小票支付方式识别和折扣系数配置](https://www.yuque.com/sinmizsk/mt0gd0/fa6gmp?singleDoc#)》。
上一篇：3.注册合作伙伴账号
下一篇：5.数采问题FAQ
