---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmaxeghjk491
---

# 状态灯服务说明
更新时间：2025-09-29 15:46:37
### **一、功能概述**
商米提供对FLEX 3状态灯进行控制的状态灯服务（StatusLightService），支持开发者三方应用对其进行调用，其功能包含以下部分：  
| 功能模块  | 功能说明  |  
| --- | --- |  
| 设置状态灯行为  | 状态灯状态：常亮、熄灭、闪烁  |  
| 设置状态灯颜色  | 红色、绿色、蓝色、黄色、青色、品红、白色(支持R/G/B/RG/GB/RB/RGB七种组合颜色）  |  
### **二、资源下载：**
状态灯SDK：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) sunmiperipher_sdk_v1.0.2.aar 
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) app-debug (1).apk 
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) statuslightdemo (1).zip 
  1. **Android Studio项目工程集成SDK**
    1. **Android Studio的libs中导入** sunmiperipher_sdk_v1.0.0.aar
    2. app的build.gradle文件中 implementation fileTree(dir: 'libs', include: ['_.aar', '_.jar'])
  2. **SDK初始化**
    1. **在Activity onCreate生命周期中初始化，并且打开设备**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
StatusLightManager.init(this) { success ->
    if(success) StatusLightManager.openDevice()
}
```

  

  3. **打开不同颜色的灯**
    1. fun setColor(color: com.sunmi.peripheralsdk.Color)
    2. 参数说明：color为灯的颜色，[值的类型有Color.Red](http://%E5%80%BC%E7%9A%84%E7%B1%BB%E5%9E%8B%E6%9C%89Color.Red)、[Color.Green](http://Color.Green)、[Color.Blue](http://Color.Blue)、Color.Yellow、Color.Magenta、Color.Cyan、Color.White。 分别代表 红色、绿色、蓝色、黄色、品红、青色、白色 七种颜色
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 //依次代表： 红色、绿色、蓝色、黄色、品红、青色、白色
 StatusLightManager.setColor(Color.Red)
 StatusLightManager.setColor(Color.Green)
 StatusLightManager.setColor(Color.Blue)
 StatusLightManager.setColor(Color.Yellow)
 StatusLightManager.setColor(Color.Magenta)
 StatusLightManager.setColor(Color.Cyan)
 StatusLightManager.setColor(Color.White)
```

  4. **关闭led灯**
    1. fun turnOff()
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
StatusLightManager.turnOff()
```

  

  5. **~~Led灯周期性闪烁（单色灯）~~ _~~（~~_** _硬件上不支持，不建议调用该接口。 应用退出时，如果没有主动调用关闭灯，灯闪烁效果经过一段时间后会自动停止）_
    1. fun setFlashing(color: com.sunmi.peripheralsdk.Color, onMS: Int, offMS: Int)
    2. 参数说明：color为灯的颜色，onMS为灯亮的时间，offMS为灯灭的时间
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
StatusLightManager.setFlashing(
      Color.Red,   // 红色
      500,          // 亮 500ms
      500           // 灭 500ms
  );
```

  

  6. **~~Led灯周期性闪烁（多色灯）（~~** _硬件上不支持，不建议调用该接口。 应用退出时，如果没有主动调用关闭灯，灯闪烁效果经过一段时间后会自动停止）_
    1. fun setMultiFlashing(colors: Array<com.sunmi.peripheralsdk.Color>, onMS: IntArray, offMS: IntArray)
    2. 参数说明：colors为颜色数组，按顺序闪烁的颜色值， onMS为每个颜色对应的亮起时间（毫秒）数组，长度需与colors一致， offMS为每个颜色对应的熄灭时间（毫秒）数组，长度需与colors一致
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
  StatusLightManager.setMultiFlashing(
      arrayOf(Color.Red, Color.Green, Color.Blue), // 红色 绿色 蓝色
      intArrayOf(500, 500, 500),// 亮 500ms
      intArrayOf(500, 500, 500)// 灭 500ms
  )
```

  

  7. **SDK反注册**
    1. **在Activity onDestory生命周期中释放资源**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
StatusLightManager.destroy(this)
```

  

  1. **可以参考上面的statuslightdemo 运行调试**


上一篇：8、钱箱驱动器APP使用说明
下一篇：CPad 内置LED指示灯管理
