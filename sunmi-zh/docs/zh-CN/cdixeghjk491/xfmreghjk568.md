---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfmreghjk568
---

# T1副屏内置副显程序对接（仅适用于T1设备）
更新时间：2025-11-11 16:05:20
T1 接入副屏客显程序说明
## 更新内容
2017-12-28 （适用版本：主屏（T1-host）： V1.14.22、7寸副屏（sub7）： V1.11.3、14寸副屏（sub14）： V1.9.11）
  * 新增14寸副屏支持轮播视频；
  * 新增14寸副屏支持视频续播；
  * 新增7寸副屏支持幻灯片；
  * 新增7寸副屏支持单个视频；
  * 新增7寸副屏支持视频轮播；
  * 新增7寸副屏支持视频续播；
  * 新增判断副屏尺寸的方法；
  * 新增管理副屏缓存文件的方法；
  * DS_Lib库更新至1.0.16；


2018-01-15
  * 更新下载资源；


  

## 一、简介
T1双屏机器有三种组合：主机、主机+7寸副屏、主机+14寸
### 初始化配置
副屏。主副屏都是运行SUNMI OS定制系统，通过商米已封装好的接口实现通信。
主屏主要用来运行业务APP，例如：收银系统。副屏主要面向顾客显示结算、广告内容。开发者有两种方式实现副屏显示：
  * T1副屏系统内置了默认副屏显示APP，内置多个常用模板。开发者仅需参照文中模板的实现代码，即可实现副屏内容显示；
  * 开发自己副屏显示APP，需要自行处理数据的收发、内容显示等动作；


