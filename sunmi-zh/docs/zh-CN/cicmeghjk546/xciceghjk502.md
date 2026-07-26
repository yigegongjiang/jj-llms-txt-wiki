---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xciceghjk502
---

# 设备配置
更新时间：2026-04-08 19:02:04
以下功能仅在未使用配置文件方案的账户显示。若账户已经选择使用了配置文件，请参考配置文件功能说明。
# 全局打印效果
商米手持机 支持更改系统设置（V1 系统版本需要升级到 2.10.3 及以上版本），将打印的字体高度、宽度增大，还可以设置字体加粗、反白、下划线和设置每行高度；而合作伙伴可以在后台批量设置渠道设备打印机输出的效果(强行覆盖 APP 的打印设置)，该设置需打印服务版本升级到4.0及以上版本才可满足。
设置打印样式的步骤：点击“设备管理”页面中的“全局打印效果”,根据需求设置打印的效果；
  

![](https://cdn.sunmi.com/public/image/mgt-document/7ab4b06cbf714289863a0aae4ae06626.png)
# 打印纸规格
为合作伙伴提供了选择打印纸规格和打印模式的选择项，方便合作伙伴批量设置。
操作说明：
“选择机型”->“选择打印纸规格“->“选择打印模式”->在选择黑标热敏之后，观察右侧的示意图，根据用户保存的值进行走纸，以此来确定黑标纸上最后的切刀位置与黑标的距离。
  

![](https://cdn.sunmi.com/public/image/mgt-document/97a7245699444688bac45aa9d2e4f5bf.png)
# 打印设置
此页面为合作伙伴提供了打印报警开关的控制项。
操作说明：
根据机型控制功能开关达到效果为，在关闭报警开关后，系统将不会自动弹出打印机相关的异常报警框（如缺纸、过热报警），仅保留非系统APP自定义的弹窗。
  

![](https://cdn.sunmi.com/public/image/mgt-document/e4f201ce8e9c470e8f6a011cb4d4c548.png)
# 扫码头设置
扫码头设置可以让客户在云端对渠道下具备扫码头的设备进行设置。
  1. 输出编码设置→确认输出的编码类型
  2. 提示方式设置→可以选择声音提示
  3. 触发模式设置→选择短按触发，或者连续扫码等
  4. 输出方式设置→如需要软件后台监听输出数据，请打开广播方式输出


注： 如果设备本身的设置和云端设置不一致，以本地设置为主。但是如果清除了本地设置，会自动同步云端配置过来。
  

![](https://cdn.sunmi.com/public/image/mgt-document/abcd56bfe92d46098ff2c31cd8928b48.png)
# 通用设置
渠道合作伙伴通过“通用设置”可实现以下功能：
1、 “第三方应用安装”：除了默认从商米应用市场获取应用外，是否允许设备安装从其他渠道（网页、本地等）获取的应用。（此功能对金融设备无效，P1 金融设备仅允许从应用市场下载安装应用）
2、 “硬件管家钱箱指令测试页面”：控制设备上硬件管家内测试钱箱弹出是否需要密码，如不需要则可以选择关闭。默认密码为16.20.0.0.0并且无法更改
3、“设备激活时显示商米ID流程”：关闭后，在机器激活时将不再显示商米ID登录注册模块，终端商户将不能更好的享受强大的商米云服务。提示：该配置仅适用于最新版且已适配的ROM版本。
4、“USB调试保护”：可设置本渠道设备通过 USB 进行调试前，是否需要对调试员身份进行授权确认。
5、“启用谷歌设置”：海外版机器可设置本渠道设备开启/关闭谷歌服务及谷歌相关应用。
![](https://cdn.sunmi.com/public/image/mgt-document/b6e3b687f9bf4165b394f8da2df5de69.png)
![](https://cdn.sunmi.com/public/image/mgt-document/45cb26d3580841b38bb1acb606b43f7d.png)
![](https://cdn.sunmi.com/public/image/mgt-document/12e3b13e319f4f0e85162fdbdfda882f.png)
![](https://cdn.sunmi.com/public/image/mgt-document/00f9df15012143c38bff9b91606c033d.png)
# iBeacon设置
本功能入口已经隐藏，若有需求，可以通过链接访问：<https://partner.sunmi.com/equipmentConfig/ibeacon>
iBeacon 是O2O 中连接线上线下的节点，此类应用已经受到越来越多的商用开发者关注，可用于微信摇一摇营销推广，淘宝摇一摇等场景。 为了渠道合作伙伴能实现更好的营销推广，支持蓝牙功能的商米设备都配备了 iBeacon 功能，允许将设备设置为 iBeacon 基站。
设置办法有2种，您可以任选其一：
  1. 合作伙伴后台设置：


设置步骤： 进入“设备管理”的“iBeacon设置”功能，来配置 iBeacon 的 UUID（通用唯一标识符）、Major、Minor。
  1. 您也可以在设备内开启iBeacon：


设置步骤：进入设置->其他设置-> iBeacon 开启，并配置相关信息。
设置效果查看：设置完成后即可让顾客使用手机摇一摇来发现该营销点，点击跳转到目标链接，实现关注商家微信号，接收商家推送的优惠信息、品牌营销信息、限时抢购信息等营销功能。
注：在没有设置 iBeacon 的情况下，商米设备默认广播商米微信号用于宣传。
  

![](https://cdn.sunmi.com/public/image/mgt-document/c93bddc548974a9495e15d87fe3374f2.png)
举例说明微信摇一摇 iBeacon 的设置方法：
  1. Beacon 平台准备工作，到微信公众平台申请摇一摇周边功能，具体申请方法请至微信平台查看帮助。（申请方法：<https://zb.weixin.qq.com/> ）
  2. 在微信平台申请通过之后会分配一个 UUID, Major,Minor；UUID 在微信平台是统一的，Major 和 Minor 会不一样，用于区分不同的公众号。
  3. 进入商米合作伙伴平台，找到“我的 iBeacon”页面，选择需要配置 iBeacon 的设备设置 iBeacon 三项参数（UUID, Major,Minor）
  

![](https://cdn.sunmi.com/public/image/mgt-document/3e9804ac43d64187b259417bb20e869a.png)
  4. 保持设备联网状态下，点击确定，iBeacon 设置会实时生效，使用手机微信周边摇试试，激活设备就可以使用了。


  

![](https://cdn.sunmi.com/public/image/mgt-document/19c9bf6943e44d968046ff76ec383385.png)
  

# 读卡数据格式（T1 T2Lite D1等）
本功能入口已经隐藏，若有需求，可以通过链接访问：<https://partner.sunmi.com/equipmentConfig/cardReader>
T1、T2lite、T2、S2、X2、D1、T2s、T2s_LITE加装 MSR/RFID 读卡器模块后，可通过“读卡器设置”来配置商米读卡器模块的数据输出格式。数据以模拟键盘输入的方式输出。开发者也可通过指令来获取磁卡数据。
注意：配置区域的磁道数据为模拟磁道数据，用户使用时磁道数据为磁卡的磁道数据。
读卡数据格式设置步骤：点击“设备配置”页面中的“读卡数据格式”,根据需求设置起始符和结束符。
  

![](https://cdn.sunmi.com/public/image/mgt-document/043eedb687ce41ebadb26b93afdb4cae.png)
##   

上一篇：系统定制
下一篇：配置文件操作说明
