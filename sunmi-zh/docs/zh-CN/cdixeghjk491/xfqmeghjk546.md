---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfqmeghjk546
---

# 8、NFC发卡工具使用指南
更新时间：2026-01-23 16:59:30
# **下载NFC发卡工具**
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) SMNFCManager_v3.0_20260123.zip 
# **设备连接**
**1.连接读卡器** ：请使用一台运行Windows 11操作系统的电脑。将“D8读写器”插入电脑的USB接口，系统将自动识别并安装驱动程序，稍等片刻即可使用。
![](https://cdn.sunmi.com/public/image/mgt-document/2b01d75090084119b3f4964b2863817d.jpg)
**2.确认安装成功** ：设备连接后，请打开“设备管理器”，并定位至“人体学输入设备”分类。若在此分类下能看到“USB输入设备”项，则表明驱动程序已成功安装。
![](https://cdn.sunmi.com/public/image/mgt-document/9d6ad8ae38054adc8e1b3c57536a19cd.png)
# **启动程序**
**1.解压与启动** ：将获取到的“[SMNFCManager.zip](http://SMNFCManager.zip)”压缩包解压至本地磁盘的任意目录。然后，双击运行解压后的“SMNFCManager.exe”可执行文件以启动程序。
![](https://cdn.sunmi.com/public/image/mgt-document/60ac47e410fc4c219ebe5d6ec1971fde.png)
**2.程序界面** ：程序启动后，将显示主操作界面。
![](https://cdn.sunmi.com/public/image/mgt-document/bc64ddea4f5640ef950de835d13ff504.png)
# **空白卡片来料检验**
**1.应用场景** ：当仓库收到空白NFC卡片时，需抽样检验其规格是否符合要求、功能是否正常，确保卡片可用。此环节需使用程序的“Check Empty Card”功能。
**2.操作步骤** ：在程序界面中切换到“Check Empty Card”标签页。将待检卡片放置于“D8读卡器”上，然后点击“Check Card”按钮。
![](https://cdn.sunmi.com/public/image/mgt-document/0a8bd8f997d344149c509f7e926b9b75.jpg)
**3.结果解读** ：检测过程中，“SN”、“Card Type”和“Card status”字段会显示实时数据。最终检测结果将以“Check Status”显示：
  * **Pass** ：表示卡片校验通过，功能正常，可以投入使用。
  * **Fail** ：表示卡片校验失败，此卡片不可用，需剔除。


![](https://cdn.sunmi.com/public/image/mgt-document/214ca697821b4e78a37299241f5bb220.png)
# **批量制卡：准备数据表格**
**1.文件准备** ：打开已有的“NFC_Card.xlsx”文件或新建一个Excel文件作为数据源。
**2.字段说明** ：请确保表格包含以下列（字段）：
  * **Shop** ：此条记录所属的店铺名称。
  * **URL** ：需要写入NFC卡片的目标网址。
  * **PackageName** ：需要写入NFC卡片的安卓应用程序包名。
  * **Password** ：为卡片数据设置写保护密码。
  * **CardSerialNo** 与 **State** ：此两列请保留空白，程序将自动填充卡序列号和写卡结果。
![](https://cdn.sunmi.com/public/image/mgt-document/ae9c43d81c214332bdfd68d423f66376.png)
# **批量制卡：写入操作**


**1.导入数据** ：在程序界面中切换到“Batch Card”标签页，点击“Import EXCEL”按钮，选择并导入准备好的Excel文件。
**2.导入逻辑** ：程序默认会导入所有“State”为空（新卡）或为失败状态的记录。已标记为成功的记录将不会被重复导入。
**3.开始写卡** ：可选择列表中的任意一条记录（通常用于从指定店铺或断点处）开始，点击“Start”按钮即可启动批量写卡流程。
![](https://cdn.sunmi.com/public/image/mgt-document/966d439880a14b84b4b81167a5d7e9b0.png)
**4.流程提示** ：
  * 点击“Start”后，按钮将变为“Stop”。
  * 将一张**空白卡片** 放置于读卡器上，若听到一声“beep”提示音，表示当前卡片写入成功。请及时移走该卡，并更换下一张空白卡片继续写入。
  * 若听到连续的三声“beep”提示音，表示本条记录写入失败（失败原因可能来自卡没有放置正确、写入内容有异常，不一定是卡片本身问题）。程序将自动跳过当前记录，并继续下一条记录的写入。
  * 若同一张卡片多次写入失败，但Excel中的记录数据经检查无误，则很可能是该张卡片存在异常。请更换一张新的空白卡片再次尝试写入该条记录。
  * 所有卡片处理完毕后，“Stop”按钮将恢复为“Start”状态，并弹出提示框显示成功与失败的数量统计。
![](https://cdn.sunmi.com/public/image/mgt-document/d031494f1e3749ccb7f744bdb276d064.png)


**5.写保护说明** ：批量制作的卡片因已设置写保护密码，无法再次通过“Batch Card”功能重复写入。如需修改，请使用下述的“Single Card”功能。
**6.结果导出** ：批量写卡完成后，可再次打开之前导入的Excel文件。程序会将每张卡片的序列号(CardSerialNo)和写卡状态(State)回填至表格中。“State”标记为“SUCCESS”表示制卡成功，其他状态则需排查原因并重新制作。
![](https://cdn.sunmi.com/public/image/mgt-document/210d6b65813643908d719ba64293af91.png)
# **单卡制作与修改**
**1.功能定位** ：在程序界面中切换到“Single Card”标签页。此功能主要用于对单张卡片进行二次写入或修改。
**2.数据填写** ：可通过点击“Read NFC”读取卡片现有信息，或直接手动输入/粘贴需要写入的“URL”和“Package Name”。
**3.重新写卡** ：如果是修改已有数据的卡片，必须输入正确的“Old Password”，然后点击“Write NFC”按钮方可重新写入。
![](https://cdn.sunmi.com/public/image/mgt-document/34992efd9397413d97f33d9502278b00.png)
**4.修改密码** ：如需更改卡片的写保护密码，请准确输入“Old Password”和“New Password”，然后点击“Change Password”按钮。密码修改成功后，卡片将立即启用新密码进行写保护。修改密码功能只能对已经写过密码的卡进行修改，不能对空白卡进行修改，如需要空白卡写卡，请用“Batch Card”功能。
# **通过网页执行NFC卡操作**
**1.解压与启动** ：将获取的“SMNFCManager.zip”压缩包解压到本地磁盘的任意目录，然后双击运行“install.bat”文件以启动安装程序。
![](https://cdn.sunmi.com/public/image/mgt-document/2fd52f9656964ce4b3183b635c31fbaf.png)
**2.程序界面** ：在浏览器中打开测试演示文件“..\WEB DEMO\SMNFCSoundBoxTest.html”，即可进入主操作界面。
![](https://cdn.sunmi.com/public/image/mgt-document/f39e3eb918584ba7b491612f415de425.png)
![](https://cdn.sunmi.com/public/image/mgt-document/b5ffe62d4c47464cbaad147337a9016a.png)
![](https://cdn.sunmi.com/public/image/mgt-document/fb05d63c0ded478280808dc811f45fc2.png)
**3.选择卡操作**
在界面中选择需要执行的卡片操作，具体选项如下：
  * **Write Card** ：写入空白卡片。
  * **Modify Card** ：修改卡片数据，执行此操作需输入旧密码和新密码。
  * **Read Card** ：读取卡片数据。
  * **URL** ：指定要写入NFC卡片的目标网址。
  * **PackageName** ：指定要写入NFC卡片的安卓应用程序包名。
  * **Password** ：为卡片数据设置写保护密码（仅适用于未受保护的Sunmi空白NFC卡）。
  * **Old Password** ：修改卡片数据时，需输入原密码以验证权限。
  * **New Password** ：为卡片数据设置新的保护密码。
  * **Server URL** ：读取卡信息回传HTTP服务器地址URL。
  * **Refresh Data** ：更新读取到的卡信息。


**4.调试信息**
可通过“Debug Information”查看写卡过程中的详细日志，便于跟踪操作状态与排查问题。
# **注意事项**
**1.容量限制** ：单张NFC卡片所能存储的数据总量（"URL" + "PackageName"）不得超过**465个字符** ，超出限制会导致写入失败。
**2.声音提示** ：
  * **一声“Beep”** ：表示操作成功（如读卡、写卡、密码修改成功）。
  * **三声“Beep”** ：表示操作过程中出现错误，请根据界面提示信息排查问题。


  

上一篇：7、收款音箱DEMO for IOS
下一篇：9、收款音箱FAQ
