---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xmfzeghjk557
---

# Windows OTA系统更新软件说明
更新时间：2025-12-10 21:44:24
# **1.软件概述**
## **1)产品简介**
SUNMI OTA（Over-The-Air）系统更新是一款专为商米设备设计的智能更新管理系统，支持远程推送系统更新、安全补丁和功能升级，包括系统配置、固件、驱动以及APP升级，确保设备始终运行在最佳状态。
## **2)核心特性**
  * **智能检测** ：支持多种触发机制（开机、定时、推送、手动）检查更新
  * **灵活下载** ：可配置下载限速、夜间升级、自动下载等策略
  * **安全校验** ：多重校验机制（MD5、签名校验）保障更新安全
  * **多模式升级** ：支持OTA远程升级、本地U盘升级
  * **智能提醒** ：个性化升级提醒，减少业务中断


## **3)系统要求**
  * **支持设备** ：T3 PRO MAX SUPER、T3 PRO SUPER、K2 SUPER、D3 WIN、Flex3 WIN
  * **系统版本** ：SUNMI WOS 1.0.4及以上版本
  * **系统架构：** ARM、X64、X86
  * **存储空间** ：至少5GB可用空间
  * **网络要求** ：需要接入互联网


# **2.升级全流程介绍**
## **1)检测流程**
### **触发检测机制**
系统支持四种检测触发方式：  
| **触发方式**  | **说明**  | **相关配置**  |  
| --- | --- | --- |  
| **开机检测**  | 设备启动时自动检查更新  | PARTNER平台配置  |  
| **云端推送**  | 管理平台主动推送更新  | MGT平台控制  |  
| **定时轮询**  | 按设定间隔自动检查  | 默认每12小时  |  
| **用户主动**  | 用户手动点击"检查更新"  | 系统更新界面  |  
用户可在此界面手动触发更新检测：
![](https://cdn.sunmi.com/public/image/mgt-document/ca87ebd8b71a401f9e20c07231e1c4f6.png)
图：检测更新界面
### **检测中状态**
检测过程中显示"检查中..."提示，保持用户感知。
系统正在检查可用更新：
![](https://cdn.sunmi.com/public/image/mgt-document/c36a1f277c944b61ac8a781c02894b6d.png)
图：检测中界面
### **检测结果处理**
**1.检测到新版本**
当检测到新版本时，显示版本信息：
  * 版本号（如：SUNMI OS 1.0.4）
  * 发布日期
  * 更新包大小
  * 更新日志


检测到新版本V1.0.4，大小416.64MB，发布日期为2025/10/28，更新日志为20251028：
![](https://cdn.sunmi.com/public/image/mgt-document/d21b440f22a84e66a223f97adffc73fd.png)
图：等待下载界面
**2.已是最新版本**
设备当前已运行最新系统版本。
![](https://cdn.sunmi.com/public/image/mgt-document/59012e4bc81444b792a21bdba31d363c.png)
  

### **检测失败处理**
**1.网络异常**
网络连接失败时提示用户检查网络。
网络不可用时显示的错误提示：
![](https://cdn.sunmi.com/public/image/mgt-document/cacab2c5f0d14c978a7d1d09ef1ae6e9.png)
图：网络异常页面
**2.检测超时**
检测过程中发生超时，系统会自动重试。
![](https://cdn.sunmi.com/public/image/mgt-document/0cf6e1ba0c3047e286356f09a9c3aeb2.png)
## **2)下载流程**
### **下载确认**
检测到新版本后，如果界面前台显示则显示等待下载界面；如果后台运行则系统会弹出下载确认对话框。
![](https://cdn.sunmi.com/public/image/mgt-document/101a8feb9398408d8b5a514c94f240c6.png)
![](https://cdn.sunmi.com/public/image/mgt-document/28550e6051b6449aa0881e3032136484.png)
### **下载进度**
下载过程中显示详细进度信息：
  * 下载百分比
  * 实时下载速度
  * 已下载文件大小


![](https://cdn.sunmi.com/public/image/mgt-document/d7a93297c5584ce78ceeed95d42d48de.png)
![](https://cdn.sunmi.com/public/image/mgt-document/9a6d41e9359b4616b33594318b5ce15b.png)
### **下载控制**
**1.暂停下载**
用户可随时暂停下载过程。
**（图：暂停下载界面）**
![](https://cdn.sunmi.com/public/image/mgt-document/89e8cc8524bd4dd09ef764aaf7714b5c.png)
**2.下载限速**
支持下载速度限制，避免影响正常业务：
  * **本地设置** ：在系统更新设置中配置
  * **云端强控** ：PARTNER平台可强制限速


![](https://cdn.sunmi.com/public/image/mgt-document/26bea7c9e7764381b87c9ae9f9f279a8.png)
![](https://cdn.sunmi.com/public/image/mgt-document/cbb8b99ba8414fc5a81c4c81a8ca639c.png)
### **下载失败**
下载过程中出现异常时会显示错误信息。
![](https://cdn.sunmi.com/public/image/mgt-document/dec02f983b904b5d9d22b638908ba31b.png)
## **3)校验流程**
### **校验过程**
下载完成后进行多重安全校验：
  1. **本地MD5校验** ：验证文件完整性
  2. **签名校验** ：验证更新包签名
  3. **云端状态校验** ：确认更新包未被撤销


![](https://cdn.sunmi.com/public/image/mgt-document/8922491927e34ce3ab034504b6839934.png)
### **校验失败处理**
**1.MD5校验失败**
文件损坏或不完整时提示重新下载。
![](https://cdn.sunmi.com/public/image/mgt-document/83c480dd0bb34d25ad3c53dbe33cfefd.png)
**2.签名校验失败**
数字签名验证失败，可能被篡改。
![](https://cdn.sunmi.com/public/image/mgt-document/64a350cf42474697b18a15462ccab1a6.png)
**3.更新包被撤销**
云端校验发现更新包已被管理员撤销。
![](https://cdn.sunmi.com/public/image/mgt-document/2295ba6e05c741a3a8208d99b0a46f02.png)
**4.更新包被删除**
更新包在校验前被意外删除。
![](https://cdn.sunmi.com/public/image/mgt-document/37fd5cd2a3974092abbb609da8aba8ee.png)
  

### **校验成功**
所有校验通过后，进入升级准备阶段。
  

## **4)更新升级流程**
### **升级确认**
校验成功后，系统提示用户确认升级。
![](https://cdn.sunmi.com/public/image/mgt-document/4f4511532141489f802aeef6099636cb.png)
图：等待更新重启
### **升级过程**
升级过程中显示进度提示，设备会自动重启。
![](https://cdn.sunmi.com/public/image/mgt-document/d3d6ef28b67342799950807bca5d26a5.png)
### **升级完成**
升级成功后系统会显示完成提示。
![](https://cdn.sunmi.com/public/image/mgt-document/6a4ae048dc3447ce8a0fb679874534d0.png)
### **升级失败**
升级失败后界面会显示升级失败提示，重新返回到检测更新界面。
![](https://cdn.sunmi.com/public/image/mgt-document/bc0b11db111a45e69e8746a4339b590d.png)
图：升级失败界面信息提示
## **5)本地更新**
### **本地更新准备**
支持通过U盘或移动存储进行本地更新：
  1. 将更新包重命名为 `summi_update.zip`
  2. 放置在存储设备根目录的 `/summi_ota/` 路径下
  3. 在系统中选择驱动器并开始更新


![](https://cdn.sunmi.com/public/image/mgt-document/eaa8bcba6ce445fa95e7406abd5a9240.png)
### **本地更新校验**
本地更新包同样需要经过安全校验。
![](https://cdn.sunmi.com/public/image/mgt-document/2acb19ced35f4a6d895baf98161d2b34.png)
![](https://cdn.sunmi.com/public/image/mgt-document/00404d3d858b42f29b82fe4c761e9782.png)
### **本地更新升级**
本地更新升级与OTA远程升级逻辑一致。
![](https://cdn.sunmi.com/public/image/mgt-document/a3aa4e799cbb4cdfb22c3720c8abb106.png)
图：本地更新升级中
本地更新升级成功等待用户重启系统提示。
本地更新包升级成功提示：
![](https://cdn.sunmi.com/public/image/mgt-document/4e43772a9cc64bfab0750e0f9fd7c153.png)
图：本地更新升级成功
## **6)自升级流程**
### **OTA应用自升级**
当OTA应用本身需要更新时，会优先进行自升级。
![](https://cdn.sunmi.com/public/image/mgt-document/aae8225ffdbe46db9a03b647bb8aadb5.png)
### **自升级特性**
  * **无缝升级** ：应用升级过程中界面会短暂消失。
  * **自动续传** ：自升级完成后继续系统OTA流程，前台界面则导航到更新升级界面自动升级，后台则静默执行升级。
  * **用户确认** ：需要用户点击确认开始自升级。


  

## **7)夜间升级**
### **触发条件**
夜间升级在以下条件满足时触发：
  * 更新包已下载完成
  * 开启夜间升级功能
  * 处于设定的夜间升级时间段


### **升级提醒**
系统会在升级前弹窗提醒用户。
![](https://cdn.sunmi.com/public/image/mgt-document/1057dba27fce4fbeadab7dad9c846bf9.png)
![](https://cdn.sunmi.com/public/image/mgt-document/5a7d0337047543e493f918d270fdba5d.png)
![](https://cdn.sunmi.com/public/image/mgt-document/69fffa75042b4f8dae602dcb496a7665.png)
![](https://cdn.sunmi.com/public/image/mgt-document/d7762d02bbf9497ebffa537ee35c859b.png)
### **配置管理**
夜间升级可通过多级配置：
![](https://cdn.sunmi.com/public/image/mgt-document/c32c5064240e4c609f9ca1fc1b870520.png)
图：PARTNER平台夜间升级设置
![](https://cdn.sunmi.com/public/image/mgt-document/24c226ca633e4a3383fce2aa74064cc9.png)
## **8)开机提醒流程**
### **升级成功开机提示**
系统升级成功后，下次开机会显示升级成功提示。
![](https://cdn.sunmi.com/public/image/mgt-document/131e3925ef114470b3cd5e3a9a104899.png)
### **待升级开机提示**
更新包已下载但未安装时，开机会提示立即升级。
![](https://cdn.sunmi.com/public/image/mgt-document/9d67f7e99eec447ba887471aed8d30db.png)
## **9)异常处理**
### **存储空间不足**
更新包下载前会检查存储空间。
![](https://cdn.sunmi.com/public/image/mgt-document/87003d6b83bd4c749c3132bde734fe42.png)
### **网络异常**
网络连接问题时的错误处理。
![](https://cdn.sunmi.com/public/image/mgt-document/763e1fb8bcf344cdb6315cb6047f9133.png)
  

# **3.Partner平台策略配置说明**
## **1)升级策略配置**
在PARTNER平台中配置设备升级策略。
![](https://cdn.sunmi.com/public/image/mgt-document/d4459fb1ab984070bfae8853c82d14f8.png)
  1. 自动安装提示：设备夜间自动升级重启前，默认会有倒计时弹窗提示。
  2. 升级消息提示：设备升级完成后重启，系统会有弹窗以及通知栏消息提示。
  3. 开机升级提示：下载完更新包后开机重启设备，会出现自动更新倒计时提示。
  4. 使用流量下载：当前Windows OTA不支持该项配置。


# **4.FAQ**
## **Q1：更新包下载失败怎么办？**
**A：**
  1. 检查网络连接是否稳定
  2. 确认存储空间充足（至少5GB）
  3. 尝试暂停后重新下载
  4. 检查下载限速设置是否合理


## **Q2：升级过程中设备重启失败？**
**A：**
  1. 不要手动断电，等待系统自动恢复
  2. 如长时间无响应，联系技术支持


## **Q3：签名校验失败如何解决？**
**A：**
  1. 确认更新包来源正规
  2. 尝试重新下载更新包
  3. 联系设备服务商获取帮助


## **Q4：夜间升级影响日间业务？**
**A：**
  1. 在PARTNER平台调整夜间升级时间段
  2. 关闭非必要的自动升级功能
  3. 设置合适的升级提醒时间


## **Q5：系统通知栏点击文本区域无法打开界面？**
**A：**
  1. 当前版本并不支持在通知栏点击文本区域打开程序界面
  2. 通知栏按钮点击均正常响应


  

上一篇：Windows 硬件管家软件说明
下一篇：商米云服务健康状态平台
