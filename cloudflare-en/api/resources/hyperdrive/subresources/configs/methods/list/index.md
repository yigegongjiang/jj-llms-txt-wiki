---
title: List Hyperdrives
---

[Skip to content](#%5Ftop) 

[API Reference](https://developers.cloudflare.com/api)

[Hyperdrive](https://developers.cloudflare.com/api/resources/hyperdrive)

[Configs](https://developers.cloudflare.com/api/resources/hyperdrive/subresources/configs)

Copy Markdown

Open in **Claude**

Open in **ChatGPT**

Open in **Cursor**

---

**Copy Markdown**

**View as Markdown**

# List Hyperdrives

GET/accounts/{account\_id}/hyperdrive/configs

Returns a list of Hyperdrives.

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

`Hyperdrive Write` `Hyperdrive Read`

##### Path ParametersExpand Collapse 

account\_id: string

Define configurations using a unique string identifier.

maxLength32

##### Query ParametersExpand Collapse 

page: optional number

Page number of paginated results.

minimum1

per\_page: optional number

Maximum number of results per page.

maximum100

minimum1

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

result: array of [Hyperdrive](https://developers.cloudflare.com/api/resources/hyperdrive#%28resource%29%20hyperdrive%20%3E%20%28model%29%20hyperdrive%20%3E%20%28schema%29) { id, name, origin, 6 more } 

id: string

Define configurations using a unique string identifier.

maxLength32

name: string

The name of the Hyperdrive configuration. Used to identify the configuration in the Cloudflare dashboard and API.

origin: object { database, host, password, 3 more } or object { access\_client\_id, access\_client\_secret, database, 4 more } or object { database, password, scheme, 2 more } 

One of the following:

PublicDatabase object { database, host, password, 3 more } 

database: string

Set the name of your origin database.

host: string

Defines the host (hostname or IP) of your origin database.

password: string

Set the password needed to access your origin database. The API never returns this write-only value.

port: number

Defines the port of your origin database. Defaults to 5432 for PostgreSQL or 3306 for MySQL if not specified.

scheme: "postgres" or "postgresql" or "mysql"

Specifies the URL scheme used to connect to your origin database.

One of the following:

"postgres"

"postgresql"

"mysql"

user: string

Set the user of your origin database.

AccessProtectedDatabaseBehindCloudflareTunnel object { access\_client\_id, access\_client\_secret, database, 4 more } 

access\_client\_id: string

Defines the Client ID of the Access token to use when connecting to the origin database.

access\_client\_secret: string

Defines the Client Secret of the Access Token to use when connecting to the origin database. The API never returns this write-only value.

database: string

Set the name of your origin database.

host: string

Defines the host (hostname or IP) of your origin database.

password: string

Set the password needed to access your origin database. The API never returns this write-only value.

scheme: "postgres" or "postgresql" or "mysql"

Specifies the URL scheme used to connect to your origin database.

One of the following:

"postgres"

"postgresql"

"mysql"

user: string

Set the user of your origin database.

DatabaseReachableThroughAWorkersVPC object { database, password, scheme, 2 more } 

database: string

Set the name of your origin database.

password: string

Set the password needed to access your origin database. The API never returns this write-only value.

scheme: "postgres" or "postgresql" or "mysql"

Specifies the URL scheme used to connect to your origin database.

One of the following:

"postgres"

"postgresql"

"mysql"

service\_id: string

The identifier of the Workers VPC Service to connect through. Hyperdrive will egress through the specified VPC Service to reach the origin database.

user: string

Set the user of your origin database.

caching: optional object { disabled } or object { disabled, max\_age, stale\_while\_revalidate } 

One of the following:

HyperdriveHyperdriveCachingCommon object { disabled } 

disabled: optional boolean

Set to true to disable caching of SQL responses. Default is false.

HyperdriveHyperdriveCachingEnabled object { disabled, max\_age, stale\_while\_revalidate } 

disabled: optional boolean

Set to true to disable caching of SQL responses. Default is false.

max\_age: optional number

Specify the maximum duration (in seconds) items should persist in the cache. Defaults to 60 seconds if not specified.

stale\_while\_revalidate: optional number

Specify the number of seconds the cache may serve a stale response. Defaults to 15 seconds if not specified.

created\_on: optional string

Defines the creation time of the Hyperdrive configuration.

formatdate-time

modified\_on: optional string

Defines the last modified time of the Hyperdrive configuration.

formatdate-time

mtls: optional object { ca\_certificate\_id, mtls\_certificate\_id, sslmode } 

mTLS configuration for the origin connection. Cannot be used with VPC Service origins; TLS must be managed on the VPC Service.

ca\_certificate\_id: optional string

Define CA certificate ID obtained after uploading CA cert.

mtls\_certificate\_id: optional string

Define mTLS certificate ID obtained after uploading client cert.

sslmode: optional string

Set SSL mode to ‘require’, ‘verify-ca’, or ‘verify-full’ to verify the CA.

origin\_connection\_limit: optional number

The (soft) maximum number of connections the Hyperdrive is allowed to make to the origin database.

Maximum allowed: 20 for free tier accounts, 100 for paid tier accounts. If not specified, defaults to 20 for free tier and 60 for paid tier. Certain Cloudflare-managed origins may be permitted a higher limit. Contact Cloudflare if you need a higher limit.

minimum5

restarted\_on: optional string

Defines the last time the Hyperdrive connection pool was explicitly restarted via the restart endpoint. Omitted if the pool has never been explicitly restarted.

formatdate-time

success: true

Return the status of the API call success.

result\_info: optional object { count, page, per\_page, total\_count } 

count: optional number

Defines the total number of results for the requested service.

page: optional number

Defines the current page within paginated list of results.

per\_page: optional number

Defines the number of results per page of results.

total\_count: optional number

Defines the total results available without any search parameters.

### List Hyperdrives

HTTP

HTTPHTTP

TypeScriptTypeScript

PythonPython

GoGo

TerraformTerraform

```
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/hyperdrive/configs \
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
      "name": "example-hyperdrive",
      "origin": {
        "database": "postgres",
        "host": "database.example.com",
        "port": 5432,
        "scheme": "postgres",
        "user": "postgres"
      },
      "caching": {
        "disabled": true
      },
      "created_on": "2017-01-01T00:00:00Z",
      "modified_on": "2017-01-01T00:00:00Z",
      "mtls": {
        "ca_certificate_id": "00000000-0000-0000-0000-0000000000",
        "mtls_certificate_id": "00000000-0000-0000-0000-0000000000",
        "sslmode": "verify-full"
      },
      "origin_connection_limit": 60,
      "restarted_on": "2017-01-01T00:00:00Z"
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
      "name": "example-hyperdrive",
      "origin": {
        "database": "postgres",
        "host": "database.example.com",
        "port": 5432,
        "scheme": "postgres",
        "user": "postgres"
      },
      "caching": {
        "disabled": true
      },
      "created_on": "2017-01-01T00:00:00Z",
      "modified_on": "2017-01-01T00:00:00Z",
      "mtls": {
        "ca_certificate_id": "00000000-0000-0000-0000-0000000000",
        "mtls_certificate_id": "00000000-0000-0000-0000-0000000000",
        "sslmode": "verify-full"
      },
      "origin_connection_limit": 60,
      "restarted_on": "2017-01-01T00:00:00Z"
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