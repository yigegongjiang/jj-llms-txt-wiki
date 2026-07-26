---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/iareghjk568
---

# 商米Android设备间通信连接服务 (ECR) SDK 开发指南
更新时间：2026-06-24 16:38:07
# **【背景说明】**
在Android生态中，当需要在两台设备（如手持设备与台式终端、或手持设备与手持设备、或台式终端与台式终端）之间建立稳定、可靠的通信连接以交换数据时，开发者通常需要面对复杂且差异化的底层硬件接口（如蓝牙、Wi-Fi、USB、串口等）。这意味着需要编写大量重复、繁琐且易于出错的底层代码，极大地增加了开发难度、时间成本和维护负担。
为了彻底解决这一问题，商米科技基于对自身硬件设备的深度理解，开发了**ECR（设备间通信连接）服务SDK** 。该SDK旨在为开发者提供一个**统一、高效、易用** 的通信框架，将复杂的连接建立、数据收发和连接管理过程封装成简洁的API。通过本SDK，开发者无需关注底层通信协议的细节，即可快速实现多种通信模式（包括蓝牙、Wi-Fi、商米虚拟串口/VSP、USB、RS232串口及iBeacon广播）下的稳定数据交互，从而显著降低开发门槛，提升开发效率与代码可靠性。
## 1.修订说明  
| 版本  | 修订日期  | 修订内容  | 修订人  |  
| --- | --- | --- | --- |  
| 1.0.1  | 2021/10/8  | 初始版本  | 倪神州  |  
| 1.0.2  | 2021/06/01  | 新增wifi通讯相关常量  | 王磊  |  
| 1.0.3  | 2023/01/06  | 增加使用指南  | 王磊  |  
| 1.0.4  | 2023/05/26  | 串口以及虚拟串口增加可配置参数，详见附录  | 王磊  |  
| 2.0.3  | 2023/07/24  | 大版本更新   
  
1. 2.0.0+ECR版本修复USB通讯遗留问题，去掉”注意事项”说明，使用2.0.0+版本已无该问题。   
  
2. 错误码重定义，增加错误码描述便于开发者自行检查问题   
  
3. 增加波特率常量定义   
  
4. ECRConnection增加等待连接状态   
  
5. ECRService增加停止函数  | 王磊  |  
| 2.0.4  | 2023/09/06  | 1.修改SDK依赖方式为远程依赖引用  | 王磊  |  
| 2.0.7  | 2023/10/19  | 1.修复RS232串口参数设置无效问题   
  
2.更新远程Lib,Lib注释修改为串口参数设置仅支持RS232  | 王磊  |  
| 2.0.8  | 2023/12/11  | 1.修复P2SE虚拟串口连接断开无回调问题   
  
2.VSP通讯增加可配置是否检测USB线连接状态功能  | 王磊  |  
| 2.0.10  | 2024/01/16  | 1.优化未检测到串口线的提示文案   
  
2.优化串口线检测逻辑，兼容已适配机型  | 王磊  |  
| 3.0.6  | 2026/01/12  | 1.新增websocket通信方式  | 王磊  |  
| 3.0.15  | 2026/06/24  | 1.新增虚拟串口模块  | 王磊  |  
## 2.SDK集成
1.在项目根目录的build.gradle添加如下maven仓库
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
repositories {
  maven { url "https://s01.oss.sonatype.org/content/groups/public/" }
}
  

```

2.在项目app目录下的build.gradle添加如下引用（2.0.4版本开始才支持远程依赖引用）
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```

dependencies {
        
    implementation 'com.sunmi:sunmi-ecr-service:3.0.15@aar'
}
  

```

3.添加完成后重新同步gradle
4.绑定远程ECR服务，推荐在applicaiton的onCreate中完成绑定
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
fun bindECRService() {
        ECRServiceKernel.getInstance().bindService(applicationContext, connectionCallback)
    }

private val connectionCallback = object : ECRServiceKernel.ConnectionCallback {

        override fun onServiceConnected() {
            Logger.e(App.TAG, "onServiceConnected")
        }

        override fun onServiceDisconnected() {
            Logger.e(App.TAG, "onServiceDisconnected")
        }

    }
  

```

