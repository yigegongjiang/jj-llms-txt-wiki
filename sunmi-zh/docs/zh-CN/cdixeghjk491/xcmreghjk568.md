---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xcmreghjk568
---

# 应用公钥生成方法
更新时间：2024-11-09 16:19:27
推荐使用 OpenSSL 工具生成密钥，以下是使用 OpenSSL 工具生成 RSA 公私钥对的方法:
# Windows
打开命令行终端（推荐使用 Git Bash，Git官方客户端默认集成了 OpenSSL 工具），分别执行如下命令生成 RSA 公私钥对。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//生成私钥
# openssl.exe genrsa -out /your/path/app_private_key.pem 2048
//生成对应公钥
# openssl.exe rsa -in /your/path/app_private_key.pem -pubout -out /your/path/app_public_key.pem
  

```

# Linux 和 Mac
打开命令行终端，分别执行如下命令生成 RSA 公私钥对。
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
//生成私钥
$ openssl genrsa -out /your/path/app_private_key.pem 2048
//生成对应公钥
$ openssl rsa -in /your/path/app_private_key.pem -pubout -out /your/path/app_public_key.pem
  

```

在 /your/path/ 文件夹下面可以看到 app_private_key.pem（RSA私钥）和 app_public_key.pem（RSA 公钥）2个文件。将RSA私钥保留，将RSA公钥配置到商米开放平台用于验证签名。以下为私钥文件和公钥文件示例。
**RSA 私钥文件示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAtI2kf6XdO09so4Wt+IlZwy4OVpPwYJn4DXtb0bs+Q0WoLi+o
xTwDV9l3fNekUWv5Ux/XOd4D1mNzRb/uBXNWjSckAK0c5n3P3yX2XAfQmHfNdg5F
Cjb78bHQYhm59WiyDXFBXOpfiGejjQCK5yptMzuVOAOGC89kSOVgLKddSxOa+ZSN
bD6g5gexxYY4zpHkyVJwzX2jOVEt+FciMAtxMVxY4HkkVWC19YhMJTBVJRBeHSeH
pdt5xwtvKynOlTgpZSDm+Nx4ifhNVSmuMU7R0pCJFtO9UQx9WKnERCOxRsY/QEHZ
rYBmGTSByzaKsKnk6puG3wEsFFqyBOE8MO29JwIDAQABAoIBAAd2l2cjXwq1lMMJ
opBUdR5pR/RyNwx3MNa106zOtZsoRPRryeklOuolthe1/jfTY8H43fepYrU49Snb
7iXRh7Fb/dP9U+V/K9mIgy94rzmuMhMAMgBjF3T9KaT0dUhcpIeDt/T0RzCnQA5z
CErX6R2mB9wB4UeZsgaFXL1bybJxsjpd55FKdLTUPtuI9mnf85j07e+Ov0MNP5+R
3orFzK/zn4hBdEGZ2j5Yw4kUq3iJ8vfQgWO2bgOXLQV76i03kMpZKdmMZbt0sj/J
JVVjgRkhPcYa2a3KB0z8VoZzEw+g9oidZdbEQ8paVC2Zr50fXxxWw7iYEgld/7+R
bppMYPECgYEA4RLLI2bWjg9HUSPDuwxKF47NV2hxxQhC9YKfSQHdJ7+ugtvOZ++T
WCA+nWKiI0DwgU0BP0fKVKw/56w+4qJdZNKe3YCa4jNheoWt8LT+FMXuM1/AXvPC
oUWH5Hs2bW3LHdL4pqk8DBACQeq/UG62Wx1NDDEyd2le17Rkg39vQakCgYEAzVzN
9NKobaBS+e5qhEt1rxeBVFEK2HI2UbVdTpU/cUCCgmW4M3ZUU7PtvtbysMjea7ga
EqvDTYx12shBa8tgxJSXYA8N9r3uc+MaHFAa09u+4gPAInxP5i9T0PfuWcyxmCiX
Tsi46W2jaP2t1Abj7Hy1wwgrQYaZFu4AV/N06k8CgYBjLdPj6roa6XKkt5zVfMnd
bajgnn6bs1NGOmQUWMTqchXlRhTDShfkmnjdA4ys+j3AzH0DNJBdFXYnkYg4eBGt
jv5AXx5hFQEca3HFgwv/fWTEsKnlbYbTZ/9E2GKYf+wkXUYCsspqClfkUhwxxkeH
CPddqwQxCHapmK0+INaW2QKBgQCFDmIrA+o4zS213bGJ8dJYFaSn2rJqNuXW3umL
psuqNx/YGMvWHA7bBLM/eMlZnWvu4yORgVRTBHS/wgwRKX6TH6tMg7SKb9j1oOkj
+wrGxZPRoJh2P5ENFRD7TghSintumK4gT5DvPSgpOGPOAoDekpa5vKs6E1lRL/dt
BMJ1pQKBgHQoD8slchB6unKjpgw2et2hVdv31ns/kYUkUEoBht1uY+QN280E3gBD
X51509HcaWP7d1pW7xGA1aVt25STlTUwg8Lttp11bSxWHCesbVIGK/hY8LApVkMr
m4WHMurr+uZ9g34kjyfgCMaeT0arFU+NO6U3tYPZk03H3JDd/uY+
-----END RSA PRIVATE KEY-----
  

```

**RSA 公钥文件示例**
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtI2kf6XdO09so4Wt+IlZ
wy4OVpPwYJn4DXtb0bs+Q0WoLi+oxTwDV9l3fNekUWv5Ux/XOd4D1mNzRb/uBXNW
jSckAK0c5n3P3yX2XAfQmHfNdg5FCjb78bHQYhm59WiyDXFBXOpfiGejjQCK5ypt
MzuVOAOGC89kSOVgLKddSxOa+ZSNbD6g5gexxYY4zpHkyVJwzX2jOVEt+FciMAtx
MVxY4HkkVWC19YhMJTBVJRBeHSeHpdt5xwtvKynOlTgpZSDm+Nx4ifhNVSmuMU7R
0pCJFtO9UQx9WKnERCOxRsY/QEHZrYBmGTSByzaKsKnk6puG3wEsFFqyBOE8MO29
JwIDAQAB
-----END PUBLIC KEY-----
  

```

上一篇：签名和验签
下一篇：公共错误码
