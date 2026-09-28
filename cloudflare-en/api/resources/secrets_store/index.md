---
title: Secrets Store
---

[Skip to content](#_top)

[API Reference](https://developers.cloudflare.com/api)

Copy Markdown

Open in **Claude**Open in **ChatGPT**Open in **Cursor**

---

**Copy Markdown****View as Markdown**

# Secrets Store

#### Secrets StoreStores

##### [List account stores](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/methods/list)

GET/accounts/{account\_id}/secrets\_store/stores

##### [Get a store by ID](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/methods/get)

GET/accounts/{account\_id}/secrets\_store/stores/{store\_id}

##### [Create a store](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/methods/create)

POST/accounts/{account\_id}/secrets\_store/stores

##### [Delete a store](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/methods/delete)

DELETE/accounts/{account\_id}/secrets\_store/stores/{store\_id}

##### ModelsExpand Collapse

<details>

<summary>

StoreListResponse object {id, created, modified, 2 more }

</summary>

id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_list_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_list_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the store.

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_list_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

account\_id: optional string

Account Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_list_response%20%3E%20(schema)%20%3E%20(property)%20account_id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores%20%3E%20(model)%20store_list_response%20%3E%20(schema)>)

<details>

<summary>

StoreGetResponse object {id, created, modified, 2 more }

</summary>

id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_get_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_get_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the store.

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_get_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

account\_id: optional string

Account Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_get_response%20%3E%20(schema)%20%3E%20(property)%20account_id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores%20%3E%20(model)%20store_get_response%20%3E%20(schema)>)

<details>

<summary>

StoreCreateResponse object {id, created, modified, 2 more }

</summary>

id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_create_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_create_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the store.

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_create_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

account\_id: optional string

Account Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores%20%3E%20(model)%20store_create_response%20%3E%20(schema)%20%3E%20(property)%20account_id">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores%20%3E%20(model)%20store_create_response%20%3E%20(schema)>)

StoreDeleteResponse = unknown

Result is null for delete operations.

[Link to this property](<#(resource)%20secrets_store.stores%20%3E%20(model)%20store_delete_response%20%3E%20(schema)>)

#### Secrets StoreStoresSecrets

##### [List store secrets](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/list)

GET/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets

##### [Get a secret by ID](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/get)

GET/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}

##### [Create a secret](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/create)

POST/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets

##### [Patch a secret](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/edit)

PATCH/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}

##### [Delete a secret](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/delete)

DELETE/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}

##### [Delete secrets](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/bulk_delete)

DELETE/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets

##### [Duplicate Secret](https://developers.cloudflare.com/api/resources/secrets_store/subresources/stores/subresources/secrets/methods/duplicate)

POST/accounts/{account\_id}/secrets\_store/stores/{store\_id}/secrets/{secret\_id}/duplicate

##### ModelsExpand Collapse

<details>

<summary>

SecretListResponse object {id, created, modified, 5 more }

</summary>

id: string

Secret identifier tag.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

status: "pending"or "active"or "deleted"

</summary>

One of the following:

"pending"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"active"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"deleted"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

store\_id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20store_id">Link to this property</a>

comment: optional string

Freeform text describing the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

scopes: optional array of "workers"or "ai\_gateway"or "dex"or 3 more

The list of services that can use this secret.

</summary>

One of the following:

"workers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"ai\_gateway"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"dex"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"access"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"containers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"websearch"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)%20%3E%20(property)%20scopes">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_list_response%20%3E%20(schema)>)

<details>

<summary>

SecretGetResponse object {id, created, modified, 5 more }

</summary>

id: string

Secret identifier tag.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

status: "pending"or "active"or "deleted"

</summary>

One of the following:

"pending"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"active"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"deleted"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

store\_id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20store_id">Link to this property</a>

comment: optional string

Freeform text describing the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

scopes: optional array of "workers"or "ai\_gateway"or "dex"or 3 more

The list of services that can use this secret.

</summary>

One of the following:

"workers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"ai\_gateway"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"dex"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"access"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"containers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"websearch"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)%20%3E%20(property)%20scopes">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_get_response%20%3E%20(schema)>)

<details>

<summary>

