---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xmfieghjk579
---

# RFID uniapp插件使用说明
更新时间：2025-12-03 11:36:40
## 1. 插件介绍
本插件用于uniapp、uniapp-x开发Sunmi L2k、L2s等Android设备上的UHF电子标签功能。  
当前插件支持对ISO18000-6C标签的盘存、读写等操作。
## 2. 插件使用
在`script`中引入插件
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import * as uhf from '../../uni_modules/sunmi-uhf'
  

```

定义要使用的方法
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
methods:{
    //初始化sdk
    init() {
        uhf.initSdk({
            model(ret) {
                
            },
            sn(ret) {
               
            },
            softVersion(ret) {
                
            },
            hardwareVersion(ret) {
             
            }
        })
    }
    
    //标签盘存
    inventory() {
        uhf.realTimeInventory(1, {
            success: (ret) => {
      
            }
        })
    }
    //释放sdk
    deInit() {
        uhf.deInitSdk()
    }
}
  

```

## 3. API列表  
| 方法名  | 简单说明  |  
| --- | --- |  
| initSdk  | 插件所有操作需要先初始化sdk，并可以获取当前设备的基础信息  |  
| deInitSdk  | 如果不使用插件可以释放sdk  |  
| getDeviceStatus  | 实时获取设备的各种状态参数  |  
| reset  | 复位设备（复位uhf模块非Android设备）  |  
| realTimeInventory  | 常规盘存方法（推荐使用）  |  
| customInventory  | 自定义参数盘存方法  |  
| switchInventory  | 快速切换天线盘存方法  |  
| accessTag  | 设置需要操作标签的EPC  |  
| cancelAccessTag  | 取消正在操作的标签  |  
| getAccessTag  | 获取正在操作的标签EPC  |  
| readAccessTag  | 读取当前标签的信息  |  
| writeAccessTag  | 写入当前标签的信息  |  
| lockAccessTag  | 锁定标签的读写区域  |  
| killAccessag  | 销毁标签  |  
| cacheInventory  | 缓存结果盘存方法  |  
| getCacheInventory  | 获取缓存盘存的标签结果  |  
| getCacheInventoryNums  | 获取缓存盘存的标签数量  |  
| resetCacheInventory  | 重置盘存的缓存  |  
| setImpinjFastTid  | 配置支持FastTID  |  
| setWorkAntenna  | 指定当前工作的天线  |  
| setOutputAllPower  | 设置各个天线的工作功率  |  
| setOutputPower  | 设置单个天线的工作功率  |  
| setTemporaryOutputPower  | 暂时设置各个天线的工作功率，复位后恢复  |  
| setFrequencyRegion  | 区域频段设置  |  
| setUserFrequencyRegion  | 自定义区域频段设置  |  
| setRfLinkProfile  | 设置射频链路  |  
| setTagMask  | 设置标签过滤规则  |  
| clearTagMask  | 清除标签过滤规则  |  
### 3.1 initSdk(deviceInfo)
初始化SDK
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| deviceInfo  | DevceInfo  | 是  | {}  | 初始化成功后返回的UHF模块信息  |  
**DevceInfo的属性值**  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| model  | (v: number) => void  | 否  | null  | 模块的型号  |  
| sn  | (v: string) => void  | 否  | null  | 模块SN  |  
| softVersion  | (v: number) => void  | 否  | null  | 模块软件版本号  |  
| hardwareVersion  | (v: number) => void  | 否  | null  | 模块版本号  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
import * as uhf from '../../uni_modules/sunmi-uhf'
export default {
    //启动时初始化
    onReady() {
        let that = this
        uhf.initSdk({
            model : function(ret) {
                that.model = ret
            },
            sn(ret) {
                that.sn = ret
            },
            softVersion(ret) {
                that.soft = ret
            },
            hardwareVersion(ret) {
                that.hard = ret
            }
        })

    },
    //退出时反初始化
    onUnload() {
        uhf.deInitSdk()
	},
}

  