如果您没有强烈的自定义需求，商米建议您使用默认副屏显示APP，可以节省很多的研发成本。
本文将对如何接入内置副屏客显程序进行说明。
## 二、如何调试应用
由于主副屏是通过USB通信的，**当主/副屏插入USB线时，主副屏的连接会断开，导致无法调试设备。**
商米提供的解决方案是：将开发电脑与将要调试的T1主机处于同一个局域网络环境下，通过网络对设备进行调试。
如何调试：
1、开启USB调试，及调试权限；
[操作说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xdrzeghjk557/)
2、通过网络对设备进行ADB调试；
[操作说明](https://developer.sunmi.com/docs/zh-CN/cdixeghjk491/xfmmeghjk546/)
三、如何接入副屏客显程序
（强调：Sunmi OS是基于Android6.0定制，Android6.0+ 要求部分敏感权限需要动态申请）
商米默认副屏APP提供了多个显示模板，开发者仅需要在自己的主屏业务APP实现控制副屏输出的代码，向默认副屏APP发送正确格式的数据即可实现副屏显示。
### 1、初始化配置
对接7寸或14寸默认副屏APP的初始化流程一样。
步骤1：
[ ](https://gitee.com/sunmios/SunmiDemoApp.git)[下载DemoApp资源文件。](http://ota.cdn.sunmi.com/DOC/resource/re_cn/MainScreenApp.zip)
步骤2：
参照DemoApp源码，直接在Android Studio的app module下的build.gradle文件中声明以下代码：   
  

bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 dependencies {
    compile 'com.sunmi:DS_Lib:1.0.15'   //商米提供的lib库，包含已封装好的接口
    compile 'compile 'com.alibaba:fastjson:1.1.67.android''  //fastjson任意版本
}
  

```

步骤3：
在清单文件AndroidMainfest.xml的节点下配置以下声明：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```

  ....
  "sunmi.ds.MsgReceiver">  //接收数据的广播
     
         "com.sunmi.hcservice">
         "com.sunmi.hcservice.status">
      
  
  

```

步骤4：
在适当的位置初始化SDK代码，可以参考DSC_Demo中的实现代码。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DSKernel mDSKernel =DSKernel.newInstance();
mDSKernel.init(context, mConnCallback);    //绑定服务的回调
mDSKernel.addReceiveCallback(mReceiveCallback);  //双屏通信接收数据回调
  

```

接下来可以参考DemoApp源码，实现套用默认副屏APP内置模板的内容显示。
**混淆ds_lib** :
-keep interface sunmi.ds.** **{ *; }**
**-keep class sunmi.ds.** **{ *; }
除此之外，还需要混淆greendao以及fastjson，具体混淆规则以greendao和fastjson官方提供为准
### 2、副屏客显程序内置模板
7寸与14寸副屏支持的模板对比：  
| 模板  | 7寸  | 14寸  |  
| --- | --- | --- |  
| 文字  | √  | √  |  
| 文字+二维码  | √  | √  |  
| 单张图片  | √  | √  |  
| 幻灯片  | √  | √  |  
| 单个视频  | √  | √  |  
| 视频轮播  | √  | √  |  
| 清单  | ×  | √  |  
| 清单+单张图片  | ×  | √  |  
| 清单+幻灯片  | ×  | √  |  
| 清单+单个视频  | ×  | √  |  
| 清单+视频轮播  | ×  | √  |  
#### 2.1、两行文字
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/20953341532097358.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
JSONObject json = new JSONObject();
json.put("title", title);//title为上面一行的标题内容
json.put("content", content);//content为下面一行的内容
String jsonStr = json.toString();
//构建DataPacket类
DataPacket packet = UPacketFactory.buildShowText(DSKernel.getDSDPackageName(), jsonStr, callback);//第一个参数是接收数据的副显应用的包名，这里参照Demo就可以,第二个参数是要显示的内容字符串，第三个参数为结果回调。

mDSKernel.sendData(packet);//调用sendData方法发送文本
  

```

#### 2.2、二维码+文字
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/022839760684922217.jpeg)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
JSONObject json = new JSONObject();
try {
    json.put("title", "微信支付");
    json.put("content", "10.00");
} catch (JSONException e) {
    e.printStackTrace();
}

mDSKernel.sendFile(DSKernel.getDSDPackageName(), json.toString(), Environment.getExternalStorageDirectory().getPath() + "/qrcode.png", new ISendCallback() {
    @Override
    public void onSendSuccess(long l) {
        //显示图片
        try {
            JSONObject json = new JSONObject();
            json.put("dataModel", "QRCODE");
            json.put("data", "default");
            mDSKernel.sendCMD(SF.DSD_PACKNAME, json.toString(), l, null);
        } catch (JSONException e) {
            e.printStackTrace();
        }
    }

    @Override
    public void onSendFail(int i, String s) {      
    }

    @Override
    public void onSendProcess(long l, long l1) {
    }
});
  

```

2.3、单张图片
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/5969997993525171.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
mDSKernel.sendFile(DSKernel.getDSDPackageName(), Environment.getExternalStorageDirectory().getPath() + "/img_01.png", new ISendCallback() {

        @Override
        public void onSendSuccess(long taskId) {
            showPicture(taskId);
        }

        @Override
        public void onSendFail(int errorId, String errorInfo) {
        }

        @Override
        public void onSendProcess(long totle, long sended) {
        }
    });
}

/**
 * 展示单张图片
 *
 * @param taskId
 */

private void showPicture(long taskId) {
    //显示图片
    try {
        JSONObject json = new JSONObject();
        json.put("dataModel", "SHOW_IMG_WELCOME");
        json.put("data", "default");
        mDSKernel.sendCMD(DSKernel.getDSDPackageName(), json.toString(), taskId, null);
    } catch (JSONException e) {
        e.printStackTrace();
    }
}
  

```

#### 2.4、幻灯片
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/6801029590257146.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
JSONObject json = new JSONObject();
json.put("rotation_time",5000); //幻灯片的切换时间，用毫秒计算，如果不传默认是10000毫秒
List pathList = new ArrayList<>();
pathList.add("/sdcard/img1.png");
pathList.add("/sdcard/img2.png");
...
mDSKernel.sendFiles(DSKernel.getDSDPackageName(), json.toString(), 
pathList, new ISendFilesCallback() {
    public void onAllSendSuccess(long fileId) {
        show(fileId);
    }
    public void onSendSuccess(final String s,final long l) {}
    public void onSendFaile(int errorId, String errorInfo) {}
    public void onSendFileFaile(String path, int errorId, String errorInfo){}
    public void onSendProcess(String path, long total, long sended) {}
});

private void show(long fileId) {
    String json = UPacketFactory.createJson(DataModel.IMAGES,"");
    mDSKernel.sendCMD(DSKernel.getDSDPackageName(),json,fileId,null);
}
  

```

#### 2.5、单个视频
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/2632634249371819.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
mDSKernel.sendFile(DSKernel.getDSDPackageName(), Environment.getExternalStorageDirectory().getPath() + "/video_01.mp4", new sunmi.ds.callback.ISendCallback() {
        @Override
        public void onSendSuccess(long l) {         
            playvideo(l);         
        }

        @Override
        public void onSendFail(int i, String s) {
            Log.d("highsixty", "发送单个文件视频文件失败 ------------>" + s);          
        }

        @Override
        public void onSendProcess(final long l, final long l1) {
        }
    });
}

