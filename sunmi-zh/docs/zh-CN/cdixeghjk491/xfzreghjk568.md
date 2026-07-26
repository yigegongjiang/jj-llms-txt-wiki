---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzreghjk568
---

# 模板管理接口
更新时间：2025-09-25 17:57:59
## 7.1 接口描述
模板管理接口用来对模板进行上传修改等操作，如果需要完全对接电子价签系统，不使用商米数字店铺任何功能，则需要对接这部分接口。
## 7.2 接口列表  
| 接口名称  | 接口  |  
| --- | --- |  
| 上传新建模板  | /template/create  |  
| 更新指定模板  | /template/update  |  
| 获取模板列表  | /template/getList  |  
| 获取模板详情  | /template/getInfo  |  
| 删除模板  | /template/delete  |  
## 7.3 接口详情
### 7.3.1 上传创建模板
**接口描述** ：通过本接口调用，用户可以通过上传json格式的模板来创建新模板。模板json文件可以在数字店铺中下载，也可以在独立提供的模板设计网页中下载。
**请求链接** ：/template/create
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| template_name  | 是  | string  | 模板名称  |  
| template_color  | 是  | int  | 模板支持的颜色类型 (1-黑白，2-黑白红)  |  
| template_screen  | 是  | int  | 模板支持的屏幕类型 (1 – 2.13寸，2 – 2.6寸，3 – 4.2寸)  |  
| template_json  | 是  | string  | json格式的模板  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
        "template_id": "1000"
    }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5005  | 文件错误  |  
| 5343  | 非法模板  |  
| 5346  | 模板名称已存在  |  
| 5041  | 非法对接软件店铺  |  
### 7.3.2 更新指定模板
**接口描述** ：通过本接口调用，用户可以更新指定模板。
**请求链接** ：/template/update
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| template_id  | 是  | string  | 模板唯一标识ID  |  
| template_name  | 否  | string  | 模板名称  |  
| template_json  | 否  | string  | json格式的模板  |  
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
| 5005  | 文件错误  |  
| 5343  | 非法模板  |  
| 5346  | 模板名称已存在  |  
| 5041  | 非法对接软件店铺  |  
### 7.3.3 获取模板列表
**接口描述** ：通过本接口调用，用户可以获取模板列表。
**请求链接** ：/template/getList
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| page_num  | 否 (默认1)  | int  | 当前页码  |  
| page_size  | 否 (默认10)  | int  | 当前页条目数量  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
　　”total_count“: 15,
       "template_list":[{
              "template_id": "100",
              "template_name": "sample",
              "template_color": 1,  /* 暂不支持*/
              "template_screen": 1},
              ... ...
   ]}
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5020  | 非法参数  |  
| 5041  | 非法对接软件店铺  |  
**返回字段描述：color**  
| **color** 取值  | 说明  |  
| --- | --- |  
| 1  | 黑白  |  
| 2  | 黑白红  |  
**返回字段描述：screen**  
| **screen** 取值  | 说明  |  
| --- | --- |  
| 1  | 2.13 寸  |  
| 2  | 2.6 寸  |  
| 3  | 4.2 寸  |  
### 7.3.4 获取模板详情
**接口描述** ：通过本接口调用，用户可以获取模板的详细属性。
**请求链接** ：/template/getInfo
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| template_id  | 是  | string  | 模板唯一标识ID  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {
              "template_name": "sample",
              "template_color_name": "BW",
              "template_screen_type_name": "2.6",
              "template_json":  "...",
              "template_color": 1,     /* 暂不支持*/
              "template_screen": 1,  /* 暂不支持*/

   }
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5005  | 文件错误  |  
| 5343  | 非法模板  |  
| 5041  | 非法对接软件店铺  |  
**返回字段描述：color**  
| **color** 取值  | 说明  |  
| --- | --- |  
| 1  | 黑白  |  
| 2  | 黑白红  |  
**返回字段描述：screen**  
| **screen** 取值  | 说明  |  
| --- | --- |  
| 1  | 2.13 寸  |  
| 2  | 2.6 寸  |  
| 3  | 4.2 寸  |  
### 7.3.5 删除模板
**接口描述** ：通过本接口调用，用户可以删除指定模板。
**请求链接** ：/template/delete
**接口参数** ：  
| 参数名称  | 是否必须  | 类型  | 说明  |  
| --- | --- | --- | --- |  
| sunmi_shop_no  | 否  | string  | 商米数字店铺平台的门店组织编号  |  
| shop_id  | 否  | string  | 第三方对接软件中门店的标识，对接的软件提供 （作为门店唯一标识即可，shop_id与sunmi_shop_no可互为替代，使用时传输任意一个即可）  |  
| template_id_list  | 是  | array  | 模板唯一标识ID列表  |  
**返回值** ：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
    code：0，       /* 其他错误参考错误列表 */
    msg: "succeed"，
    data: {}
}
  

```

**错误码：**  
| 错误码  | 说明  |  
| --- | --- |  
| 5000  | 数据库错误  |  
| 5041  | 非法对接软件店铺  |  
上一篇：基站管理接口
下一篇：价签绑定货架编号
