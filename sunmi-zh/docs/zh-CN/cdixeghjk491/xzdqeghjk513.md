---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzdqeghjk513
---

# 设备应用参数列表
更新时间：2026-05-28 17:46:10
# **接口说明**
> 设备应用版本参数列表
# 使用限制
  * 使用该接口需要先完成[开发者能力接入流程](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmmeghjk546/)。
  * 完成**云** 应用的创建并添加`参数管理`能力。


# 基本信息  
| 名称  | 描述  |  
| --- | --- |  
| HTTP URL  | https://openapi.sunmi.com/v2/fin/params/open/device/list  |  
| HTTP Method  | POST  |  
# 请求头  
| 字段名称  | 类型  | 是否必填  | 示例  | 描述  |  
| --- | --- | --- | --- | --- |  
| Sunmi-Appid  | String  | 是  | 499cf21264db4e42a6d7222b74335fb4  | 你的 APPID  |  
| Sunmi-Nonce  | String  | 是  | 953476  | 6位随机数  |  
| Sunmi-Timestamp  | Integer  | 是  | 1725503361  | 10位时间戳  |  
| Sunmi-Sign  | String  | 是  | 05ef4b90fa594aaca8eb31746edb44a2  | 计算出的签名  |  
# 请求参数  
| 名称  | 类型  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| page_no  | integer  | 必须  | 1  | 分页列表，页序号。min:1  |  
| page_size  | integer  | 必须  | 10  | 分页列表，每页数量。min:1; max:100  |  
| msn_list  | array  | 非必须  | [“SN1“,”SN2”]  | 指定msn查询，最多100个。  |  
| package_name  | string  | 非必须  | com.sunmi.test  | 指定应用包名  |  
| version_code  | integer  | 非必须  | 0  | 指定应用内部版本号  |  
# 请求示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
curl --http1.1 -X POST 'https://openapi.sunmi.com/v2/fin/params/open/device/list' 
-H "Content-Type: application/json" 
-H 'Sunmi-Appid: 499cf21264db4e42a6d7222b74335fb4' 
-H 'Sunmi-Nonce: 953476' 
-H 'Sunmi-Timestamp: 1725503361' 
-H 'Sunmi-Sign: 05ef4b90fa594aaca8eb31746edb44a2' 
-d '{"page_no":1,"page_size":10}'
  

```

# 响应参数  
| 名称  | 类型  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| code  | integer  | 必须  | 1  | 返回码  |  
| msg  | string  | 必须  | success  | 返回信息  |  
| data  | object  | 必须  |   
 | 返回数据  |  
| ├─ total  | integer  | 必须  | 1  | 模板总数  |  
| ├─ list  | array  | 必须  |   
 | 模板列表  |  
|  ├─ msn  | string  | 必须  | SN2  | 设备SN  |  
|  ├─ app_id  | integer  | 必须  | 234  | 应用ID，唯一值  |  
|  ├─ app_name  | string  | 必须  | SUNMI Test App  | 应用名称  |  
|  ├─ package_name  | string  | 必须  | com.sunmi.test  | 应用包名  |  
|  ├─ version_id  | integer  | 必须  | 1123  | 应用版本ID，唯一值  |  
|  ├─ version_code  | integer  | 必须  | 1234  | 应用内部版本号  |  
|  ├─ version_name  | string  | 必须  | v1.2.31  | 应用外部版本号  |  
|  ├─ status  | integer  | 必须  | 1  | 状态：1拉取中；2已拉取  |  
|  ├─ create_time  | integer  | 必须  | 1765422780  | 创建时间戳，单位：秒  |  
|  ├─ update_time  | integer  | 必须  | 1765422780  | 最后更新时间戳，单位：秒  |  
# 响应示例
## 成功示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": {
        "total":1,
        "list":[
        {
           "msn":"SN2",
           "app_id":234,
           "app_name":"SUNMI Test App",
           "package_name":"com.sunmi.test",
           "version_id":1123,
           "version_code":1234,
           "version_name":"v1.2.31",
           "status":1,
           "create_time":1765422780,
           "update_time":1765422780
       }
    }
}
  

```

  

## 异常示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 30001,
    "data": "",
    "msg": "can not get ability"
}
  

```

# 错误码
接口返回错误码请参阅 [错误码说明](https://docs.sunmi.com/preview/zh-CN/xzqceghjk502)
上一篇：参数模板检查校验
下一篇：设备应用参数详情
