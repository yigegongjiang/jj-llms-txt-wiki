---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfqreghjk568
---

# 电子秤 SDK 开发
更新时间：2025-09-25 17:02:07
## 一.电子秤参数  
| 型号:  | S2, S2 CC, S2L CC  |  
| --- | --- |  
| 最大量程  | 6kg≤Max≤30kg  |  
| n(等级)  | 3000 (III)  |  
| 检定分度  | 1/2g,2/5g,5/10g  |  
| 工作温度  | -10~+40℃  |  
## 二.计量量相关基础知识
### 1. 零点
每次智能电子秤重新上电的时候，电子秤都会自动记录初始零点作为后续称重的参考。开机重启的零点范围通常是满量程的10%。当重量超过满量程的10%的时候，电子秤将无法找到零点位置。例如，对15kg的电子秤而言，如果开机的时候秤盘的重量超过1.5kg的时候，电子秤将无法找到零点位置。如果重量小于1.5kg的时候，电子秤将默认从零点开始称量。
我们推荐您在上电开机的前清空秤盘上所有的物品。
### 2. 手动清零/零点设置
在日常称重时，倘若需要回到零点时，可以通过手动清零回到初始零点位置。手动清零的范围是满量程的2%。对15kg的电子秤而言，手动清零的范围是300g。例如，在您清洁完秤盘后，可能称重显示为-0.004kg。当您点击清零按钮后，称重显示即回归“0.000kg”。
注意：在有皮重的情况下，清零按钮将无效。
### 3. 去皮
在称重过程中，如果需要将商品包装的重量去除的时候，可以通过去皮操作执行。 去皮的重量将会影响到称量的范围。例如，如果一台15kg的电子秤设置了5kg的皮重之后，那么剩余的称重范围将仅剩余10kg。
称重去皮：先把包装放在秤盘上，点击去皮按钮后，包装的重量将自动记录为皮重；
预置皮重 如果您已知包装重量的时候，可以手动输入皮重值；
对多量程的电子秤而言，最大去皮重量通常是Max1-e1。例如，6/15kg的电子秤，最大皮重是-5.998kg。
注意：预置皮重的值必须准确设置。首先，预置皮重的值必须小于最大皮重；其次，预置皮重值必须按照正确的分度值设置。例如，对一台6/15kg（n=3000）的电子秤，6.005kg或者是0.019kg的预置皮重值都是不对。
### 4. 净重
净重是指消费者应该支付商品的重量值。如果商品带包装，需要先将包装作为皮重去掉。
### 5. 毛重
毛重=皮重+净重
## **三.电⼦秤软件开发**
认证组件共由4个板块组成，分别为：
  