private void playvideo(long taskID) {  
    String json = UPacketFactory.createJson(DataModel.VIDEO, "true"); //"true"视频续播；false视频从头播放
    mDSKernel.sendCMD(DSKernel.getDSDPackageName(), json, taskID, null);
}
  

```

#### 2.6、视频轮播
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/946208968630099.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
/**
 * 发送多视频
 */

private void sendVideos() {
    //请对文件是否存在做判断
    List files = new ArrayList<>();
    files.add(Environment.getExternalStorageDirectory().getPath() + "/video_01.mp4");
    files.add(Environment.getExternalStorageDirectory().getPath() + "/video_02.mp4");
    files.add(Environment.getExternalStorageDirectory().getPath() + "/video_03.mp4");
    mDSKernel.sendFiles(DSKernel.getDSDPackageName(), "", files, new ISendFilesCallback() {

        @Override
        public void onAllSendSuccess(long fileid) {
            playvideos(fileid);
        }

        @Override
        public void onSendSuccess(String path, long taskId) {           
        }

        @Override
        public void onSendFaile(int errorId, String errorInfo) {           
        }

        @Override
        public void onSendFileFaile(String path, int errorId, String errorInfo) {            
        }

        @Override
        public void onSendProcess(String path, long totle, long sended) {
        }
    });
}

private void playvideos(long taskID) {
    Log.d(TAG, "playvideos: ------------>" + taskID);
    String json = UPacketFactory.createJson(DataModel.VIDEOS, "true"); //第二个参数true：视频续播 false：视频从头播放
    mDSKernel.sendCMD(DSKernel.getDSDPackageName(), json, taskID, null);
}
  

```

#### 2.7、清单
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/5354788719931796.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
        "title": "三米奶茶店欢迎您", 
        "head": {
            "param1": "序列号",
            "param2": "商品名",
            "param3": "单价"，
            "param4": "数量"，
            "param5": "小结"
        },
        "list": [
            {
              "param1": "1",
              "param2": "华夫饼",
              "param3": "10.00"，
              "param4": "1"，
             "param5":"10.00"
            },
            {
              "param1": "1",
              "param2": "吞拿鱼华夫饼",
              "param3": "12.00"，
              "param4": "1"，
             "param5":"12.00"
            }
            ... ...//这里是相同格式的数据
        ],
        "KVPList": [
            {
                "name": "收款",
                "value": "￥40.00"
            },
            {
                "name": "优惠",
                "value": "￥3.00"
            }，
           {
                "name": "找零",
                "value": "￥3.00"
            }，
           {
                "name": "实收",
                "value": "￥37.00"
            }
        ]
}

/**
*JSON数据格式说明：
*1:title字段为标题
*2:head字段是表头字段
*3:list是商品列表，表头和表的字段数量要一致 
*4:KVPList为结算键值对列表
*/

/**
*规则:
*当显示图文混合时：head的params赋值个数最小1个最大4个;list中每个元素的params赋值个数最*小1个最大4个；KVPList的size
*最小1个最大4个。
*/ 

