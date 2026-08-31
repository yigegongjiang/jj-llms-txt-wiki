---
title: KV
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

# KV

#### KVNamespaces

##### [List Namespaces](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/list)

GET/accounts/{account\_id}/storage/kv/namespaces

##### [Get a Namespace](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/get)

GET/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}

##### [Create a Namespace](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/create)

POST/accounts/{account\_id}/storage/kv/namespaces

##### [Rename a Namespace](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/update)

PUT/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}

##### [Remove a Namespace](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/delete)

DELETE/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}

##### [Write multiple key-value pairs](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk%5Fupdate)

PUT/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/bulk

##### [Delete multiple key-value pairs](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk%5Fdelete)

POST/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/bulk/delete

##### [Get multiple key-value pairs](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/bulk%5Fget)

POST/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/bulk/get

##### ModelsExpand Collapse 

Namespace object { id, title, jurisdiction, supports\_url\_encoding } 

id: string

Namespace identifier tag.

maxLength32

title: string

A human-readable string name for a Namespace.

maxLength512

jurisdiction: optional "eu" or "fedramp" or "us"

Specify the jurisdiction to restrict the KV namespace to durably store data within. Can only be set at namespace creation time.

One of the following:

"eu"

"fedramp"

"us"

supports\_url\_encoding: optional boolean

True if keys written on the URL will be URL-decoded before storing. For example, if set to “true”, a key written on the URL as “%3F” will be stored as ”?”.

NamespaceDeleteResponse object { } 

NamespaceBulkUpdateResponse object { successful\_key\_count, unsuccessful\_keys } 

successful\_key\_count: optional number

Number of keys successfully updated.

unsuccessful\_keys: optional array of string

Name of the keys that failed to be fully updated. They should be retried.

NamespaceBulkDeleteResponse object { successful\_key\_count, unsuccessful\_keys } 

successful\_key\_count: optional number

Number of keys successfully updated.

unsuccessful\_keys: optional array of string

Name of the keys that failed to be fully updated. They should be retried.

NamespaceBulkGetResponse \= object { values } or object { values } 

One of the following:

WorkersKVBulkGetResult object { values } 

values: optional map\[string or number or boolean or map\[unknown\]\]

Requested keys are paired with their values in an object.

One of the following:

string

number

boolean

map\[unknown\]

WorkersKVBulkGetResultWithMetadata object { values } 

values: optional map\[object { metadata, value, expiration } \]

Requested keys are paired with their values and metadata in an object.

metadata: unknown

The metadata associated with the key.

value: unknown

The value associated with the key.

expiration: optional number

Expires the key at a certain time, measured in number of seconds since the UNIX epoch.

#### KVNamespacesKeys

##### [List a Namespace's Keys](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/keys/methods/list)

GET/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/keys

##### [Write multiple key-value pairs](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/keys/methods/bulk%5Fupdate)

Deprecated

PUT/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/bulk

##### [Delete multiple key-value pairs](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/keys/methods/bulk%5Fdelete)

Deprecated

POST/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/bulk/delete

##### [Get multiple key-value pairs](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/keys/methods/bulk%5Fget)

Deprecated

POST/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/bulk/get

##### ModelsExpand Collapse 

Key object { name, expiration, metadata } 

A name for a value. A value stored under a given key may be retrieved via the same key.

name: string

A key’s name. The name may be at most 512 bytes. All printable, non-whitespace characters are valid. Use percent-encoding to define key names as part of a URL.

maxLength512

expiration: optional number

The time, measured in number of seconds since the UNIX epoch, at which the key will expire. This property is omitted for keys that will not expire.

metadata: optional unknown

Arbitrary JSON that is associated with a key.

KeyBulkUpdateResponse object { successful\_key\_count, unsuccessful\_keys } 

successful\_key\_count: optional number

Number of keys successfully updated.

unsuccessful\_keys: optional array of string

Name of the keys that failed to be fully updated. They should be retried.

KeyBulkDeleteResponse object { successful\_key\_count, unsuccessful\_keys } 

successful\_key\_count: optional number

Number of keys successfully updated.

unsuccessful\_keys: optional array of string

Name of the keys that failed to be fully updated. They should be retried.

KeyBulkGetResponse \= object { values } or object { values } 

One of the following:

WorkersKVBulkGetResult object { values } 

values: optional map\[string or number or boolean or map\[unknown\]\]

Requested keys are paired with their values in an object.

One of the following:

string

number

boolean

map\[unknown\]

WorkersKVBulkGetResultWithMetadata object { values } 

values: optional map\[object { metadata, value, expiration } \]

Requested keys are paired with their values and metadata in an object.

metadata: unknown

The metadata associated with the key.

value: unknown

The value associated with the key.

expiration: optional number

Expires the key at a certain time, measured in number of seconds since the UNIX epoch.

#### KVNamespacesMetadata

##### [Read the metadata for a key](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/metadata/methods/get)

GET/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/metadata/{key\_name}

##### ModelsExpand Collapse 

MetadataGetResponse \= unknown

Arbitrary JSON that is associated with a key.

#### KVNamespacesValues

##### [Read key-value pair](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/get)

GET/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/values/{key\_name}

##### [Write key-value pair with optional metadata](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/update)

PUT/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/values/{key\_name}

##### [Delete key-value pair](https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/values/methods/delete)

DELETE/accounts/{account\_id}/storage/kv/namespaces/{namespace\_id}/values/{key\_name}

##### ModelsExpand Collapse 

ValueUpdateResponse object { } 

ValueDeleteResponse object { }