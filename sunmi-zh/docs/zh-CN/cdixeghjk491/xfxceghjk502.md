---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfxceghjk502
---

# Flutter 打印机查询接口
更新时间：2025-09-24 18:37:31
### **1.功能介绍**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
QueryApi get queryApi => QueryApiImpl.instance;
```

打印机查询类方法集合，通过这些方法可以获取打印机的基础信息和实时状态
> Future<Status> getStatus();
> Future<String?> getInfo(PrinterInfo info);
### **2.接口说明**
#####  **2.1****获取打印机实时状态**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<Status> getStatus();
```

打印机状态分为两类，其中一类以ERR_为前缀的打印机错误状态表示打印机是否可打印的状态；
另一类以WARN_为前缀的警告状态并不影响打印机打印，只是做提示处理，开发者可以根据业务需要决定是否使用；
获取状态接口是一个耗时的操作，不建议频繁调用，最好在子线程中使用
###### **a. 打印机错误状态说明**  
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
###### **b. 打印机警告状态**  
| 返回值  | 返回值说明  |  
| --- | --- |  
| Status.WARN_THERMAL_PAPER  | 热敏打印纸纸将尽  |  
| Status.WARN_STANDARD_PAPER  | 标准打印纸纸盒即将用尽（需要激光打印机支持）  |  
| Status.WARN_SPECIAL_PAPER  | 多功能纸盒即将用尽（需要激光打印机支持）  |  
| Status.WARN_CARTRIDGE  | 墨盒即将用尽（需要激光打印机支持）  |  
| Status.WARN_PICK_PAPER  | 当前打印纸待取走（需要具有取值传感器的设备支持）  |  
部分打印机状态需要对应打印机支持请根据使用的打印机以及如上状态说明酌情处理
##### **2.2 获取打印机信息**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Future<String?> getInfo(PrinterInfo info);
```

###### **PrinterInfo**  
| 查询信息  | 信息说明  | 返回值说明  |  
| --- | --- | --- |  
| [PrinterInfo.ID](http://PrinterInfo.ID)  | 打印机硬件版本号  | 根据具体型号打印机返回值略有不同  |  
| [PrinterInfo.NAME](http://PrinterInfo.NAME)  | 打印机名称  | 返回固定的打印机名称或自定义打印机名称  |  
| PrinterInfo.VERSION  | 打印机固件版本号  | 自定义打印机不支持查询版本号  |  
| PrinterInfo.DISTANCE  | 打印机已经打印的距离  | 单位mm，自定义打印机不支持查询  |  
| PrinterInfo.CUTTER  | 打印机使用切刀次数  | 只有支持切刀的打印机将记录切刀次数  |  
| [PrinterInfo.HOT](http://PrinterInfo.HOT)  | 打印机发生过热次数  | 自定义打印机不支持查询  |  
| PrinterInfo.DENSITY  | 打印机浓度  | 浓度范围70%~130% 单位百分制，自定义打印机不支持查询  |  
| PrinterInfo.TYPE  | 打印机类型  | 返回内容见说明  |  
| PrinterInfo.PAPER  | 打印机当前支持的纸张可打印区域  |  根据当前打印机设置返回打印机的可打印区域  部分打印机可能因配置无法获取将返回空  |  
记录中统计的打印距离、切刀次数和过热次数如果恢复设备出厂设置将重新统计
###### **PrinterInfo.TYPE**  
| 类型参数  | 类型说明  |  
| --- | --- |  
| thermal printer（默认）  | 普通热敏打印机  |  
| black mark printer  | 黑标热敏打印机  |  
| label printer  | 标签热敏打印机  |  
| needle printer  | 针式打印机  |  
| laser printer  | 激光打印机  |  
商米V2pro、V2s系列打印机支持普通热敏和标签热敏打印机的模式切换 商米台式打印机支持普通热敏和黑标热敏打印机的模式切换
上一篇：Flutter 指令集打印接口
下一篇：Flutter 钱箱控制接口
