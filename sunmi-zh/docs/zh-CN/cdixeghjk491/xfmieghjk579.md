---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmieghjk579
---

# 基站管理接口
更新时间：2025-09-25 17:54:01
## 6.1 接口描述
基站管理接口用来管理电子价签使用的无线基站，包括基站的绑定解绑重启等。
## 6.2 接口列表  
| 接口名称  | 接口  |  
| --- | --- |  
| 绑定基站到门店  | /device/ap/bind  |  
| 从门店解绑基站  | /device/ap/unbind  |  
| 获取基站列表  | /device/ap/getList  |  
| 获取基站详情  | /device/ap/getInfo  |  
| 修改基站信息  | /device/ap/updateName  |  
| 重启基站  | /device/ap/reboot  |  
| 获取商铺下所有基站信息  | /device/ap/getListByCompany  |  
## 6.3 接口详情
### 6.3.1 **绑定基站到门店**
**接口描述** ：通过本接口调用，用户可以绑定无线基站到指定门店。
**请求链接** ：/device/ap/bind
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| ap_sn  | 是  | string  | 无线基站SN  |  
| ap_name  | 否  | string  | 无线基站名称  |  
| ap_mac  | 否  | string  | 无线基站MAC地址  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
        “ap_id": ”129200“,
    }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5023  | 非法参数  |  
| 5300  | 非法基站  |  
| 5339  | 基站已被店铺绑定  |  
| 5041  | 非法对接软件店铺  |  
### 6.3.2 从门店解绑基站
**接口描述** ：通过本接口调用，用户可以从门店中解绑无线基站。
**请求链接** ：/device/ap/unbind
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| ap_id  | 是  | string  | 无线基站编号  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {  }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5300  | 非法基站  |  
| 5041  | 非法对接软件店铺  |  
### 6.3.3 获取基站列表
**接口描述** ：通过本接口调用，用户可以获取无线基站列表 。
**请求链接** ：/device/ap/getList
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| page_num  | 否（默认1）  | int  | 当前页码  |  
| page_size  | 否（默认10）  | int  | 当前页条目数量  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
         "total_count": 100,
         "ap_list": [{
            "ap_id": “1000”,
            "ap_sn": "B201E96500001",
            "ap_name": "Gate 5"，
            "esl_count": 1920,
            "status": 2
        },
       ... ...
  ] }
}
  

```

**返回字段描述：status**  
| status 取值  | 说明  |  
| --- | --- |  
| 0  | 未激活  |  
| 1  | 在线  |  
| 2  | 离线  |  
**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5020  | 非法参数  |  
| 5041  | 非法对接软件店铺  |  
### 6.3.4 获取基站详情
**接口描述** ：通过本接口调用，用户可以获取无线基站详情 。
**请求链接** ：/device/ap/getInfo
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| ap_id  | 是  | string  | 无线基站编号  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
          "ap_id": “1000”,
          "ap_sn": "B201E96500001",
          "ap_name": "Gate 5",
          "model_name": "SLAP1",
          "status": 2,
          "esl_count": 1920,
          "software_version": "1.0.1",
          "connect_time": 15683920394，
  ] }
}
  

```

**返回字段描述：status**  
| status 取值  | 说明  |  
| --- | --- |  
| 0  | 未激活  |  
| 1  | 在线  |  
| 2  | 离线  |  
**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5011  | 非法设备机型  |  
| 5300  | 非法基站  |  
| 5041  | 非法对接软件店铺  |  
### 6.3.5 修改基站信息
**接口描述** ：通过本接口调用，用户可以修改无线基站名称 。
**请求链接** ：/device/ap/updateName
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| ap_id  | 是  | string  | 无线基站编号  |  
| ap_name  | 是  | string  | 修改的名字  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: { }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5300  | 非法基站  |  
| 5041  | 非法对接软件店铺  |  
### 6.3.6 重启基站
**接口描述** ：通过本接口调用，用户可以重启基站。
**请求链接** ：/device/ap/reboot
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| ap_id  | 是  | string  | 无线基站编号  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: { }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5300  | 非法基站  |  
| 5041  | 非法对接软件店铺  |  
### 6.3.7 获取商铺下所有基站信息
**接口描述** ： 通过本接口调用，用户可以分页获取指定商铺下的基站信息
**请求链接 ：** /device/ap/getListByCompany
**请求参数：**  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_company_no  | 是  | string  | 商米店铺平台中的企业编号  |  
| page_num  | 否(默认1)  | int  | 页码  |  
| page_size  | 否（默认10）  | int  | 每页记录数  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "data": {
        "total_count": 1,
        "ap_list": [
            {
                "id": "314159283514",
                "sn": "B203P9CD00003",
                "name": "zqzqzzq",
                "mac": "0C:25:76:0C:25:08",
                "ip": "192.168.100.190",
                "model_name": "SLAP1",
                "esl_number": 0,
                "bin_version": "0.1.2",
                "status": 2
            }
        ]
    },
    "code": 0,/* 其他错误参考错误列表 */
    "msg": "succeed"
}
  

```

**返回字段描述：status**  
| status取值  | 说明  |  
| --- | --- |  
| 0  | 未激活  |  
| 1  | 在线  |  
| 2  | 离线  |  
**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5041  | 非法对接软件店铺  |  
| 5903  | 该商铺不是当前saas创建，无权查看  |  
上一篇：闪灯接口
下一篇：模板管理接口
