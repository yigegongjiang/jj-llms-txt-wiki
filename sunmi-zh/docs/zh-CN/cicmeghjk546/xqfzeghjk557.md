---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xqfzeghjk557
---

# 远程密钥注入
更新时间：2026-07-10 17:42:29
# 使用前请阅读
## 对于已经使用RKI的客户
非常感谢您的选择和持续的宝贵建议。我们很清楚，RKI仍然需要前进和优化。
所以我们做了这个调整。我们正在尝试将SUNMI RKI功能合并到合作伙伴平台中，以便您可以使用合作伙伴帐户来管理设备并同时注入密钥，而无需管理两个不同的平台和帐户/权限。
因此，以后可以登录partner.sunmi.com继续使用RKI功能，同时，RKI数据不会改变。请不要担心
## 合并到合作伙伴平台的好处
  * 减少管理负担: 调整后，只保留一个平台，不需要同时管理两个不同的平台，这将减轻您对It的管理负担。
  * 使用更轻松，更快捷: 调整后，设备信息将与合作伙伴平台相同，无需再次导入; 此外，在设备管理中将有更有效的功能，例如创建下载任务，以加快操作速度。
  * 更统一的数据: 调整后，客户可以更方便地查看和分析数据。


## 初始密钥交换
在使用RKI进行密钥注入之前，请确保您已经与我们的RKI技术团队进行了沟通并交换了初始密钥 (请参阅文档 'Sunmi RKI安全初始化指南。pdf' 用于初始密钥的概念以及如何交换它)。
现在，SUNMI RKI可以支持在线交换初始密钥。
## 我们目前支持的密钥 (不定期更新)
支持: DUKPT，MK/SK
支持: 3DES，AES128(AES192和AES256将很快支持)
支持: 蛋糕 \ TMK \ PIK \ MAK \ TDK \ DUKPT_BDK \ DUKPT_IPEK \ KBPK
支持: TR31键/普通键
支持: 自定义KSN
# 定义  
| 参数  | 评论  |  
| --- | --- |  
| KEK  | 密钥加密/交换密钥  |  
| TMK  | 终端主密钥  |  
| PIK  | PIN键  |  
| MAK  | Mac密钥  |  
| TDK  | 帐户数据加密密钥  |  
| DUKPT_BDK  | DUKPT基础派生密钥  |  
| DUKPT_IPEK  | DUKPT初始PIN加密密钥  |  
| KBPK  | 密钥块保护密钥  |  
# 操作说明
RKI功能在 “支付安全中心” 中涉及，如屏幕截图所示。
![](https://cdn.sunmi.com/public/image/mgt-document/4f7bf157c7ec4e7095c1cdb0074da581.png)
## 设备管理
在这个阶段，我们将设备分为两个部分
  * 设备属于您的渠道。
  * 设备属于其他渠道。


![](https://cdn.sunmi.com/public/image/mgt-document/2313e191c4324bce99bb677849c3c9b1.png)
### 设备属于您的渠道
  * 所有设备属于您的频道，将在此处自动显示 (高亮)，无需再次上传。
  * 显示在这里的设备的条件:
    * 只有支付设备将显示在这里，如P系列; T/D/K/V等将不显示在这里。
    * 如果设备不属于您的渠道，则不会在此处显示。


![](https://cdn.sunmi.com/public/image/mgt-document/5b7fa692e44d45409c2c7ee948ca0884.png)
### 设备属于子账号
您公司下的所有 控制型子机构 会全部显示在“子账号”下
选择不同的子账号后，右侧的设备列表会显示出该子账号下的所有设备，无需上传。
![](https://cdn.sunmi.com/public/image/mgt-document/846323997cef4eef93c0de8e24cc150b.png)
### 设备属于其他渠道
由于合作伙伴平台不允许上传设备，但与此同时，我们意识到我们的一些客户将为其他公司的设备注入密钥。
因此，我们在这里设置了一个区域，供我们的客户上传其他公司的设备并为其注入密钥。
![](https://cdn.sunmi.com/public/image/mgt-document/6f68bf44135f42f7b5bf64b1eca5cf37.png)
### 如何添加其他公司
  * 首先和这家公司互相添加 “好友”
  * 让该公司在“好友”页面里，对您公司进行授权，看到“RKI”表示授权成功
  * 之后，您可以在 您的RKI菜单里的“其他公司设备” 中看到此合作伙伴，即可上传该公司的设备


![](https://cdn.sunmi.com/public/image/mgt-document/149fd643c20a467b82b66dd22f1192e7.png)
### 3.1.4如何上传其他公司的设备
  * 当你在这里看到其他公司时，你可以上传他们的SN文件。
  * 系统将检查SN文件，设备将成功上传，如果SN:
    * SN属于该公司。
    * SN是P系列设备。


![](https://cdn.sunmi.com/public/image/mgt-document/db42b977a47b4c7ab3d37da2e124a4f0.png)
  

  * 点击 “上传设备” 并上传SN文件，我们支持.csv/.xls/.xlsx格式


![](https://cdn.sunmi.com/public/image/mgt-document/94c8e35e143b4d11bccc3a14989f4ab5.png)
### 导出设备
_您可以使用此功能导出SN列表，请注意，不要一次导出一些许多设备，它会很慢。建议批量导出_
![](https://cdn.sunmi.com/public/image/mgt-document/538ac7fc3f7b414483f1268d0f6f8166.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/a01b0c8335c64e168af115f17c34f616.png)
### 设备组
![](https://cdn.sunmi.com/public/image/mgt-document/67a8bd7ea1324cdbae007b329f53f5ac.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/888e540c2c924d50abc0f22885fb9ceb.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/23b75a94929d48f1b2dbad47dcca6e9c.png)
### 操作日志和操作进度
![](https://cdn.sunmi.com/public/image/mgt-document/09b6d44554c34478abe03f15e27645e1.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/e35680a2efe24a78bceb24d8bcfcaf50.png)
### 锁定/解锁
  * 当设备下载已分配的密钥时，设备状态将更改为 “锁定”，以避免再次注入。如果您分配新密钥，并且要注入新密钥，则需要先 “解锁” 设备。
  * 选择 “就绪” 或 “锁定” 以快速过滤设备。
  * 单击 “解锁” 按钮，以解锁一个特定设备。
  * 点击 “批量解锁” 按钮，通过导入文件的方式一次解锁多个设备。


![](https://cdn.sunmi.com/public/image/mgt-document/7a80518b3ce04c62932edda22d38782b.png)
## 密钥管理
在此阶段，上传的是经过初始密钥加密后的工作密钥，如BDK，TMK等；上传的密钥是用来最终注入到设备里的。
### 上传密钥
单击 “密钥” 并跳转到密钥管理页面。
![](https://cdn.sunmi.com/public/image/mgt-document/22304015700541b3baf457128c5c9f76.png)
  

输入密钥信息，然后单击 “确定”
![](https://cdn.sunmi.com/public/image/mgt-document/613ac33a7500477698ab559a267fb081.png)
字段说明:  
| 字段  | 描述  |  
| --- | --- |  
| 密钥名称  | 通常用于描述钥匙的目的，以方便后续管理。PIN_BDK_3DES_16  |  
| 密文密钥  | 您要下载到设备中的密钥，需要通过初始密钥进行加密。  |  
| KCV（Key Check Value）  |  3DES: 通过明文密钥使用ECB模式加密16个零 AES: 使用CMAC通过明文密钥加密16个零  |  
| Key Index（密钥索引）  | 通常，索引是您的支付应用程序需要使用此密钥来加密PIN/数据/MAC  |  
| 密钥类型  | 查看文档的第二部分：定义  |  
| 算法类型  | 3DES或AES  |  
| 密钥长度  |  3DES: 128, 192 AES: 128, 192, 256  |  
| 软件包名  | 允许使用此密钥的Android APK包名称。如com.sunmipayment  |  
### 自定义KSN
  * 当您选择BDK时，KSN将更改为 “KSN生成规则”，您需要选择 “根据SN随机生成” 或 “自定义KSN”。
    * 根据SN随机生成: 您只需要定义KSN的前3个字节 (6位)，KSN的其他部分将由每个SN随机派生。
    * 自定义KSN: 您可以定义KSN的每个部分，KID \ DID \ 计数器 \ Increment(自增规则），KSN将按照配置递增。


![](https://cdn.sunmi.com/public/image/mgt-document/b32a17bd09d54a8b8bb37e0f34b6d29d.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/6d749bc4a9ad43fc9a5f1e3549c19d5b.png)
### 密钥组
![](https://cdn.sunmi.com/public/image/mgt-document/f343a3c0eb3d4d4ab4ba24c60f7e09e1.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/fd55b82faf04408ba46b51c5c9d01bd7.png)
  

### 密钥操作日志
![](https://cdn.sunmi.com/public/image/mgt-document/791b608ab47049c7b8f71c68a4424854.png)
### 生成随机密钥
  * 密钥均由商米合规的加密机生成而来
  * 随机密钥主要用于用户在还没有密钥的初期，熟悉整个密钥注入流程
  * 如果您允许，随机生成的密钥也可以用于您的业务流程


![](https://cdn.sunmi.com/public/image/mgt-document/c7c43b69a9fa4dbea13a8b8de877b3f3.png)
![](https://cdn.sunmi.com/public/image/mgt-document/c6f629232b8a477ba5840e713c7fa656.png)
## 密钥分配
_在此阶段，指定将哪个密钥注入哪个设备。_
### 分配密钥
  * _单击 “密钥分配” 并跳转到密钥管理页面。_
  * 单击上面的 “分配密钥” 按钮或SN后面的 “分配密钥”，然后将弹出分配密钥对话框。


![](https://cdn.sunmi.com/public/image/mgt-document/4fb4b4ca43f045528b3019f69e000d34.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/79053516e5de400cba45cfb80d9a4434.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/504899528cc04a68a8cf018969ccdb22.png)
  * 选择设备和密钥，单击 “确定” 按钮，会将密钥分配给设备，**此时并不代表密钥注入，仅仅是分配** 。
    * 您可以将多个密钥分配给一个设备。这意味着这个设备后续需要下载多个密钥。
    * 您还可以将一个密钥分配给多个设备。这意味着该密钥需要注入到多个设备中。


![](https://cdn.sunmi.com/public/image/mgt-document/96fd6a3d1c3844d898b8b4eb2b11667a.png)
## 密钥下载记录
_在此阶段，您可以检查所有密钥下载记录并导出记录。_
![](https://cdn.sunmi.com/public/image/mgt-document/4bf7efcf2fb74b3dad81d3a5101567bf.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/6534e552544a49e8b9b1011a0a8e51ed.png)
## 初始密钥交换
在此阶段，您可以在线与我们交换初始密钥。它可以显著提高初始密钥交换的效率，并降低您成本和周期。
您可以先跳过此过程，但它会影响您后续的使用
![](https://cdn.sunmi.com/public/image/mgt-document/b550bb4b720b4cde9fb338fadc5cc169.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/2e4d708bce804cf0a3839d665c2f3c0a.png)
  

按照步骤在线交换初始密钥。
![](https://cdn.sunmi.com/public/image/mgt-document/72e1a942a7414d14b8db64362fc7a74c.png)
## 许可证 (仅适用于预付费客户)
  * 只有预付费客户才有此功能。
  * 如果您是后付费客户，则不会看到此功能。
  * 点击 “许可证”，您可以看到 “总下载量” 和 “剩余下载量”。
  * 确保有足够的剩余下载。


![](https://cdn.sunmi.com/public/image/mgt-document/9f5855e1547047b79e98303ece62bfcf.png)
## 自动下载密钥
  * 此功能需要支持特定的ROM版本或更高版本。
  * 该任务将持续30天。
  * 单击 “创建下载任务” 为一个设备创建。
  * 选择多个设备并一次性批量创建下载任务。


![](https://cdn.sunmi.com/public/image/mgt-document/375412ab2c354d82ba8bc7266f12cf69.png)
### 创建任务
批量创建和单个创建都显示相同的对话框。
![](https://cdn.sunmi.com/public/image/mgt-document/e5381f8e9af641f2bd6b3e17720e16c2.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/95e4b0d7eaa34a10aa94a27ae1e84dc7.png)
  

### 检查下载任务
  * 您可以单击 “未完成” 和 “已完成” 来切换任务的状态。
  * 单击 “终止” 以终止未完成的任务，然后任务将移至 “已完成”，并带有备注 “手动终止”
  * 总的来说，备注有四个结果:
    * 手动终止
    * 正常结束
    * 被动终止: 从设备解除绑定密钥
    * 被动终止: 任务到期
  * 通过过滤时间和备注后导出任务报告


![](https://cdn.sunmi.com/public/image/mgt-document/b9b73d6be541432db614187871a54f33.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/a2f9e6ff61e14caeaafa53343dbb2a22.png)
### 延长任务有效期
勾选任务批量延长有效期，或单台延长有效期
![](https://cdn.sunmi.com/public/image/mgt-document/de3c146b78f340c88fabd76b8f8141bd.png)
点击“设置自动延期”，可以设置全局的自动延期
![](https://cdn.sunmi.com/public/image/mgt-document/39bf5f319bd34fbd92388593fdda35b8.png)
### 自动密钥下载的重试机制
  

![](https://cdn.sunmi.com/public/image/mgt-document/860a90f4101a413c99a638b0dd88c92d.png)
考虑到某些设备的网络较差，自动密钥下载功能可能无法一次性成功，这将导致密钥下载失败。
我们优化了推送机制:
  1. 一旦设备连接到网络，云立即开始推送。
  2. 云端将每隔几分钟再次推送，直到设备返回成功的密钥下载消息。


如果在创建任务后发现任务尚未立即完成，请等待几分钟。
一般来说，如果设备的网络只是速度慢而不是断开连接，它将在5-10分钟内完成。
## 帐户和权限
您可以通过 “操作员” 为每个用户设置权限。
![](https://cdn.sunmi.com/public/image/mgt-document/a7d92b4a99d145aca284d4828a070b62.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/2532c498bcb6400b8ed050948d8942c2.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/01f89db73b674951b9aa2bdd494fda29.png)
## 审计日志
通过系统管理里的 “审计日志” ，并筛选RKI内容，可以查到相应的所有日志。
![](https://cdn.sunmi.com/public/image/mgt-document/3f99dca8025748f1bc5cc402e3d7522d.png)
  

上一篇：财务管理
下一篇：DMP跨品牌及跨平台能力接入说明2026版