```

### 3.2 deInitSdk()
释放SDK，当不需要使用uhf时可以调用完全释放资源。
### 3.3 getDeviceStatus(status)
获取模块运行时的配置状态
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| query  | Array  | []  | 无  | 指定查询的状态，即DeviceStatus中的名称  |  
| status  | DeviceStatus  | 是  | {}  | 查询到当前模块的各种状态或参数信息  |  
**DeviceStatus的属性值**  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| batteryRemainingPercent  | (v: string) => void  | 否  | null  | 返回当前模块剩电量的百分比 *%  |  
| batteryVoltage  | (v: string) => void  | 否  | null  | 返回当前模块的电池电压 *mV  |  
| batteryChargeState  | (v: number) => void  | 否  | null  | 充电状态  |  
| temperature  | (v: string) => void  | 否  | null  | 模块当前的温度 *°C  |  
| fastTid  | (v: boolean) => void  | 否  | null  | 返回FTID的设置状态  |  
| antId  | (v: number) => void  | 否  | null  | 返回当前正使用的天线序号  |  
| power  | (v: Array<number>) => void  | 否  | null  | 返回当前每个天线的功率 dBm  |  
| regionInfo  | (v: RegionInfo) => void  | 否  | null  | 返回当前的区域频段配置  |  
| readerId  | (v: Array<number>) => void  | 否  | null  | 返回模块的ReaderId  |  
| rflink  | (v: number) => void  | 否  | null  | 射频链路  |  
| maskRule  | (v: MaskRule) => void  | 否  | null  | 返回当前的标签过滤规则  |  
**RegionInfo的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| region  | number  | 当前的频段区域  |  
| frequencyStart  | number  | 开始频率序号  |  
| frequencyEnd  | number  | 结束频率序号  |  
| frequencyHead  | number  | 自定义开始频率单位为MHz  |  
| frequencyInterval  | number  | 自定义频段间隔（1~88)*Mhz  |  
| frequencyQuantity  | number  | 自定义频段数量个数(1~60)  |  
**MaskRule的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| maskNo  | number  | 指定过滤规则的序号  |  
| session  | number  | 自定义盘存模式  |  
| rules  | number  | 过滤规则  |  
| maskMem  | number  | 过滤区域  |  
| maskValue  | Array<number>  | 过滤内容  |  
| address  | number  | 过滤内容所在区域的起始位置  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//例如进入页面时查询当前的标签过滤信息
onLoad() {
    uhf.getDeviceStatus(["maskRule"], {
        maskRule(rules) {
            console.log(rules)
        }
    })
},
  

```

### 3.4 reset(complete)
复位模块，复位完成将通过异步回调返回。
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| complete  | () => void  | 是  |   
 | 复位完成后触发的回调  |  
### 3.5 realTimeInventory(repeate, result)
常规的盘存方法，每次使用API盘存将会激活标签，获取标签信息，实际使用中需要反复调用API来检索区域内的标签，repeate表示本次盘存过程中同一个标签的上报数量。
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| repeate  | number  | 是  | 1  | 盘存次数  |  
| result  | InventoryResult  | 是  | {}  | 盘存结果  |  
**InventoryResult的属性**  
盘存结果  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| success  | (v: InventoryResultInfo) => void  | 盘存成功的反馈  |  
| fail  | (v: string) => void  | 盘存失败的反馈  |  
| tag  | (v: TagInfo) => void  | 盘存到的标签信息  |  
**InventoryResultInfo的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| rate  | number  | 本次盘存的标签识别速度 *个/秒  |  
| count  | number  | 本次盘存的标签数量  |  
| start  | number  | 本次盘存开始时间戳  |  
| end  | number  | 本次盘存结束时间戳  |  
| duration  | number  | 轮询天线盘存方法下的总耗时  |  
| cache  | number  | 缓存盘存方法下的缓存标签数量  |  
**TagInfo的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| state  | number  | 标签状态  |  
| antId  | number  | 盘存本标签的天线ID  |  
| pc  | string  | pc数据  |  
| epc  | string  | epc数据  |  
| crc  | string  | crc数据  |  
| rssi  | string  | 信号强度 *dBm  |  
| readCount  | number  | 标签已经被识读的次数  |  
| freq  | string  | 载波频率  |  
| updateTime  | number  | 最后识别到标签的时间戳  |  
| firstAnt  | number  | 轮询盘存标签被第一个天线识别的次数  |  
| secondAnt  | number  | 轮询盘存标签被第二个天线识别的次数  |  
| thirdAnt  | number  | 轮询盘存标签被第三个天线识别的次数  |  
| forthAnt  | number  | 轮询盘存标签被第四个天线识别的次数  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.realTimeInventory(1, {
                        success(inventory) {
                            //盘存执行成功完成
                        },
                        fail(error) {
                            //返回盘存失败结束
                        },
                        tag(info) {
                            //盘存过程中盘存到的标签信息
                        }
                    })
  

