---
title: List Web3 Hostnames
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Web3](https://developers.cloudflare.com/api/resources/web3)

[Hostnames](https://developers.cloudflare.com/api/resources/web3/subresources/hostnames)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# List Web3 Hostnames

GET/zones/{zone\_id}/web3/hostnames

List Web3 Hostnames

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

`Web3 Hostnames Write` `Web3 Hostnames Read`

##### Path ParametersExpand Collapse 

zone\_id: string

Specify the identifier of the hostname.

maxLength32

##### ReturnsExpand Collapse 

errors: array of [ResponseInfo](https://developers.cloudflare.com/api/resources/$shared#%28resource%29%20%24shared%20%3E%20%28model%29%20response%5Finfo%20%3E%20%28schema%29) { code, message, documentation\_url, source } 

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

messages: array of [ResponseInfo](https://developers.cloudflare.com/api/resources/$shared#%28resource%29%20%24shared%20%3E%20%28model%29%20response%5Finfo%20%3E%20%28schema%29) { code, message, documentation\_url, source } 

code: number

minimum1000

message: string

documentation\_url: optional string

source: optional object { pointer } 

pointer: optional string

result: array of [Hostname](https://developers.cloudflare.com/api/resources/web3#%28resource%29%20web3.hostnames%20%3E%20%28model%29%20hostname%20%3E%20%28schema%29) { id, created\_on, description, 5 more } 

id: optional string

Specify the identifier of the hostname.

maxLength32

created\_on: optional string

formatdate-time

description: optional string

Specify an optional description of the hostname.

maxLength500

dnslink: optional string

Specify the DNSLink value used if the target is ipfs.

modified\_on: optional string

formatdate-time

name: optional string

Specify the hostname that points to the target gateway via CNAME.

maxLength255

status: optional "active" or "pending" or "deleting" or "error"

Specifies the status of the hostname’s activation.

One of the following:

"active"

"pending"

"deleting"

"error"

target: optional "ethereum" or "ipfs" or "ipfs\_universal\_path"

Specify the target gateway of the hostname.

One of the following:

"ethereum"

"ipfs"

"ipfs\_universal\_path"

success: true

Specifies whether the API call was successful.

result\_info: optional object { count, page, per\_page, total\_count } 

count: optional number

Specifies the total number of results for the requested service.

page: optional number

Specifies the current page within paginated list of results.

per\_page: optional number

Specifies the number of results per page of results.

total\_count: optional number

Specifies the total results available without any search parameters.

### List Web3 Hostnames

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/web3/hostnames \
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
  "result": [
    {
      "id": "023e105f4ecef8ad9ca31a8372d0c353",
      "created_on": "2014-01-01T05:20:00.12345Z",
      "description": "This is my IPFS gateway.",
      "dnslink": "/ipns/onboarding.ipfs.cloudflare.com",
      "modified_on": "2014-01-01T05:20:00.12345Z",
      "name": "gateway.example.com",
      "status": "active",
      "target": "ipfs"
    }
  ],
  "success": true,
  "result_info": {
    "count": 1,
    "page": 1,
    "per_page": 20,
    "total_count": 2000
  }
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
  "result": [
    {
      "id": "023e105f4ecef8ad9ca31a8372d0c353",
      "created_on": "2014-01-01T05:20:00.12345Z",
      "description": "This is my IPFS gateway.",
      "dnslink": "/ipns/onboarding.ipfs.cloudflare.com",
      "modified_on": "2014-01-01T05:20:00.12345Z",
      "name": "gateway.example.com",
      "status": "active",
      "target": "ipfs"
    }
  ],
  "success": true,
  "result_info": {
    "count": 1,
    "page": 1,
    "per_page": 20,
    "total_count": 2000
  }
}
```