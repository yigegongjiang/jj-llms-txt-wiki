---
url: https://docs.sunmi.com/zh-CN/cicmeghjk546/xqfmeghjk546
---

# 单点登录SSO
更新时间：2025-02-24 12:27:19
# 名词解释  
| 概念  | 说明  |  
| --- | --- |  
| IdP（Identity Provider 身份提供商）  | 一个包含有关外部身份提供商元数据的实体，身份提供商可以提供身份管理服务。 企业本地 IdP：Microsoft Active Directory Federation Service （ADFS）、Shibboleth 等。 Cloud IdP：Azure AD、Google Workspace、Okta、OneLogin 等。  |  
| SP（Service Provider服务提供商）  | 利用 IdP 的身份管理功能，为用户提供具体服务的应用，SP 会使用 IdP 提供的用户信息。一些非 SAML 协议的身份系统（例如：OpenID Connect），也把服务提供商称作 IdP 的信赖方。  |  
| SAML 2.0（安全断言标记语言）  | 实现企业级用户身份认证的标准协议，它是 SP 和 IdP 之间实现沟通的技术实现方式之一。SAML 2.0已经是目前实现企业级 SSO 的一种事实标准。  |  
| DefaultRelayState  | 登录重定向到 SP 的页面  |  
| SAML 断言（SAML assertion）  | SAML 协议中用来描述认证请求和认证响应的核心元素。例如：用户的具体属性就包含在认证响应的断言里。  |  
| 信赖（Trust）  | 建立在 SP 和 IdP 之间的互信机制，通常由公钥和私钥来实现。SP 通过可信的方式获取 IdP 的 SAML元数据，元数据中包含 IdP 签发 SAML 断言的签名验证公钥，SP 则使用公钥来验证断言的完整性。  |  
| OIDC（OpenID Connect）  | OIDC（OpenID Connect）是建立在 OAuth 2.0基础上的一个认证协议。 OAuth 是授权协议，而 OIDC 在 OAuth 协议上构建了一层身份层，除了 OAuth 提供的授权能力，它还允许客户端能够验证终端用户的身份，以及通过 OIDC 协议的 API（HTTP RESTful 形式）获取用户的基本信息。  |  
| OIDC 令牌  | OIDC 可以给应用签发代表登录用户的身份令牌，即 OIDC 令牌（OIDC Token）。OIDC 令牌用于获取登录用户的基本信息。  |  
| 客户端 ID  | 您的应用在外部 IdP 注册的时候，会生成一个客户端 ID（Client ID）。当您从外部 IdP 申请签发 OIDC 令牌时必须使用该客户端 ID，签发出来的 OIDC 令牌也会通过 aud 字段携带该客户端 ID。在创建 OIDC 身份提供商时配置该客户端 ID，然后在使用 OIDC 令牌换取 STS Token 时，PTN 会校验OIDC 令牌中 aud 字段所携带的客户端 ID 与 OIDC 身份提供商中配置的客户端 ID 是否一致。只有一致时，才允许扮演角色。  |  
| 验证指纹  | 为了防止颁发者 URL 被恶意劫持或篡改，您需要配置外部 IdP 的 HTTPS CA 证书生成的验证指纹。PTN 云会辅助您自动计算该验证指纹，但是建议您在本地自己计算一次（例如：使用 OpenSSL 计算指纹），与 PTN 计算的指纹进行对比。如果对比发现不同，则说明该颁发者 URL 可能已经受到攻击，请您务必再次确认，并填写正确的指纹。  |  
| 身份提供商 URL  | OpenID Connect 身份提供商标识。 对应身份提供商提供的 OpenID Connect 元数据文档中的 "issuer" 字段值。  |  
| 映射字段  | OpenID Connect 身份提供商中与 PTN 用户名映射的字段。 可选身份提供商提供的 OpenID Connect 元数据文档中 "claims_supported" 的值，此示例中使用 name 字段映射 CAM 的 username。  |  
| 签名公钥  | 验证 OpenID Connect 身份提供商 ID Token 签名的公钥。 对应身份提供商提供的 OpenID Connect 元数据文档中 "jwks_uri" 字段中链接的内容（在浏览器中打开链接获取内容）。 为了您的账号安全，建议您定期轮换签名公钥。  |  
# SSO方式
SP通常提供SSO的对接方式：
## 用户SSO
### 概念
SP通过IdP颁发的SAML断言确定企业用户与SP 用户的对应关系 。企业用户登录后，使用该用户访问SP资源
partner与企业进行用户SSO时，partner是服务提供商（SP），而企业自有的身份管理系统（Okta）则是身份提供商（IdP）。通过用户SSO，企业员工在登录后，访问partner。
### 流程
举例：
当管理员在完成用户SSO的相关配置后，企业员工Jessica可以通过如下图所示的方法登录到partner。
![](https://cdn.sunmi.com/public/image/mgt-document/90b09624ba1748ee94d35cde3342c7ef.png)
  1. Jessica使用浏览器登录partner，partner将SAML认证请求返回给浏览器。
  2. 浏览器向IdP转发SAML认证请求。
  3. IdP提示Jessica登录，并在Jessica登录成功后生成SAML响应返回给浏览器。
  4. 浏览器将SAML响应转发给SSO服务。
  5. SSO服务通过SAML互信配置，验证SAML响应的数字签名来判断SAML断言的真伪，并通过SAML断言的`NameID`元素值，匹配到对应partner账号中的用户。
  6. SSO服务向浏览器返回控制台的URL。
  7. 浏览器重定向到partner控制台


> **说明**
> 在第1步中，企业员工从partner发起登录并不是必须的。企业员工也可以在企业自有IdP（Okta）的登录页直接单击登录到partner的链接，向企业IdP发出登录到partner的SAML认证请求。
## 角色SSO
### 概念
基于SAML 2.0角色SSO：
  * SAML角色SSO：SP通过IdP颁发的SAML断言确定企业用户在SP上可以使用的角色。企业用户登录后，使用SAML断言中指定的角色访问SP 资源。


partner与企业进行角色SSO时，partner是服务提供商（SP），而企业自有的身份管理系统则是身份提供商（IdP）。通过角色SSO，企业可以在本地IdP中管理员工信息，无需进行partner和企业IdP间的用户同步，企业员工将使用指定的角色登录partner。
### 流程
企业员工可以通过控制台或程序访问partner。
  * 通过控制台访问partner


当管理员在完成角色SSO的相关配置后，企业员工Jessica可以通过如下图所示的方法登录到partner。
  

![](https://cdn.sunmi.com/public/image/mgt-document/1cf3351fb61245f1b99ad02ef6851b4c.png)
  1. Jessica使用浏览器在IdP的登录页面中选择partner作为目标服务。


例如：如果企业IdP使用AD FS（Microsoft Active Directory Federation Service），则登录URL为：`https://ADFSServiceName/adfs/ls/IdpInitiatedSignOn.aspx`。
> **说明**
> 有些IdP会要求用户先登录，再选择代表partner的SSO应用。
  1. IdP生成一个SAML响应并返回给浏览器。
  2. 浏览器重定向到SSO服务页面，并转发SAML响应给SSO服务。
  3. SSO服务使用SAML响应向partnerSTS服务请求临时安全凭证，并生成一个可以使用临时安全凭证登录partner控制台的URL。


> **说明**
> 如果SAML响应中包含映射到多个角色的属性，系统将会首先提示用户选择一个用于访问partner的角色。
  1. SSO服务将URL返回给浏览器。
  2. 浏览器重定向到该URL，以指定角色登录到partner控制台。


# SSO方式比较  
| SSO方式  | SP发起的SSO  | IdP发起的SSO  | 使用账号和密码登录  | 一次性配置IdP关联多个Partner账号  | 多个IdP  |  
| --- | --- | --- | --- | --- | --- |  
| 用户SSO  | 支持  | 支持  | 不支持  | 不支持  | 不支持  |  
| 角色SSO  | 不支持  | 支持  | 支持  | 支持  | 支持  |  
## 适用场景
### 角色SSO
角色SSO适用于以下场景：
  * 出于管理成本考虑，您不希望在云端创建和管理用户，从而避免用户同步带来的工作量。
  * 您希望在使用SSO的同时，仍然保留一部分云上本地用户，可以在partner直接登录。云上本地用户的用途可以是新功能测试、网络或企业IdP出现问题时的备用登录方式等。
  * 您希望根据用户在本地IdP中加入的组或者用户的某个特殊属性，来区分云上拥有的权限。当进行权限调整时，只需要在本地进行分组或属性的更改。
  * 您拥有多个partner账号但使用统一的企业IdP，希望在企业IdP配置一次，就可以实现到多个partner账号的SSO。
  * 您的各个分支机构存在多个IdP，都需要访问同一个partner账号，您需要在一个partner账号内配置多个IdP进行SSO。
  * 除了控制台，您也希望使用程序访问的方式来进行SSO。


### 用户SSO
用户SSO适用于以下场景：
  * 您希望从partner的登录页面开始发起登录，而非直接访问您IdP的登录页面。
  * 您希望适用的产品的部分功能不是通过角色来分配权限而是直接指定用户。
  * 您的IdP不支持复杂的自定义属性配置。
  * 您没有上述需要使用角色SSO的业务需求，而又希望尽量简化IdP配置。


# 产品方案
  * 提供按用户和按角色配置SSO两种方式


## S1按角色配置方式
概述：在Partner创建SSO专用角色，在Okta配置并绑定到该角色，Okta用户登录时会自动使用该角色进入partner。
  

![](https://cdn.sunmi.com/public/image/mgt-document/77b4d5881c7f468b94622bada0e715ce.png)
### 步骤一：在Okta创建支持SAML SSO的应用
  1. 登录Okta门户
  2. 单机页面右上方[管理员]按钮，进入管理员控制台
![](https://cdn.sunmi.com/public/image/mgt-document/73a4c47c747c4492b126f4976c8b96bf.png)
  3. 在左侧导航栏，选择Application->Applications，并点击Create App Intergration
![](https://cdn.sunmi.com/public/image/mgt-document/9b074f63b38443a5a33e9efbdb59a09b.png)
  4. 在**Create a new app integration** 对话框，单击**SAML 2.0** ，然后单击**Next**
![](https://cdn.sunmi.com/public/image/mgt-document/f3653e7bdbb84a2d9656c468fab17802.png)
  5. 配置应用名，如SunmiPartner_RoleAdmin，单击Next
![](https://cdn.sunmi.com/public/image/mgt-document/c37fba9c024b4817a598c99a3e0ef0cd.png)
  6. 配置SAML，然后单击Next


  * **Single sign-on URL** ：`https://webapi.sunmi.com/public/v1/partner/sso/saml`


> 说明：根据客户所在云不同，需要使用不同的参数，详见下方列表
  * **Audience URI (SP Entity ID)** ：`https://webapi.sunmi.com`


> 说明：根据客户所在云不同，需要使用不同的参数，详见下方列表
  * **Default RelayState** ：保持默认值
  * **Name ID format** ：保持默认值
  * **Application username** ：选择**Email**
  * **Update application username on** ：保持默认值
![](https://cdn.sunmi.com/public/image/mgt-document/79e7ffa60f4b4980b8c6ce1c110dfd59.png)

  
| key  | value  | 说明  |  
| --- | --- | --- |  
| Single sign-on URL  | <https://webapi.sunmi.com/public/v1/partner/sso/saml>  | user in [partner.sunmi.com](http://partner.sunmi.com)  |  
|   
 | <https://webapi.us.sunmi.com/public/v1/partner/sso/saml>  | user in [partner.us.sunmi.com](http://partner.us.sunmi.com)  |  
|   
 | <https://webapi.eu.sunmi.com/public/v1/partner/sso/saml>  | user in [partner.eu.sunmi.com](http://partner.eu.sunmi.com)  |  
| Audience URI (SP Entity ID)  | <https://webapi.sunmi.com>  | user in [partner.sunmi.com](http://partner.sunmi.com)  |  
|   
 | <https://webapi.us.sunmi.com>  | user in [partner.us.sunmi.com](http://partner.us.sunmi.com)  |  
|   
 | <https://webapi.eu.sunmi.com>  | user in [partner.eu.sunmi.com](http://partner.eu.sunmi.com)  |  
| EntityID  |  ![](https://cdn.sunmi.com/public/image/mgt-document/43aeb2b5a2404af4a54e0b78b76f9d4c.png)  | 主体ID  |  
| RoleName  | 此处需要填写您将会在Partner平台为OKTA用户创建的角色名，如Admin。在步骤四中创建的角色名必须和此选项填入内容一致  | 角色名  |  
  1. 在**Feedback** 页面，根据需要选择合适的应用类型，然后单击**Finish** 。


### 步骤二：在Okta获取SAML Idp元数据
  1. 在Okta创建的应用程序SunmiPartner_RoleAdmin详情页，点击Sign On页签
  2. 在**SAML 2.0** 区域，复制**Metadata URL** ，在新的浏览器页面中复制该URL，并将（IdP元）数据另存到本地


  

![](https://cdn.sunmi.com/public/image/mgt-document/4ce9401d993c4ad0b0074f262989b5ed.png)
### 步骤三：在Partner创建SAML身份提供商
  1. 使用管理员帐号登录Partner
  2. 在左侧导航栏，选择**系统管理- >SSO管理**，或者在页面右上角点击头像，在企业信息显示弹窗中点击SSO
![](https://cdn.sunmi.com/public/image/mgt-document/da7667f003cc4fa1b4b64a480e7f5046.png)
  3. 在SSO管理页面，选择**角色SSO** ，点击**创建身份提供商**
![](https://cdn.sunmi.com/public/image/mgt-document/78bb55dc864b45bfada4c88303306e0f.png)
  4. 在创建**身份提供商** 弹窗，输入**身份提供商名称** （okta-provider）和**备注，并在元数据文档** 区域，上传从步骤二中获取的元数据
![](https://cdn.sunmi.com/public/image/mgt-document/7f0a4f3895bb4d6f92eeef8c858e5119.png)


### 步骤四：在Partner创建角色
  1. 在Partner控制台的左侧导航栏，选择**系统管理 > 角色**。
  2. 在**角色** 页面，单击**创建角色，选择** SSO用户角色
![](https://cdn.sunmi.com/public/image/mgt-document/e138e8ab22a54786b61feab066b4b9c7.png)


### 步骤五：在Okta创建用户并分配应用
  1. 创建用户。
    1. 在Okta左侧导航栏，选择**Directory > People**。
    2. 单击**Add person** 。
    3. 在**Add Person** 页面，填写基本信息并将**Primary email** 配置为被邀请用户的Email，例如：username@example.com，然后单击**Save** 。
    4. 在用户列表中，单击用户username@example.com **Status** 列的**Activate** ，然后根据页面提示激活username@example.com。
  2. 分配应用。


分配应用有以下两种方式，请任选其一。
  * 为单个用户分配应用。
    1. 在Okta左侧导航栏，选择**Applications > Applications**。
    2. 单击目标应用名称SunmiPartner_RoleAdmin后，在**Assignments** 页签下，选择**Assign > Assign to People**。
    3. 单击目标用户username@example.com后的**Assign** 。
    4. 选择**approle** 为admin。
    5. 单击**Save and Go Back** 。
    6. 单击**Done** 。
  * 将用户加入组，为组分配应用。
    1. 在Okta左侧导航栏，先选择**Directory > Groups**，然后单击**Add Group** ，创建一个组。
    2. 单击组名称，然后单击**Manage People** ，添加用户到组中。
    3. 在Okta左侧导航栏，选择**Applications > Applications**。
    4. 单击目标应用名称**SunmiPartner_RoleAdmin** 后，在**Assignments** 页签下，单击**Assign > Assign to Groups**。
    5. 单击目标组后的**Assign** 。
    6. 选择**approle** 为admin。
    7. 单击**Save and Go Back** 。
    8. 单击**Done** 。


> **说明**
> 如果一个用户属于多个组，生效的属性值只能有一个，即在应用的Assignments页签下第一个加入的组的相应属性会生效。如果用户所属的组发生变化，则会影响approle的取值。详情请参见[Okta用户指南](https://help.okta.com/en/prod/Content/index.htm)。
### 步骤六：验证结果
  1. 在Okta左侧导航栏，选择**Applications > Applications**。
  2. 单击应用名称SunmiPartner_RoleAdmin。
  3. 在**General** 页签下的**App Embed Link** 区域，复制**Embed Link** 中的URL。


  

![](https://cdn.sunmi.com/public/image/mgt-document/2d94f3bc8e9648d8bba86d6eeb097da8.png)
  1. 打开另一个浏览器，输入获取到的URL，如果成功跳转到您设置的`Default RelayState`对应页面（或默认的partner控制台首页），则说明登录成功。


## S2按用户配置方式
概述：Okta与partner进行用户SSO的示例，帮助企业IdP与partner进行SSO的端到端配置流程。
### 步骤一：在Okta创建支持SAML SSO的应用
  1. 登录[Okta门户](https://www.okta.com/)。
  2. 单击页面右上方的账号图标，然后单击**Your Org** 。
  3. 在左侧导航栏，选择**Applications > Applications**。
  4. 在**Applications** 页面，单击**Create App Integration** 。


  

![](https://cdn.sunmi.com/public/image/mgt-document/f10a150129454bf6a94abd8e40d38527.png)
  1. 在**Create a new app integration** 对话框，单击**SAML 2.0** ，然后单击**Next** 。


  

![](https://cdn.sunmi.com/public/image/mgt-document/a9a765c2a2d14c0fbcf125bc574ff6a3.png)
  1. 配置应用名称为SunmiPartner_User，单击**Next** 。
  2. 配置SAML，然后单击**Next** 。


  

![](https://cdn.sunmi.com/public/image/mgt-document/36431b02ccda41ac85ed91a23a0ce029.png)
  * **Single sign-on URL：**`https://webapi.sunmi.com/public/v1/partner/sso/saml`
> **说明：根据客户所在云不同，需要使用不同的参数，详见下方列表**
  * **Audience URI (SP Entity ID)：**`https://webapi.sunmi.com/这里是主体EntityID/saml/sso`
> **说明：根据客户所在云不同，需要使用不同的参数，详见下方列表**
  * **Default RelayState** 用来配置用户SSO登录成功后跳转到的partner页面


> **说明：出于安全原因，您只能填写sunmi的域名URL作为** Default RelayState的值，例如：*.sunmi.com，否则配置无效。若不配置，默认跳转到partner控制台首页。
  * **Name ID format** 选择**Persistent** 。
  * **Application username** 选择**Email** 。
  * 在**Feedback** 页面，根据需要选择合适的应用类型，然后单击**Finish** 。

  
| key  | value  | 说明  |  
| --- | --- | --- |  
| Single sign-on URL  | https://webapi.sunmi.com/public/v1/partner/sso/saml  | user in [partner.sunmi.com](http://partner.sunmi.com)  |  
|   
 | <https://webapi.us.sunmi.com/public/v1/partner/sso/saml>  | user in [partner.us.sunmi.com](http://partner.us.sunmi.com)  |  
|   
 | <https://webapi.eu.sunmi.com/public/v1/partner/sso/saml>  | user in [partner.eu.sunmi.com](http://partner.eu.sunmi.com)  |  
| Audience URI (SP Entity ID)  | [https://webapi.sunmi.com/这里是主体EntityID/saml/sso](https://webapi.sunmi.com/VEZOYQ4DW8FZE/saml/sso)  | user in [partner.sunmi.com](http://partner.sunmi.com)  |  
|   
 | [https://webapi.us.sunmi.com/](https://webapi.us.sunmi.com/VEZOYQ4DW8FZE/saml/sso)[这里是主体EntityID](https://webapi.sunmi.com/VEZOYQ4DW8FZE/saml/sso)[/saml/sso](https://webapi.us.sunmi.com/VEZOYQ4DW8FZE/saml/sso)  | user in [partner.us.sunmi.com](http://partner.us.sunmi.com)  |  
|   
 | [https://webapi.eu.sunmi.com/这里是主体EntityID/saml/sso](https://webapi.eu.sunmi.com/VEZOYQ4DW8FZE/saml/sso)  | user in [partner.eu.sunmi.com](http://partner.eu.sunmi.com)  |  
| EntityID  |  ![](https://cdn.sunmi.com/public/image/mgt-document/43aeb2b5a2404af4a54e0b78b76f9d4c.png)  | 主体ID  |  
### 步骤三：在Okta获取SAML IdP元数据
  1. 在应用程序SunmiPartner_User详情页，单击**Sign On** 。
  2. 在**Settings** 区域，单击**Identity Provider metadata** ，将IdP元数据另存到本地。


  

![](https://cdn.sunmi.com/public/image/mgt-document/a749901109674aed9a8e087db41cfee5.png)
### 步骤四：在partner开启用户SSO
  1. 在partner的左侧导航栏，选择**系统管理 > SSO管理**。
  2. 在**SSO管理** 页面，单击**用户SSO** 页签。
  3. 在**SSO登录设置** 区域，单击**编辑** 。


  

![](https://cdn.sunmi.com/public/image/mgt-document/92e53bed4b5b4c20b37d2b816a37da77.png)
  1. 在弹出的**编辑SSO登录设置** 窗口的**SSO功能状态** 区域，单击**开启** 。


  

![](https://cdn.sunmi.com/public/image/mgt-document/3fbec2ef7324469ea9ac339fad3958d6.png)
> **说明：**用户SSO是一个全局功能，开启后，所有用户都需要使用SSO登录。 如果您是通过用户配置的，请先保留为关闭状态，您需要先完成用户的创建，以免配置错误导致自己无法登录。您也可以通过partner账号（主账号）进行配置来规避此问题。
  1. 在**元数据文档** 区域，单击**上传文件** ，上传从步骤三中获取的IdP元数据。
  2. 单击**确定** 。


### 步骤五：在Okta创建用户并分配应用
  1. 在Okta左侧导航栏，选择**Directory > People**。
  2. 单击**Add Person** 。


  

![](https://cdn.sunmi.com/public/image/mgt-document/5224d62c353a4fa982daa41ce193c044.png)
  1. 在**Add Person** 页面，填写基本信息并将**Primary email** 配置为u2@example.com，然后单击**Save** 。
  2. 在用户列表中，单击用户u2@example.com**Status** 列的**Activate** ，然后根据页面提示激活u2@example.com。
  3. 在左侧导航栏，选择**Applications > Applications**。
  4. 单击目标应用名称（SunmiPartner_User）后，在**Assignments** 页签，选择**Assign > Assign to People**。


  

![](https://cdn.sunmi.com/public/image/mgt-document/93b4b023595a4bb984a7445d838d5bc8.png)
  1. 单击目标用户（u2@example.com）后的**Assign** 。
  2. 单击**Save and Go Back** 。
  3. 单击**Done** 。


### 步骤六：在partner创建用户
  1. 在partner的左侧导航栏，选择系统管理 **> 用户**。
  2. 在**用户** 页面，单击添加操作员。


> **说明**
> 请确保用户的登录邮箱与Okta中的用户邮箱保持一致。
  1. 单击**发送邀请** 。
  2. 在邮箱中点击激活连接并完成注册。


### 步骤七：验证结果
完成上述配置后，您可以从partner或Okta发起SSO登录。
使用Okta用户登录[Okta门户](https://www.okta.com/)，在Okta的主页，找到并单击**SunmiPartner_User** 应用。
系统将自动SSO登录并重定向到您指定的**DefaultRelayState** 页面。如果未指定**DefaultRelayState** 或超出允许范围，则系统会访问以下partner控制台首页。如果登录到partner，表示配置成功。
上一篇：帐号管理
下一篇：用户和权限