## 3.API
### 3.1 ECRServiceKernel SDK 操作对象
#### 3.1.1 获取ECRServiceKernel实例  
| 原型  | ECRServiceKernel getInstance()  |  
| --- | --- |  
| 功能  | 获取ECR SDK单例对象  |  
| 参数  | 无  |  
| 返回值  | 返回ECRServiceKernel实例对象  |  
| 备注  | 无  |  
#### 3.1.2 连接ECR 服务  
| 原型  | void bindService(Context context, ConnectionCallback callback)  |  
| --- | --- |  
| 功能  | 连接ECR服务  |  
| 参数  | context [in]--上下文对象callback [in] --连接状态的回调，详见ConnectionCallback  |  
| 返回值  | 无  |  
| 备注  | 建议在App的Application类中连接ECR服务，未调用该方法去调用任意SDK API方法将会抛出NullPointerException。  |  
#### 3.1.3 断开ECR SDK  
| 原型  | void unbindService()  |  
| --- | --- |  
| 功能  | 断开ECR SDK连接  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 断开与ECR服务的连接。调用该函数后，ECRServiceKernel中所有操作模块对象全部置为null,调用任意SDK API会抛出NullPointerException。  |  
### 3.2 ConnectionCallback 连接回调
#### 3.2.1 ECR SDK 连接成功  
| 原型  | void onServiceConnected()  |  
| --- | --- |  
| 功能  | 回调方法，表示ECR SDK连接成功  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | SDK所有的API必须在收到该回调后调用，初始化SDK后，开发者需要关注该函数的回调，收到该函数表示SDK初始化成功，可以调用SDK相关API。  |  
#### 3.2.2 ECR SDK 连接失败  
| 原型  | void onServiceDisconnected()  |  
| --- | --- |  
| 功能  | 回调方法，表示ECR SDK连接失败或者连接断开  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 收到此回调表明当前App已断开与ECR服务的连接，此后若调用任意SDK API 将抛出NullPointerException。若要重新连接ECR服务，请调用bindService()方法。  |  
### 3.3 公共成员变量
连接ECR SDK后，在ECRServiceKernel中，可以使用一些功能模块与ECR SDK交互，如下表所示：  
| 变量名  | 说明  | 备注  |  
| --- | --- | --- |  
| ecrService  | ECRService操作模块  | 包含ECRService相关API  |  
### 3.4 ECRService操作对象
#### 3.4.1 建立连接  
| 原型  | void connect(Bundle bundle, ECRConnection connection)  |  
| --- | --- |  
| 功能  | 在2台设备之间建立连接  |  
| 参数  | bundle [in]--Bundle对象connection [in]--连接状态的回调。详见ECRConnection  |  
| 返回值  | 无  |  
| 备注  | 该方法和连接ECR SDK方法不同，该方法是建立设备之间的连接，而ECR SDK连接方法，是建立您的应用程序和ECR SDK之间的连接，并不是建立设备之间的连接。  |  
注意：必须指定连接方法和类型。“type”表示设备的连接类型，是服务端还是客户端。“mode”表示连接方法，如蓝牙或USB。如果是蓝牙的连接方式，你需要指定连接蓝牙的mac地址。关于Bundle参数的更多信息。详见ECRConstant
#### 3.4.2 断开连接  
| 原型  | void disconnect()  |  
| --- | --- |  
| 功能  | 断开2台设备之间的连接  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 该方法只是断开了2 台设备之间的连接，并没有断开与ECR SDK 之间的连接。如果后面您需要重新建立2台设备之间的连接，只需要重新调用connect()。  |  
#### 3.4.3 注册监听器  
| 原型  | register(ECRListener listener)  |  
| --- | --- |  
| 功能  | 注册监听器，接收数据  |  
| 参数  | listener [in]-- 数据监听器。详见ECRListener  |  
| 返回值  | 无  |  
| 备注  | 当另一端向该设备发送数据时，可以通过该监听器接收数据。  |  
#### 3.4.4 移除监听器  
| 原型  | void unregister(ECRListener listener)  |  
| --- | --- |  
| 功能  | 移除监听器  |  
| 参数  | listener [in] 数据监听器。详见ECRListener  |  
| 返回值  | 无  |  
| 备注  | 无  |  
#### 3.4.5 发送数据  
| 原型  | void send(byte[] bytes, ECRRequestCallback callback)  |  
| --- | --- |  
| 功能  | 向另一端发送数据  |  
| 参数  | bytes [in] -- 待发送的数据callback [in] -- 发送数据状态的回调。详见ECRRequestCallback  |  
| 返回值  | 无  |  
| 备注  | 无  |  
#### 3.4.6 停止发送数据  
| 原型  | void stop()  |  
| --- | --- |  
| 功能  | 停止数据发送(仅ibeacon用于停止数据的发送)  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 无  |  
#### 3.4.7 获取虚拟串口模块  
| 原型  | UartManager getUartManager()  |  
| --- | --- |  
| 功能  | 获取虚拟串口管理模块(仅3.0.15以及更高版本支持)  |  
| 参数  | 无  |  
| 返回值  | UartManager串口操作对象  |  
| 备注  | 无  |  
  

