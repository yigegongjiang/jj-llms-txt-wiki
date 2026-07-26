---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfafeghjk535
---

# 摄像头扫码SDK说明
更新时间：2026-03-05 12:27:51
# 一. 概述
商米提供了带摄像头设备的扫码SDK，并具有以下五个优势：
  * **高识别率** ：相对ZXing等开源扫码方案有更高的识读成功率，污损扭曲条码解码效果更好；
  * **快速扫码** ：百万像素毫秒级解码；
  * **简单便捷** ：几行代码即可集成到客户应用程序；
  * **高适配度** ：与商米的设备完美适配，软硬件结合可以保证功能的高效稳定。
  * **支持多种常用码制** ：已支持EAN-8, EAN-13, UPC-A, UPC-E, Codabar, Code39, Code93, Code128, ISBN10, ISBN13, ISSN, DataBar, DataBar Expanded, Interleaved 2 of 5, QR Code, MicroQR, PDF417, MicroPDF417，DataMatrix，AZTEC, Hanxin.


# 二. 使用说明
开发者有两种方式使用商米扫码SDK：
  1. 开发者的应用调用SUNMIUI系统集成的扫码模块完成扫码，获取返回值，该方法简单易用。
  2. 开发者自己开发相机预览扫码界面，调用商米扫码SDK完成图像数据的解析，该方式相对复杂，但提供了较高的灵活度。


### 方式一
为了降低开发难度，SUNMI OS内置了一个扫码的模块，开发者在项目需要调用扫码的地方通过startActivityForResult()调用商米的扫码模块，然后在onActivityResult()方法中接收扫码结果返回值。调用商米的扫码模块的示例代码如下：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
对于target SDK 大于等于 Android 11 的项目需要在配置文件 AndroidManifest.xml 增加一下声明：

<manifest>

  <!-- Android target sdk >= 30 add -->

  <queries>

    <package android:name="com.sunmi.scanner" />

    <package android:name="com.sunmi.sunmiqrcodescanner" />

  </queries>

...

</manifest>
/*
外部应用在自己的业务代码需要启动扫码的地方使用下面的方式创建Intent，然后使用startActivityForResult()调用起商米的扫码模块
*/
public static PackageInfo getPackageInfo(Context context, String pkg) {
    PackageInfo packageInfo;
    try {
        packageInfo = context.getPackageManager().getPackageInfo(pkg, 0);
    } catch (PackageManager.NameNotFoundException e) {
       packageInfo = null;
       e.printStackTrace();
    }
    return packageInfo;
}

public boolean hasScanner(Context ctx) {
    PackageInfo info = getPackageInfo(ctx, "com.sunmi.scanner");

    return info != null && compareVer(info.versionName, "4.4.4", true, 3);
}

public boolean compareVer(String nVer, String oVer, boolean isEq, int bit) {
    if (nVer.isEmpty() || oVer.isEmpty()) return false;
    String[] nArr = nVer.split("[.]");
    String[] oArr = oVer.split("[.]");
    if (nArr.length < bit || oArr.length < bit) return false;
    boolean vup = false;
    for (int i = 0; i < bit; i++) {
        int n = Integer.parseInt(nArr[i]);
        int o = Integer.parseInt(oArr[i]);
        if (n >= o) {
           if (n > o) {
               vup = true;
               break;
            } else if (isEq && i == (bit - 1)) {
               vup = true;
               break;
            }
        } else {
           break;
        }
   }
   return vup;
}

//注意：旧版本SunmiScanner组件v1.x.x
Intent intent = new Intent("com.summi.scan");

//注意：设备中含有ScannerHead组件v4.4.4及以上版本
if (hasScanner(Context ctx)) {
    intent.setAction("com.sunmi.scanner.qrscanner");
}

