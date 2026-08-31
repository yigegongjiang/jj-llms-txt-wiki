---
title: Create D1 Database
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[D1](https://developers.cloudflare.com/api/resources/d1)

[Database](https://developers.cloudflare.com/api/resources/d1/subresources/database)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# Create D1 Database

POST/accounts/{account\_id}/d1/database

Returns the created D1 database.

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

`D1 Write`

##### Path ParametersExpand Collapse 

account\_id: string

Account identifier tag.

maxLength32

##### Body ParametersJSONExpand Collapse 

name: string

D1 database name.

jurisdiction: optional "eu" or "fedramp" or "us"

Specify the location to restrict the D1 database to run and store data. If this option is present, the location hint is ignored.

One of the following:

"eu"

"fedramp"

"us"

primary\_location\_hint: optional "wnam" or "enam" or "weur" or 3 more

Specify the region to create the D1 primary, if available. If this option is omitted, the D1 will be created as close as possible to the current user.

One of the following:

"wnam"

"enam"

"weur"

"eeur"

"apac"

"oc"

read\_replication: optional object { mode } 

Configuration for D1 read replication.

mode: "auto" or "disabled"

The read replication mode for the database. Use ‘auto’ to create replicas and allow D1 automatically place them around the world, or ‘disabled’ to not use any database replicas (it can take a few hours for all replicas to be deleted).

One of the following:

"auto"

"disabled"

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

result: [D1](https://developers.cloudflare.com/api/resources/d1#%28resource%29%20d1%20%3E%20%28model%29%20d1%20%3E%20%28schema%29) { created\_at, file\_size, jurisdiction, 5 more } 

The details of the D1 database.

created\_at: optional string

Specifies the timestamp the resource was created as an ISO8601 string.

formatdate-time

file\_size: optional number

The D1 database’s size, in bytes.

jurisdiction: optional "eu" or "fedramp" or "us"

Specify the location to restrict the D1 database to run and store data. If this option is present, the location hint is ignored.

One of the following:

"eu"

"fedramp"

"us"

name: optional string

D1 database name.

num\_tables: optional number

read\_replication: optional object { mode } 

Configuration for D1 read replication.

mode: "auto" or "disabled"

The read replication mode for the database. Mode ‘auto’ denotes that D1 creates replicas and automatically places them around the world. Mode ‘disabled’ denotes that no database replicas are used.

One of the following:

"auto"

"disabled"

uuid: optional string

D1 database identifier (UUID).

version: optional string

success: true

Whether the API call was successful

### Create D1 Database

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/d1/database \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "name": "my-database",
          "jurisdiction": "eu",
          "primary_location_hint": "wnam"
        }'
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
    "created_at": "2022-11-15T18:25:44.442097Z",
    "file_size": 12,
    "jurisdiction": "eu",
    "name": "my-database",
    "num_tables": 12,
    "read_replication": {
      "mode": "auto"
    },
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    "version": "production"
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
    "created_at": "2022-11-15T18:25:44.442097Z",
    "file_size": 12,
    "jurisdiction": "eu",
    "name": "my-database",
    "num_tables": 12,
    "read_replication": {
      "mode": "auto"
    },
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    "version": "production"
  },
  "success": true
}
```