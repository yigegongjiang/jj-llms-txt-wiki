---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmdeghjk524
---

# 商品管理接口
更新时间：2025-09-25 17:42:15
## 3.1 接口描述
商品管理接口包括两部分：
一部分是商品新增修改删除接口，这是数字店铺的通用基础功能，很多设备和服务都会用到，参考单独的商品对接的文档。
另一部分是价签业务相关的功能，包括商品与价签的绑定解绑等，在本文中进行描述。
## 3.2 接口列表  
| 接口名称  | 接口  |  
| --- | --- |  
| 商品绑定价签  | /product/bindEsl  |  
| 商品解绑价签  | /product/unbindEsl  |  
| 获取商品绑定价签  | /product/getBindEslList  |  
| 获取商品列表  | /product/getList  |  
| 获取商品详情  | /product/getInfo  |  
## 3.3 接口详情
### 3.3.1 商品绑定价签
**接口描述** ：通过本接口调用，用户可以将商品与指定价签进行绑定，同时指定对应模板。绑定之后价签将开始刷图。
**请求链接** ：/product/bindEsl
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| product_id  | 是  | string  | 商品数据库ID，如果进行了对接，和数据对接传过来的id一致  |  
| esl_code  | 否（esl_code与esl_id至少提供一个）  | string  | 电子价签8位ID（价签正面的条码）  |  
| esl_id  | 否（esl_code与esl_id至少提供一个）  | string  | 电子价签数据库ID转码  |  
| template_id  | 是  | string  | 模板数据库ID  |  
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
| 5020  | 参数错误  |  
| 5015  | 非法商品  |  
| 5343  | 非法模板  |  
| 5300  | 非法基站  |  
| 5301  | 非法价签  |  
| 5338  | 价签已被其他店铺绑定  |  
| 5342  | 非法价签图片  |  
| 5006  | OSS错误  |  
| 5041  | 非法对接软件店铺  |  
### 3.3.2 商品解绑价签
**接口描述** ：通过本接口调用，用户可以将商品与指定价签解除绑定。解绑之后价签将刷新显示解绑模板对应的内容。
**请求链接** ：/product/unbindEsl
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| esl_code  | 否（esl_code与esl_id至少提供一个）  | string  | 电子价签8位ID （价签正面的条码）  |  
| esl_id  | 否（esl_code与esl_id至少提供一个）  | string  | 电子价签数据库ID转码  |  
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
| 5020  | 参数错误  |  
| 5301  | 非法价签  |  
| 5320  | 价签未绑定  |  
| 5338  | 价签已被其他店铺绑定  |  
| 5342  | 非法价签图片  |  
| 5006  | OSS错误  |  
| 5041  | 非法对接软件店铺  |  
### 3.3.3 获取商品绑定价签
**接口描述** ：通过本接口调用，用户可以获取商品绑定的价签列表。
**请求链接** ：/product/getBindEslList
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| product_id  | 是  | string  | 商品数据库ID，如果进行了对接，和数据对接传过来的id一致  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
       "esl_list": [{
            "esl_id": ”1000“,
            "esl_code": "DJKS90EN",
            "template_id": ”10002“,
            "status": 1,
     } ...
    ]}
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5020  | 参数错误  |  
| 5301  | 非法价签  |  
| 5015  | 非法商品  |  
| 5041  | 非法对接软件店铺  |  
### 3.3.5 获取商品列表
**接口描述** ：通过本接口调用，用户可以使用关键字搜索商品。
**请求链接** ：/product/getList
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| keyword  | 否  | string  | 关键字  |  
| page_num  | 否  | int  | 页码  |  
| page_size  | 否  | int  | 每页记录数  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    "code": 0,       /* 其他错误参考错误列表 */
    "msg": "succeed"
    "data": {
        "total_count": 1，
        "product_list": [
            {
                "id": ,
                "name": ,
                "seq_num": ,
                "bar_code": ,
                "category_id": ,
                "price": ,
                "modified_time":
            }
        ],
    },
 }
  

```

**错误码：** （历史原因，成功为0，失败为1）
### 3.3.6 获取商品信息
**接口描述** ：通过本接口调用，用户可以获取商品信息。
**请求链接** ：/product/getInfo
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| product_id  | 是  | string  | 商品数据库ID，如果进行了对接，和数据对接传过来的id一致  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
        "id":,
        "name":,
        "alias":,
        "seq_num":,
        "bar_code":,
        "qr_code":,
        "unit":,
        "spec":,
        "area":,
        "level":,
        "brand":,
        "expire_time":,
        "price":,
        "promote_price":,
        "member_price":,
    }
}
  

```

**错误码：** （历史原因，成功为0，失败为1）
上一篇：对接引导
下一篇：价签管理接口
