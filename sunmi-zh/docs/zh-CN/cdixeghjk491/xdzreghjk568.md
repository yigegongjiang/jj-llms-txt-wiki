---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdzreghjk568
---

# 打印机查询接口
更新时间：2025-09-24 16:49:58
# 一、功能介绍
* * *
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public QueryApi queryApi()
  

```

打印机查询类方法集合，通过这些方法可以获取打印机的基础信息和实时状态
> Status getStatus()
> String getInfo(PrinterInfo info)
# 二、接口说明
## 1. 获取打印机实时状态
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Status getStatus()
  

```

打印机状态分为两类，其中一类以ERR_为前缀的打印机错误状态表示打印机是否可打印的状态；
另一类以WARN_为前缀的警告状态并不影响打印机打印，只是做提示处理，开发者可以根据业务需要决定是否使用；
获取状态接口是一个耗时的操作，不建议频繁调用，最好在子线程中使用
### a. 打印机错误状态说明  
| 返回值  | 返回值说明  |  
| --- | --- |  
| Status.READY  | 打印机已准备好，可正常打印  |  
| Status.OFFLINE  | 打印机离线/故障  |  
| Status.COMM  | 打印机通信异常  |  
| Status.ERR_PAPER_OUT  | 打印机缺纸中  |  
| Status.ERR_PAPER_JAM  | 打印机堵纸中  |  
| Status.ERR_PAPER_MISMATCH  | 当前打印纸不匹配打印机  |  
| Status.ERR_PRINTER_HOT  | 打印头过热  |  
| Status.ERR_MOTOR_HOT  | 马达过热  |  
| Status.ERR_COVER  | 打印机未关闭纸舱盖  |  
| Status.ERR_COVER_INCOMPLETE  | 打印机未完全关闭纸舱盖  |  
| Status.ERR_CUTTER  | 打印机切刀错误（需要切刀支持）  |  
| Status.ERR_CARTRIDGE_LOSS  | 墨盒未安装（需要激光打印机支持）  |  
| Status.ERR_CARTRIDGE_MISMATCH  | 墨盒不匹配（需要激光打印机支持）  |  
| Status.ERR_CARTRIDGE_EMPTY  | 墨盒已用完（需要激光打印机支持）  |  
| Status.ERR_DUPLEX_LOSS  | 双面单元未安装（需要激光打印机支持）  |  
| Status.ERR_CARTON_LOSS  | 纸盒未安装（需要激光打印机支持）  |  
| Status.ERR_CARTON_MISMATCH  | 纸盒不匹配（需要激光打印机支持）  |  
| Status.ERR_CARTON_EMPTY  | 纸盒缺纸（需要激光打印机支持）  |  
| Status.ERR_DRUM_LOSS  | 鼓组件未安装（需要激光打印机支持）  |  
| Status.ERR_DRUM_MISMATCH  | 鼓组件不匹配（需要激光打印机支持）  |  
| Status.ERR_DRUM_EMPTY  | 鼓组件已用完（需要激光打印机支持）  |  
| Status.ERR_STEP  | 字车故障（需要激光打印机支持）  |  
### b. 打印机警告状态  
| 返回值  | 返回值说明  |  
| --- | --- |  
| Status.WARN_THERMAL_PAPER  | 热敏打印纸纸将尽  |  
| Status.WARN_STANDARD_PAPER  | 标准打印纸纸盒即将用尽（需要激光打印机支持）  |  
| Status.WARN_SPECIAL_PAPER  | 多功能纸盒即将用尽（需要激光打印机支持）  |  
| Status.WARN_CARTRIDGE  | 墨盒即将用尽（需要激光打印机支持）  |  
| Status.WARN_PICK_PAPER  | 当前打印纸待取走（需要具有取值传感器的设备支持）  |  
部分打印机状态需要对应打印机支持请根据使用的打印机以及如上状态说明酌情处理
### c. 打印机状态广播
除了通过接口获取状态外，还支持部分状态通过自定义广播获取，可捕获的广播如下：  
| 广播Action  | 广播说明  |  
| --- | --- |  
| woyou.aidlservice.jiuv5.NORMAL_ACTION  | 打印机当前状态正常可以打印  |  
| woyou.aidlservice.jiuv5.OUT_OF_PAPER_ACTION  | 打印机当前缺纸中  |  
| woyou.aidlservice.jiuv5.PAPER_ERROR_ACITON  | 打印机当前纸舱口堵纸  |  
| woyou.aidlservice.jiuv5.OVER_HEATING_ACITON  | 打印机当前打印头过热中  |  
| woyou.aidlservice.jiuv5.MOTOR_HEATING_ACITON  | 打印机当前马达过热中  |  
| woyou.aidlservice.jiuv5.COVER_OPEN_ACTION  | 打印机当前舱盖被打开  |  
| woyou.aidlservice.jiuv5.COVER_ERROR_ACTION  | 打印机当前舱盖未完全关闭  |  
| woyou.aidlservice.jiuv5.KNIFE_ERROR_ACTION_1  | 打印机切刀异常  |  
| woyou.aidlservice.jiuv5.KNIFE_ERROR_ACTION_2  | 打印机的切刀已经修复  |  
| woyou.aidlservice.jiuv5.BLACKLABEL_NON_EXISTENT_ACITON  | 未检测到黑标纸  |  
| woyou.aidlservice.jiuv5.LABEL_NON_EXISTENT_ACITON  | 未检测到标签纸  |  
| woyou.aidlservice.jiuv5.ERROR_ACTION  | 打印机当前出现未知异常  |  
| woyou.aidlservice.jiuv5.PICK_PAPER_ACTION  | 打印机纸舱口有纸待取走  |  
| woyou.aidlservice.jiuv5.LESS_OF_PAPER_ACTION  | 打印机打印纸即将用完  |  
| woyou.aidlservice.jiuv5.PRINTER_NON_EXISTENT_ACITON  | 未检测到打印机  |  
打印机纸待取走、纸即将用完不影响打印机工作，所以只做打印业务时可以作为参考
未检测到打印机广播一般情况下仅在开机阶段发出
## 2. 获取打印机信息
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
String getInfo(PrinterInfo info)
  

