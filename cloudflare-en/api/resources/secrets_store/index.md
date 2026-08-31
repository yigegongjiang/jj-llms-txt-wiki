---
title: Secrets Store
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# Secrets Store

#### Secrets StoreStores

##### [List account stores](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/methods/list)

GET/accounts/{account\_id}/secrets\_store/stores

##### [Get a store by ID](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/methods/get)

GET/accounts/{account\_id}/secrets\_store/stores/{store\_id}

##### [Create a store](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/methods/create)

POST/accounts/{account\_id}/secrets\_store/stores

##### [Delete a store](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/methods/delete)

DELETE/accounts/{account\_id}/secrets\_store/stores/{store\_id}

##### ModelsExpand Collapse 

StoreListResponse object { id, created, modified, 2 more } 

id: string

Store Identifier.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the store.

account\_id: optional string

Account Identifier.

maxLength32

StoreGetResponse object { id, created, modified, 2 more } 

id: string

Store Identifier.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the store.

account\_id: optional string

Account Identifier.

maxLength32

StoreCreateResponse object { id, created, modified, 2 more } 

id: string

Store Identifier.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the store.

account\_id: optional string

Account Identifier.

maxLength32

StoreDeleteResponse \= unknown

Result is null for delete operations.

#### Secrets StoreStoresSecrets

##### [List store secrets](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/list)

GET/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets

##### [Get a secret by ID](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/get)

GET/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}

##### [Create a secret](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/create)

POST/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets

##### [Patch a secret](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/edit)

PATCH/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}

##### [Delete a secret](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/delete)

DELETE/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}

##### [Delete secrets](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/bulk%5Fdelete)

DELETE/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets

##### [Duplicate Secret](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/stores/subresources/secrets/methods/duplicate)

POST/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}/duplicate

##### ModelsExpand Collapse 

SecretListResponse object { id, created, modified, 5 more } 

id: string

Secret identifier tag.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the secret.

status: "pending" or "active" or "deleted"

One of the following:

"pending"

"active"

"deleted"

store\_id: string

Store Identifier.

maxLength32

comment: optional string

Freeform text describing the secret.

scopes: optional array of "workers" or "ai\_gateway" or "dex" or 3 more

The list of services that can use this secret.

One of the following:

"workers"

"ai\_gateway"

"dex"

"access"

"containers"

"websearch"

SecretGetResponse object { id, created, modified, 5 more } 

id: string

Secret identifier tag.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the secret.

status: "pending" or "active" or "deleted"

One of the following:

"pending"

"active"

"deleted"

store\_id: string

Store Identifier.

maxLength32

comment: optional string

Freeform text describing the secret.

scopes: optional array of "workers" or "ai\_gateway" or "dex" or 3 more

The list of services that can use this secret.

One of the following:

"workers"

"ai\_gateway"

"dex"

"access"

"containers"

"websearch"

SecretCreateResponse object { id, created, modified, 5 more } 

id: string

Secret identifier tag.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the secret.

status: "pending" or "active" or "deleted"

One of the following:

"pending"

"active"

"deleted"

store\_id: string

Store Identifier.

maxLength32

comment: optional string

Freeform text describing the secret.

scopes: optional array of "workers" or "ai\_gateway" or "dex" or 3 more

The list of services that can use this secret.

One of the following:

"workers"

"ai\_gateway"

"dex"

"access"

"containers"

"websearch"

SecretEditResponse object { id, created, modified, 5 more } 

id: string

Secret identifier tag.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the secret.

status: "pending" or "active" or "deleted"

One of the following:

"pending"

"active"

"deleted"

store\_id: string

Store Identifier.

maxLength32

comment: optional string

Freeform text describing the secret.

scopes: optional array of "workers" or "ai\_gateway" or "dex" or 3 more

The list of services that can use this secret.

One of the following:

"workers"

"ai\_gateway"

"dex"

"access"

"containers"

"websearch"

SecretDeleteResponse \= unknown

Result is null for delete operations.

SecretBulkDeleteResponse \= unknown

Result is null for delete operations.

SecretDuplicateResponse object { id, created, modified, 5 more } 

id: string

Secret identifier tag.

maxLength32

created: string

When the secret was created.

formatdate-time

modified: string

When the secret was modified.

formatdate-time

name: string

The name of the secret.

status: "pending" or "active" or "deleted"

One of the following:

"pending"

"active"

"deleted"

store\_id: string

Store Identifier.

maxLength32

comment: optional string

Freeform text describing the secret.

scopes: optional array of "workers" or "ai\_gateway" or "dex" or 3 more

The list of services that can use this secret.

One of the following:

"workers"

"ai\_gateway"

"dex"

"access"

"containers"

"websearch"

#### Secrets StoreQuota

##### [View secret usage](https://developers.cloudflare.com/api/resources/secrets%5Fstore/subresources/quota/methods/get)

GET/accounts/{account\_id}/secrets\_store/quota

##### ModelsExpand Collapse 

QuotaGetResponse object { secrets } 

secrets: object { quota, usage } 

quota: number

The number of secrets the account is entitled to use.

usage: number

The number of secrets the account is currently using.