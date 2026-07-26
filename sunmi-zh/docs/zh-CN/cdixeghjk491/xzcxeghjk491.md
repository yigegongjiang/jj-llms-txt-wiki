---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xzcxeghjk491
---

# 生物识别（指纹）开发指南
更新时间：2026-04-03 23:37:01
# 核心背景（必看）
  1. **旧 API 废弃** ：`FingerprintManager`（API 23-28）被 Google 标记废弃，商米新 ROM / 新设备**禁用** 该接口，调用直接报错。
  2. **新 API 要求** ：强制迁移至 `BiometricPrompt + BiometricManager`（AndroidX 库），支持指纹 / 人脸，系统级安全验证，商米全系列 POS 原生适配。
  3. **官方文档** ：


  * Android 官方生物识别总览：<https://developer.android.google.cn/reference/android/hardware/biometrics/package-summary>
  * AndroidX Biometric 库文档：<https://developer.android.google.cn/jetpack/androidx/releases/biometric>
  * BiometricPrompt 开发指南：<https://developer.android.google.cn/training/sign-in/biometric-auth>


# 开发配置
## 依赖配置（Module 级 build.gradle）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
android {
    compileSdkVersion 34
    defaultConfig {
        minSdkVersion 23  // 最低系统版本
        targetSdkVersion 34
    }
}

dependencies {
    // 核心依赖
    implementation 'androidx.biometric:biometric:1.1.0'
    implementation 'androidx.appcompat:appcompat:1.6.1'
}
  

```

## 权限配置（AndroidManifest.xml）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
<!-- 生物识别核心权限 -->
<uses-permission android:name="android.permission.USE_BIOMETRIC" />
<!-- 兼容旧系统（可选） -->
<uses-permission android:name="android.permission.USE_FINGERPRINT" />

<!-- 声明硬件支持（非强制，避免过滤设备） -->
<uses-feature
    android:name="android.hardware.fingerprint"
    android:required="false" />
  

```

## 动态权限申请（Android 10+ 必做）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
private static final int REQUEST_BIOMETRIC_PERMISSION = 1001;

// 申请权限
private void requestBiometricPermission(Context context) {
    if (ContextCompat.checkSelfPermission(context, Manifest.permission.USE_BIOMETRIC)
            != PackageManager.PERMISSION_GRANTED) {
        ActivityCompat.requestPermissions((Activity) context,
                new String[]{Manifest.permission.USE_BIOMETRIC},
                REQUEST_BIOMETRIC_PERMISSION);
    }
}

// 权限回调
@Override
public void onRequestPermissionsResult(int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults) {
    super.onRequestPermissionsResult(requestCode, permissions, grantResults);
    if (requestCode <mark> REQUEST_BIOMETRIC_PERMISSION) {
        if (grantResults.length </mark> 0 || grantResults[0] != PackageManager.PERMISSION_GRANTED) {
            Toast.makeText(this, "指纹权限未开启，功能不可用", Toast.LENGTH_LONG).show();
        }
    }
}
  

```

# 核心对接代码
## 完整 Java 实现（Activity/Fragment）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import android.os.Bundle;
import android.widget.Toast;
import androidx.appcompat.app.AppCompatActivity;
import androidx.biometric.BiometricManager;
import androidx.biometric.BiometricPrompt;
import androidx.core.content.ContextCompat;
import java.util.concurrent.Executor;

public class SunmiBiometricActivity extends AppCompatActivity {

    private BiometricPrompt biometricPrompt;
    private BiometricPrompt.PromptInfo promptInfo;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_sunmi_biometric);

        // 1. 申请权限
        requestBiometricPermission(this);
        // 2. 初始化生物识别
        initBiometric();
        // 3. 绑定按钮触发验证
        findViewById(R.id.btn_verify).setOnClickListener(v -> startBiometricVerify());
    }

    /**
     * 初始化生物识别组件
     */
    private void initBiometric() {
        Executor executor = ContextCompat.getMainExecutor(this);
        // 构建验证回调
        biometricPrompt = new BiometricPrompt(this, executor, new BiometricPrompt.AuthenticationCallback() {
            @Override
            public void onAuthenticationSucceeded(BiometricPrompt.AuthenticationResult result) {
                super.onAuthenticationSucceeded(result);
                runOnUiThread(() -> {
                    Toast.makeText(SunmiBiometricActivity.this, "验证成功，执行业务", Toast.LENGTH_SHORT).show();
                    // TODO: 对接收银/登录/支付等业务逻辑
                });
            }

            @Override
            public void onAuthenticationFailed() {
                super.onAuthenticationFailed();
                runOnUiThread(() -> Toast.makeText(SunmiBiometricActivity.this, "指纹不匹配，请重试", Toast.LENGTH_SHORT).show());
            }

            @Override
            public void onAuthenticationError(int errorCode, CharSequence errString) {
                super.onAuthenticationError(errorCode, errString);
                runOnUiThread(() -> Toast.makeText(SunmiBiometricActivity.this, "验证错误：" + errString, Toast.LENGTH_LONG).show());
            }
        });

        // 构建验证弹窗
        promptInfo = new BiometricPrompt.PromptInfo.Builder()
                .setTitle("指纹验证")
                .setSubtitle("按压指纹完成收银/授权")
                .setNegativeButtonText("取消")
                .setAllowedAuthenticators(BiometricManager.Authenticators.BIOMETRIC_STRONG) // 强安全模式
                .build();
    }

    /**
     * 启动指纹验证（前置检查设备支持性）
     */
    private void startBiometricVerify() {
        BiometricManager biometricManager = BiometricManager.from(this);
        switch (biometricManager.canAuthenticate(BiometricManager.Authenticators.BIOMETRIC_STRONG)) {
            case BiometricManager.BIOMETRIC_SUCCESS:
                biometricPrompt.authenticate(promptInfo);
                break;
            case BiometricManager.BIOMETRIC_ERROR_NO_HARDWARE:
                Toast.makeText(this, "设备无指纹硬件", Toast.LENGTH_LONG).show();
                break;
            case BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED:
                Toast.makeText(this, "未录入指纹，请前往设置添加", Toast.LENGTH_LONG).show();
                break;
            default:
                Toast.makeText(this, "指纹功能不可用", Toast.LENGTH_LONG).show();
        }
    }

    // 动态权限申请方法（同 2.3）
    private void requestBiometricPermission(Context context) {
        // ... 复制 2.3 代码 ...
    }

    @Override
    public void onRequestPermissionsResult(int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults) {
        // ... 复制 2.3 代码 ...
    }
}
  

```

# 适配注意事项
  * **系统版本适配**
    * Android 6.0-8.1：自动兼容旧指纹底层，无需额外修改。
    * Android 9.0+：强制使用 `BiometricPrompt`，需确保 `targetSdkVersion ≥ 28`。
    * 商米新 POS（Android 11+）：彻底禁用 `FingerprintManager`，旧代码需完全删除。
  * **错误处理重点**

  
| 错误码  | 处理建议  |  
| --- | --- |  
| `ERROR_LOCKOUT`  | 提示用户 30 秒后重试  |  
| `ERROR_LOCKOUT_PERMANENT`  | 引导用户用设备密码解锁  |  
| `ERROR_HW_UNAVAILABLE`  | 建议用户重启设备或联系售后  |  
# 技术支持
商米开发者平台：[https://sunmideveloper.com](https://sunmideveloper.com/forum/zh-CN)
技术问题反馈：联系商米技术支持团队
上一篇：价签绑定货架编号
