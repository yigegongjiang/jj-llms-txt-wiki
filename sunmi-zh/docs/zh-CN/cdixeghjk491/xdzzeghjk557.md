---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xdzzeghjk557
---

# 打印文件接口
更新时间：2025-09-24 16:51:30
## 一、功能介绍
* * *
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public FileApi fileApi()
  

```

文件渲染接口集合，文件渲染接口设计用于直接打印文件类内容，如PDF文件、JPEG文件等
> void printFile(String path, PrintResult listener)
> void printFile(String path, FileStyle style, PrintResult listener)
## 二、使用限制
文件接口支持如下型号:
奔图系列：P3017D、CP1100、BP5126DN
汉印系列：U100
  

![](https://cdn.sunmi.com/public/image/mgt-document/6b07081aa361487298734776ba81841d.png)
  

![](https://cdn.sunmi.com/public/image/mgt-document/2ae5208e9b4f4ebba94389e79484077c.png)
## 三、接口说明
### 1. 打印指定文件
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printFile(String path, PrintResult listener) 
  

```

指定文件保存路径，支持网络URL，打印一份内容
**String path 说明**
指定打印文件的绝对路径
同时也支持传入网络URL和本地URI文件
**FileStyle 说明**  
| 可用方法  | 方法说明  | 参数说明  | 默认值  |  
| --- | --- | --- | --- |  
| setFileCopies  | 指定文件打印份数  | 份数  | 1  |  
| setFileDuplex  | 指定单双面打印  | 需要打印机硬件支持  | 单面打印  |  
| setFileRotate  | 指定打印的方向  | 打印内容的旋转方向  | 0度  |  
| setFileCollate  | 设置是否逐份打印  | 如果是pdf文件且要打印文件份数超过一张时逐份打印  | 逐份打印  |  
| setFileStart  | 设置文件打印起始页数  | 如果文件包含多页时的页号  | 0 首页  |  
| setFileEnd  | 设置文件打印最终页数  | 如果文件包含多页时的页号  | 0 末页  |  
### 2. 打印指定文件并接收回调
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
void printFile(String path, FileStyle style, PrintResult listener) 
  

```

指定文件保存路径，支持网络URL，打印一份内容，并监听打印结果
**PrintResult 说明**
resultCode 打印成功将返回0 打印失败将返回非0
message 失败时附加信息
  * **示例**


bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
FileStyle fileStyle = FileStyle.getStyle()
                              .setFileCopies(4)
                              .setFileStart(1)
                              .setFileEnd(4)
                              .setFileCollate(true)
                              .setFileRoatate(Rotate.ROTATE_0)
                              .setFileDuplex(FileDuplex.SINGLE);
printer.fileApi().printFile("文件绝对路径", fileStyle, new PrintResult() {
            @Override
            public void onResult(int resultCode, final String message) throws RemoteException {

            }
        })
  

```

上一篇：打印标签小票接口
下一篇：打印机查询接口
