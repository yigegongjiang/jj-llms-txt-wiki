---
title: List all Workflows
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Workflows](https://developers.cloudflare.com/api/resources/workflows)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# List all Workflows

GET/accounts/{account\_id}/workflows

Lists all workflows configured for the account.

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

##### Query ParametersExpand Collapse 

page: optional number

minimum1

per\_page: optional number

maximum100

minimum1

search: optional string

Allows filtering workflows\` name.

maxLength64

minLength1

##### ReturnsExpand Collapse 

errors: array of object { code, message } 

code: number

message: string

messages: array of object { code, message } 

code: number

message: string

result: array of object { id, class\_name, created\_on, 6 more } 

id: string

formatuuid

class\_name: string

created\_on: string

formatdate-time

instances: object { complete, errored, paused, 6 more } 

complete: optional number

errored: optional number

paused: optional number

queued: optional number

rollingBack: optional number

running: optional number

terminated: optional number

waiting: optional number

waitingForPause: optional number

modified\_on: string

formatdate-time

name: string

maxLength64

minLength1

script\_name: string

triggered\_on: string

formatdate-time

schedules: optional array of object { cron, next\_instance } 

cron: string

next\_instance: string

success: true

result\_info: optional object { count, per\_page, total\_count, 3 more } 

count: number

per\_page: number

total\_count: number

cursor: optional string

page: optional number

total\_pages: optional number

### List all Workflows

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/workflows \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

200 example

```
{
  "errors": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "messages": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "result": [
    {
      "id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
      "class_name": "class_name",
      "created_on": "2019-12-27T18:11:19.117Z",
      "instances": {
        "complete": 0,
        "errored": 0,
        "paused": 0,
        "queued": 0,
        "rollingBack": 0,
        "running": 0,
        "terminated": 0,
        "waiting": 0,
        "waitingForPause": 0
      },
      "modified_on": "2019-12-27T18:11:19.117Z",
      "name": "x",
      "script_name": "script_name",
      "triggered_on": "2019-12-27T18:11:19.117Z",
      "schedules": [
        {
          "cron": "cron",
          "next_instance": "next_instance"
        }
      ]
    }
  ],
  "success": true,
  "result_info": {
    "count": 0,
    "per_page": 0,
    "total_count": 0,
    "cursor": "cursor",
    "page": 0,
    "total_pages": 0
  }
}
```

##### Returns Examples

200 example

```
{
  "errors": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "messages": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "result": [
    {
      "id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
      "class_name": "class_name",
      "created_on": "2019-12-27T18:11:19.117Z",
      "instances": {
        "complete": 0,
        "errored": 0,
        "paused": 0,
        "queued": 0,
        "rollingBack": 0,
        "running": 0,
        "terminated": 0,
        "waiting": 0,
        "waitingForPause": 0
      },
      "modified_on": "2019-12-27T18:11:19.117Z",
      "name": "x",
      "script_name": "script_name",
      "triggered_on": "2019-12-27T18:11:19.117Z",
      "schedules": [
        {
          "cron": "cron",
          "next_instance": "next_instance"
        }
      ]
    }
  ],
  "success": true,
  "result_info": {
    "count": 0,
    "per_page": 0,
    "total_count": 0,
    "cursor": "cursor",
    "page": 0,
    "total_pages": 0
  }
}
```