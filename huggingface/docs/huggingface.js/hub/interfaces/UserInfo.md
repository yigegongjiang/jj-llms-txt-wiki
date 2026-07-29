# Interface: UserInfo

## Properties

### canPay

• `Optional` **canPay**: `boolean`

Hugging Face field. Whether the user has a payment method set up. Needs "read-billing" scope.

#### Defined in[[canpay.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:45](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L45)

___

### email

• `Optional` **email**: `string`

OpenID Connect field, available if scope "email" was granted.

#### Defined in[[email.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:24](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L24)

___

### email\_verified

• `Optional` **email\_verified**: `boolean`

OpenID Connect field, available if scope "email" was granted.

#### Defined in[[emailverified.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:20](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L20)

___

### isPro

• **isPro**: `boolean`

Hugging Face field. Whether the user is a pro user.

#### Defined in[[ispro.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:41](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L41)

___

### name

• **name**: `string`

OpenID Connect field. The user's full name.

#### Defined in[[name.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:12](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L12)

___

### orgs

• `Optional` **orgs**: \{ `canPay?`: `boolean` ; `missingMFA?`: `boolean` ; `name`: `string` ; `pendingSSO?`: `boolean` ; `picture`: `string` ; `plan?`: `string` ; `preferred_username`: `string` ; `roleInOrg?`: `string` ; `securityRestrictions?`: (``"mfa"`` \| ``"sso"`` \| ``"ip"`` \| ``"token-policy"``)[] ; `sub`: `string`  }[]

Hugging Face field. The user's orgs

#### Defined in[[orgs.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:49](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L49)

___

### picture

• **picture**: `string`

OpenID Connect field. The user's profile picture URL.

#### Defined in[[picture.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:28](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L28)

___

### preferred\_username

• **preferred\_username**: `string`

OpenID Connect field. The user's username.

#### Defined in[[preferredusername.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:16](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L16)

___

### profile

• **profile**: `string`

OpenID Connect field. The user's profile URL.

#### Defined in[[profile.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:32](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L32)

___

### sub

• **sub**: `string`

OpenID Connect field. Unique identifier for the user, even in case of rename.

#### Defined in[[sub.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:8](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L8)

___

### website

• `Optional` **website**: `string`

OpenID Connect field. The user's website URL.

#### Defined in[[website.defined-in]]

[packages/hub/src/lib/oauth-handle-redirect.ts:36](https://github.com/huggingface/huggingface.js/blob/main/packages/hub/src/lib/oauth-handle-redirect.ts#L36)