/**
*receiverPackageName 接收数据的副屏显示app的包名DataType.DATA 
*DataType.DATA 
*DataModel.TEXT 
*jsonStr 显示的数据内容,具体格式参见下面的框
*callback 回调
*/
DataPacket pack = buildPack(receiverPackageName, DataType.DATA, DataModel.TEXT, jsonStr, callback);
mDSKernel.sendData(pack);
  

```

#### 2.8、清单+单张图片
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/8533668254245868.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
 {
        "title": "三米奶茶店欢迎您", 
        "head": {
            "param1": "序列号",
            "param2": "商品名",
            "param3": "单价"，
            "param4": "数量"，
            "param5": "小结"
        },
        "list": [
            {
              "param1": "1",
              "param2": "华夫饼",
              "param3": "10.00"，
              "param4": "1"，
             "param5":"10.00"
            },
            {
              "param1": "1",
              "param2": "吞拿鱼华夫饼",
              "param3": "12.00"，
              "param4": "1"，
             "param5":"12.00"
            }
            ... ...//这里是相同格式的数据
        ],
        "KVPList": [
            {
                "name": "收款",
                "value": "￥40.00"
            },
            {
                "name": "优惠",
                "value": "￥3.00"
            }，
           {
                "name": "找零",
                "value": "￥3.00"
            }，
           {
                "name": "实收",
                "value": "￥37.00"
            }
        ]
}

/**
*JSON数据格式说明：
*1:title字段为标题
*2:head字段是表头字段
*3:list是商品列表，表头和表的字段数量要一致 
*4:KVPList为结算键值对列表
*/

/**
*规则:
*当显示图文混合时：head的params赋值个数最小1个最大4个;list中每个元素的params赋值个数最*小1个最大4个；KVPList的size
*最小1个最大4个。
*/ 

String filePath = "xxx/img.png";//显示的图片路径
mDSKernel.sendFile(DSKernel.getDSDPackageName(), filePath, new ISendCallback() {
    public void onSendSuccess(long fileId) {
        show(fileId, jsonStr);//图片发送成功，显示文字内容
    } 
    public void onSendFail(int errorId, String errorInfo) {}
    public void onSendProcess(long total, long sended) {}
});

void show(long fileId, String jsonStr){
    jsonStr = UPacketFactory.createJson(DataModel.SHOW_IMG_LIST, jsonStr);
    //第一个参数DataModel.SHOW_IMG_LIST为显示布局模式，jsonStr为要显示的内容字符
    mDSKernel.sendCMD(DSKernel.getDSDPackageName(), jsonStr, fileId,null);
}
  

```

2.9、清单+幻灯片
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/031263079562512175.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
{
        "title": "三米奶茶店欢迎您", 
        "head": {
            "param1": "序列号",
            "param2": "商品名",
            "param3": "单价"，
            "param4": "数量"，
            "param5": "小结"
        },
        "list": [
            {
              "param1": "1",
              "param2": "华夫饼",
              "param3": "10.00"，
              "param4": "1"，
             "param5":"10.00"
            },
            {
              "param1": "1",
              "param2": "吞拿鱼华夫饼",
              "param3": "12.00"，
              "param4": "1"，
             "param5":"12.00"
            }
            ... ...//这里是相同格式的数据
        ],
        "KVPList": [
            {
                "name": "收款",
                "value": "￥40.00"
            },
            {
                "name": "优惠",
                "value": "￥3.00"
            }，
           {
                "name": "找零",
                "value": "￥3.00"
            }，
           {
                "name": "实收",
                "value": "￥37.00"
            }
        ]
}

/**
*JSON数据格式说明：
*1:title字段为标题
*2:head字段是表头字段
*3:list是商品列表，表头和表的字段数量要一致 
*4:KVPList为结算键值对列表
*/

/**
*规则:
*当显示图文混合时：head的params赋值个数最小1个最大4个;list中每个元素的params赋值个数最*小1个最大4个；KVPList的size
*最小1个最大4个。
*/ 

