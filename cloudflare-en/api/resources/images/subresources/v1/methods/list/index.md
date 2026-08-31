---
title: List images
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Images](https://developers.cloudflare.com/api/resources/images)

[V1](https://developers.cloudflare.com/api/resources/images/subresources/v1)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# List images

Deprecated

GET/accounts/{account\_id}/images/v1

List up to 100 images with one request. Use the optional parameters below to get a specific range of images.

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

`Images Read` `Images Write`

##### Path ParametersExpand Collapse 

account\_id: string

Account identifier tag.

maxLength32

##### Query ParametersExpand Collapse 

creator: optional string

Internal user ID set within the creator field. Setting to empty string "" will return images where creator field is not set

page: optional number

Page number of paginated results.

minimum1

per\_page: optional number

Number of items per page.

maximum10000

minimum10

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

result: object { images } 

images: optional array of [Image](https://developers.cloudflare.com/api/resources/images#%28resource%29%20images.v1%20%3E%20%28model%29%20image%20%3E%20%28schema%29) { id, creator, filename, 4 more } 

id: optional string

Image unique identifier.

maxLength32

creator: optional string

Can set the creator field with an internal user ID.

maxLength1024

filename: optional string

Image file name.

maxLength255

meta: optional unknown

User modifiable key-value store. Can be used for keeping references to another system of record for managing images. Metadata must not exceed 1024 bytes.

requireSignedURLs: optional boolean

Indicates whether the image can be a accessed only using it’s UID. If set to true, a signed token needs to be generated with a signing key to view the image.

uploaded: optional string

When the media item was uploaded.

formatdate-time

variants: optional array of string

Object specifying available variants for an image.

success: true

Whether the API call was successful

### List images

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/images/v1 \
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
  "result": {
    "images": [
      {
        "id": "id",
        "creator": "107b9558-dd06-4bbd-5fef-9c2c16bb7900",
        "filename": "logo.png",
        "meta": {
          "key": "value"
        },
        "requireSignedURLs": true,
        "uploaded": "2014-01-02T02:20:00.123Z",
        "variants": [
          "https://imagedelivery.net/MTt4OTd0b0w5aj/107b9558-dd06-4bbd-5fef-9c2c16bb7900/thumbnail",
          "https://imagedelivery.net/MTt4OTd0b0w5aj/107b9558-dd06-4bbd-5fef-9c2c16bb7900/hero",
          "https://imagedelivery.net/MTt4OTd0b0w5aj/107b9558-dd06-4bbd-5fef-9c2c16bb7900/original"
        ]
      }
    ]
  },
  "success": true
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
  "result": {
    "images": [
      {
        "id": "id",
        "creator": "107b9558-dd06-4bbd-5fef-9c2c16bb7900",
        "filename": "logo.png",
        "meta": {
          "key": "value"
        },
        "requireSignedURLs": true,
        "uploaded": "2014-01-02T02:20:00.123Z",
        "variants": [
          "https://imagedelivery.net/MTt4OTd0b0w5aj/107b9558-dd06-4bbd-5fef-9c2c16bb7900/thumbnail",
          "https://imagedelivery.net/MTt4OTd0b0w5aj/107b9558-dd06-4bbd-5fef-9c2c16bb7900/hero",
          "https://imagedelivery.net/MTt4OTd0b0w5aj/107b9558-dd06-4bbd-5fef-9c2c16bb7900/original"
        ]
      }
    ]
  },
  "success": true
}
```