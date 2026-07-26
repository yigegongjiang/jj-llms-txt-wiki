---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xffceghjk502
---

# 2、开通云打印对接能力
更新时间：2025-11-11 15:44:52
本功能仅支持以下机型和版本：
58票据云打印机，Model：NT21x，Firmware version: 2.4.0，SUNMI APP Version：2.6.0 以上设备使用；
80后厨云打印机，Model：NT31x，Firmware version: 2.7.0，SUNMI APP Version：2.19.0以上设备使用；
80扫码打印机，Model：NT32x，Firmware version: 4.1.19，SUNMI APP Version：4.1.32以上设备使用；
如需升级请联系客服！
## **特别声明：**
> 商米云打印机对接通道为更好的保护合作伙伴的数据隐私，又能实现完整的订单打印过程，特订立以下对接规范，并需要商米与合作伙伴双方共同努力才能实现打印通道业务。
# **注册合作伙伴平台账号：**
> 关于注册合作伙伴平台账号流程请参考：<https://developer.sunmi.com/docs/zh-CN/xeghjk491/cmieghjk579>
  1. 合作伙伴业务基于商米云提供的安全保障前提下，所以要求合作伙伴需要拥有一个商米合作伙伴平台账户。
  2. 要求打印机设备在商米公司出库时，出库渠道一定要选择该账户下，否则打印机会因归属权问题无法进行下一步绑定业务。


# **创建应用和关联能力：**
1.登录【商米伙伴平台】，注册成为【开发者】。点击【开发服务】-【应用接入】-【创建接入能力的应用】，在创建应用界面输入“应用名称”，选择“云应用”，点击【提交】
  

![](https://cdn.sunmi.com/public/image/mgt-document/cbd44d5de0f64bc883c89aad9cee36cb.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/0912f9d62aea4a0fa2b4008c3ca636ca.png)
2.创建应用成功后，还需进一步关联能力，点击应用卡片进入【应用详情】
  

![](https://cdn.sunmi.com/public/image/mgt-document/8710f9e13eaf4d44a4a979cfd5c1218d.png)
3.详情中提供了【AppID】【AppKey】，请复制下来备用。
点击【添加能力】，勾选【云打印】能力，将“云打印”能力添加到“关联能力列表”中
  

![](https://cdn.sunmi.com/public/image/mgt-document/2d30f04ffe9848268c750388321c30c9.png)
4.应用关联能力后，就可以开始调用能力对应的接口。
> 具体接口请求规格请参考：<https://developer.sunmi.com/docs/zh-CN/xeghjk491/fdxeghjk491>
# **能力设置：**
1.点击【开发服务】-【应用接入】，点击应用卡片进入【应用详情】，在关联能力列表选择【云打印】-【设置】
  

![](https://cdn.sunmi.com/public/image/mgt-document/774faa8bae4c4232b3e284d9ee60cf14.png)
2.云打印机设置中，依据票据打印的隐私性选择【直推模式】或【回调模式】。具体说明参考[对接流程](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xffaeghjk480)
“【回调模式】订单请求地址”：打印机使用【直推模式】时不需要填写，使用[直推模式接口](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xffdeghjk524)给打印机直接推送订单；需要使用【回调模式】时请输入合作伙伴云平台订单请求URL地址，使用[回调模式接口](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xffdeghjk524)推送订单消息，并[开发回调接口](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfffeghjk535)供打印机调用订单。
输入的回调地址URL为合作伙伴云平台的域名和接口路径，不需要包含回调接口名。
比如输入的回调URL地址为【http://xxx.yyy.com/api】，那么打印机会自动拼接请求下面三个接口完成订单打印：
【http://xxx.yyy.com/api/printTicket/getPrintTicketOrderId】
【http://xxx.yyy.com/api/printTicket/getPrintTicketInfo】
【http://xxx.yyy.com/api/printTicket/updatePrintTicketStatus】
  

![](https://cdn.sunmi.com/public/image/mgt-document/ad401be45d0d4349af91ec4b7ba38b08.png)
3.云打印机设置中，提供了合作伙伴获得打印机的实时运行状态的能力。
当需要获得实时状态能力时，首先需要合作伙伴在云端开发一套状态信息接收接口。具体开发说明参考[回调设备信息API接口](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xffmeghjk546)；然后将开发好的接口URL地址在“设备状态回调地址”中输入；最后选择需要回调的设备状态信息；
【打印机基础信息】：当设备开机时，就会上传一些版本号、设备SN号等硬件基础信息；
【打印机状态信息】：当设备实时发生开盖、切刀异常、开钱箱、经纬度等变更时，就会上传；
【打印机定时信息】：设备在每间隔一段时间后，就会将打印公里数、开钱箱次数、信号强度等信息上传；
【打印机在线信息】：设备每次在线/离线动作发生，就会上传；但由于网络异常，设备离线的信息并不准确，无法做到及时，需要滞后几分钟。
输入的状态回调URL地址是完整的域名和接口名，并且符合POST请求参数的要求：
比如输入的回调地址为【http://xxx.yyy.com/api/reportDeviceInfo】，其中【http://xxx.yyy.com/api/】是域名和路径，【reportDeviceInfo】是对应的接口名。
  

![](https://cdn.sunmi.com/public/image/mgt-document/5b89946c52e74fa39ea0fc0ebf8e29a3.png)
4.点击【保存】完成能力设置。
# **能力列表和关联应用：**
1.点击【开发服务】-【我的能力】，开发者可以查看可对接的所有能力；
  

![](https://cdn.sunmi.com/public/image/mgt-document/5abb27f017014729b8e32d528ab05533.png)
2.点击【能力信息/详情】，开发者可以查看能力的基本信息；
  

![](https://cdn.sunmi.com/public/image/mgt-document/3fa69db6d6064522a955806676da66bd.png)
3.点击【接入管理】，开发者可以查看能力已关联的应用
  

![](https://cdn.sunmi.com/public/image/mgt-document/89a68eecc306407f80361d2b7c24e229.png)
上一篇：1、了解云打印机业务对接流程
下一篇：3、开发对接API接口
