---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfzaeghjk480
---

# T1 自定义副显程序开发（仅适用于T1设备）
更新时间：2025-11-11 16:08:25
**【提示：在开发之前，请认真阅读文档中的内容，如有疑问可以联系商米技术支持咨询。】**
注：插入usb调试线，会导致主副屏断开。
### **关于T1双屏**
商米T1有两种双屏配置：
  1. 主屏14寸，副屏7寸。


注：7寸副屏仅支持浏览，不支持触控，不支持安装app。用户如需要修改副屏界面，请参考[内置副显程序对接文档](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfmreghjk568/)
  

![](https://cdn.sunmi.com/public/image/mgt-document/bad19554d47547a9a78cf7c02652dc18.png)
  1. 主屏14寸，副屏14寸，副屏可触摸。


  

![](https://cdn.sunmi.com/public/image/mgt-document/c2ff6ab3c6b74e358369bc21e776b854.png)
详细的硬件说明，请查看[官网介绍](http://www.sunmi.com/T1.php)，设备主副屏运行的都是SUNMI OS系统，相互通过商米封装好的接口实现通信。
注：不要把sunmi的有关代码混淆，混淆可能造成组件无法工作
#sunmi
-keep class com.luedongtech.kr3000.sunmi.** {*;}
-keep class sunmi.ds.**{*;}
-keep class com.sunmi.**{*;}
-keep class [sunmi.sc](http://sunmi.sc).**{*;}
-keep class com.google.gson.**{*;}
通常开发者的业务app运行在主屏，同时需要副屏显示清单信息、宣传图片，视频等内容，有时还需要实现简单的交互操作。开发者有两种方式使用T1副屏显示内容：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1. 使用T1副屏系统内置的显示程序。只要主屏app遵照商米的规范，向副显程序发特定格式的数据就可让副显程序显示内容(**开发成本低，推荐**)。



2. 开发者自己写副屏显示app（**开发成本高，自由度高**）。
  

```

虽然商米封装的副显程序能满足绝大部分的开发者对于副屏显示内容的需求，但是业务是多样的，商米始终秉持开放原则，支持开发者自己写副显程序。下面将对自己写副显程序做详细的说明。
  * 副显app的分发


我们规定，只要在您的主屏业务App的AndroidManifest.xml中添加一行配置(文档最后讲)，业务App在通过应用市场安装到主屏的同时，也会在副屏上安装，也就是说，您的主屏App同时也是副屏App，这里需要您在代码中进行首页显示内容的控制 。
### 进入正题
**自定义副显程序开发的核心问题是主副屏的通信，实现了主副屏通信后，您可以把副屏当成是普通的平板电脑开发相关的副屏app** 。这里有必要先了解一下双屏的通信方式，我们通过一张图片向您直观展示T1双屏的通信协议。
  

![](https://cdn.sunmi.com/public/image/mgt-document/f3eb182c1a9944a5964fb1dc47d51552.png)
T1的通信协议，从下到上依次包括：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1. Driver层，该层是最底层的通信协议，开发者无需关注。

2. Service层，该层是商米封装的一个通信服务，开发者也无需关注。

3. SDK层，该层是T1自带的副显程序使用的通信接口，**自定义的副显程序调用的也是这层的接口**。

4. APP层，指的是开发者自己的业务app和内置的副显程序。
  

```

  * 如果使用的是T1内置副显程序，数据通信方式如上图所示，开发者的主屏业务App只要调用商米的SDK按照商米规定好的数据格式向副显发数据，剩下的交给系统处理即可。
  * 如果您自己开发副显程序，就要自己处理数据的收发，内容显示等动作，如果您没有强烈的自定义需求，我们建议您使用T1自带的副显程序显示内容，可以节约很多的人力，物力成本。


### 开始开发
我们给开发者准备了一个双屏通信的Demo，您可以下载[副显程序开发资源](http://ota.cdn.sunmi.com/DOC/resource/re_cn/vice_screen_cn_1.zip)，参照Demo中的代码实现您自己的副显app，请按照下面的流程部署该Demo：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
1. 将资源文件中的HCService.apk安装到模拟主副屏通信的两台设备上，资源中包含两个HCService的apk，请先安装release版本，如果安装失败请尝试安装unsigned版本的apk。



2. 主副屏上分别运行这个程序，在第一台设备上输入自己的IP地址，勾选”作为主屏”，点击确定。



3. 在另一台设备上输入主屏机器的IP地址，点击确定(不要勾选‘作为主屏’)，可以看到屏幕中弹出toast提示“与主/副屏连接”，就代表主副屏能通信了，如果没有弹出提示，请在设置中的应用程序管理中找到HCService程序 (进入设置->应用->点击右上角的更多->点击显示系统进程，才能看到HCService)，强制停止主副屏上的HCService服务，再重复上面的步骤试试。



4. 将MyDSD项目导入到AndroidStudio中，如果报错了请检查您的编译环境的版本是否兼容demo，可以自己做相关的调整。**注：我们不建议您使用eclipse开发副显程序，目前没有提供相关的eclipse资源包**。



5. 这里请修改AndroidManifest.xml中的启动页面为MainScreenActivity，然后将项目部署到主屏设备中，运行。



6. 再修改启动页面为ViceScreenActivity，然后将项目部署到副屏设备中，运行。



7.在主屏上点击发送文字的按钮，查看副屏显示内容的变更，然后可以发送图片给副屏查看显示情况。
  

```

在这个Demo中包含了主副屏相互发送字符数据的方法，发送单个文件的方法和发送多个文件的方法。以下我们将基于Demo讲解主副屏通信的开发流程：
  1. 在build.gradle中添加以下依赖：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
dependencies {
    compile files('libs/commons-codec-1.9.jar')
    provided files('libs/framework.jar')
    compile 'com.android.support:appcompat-v7:22.2.1'
    compile 'com.sunmi:DS_Lib:latest.release'
    compile 'com.google.code.gson:gson:2.6.2'
}
  

```

需要添加的权限：（强调：Sunmi OS是基于Android6.0定制，Android6.0+ 要求部分敏感权限需要动态申请）
还有一个广播接收器，注意，这个是固定的写法，不要做修改哦。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
"sunmi.ds.MsgReceiver">
    
        "com.sunmi.hcservice" />
        "com.sunmi.hcservice.status" />
    
  

```

  1. 初始化，您可以在Application中初始化SDK的代码，也可以在任何您想要的地方初始化：


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
private void initSDK() {
    mDSKernel = DSKernel.newInstance();
    mDSKernel.init(this, mConnCallback);
}
  

```

以上DSKernel类是SDK中的核心类，几乎所有双屏通信相关的方法都在这个类中。在初始化后需要添加两个监听类：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//添加主副屏连接状态的监听
mDSKernel.addConnCallback(new IConnectionCallback(){
                        @Override
                        public void onConnected(ConnState arg0) {
                        //连接上了回调
                        }

                        @Override
                        public void onDisConnect() {
                        //断开连接的回调
                        }
                        
                });

//添加接收到数据的回调                
mDSKernel.addReceiveCallback(new IReceiveCallback(){

                        @Override
                        public void onReceiveCMD(DSData arg0) {
                          //收到命令      
                        }

                        @Override
                        public void onReceiveData(DSData arg0) {
                           //收到数据
                        }

                        @Override
                        public void onReceiveFile(DSFile arg0) {
                           //收到单个文件
                        }

                        @Override
                        public void onReceiveFiles(DSFiles arg0) {
                           //收到多个文件
                        }
             
                        
                });   
  

```

以上我们完成了初始化，通信当然会分信息的发送和接收了，主副屏可以相互发送接收数据。可以看到在MyDSD里面，MainScreenActivity中包含很多发数据的方法，在ViewScreenActiivty中包含很多接收数据的方法。
**所有的方法可以在DSKernel类中查看到，详情可以查看**[**T1双屏通信接口文档**](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfzceghjk502/) ，以下我们将讲解部分接口的使用：
1.发送字符数据，副屏接收显示
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DataPacket packet = UPacketFactory.buildShowText(DSKernel.getDSDPackageName(), "成功接收字符数据！",null);
mDSKernel.sendData(packet);//参数DataPacket是双屏通信数据打包类，请参照Demo生成该数据
  

```

接收的方法是在IReceiveCallback接口的onReceiveData(DSData data)中：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
@Override
public void onReceiveData(DSData data) {
    Log.d(TAG, "获取到数据：" + data.data);//将接收到的数据打印出来
}
  

```

2.发送指令的方法：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
String json = UPacketFactory.createJson(DataModel.QRCODE, "");
//三个参数：1.接收指令的副屏app的包名;2.要发送的指令，这里可以参照Demo;3.发送结果回调
mDSKernel.sendCMD(DSKernel.getDSDPackageName(), json.toString(), l,null);
  

```

接收指令的方法：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
@Override
public void onReceiveCMD(DSData cmd) {
     Log.d(TAG, "获取到数据：" + cmd.data);//将接收到的数据打印出来
}
  

```

指令的使用场景举例：图片视频等的显示需要先将文件发送到副屏，副屏会返回一个文件的id给主屏，然后向副屏发送指令显示指定id的图片或视频。
3.发送单个文件的方法：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//该方法有三个参数：1.接收端app的包名;2.文件在本地存储中的路径;3.发送结果回调
mDSKernel.sendFile(PACKAGE_OF_VICE, Environment.getExternalStorageDirectory().toString()+"/"+videoInAsserts,
      new ISendCallback() {
         public void onSendSuccess(long l) {
            Log.i("sunmi", "这里表示发送成功" + l);
            showVideo(l);
         }

      
         public void onSendFail(int i, String s) {
            //tv_show_id.setText("fail:" + s);
            Log.i("sunmi", "---:" + s);
         }

         public void onSendProcess(long l, long l1) {
            //发送进度的方法
         }
      });
  

```

接收单个文件的方法，IReceiveCallback接口的onReceiveFile(DSFile file) 中通过调用FilesManager.saveFile(DSFile file)将文件保存在本地：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
@Override
public void onReceiveFile(DSFile file) {
   Log.i(TAG,"接收到的单个文件的路径:"+file.path);
   Log.i(TAG,"接收到的单个文件的taskId:"+file.taskId);
    mFilesManager.saveFile(file);
}
  

```

4.发送多个文件的方法
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
mDSKernel.sendFiles(PACKAGE_OF_VICE, json.toString(), paths, new ISendFilesCallback(){
   @Override
   public void onAllSendSuccess(long l) {
     //所有文件发送成功
   }
   @Override
   public void onSendFaile(int arg0, String arg1) {}
   @Override
   public void onSendFileFaile(String arg0, int arg1, String arg2) {}
   @Override
   public void onSendProcess(String arg0, long arg1, long arg2) {}
   @Override
   public void onSendSuccess(String arg0, long arg1) {
   }
   
});
  

```

多个文件的接收方法：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
@Override
public void onReceiveFiles(DSFiles dsFiles) {
            mFilesManager.saveFiles(dsFiles);
        }
  

```

5.启动指定应用的方法
参照Demo生成数据，向副屏发指令，该代码需要DSD程序的支持，可以在下载的资源文件中找到该apk，安装在副屏。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DataPacket dsPacket   = UPacketFactory.buildOpenApp(PACKAGE_OF_VICE, null);
//PACKAGE_OF_VICE为要启动的副屏指定应用的包名，如：com.sunmi.mydsd
mDSKernel.sendCMD(dsPacket);
  

```

以上介绍了一些基本接口的使用，更多的接口请参看DSKernel中的代码注释。
### 应用的发布
主副屏的应用都开发好了，当然是通过商米应用市场将该应用大规模分发出去了，上面我们讲了，只要在应用的AndroidManifest.xml中添加一行配置，该应用通过应用市场在主屏安装的时候，只要双屏通信畅通，就会在副屏上同时安装该应用(**必须通过应用市场安装才能生效**)。主屏和副屏程序是写在一个应用里的，所以需要您在入口类中添加相关的判断逻辑，这就是运行Demo时您需要修改启动页的原因，具体判断主副屏的代码可以参考Demo。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```



"sunmi_dual" android:value="open"/>
  

```

上一篇：T1副屏内置副显程序对接（仅适用于T1设备）
下一篇：T1双屏通信接口文档（仅适用于T1设备）
