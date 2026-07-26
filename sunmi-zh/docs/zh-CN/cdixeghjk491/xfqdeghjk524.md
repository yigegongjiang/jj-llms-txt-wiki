---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfqdeghjk524
---

# 6、收款音箱DEMO for Android

更新时间：2026-05-13 18:35:10

# 概述

本文档聚焦于演示通过手机扫描二维码无缝接入H5演示页面，生动展现收款音箱的语音播报功能。此过程涉及输入模拟金额，旨在全方位展示播报效果。敬请留意，此操作仅为演示之用，不涉及任何实际资金交易。

# 使用前准备事项

1、收款音箱仅提供4G通信方式，为确保顺利启用，请预先装备一张具有4G数据流量的Micro SIM卡，并将其插入设备的SIM卡插槽内。开启电源后，设备将自动连接至云服务平台，此时指示灯呈现蓝色，伴随响起的联网状况语音通报。正确接入网络后，收款音箱就可以进行DEMO体验了。

2、收款音箱接入云服务平台，还需要到商米Partner平台开通云应用能力，并获得【AppID】和【AppKey】。

![](https://cdn.sunmi.com/public/image/mgt-document/d782fb407f0c460aae0088faca82d019.png)

3、每个收款音箱都拥有独立的SN号，查看收款音箱标签，获得设备【SN】号。

![](https://cdn.sunmi.com/public/image/mgt-document/5d627d936c434bcfa4b0dd64c8361279.png)

4、收款音箱按开机键2秒开机，等待蓝灯常亮，就可以正常使用了。

（1）开关机键/Power button

长按：开机/关机；

短按：播报当前电量/网络信号质量；

（2）电源指示灯/Power light

蓝灯常亮：设备联网，且已绑定；

蓝灯呼吸：设备开机已绑定并充电中；

紫灯呼吸：关机充电；

紫灯常亮：关机已经充满；

红灯常亮：设备未绑定；

红灯闪烁：设备未联网；

（3）音量+按键/Volume up button

订单查询模式短按：播报上笔交易信息；

默认短按：音量增大；

（4）音量-按键/Volume down button

订单查询模式下短按：播报下笔交易信息；

默认短按：音量减小；

（5）重播键/Replay button

长按：进入订单查询模式；

订单查询模式短按：退出查询模式；

非订单查询模式短按：只重播当前最新的那一条（如果有）；

![](https://cdn.sunmi.com/public/image/mgt-document/7bbe4fcdc51b453fb3e7190c4a4cebf2.png)

# DEMO使用说明

## 1、扫描二维码进入页面

打开手机扫描以下二维码，进入DEMO页面。

备注：网址【https://h5.sunmi.com/soundbox/#/auth?sn=xxxxxxxxxxxx】

![](https://cdn.sunmi.com/public/image/mgt-document/59988ad26ef14e7fb6527e679ea16755.jpg)

## 2、手机NFC碰一碰进入页面

（1）首先在手机下载以下DEMO软件：

附件：NFCDemo.apk

（2）然后手机解锁，进入系统桌面，将手机NFC天线区域靠近码牌NFC天线区域，读取信息进入DEMO页面。

![](https://cdn.sunmi.com/public/image/mgt-document/89d3b8460ca84d199410eba480389edc.JPG)

## 3、DEMO操作页面

（1）首次扫码进入后，在授权界面输入之前准备好的【AppID】、【AppKey】、【SN】，点击【Confirm】绑定设备。

![](https://cdn.sunmi.com/public/image/mgt-document/61730ca9b91a4b3da9988b9a1f632c0e.png)

![](https://cdn.sunmi.com/public/image/mgt-document/0f82633be29340bba727460721111b1a.png)

（2）进入测试界面时，收款音箱会播报一个【准备支付】声音。

![](https://cdn.sunmi.com/public/image/mgt-document/b67937b3750248df9d7cb16c963b3b43.png)

（3）在【Payment】页面输入需要播报的金额，点击【Confirm】，发送成功，会提示【Payment successful】，同时收款音箱会播报支付成功的金额语音。

![](https://cdn.sunmi.com/public/image/mgt-document/646196f0dec94e04ae27bfd6ba99efab.png)

（4）在【Refund】页面输入需要播报的金额，点击【Confirm】，发送成功，会提示【Refund successful】，同时收款音箱会播报退款成功的金额语音。

![](https://cdn.sunmi.com/public/image/mgt-document/0cc4ff357b11425f97115aee8f6d93b3.png)

（5）点击【Cancel】退出，收款音箱会播报客户取消支付的提示语音。

![](https://cdn.sunmi.com/public/image/mgt-document/777e17c00337494286d318dfcca5a41b.png)

---

上一篇：5、远程管理收款音箱
下一篇：7、收款音箱DEMO for IOS
