---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzdmeghjk546
---

# 设备应用参数推送保存
更新时间：2026-05-28 17:45:23
# **接口说明**
> 推送设备应用版本参数
# 使用限制
  * 使用该接口需要先完成[开发者能力接入流程](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xcmmeghjk546/)。
  * 完成**云** 应用的创建并添加`参数管理`能力。


# 基本信息  
| 名称  | 描述  |  
| --- | --- |  
| HTTP URL  | https://openapi.sunmi.com/v2/fin/params/open/device/push/save  |  
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
| msn  | string  | 必须  | SN2  | 指定msn查询  |  
| app_id  | integer  | 必须  | 234  | 应用ID，唯一值  |  
| version_id  | integer  | 必须  | 1123  | 应用版本ID，唯一值  |  
| params  | string  | 必须  | {"version": 1, "isOffice": true, "template": {}}  | 参数详情，json字符串，最大200000字符  |  
模板 json 结构参见 [参数JSON结构说明](https://docs.sunmi.com/preview/zh-CN/xzqdeghjk524)
# 请求示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
curl --http1.1 -X POST 'https://openapi.sunmi.com/v2/fin/params/open/device/push/save' 
-H "Content-Type: application/json" 
-H 'Sunmi-Appid: 499cf21264db4e42a6d7222b74335fb4' 
-H 'Sunmi-Nonce: 953476' 
-H 'Sunmi-Timestamp: 1725503361' 
-H 'Sunmi-Sign: 05ef4b90fa594aaca8eb31746edb44a2' 
-d '{"app_id":234,"version_id":1123,"msn":"SN2","params":"{\"version\": 1, \"isOffice\": false, \"template\": {}}"}'
  

```

# 响应参数  
| 名称  | 类型  | 是否必须  | 示例  | 备注  |  
| --- | --- | --- | --- | --- |  
| code  | integer  | 必须  | 1  | 返回码  |  
| msg  | string  | 必须  | success  | 返回信息  |  
| data  | object  | 必须  |   
 | 返回数据  |  
|  ├─ push_id  | string  | 必须  | 82905cf7f05d791075efb35e12b2276a  | 推送ID标识，唯一值  |  
# 响应示例
## 成功示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 1,
    "msg": "success",
    "data": {
       "push_id":"82905cf7f05d791075efb35e12b2276a"
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
上一篇：设备应用参数删除
下一篇：设备应用参数中止推送