![](https://cdn.sunmi.com/public/image/mgt-document/c7276dceba1f4634829d5d36fcf8ab69.png)
### 1. 基础信息
  * 计量单位信息：用于展示秤AD的基础信息。

  
| 显示内容  | 显示数值  |  
| --- | --- |  
| Max=XX/XXkg  | 6/15kg、3/6kg、15/30kg、15kg、30kg、6kg  |  
| Min=XXg  | 40g、20g、100g、100g、200g、40g  |  
| e=X/Xg  | 2/5g、1/2g、5/10g、5g、10g、2g  |  
| T=-XX.XXXkg  | -5.998kg、-2.999kg、-14.995kg、-14.995kg、-29.990kg、-5.998kg  |  
  * 认证组件基础信息：用于展示秤相关服务版本信息。

  
| 显示内容  | 显示说明  |  
| --- | --- |  
| ADFW ：XXX  | 显示称重AD版固件的版本  |  
| DISPLAY Version：XXX  | 显示当前秤应用的版本号  |  
| SERVICE ：XXX  | 显示当前秤服务Scale Service的版本号  |  
| CRC status  | 显示AD秤重上的数据与上位机的数据进行校验结果匹配一致：显示绿色锁图标匹配不一致：显示未锁定图标  |  
### 2.称重状态
  * 显示在称重过程的事件状态。

  
| 事件  | 显示说明  |  
| --- | --- |  
| 稳定  | 当称重台重量稳定时，进行稳定状态提示。稳定时，状态灯显示颜色为绿色未稳定时，状态灯显示颜色为灰色  |  
| 零位  | 当净重和皮重之和为0的时候（「net+tare=0」），进行零位状态提示。零位时，状态灯显示颜色为绿色未零位时，状态灯显示颜色为灰色  |  
| 净重  | 当去皮操作后，皮重不为0，进行净重状态提示。进行去皮时，状态灯显示颜色为绿色未进行去皮时，状态灯显示颜色为灰色  |  
### 3.称重数值
  * 显示在称重过程的物品重量信息。

  
| 称重区域  | 显示说明  |  
| --- | --- |  
| 皮重（kg）  | 当有去皮操作时，对应的皮重信息数据显示在该数据栏中。皮重的重量数值信息，无物体时，数值为0.000。在实际的物理显示屏中的显示高度，不低于9.5mm。  |  
| 净重（kg）  | 显示为实际的净重重量。当净重重量大于过载门限(皮重+净重+9*e2)时，应当显示过载提示。当净重重量低于欠载门限(皮重+净重< -20e),应当显示欠载提示。净重的重量数值信息，无物体时，数值为0.000。净重的重量数字信息，在实际的物理显示屏中的显示高度，不低于9.5mm。  |  
| 单价（元/kg）  | 显示为当前选中商品的单价信息。价格的数值信息，无物体时，数值为0.00。单价的数字信息，在实际的物理显示屏中的显示高度，不低于9.5mm。  |  
| 总价（元）  | 显示为当前选中商品的总价信息，重量*单价。总价的数值信息，无物体时，数值为0.00。总价的数字信息，在实际的物理显示屏中的显示高度，不低于9.5mm。  |  
### 4.称重操作
  * 可对秤进行相关业务操作。

  
| 操作  | 操作说明  |  
| --- | --- |  
| 去皮  | 将当前秤上的物体进行去皮操作，让皮重不计入数值。  |  
| 预去皮  | 将当前秤上的物体进行预去皮操作，让皮重不计入数值。  |  
| 置零  | 将当前秤上所有数值清空。  |  
| 设置  | 对认证组件自身的参数进行设置设置小数点显示样式货币符号设置可设置货币符号人民币、美元、英镑、欧元可自定义符号主屏认证组件控制允许认证组件在主屏移动副屏图片设置可选择系统默认图片（黑色商米logo）可通过上传图片，自定义副屏背景图片展示风格设置可选择白色、黑色的颜色风格  |  
## 四. 电子秤服务使用方法
### 1、[下载SDK](https://developer.sunmi.com/docs/preview/zh-CN/xcqxeghjk491)
### 2、初始化SDK
导入jar包后, 连接Sunmi电⼦秤服务，当服务连接后即可使用SDK中的各个功能
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
ScaleManager scaleManager = ScaleManager.getInstance(context);

//绑定sunmi service 
scaleManager.connectService(newScaleManager.ScaleServiceConnection() {

  @Override
  public void onServiceConnected() {                 
    //服务绑定
  }
  @Override
  public void onServiceDisconnect(){                      
    //服务解绑
  }
}
  

```

Android 11 为了加强了隐私保护策略，引入了很多变更和限制，其中软件包可见性变更，将会导致第三方应用无法成功初始化SDK
因此需要在AndroidManifest.xml中添加如下:
<manifest package="开发者应用包名">
<queries>
<package android:name="com.sunmi.electronicscaleservice" />
</queries>
...
</manifest>
  

**特别需要注意的是，Android11 的该变更只会影响到升级** targetSdkVersion=30**的应用，未升级的应用暂不受影响**
### 3、获取称重数据
通过接⼝回调的方式快速获取电⼦秤数据的方法
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
scaleManager.getData(new ScaleResult(){

  @Override
  public void getResult(int net,int tare，boolean isStable) throws RemoteException {
    //这里返回称重结果
  } 

  @Override
  public void getStatus(boolean isLightWeight, boolean overload, boolean clearZeroErr, boolean calibrationErr) throws RemoteException {
    //这里返回称重状态
  }

  @Override
  public void getPrice(int net, int tare, int unit, String unitPrice, String totalPrice, boolean isStable, boolean isLightWeight) {
    //这里返回计价结果
  }

  @Override
  public void error(int errorCode) {
    //crc异常回调
  }

};
  

```

**称重结果参数说明**  
| 参数  | 类型  | 含义  | 说明  |  
| --- | --- | --- | --- |  
| net  | int  | 获取称量净重  | 单位：克  |  
| tare  | int  | 获取称量⽪重  | 单位：克  |  
| isStable  | boolean  | 秤稳定状态  | true:稳定 false：浮动  |  
**称重状态参数说明**  
| 参数  | 类型  | 含义  | 说明  |  
| --- | --- | --- | --- |  
| isLightWeight  | boolean  | 秤是否过轻  | true:过轻 false:正常  |  
| overload  | boolean  | 秤是否过载  | true:过载 false:正常  |  
| clearZeroErr  | boolean  | 秤是否有清零错误  | true:错误 false:正常  |  
| calibrationErr  | boolean  | 秤是否有标定错误  | true:错误 false:正常  |  
**称重价格参数说明**  
| 参数  | 类型  | 含义  | 说明  |  
| --- | --- | --- | --- |  
| net  | int  | 获取称量净重  | 单位：克  |  
| tare  | int  | 获取称量⽪重  | 单位：克  |  
| unit  | int  | 重量单位  | 0： g（默认）   
  
1 ：100g   
  
2： 500g   
  
3： kg  |  
| unitPrice  | String  | 当前计价设置的单价  | 默认为0，无单位（由业务方自定义）  |  
| totalPrice  | String  | 电子秤计算后的总价  | 当前计价根据重量和设置单价计算的总价，默认0  |  
| isStable  | boolean  | 秤稳定状态  | true:稳定 false：浮动  |  
| isLightWeight  | boolean  | 秤是否过轻  | true:过轻 false:正常  |  
计价接口返回需要系统中电子秤服务的版本在1.4.39以上
**CRC结果**
errorcode: 0 表示称重数据正常 非0 表示数据异常
对于称重结果在业务方软件中的展示推荐做法是：
醒目显示净重、皮重结果并有提示当前稳定状态、零点状态和净重状态的标志
**零点状态：皮重+净重 = 0**
**净重状态：皮重** ≠ **0**
当称重异常时可通过回调结果提示当前欠载、过载和通信异常状态
**欠载状态：需要判断当前重量是否小于 -20e，e为最小分度值可通过其他API获取**
### 4、其他接口说明  
| 功能  | 含义  | 说明  |  
| --- | --- | --- |  
| String getServiceVersion()  | 获取电子秤服务版本号  | 返回当前电子秤服务的版本号字符串  |  
| int getFirmwareVersion()  | 获取固件版本号  | 接返回当前固件的版本号-五位整型数据，如10034  |  
| void zero()  | 清零  | 清零操作只可清除300g以内的结果偏差  |  
| void tare()  | 去⽪/清⽪  | 秤上有重量时为去⽪，没有时为清⽪  |  
| void digitalTare(int i)  | 数字去⽪  | 直接给电子秤下发去皮的重量  |  
| void cancelGetData()  | 取消获取数据  | 退出应⽤时调⽤，与getData成对使⽤  |  
| int[] readAcceleData()  | 读取加速度数据  | [0][1][2]分别为 X，Y，Z⽅向数据  |  
| int readSealState()  | 获取铅封状态  | 0：正常 1：铅封被破坏  |  
| int getCalStatus()  | 读取标定按钮开关状态  | 0：未按下 1：按下  |  
| int[][] getCalInfo()  | 读取电子秤参数信息  | 返回值为一个多个量程的二维数组   
  
例如量程为6/15kg e=2/5g 多量程电子秤将返回[[6, 2],[15,5]]  |  
| void restart()  | 重启电子秤  | 电子秤重启会重新读取零点请谨慎调用此方法防止秤重读数不准确  |  
> 电子秤服务V1.4.39新增支持接口  
| 功能  | 含义  | 说明  |  
| --- | --- | --- |  
| void setUnitPrice(String unitPrice)  | 设置单价  | 由电子秤服务计算价格时设置，将影响返回的计价结果   
  
支持小数点两位的计算  |  
| String getUnitPrice()  | 获取当前已经设置的单价  | 由电子秤服务计算价格时使用的单价设置  |  
| void setUnit(int unit)  | 设置价格计算时的重量单位  | 0：按g计重   
  
1：按100g计重   
  
2：按500g计重   
  
3：按kg计重  |  
| int getUnit()  | 获取当前价格计算的重量单位  | 由电子秤服务计算价格时使用的价格重量单位，会影响计算总价的结果  |  
> 电子秤服务V1.4.52新增支持接口  
| 功能  | 含义  | 说明  |  
| --- | --- | --- |  
| int getScaleType()  | 获取当前电子秤的计量类型  | 电子秤服务从**1.4.52** 版本开始支持标定时选择公斤秤或   
  
磅秤标定，可通过此接口获取当前的计量类型   
  
返回0表示秤为公斤秤   
  
返回1表示秤为磅秤  |  
上一篇：ClientView 开源-副屏异显解决方案
下一篇：NFC相关SDK说明
