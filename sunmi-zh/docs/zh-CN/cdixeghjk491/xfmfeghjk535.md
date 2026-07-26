---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmfeghjk535
---

# 价签管理接口
更新时间：2025-09-25 17:46:34
## 4.1 接口描述
设备管理接口用来管理电子价签设备本身，包括价签与门店的绑定解绑等。
## 4.2 接口列表  
| 接口名称  | 接口  |  
| --- | --- |  
| 添加价签到门店  | /device/esl/bind  |  
| 从门店删除价签  | /device/esl/unbind  |  
| 获取价签列表  | /device/esl/getList  |  
| 获取价签详情  | /device/esl/getInfo  |  
| 对价签推特定图片  | /device/esl/pushImage  |  
| 获取统计信息  | /device/getOverview  |  
| 获取商铺下所有价签信息  | /device/esl/getListByCompany  |  
## 4.3 接口详情
### 4.3.1 添加价签到门店
**接口描述** ：通过本接口调用，用户可以绑定价签到指定门店。这一步操作不是必须，在执行商品绑定价签的时候，如果对应价签没有被任何门店绑定，也会执行绑定价签到门店的动作。
**请求链接** ：/device/esl/bind
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| esl_code  | 否（esl_code和esl_sn至少提供一个）  | string  | 电子价签8位ID （价签正面的条码）  |  
| esl_sn  | 否（esl_code和esl_sn至少提供一个）  | string  | 电子价签SN  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
        "esl_id": “129200”,
    }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5301  | 非法价签  |  
| 5338  | 价签已被店铺绑定  |  
| 5041  | 非法对接软件店铺  |  
### 4.3.2 从门店删除价签
**接口描述** ：通过本接口调用，用户可以从指定店铺上解绑价签，价签删除后会显示出厂图，可以再次被其他门店使用。
**请求链接** ：/device/esl/unbind
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| esl_id  | 是  | string  | 电子价签数据库ID转码  |  
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
| 5301  | 非法价签  |  
| 5041  | 非法对接软件店铺  |  
### 4.3.3 获取价签列表
**接口描述** ：通过本接口调用，用户可以获取价签列表，列表中包含价签状态。
**请求链接** ：/device/esl/getList
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| page_num  | 否（默认1）  | int  | 当前页码  |  
| page_size  | 否 (默认10)  | int  | 当前页条目数量  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
         "total_count": 100,
        "esl_list": [{
               "esl_id": “1000”,
                "esl_code": "SKDI39DN",
                "esl_sn": "B101194N00002",
                "model_name": "SL121+"，
                "status": 2
             },
             ... ...
       ] }
  

```

**返回字段描述：status**  
| status 取值  | 说明  |  
| --- | --- |  
| 0  | 未激活  |  
| 1  | 未绑定  |  
| 2  | 待推送  |  
| 3  | 推送成功  |  
| 4  | 推送失败  |  
**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5020  | 非法参数  |  
| 5041  | 非法对接软件店铺  |  
### 4.3.4 获取价签详情
**接口描述** ：通过本接口调用，用户可以获取价签详情(包含MAC地址，电量，信号强度等)。
**请求链接** ：/device/esl/getInfo
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| esl_code  | 否（esl_code和esl_id至少输入一个）  | string  | 电子价签8位ID （价签正面的条码）  |  
| esl_id  | 否 (esl_code和esl_id至少输入一个)  | string  | 电子价签数据库ID转码  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
            "esl_id": "1000",
            "esl_code": "SKDI39DN",
            "esl_sn": "B101194N00002",
            "model_name": "SL121+",
            "status": 2,
            "screen_size_name": "2.13寸",
            "software_version": "1.0.1",
            "battery": 90,
            "rssi": -37,
            "connect_time": 15683920394,
            "ap_id":  "10200",
            "ap_sn": "B201E95D00001",
            "ap_name": "",
      }
}
  

```

**返回字段描述：status**  
| status 取值  | 说明  |  
| --- | --- |  
| 0  | 未激活  |  
| 1  | 未绑定  |  
| 2  | 待推送  |  
| 3  | 推送成功  |  
| 4  | 推送失败  |  
**错误码**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5023  | 缺少参数  |  
| 5502  | 非法设备机型  |  
| 5041  | 非法对接软件店铺  |  
### 4.3.5 对价签推特定图片
**接口描述** ：通过本接口调用，用户可以指定价签推图。
**请求链接** ：/device/esl/pushImage
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| esl_id  | 是  | string  | 电子价签数据库ID转码  |  
| pic  | 是  | file  | 上传的刷图文件  |  
**图片分辨率：**
上传图片的颜色和大小尺寸需要符合要求，否则无法成功下发显示到电子价签屏幕上，相关参数参考下表。  
| 电子价签型号  | 支持颜色  | 支持图片分辨率（px）  |  
| --- | --- | --- |  
| SL115  | 黑白、黑白红  | 152 * 152  |  
| SL121  | 黑白、黑白红  | 212 * 104  |  
| SL126  | 黑白、黑白红  | 296 * 152  |  
| SL126+  | 黑白  | 296 * 152  |  
| SL142  | 黑白、黑白红  | 400 * 300  |  
| SL142+  | 黑白、黑白红  | 400 * 300  |  
| SL175  | 黑白、黑白红  | 640 * 384  |  
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
| 5004  | 系统错误  |  
| 5005  | 非法文件  |  
| 5020  | 非法参数  |  
| 5300  | 非法基站  |  
| 5301  | 非法价签  |  
| 5041  | 非法对接软件店铺  |  
### 4.3.6 获取统计信息
**接口描述** ： 通过本接口调用，用户可以获取价签和基站的统计概览信息。
**请求链接** ：/device/getOverview
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code":0,
    "data":{
            "ap_total_count":24,
            "esl_total_count":55,
            "esl_pending_count":54,
            "esl_failed_count":3
        },
    "msg":""
}
  

```
  
| 错误码  | 说明  |  
| --- | --- |  
| 5041  | 非法对接软件店铺  |  
### 4.3.7 获取商铺下所有价签信息
**接口描述** ：通过本接口调用，用户可以分页获取指定商铺下的价签信息。
**请求链接** ： /device/esl/getListByCompany
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_company_no  | 是  | string  | 商米数字店铺平台的企业编号  |  
| page_num  | 否(默认1）  | int  | 页码  |  
| page_size  | 否（默认10）  | int  | 每页记录数  |  
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "data": {
        "total_count": 1,
        "esl_list": [
            {
                "id": "314159282628",
                "esl_code": "HRBAGJAY",
                "sn": "B101194N00006",
                "mac": "00:01:01:02:02:02",
                "bin_version": "0.6.6",
                "battery": 4,
                "rssi": 0,
                "status": 1,
                "ap_sn": "tongyutestsn",
                "ap_id": "314159333800",
                "model_name": "SL121"
            }
        ]
    },
    "code": 0, /* 其他错误参考错误列表 */
    "msg": "succeed"
}
  

```

**返回字段描述：status**  
| status取值  | 说明  |  
| --- | --- |  
| 0  | 未激活  |  
| 1  | 未绑定  |  
| 2  | 待推送  |  
| 3  | 推送成功  |  
| 4  | 推送失败  |  
**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5041  | 非法对接软件店铺  |  
| 5903  | 该商铺不是当前 saas 创建，无权查看  |  
上一篇：商品管理接口
下一篇：闪灯接口