### 3.5 ECRConnection连接回调
#### 3.5.1 设备连接成功  
| 原型  | void onConnected()  |  
| --- | --- |  
| 功能  | 回调方法，设备之间连接成功的回调  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 无  |  
#### 3.5.2 设备连接失败  
| 原型  | void onDisconnected(int code, String message)  |  
| --- | --- |  
| 功能  | 回调方法，设备之间连接失败或者连接断开的回调  |  
| 参数  | code [in] -- 错误码message [in] -- 错误描述  |  
| 返回值  | 无  |  
| 备注  | 无  |  
#### 3.5.3 等待设备连接  
| 原型  | void onWaitingConnect()  |  
| --- | --- |  
| 功能  | 回调方法，Master设备已经就绪，等待slave设备连接  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 无  |  
### 3.6 ECRListener监听器
#### 3.6.1 接收数据  
| 原型  | onReceive(byte[] bytes)  |  
| --- | --- |  
| 功能  | 回调方法，接收另一端发送的数据  |  
| 参数  | bytes [in] -- 接收的数据  |  
| 返回值  | 无  |  
| 备注  | 无  |  
### 3.7 ECRRequestCallback 发送数据回调
#### 3.7.1 数据发送成功  
| 原型  | void onSuccess()  |  
| --- | --- |  
| 功能  | 回调方法，数据发送成功的回调  |  
| 参数  | 无  |  
| 返回值  | 无  |  
| 备注  | 无  |  
#### 3.7.2 数据发送失败  
| 原型  | void onFailure(int code, String message)  |  
| --- | --- |  
| 功能  | 回调方法，数据发送失败的回调  |  
| 参数  | code [in] -- 错误码message [in] -- 错误描述  |  
| 返回值  | 无  |  
| 备注  | 无  |  
### 3.8 UartManager模块(虚拟串口能力透传模块)
#### 3.8.1 打开串口  
| **原型**  | **void open(int channel, String attr) throws IllegalStateException**  |  
| --- | --- |  
| 功能  | 打开串口  |  
| 参数  | channel： 当前仅支持4，对应为常量为ECRConstant.VSPChannel.CHANNEL_VSP  |  
| attr:预留参数。传任意值均会被忽略 |  
| 返回值  | 无  |  
| 异常  | 函数调用需要捕获IllegalStateException异常，函数执行异常状况均通过异常抛出  |  
| 备注  | 无  |  
#### 3.8.2 关闭串口  
| **原型**  | **void close(int channel) throws IllegalStateException**  |  
| --- | --- |  
| 功能  | 关闭串口  |  
| 参数  | channel： 当前仅支持4，对应为常量为ECRConstant.VSPChannel.CHANNEL_VSP  |  
| 返回值  | 无  |  
| 异常  | 函数调用需要捕获IllegalStateException异常，函数执行异常状况均通过异常抛出  |  
| 备注  | 无  |  
#### 3.8.3 清除缓冲区  
| **原型**  | **void reset(int channel) throws IllegalStateException**  |  
| --- | --- |  
| 功能  | 清除缓冲区  |  
| 参数  | channel： 当前仅支持4，对应为常量为ECRConstant.VSPChannel.CHANNEL_VSP  |  
| 返回值  | 无  |  
| 异常  | 函数调用需要捕获IllegalStateException异常，函数执行异常状况均通过异常抛出  |  
| 备注  | 无  |  
#### 3.8.4 发送数据  
| **原型**  | **void sends(int channel, byte[] data) throws IllegalStateException**  |  
| --- | --- |  
| 功能  | 发送数据  |  
| 参数  | channel： 当前仅支持4，对应为常量为ECRConstant.VSPChannel.CHANNEL_VSP  |  
| data：等待发送的数据 (数据最大长度为8KB) |  
| 返回值  | 无  |  
| 异常  | 函数调用需要捕获IllegalStateException异常，函数执行异常状况均通过异常抛出  |  
| 备注  | 无  |  
#### 3.8.5 接收数据  
| **原型**  | **byte[] recvs(int channel, int recvsLenght, int usTimeOut) throws IllegalStateException**  |  
| --- | --- |  
| 功能  | 接收数据  |  
| 参数  | channel： 当前仅支持4，对应为常量为ECRConstant.VSPChannel.CHANNEL_VSP  |  
| recvsLenght：期望接受长度 |  
| usTimeOut：超时时间 |  
| 返回值  | 实际接收到的数据  |  
| 异常  | 函数调用需要捕获IllegalStateException异常，函数执行异常状况均通过异常抛出  |  
| 备注  | 无  |  
#### 3.8.6 检查接收缓冲区数据  
| **原型**  | **int rxPoolCheck(int channel) throws IllegalStateException**  |  
| --- | --- |  
| 功能  | 检查接收缓冲区数据  |  
| 参数  | channel： 当前仅支持4，对应为常量为ECRConstant.VSPChannel.CHANNEL_VSP  |  
| 返回值  | 接收缓冲区未被读取的数据长度  |  
| 异常  | 函数调用需要捕获IllegalStateException异常，函数执行异常状况均通过异常抛出  |  
| 备注  | 无  |  
## 4.附录
### 4.1 ECRConstant常量定义
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public final class ECRConstant {

    @Retention(RetentionPolicy.SOURCE)
    @StringDef({SLAVE, MASTER})
    public @interface Type {
        //从设备
        String SLAVE = "Slave";
        //主设备
        String MASTER = "Master";
    }


    @Retention(RetentionPolicy.SOURCE)
    @StringDef({VSP, NFC, USB, WIFI, RS232, iBeacon, Bluetooth})
    public @interface Mode {
        //虚拟串口(上位机仅支持非商米设备)
        String VSP = "VSP";
        //暂不支持
        String NFC = "NFC";
        //USB AOA通讯
        String USB = "USB";
        //WIFI通讯(同一局域网，且通讯设备可访问)
        String WIFI = "WIFI";
        //RS232 USB OTG转串
        String RS232 = "RS232";
        //蓝牙iBeacon
        String iBeacon = "iBeacon";
        //蓝牙通讯(需要先手动配对)
        String Bluetooth = "Bluetooth";
        //WebSocket通讯(同一局域网，且通讯设备可访问)
        String WebSocket = "WebSocket";
    }


    public interface SerialPort {

        @Retention(RetentionPolicy.SOURCE)
        @IntDef({DATABITS_5, DATABITS_6, DATABITS_7, DATABITS_8})
        @interface DataBits {

            /**
             * 5 data bits.
             */
            int DATABITS_5 = 5;
            /**
             * 6 data bits.
             */
            int DATABITS_6 = 6;
            /**
             * 7 data bits.
             */
            int DATABITS_7 = 7;
            /**
             * 8 data bits.
             */
            int DATABITS_8 = 8;
        }

        @Retention(RetentionPolicy.SOURCE)
        @IntDef({PARITY_NONE, PARITY_ODD, PARITY_EVEN})
        @interface Parity {
            /**
             * No parity.
             */
            int PARITY_NONE = 0;
            /**
             * Odd parity.
             */
            int PARITY_ODD = 1;
            /**
             * Even parity.
             */
            int PARITY_EVEN = 2;
        }


        @Retention(RetentionPolicy.SOURCE)
        @IntDef({STOPBITS_1, STOPBITS_1_5, STOPBITS_2})
        @interface StopBits {
            /**
             * 1 stop bit.
             */
            int STOPBITS_1 = 1;

            /**
             * 2 stop bits.
             */
            int STOPBITS_2 = 2;
        }

    }
    
        @Retention(RetentionPolicy.SOURCE)
        @interface Baudrate {
            int B0 = 0;
            int B50 = 50;
            int B75 = 75;
            int B110 = 110;
            int B134 = 134;
            int B150 = 150;
            int B200 = 200;
            int B300 = 300;
            int B600 = 600;
            int B1200 = 1200;
            int B1800 = 1800;
            int B2400 = 2400;
            int B4800 = 4800;
            int B9600 = 9600;
            int B19200 = 19200;
            int B38400 = 38400;
            int B57600 = 57600;
            int B115200 = 115200;
            int B230400 = 230400;
            int B460800 = 460800;
            int B500000 = 500000;
            int B576000 = 576000;
            int B921600 = 921600;
            int B1000000 = 1000000;
            int B1152000 = 1152000;
            int B1500000 = 1500000;
            int B2000000 = 2000000;
            int B2500000 = 2500000;
            int B3000000 = 3000000;
            int B3500000 = 3500000;
            int B4000000 = 4000000;
        }

        @Retention(RetentionPolicy.SOURCE)
        public @interface VSPCableCheckState {
           /**
            * 打开VSP通讯模式下，检测USB线连接状态(USB线断开连接会导致VSP连接断开，默认为该模式)
            */
           int ENABLE_CHECK = 1;
           /**
            * 关闭VSP通讯模式下，检测USB线连接状态(USB线断开连接不会导致VSP连接断开)
            */
           int DISABLE_CHECK = 2;
         }
         @Retention(RetentionPolicy.SOURCE)
         public @interface VSPChannel {
         /**
          * UartManager模块，虚拟串口通道下标
          */
          int CHANNEL_VSP = 4;
    }

}
  

