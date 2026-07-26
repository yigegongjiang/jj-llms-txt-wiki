---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmaceghjk502
---

# 磁条卡服务说明

更新时间：2025-09-29 14:53:59

### 一、功能概述

商米提供支持磁条卡服务的刷卡SDK（SunmiMSCardService），其功能包含两部分：

在符合ISO7810/7811规范的读卡器进行刷卡动作，支持返回MSR中读取的所有数据，系统的焦点输入框会自动返回的读卡内容，并对磁轨信息进行区分。

详细磁条卡内容如以下：

#### 第1磁道数据格式

数据编码最大记录长度为79个字符，格式如下所示。

<!-- prettier-ignore -->
| 字段 | 长度 | 内容 | 说明 |
| --- | --- | --- | --- |
| 起始字符 STX | 1 | % | 第一磁道起始标识 |
| 格式代码 FC | 1 | B | 表明格式是B |
| 主账号 PAN | <=19 |  | 标明可以处理交易的发卡机构(issuer identification) <br>和持卡者(individual account identification) |
| 字段分隔符 FS | 1 | ∧ | 分隔符 |
| 姓名 NM | 2~26 |  | 持卡人的姓名 |
| 字段分隔符 FS | 1 | ∧ | 分隔符 |
| 失效日期 ED | 4 | 格式YYMM | 如不定义失效日期该字段应为一分隔符 |
| 服务代码 SC | 3 |  | 标明银行卡可使用的服务类型， <br>如不存在或不指定该字段以一个分隔符代替 |
| 附加数据 DD | 11 |  | 存放卡片验证码（CVN）及对发卡机构 <br>有意义的任意数据，应使整个磁道数据 <br>不超过79个字符 |
| 结束标识 ETX | 1 | ? | 第一磁道结束标记 |
| 纵向冗余校验字符 LRC | 1 | ‘ ‘ |  |
| 备用数据 | 13 | 空格填充 |  |

#### 第2磁道数据格式

数据编码最大记录长度为40个字符，格式如下表所示。

<!-- prettier-ignore -->
| 字段 | 长度 | 内容 | 说明 |
| --- | --- | --- | --- |
| 起始字符 STX | 1 | ； | 第二磁道起始标识 |
| 主账号 PAN | <=19 |  | 标明可以处理交易的发卡机构和持卡者 |
| 字段分隔符 FS | 1 | \= | 分隔符 |
| 失效日期 ED | 4 | 格式YYMM | 如不定义失效日期该字段应为一分隔符 |
| 服务代码 SC | 3 |  | 标明银行卡可使用的服务类型， <br>如不存在或不指定该字段以一个分隔符代替 |
| 附加数据 DD | 13 |  | 存放卡片验证码（CVN）及对发卡机构 <br>有意义的任意数据，应使整个磁道数据 <br>不超过40个字符 |
| 结束标识 ETX | 1 | ? | 第二磁道结束标记 |
| 纵向冗余校验字符 LRC | 1 | \_ |  |

#### 第3磁道数据格式

共113位，其中107位是银联中三磁道数据最大长度，格式如下表所示。

<!-- prettier-ignore -->
| 字段 | 长度 | 内容 | 说明 |
| --- | --- | --- | --- |
| 起始字符 STX | 1 | ； | 第三磁道起始标识 |
| 格式代码 FC | 2 | 99 |  |
| 主账号 PAN | 16 |  | 16位卡号， <br>标明可以处理交易的发卡机构和持卡者 |
| 字段分隔符 FS | 1 | \= | 分隔符 |
| 国家代码 CC | 3 | 156 | 如存在应为CCC格式的3个数字， <br>标明可以处理由银行卡产生交易的国家 |
| 货币代码 | 3 |  | 表明结算时使用的货币类型:3位数字 |
| 金额指数 | 1 |  | 决定周期授权量与本周期余额两字段的基值 |
| 周期授权量 | 4 |  | 由发卡机构自定授权量， <br>表示在一个周期内累积交易不能超过的金额 |
| 本周期余额 | 4 |  | 当前周期内的可用余额 |
| 周期开始日期 | 4 | 格式YDDD | 表示一个新周期开始的日期 |
| 周期长度 | 2 |  | 2位数字，表示所有交易的累积值不能超过授 <br>权量的时间期限 |
| 密码重输次数 | 1 |  | 记录允许未成功输入密码的次数 |
| 个人授权控制参数 | 6 |  | 提供一种可选择的安全性能 |
| 交换控制符 | 1 |  | 标明银行卡适用于交换的范围 |
| PAN的TA和SR | 2 |  | 定义PAN的账户类型和可提供的服务 |
| SAN-1的TA和SR | 2 |  |  |
| SAN-2的TA和SR | 2 |  |  |
| 失效日期 ED | 4 | 格式YYMM |  |
| 卡序列号 | 1 |  | 区别具有相同PAN的卡，由发卡机构定义， <br>在最初发卡或卡失效后换卡时赋值 |
| 卡保密号 | 1 |  | 用于建立磁条所含数据与物理卡的联系 |
| SAN-1 | 8 |  | 标明第一个可选用的辅助账号 <br>(first subsidiary account number) |
| 字段分隔符 FS | 1 | \= |  |
| SAN-2 | 0 |  | 标明第二个可选用的辅助账号 <br>(second subsidiary account number) |
| 字段分隔符 FS | 1 | \= |  |
| 传递标志 | 1 |  | 提供可减少传送交换信息长度的功能， <br>它表明交换信息是否包含附加数据的内容 |
| 加密校验数 CCD | 6 |  | 通过使用加密公式提供一种校验该磁道上 <br>数据完整性的方法 |
| 附加数据 DD | 8 |  | 存放卡片验证码（CVN）及对发卡机构 <br>有意义的任意数据，应使整个磁道数据 <br>不超过113个字符 |
| 结束标志 ETX | 1 | ? |  |
| 纵向冗余校验码 LRC | 1 | ‘ ‘ |  |
| 备用数据 | 19 | 空格填充 |  |
| 填充数据 | 6 | 空格填充至8的倍数位 |  |

### 二、快速开始：

附件：sunmiperipher\_sdk\_v1.0.2.aar

1.  **Android Studio的libs中导入**sunmiperipher\_sdk\_v1.0.2.aar

2.  app的build.gradle文件中 implementation fileTree(dir: 'libs', include: \['*.aar', '*.jar'\])


### 三、接口说明：

在Activity onCreate生命周期中初始化，并且打开设备

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
CardManager.init(this) { success ->
    ...
}
```

在需要时注册数据监听

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
CardManager.registerDataListener(listener)
```

在不需要时取消数据监听

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
CardManager.unregisterDataListener(listener)
```

获取模块信息

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
CardManager.getName()
CardManager.getVersion()
CardManager.getSn()
```

在组件销毁时可以销毁实例

bashccppcsharpcssgojavajavascriptjsonkotlinlessmakefilemarkdownobjectivecphpplaintextpythonrustscssshellsqlswifttypescriptxmlyaml

```
CardManager.destroy(this)
```

---

上一篇：NFC相关SDK说明
下一篇：PSAM、ETC、M112读卡开发