List paths = new ArrayList<>();
paths.add(Environment.getExternalStorageDirectory().getPath() + "/sunmi1.png");
paths.add(Environment.getExternalStorageDirectory().getPath() + "/sunmi2.png");
paths.add(Environment.getExternalStorageDirectory().getPath() + "/sunmi3.png");
paths.add(Environment.getExternalStorageDirectory().getPath() + "/sunmi4.png");

mDSKernel.sendFiles(DSKernel.getDSDPackageName(), "", paths, new ISendFilesCallback() {
    @Override
    public void onAllSendSuccess(long fileId) {               
      sendImgsListCMD(fileId,json);                           
    }

    @Override
    public void onSendSuccess(String path, long taskId) {}
    @Override
    public void onSendFaile(int errorId, String errorInfo) {}
    @Override
    public void onSendFileFaile(String path, int errorId, String errorInfo) {}
    @Override
    public void onSendProcess(String path, long totle, long sended) {}
    });
    
    void sendImgsListCMD(long fileId, String jsonStr){   
      jsonStr= UPacketFactory.createJson(DataModel.SHOW_IMGS_LIST, json);
        mDSKernel.sendCMD(DSKernel.getDSDPackageName(), jsonStr, fileId,null);
    }
  

```

2.10、清单+单个视频
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/39652703343575424.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
mDSKernel.sendFile(DSKernel.getDSDPackageName(), Environment.getExternalStorageDirectory().getPath() + "/video_01.mp4", new sunmi.ds.callback.ISendCallback() {
    @Override
    public void onSendSuccess(long l) {     
        playVideoMenu(l);      
    }

    @Override
    public void onSendFail(int i, String s) {
        Log.d("highsixty", "发送单个文件视频和清单文件失败 ------------>" + s);      
    }

    @Override
    public void onSendProcess(final long l, final long l1) {
    }
});

private void playVideoMenu(long fileId) {
    try {
        JSONObject data = new JSONObject();
        data.put("title", "商米奶茶店收银");
        JSONObject head = new JSONObject();
        head.put("param1", "序号");
        head.put("param2", "商品名");
        head.put("param3", "单价");
        data.put("head", head);
        data.put("flag", "true");    //设置为true，则视频将接着之前的进度播放。设置为false，则视频将从头播放。
        JSONArray list = new JSONArray();
        for (int i = 1; i < 11; i++) {
            JSONObject listItem = new JSONObject();
            listItem.put("param1", "" + i);
            listItem.put("param2", products.get(i - 1));
            listItem.put("param3", prices.get(i - 1));
            list.put(listItem);
        }

        data.put("list", list);
        JSONArray KVPList = new JSONArray();
        JSONObject KVPListOne = new JSONObject();
        KVPListOne.put("name", "总计 ");
        KVPListOne.put("value", "132.00");
        JSONObject KVPListTwo = new JSONObject();
        KVPListTwo.put("name", "优惠 ");
        KVPListTwo.put("value", "12.00");
        JSONObject KVPListThree = new JSONObject();
        KVPListThree.put("name", "数量 ");
        KVPListThree.put("value", "10");
        JSONObject KVPListFour = new JSONObject();
        KVPListFour.put("name", "应收 ");
        KVPListFour.put("value", "120.00");
        KVPList.put(0, KVPListOne);
        KVPList.put(1, KVPListTwo);
        KVPList.put(2, KVPListThree);
        KVPList.put(3, KVPListFour);
        data.put("KVPList", KVPList);
        Log.d("HHHH", "onClick: ---------->" + data.toString());
        String json = UPacketFactory.createJson(DataModel.SHOW_VIDEO_LIST, data.toString());
        mDSKernel.sendCMD(DSKernel.getDSDPackageName(), json, fileId, null);
    } catch (Exception e) {
        e.printStackTrace();
    }
}
  

```

2.11、清单+视频轮播
  