```

### 4.2 ECRParameters常量定义
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
public class ECRParameters {
    /**
     * 通讯模式选择，详见Mode对象定义{@link ECRConstant.Mode}
     */
    public static final String MODE = "mode";
    
    /**
     * 设备主从类型，详见Type{@link ECRConstant.Type}
     */
    public static final String TYPE = "type";
    
    /**
     * 蓝牙Mac地址(value为String类型目标设备蓝牙MAC)
     * 
     * 仅当通讯类型TYPE为Bluetooth时，设备为MODE为Slave时，
     * Slave设备连接Master设备需要传入master设备蓝牙Mac地址(value为String类型)
     */
    public static final String BLUETOOTH_MAC_ADDRESS = "bluetoothMacAddress";
    
    /**
     * WiFi IP地址
     * 
     * 仅当通讯类型TYPE为WIFI与WebSocket时，设备为MODE为Slave时，
     * Slave设备连接Master设备需要传入master设备IP地址(value为String类型)
     */
    public static final String WIFI_ADDRESS = "WIFIAddress";
    
    /**
     * WiFi 端口号，
     * 
     * 仅当通讯类型为WIFI与WebSocket时，设备为类型为Master与Slave时，
     * Master设备connect需要传入符合端口号规则的自定义端口号
     * Slave设备connect Master设备需要传入master设备设置的端口号(value为String类型)
     */
    public static final String WIFI_PORT = "WIFIPort";
    
    /**
     * 数据位，详见Type{@link ECRConstant.SerialPort.DataBits}
     * 仅当{@link #MODE}等于
     *
     * @link com.pos.connection.bridge.binder.ECRConstant.Mode#RS232
     * } 时传入参数有效
     * <p>
     * 当未传入参数时，默认值为:{@link ECRConstant.SerialPort.DataBits#DATABITS_8}
     */
    public static final String DATA_BITS = "DataBits";
    
    /**
     * 校验位，详见Type{@link ECRConstant.SerialPort.Parity}
     * 仅当{@link #MODE}等于
     *
     * @link com.pos.connection.bridge.binder.ECRConstant.Mode#RS232
     * } 时传入参数有效
     * <p>
     * 当未传入参数时，默认值为:{@link ECRConstant.SerialPort.Parity#PARITY_NONE}
     */
    public static final String PARITY = "Parity";
    
    /**
     * 停止位，详见Type{@link ECRConstant.SerialPort.StopBits}
     * 仅当{@link #MODE}等于
     *
     * @link com.pos.connection.bridge.binder.ECRConstant.Mode#RS232
     * } 时传入参数有效
     * <p>
     * 当未传入参数时，默认值为:{@link ECRConstant.SerialPort.StopBits#STOPBITS_1}
     */
    public static final String STOP_BITS = "StopBits";
    
    /**
    * 波特率，详见Type{@link ECRConstant.SerialPort.Baudrate}
    * 仅当{@link #MODE}等于
    *
    * @link ECRConstant.Mode#RS232
    * } 时传入参数有效
    * <p>
    * 当未传入参数时，默认值为:{@link ECRConstant.SerialPort.Baudrate#B115200}
    */
    public static final String BAUD_RATE = "baudRate";

     /**
     * VSP通讯模式下，是否检查USB线连接状态，详见Type{@link ECRConstant.VSPCableCheckState}
     * <p>
     * 仅当{@link #MODE}等于
     * {@link ECRConstant.Mode#VSP}时传入参数有效
     * <p>
     * 当未传入参数时，默认值为:{@link ECRConstant.VSPCableCheckState#ENABLE_CHECK}
     */
    public static final String ENABLE_VSP_CABLE_CHECK = "enable_vsp_cable_check";

    /**
     * WebSocket 心跳间隔(秒)
     * <p>
     * 仅当{@link #MODE}等于{@link ECRConstant.Mode#WebSocket}时传入参数有效
     * <p>
     * 当未传入参数时，默认值为: 30秒
     */
    public static final String WEBSOCKET_HEARTBEAT_INTERVAL = "WebSocketHeartbeatInterval";

    /**
     * WebSocket 单次心跳超时时间(秒)
     * <p>
     * 仅当{@link #MODE}等于{@link ECRConstant.Mode#WebSocket}时传入参数有效
     * <p>
     * 当未传入参数时，默认值为: 10秒
     */
    public static final String WEBSOCKET_HEARTBEAT_TIMEOUT = "WebSocketHeartbeatTimeout";

    /**
     * WebSocket 最大允许心跳丢失次数
     * <p>
     * 仅当{@link #MODE}等于{@link ECRConstant.Mode#WebSocket}时传入参数有效
     * <p>
     * 当未传入参数时，默认值为: 3次
     */
    public static final String WEBSOCKET_HEARTBEAT_MAX_LOSS = "WebSocketHeartbeatMaxLoss";

}
  

```

