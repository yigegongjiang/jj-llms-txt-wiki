---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xcmzeghjk557
---

# 签名和验签
更新时间：2024-11-22 15:30:19
## 一、请求规则
请求方式使用 POST ，请求数据格式为 application/json，文件上传类接口请求数据格式为 multipart/form-data。
请求必传header：
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Sunmi-Timestamp    // 当前时间戳
Sunmi-Sign         // 签名内容，小写
Sunmi-Nonce        // 6位随机数
Sunmi-Appid        // 申请的APPID
  

```

# 二、接口请求数据的签名方式
## 一般接口（非文件上传类接口）
需要根据接口要求的签名类型来计算签名，如果接口要求使用使用Appkey则使用Appkey来计算签名，如果接口要求使用RSA则使用RSA来计算签名。
Appkey 签名方式
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
HMACSHA256(json-body + Sunmi-Appid + Sunmi-Timestamp + Sunmi-Nonce, Appkey)
  

```

如果结果不是十六进制字符，则需要转成小写的十六进制字符。
RSA 签名方式，RsaPrivateKey为开发者私钥
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Base64Encode(Sha256WithRsa(json-body + Sunmi-Appid + Sunmi-Timestamp + Sunmi-Nonce, RsaPrivateKey))
  

```

## 文件上传类接口
文件上传统一使用 multipart/form-data 方式，包含如下参数
  * file // 文件流
  * params // json字符串，包含接口需要的所有参数
  * md5 // 文件的md5值 必填
  * file_type // 例如文件类型


统一使用Appkey来计算签名，签名方式为
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
HMACSHA256(params + Sunmi-Appid + Sunmi-Timestamp + Sunmi-Nonce , Appkey)
  

```

如果结果不是十六进制字符，则需要转成十六进制字符。
# 三、接口响应数据的签名方式
对于签名验证成功的请求，会使用商米私钥或 Appkey 对响应数据进行签名，签名方式和对请求数据的签名方式保持一致，签名字段 Sunmi-Sign 放在HTTP头部中。对于使用Appkey签名的数据，开发者需要使用商米公钥来校验签名；对于使用RSA方式签名的数据，开发者需要使用商米公钥来校验签名。
# 四、回调通知数据的签名方式
当回调开发者的接口时，会使用商米私钥或 Appkey 对回调数据进行签名，header 包含如下字段
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Sunmi-Timestamp    // 当前时间戳
Sunmi-Sign         // 签名内容
Sunmi-Nonce        // 6位随机数
Sunmi-Appid        // 申请的APPID
Sunmi-NotifyType   // 通知类型
  

```

Appkey 签名方式
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
HMACSHA256( json-body + Sunmi-Appid + Sunmi-Timestamp + Sunmi-Nonce + Sunmi-NotifyType, APPKEY )
  

```

如果结果不是十六进制字符，则需要转成十六进制字符。
RSA 签名方式，RsaPrivateKey为商米私钥
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
Base64Encode(Sha256WithRsa( json-body + Sunmi-Appid + Sunmi-Timestamp + Sunmi-Nonce + Sunmi-NotifyType, RsaPrivateKey))
  

```

对于使用Appkey签名的数据，开发者需要使用商米公钥来校验签名；对于使用RSA方式签名的数据，开发者需要使用商米公钥来校验签名。
# 五、签名代码示例
## java
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// import java.security.*;
// import java.security.spec.InvalidKeySpecException;
// import java.security.spec.PKCS8EncodedKeySpec;
// import java.util.Base64;
public String sign(String body, String appId, String timestamp, String nonce, String rsaPrivateKey) throws NoSuchAlgorithmException, InvalidKeySpecException, InvalidKeyException, SignatureException {
	String content = body + appId + timestamp + nonce;
	byte[] keyBytes = Base64.getDecoder().decode(rsaPrivateKey.replaceAll("(\\s)|(--.*--)", ""));
	PKCS8EncodedKeySpec pkcs8KeySpec = new PKCS8EncodedKeySpec(keyBytes);
	KeyFactory keyFactory = KeyFactory.getInstance("RSA");
	PrivateKey priKey = keyFactory.generatePrivate(pkcs8KeySpec);
	Signature signature = Signature.getInstance("SHA256withRSA");
	signature.initSign(priKey);
	signature.update(content.getBytes());
	return Base64.getEncoder().encodeToString(signature.sign());
}
  

```

## golang
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
func sign(body string, appId string, timestamp string, nonce string, rsaPrivateKey string) (string, error) {
	sha := sha256.New()
	sha.Write([]byte(body + appId + timestamp + nonce))
	hashBytes := sha.Sum(nil)
	privBlock, _ := pem.Decode([]byte(rsaPrivateKey))
	if privBlock == nil {
		return "", errors.New("invalid private key")
	}
	isPkcs1 := strings.Contains(strings.ToUpper(rsaPrivateKey), " RSA ")
	var privKey *rsa.PrivateKey
	if isPkcs1 {
		privateKey, err := x509.ParsePKCS1PrivateKey(privBlock.Bytes)
		if err != nil {
			return "", err
		}
		privKey = privateKey
	} else {
		privateKey, err := x509.ParsePKCS8PrivateKey(privBlock.Bytes)
		if err != nil {
			return "", err
		}
		privKey = privateKey.(*rsa.PrivateKey)
	}
	signature, err := rsa.SignPKCS1v15(cryptoRand.Reader, privKey, crypto.SHA256, hashBytes)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(signature), nil
}
  

```

## python
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
// pip3 install cryptography
import base64

from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives.serialization import load_pem_private_key
from cryptography.hazmat.backends import default_backend

def sign(body, appId, timestamp, nonce, rsaPrivateKey):
    key = load_pem_private_key(bytes(rsaPrivateKey, "UTF-8"), password=None, backend=default_backend())
    sign = key.sign(
        bytes(body + appId + timestamp + nonce, "UTF-8"),
        padding.PKCS1v15(),
        hashes.SHA256()
    )
    return base64.b64encode(sign)
  

```

## nodejs
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
const crypto = require("crypto");

function createSign(body, appId, timestamp, nonce, rsaPrivateKey) {
    const sign = crypto.createSign('RSA-SHA256');
    sign.update(body + appId + timestamp + nonce);
    sign.end();
    return sign.sign(rsaPrivateKey).toString('base64')
}
  

```

## php
bash c cpp csharp css go java javascript json kotlin less makefile markdown objectivec php plaintext python rust scss shell sql swift typescript xml yaml

```
function sign($body, $appId, $timestamp, $nonce, $rsaPrivateKey){
    $privateKey = openssl_pkey_get_private($rsaPrivateKey);
    if (!$privateKey) {
        throw new Exception("invalid private key");
    }
    openssl_sign($body . $appId . $timestamp . $nonce, $sign, $privateKey, OPENSSL_ALGO_SHA256);
    return base64_encode($sign);
}
  

```

# 六、SDK
  * [**GO**](https://github.com/sunmi-OS/sunmi-openapi-go-sdk)
  * [**JAVA**](https://github.com/sunmi-OS/sunmi-openapi-java-sdk)
  * [**PYTHON**](https://github.com/sunmi-OS/sunmi-openapi-python-sdk)


上一篇：开发者接入能力流程
下一篇：应用公钥生成方法
