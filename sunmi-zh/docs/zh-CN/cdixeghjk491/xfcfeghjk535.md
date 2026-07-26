---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfcfeghjk535
---

# 副屏API接口文档
更新时间：2025-09-25 11:44:51
## 副屏显示 API：（Android 原生 API）
<https://developer.android.google.cn/reference/android/app/Presentation>
## 调用参考案例
您可以参考以下代码对设备副屏进行适配， 保障您的业务正常进行：
双屏功能： 双屏使用的是**Presentation** 类来实现双屏异显： 首先是权限 创建一个类继承Presentation publicclassTextDisplayextendsPresentation
**获得真实存在的副屏**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public Display getPresentationDisplays() {
    DisplayManager mDisplayManager= (DisplayManager)getSystemService(Context.DISPLAY_SERVICE);
    Display[] displays =mDisplayManager.getDisplays();
    for(int i=0;  i < displays.length; i++){
        Log.e(TAG,"屏幕" +displays[i] + " Flag: " + displays[i].getFlags());
        if((displays[i].getFlags() & Display.FLAG_SECURE)!=0
                &&(displays[i].getFlags() & Display.FLAG_SUPPORTS_PROTECTED_BUFFERS)!=0
                &&(displays[i].getFlags() & Display.FLAG_PRESENTATION) !=0){
            Log.e(TAG,"第一个真实存在的副屏屏幕" + displays[i]);
            return displays[i];
        }
    }
    return null;
}
  

```

**显示副屏**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
textDisplay=newTextDisplay(this,getPresentationDisplays());

textDisplay.show();
  

```

特殊说明： 如果希望主屏Activity返回桌面后，副屏View仍然显示，可 以使用如下代码完成：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
getWindow()/*副屏的
Window*/.setType(WindowManager.LayoutParams.TYPE_SY
STEM_OVERLAY);
  

```

注意：普通应用添加此类型Window时，需要使用如下代码申请权限
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
if(!Settings.canDrawOverlays(this)){
Toast.makeText(this,"请同意显示窗口权限",To
ast.LENGTH_SHORT).show();
startActivity(newIntent(Settings.ACTION_MAN
AGE_OVERLAY_PERMISSION));
}
  

```

上一篇：扫码器使用指南
下一篇：USB显示器介绍