### 4.3 错误码  
| 错误码  | 描述  | 备注  |  
| --- | --- | --- |  
| -1001  | 参数错误  |   
  
  
 |  
| -1002  | 该设备当前处于其他连接方式已经连接状态，无法重复连接  | 先调用断开连接函数后重新调用连接可解决该报错  |  
| -1003  | 连接已断开，通讯不可用  | 当前连接断开，连接失败或连接被断开将返回该错误码  |  
| -1004  | 授权被拒绝，请检查  | 必要权限被用户拒绝，例如USB通信弹框授权被拒绝  |  
| -1005  | 该设备不支持蓝牙  |   
  
  
 |  
| -1006  | 目标设备未就绪，连接失败  | 蓝牙通信中master设备未就绪，请先连接master，当master设备回调等待连接状态时，slaver设备再去连接  |  
| -1007  | 数据线未连接或OTG被禁用，请检查  | 可能出现该错误的原因:   
  
1.数据线未连接   
  
2.连接的数据线不具备数据传输功能   
  
3.使用了OTG功能(比如:USB转串口)，但是OTG被禁用(P2-A11支持禁用OTG)  |  
| -1008  | 未找到可用设备   
  
  
 | 串口通讯:未找到可用的设备，可能当前设备不支持当前连接的OTG转串芯片USB通信:未找到可用的设备，slave设备不为商米设备  |  
