---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xmfreghjk568
---

# Windows 硬件管家软件说明
更新时间：2025-12-10 18:55:06
# **1.软件概述**
## **1)软件简介**
硬件管家是商米设备专用的硬件诊断与管理工具，提供全面的硬件状态监测、故障诊断和设备配置功能，帮助用户快速定位和解决硬件问题。
## **2)核心功能**
  * **双系统切换** ：支持Windows与Android系统间无缝切换
  * **设备信息监控** ：实时显示设备硬件配置和状态
  * **硬件健康检测** ：自动与手动两种检测模式
  * **外设设备管理** ：客显屏、打印机、钱箱等外设配置
  * **专业诊断报告** ：生成详细的硬件健康报告


## **3)系统要求**
  * **支持设备** ：T3 PRO MAX SUPER、T3 PRO SUPER、K2 SUPER、D3 WIN、Flex3 WIN，不同设备所支持的功能依据设备功能差异会有一定的变化
  * **系统版本** ：SUNMI WOS 1.0.1及以上版本
  * **系统架构** ：ARM、X64、X86
  * **存储空间** ：无要求
  * **网络要求** ：需要接入互联网


  

# **2.界面概览**
硬件管家采用左侧导航栏设计，包含四个主要功能模块：  
| **导航项**  | **功能描述**  |  
| --- | --- |  
| **双系统切换**  | Windows与Android系统切换  |  
| **设备信息**  | 查看详细硬件配置信息  |  
| **硬件检测**  | 执行硬件健康检查  |  
| **硬件设置**  | 外设设备配置管理  |  
主界面展示设备信息和快速检测入口：
![](https://cdn.sunmi.com/public/image/mgt-document/6449605586dd4d50835ebb42b409110f.PNG)
图：硬件管家主界面
  

# **3.功能介绍**
## **1)双系统切换**
### **功能说明**
支持在Windows和Android系统之间进行切换，满足不同业务场景需求。
### **操作流程**
1.点击左侧导航栏"双系统切换"
2.点击"切换进入安卓系统"按钮
3.确认切换操作
双系统切换主界面，显示当前系统状态：
![](https://cdn.sunmi.com/public/image/mgt-document/bc77f39d7d364070b711733f477158db.PNG)
图：双系统切换界面
4.点击“确认”按钮启动切换安卓系统流程，点击“取消”按钮则取消切换安卓系统流程
切换系统前的确认提示页面：
![](https://cdn.sunmi.com/public/image/mgt-document/35190c78c9ec4e018f2f2c8d432f0127.PNG)
图：切换确认弹窗
### **切换过程**
  * 点击确认后，系统全屏显示切换动画
  * 流程执行完毕后设备自动重启
  * 重启后进入安卓操作系统


## **2)设备信息**
### **信息概览**
展示设备的详细硬件信息和系统状态。
设备详细信息页面：
![](https://cdn.sunmi.com/public/image/mgt-document/7c97b66bc9df47358d225ca04ae47e9e.PNG)
图：设备信息页面
### **信息分类**
设备标识
  * **产品名称** ：设备在系统中的识别名称
  * **设备型号** ：硬件型号（如：T3PRO_SUPER）
  * **市场型号** ：市场销售型号（如：L15B2）
  * **SN号** ：设备唯一序列号


系统信息
  * **系统版本** ：当前操作系统版本
  * **CPU处理器** ：处理器型号和规格
  * **SMBIOS版本** ：系统基础信息版本
  * **系统固件版本** ：设备固件版本


资源状态
  * **内存使用** ：已使用内存/总内存（如：2/7GB）
  * **磁盘使用** ：已使用存储/总存储（如：33/130GB）


## **3)硬件检测**
硬件检测提供两种模式：**自动检测** 和**单项检测** 。
### **自动检测**
  * 检测概览


自动按顺序检测所有硬件组件，生成综合健康报告。
自动检测开始页面，显示检测状态；每个检测项也可以进行单独检测（点击检测卡片按钮即可），单项检测结果会修改自动检测的结果报告：
![](https://cdn.sunmi.com/public/image/mgt-document/7d70031cc12d456bbc43cba24c854773.PNG)
图：自动检测首页
  * 检测流程


检测顺序：**网络 → 触摸屏 → 屏幕 → 打印机 → 麦克风 → 喇叭 → 摄像头 → 蓝牙**
触摸屏画线检测界面，全部填满格子则检测成功，异常点击异常按钮、跳过该项检测则点击跳过即可（默认60s倒计时）：
![](https://cdn.sunmi.com/public/image/mgt-document/7581faaa7e98470187c8b8a29744504d.PNG)
图：触摸画线检测
同时5个手指触摸屏幕检测，跳过该项检测则点击跳过按钮（默认60s倒计时）：
![](https://cdn.sunmi.com/public/image/mgt-document/bd3a903bb5a94537890536c221b22f9c.PNG)
图：多指触摸检测
屏幕坏点和色彩异常检测：
![](https://cdn.sunmi.com/public/image/mgt-document/b8835561f2c84a9cb616df23d5c0929e.PNG)
图：屏幕坏点检测
摄像头清晰度检测界面：
![](https://cdn.sunmi.com/public/image/mgt-document/952dddec496b476f8c8c33d27d4ddf85.PNG)
图：摄像头检测
  * 检测结果


检测完成后显示综合报告，涵盖如下内容：
  1. 检测时间和设备信息
  2. 各项硬件检测结果（正常/异常/跳过）
  3. 设备健康评分


所有硬件检测正常的结果展示：
![](https://cdn.sunmi.com/public/image/mgt-document/3df30e368de24167863094967a2de305.PNG)
图：检测成功页面
存在硬件异常时的结果展示：
![](https://cdn.sunmi.com/public/image/mgt-document/634b99431a7143e482c2888b5944fab3.PNG)
图：检测异常页面
部分检测项被跳过时的结果：
![](https://cdn.sunmi.com/public/image/mgt-document/29a8f161987c40509fe48c8f22ae96c7.PNG)
图：检测跳过页面
  

### **单项检测-网络检测**
网络检测主界面：
![](https://cdn.sunmi.com/public/image/mgt-document/f7d39e7036b440e786e47f57e638aa25.PNG)
图：网络检测首页
检测类型：
**1.网络测试**
  * 测试上传和下载速度
  * 检查局域网连通性、互联网连通性、网络延迟


网络速度测试结果界面：
![](https://cdn.sunmi.com/public/image/mgt-document/bcc13d7569d940dfb86c69304ad03217.PNG)
图：网络速度测试
**2.信号强度测试**
  * 显示网络信号强度曲线
  * 实时监测信号稳定性


信号强度测试结果界面：
![](https://cdn.sunmi.com/public/image/mgt-document/47f58a8f65e741fabe7b1f0f9070a43e.PNG)
图：信号强度测试
**3.连通性测试**
  * 自定义节点地址和连接次数
  * 测试网络延迟和丢包率


网络连通性测试配置界面：
![](https://cdn.sunmi.com/public/image/mgt-document/70de76478a3f4827a61aa8add778b8b9.PNG)
图：连通性测试
  

### **单项检测-打印机检测**
打印机状态检测界面：
![](https://cdn.sunmi.com/public/image/mgt-document/1ab0db85fb264b8799aec3f83337956c.PNG)
图：打印机检测首页
**检测功能：**
  * **状态监测** ：自动检测缺纸、开盖状态
  * **打印测试** ：执行测试页打印
  * **信息显示** ：
    * 小票格式（如：80mm）
    * 打印浓度设置
    * 驱动版本和固件版本
    * 打印距离


  

### **单项检测-麦克风检测**
麦克风录音检测界面：
![](https://cdn.sunmi.com/public/image/mgt-document/284af788dd074806b612e9ebc5d04921.PNG)
图：麦克风录音界面
麦克风录音播放界面：
![](https://cdn.sunmi.com/public/image/mgt-document/7d70f4113c0b4a0a86e60a1d5bc7aeda.PNG)
图：麦克风播放界面
**检测流程：**
  1. **录音检测** ：根据麦克风输入音频生成实时波形图
  2. **播放验证** ：回放录音数据验证音质
  3. **质量评估** ：检测音频清晰度和噪声情况


  

### **单项检测-喇叭检测**
喇叭检测功能界面：
![](https://cdn.sunmi.com/public/image/mgt-document/84c2d3ae90494e59a720abfac41ea2cf.PNG)
图：喇叭检测界面
**检测项目：**
  * **音频播放** ：播放测试音频文件
  * **音量调节** ：测试不同音量级别的播放效果
  * **音质评估** ：检测音频失真和杂音


### **单项检测-钱箱检测**
钱箱检测和配置界面：
![](https://cdn.sunmi.com/public/image/mgt-document/b9128e9c3fde47608590ece6e61cb14b.PNG)
图：钱箱检测界面
**操作步骤：**
  1. 输入打开钱箱的特定指令（十进制）
  2. 点击"点击打开钱箱"按钮
  3. 观察钱箱是否正常打开
  4. 统计钱箱打开次数


## **4)硬件设置**
### **客显屏管理**
**连接状态**
支持单副屏和多副屏配置，实时显示连接状态。
未检测到客显屏连接时的提示：
![](https://cdn.sunmi.com/public/image/mgt-document/889fad527f564e74a9d812f0511b04a8.PNG)
图：无副屏连接
单个客显屏已连接状态：
![](https://cdn.sunmi.com/public/image/mgt-document/9e7c2083723a4833be3f52b4fc17df38.PNG)
图：单副屏连接
多个客显屏连接状态显示：
![](https://cdn.sunmi.com/public/image/mgt-document/766a97421daa4d92a6fcf8519f7df513.PNG)
图：多副屏连接
多个客显屏未连接状态：
![](https://cdn.sunmi.com/public/image/mgt-document/1b2fe2c7f71f4905b376c16b2909e514.PNG)
图：多副屏未连接
  

**客显屏设置**
客显屏设置变更后会持久化记忆在本地，客户后续连续已记忆副屏时会直接显示之前设置的数据信息并默认生效。
客显屏详细设置界面：
![](https://cdn.sunmi.com/public/image/mgt-document/d53c6145e48c4ac8b045a0b70db1b29b.PNG)
图：客显屏设置页面
配置选项：
  * 客显屏命名：自定义设备显示名称
  * 客显屏开关：启用/禁用客显屏功能，关闭后副屏不再显示（黑屏）
  * 触控功能：配置触摸屏功能开关，关闭后副屏不再响应触摸功能
  * 亮度调节：调整客显屏亮度级别


信息显示：
  * 驱动版本信息
  * 客显屏Model值
  * SN序列号
  * 分辨率大小和尺寸信息


  

**重命名功能**
客显屏重命名设置弹窗：
![](https://cdn.sunmi.com/public/image/mgt-document/3d261a8935bb4e7e9a45879dd03186fe.PNG)
图：重命名弹窗
客显屏重命名成功后的显示效果：
![](https://cdn.sunmi.com/public/image/mgt-document/4a43175f9b404cb9a4fc272a4d52c1ba.JPG)
图：重命名结果
**删除客显屏**
用户可以删除已记忆但未连接的客显屏记录，删除后连接该副屏时不再直接显示（生效）之前设置的数据信息。
客显屏删除确认弹窗的显示效果：
![](https://cdn.sunmi.com/public/image/mgt-document/78df461facfd4a12bf94123dde45a7b5.PNG)
图：删除客显屏弹窗确认
# **4.关于**
硬件管家版本信息：
![](https://cdn.sunmi.com/public/image/mgt-document/7177165990ac4d3fad84d49ba24eecb2.PNG)
图：关于弹窗
# **5.FAQ**
#### **Q1：检测过程中设备无响应怎么办？**
**A：**
  1. 等待检测超时（通常30-60秒）
  2. 强制退出硬件管家应用
  3. 重启设备后重新检测
  4. 如问题持续，联系技术支持


#### **Q2：客显屏无法被识别？**
**A：**
  1. 检查物理连接是否牢固
  2. 在硬件设置中刷新设备列表
  3. 检查驱动或固件版本是否需要更新


#### **Q3：打印机检测显示缺纸但实际有纸？**
**A：**
  1. 检查纸张安装是否正确
  2. 清理纸张传感器
  3. 重启打印机设备
  4. 执行打印机自检程序


#### **Q4：网络检测速度异常？**
**A：**
  1. 检查网络连接稳定性
  2. 确认无其他大流量应用运行
  3. 尝试不同时段进行测试
  4. 联系网络管理员检查带宽限制


上一篇：WinRE备份还原操作说明
下一篇：Windows OTA系统更新软件说明
