---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmzeghjk557
---

# 闪灯接口
更新时间：2025-09-25 17:50:32
## 5.1 接口描述
闪灯接口用于管理价签闪灯相关功能，包括让指示灯以某种颜色、频率进行闪烁。
## 5.2 接口列表  
| 接口名称  | 接口  |  
| --- | --- |  
| 指定价签闪灯  | /device/esl/flashLed  |  
## 5.3 接口详情
### 5.3.1 指定价签闪灯
**接口描述** ：通过本接口调用，可以让某些价签以某种方式闪灯。
**请求链接** ：/device/esl/flashLed
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| esl_id  | 是  | string  | 电子价签数据库ID转码  |  
| channel  | 否  | int （默认为4）  | LED颜色： 1-白， 2-蓝， 4-绿， 8-红，512-青， 1024-紫， 2048-黄  |  
| cycle  | 否  | int （默认为100）  | 单次闪烁周期， 单位10ms；即 1s=100个cycle  |  
| duration  | 否  | int （默认为8）  | 总共闪灯次数  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: { }
}
  

```

**错误码**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5301  | 非法价签  |  
| 5041  | 非法对接软件店铺  |  
上一篇：价签管理接口
下一篇：基站管理接口