| -1009  | 未找到驱动程序  | 串口通信未找匹配到驱动程序  |  
| -1010  | 设备无可用端口  | 串口通信中，当前找到的设备没有可用端口  |  
| -1011  | 正在权限申请中，无法处理本次连接请求  | 上次函数调用正处于请求权限中，再次调用将被拒绝。需要主动调用disconnect，或等待上次授权请求结束  |  
| -1012  | 未找到可用Accessory设备，请检查是否被其他程序占用  | USB通信中，找不到可用的accessory设备  |  
| -1013  | 打开设备失败(UsbDeviceConnection为空)  | USB通信中，打开device失败  |  
| -1014  | 设备不支持商米虚拟串口  |   
  
  
 |  
| -1015  | wifi通讯服务端Master已经初始化  | Wifi通讯，master设备已经处于accept客户端连接状态，如果需要重新初始化，请先调用断开连接函数  |  
| -1016  | iBeacon发送的数据超过20个字节  |   
  
  
 |  
| -1017  | 数据发送失败  |   
  
  
 |  
| -1018  | 无法连接目标设备，请检查设备是否就绪  | WIFI通讯master设备未就绪，需要先连接master设备  |  
| -1019  | 未找到ECR服务，请先安装服务  | 设备未安装ecr服务  |  
| -1020  | ECR服务版本过低，请升级到2.0.0+版本  | ECR服务版本过低，请升级  |  
| -1021  | 发送数据超过限制(max:8KB)  | 串口通信单次发送数据超过限制，最大允许8KB  |  
| -1022  | IPC传输数据超过限制(max:240KB)  | 进程间通信单次发送数据超过限制，最大不建议超过240KB  |  
| -1023  | USB设备请求独占失败  | USB通信master端请求独占USB接口失败  |  
## 5.使用指南
### 5.1 蓝牙通讯方式
#### 1. 使用前两台设备需要打开蓝牙，并且蓝牙需要先配对成功
  * 1.1 两台需要配对设备，进入设备设置，选择开启蓝牙
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/8cb6ba6d87fb40ba8daec51c28d36101.png)
  * 1.2. 找到目标设备，点击，弹出配对确认对话框，点击配对
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/27df1a248d4d41f3bdee5ed250cbd77a.png)
  * 1.3. 查看已配对设备列表，确认设备已经配对成功
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/a197858727794d2098f8007a44af6f06.png)