```

### 3.6 customInventory(strategy, result)
自定义盘存方法
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| strategy  | CustomStrategy  | 是  |   
 | 自定义盘存的策略  |  
| result  | InventoryResult  | 是  | {}  | 盘存结果  |  
**CustomStrategy的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| repeate  | number  | 盘存次数  |  
| session  | number  | 自定义盘存模式  |  
| target  | number  | 盘存标志 0-1  |  
| flag  | number  | 选择标志 0-3  |  
| phase  | number  | 相位开关 0-1  |  
| powersave  | number  | 节能标志 0-255  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.customInventory({
    repeate: 1,
    session: 1,
    target: 0,
    flag: 0,
    phase: 0,
    powersave: 0,
}, {
    success(inventory) {
        //盘存成功完成
    },
    fail(error) {
        //盘存失败结束
    },
    tag(info) {
        //盘存到的标签信息
    }
})
  

```

### 3.7 switchInventory(strategy, result)
轮询天线盘存方法
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| strategy  | SwitchStrategy  | 是  |   
 | 自定义盘存的策略  |  
| result  | InventoryResult  | 是  | {}  | 盘存结果  |  
**SwitchStrategy的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| repeate  | number  | 盘存次数  |  
| firstId  | number  | 指定第一个轮询天线的ID 0-3  |  
| firstNums  | number  | 指定第一个轮询天线的次数  |  
| secondId  | number  | 指定第二个轮询天线的ID 0-3  |  
| secondNums  | number  | 指定第二个轮询天线的次数  |  
| thirdId  | number  | 指定第三个轮询天线的ID 0-3  |  
| thirdNums  | number  | 指定第三个轮询天线的次数  |  
| forthId  | number  | 指定第四个轮询天线的ID 0-3  |  
| forthNums  | number  | 指定第四个轮询天线的次数  |  
| interval  | number  | 每个天线间隔轮询的间隔时间单位ms  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.switchInventory({
        repeate: 1,
        firstId: 0,
        firstNums: 1,
        secondId: 1,
        secondNums: 1,
        thirdId: 2,
        thirdNums: 1,
        forthId: 3,
        forthNums: 1,
        intervalTime: 10,
    }, {
        success(inventory) {
            //盘存成功完成
        },
        fail(error) {
            //盘存失败结束
        },
        tag(info) {
            //盘存到的标签信息
        }
    })
  

```

### 3.8 accessTag(epc, result)
设置预操作的标签，对标签的读写等控制需要先指定标签。
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| epc  | string  | 是  | 无  | 欲操作的标签EPC  |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
**OptResult的属性**
部分API的标签操作结果只需要关心成功和失败的回调即可  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| success  | (opt: OptResultInfo) => void  | 操作成功的结果  |  
| fail  | (error: string) => void  | 操作失败的结果  |  
**OptResultInfo的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| start  | number  | 命令执行开始的时间戳  |  
| end  | number  | 命令执行结束的时间戳  |  
| accessTag  | string  | 当前指定标签的EPC  |  
| pc  | string  | pc字段  |  
| crc  | string  | crc字段  |  
| epc  | string  | epc字段  |  
| data  | string  | 操作数据  |  
| length  | number  | 操作数据长度  |  
| antId  | number  | 天线ID  |  
| readCount  | number  | 识读次数  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.accessTag(/* 盘存得到的EPC字符串 */, {
                success(opt) {
                    //仅需要关注是否成功
                    uni.showToast({
                        title: "access tag successfully",
                        icon:'none'
                    })
                },
                fail(error) {
                    //仅需要关注是否失败
                    uni.showToast({
                        title: error,
                        icon:'none'
                    })
                }
            })
  

```

### 3.9 cancelAccessTag(result)
取消要操作的标签
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
### 3.10 getAccessTag(result)
获取当前正在操作的标签
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.getAccessTag({
    success(opt) {
        //获取记录的标签信息
        console.log(opt.accessTag)
    },
    fail(error) {
        console.log(error)
    }
})
  

```

### 3.11 readAccessTag(request, result)
读取指定的标签数据
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| request  | ReadOptequest  | 是  | {}  | 读取标签参数  |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
**ReadOptRequest的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| password  | string  | 操作标签的密码 指定4位  |  
| mem  | number  | 读取存储区域  |  
| address  | number  | 读取数据在标签区域的首地址  |  
| length  | number  | 读取数据的长度  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.readAccessTag({
        password: "1234",
        mem: 0,
        address: 0,
        length: 1
    }, {
        success(opt) {
            //可展示读取到的标签数据
            console.log(opt)
        },
        fail(error) {
            console.log(error)
        }
    })
  

```

