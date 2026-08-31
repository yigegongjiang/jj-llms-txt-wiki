---
title: List live inputs
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Stream](https://developers.cloudflare.com/api/resources/stream)

[Live Inputs](https://developers.cloudflare.com/api/resources/stream/subresources/live%5Finputs)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# List live inputs

GET/accounts/{account\_id}/stream/live\_inputs

Lists the live inputs created for an account. To get the credentials needed to stream to a specific live input, request a single live input.

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

`Stream Write` `Stream Read`

##### Path ParametersExpand Collapse 

account\_id: string

Identifier.

maxLength32

##### Query ParametersExpand Collapse 

include\_counts: optional boolean

Includes the total number of videos associated with the submitted query parameters.

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

result: optional object { liveInputs, range, total } 

liveInputs: optional array of object { created, deleteRecordingAfterDays, enabled, 3 more } 

created: optional string

The date and time the live input was created.

formatdate-time

deleteRecordingAfterDays: optional number

Indicates the number of days after which the live inputs recordings will be deleted. When a stream completes and the recording is ready, the value is used to calculate a scheduled deletion date for that recording. Omit the field to indicate no change, or include with a `null` value to remove an existing scheduled deletion.

minimum30

enabled: optional boolean

Indicates whether the live input is enabled and can accept streams.

meta: optional unknown

A user modifiable key-value store used to reference other systems of record for managing live inputs.

modified: optional string

The date and time the live input was last modified.

formatdate-time

uid: optional string

A unique identifier for a live input.

maxLength32

range: optional number

The total number of remaining live inputs based on cursor position.

total: optional number

The total number of live inputs that match the provided filters.

### List live inputs

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/stream/live_inputs \
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
  "result": {
    "liveInputs": [
      {
        "created": "2014-01-02T02:20:00Z",
        "deleteRecordingAfterDays": 45,
        "enabled": true,
        "meta": {
          "name": "test stream 1"
        },
        "modified": "2014-01-02T02:20:00Z",
        "uid": "66be4bf738797e01e1fca35a7bdecdcd"
      }
    ],
    "range": 1000,
    "total": 35586
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
  "success": true,
  "result": {
    "liveInputs": [
      {
        "created": "2014-01-02T02:20:00Z",
        "deleteRecordingAfterDays": 45,
        "enabled": true,
        "meta": {
          "name": "test stream 1"
        },
        "modified": "2014-01-02T02:20:00Z",
        "uid": "66be4bf738797e01e1fca35a7bdecdcd"
      }
    ],
    "range": 1000,
    "total": 35586
  }
}
```