intent.putExtra("PLAY_SOUND", true);// 扫描完成声音提示 默认true
intent.putExtra("PLAY_VIBRATE", false);
//扫描完成震动,默认false，目前M1硬件支持震动可用该配置，V1不支持
intent.putExtra("IDENTIFY_MORE_CODE", false);// 识别画面中多个二维码，默认false
intent.putExtra("IS_SHOW_SETTING", true);// 是否显示右上角设置按钮，默认true
intent.putExtra("IS_SHOW_ALBUM", true);// 是否显示从相册选择图片按钮，默认true
intent.putExtra("IDENTIFY_INVERSE", true);// 允许识读反色二维码，默认true
intent.putExtra("IS_EAN_8_ENABLE", true);//允许识读EAN-8码，默认true：允许
intent.putExtra("IS_UPC_E_ENABLE", true);//允许识读UPC-E码，默认true：允许
intent.putExtra("IS_ISBN_10_ENABLE", false);//允许识读ISBN-10 (from EAN-13)码，默认false：不允许
intent.putExtra("IS_CODE_11_ENABLE", true);//允许识读CODE-11码，默认false：不允许
intent.putExtra("IS_UPC_A_ENABLE", true);//允许识读UPC-A码，默认true：允许
intent.putExtra("IS_EAN_13_ENABLE", true);//允许识读AN-13码，默认true：允许
intent.putExtra("IS_ISBN_13_ENABLE", true);//允许识读ISBN-13 (from EAN-13)码，默认true：允许
intent.putExtra("IS_INTERLEAVED_2_OF_5_ENABLE", true);//允许识读Interleaved 2 of 5码，默认false：不允许
intent.putExtra("IS_CODE_128_ENABLE", true);//允许识读ECode 128码，默认true：允许
intent.putExtra("IS_CODABAR_ENABLE", true);//允许识读Codabar码，默认true：允许
intent.putExtra("IS_CODE_39_ENABLE", true);//允许识读Code 39码，默认true：允许
intent.putExtra("IS_CODE_93_ENABLE", true);//允许识读Code 93码，默认true：允许
intent.putExtra("IS_DATABAR_ENABLE", true);//允许识读DataBar (RSS-14)码，默认true：允许
intent.putExtra("IS_DATABAR_EXP_ENABLE", true);//允许识读DataBar Expanded码，默认true：允许
intent.putExtra("IS_Micro_PDF417_ENABLE", true);//允许识读Micro PDF417码，默认true：允许
intent.putExtra("IS_MicroQR_ENABLE", true);//允许识读Micro QR Code码，默认true：允许
intent.putExtra("IS_OPEN_LIGHT", true);// 是否显示闪光灯，默认false
intent.putExtra("SCAN_MODE", false);// 是否是循环模式，默认false
intent.putExtra("IS_QR_CODE_ENABLE", true);// 允许识读QR码，默认true
intent.putExtra("IS_PDF417_ENABLE", true);// 允许识读PDF417码，默认false
intent.putExtra("IS_DATA_MATRIX_ENABLE", true);// 允许识读DataMatrix码，默认false
intent.putExtra("IS_AZTEC_ENABLE", true);// 允许识读AZTEC码，默认false
intent.putExtra("IS_Hanxin_ENABLE", false);// 允许识读Hanxin码，默认false
startActivityForResult(intent, START_SCAN);
  

```

在onActivityResult方法中接收返回的扫码结果参数：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
@Override
protected void onActivityResult(int requestCode, int resultCode, Intent data) {
	super.onActivityResult(requestCode, resultCode, data);
	if (requestCode == START_SCAN && data != null) {
		Bundle bundle = data.getExtras();
		ArrayList result = (ArrayList<HashMap<String, Object>>) bundle .getSerializable("data");
		Iterator<> it = result.iterator();
		while (it.hasNext()) {
			HashMap hashMap = it.next();
			Log.i("sunmi", hashMap.get("TYPE"));//扫码类型
			Log.i("sunmi", hashMap.get("VALUE"));//扫码结果
		}
	}
}
  

```