#### 2. 打开ECR Demo, 设备类型选择，一台设备选择为Server,一台设备选择为Client
  * 2.1. 选择为Server的设备，点击右上角toolbar，下拉列表选择蓝牙后，点击连接，此时连接按钮状态变为不可点击，同时设备处于等待客户端连接状态
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/396917e62e5543baa0cdd0101bedffdb.png)
  * 2.2. 选择为Client的设备，点击右上角toolbar，下拉列表选择蓝牙后，弹出对话框用于选择已配对蓝牙设备
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/fe4313353d9b4e358abb09b1ac32ef61.png)
  * 2.3. Client设备，选择已配对蓝牙，点击确认，对话框关闭，点击右侧连接按钮。连接成功后UI显示已连接，此时两台设备即可进行数据交互
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/c5c454218576446f9de92267020a7d18.png)


### 5.2 iBeacon通讯方式
  1. 进入设备设置，选择开启蓝牙，无需配对
  2. 打开ECR Demo, 设备主从类型选择，一台设备选择为Server,一台设备选择为Client
  

![](https://cdn.sunmi.com/public/image/mgt-document/5733789a6e5d495bb652042d87ead1f6.png)
  3. 点击右上角toolbar，下拉列表选择Ibeacon
  

![](https://cdn.sunmi.com/public/image/mgt-document/340cd6d1b9804b618648b3797381065c.png)
  4. 点击连接，等待左下角显示连接成功后即可发送数据，ibeacon通讯方式为粘性数据，数据发送后会一直重复对外进行广播。需要手动取消数据发送。


### 5.3 WIFI通讯方式
#### 1.两台商米设备通过ECR服务WIFI通讯方式通信
  * 1.1 主从设备确保连接同一个WiFi，需要确保路由器没有限制局域网设备通信(验证方法:A,B两台设备连接同一个网段，查看设备A的IP地址，B设备通过ADB连接电脑，电脑CMD窗口通过adb shell命令ping A设备IP地址，查看是否能正常ping通)
  * 1.2 打开ECR Demo, 设备类型选择，一台设备选择为Server,一台设备选择为Client
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/4799c2bfc1a0464e900857b6a078ac30.png)
  * 1.3 选择为Server的设备。点击右上角toolbar，下拉列表选择WIFI后，点击连接，此时弹出对话框，需要输入WIFI通讯Master设备自定义的端口号(自行设定，只需要符合局域网通信的端口号规则即可)，点击CONFIRM，设备处于等待客户端连接状态。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/8c00eeb35edd42a384b6a9a2b8e70468.png)
  * 1.4 选择为Cilent的设备。点击右上角toolbar，下拉列表选择WIFI后，点击连接，此时弹出对话框，需要输入WIFI通讯Master设备IP地址以及端口号
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/75683b6421de454ca99c8f183228bcc8.png)
  * 1.5 选择为Cilent的设备。输入正确的Master设备IP地址以及Master设备设置的端口号后点击CONFIRM。等待WIFI通讯连接成功后即可进行数据收发
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/14a2393f26ff48d99e32ad087174a846.png)


#### 2. 一台商米设备与非商米设备连接(PC或其他支持标准socket协议的设备)
以商米手持设备与PC通讯为例
2.1 PC作为Master设备，商米手持设备作为Slave设备
  * 2.1.1 PC与商米手持设备确保连接同一个WiFi，需要确保路由器没有限制局域网设备通信(验证方法，设备与电脑连接同一个WiFi，电脑查看设备IP地址，电脑通过ping命令ping设备IP地址，查看是否能正常ping通)
  * 2.1.2 PC上安装SSCom程序(该程序支持局域网通信,可替换为任意支持所需功能的Windows程序)，打开SSCOM，端口号处下拉选择TCPServer,点击侦听。程序进入等待客户端连接状态
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/e7def5a1b2ca4ae39c8ef0f302facb50.png)
  * 2.1.3 商米手持设备上，打开ECR Demo, 设备类型选择，设备选择为Client
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/34a6f5e8ba8f4deeb28481b4fe16f1a8.png)
  * 2.1.4 点击右上角toolbar，下拉列表选择WIFI后，点击连接，此时弹出对话框，需要输入WIFI通讯Master设备IP地以及端口号(即输入PC上SSOM程序显示的本地IP地址以及端口号如下图红框所示)。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/5f0f7dd216e14b049bf8c817aea11d33.png)
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/a31b3e6129474b35add14e1caa14aaac.png)
  * 2.1.5 输入正确IP以及端口号，点击对话框CONFIRM后等待连接成功后即可与PC进行数据交互
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/6ecaac93bdcd40c59fb21398321bec9d.png)


