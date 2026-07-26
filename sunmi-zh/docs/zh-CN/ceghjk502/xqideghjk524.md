---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xqideghjk524
---

# L3
更新时间：2026-05-22 12:09:26
## **1、L3产品介绍**  
|  ![](https://cdn.sunmi.com/public/image/mgt-document/56e547ad2dc9496c826813f79eb10bea.png)  |  ![](https://cdn.sunmi.com/public/image/mgt-document/0ee562457158498fba4c43b3683fb25b.png)  |  
| --- | --- |  
[更多产品详情 →](https://www.sunmi.com/zh-CN/l3/)
  

  * 资源下载：


  * [产品单页下载](https://cdn.sunmi.com/public/generalfile/mgt_import/9c0368888c3f4574a90d29277845f3df.pdf)
    * [产品六视图下载](https://cdn.sunmi.com/public/generalfile/mgt_import/0d930c893acf4e898f24b8817fc51c2c.zip)
    * [开箱视频下载](https://cdn.sunmi.com/public/generalfile/mgt_import/30e170ed1c2d471a9036d47543748904.mp4)
    * [场景演示下载](https://cdn.sunmi.com/public/generalfile/mgt_import/71b46e45169d4dd582a002f3b58dcf99.zip)


  * 技术规格
    * L3
![](https://cdn.sunmi.com/public/image/mgt-document/49720a3f8bff438b87d3455282490e98.png)


  

## **2、软件开发说明**
  * 开启设备的USB调试
    * [调试设备说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrzeghjk557)
  * 设备开发集成
    * [如何获取设备信息](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdqieghjk579)
    * 扫码
      * [摄像头扫码说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfafeghjk535)
      * [扫码头引擎（红外线扫码）](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfareghjk568)
    * NFC
      * 参考谷歌官方文档，NFC API接口说明： [谷歌官方的安卓原生NFC API接口说明](https://developer.android.google.cn/reference/android/nfc/package-summary)
    * 指纹
      * [生物识别（指纹）开发指南](https://developer.sunmi.com/docs/preview/zh-CN/xzcxeghjk491)
    * UHF RFID
      * [RFID SDK集成说明](https://docs.sunmi.com/zh-CN/cdixeghjk491/xfcreghjk568)
    * 其他开发小贴士
      * [自定义音量键](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrieghjk579)
      * [如何实现应用全屏显示](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrceghjk502)
      * [如何避免重复申请外设权限](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrqeghjk513)
      * [如何设置应用自动安装/更新](https://developer.sunmi.com/docs/zh-CN/cicmeghjk546/xcrieghjk579)
  * 应用发布
    * [商米应用市场发布应用说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/qaaeghjk480)


  

## **3、常见问题**
###  **Q1.** 设备扫码返回乱码，如何快速诊断？
  * 此问题通常与扫码头的激活状态或通信异常有关。
    * **诊断方法** ：
      * 进入设备 **【设置】 >【扫码设置/硬件测试】​ 等相关菜单，找到内置的扫码测试工具**。
      * 使用该工具扫描一个标准的条码（如商品上的EAN-13码）。
      * **观察返回结果** ：
        * **正常激活** ：工具清晰地显示出条码的原始数字/字符数据。（正常激活的扫码头将返回以下数据）
        * **未激活或异常** ：工具显示乱码、空白或无任何数据返回。


  * **若诊断结果为“未激活或异常”** ：请收集该**设备的SN码** ，并提供给技术支持，以便查询设备的激活记录和进行下一步处理。


###  **Q2.** 扫码时断时续、经常丢码怎么办？​
此问题最常见的原因是扫码窗口镜片脏污。
  * 解决方案：


  1. 使用柔软、不起毛的清洁布（如眼镜布）。
  2. 可蘸取少量电子设备清洁剂或高纯度酒精。
  3. 轻轻擦拭扫码器或设备摄像头的镜片/窗口表面，确保其洁净无指纹、油污或灰尘。


  * 后续验证：


清洁完毕后，测试扫码连贯性是否恢复正常。
如果问题依旧，请参考《[手持设备扫码问题综合排查与快速定位指南​](https://sunmideveloper.com/forum/zh-CN/questions/10010000000000428/guan-fang-xiang-jie-shou-chi-she-bei-sao-ma-wen-ti-zong-he-pai-cha-yu-kuai-su-ding-wei-zhi-nan)》进行进一步排查。
  

**扩展阅读​**
关于商米扫码服务的完整文档与API详情，请参阅：[《](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdzaeghjk480)[商米扫码服务开发指南](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfafeghjk535)[》](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdzaeghjk480)
**需要更多帮助？​**
欢迎在评论区继续交流。如需人工支持，请点击右下角「在线客服」。
上一篇：K1
下一篇：L2
