---
title: List dispatch namespaces
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Workers For Platforms](https://developers.cloudflare.com/api/resources/workers%5Ffor%5Fplatforms)

[Dispatch](https://developers.cloudflare.com/api/resources/workers%5Ffor%5Fplatforms/subresources/dispatch)

[Namespaces](https://developers.cloudflare.com/api/resources/workers%5Ffor%5Fplatforms/subresources/dispatch/subresources/namespaces)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# List dispatch namespaces

GET/accounts/{account\_id}/workers/dispatch/namespaces

Fetch a list of Workers for Platforms namespaces.

##### Security

API Token

The preferred authorization scheme for interacting with the Cloudflare API. [Create a token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/).

**Example:**`Authorization: Bearer Sn3lZJTBX6kkg7OdcBUAxOO963GEIyGQqnFTOFYY`

API Email + API Key

The previous authorization scheme for interacting with the Cloudflare API, used in conjunction with a Global API key.

**Example:**`X-Auth-Email: user@example.com`

The previous authorization scheme for interacting with the Cloudflare API. When possible, use API tokens instead of Global API keys.

**Example:**`X-Auth-Key: 144c9defac04969c7bfad8efaa8ea194`

##### Accepted Permissions (at least one required)

`Workers Tail Read` `Workers Scripts Write` `Workers Scripts Read`

##### Path ParametersExpand Collapse 

account\_id: string

Identifier.

maxLength32

##### ReturnsExpand Collapse 

errors: array of object { code, message, documentation\_url, source } 

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

messages: array of object { code, message, documentation\_url, source } 

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

success: true

Whether the API call was successful.

result: optional array of object { created\_by, created\_on, modified\_by, 5 more } 

created\_by: optional string

Identifier.

maxLength32

created\_on: optional string

When the script was created.

formatdate-time

modified\_by: optional string

Identifier.

maxLength32

modified\_on: optional string

When the script was last modified.

formatdate-time

namespace\_id: optional string

API Resource UUID tag.

maxLength36

namespace\_name: optional string

Name of the Workers for Platforms dispatch namespace.

script\_count: optional number

The current number of scripts in this Dispatch Namespace.

trusted\_workers: optional boolean

Whether the Workers in the namespace are executed in a “trusted” manner. When a Worker is trusted, it has access to the shared caches for the zone in the Cache API, and has access to the `request.cf` object on incoming Requests. When a Worker is untrusted, caches are not shared across the zone, and `request.cf` is undefined. By default, Workers in a namespace are “untrusted”.

### List dispatch namespaces

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workers/dispatch/namespaces \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

200 example

```
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": [
    {
      "created_by": "023e105f4ecef8ad9ca31a8372d0c353",
      "created_on": "2017-01-01T00:00:00Z",
      "modified_by": "023e105f4ecef8ad9ca31a8372d0c353",
      "modified_on": "2017-01-01T00:00:00Z",
      "namespace_id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
      "namespace_name": "my-dispatch-namespace",
      "script_count": 800,
      "trusted_workers": false
    }
  ]
}
```

##### Returns Examples

200 example

```
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "success": true,
  "result": [
    {
      "created_by": "023e105f4ecef8ad9ca31a8372d0c353",
      "created_on": "2017-01-01T00:00:00Z",
      "modified_by": "023e105f4ecef8ad9ca31a8372d0c353",
      "modified_on": "2017-01-01T00:00:00Z",
      "namespace_id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
      "namespace_name": "my-dispatch-namespace",
      "script_count": 800,
      "trusted_workers": false
    }
  ]
}
```