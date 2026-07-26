---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xdmceghjk502
---

# WinRE备份还原操作说明
更新时间：2025-12-12 16:39:48
SUNMI SuperPOS版本的System backup and restore是一个用于备份系统镜像以及还原的工具，方便在将系统进行自定义后（如：安装软件、系统设置...），用该工具将配置好的系统进行一键备份，然后批量部署到其他机器上。
  * WinRE — Windows系统恢复环境(Windows Recovery Environment)1


# **1.前提条件--打开程序：**
1)设置->系统->恢复->高级启动->立即重新启动: 选择“立即重启”，等待系统重启进入winre环境（如下图）
(或者直接重启计算机，开机时按F9也可以直接进入到winre环境)
![](https://cdn.sunmi.com/public/image/mgt-document/77e81cf62ac54f9b811ac22522cec782.png)
2)Troubleshoot->System backup and restore，即可打开程序。
# **2.镜像备份操作步骤：**
## **1)系统备份：**
选择系统备份将会捕获系统分区生成一个*.wim格式的镜像文件，请根据以下步骤进行。
第一步：选择"Backup settings"
第二步：点击"Source drive"下拉框，选择*:(Windows)系统盘（winre下会动态分配盘符，通常系统盘会分配为G盘，认准"（Windows）"标识的就是系统盘）
第三步：点击"Browse"，选择镜像文件的存储路径（注意选择除系统盘之外的磁盘作为存储路径，可用空间不少于30G，推荐NTFS格式）
第四步：点击"Start backup" 开始备份（系统备份一般在20分钟左右，预估时间仅供参考，以实际为准）
![](https://cdn.sunmi.com/public/image/mgt-document/2cf6fd24c69940a19441769a604bc545.png)
备份完成会有弹框提示，点击确认即可
![](https://cdn.sunmi.com/public/image/mgt-document/16bf0a62469945aca57c96a499f653d6.png)
最后在存储路径下会生成SystemImage*.wim镜像文件以及SystemImage*.wim.md5校验文件（注意非必要不可修改文件名以及文件内容，否则会导致还原失败）
![](https://cdn.sunmi.com/public/image/mgt-document/76babcacaa9c4d7a992a5a6ca23dbb04.png)
## **2)全盘备份（除共享盘）：**
选择全盘备份将会捕获整个磁盘（包括EFI分区、恢复分区、系统分区、数据分区）生成一个*.ffu格式的镜像文件，请根据以下步骤进行。
第一步：选择"Backup settings"
第二步：点击"Source drive"下拉框，选择All disks
第三步：点击"Browse"，选择镜像文件的存储路径（注意选择除本地磁盘之外的磁盘作为存储路径，可用空间不少于30G，推荐NTFS格式）
第四步：点击"Start backup" 开始备份（全盘备份一般在30分钟左右，预估时间仅供参考，以实际为准）
![](https://cdn.sunmi.com/public/image/mgt-document/6dc16bd8188844b58e1a3c1493e21d63.png)
备份完成会有弹框提示，点击确认即可
最后在存储路径下会生成SystemImage*.ffu镜像文件以及SystemImage*.ffu.md5校验文件（注意非必要不可修改文件名以及文件内容，否则会导致还原失败）
# **3.镜像还原操作步骤：**
## **1)系统还原：**
第一步：选择"Restore settings"
第二步：点击"Browse"选择用于还原的（SystemImage*.wim）系统镜像文件（注意：该文件不可存放在本机系统分区的任何目录中，否则无法还原）
![](https://cdn.sunmi.com/public/image/mgt-document/d2c9cd7bc22645aca2310959ad6b0ff7.png)
还原完成会有弹框提示，点击确认即可
![](https://cdn.sunmi.com/public/image/mgt-document/8fcca36875e047948c10fc065231c38c.png)
## **2)全盘还原（除共享盘）：**
第一步：选择"Restore settings"
第二步：点击"Browse"选择用于还原的（SystemImage*.ffu）系统镜像文件（注意：该文件必须存放在可移动磁盘，不可存放在本地磁盘的任何目录中，否则无法还原）
第三步：点击"Start Restore"后会有弹框提示，该操作将会格式化系统盘，确认后开始还原
![](https://cdn.sunmi.com/public/image/mgt-document/9c0964b204384932b97e1bd795b3f2c4.png)
还原完成会有弹框提示，点击确认即可
# **4.详细功能介绍：**
## **1)Storage information: 驱动器信息界面**
**驱动器列表：**
  1. Drive: 显示驱动器盘符以及名称。（需要注意的是winre环境下分配盘符是动态临时分配的机制，和正常OS分配盘符的方式不一样，所以系统盘通常会被映射为G盘或者其他盘符，D盘通常被映射为H盘或者其他盘符，A盘也可能会映射为其他盘符，选择驱动器时可通过（）里的名称判断是否为Windows，Windows则是系统盘也就是OS下的C盘）
  2. Media type: 显示驱动器类型（本地磁盘、可移动磁盘等）
  3. Size: 驱动器空间大小
  4. Available: 驱动器可用空间大小


**Format:** 格式化磁盘（慎用该功能，格式化会清除掉该磁盘的所有数据）
**Eject:** 弹出磁盘，仅用于可移动磁盘的弹出
![](https://cdn.sunmi.com/public/image/mgt-document/1ff292d65f24425fbe5ce6139e544c7d.png)
## **2)Backup setting: 备份设置界面**
**Source drive:** 选择需要备份的源驱动器，这里有两种选项：
  1. C:(Windows))：系统盘备份（在winre下通常为G盘或者其他盘符）
  2. All disks: 全盘备份（不包含共享盘）


**Target file type:** 指备份生成的镜像文件的类型，这里是根据Soure drive的选项自动识别，不需要手动选择
  1. .wim: 对应系统盘备份
  2. .ffu：对应全盘备份


**Target file:** 存储备份的镜像文件目录（仅支持英文目录）
**Browse:** 选择镜像文件的存储目录，（注意：系统盘备份时，不可选择系统磁盘下的目录；全盘备份时，只能选择除本地磁盘以外的磁盘下的目录）
**Backed up:** 备份进度实时显示
**Remaining time:** 剩余时间实时显示（只作为参考，是根据已经备份完成的进度动态计算剩余时间）
**Start backup:** 点击开始备份，备份结束后会生成镜像文件以及同名的MD5文件（未选择存储目录时，该按钮置灰不可点击）
**Cancel:** 点击取消备份，点击确认之后会等待线程安全退出（预计等待5秒）
![](https://cdn.sunmi.com/public/image/mgt-document/9f7a5a8386874c32ba0f5f7fecb1d639.png)
## **3)Restore setting: 还原设置界面**
**Source file:** 存储用来还原的镜像文件路径（仅支持英文目录）
**Browse:** 选择用来还原的镜像文件，若选择*.wim格式的镜像文件，则程序自动判定为系统盘还原；若选择*.ffu格式的镜像文件，则程序自动判定为全盘还原。（注意：*.wim格式的镜像文件不可存放在系统盘目录；*.ffu格式的镜像文件不可存放在本地磁盘目录）
**Source file type:** 指用来还原的镜像文件的类型，这里是根据Soure file的选项自动识别，不需要手动选择
  1. .wim: 对应系统盘还原
  2. .ffu：对应全盘还原


**Target drive:** 指需要还原的目标驱动器，这里有两种选项：
  1. C:(Windows)：系统盘还原（在winre下通常为G盘或者其他盘符），正式开始还原后系统盘将会被格式化
  2. All disks: 全盘还原，正式开始还原后全盘将会被格式化，（不包含共享盘）


**Restored:** 还原进度实时显示
**Remaining time:** 剩余时间实时显示（只作为参考，是根据已经备份完成的进度动态计算剩余时间）
**Start Restore:** 点击开始还原，会先校验镜像文件的MD5值，再格式化磁盘（1.未选择存储目录时，该按钮置灰不可点击；2. 镜像文件名与对应的MD5文件名不建议修改，如果一定要修改，两个文件需要同步修改为一致的命名，并且*.wim，*.wim.md5只能修改*的部分）
**Cancel:** 点击取消还原，点击确认之后会等待线程安全退出（预计等待5秒）
![](https://cdn.sunmi.com/public/image/mgt-document/73fb994e4dcf4ae9b91a527c45e1f47a.png)
## **4)Log record: 日志信息界面**
**日志信息列表**
  1. NO.: 日志文件序号
  2. Log file: 所有日志文件
  3. File size: 日志文件大小
  4. Creation time: 日志文件创建时间
  5. Update time: 日志文件最近更新的时间


**Export all logs:** 导出所有日志文件到指定的目录
**Select all:** 全选或者全不选
**Open file:** 打开所选择的一个或多个日志文件
**Delete record:** 删除所选择的一个或多个日志文件
![](https://cdn.sunmi.com/public/image/mgt-document/be5064aac2374f10b3ca1289f030c7e1.png)
## **5)Exit: 退出程序按钮**
# **5.常见问题：**
### **1)弹框报错：Backup file storage does not support FAT32 file system, please format to NTFS file system and try again**
A:该报错是检查到所选目录的文件系统格式为FAT32格式，备份文件不支持存储在FAT32格式的磁盘上，建议选择NTFS格式的磁盘目录
### **2)弹框报错：Insufficient space in the target storage location. Please free up space before tapping "Start backup."**
A:该报错是备份操作时，所选目录的可用空间不足30G，需先清理空间再进行备份
### **3)弹框报错：Insufficient space in the target storage location. Please free up space before tapping "Start restore"**
A:该报错是还原操作时，检测到本机系统分区总容量不符合70G标准，分区大小被调整过，仅做系统备份还原可能导致无法进系统，建议使用全盘备份还原。
上一篇：常见故障排查手册
下一篇：Windows 硬件管家软件说明