SecretCreateResponse object {id, created, modified, 5 more }

</summary>

id: string

Secret identifier tag.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

status: "pending"or "active"or "deleted"

</summary>

One of the following:

"pending"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"active"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"deleted"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

store\_id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20store_id">Link to this property</a>

comment: optional string

Freeform text describing the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

scopes: optional array of "workers"or "ai\_gateway"or "dex"or 3 more

The list of services that can use this secret.

</summary>

One of the following:

"workers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"ai\_gateway"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"dex"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"access"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"containers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"websearch"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)%20%3E%20(property)%20scopes">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_create_response%20%3E%20(schema)>)

<details>

<summary>

SecretEditResponse object {id, created, modified, 5 more }

</summary>

id: string

Secret identifier tag.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

status: "pending"or "active"or "deleted"

</summary>

One of the following:

"pending"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"active"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"deleted"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

store\_id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20store_id">Link to this property</a>

comment: optional string

Freeform text describing the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

scopes: optional array of "workers"or "ai\_gateway"or "dex"or 3 more

The list of services that can use this secret.

</summary>

One of the following:

"workers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"ai\_gateway"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"dex"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"access"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"containers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"websearch"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)%20%3E%20(property)%20scopes">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_edit_response%20%3E%20(schema)>)

SecretDeleteResponse = unknown

Result is null for delete operations.

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_delete_response%20%3E%20(schema)>)

SecretBulkDeleteResponse = unknown

Result is null for delete operations.

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_bulk_delete_response%20%3E%20(schema)>)

<details>

<summary>

SecretDuplicateResponse object {id, created, modified, 5 more }

</summary>

id: string

Secret identifier tag.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20id">Link to this property</a>

created: string

When the secret was created.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20created">Link to this property</a>

modified: string

When the secret was modified.

formatdate-time

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20modified">Link to this property</a>

name: string

The name of the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20name">Link to this property</a>

<details>

<summary>

status: "pending"or "active"or "deleted"

</summary>

One of the following:

"pending"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%200">Link to this property</a>

"active"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%201">Link to this property</a>

"deleted"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20status%20%3E%20(member)%202">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20status">Link to this property</a>

store\_id: string

Store Identifier.

maxLength32

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20store_id">Link to this property</a>

comment: optional string

Freeform text describing the secret.

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20comment">Link to this property</a>

<details>

<summary>

scopes: optional array of "workers"or "ai\_gateway"or "dex"or 3 more

The list of services that can use this secret.

</summary>

One of the following:

"workers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%200">Link to this property</a>

"ai\_gateway"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%201">Link to this property</a>

"dex"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%202">Link to this property</a>

"access"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%203">Link to this property</a>

"containers"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%204">Link to this property</a>

"websearch"

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes%20%3E%20(items)%20%3E%20(member)%205">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)%20%3E%20(property)%20scopes">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.stores.secrets%20%3E%20(model)%20secret_duplicate_response%20%3E%20(schema)>)

#### Secrets StoreQuota

##### [View secret usage](https://developers.cloudflare.com/api/resources/secrets_store/subresources/quota/methods/get)

GET/accounts/{account\_id}/secrets\_store/quota

##### ModelsExpand Collapse

<details>

<summary>

QuotaGetResponse object {secrets }

</summary>

<details>

<summary>

secrets: object {quota, usage }

</summary>

quota: number

The number of secrets the account is entitled to use.

<a href="#(resource)%20secrets_store.quota%20%3E%20(model)%20quota_get_response%20%3E%20(schema)%20%3E%20(property)%20secrets%20%3E%20(property)%20quota">Link to this property</a>

usage: number

The number of secrets the account is currently using.

<a href="#(resource)%20secrets_store.quota%20%3E%20(model)%20quota_get_response%20%3E%20(schema)%20%3E%20(property)%20secrets%20%3E%20(property)%20usage">Link to this property</a>

</details>

<a href="#(resource)%20secrets_store.quota%20%3E%20(model)%20quota_get_response%20%3E%20(schema)%20%3E%20(property)%20secrets">Link to this property</a>

</details>

[Link to this property](<#(resource)%20secrets_store.quota%20%3E%20(model)%20quota_get_response%20%3E%20(schema)>)