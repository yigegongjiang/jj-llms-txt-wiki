---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmafeghjk535
---

# 快速入门
更新时间：2026-05-29 19:17:46
本指南将介绍如何注册商米开发者账号、搭建开发环境、集成商米统一 SDK 并完成应用调试与上架的完整流程。
## 1. 整体流程概览
从样机申请到应用上架的核心工作流如下：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
[ 申请测试样机 ] ──> [ 注册 Partner 平台 ] ──> [ 申请开发者认证 ]
                                                       │
                                                       ▼
[ 应用发布上架 ] <── [ 集成开发与调试 ] <── [ 获取设备开放能力 ]
  

```

## 2. 前期准备与认证
在开始编码前，需要完成商米环境的入驻认证以获取对应的平台权限。
### 2.1 申请测试样机
请联系对接的商米销售人员申请开发测试机型。
  * **提交资料** ：企业信息、联系人、目标机型需求、详细开发计划。
  * **审核周期** ：1～3 个工作日。


### 2.2 注册商米 Partner 平台账号
根据业务出海或本地化需求，选择对应的云服务站点注册企业账户：
  * **中国/全球站点** ：<https://partner.sunmi.com>
  * **北美站点** ：<https://partner.us.sunmi.com>
  * **欧洲站点** ：<https://partner.eu.sunmi.com>


> **注意** ：建议使用**公司官方邮箱** 注册，以便于后续的设备批量管理与应用合规发布。
### 2.3 申请开发者认证
  * 登录商米 Partner 平台，进入 **申请成为开发者** 页面。
  * 选择主体类型（企业开发者 / 个人开发者），并提交对应资质。
  * 等待平台审核（通常为 1~3 个工作日）。审核通过后即可激活商米开放能力的接入权限。


## 3. 开发环境搭建
商米设备运行的 Sunmi OS 基于 Android 系统深度定制。应用开发采用原生 Android 开发标准。
### 前置条件（Prerequisites）
  * **开发工具** ：Android Studio（推荐最新稳定版）
  * **开发语言** ：Java / Kotlin
  * **最低配置** ：建议兼容至 Android 9（API 级别 28）及以上，具体取决于样机型号。


### 3.1 添加 SDK 依赖
在模块级别的 `build.gradle` (通常为 `app/build.gradle`) 文件中添加商米打印服务库（以基础打印为例）：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
dependencies {
    // 引入商米核心打印 SDK
    implementation 'com.sunmi:printerlibrary:1.0.23'
}
```

### 3.2 绑定底层硬件服务
由于商米设备通过系统级 Service 通信，应用启动时需异步绑定商米打印服务（`SunmiPrinterService`）。
请在自定义的 `Application` 类中实现以下初始化逻辑：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
package com.sm.sdk.demo;

import android.app.Application;
import android.content.Context;
import com.sunmi.peripheral.printer.InnerPrinterCallback;
import com.sunmi.peripheral.printer.InnerPrinterException;
import com.sunmi.peripheral.printer.InnerPrinterManager;
import com.sunmi.peripheral.printer.SunmiPrinterService;

public class MyApplication extends Application {
    private static MyApplication instance;
    private SunmiPrinterService sunmiPrinterService;

    public static MyApplication getInstance() {
        return instance;
    }

    @Override
    public void onCreate() {
        super.onCreate();
        instance = this;
        bindPrintService(this);
    }

    /**
     * 异步绑定商米内置打印服务
     */
    private void bindPrintService(Context context) {
        try {
            InnerPrinterManager.getInstance().bindService(context, new InnerPrinterCallback() {
                @Override
                protected void onConnected(SunmiPrinterService service) {
                    sunmiPrinterService = service;
                }

                @Override
                protected void onDisconnected() {
                    sunmiPrinterService = null;
                }
            });
        } catch (InnerPrinterException e) {
            e.printStackTrace();
        }
    }

    /**
     * 获取打印服务实例
     * @return SunmiPrinterService，可能为 null（未连接成功时）
     */
    public SunmiPrinterService getSunmiPrinterService() {
        return sunmiPrinterService;
    }
}
```

## 4. 应用开发与调试
### 4.1 示例：调用打印机输出文本
在 `Activity` 中，通过 `MyApplication` 获取服务实例并调用 `printText` 方法。**注意：生产环境必须进行非空校验。**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
package com.sm.sdk.demo;

import android.os.Bundle;
import android.util.Log;
import androidx.appcompat.app.AppCompatActivity;
import com.sunmi.peripheral.printer.SunmiPrinterService;

public class MainActivity extends AppCompatActivity {
    private static final String TAG = "SunmiDebug";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        // 执行测试打印
        executeTextPrint("Hello SUNMI!\n");
    }

    private void executeTextPrint(String text) {
        SunmiPrinterService printerService = MyApplication.getInstance().getSunmiPrinterService();
        
        if (printerService != null) {
            try {
                // 调用打印接口，第二位参数为回调控制（此处传 null 使用默认配置）
                printerService.printText(text, null);
            } catch (Exception e) {
                Log.e(TAG, "打印指令发送失败", e);
            }
        } else {
            Log.w(TAG, "商米打印服务未就绪，请检查服务绑定状态或设备型号。");
        }
    }
}
```

### 4.2 真机调试步骤
商米设备支持标准 Android 调试模式。
  1. 使用 **USB 数据线** 将商米真机连接至开发电脑。
  2. 在商米设备上进入 **设置 - > 关于设备**，连续点击 **版本号** 触发开发者模式。
  3. 返回上一级菜单，进入 **开发者选项** ，开启 **USB 调试** 。
  4. 在 Android Studio 的设备列表中选择该商米设备，点击 **Run (▶)** 编译并运行。


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Hello SUNMI!
```

## 5. 应用发布与上架
开发测试完成后，您可以通过商米应用市场（Sunmi App Store）向特定设备或全球市场分发您的应用。
### 5.1 准备上架素材
在 Partner 平台提交审核前，请确保准备好以下合规资产：
  * **编译产物** ：正式签名的 `APK` 文件（建议配置 V2/V3 签名）。
  * **视觉素材** ：高分辨率应用图标（Icon）、标准尺寸的应用截图（至少 3 张）。
  * **合规文本** ：多语言应用简介、明确的**隐私政策 URL** 以及敏感权限声明。


### 5.2 提交审核与定向分发
  1. 登录 [商米 Partner 平台](https://partner.sunmi.com)。
  2. 进入应用管理页面，填写版本信息并上传 APK。
  3. **配置分发策略** ：商米平台支持高度定制化的灰度发布。您可以指定按**设备型号** 、**所属区域** 、或特定的**商户账号** 进行定向灰度推送。
  4. 提交商米官方进行安全与功能合规审核（通常需要 1~3 个工作日）。审核通过后，应用将自动上架至指定的 `Sunmi App Store` 区域。


## 6. 获取技术支持
在集成开发过程中遇到疑难问题，可通过以下渠道获取帮助：
  * **完整官方技术文档** ：[商米开发者文档中心](https://docs.sunmi.com/zh-CN/ciczeghjk557/xdxmeghjk546)
  * **技术支持热线** ：400-6666-509（工作日 9:00 - 18:00）


  

下一篇：USB 调试管控使用指南