### 3.12 writeAccessTag(request, result)
向指定标签写入数据
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| request  | WriteOptRequest  | 是  | {}  | 写入标签参数  |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
**WriteOptRequest的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| password  | string  | 操作标签的密码 指定4位  |  
| mem  | number  | 写入存储区域  |  
| address  | number  | 写入数据在标签区域的首地址  |  
| length  | number  | 写入数据的长度  |  
| data  | string  | 写入的数据Hex字符串  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.writeAccessTag({
                password: "1234"
                mem: 0,
                address: 0,
                length: 4,
                data: "30313233"//即0123
            }, {
                success(opt) {
                    console.log(opt)
                },
                fail(error) {
                    console.log(error)
                }
            })
  

```

### 3.13 lockAccessTag(request, result)
锁定指定标签的存储区域
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| request  | LockOptRequest  | 是  | {}  | 锁定标签参数  |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
**LockOptRequest的属性**  
| 名称  | 类型  | 描述  |  
| --- | --- | --- |  
| password  | string  | 操作标签的密码 指定4位  |  
| lockMem  | number  | 需要锁操作的区域  |  
| lockType  | number  | 锁定操作方式  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.lockAccessTag({
                    password: "1234",
                    lockMem: 0,
                    lockType: 0
                },{
                    success(opt) {
                        console.log(opt)
                    },
                    fail(error) {
                        console.log(error)
                    }
                })
  

```

### 3.14 killAccessag(password, result)
对指定的标签执行销毁操作，此操作是不可逆的，请谨慎使用。
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| password  | string  | 是  | {}  | 执行标签销毁的密码(与操作标签密码不同)  |  
| result  | OptResult  | 是  | {}  | 操作结果反馈  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.killAccessTag("1111", {
    success(opt) {
        console.log(opt)
    },
    fail(error) {
        console.log(error)
    }
})
  

```

### 3.15 cacheInventory(repeate, result)
缓存盘存方法，开启缓存盘存，使用此方法不会直接返回盘存到的标签结果，可在需要时通过读取方法获取缓存的标签数据。
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| repeate  | number  | 是  | 1  | 盘存次数  |  
| result  | InventoryResult  | 是  | {}  | 盘存结果  |  
### 3.16 getCacheInventory(clear, result)
获取缓存盘存到的标签
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| clear  | boolean  | 是  | 无  | 是否获取后清除缓存记录  |  
| result  | InventoryResult  | 是  | {}  | 盘存结果  |  
### 3.17 getCacheInventoryNums(result)
获取缓存盘存到的有效标签数量
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| result  | InventoryResult  | 是  | {}  | 执行结果  |  
### 3.18 resetCacheInventory(result)
重置缓存标签
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| result  | InventoryResult  | 是  | {}  | 执行结果  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//1、首先启动盘存
uhf.cacheInventory(1, {
        success(inventory) {
            //盘存成功完成
        },
        fail(error) {
            //盘存失败结束
        }
    })
//2、获取盘存到的标签信息
uhf.getCacheInventory(false, {
                success(inventory) {
                    //获取完成
                },
                tag(info) {
                    //标签信息
                },
                fail(error) {
                   //获取失败
                },
            })
//3、可选清除缓存到的标签信息
uhf.resetCacheInventory({
            success(inventory) {
                //清除成功
            },
            fail(error) {
                //清除失败
            }
        })
  

```

### 3.19 setImpinjFastTid(open, save, result)
针对Impinj Monza标签启用FastTID功能
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| open  | boolean  | 是  | 无  | 是否开启FastTID能力  |  
| save  | boolean  | 是  | 无  | 是否将设置保存到模块  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.setImpinjFastTid(false, false, {
        success(opt) {
            console.log("setImpinjFastTid success")
        },
        fail(error) {
            console.log(`setImpinjFastTid ${error}`)
        }
    })
  