![](https://file.cdn.sunmi.com/SUNMIDOCS/9531540227164912.png)
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
/**
 * 左视频右清单文件
 */
private void sendMenuVideos() {
    Log.d(TAG, "sendMenuVideos: ------------->");
    List files = new ArrayList<>();
    files.add(Environment.getExternalStorageDirectory().getPath() + "/video_01.mp4");
    files.add(Environment.getExternalStorageDirectory().getPath() + "/video_02.mp4");
    files.add(Environment.getExternalStorageDirectory().getPath() + "/video_03.mp4");

    mDSKernel.sendFiles(DSKernel.getDSDPackageName(), "", files, new ISendFilesCallback() {

        @Override
        public void onAllSendSuccess(long fileid) {          
            playMenuVideos(fileid);
        }

        @Override
        public void onSendSuccess(String path, long taskId) {            
        }

        @Override
        public void onSendFaile(int errorId, String errorInfo) {           
        }

        @Override
        public void onSendFileFaile(String path, int errorId, String errorInfo) {           
        }

        @Override
        public void onSendProcess(String path, long totle, long sended) {
        }
    });
}

private void playMenuVideos(long taskID) {    
    try {
        JSONObject data = new JSONObject();
        data.put("title", "商米奶茶店收银");
        JSONObject head = new JSONObject();
        head.put("param1", "序号");
        head.put("param2", "商品名");
        head.put("param3", "单价");
        data.put("head", head);
        data.put("flag", "true"); //“true"视频续播；false 从头播放
        JSONArray list = new JSONArray();
        for (int i = 1; i < 11; i++) {
            JSONObject listItem = new JSONObject();
            listItem.put("param1", "" + i);
            listItem.put("param2", products.get(i - 1));
            listItem.put("param3", prices.get(i - 1));
            list.put(listItem);
        }
        data.put("list", list);
        JSONArray KVPList = new JSONArray();
        JSONObject KVPListOne = new JSONObject();
        KVPListOne.put("name", "总计 ");
        KVPListOne.put("value", "132.00");
        JSONObject KVPListTwo = new JSONObject();
        KVPListTwo.put("name", "优惠 ");
        KVPListTwo.put("value", "12.00");
        JSONObject KVPListThree = new JSONObject();
        KVPListThree.put("name", "数量 ");
        KVPListThree.put("value", "10");
        JSONObject KVPListFour = new JSONObject();
        KVPListFour.put("name", "应收 ");
        KVPListFour.put("value", "120.00");
        KVPList.put(0, KVPListOne);
        KVPList.put(1, KVPListTwo);
        KVPList.put(2, KVPListThree);
        KVPList.put(3, KVPListFour);
        data.put("KVPList", KVPList);
        Log.d("HHHH", "onClick: ---------->" + data.toString());
        String json = UPacketFactory.createJson(DataModel.MENUVIDEOS, data.toString());
        mDSKernel.sendCMD(DSKernel.getDSDPackageName(), json, taskID, null);
    } catch (JSONException e) {
        e.printStackTrace();
    }
}
  

```

### 3、判断副屏的尺寸
主屏业务应用可通过获取副屏Model值判断副屏尺寸，实现根据副屏尺寸输出相应的显示内容的业务逻辑。
#### 3.1、从主屏获取副屏Model值
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
String subModel = Settings.Global.getString(getContentResolver(), "sunmi_sub_model")

//返回值：“t1sub14”对应14寸副屏，“t1sub7”对应7寸副屏。返回其它值表示获取失败。
  

```

#### 3.2、从副屏获取副屏Model值
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//从副屏获取副屏model
case R.id.btn_get_sub_model:
JSONObject jsonObject2 = new JSONObject();
try {
  jsonObject2.put("dataModel", "GET_MODEL");
  jsonObject2.put("data", "");
} catch (JSONException e) {
  e.printStackTrace();
}

DataPacket p2 = new DataPacket.Builder(DSData.DataType.CMD).recPackName(SF.SUNMI_DSD_PACKNAME).data(jsonObject2.toString())
.addCallback(new ISendCallback() {
  @Override
  public void onSendSuccess(long taskId) {
  }    
  
  @Override                         
  public void onSendFail(int errorId, String errorInfo) {           
  }     
                       
  @Override
  public void onSendProcess(long totle, long sended) {              
  }
}).build();

mDSKernel.sendQuery(p2, new QueryCallback() {                 
  @Override                 
  public void onReceiveData(final DSData data) {                      Log.d("highsixty", "onReceiveData: ------------>" + data.data); runOnUiThread(new Runnable() {                         
      @Override                         
      public void run() {                             
        Toast.makeText(MainActivity.this, "从副屏获取副屏model-->" + data.data, Toast.LENGTH_LONG).show();                         
      }
    });
  }
});             
break;
//返回值：“t1sub14”对应14寸副屏，“t1sub7”对应7寸副屏。返回其它值表示获取失败。
  

```

### 4、清除副屏缓存
#### 4.1、查询副屏指定缓存目录大小
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
JSONObject jsonObject = new JSONObject();
try {
    jsonObject.put("dataModel", "GETVICECACHEFILESIZE");
    jsonObject.put("data", Environment.getExternalStorageDirectory().getAbsolutePath() + "/HCService/" + getPackageName().replace(".", "_"));
} catch (JSONException e) {
    e.printStackTrace();
}

DataPacket packet = new DataPacket.Builder(DSData.DataType.CMD).recPackName(SF.SUNMI_DSD_PACKNAME).data(jsonObject.toString())
        .addCallback(null).build();
mDSKernel.sendQuery(packet, new QueryCallback() {
    @Override
    public void onReceiveData(final DSData data) {
        Log.d("highsixty", "onReceiveData: ------------>" + data.data);
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                Toast.makeText(MainActivity.this, "副屏缓存文件大小字节数为" + data.data, Toast.LENGTH_LONG).show();
            }
        });
    }
});
  

```

#### 4.2、清除副屏指定缓存目录
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
DataPacket packet2 = UPacketFactory.remove_folders(DSKernel.getDSDPackageName(), Environment.getExternalStorageDirectory().getAbsolutePath() + "/HCService/" + getPackageName().replace(".", "_"), new ISendCallback() {
    @Override
    public void onSendSuccess(long taskId) {
        showToast("清除缓存文件成功");
    }

    @Override
    public void onSendFail(int errorId, String errorInfo) {
        showToast("清除缓存文件失败");
    }

    @Override
    public void onSendProcess(long totle, long sended) {
    }
});

mDSKernel.sendCMD(packet2);
  

```

#### 4.3、查询指定缓存文件是否存在
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//说明：该方法检查fileId对应的文件在副屏是否存在，
第一个参数为之前发送文件时副屏返回的文件id，该id是所有的文件唯一的，第二个参数为结果回调
mDSKernel.checkFileExist(fileId, new ICheckFileCallback(){
  @Override
  public void onCheckFail() {}
 
  @Override
  public void onResult(boolean arg0) {
     //返回值为true是存在
     //返回值false是不存在  
      }                                    
});
  

```

#### 4.4、清除副屏指定缓存文件
实现代码：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
mDSKernel.deleteFileExist(FileID, new ICheckFileCallback() {
    @Override
    public void onCheckFail() {
        Log.d(TAG, "onCheckFail: ----------->");
    }

    @Override
    public void onResult(boolean exist) {
        Log.d(TAG, "onResult: ---------->" + exist);
    }
});
  

```

### 5、图片/视频显示规则
图片/视频的缩放规则：
  * 图片/视频默认居中等比例缩放，至少有两条边与显示容器边界接触；


图片/视频的建议分辨率：
  * 7寸：
  * 全屏显示图片：1024*600
  * 全屏显示视频：1024*600
  * 14寸：
  * 全屏显示图片/视频：1920*1080
  * 图片/视频+清单：1186*1080


建议文件大小：
  * 图片：≤1MB/张
  * 视频：≤5MB/个


建议幻灯片文件数量：
  * 图片：≤10张


建议轮播视频文件数量：
  * 视频：≤10个


注：因文件传送至副屏需要时间，显示文件时可能会有延时。故尽量在空闲时先将图片和视频文件缓存至副屏，当需要在副屏显示时，根据文件ID调用。
上一篇：调试说明（仅适用于T1设备）
下一篇：T1 自定义副显程序开发（仅适用于T1设备）