```

**PrinterInfo**  
| 查询信息  | 信息说明  | 返回值说明  |  
| --- | --- | --- |  
| PrinterInfo.ID  | 打印机硬件版本号  | 根据具体型号打印机返回值略有不同  |  
| PrinterInfo.NAME  | 打印机名称  | 返回固定的打印机名称或自定义打印机名称  |  
| PrinterInfo.VERSION  | 打印机固件版本号  | 自定义打印机不支持查询版本号  |  
| PrinterInfo.DISTANCE  | 打印机已经打印的距离  | 单位mm，自定义打印机不支持查询  |  
| PrinterInfo.CUTTER  | 打印机使用切刀次数  | 只有支持切刀的打印机将记录切刀次数  |  
| PrinterInfo.HOT  | 打印机发生过热次数  | 自定义打印机不支持查询  |  
| PrinterInfo.DENSITY  | 打印机浓度  | 浓度范围70%~130% 单位百分制，自定义打印机不支持查询  |  
| PrinterInfo.TYPE  | 打印机类型  | 返回内容见说明  |  
| PrinterInfo.PAPER  | 打印机当前支持的纸张可打印区域  | 根据当前打印机设置返回打印机的可打印区域   
  
部分打印机可能因配置无法获取将返回空  |  
记录中统计的打印距离、切刀次数和过热次数如果恢复设备出厂设置将重新统计
**PrinterInfo.TYPE**  
| 类型参数  | 类型说明  |  
| --- | --- |  
| thermal printer（默认）  | 普通热敏打印机  |  
| black mark printer  | 黑标热敏打印机  |  
| label printer  | 标签热敏打印机  |  
| needle printer  | 针式打印机  |  
| laser printer  | 激光打印机  |  
商米V2pro、V2s系列打印机支持普通热敏和标签热敏打印机的模式切换 商米台式打印机支持普通热敏和黑标热敏打印机的模式切换
上一篇：打印文件接口
下一篇：指令集打印接口