### 方式二
适用于对安卓设备摄像头的使用比较熟悉的开发者，可以自行打开摄像头获取预览数据流，然后调用SDK解码。
**1.添加依赖项**
在项目的libs目录中按以下层级添加**libsunmiscan.so** 和**sunmiscan.jar** 两个文件，以及在build.gradle添加扫码库的引用，如下图所示：
![](https://cdn.sunmi.com/public/image/mgt-document/1ae25bce14e34889992bab7dcb831fea.png)
**2.引入相关java接口类**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import com.sunmi.camerascan.Config;
import com.sunmi.camerascan.Image;
import com.sunmi.camerascan.ImageScanner;
import com.sunmi.camerascan.Symbol;
import com.sunmi.camerascan.SymbolSet;
  

```

**3.实例创建**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//创建ImageScanner实例
private ImageScanner scanner; 
scanner = new ImageScanner();

/*创建解码图像实例
"previewSize_width"和"previewSize_height"分别为摄像头预览分辨率的宽度和高度，一般来说,分辨率越高图像越清晰，
但解码速度越慢，建议选取不大于1280X720分辨率，如640X480,800X480,1280X720，推荐的分辨率为1280X720，可以通过
getSupportedPreviewSizes获取当前设备支持的分辨率。目前仅支持灰度图像数据（即每个像素点的灰度值范围为0~255），
如果使用安卓预览的图像数据，其默认格式为YCbCr_420_SP, 参数"Y800"表示取YUV的“Y”分量数据。
*/
Image source = new Image(previewSize_width, previewSize_height, "Y800");
  

```

**4.参数配置**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//可根据需要设置相关参数
/*
//是否开启同一幅图一次解多个条码,0：只解一个，1：可解多个条码
scanner.setConfig(Symbol.NONE, Config.ENABLE_MULTILESYMS, 0);//默认0: 只解一个

//是否开启/不开启识读相关条码码制
scanner.setConfig(Symbol.EAN8, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.UPCE, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.UPCA, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.EAN13, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.ISBN10, Config.ENABLE, 0);//默认0：不开启，ISBN13,ISBN10不能同时设置为1
scanner.setConfig(Symbol.ISBN13, Config.ENABLE, 0);//默认0：不开启，ISBN13,ISBN10不能同时设置为1
scanner.setConfig(Symbol.CODE11, Config.ENABLE, 0);//默认0：不开启
scanner.setConfig(Symbol.I25, Config.ENABLE, 0);//默认0：不开启
scanner.setConfig(Symbol.CODE128, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.CODABAR, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.CODE39, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.CODE93, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.DATABAR, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.DATABAR_EXP, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.QRCODE,Config.ENABLE, 1);//默认1: 开启
scanner.setConfig(Symbol.QRCODE, Config.ENABLE_INVERSE, 2);//0：normal, 1：only inverse，默认2：auto
scanner.setConfig(Symbol.MicroQR, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.MicroQR, Config.ENABLE_INVERSE, 2);//0：normal, 1：only inverse，默认2：auto
scanner.setConfig(Symbol.PDF417,Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.PDF417, Config.ENABLE_INVERSE, 0);//暂不支持反色读取
scanner.setConfig(Symbol.MicroPDF417, Config.ENABLE, 0);//默认0：不开启
scanner.setConfig(Symbol.MicroPDF417, Config.ENABLE_INVERSE, 0);//暂不支持反色读取
scanner.setConfig(Symbol.DataMatrix, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.DataMatrix, Config.ENABLE_INVERSE, 2);//0：normal, 1：only inverse，默认2：auto
scanner.setConfig(Symbol.AZTEC, Config.ENABLE, 1);//默认1：开启
scanner.setConfig(Symbol.AZTEC, Config.ENABLE_INVERSE, 2);//0：normal, 1：only inverse，默认2：auto
scanner.setConfig(Symbol.Hanxin, Config.ENABLE, 0);//默认0：不开启
scanner.setConfig(Symbol.Hanxin, Config.ENABLE_INVERSE, 2);//0：normal, 1：only inverse，默认2：auto
*/
  

```

**5.解码**
如果是使用android系统相机解码，可以在图像预览回调函数即PreviewCallback.onPreviewFrame(byte[] data, Camera camera)方法中直接调用预览数据解码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
source.setData(data); //填充图像数据,data为摄像头yuv数据，解码库仅使用其y分量
int result = scanner.scanImage(source); //解码
  

```

_注：目前解码库只处理灰度图像数据（即每个像素点的灰度值范围为0~255），如果是其他形式的图像数据，如相册图片一般是JPG等其他格式，需要将相应格式的图像转换为BMP格式（如果是彩色图像，还需要转化为灰度图像），然后提取其中的图像数据部分进行解码。_
**6.获取解码结果**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// result > 0：解码成功
// result == 0：解码失败
// result<0：其他异常（目前代表内存分配失败）
if(result > 0){
	SymbolSet syms = scanner.getResults();
	for (Symbol sym : syms) {
		Log.i("sunmi", "码制:"+sym.getSymbolName());//条码类型,如“EAN-8”
		Log.i("sunmi","结果:"+sym.getResult())//解码结果字符串，UTF-8格式字符串
        //Log.i("sunmi","结果:"+sym.getBytes())//原始字节模式输出
	}
}  
  

```

**7.实例销毁**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
scanner.destroy();
  

```

注： 商米扫码SDK 只有一个实例，是线程不可重入的。只能同时由一个线程调用，不能在多个线程里面同时调用。
# 三. 资源下载
1.最新解码库SDK：
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) libs(sunmi_scanner_sdk_v1.5.8).zip 
2.对于手持设备如V, P, M系列的安卓设备，用户可直接使用android提供的摄像头调用api对摄像头进行操作，如果开发者对摄像头调用不是很熟悉建议使用方式一的调用方法。
3.对于T1mini/T2mini等台式设备（如果用户把手持机器的扫码功能移植到T1mini/T2mini上，需要调整分辨率和显示界面的大小，注意DEMO中的libs文件请自行更新到最新版本）
![](data:image/svg+xml,%3csvg%20width='16'%20height='16'%20viewBox='0%200%2016%2016'%20fill='none'%20xmlns='http://www.w3.org/2000/svg'%3e%3cpath%20d='M4.5%203C4.5%201.61929%205.61929%200.5%207%200.5C8.38071%200.5%209.5%201.61929%209.5%203V12C9.5%2012.8284%208.82843%2013.5%208%2013.5C7.17157%2013.5%206.5%2012.8284%206.5%2012V5C6.5%204.72386%206.72386%204.5%207%204.5C7.27614%204.5%207.5%204.72386%207.5%205V12C7.5%2012.2761%207.72386%2012.5%208%2012.5C8.27614%2012.5%208.5%2012.2761%208.5%2012V3C8.5%202.17157%207.82843%201.5%207%201.5C6.17157%201.5%205.5%202.17157%205.5%203V12C5.5%2013.3807%206.61929%2014.5%208%2014.5C9.38071%2014.5%2010.5%2013.3807%2010.5%2012V5C10.5%204.72386%2010.7239%204.5%2011%204.5C11.2761%204.5%2011.5%204.72386%2011.5%205V12C11.5%2013.933%209.933%2015.5%208%2015.5C6.067%2015.5%204.5%2013.933%204.5%2012V3Z'%20fill='black'/%3e%3c/svg%3e) Android Studio Demo_T2mini.zip 
上一篇：7.云打印机蓝牙配网SDK（仅适用于使用了SDK_V1接口的旧设备）
下一篇：摄像头扫码Flutter SDK说明
