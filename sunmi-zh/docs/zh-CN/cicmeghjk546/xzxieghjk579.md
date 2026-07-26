---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xzxieghjk579
---

# 商米OEMConfig

更新时间：2026-04-15 17:08:14

# 什么是OEMConfig

OEMConfig 是 **Android Enterprise 官方定义的标准化设备管理扩展框架**，用于解决传统 MDM/EMM 无法统一管理厂商专属硬件能力的问题。通过 OEMConfig，企业与开发者可在**不定制系统、不开发专属插件**的前提下，通过标准管理通道对设备硬件进行集中配置、远程下发与状态监控。

OEMConfig 以**托管配置应用（Managed Configuration App）的形式提供，由设备厂商基于 Android 标准规范开发实现。它作为EMM/MDM 与设备底层硬件服务之间的标准化桥梁**，将扫码头、外设接口、系统参数、固件升级等私有硬件能力，以统一、可配置的方式开放给企业管理平台，实现跨设备、跨厂商的一致化管控。

核心价值：

-   **标准化管理通道**：遵循 Google Android Enterprise 规范，兼容主流 EMM/MDM 平台，无需额外插件适配。

-   **硬件能力开放**：支持扫码头、串口、GPIO、显示、按键、外设、系统设置等厂商专属能力配置。

-   **集中远程部署**：可批量下发配置文件、参数策略、固件升级包，实现设备规模化运维。

-   **安全可控**：基于企业管理权限运行，配置仅对授权管理平台开放，确保设备与数据安全。


# 下载支持

-   APK 下载


附件：SunmiOEMConfig\_v0.0.31\_release.apk

# 支持的能力

<!-- prettier-ignore -->
| 功能 | 说明 |
| --- | --- |
| 蓝牙开关（Bluetooth Switch） | 设置设备蓝牙功能开启或关闭 |
| 屏幕熄屏时间设置（Screen Off Timeout） | 设置设备屏幕自动息屏时间 |
| 设备字体大小设置（Font Size） | 设置系统字体大小 |
| 显示缩放比例设置（Display Size） | 设置系统显示大小 |
| 时间显示格式设置（Time format ） | 设置系统时间格式：12 小时制/24 小时制 |
| APN配置（APN） | 添加 APN 配置 |
| 设备语言设置（Language） | 设置系统语言（默认英语） |
| 设备数据漫游开关（Data Roaming） | 设置设备数据漫游功能开启或关闭 |
| 打印警告弹窗显示管理（Printer Alert Switch） | 切换打印机告警弹窗是否显示 |
| 锁定屏幕方向（Locked Orientation Positions） | 设置屏幕方向，并锁定（关闭自动旋转） |
| 电源键控制(Power Key Management) | 设置机身电源按键是否可用（仅支持Flex3） |
| NTP 时间同步频率（NTP Time Synchronization Frequency） | 设置系统 NTP 时间同步频率 |
| 霸屏模式导航栏状态管理（Kiosk Mode Navigation Bar State Management） | 设置霸屏模式下导航栏显示状态 |

# 如何使用

本指引以 SOTI 为范例，其余 EMM 平台均可支持

## 前置准备

1.  确认设备已成功 enrollment 至 SOTI MobiControl 平台，且已归类至对应设备组

2.  获取 OEMConfig App APK


## 上传应用

进入「应用」点击「添加应用」，上传安装包并填写应用信息。

![](https://cdn.sunmi.com/public/image/mgt-document/2a07fdc0363c45779618fd89bbdbfabc.png)

选择 Android Enterprise

![](https://cdn.sunmi.com/public/image/mgt-document/e0d68ff4aa5447ebada1220544669b29.png)

添加应用，选择通过Enterprise App Store 关联

![](https://cdn.sunmi.com/public/image/mgt-document/58c1afe878474a5eb3f2d1e8dc7cc814.png)

![](https://cdn.sunmi.com/public/image/mgt-document/86ce7d7155584890a9ec05192e53338a.png)

可选择已有应用或点击+号上传新应用

![](https://cdn.sunmi.com/public/image/mgt-document/10777c133da84f9789cffebf9ba48e34.png)

上传 apk 新增应用

![](https://cdn.sunmi.com/public/image/mgt-document/c722ef0c06084c4c9071c5963b647efc.png)

添加完成

![](https://cdn.sunmi.com/public/image/mgt-document/c2de3d88b6cf4163a5c912635583a7bd.png)

## 配置 OEMConfig

![](https://cdn.sunmi.com/public/image/mgt-document/215ad78db79c4da89d9629c7d13aab5c.png)

启用 App Config

![](https://cdn.sunmi.com/public/image/mgt-document/5c37d7d7266d4e3aa1af7d31b8a46045.png)

![](https://cdn.sunmi.com/public/image/mgt-document/2620b5ff76a3412e8cd7edd9f72f7fa8.png)

设置并保存配置

## Config 配置关联

![](https://cdn.sunmi.com/public/image/mgt-document/f87e7e2b0f4b43cfa155ba8d4943a7d8.png)

关联分组

![](https://cdn.sunmi.com/public/image/mgt-document/1f4a4356feec4c4691ee398cbfe9d750.png)

![](https://cdn.sunmi.com/public/image/mgt-document/6a9a00d1691a463db1d17129b40bac2f.png)

完成配置，查看分组内设备是否同步 OEMConfig App 以及完成配置

---

上一篇：如何使用安卓AER解决方案
下一篇：商米投播用户手册
