---
url: https://docs.sunmi.com/zh-CN/ceghjk502/xzxaeghjk480
---

# 终端设备外接MINI AP用户手册

更新时间：2026-03-23 20:08:16

# 1. 外接MINI AP的两种方式

Client/接收模式的MINI AP有两种与终端设备的连接方式：

<!-- prettier-ignore -->
|  | 方式1 | 方式2 |
| --- | --- | --- |
| **连接方式** | 通过**网线**连接MINI AP的LAN口和终端设备的LAN口<br>（需通过USB Type-C口给MINI AP供电） | 通过**USB Type C-C线**连接MINI AP的USB Type-C口和终端设备的USB Type-C口 |
| **接口图示** |
![](https://cdn.sunmi.com/public/image/mgt-document/b599029c8f0b420ba20aee60438541e0.png)

 |

![](https://cdn.sunmi.com/public/image/mgt-document/44f0fe284207439cb3bb12bc109b1120.png)

 |
| **适用终端** |

-   带有LAN口的一切终端设备（Android、Windows、Linux等系统均支持）


 |

-   不带LAN口、只有USB Type-C口的安卓设备（与设备本身的安卓系统有关）

-   支持RDNIS的终端设备（如部分Windows设备）


 |
| **优势** |

-   LAN口为标准接口，通用性极好，兼容性极好

-   没有LAN口的设备还可以通过USB-LAN转接器实现


 |

-   只需要一根线，同时实现供电和供网，减少线材成本

-   减少一根线，部署更简单、美观


 |
| **不足** |

-   还需专门为MINI AP供电，需要额外的一根电源线和配套的插头、插座


 |

-   MINI AP固件版本需要在**1.3.1**及以上

-   因为USB Type-C接口标准不统一，因此对安卓终端设备自身的系统能力有要求，兼容性不如LAN口

-   部分终端设备可通过安装RDNIS驱动实现支持RDNIS（请自行查找所用设备是否有可用的RDNIS驱动）

-   iPhone、iPad暂不支持


 |
| **共性** | MINI AP充当终端设备的外接网卡，相当于连接有线网，因此此时设备的Wi-Fi不会生效 |

# 2. 外接MINI AP时的终端表现

外接Client/接收模式的MINI AP时，当终端有如下表现时，表示此时网络已连接生效：

<!-- prettier-ignore -->
| 连接方式 | 方式1：网线连接MINI AP的LAN口和终端设备的LAN口 | 方式2：USB Type C-C线连接MINI AP的USB Type-C口和终端设备的USB Type-C口 |
| --- | --- | --- |
| **标识概述** | 因为MINI AP充当终端设备的外接网卡，所以此时设备相当于连接有线网，**因此在终端上会出现表示有线网连接的图标** |
| **表现示例** | **Windows电脑** | **Windows电脑（支持RDNIS）** |
| **Android台式** | **Android台式** |
| / | **Android手持** |

---

上一篇：MINI AP DMP用户手册
下一篇：MINI AP常见问题