```

### 3.20 setWorkAntenna(antId, result)
指定当前模块使用的天线
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| antId  | number  | 是  | 无  | 指定工作的天线ID 0-3  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.21 setOutputAllPower(power, result)
设置所有天线的工作功率
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| power  | number  | 是  | 无  | 工作天线的功率 0-33dBm  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.22 setOutputPower(power1, power2, power3, power4, result)
设置各个天线的工作功率
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| power1  | number  | 是  | 无  | ID0工作天线的功率 0-33dBm  |  
| power2  | number  | 是  | 无  | ID1工作天线的功率 0-33dBm  |  
| power3  | number  | 是  | 无  | ID2工作天线的功率 0-33dBm  |  
| power4  | number  | 是  | 无  | ID3工作天线的功率 0-33dBm  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.23 setTemporaryOutputPower(power, result)
临时设置所有天线的工作功率，模块复位后将恢复
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| power  | number  | 是  | 无  | 临时功率 20-33dBm  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.24 setFrequencyRegion(region, start, end, result)
模块频段区域设置，指定模块使用的区域以及此区域下的频率范围
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| region  | number  | 是  | 无  | 频段区域 1-3  |  
| start  | number  | 是  | 无  | 根据选择的频段区域范围设置起始频率  |  
| end  | number  | 是  | 无  | 根据选择的频段区域范围设置结束频率  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.25 setUserFrequencyRegion(interval, quantity, start, result)
设置自定义的频段范围
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| interval  | number  | 是  | 无  | 自定义频段的间隔 1-88Mhz  |  
| quantity  | number  | 是  | 无  | 自定义频段的数量 1-60个  |  
| start  | number  | 是  | 无  | 自定义频段的起始频率 Mhz  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.26 setRfLinkProfile(profile, result)
射频链路配置
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| profile  | number  | 是  | 无  | 射频链路  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
### 3.27 setTagMask(maskRule, result)
设置标签盘存过滤规则，可根据设置的规则过滤掉盘存到的标签。
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| maskRule  | MaskRule  | 是  | 无  | 配置规则参数  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
#### 示例
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
uhf.setTagMask({
    maskNo: 1,
    session: 0,
    rules: 0,
    maskMem: 0,
    maskValue: [30,31,32,33],
    address: 0  //标签EPC内容过滤的位置
}, {
    success(opt) {
        console.log("set tag success")
    },
    fail(error) {
        console.log(error)
    }
})
  

```

### 3.28 clearTagMask(maskNo, result)
清除标签盘存过滤规则
#### 参数  
| 名称  | 类型  | 必填  | 默认值  | 描述  |  
| --- | --- | --- | --- | --- |  
| maskNo  | number  | 是  | 无  | 指定清除的已设置规则序号  |  
| result  | OptResult  | 是  | {}  | 操作执行结果  |  
#### 示例
TODO
## 4. 详细说明
### 4.1 模块的型号
UHF模块的型号：  
100 未识别  
101 R2000  
102 INNER  
103 S700
### 4.2 充电状态
模块的充电状态：  
0 未充电  
1 预充电  
2 快速充电  
3 充电完成
### 4.3 射频链路
模块的射频链路配置信息：  
0 Tari 25uS,FM0 40KHz  
1 Tari 25uS,Miller 4 250KHz  
2 Tari 25uS,Miller 4 300KHz  
3 Tari 6.25uS,FM0 400KHz
### 4.4 频段区域
模块的区域频段类型：  
1 美规频段 902~928MHz  
2 欧规频段 865~868MHz  
3 中规频段 920~925MHz  
4 自定义频段
### 4.5 自定义盘存模式
当前盘存的模式： 0 S0模式 1 S1模式 2 S2模式 3 S3模式
### 4.6 标签过滤规则
设置或读取标签的过滤规则：  
0 (匹配)确认 SL 标志或已盘标志 → A  
(不匹配)取消确认 SL 标志或已盘标志 → B  
1 (匹配)确认 SL 标志或已盘标志 → A  
(不匹配)无作为  
2 (匹配)无作为  
(不匹配)取消确认 SL 标志或已盘标志 → B  
3 (匹配)否定 SL 标志或已盘标志 A → B，B → A  
(不匹配)无作为  
4 (匹配)取消确认 SL 标志或已盘标志 → B  
(不匹配)确认 SL 标志或已盘标志 → A  
5 (匹配)取消确认 SL 标志或已盘标志 → B  
(不匹配)无作为  
6 (匹配)无作为  
(不匹配)确认 SL 标志或已盘标志 → A  
7 (匹配)无作为  
(不匹配)否定 SL 标志或已盘标志 A → B，B → A
### 4.7 标签数据区域
设置或读取标签的过滤规则生效区域：  
0 None  
1 EPC  
2 TID  
3 USER
### 4.8 标签状态
盘存到的标签状态：  
1 表示此标签为首次识别  
2 表示此标签为再次识别
### 4.9 盘存次数
盘存次数表示每次使用盘存方法时，对同一个标签的返回次数，如果设置为1表示只上报一次标签，同一个标签在此标签生命周期中不再上报。  
为255则此轮盘存时间为最短时间。 如果射频区域内只有一张标签，则此轮的盘存约耗时为30-50ms。一般在四通道机器上快速轮询多个天线时使用此参数值。
### 4.10 可锁定数据区域
对标签执行锁操作的数据区域  
1 USER  
2 TID  
3 EPC  
4 PASSWORD  
5 KILL PASSWORD
### 4.11 锁定方法
对标签执行锁操作的类型，注意使用永久方式对标签是不可逆的，请谨慎使用  
0 开放  
1 锁定  
2 永久开放  
3 永久锁定
上一篇：RFID SDK集成说明
下一篇：商米设备如何操作钱箱
