---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xqdxeghjk491
---

# V3
更新时间：2026-05-22 12:10:51
## **1、V3产品介绍**  
|  ![](https://cdn.sunmi.com/public/image/mgt-document/ccff429739f746aba8cecae2a829c40c.png)  |  ![](https://cdn.sunmi.com/public/image/mgt-document/28411b91a05c47a0ae370deb44e46483.png)  |  
| --- | --- |  
[更多产品详情 →](https://www.sunmi.com/zh-CN/v3-family)
  

  * 资源下载：


  * [产品单页下载](https://cdn.sunmi.com/public/generalfile/mgt_import/bf5d9303a874447ca90f7b591a28c441.pdf)
    * [产品六视图下载](https://cdn.sunmi.com/public/generalfile/mgt_import/de9b7f9f9754456aa977dd5ae523efe5.zip)
    * [开箱视频下载](https://cdn.sunmi.com/public/generalfile/mgt_import/6b86631d1c7c4423a8d581cab8c30e30.mp4)
    * [场景演示下载](https://cdn.sunmi.com/public/generalfile/mgt_import/57917845b0df40a596e10960ffa362bd.zip)


  * 技术规格//技术参数/产品规格/产品详情/产品资料/产品参数明细

  
| V3 |  
| --- |  
|  **认证型号** T5F1A  |  **处理器** 高通六核处理器，最高 2.4GHz  |  **操作系统** 商米OS (Based on Android 13 64bit)  |  **内存** 3GB+32GB  |  **显示屏** 6.75" HD+，720 * 1600，IPS  |  
 |  
|  **触摸屏** 电容多点式触摸  |  **打印机** 高速58热敏打印，最高打印速度80mm/s， 支持50mm直径打印卷纸， 支持热敏小票 支持标签打印  |  **按键** 电源键， 音量键， 扫码键*2(可选）  |  **SIM卡槽** 2* Nano SIM card  |  **PSAM卡槽** 2*PSAM  |  
 |  
|  **SD卡槽** Micro SD card  |  **接口** USB接口，Type-C支持 OTG  |  **网络支持** 4G/3G/2G  |  **Wi-Fi** 2.4GHz/5GHz,Support IEEE 802.11 a/b/g/n/ac  |  **蓝牙** 支持 2.1/3.0/4.2/5.0， BLE  |  
 |  
|  **GPS** GPS/Glonass/Beidou/Galileo  |  **摄像头** 后摄：5MP 自动对焦+闪光灯，支持1D/2D条码识别（8M 可选）  |  **扫码头** 选配激光扫码头  |  **NFC** 选配  |  **音频** 扬声器 95dB 无麦克风  |  
 |  
|  **电池** 可拆卸电池锂电池 7.7V 3100mAh  |  **Pogo Pin** 6 Pin 扩展 USB接口  |  **电源适配器** 输入：AC 100-240V 输出：5V/2A  |  **尺寸** 238*81.8*16.8 mm  |  **重量** 419克(带电池）  |  
 |  
|  **工作环境** 工作温度：-10°C~50°C 存储温度：-20°C~60°C  |  **配件（可选）** 扩展底座 硅胶保护套 屏幕保护膜 挂绳 手带  |   
 |   
 |   
 |  
  

## **2、软件开发说明**
  * 开启设备的USB调试
    * [调试设备说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrzeghjk557/)
  * 设备开发集成
    * [如何获取设备信息](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdqieghjk579)
    * 扫码
      * [摄像头扫码说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfafeghjk535/)
      * [扫码头引擎（红外线扫码）](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfareghjk568/)
    * 打印
      * [打印说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdzceghjk502)
    * NFC
      * 参考谷歌官方文档，NFC API接口说明： [谷歌官方的安卓原生NFC API接口说明](https://developer.android.google.cn/reference/android/nfc/package-summary)
    * 指纹
      * [生物识别（指纹）开发指南](https://developer.sunmi.com/docs/preview/zh-CN/xzcxeghjk491)
    * 其他开发小贴士
      * [自定义音量键](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrieghjk579/)
      * [如何实现应用全屏显示](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrceghjk502/)
      * [如何避免重复申请外设权限](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrqeghjk513/)
      * [如何设置应用自动安装/更新](https://developer.sunmi.com/docs/zh-CN/cicmeghjk546/xcrieghjk579)
  * 应用发布
    * [商米应用市场发布应用说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/qaaeghjk480)


  

## **3、常见问题**
###  **Q1.** 打印功能支持 Flutter、uni-app 等跨平台插件吗**？**
为简化并加速开发者在商米打印机上的适配流程，商米提供了对应技术栈的插件。第三方开发者只需调用相应插件，即可便捷地使用商米设备的内置打印机，轻松实现打印功能，从而提升开发效率、降低对接成本。目前支持的插件包括：
  * JavaScript
  * Flutter
  * uni-app


###  **Q2.** 设备扫码返回乱码，如何快速诊断？
  * 此问题通常与扫码头的激活状态或通信异常有关。
    * **诊断方法** ：
      * 进入设备 **【设置】 >【扫码设置/硬件测试】​ 等相关菜单，找到内置的扫码测试工具**。
      * 使用该工具扫描一个标准的条码（如商品上的EAN-13码）。
      * **观察返回结果** ：
        * **正常激活** ：工具清晰地显示出条码的原始数字/字符数据。（正常激活的扫码头将返回以下数据）
        * **未激活或异常** ：工具显示乱码、空白或无任何数据返回。


  * **若诊断结果为“未激活或异常”** ：请收集该**设备的SN码** ，并提供给技术支持，以便查询设备的激活记录和进行下一步处理。


  

  

上一篇：V3 MIX
下一篇：V3 PLUS