### 5.4 商米虚拟串口通讯方式
商米虚拟串口仅支持商米设备直接通过数据线的方式连接PC。不需要额外的USB转串线
  * 准备具备数据传输功能的数据线，一端连接商米设备的Type-c口，另外一端连接电脑USB口。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/224554d26eea42b49f30e05ed9ecb0a4.png)
  * 电脑端用SSCOM作为测试工具。打开SSCOM，点击通讯端口，刷新通讯端口后，点击端口号下拉列表，选择对应的COM口。波特率选择为115200，其他参数为默认参数，不需要调整
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/c001ad9372ad4a54a485496b57e6b85f.png)
  * 商米设备端打开ECR Demo，主从设备选择可随意选择，虚拟串口通信不区分主从设备。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/d5299bf316d442ac9a8329df72aaafea.png)
  * ECR Demo点击右上角toolbar，下拉列表选择商米虚拟串口后，点击连接，等待界面显示连接成功后即可与PC进行数据通信。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/89b692d266524edb96ac40353a44924a.png)


### 5.5 串口通讯方式
  * 1.准备PL2303串口公母线(品牌为:UNITEK)，分别连接商米设备的Type-c口，不区分主从设备，公母头可以任意连接设备
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/c065a1a2229b479e9d17162dc349aa8a.png)
  * 两台商米设备分别打开ECR Demo，主从设备选择可随意选择，串口通信不区分主从设备。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/4c6789afd28f4f9985b209ca3f5b939f.png)
  * 两台设备上的ECR Demo点击右上角toolbar，下拉列表选择串口后点击连接，等待界面显示连接成功后两台设备即可进行数据通信。
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/1f1c2d1aedab4d84b9235c741296c9ab.png)


### 5.6 USB通讯方式
  * 准备具备数据传输功能的数据线，一个Type-c转USB转接头。分别连接两台商米设备
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/2d2cfb248ea34ba7951f804314af49cd.png)
  * Type-c转USB转接头连接的设备，打开ECRDemo,弹框选择为Server设备
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/93e5b28118a64ab2a9e58567dc3edcd3.png)
  * type-c直接连接的设备，打开ECR Demo,弹框选择为Client设备
  *   

![](https://cdn.sunmi.com/public/image/mgt-document/bc4f765127b24ab291514d1a39ab5bd6.png)
  * 选择为Server的设备，点击右上角toolbar,下拉列表选择USB，点击连接。连接成功进入等待从设备连接状态，UI显示USB已连接。（有可能会弹出USB授权弹窗，需要用户手动授权，不授权无法连接成功）
    * 假如有授权弹窗，请点击确认，赋予应用USB权限
    *   

![](https://cdn.sunmi.com/public/image/mgt-document/894a8ee64b514d5d968900b45e7465e0.png)
    * Server设备打开USB成功，等待cilent设备连接
    *   

![](https://cdn.sunmi.com/public/image/mgt-document/249749b93c8d438d87a3736061c07971.png)
  * 选择为Cilent的设备,点击右上角toolbar,下拉列表选择USB，点击连接，UI显示USB已连接，即可与Server设备进行数据交互。（有可能会弹出USB授权弹窗，需要用户手动授权，不授权无法连接成功）
    * 假如有授权弹窗，请点击确认，赋予应用USB权限
    *   

![](https://cdn.sunmi.com/public/image/mgt-document/635678bb1b404e37bbac651599db9983.png)
    * 连接成功后即可进行数据交互
    *   

![](https://cdn.sunmi.com/public/image/mgt-document/09ca1fd915b84a3aa9b665b258ac2524.png)


## 6.节点说明
全机型ROM支持虚拟串口的设备虚拟串口节点: /dev/ttyGS0/
P2smartpad三合一线串口节点:/dev/ttyHS1/ 其余设备根据UsbManagerService自动寻找挂载的PL2303设备，无需指定串口节点
上一篇：指纹文档
下一篇：1、了解收款音箱业务对接流程
