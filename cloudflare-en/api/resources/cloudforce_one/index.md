# Cloudforce One

# Scans

# Results

## Get the Latest Scan Result

**get** `/accounts/{account_id}/cloudforce-one/scans/results/{config_id}`

Get the Latest Scan Result

### Path Parameters

- `account_id: string`

  Defines the Account ID.

- `config_id: string`

  Defines the Config ID.

### Returns

- `errors: array of string`

- `messages: array of string`

- `result: object { "1.1.1.1" }`

  - `"1.1.1.1": array of ScanResult`

    - `number: optional number`

    - `proto: optional string`

    - `status: optional string`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/scans/results/$CONFIG_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    "string"
  ],
  "messages": [
    "string"
  ],
  "result": {
    "1.1.1.1": [
      {
        "number": 8080,
        "proto": "tcp",
        "status": "open"
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Scan Result

- `ScanResult object { number, proto, status }`

  - `number: optional number`

  - `proto: optional string`

  - `status: optional string`

### Result Get Response

- `ResultGetResponse object { "1.1.1.1" }`

  - `"1.1.1.1": array of ScanResult`

    - `number: optional number`

    - `proto: optional string`

    - `status: optional string`

# Config

## List Scan Configs

**get** `/accounts/{account_id}/cloudforce-one/scans/config`

List Scan Configs

### Path Parameters

- `account_id: string`

  Defines the Account ID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of object { id, account_id, frequency, 2 more }`

  - `id: string`

    Defines the Config ID.

  - `account_id: string`

  - `frequency: number`

    Defines the number of days between each scan (0 = One-off scan).

  - `ips: array of string`

    Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

  - `ports: array of string`

    Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/scans/config \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
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
      "id": "uuid",
      "account_id": "abcd1234abcd1234abcd1234abcd1234",
      "frequency": 7,
      "ips": [
        "1.1.1.1",
        "2606:4700:4700::1111"
      ],
      "ports": [
        "default"
      ]
    }
  ]
}
```

## Create a new Scan Config

**post** `/accounts/{account_id}/cloudforce-one/scans/config`

Create a new Scan Config

### Path Parameters

- `account_id: string`

  Defines the Account ID.

### Body Parameters

- `ips: array of string`

  Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

- `frequency: optional number`

  Defines the number of days between each scan (0 = One-off scan).

- `ports: optional array of string`

  Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { id, account_id, frequency, 2 more }`

  - `id: string`

    Defines the Config ID.

  - `account_id: string`

  - `frequency: number`

    Defines the number of days between each scan (0 = One-off scan).

  - `ips: array of string`

    Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

  - `ports: array of string`

    Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/scans/config \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "ips": [
            "1.1.1.1",
            "2606:4700:4700::1111"
          ],
          "frequency": 7,
          "ports": [
            "default"
          ]
        }'
```

#### Response

```json
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
    "id": "uuid",
    "account_id": "abcd1234abcd1234abcd1234abcd1234",
    "frequency": 7,
    "ips": [
      "1.1.1.1",
      "2606:4700:4700::1111"
    ],
    "ports": [
      "default"
    ]
  }
}
```

## Update an existing Scan Config

**patch** `/accounts/{account_id}/cloudforce-one/scans/config/{config_id}`

Update an existing Scan Config

### Path Parameters

- `account_id: string`

  Defines the Account ID.

- `config_id: string`

  Defines the Config ID.

### Body Parameters

- `frequency: optional number`

  Defines the number of days between each scan (0 = One-off scan).

- `ips: optional array of string`

  Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

- `ports: optional array of string`

  Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { id, account_id, frequency, 2 more }`

  - `id: string`

    Defines the Config ID.

  - `account_id: string`

  - `frequency: number`

    Defines the number of days between each scan (0 = One-off scan).

  - `ips: array of string`

    Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

  - `ports: array of string`

    Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/scans/config/$CONFIG_ID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
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
    "id": "uuid",
    "account_id": "abcd1234abcd1234abcd1234abcd1234",
    "frequency": 7,
    "ips": [
      "1.1.1.1",
      "2606:4700:4700::1111"
    ],
    "ports": [
      "default"
    ]
  }
}
```

## Delete a Scan Config

**delete** `/accounts/{account_id}/cloudforce-one/scans/config/{config_id}`

Delete a Scan Config

### Path Parameters

- `account_id: string`

  Defines the Account ID.

- `config_id: string`

  Defines the Config ID.

### Returns

- `errors: array of string`

- `messages: array of string`

- `result: unknown`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/scans/config/$CONFIG_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    "string"
  ],
  "messages": [
    "string"
  ],
  "result": {},
  "success": true
}
```

## Domain Types

### Config List Response

- `ConfigListResponse object { id, account_id, frequency, 2 more }`

  - `id: string`

    Defines the Config ID.

  - `account_id: string`

  - `frequency: number`

    Defines the number of days between each scan (0 = One-off scan).

  - `ips: array of string`

    Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

  - `ports: array of string`

    Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Config Create Response

- `ConfigCreateResponse object { id, account_id, frequency, 2 more }`

  - `id: string`

    Defines the Config ID.

  - `account_id: string`

  - `frequency: number`

    Defines the number of days between each scan (0 = One-off scan).

  - `ips: array of string`

    Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

  - `ports: array of string`

    Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Config Edit Response

- `ConfigEditResponse object { id, account_id, frequency, 2 more }`

  - `id: string`

    Defines the Config ID.

  - `account_id: string`

  - `frequency: number`

    Defines the number of days between each scan (0 = One-off scan).

  - `ips: array of string`

    Defines a list of IP addresses or CIDR blocks to scan. The maximum number of total IP addresses allowed is 5000.

  - `ports: array of string`

    Defines a list of ports to scan. Valid values are:"default", "all", or a comma-separated list of ports or range of ports (e.g. ["1-80", "443"]). "default" scans the 100 most commonly open ports.

### Config Delete Response

- `ConfigDeleteResponse = unknown`

# Binary Storage

## Retrieves a file from Binary Storage

**get** `/accounts/{account_id}/cloudforce-one/binary/{hash}`

Retrieves a binary file from the Cloudforce One binary storage for analysis.

### Path Parameters

- `account_id: string`

  Account ID.

- `hash: string`

  hash of the binary

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/binary/$HASH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

## Posts a file to Binary Storage

**post** `/accounts/{account_id}/cloudforce-one/binary`

Uploads a binary file to Cloudforce One's binary database for malware analysis and threat intelligence correlation.

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `content_type: string`

- `md5: string`

- `sha1: string`

- `sha256: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/binary \
    -H 'Content-Type: multipart/form-data' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -F 'file=@/path/to/file'
```

#### Response

```json
{
  "content_type": "text/plain",
  "md5": "5d84ade76d2a8387c81175bb0cbe6492",
  "sha1": "9aff6879626d957eafadda044e4f879aae1e7278",
  "sha256": "0000a7f2692ef479e2e3d02661568882cadec451cc8a64d4e7faca29810cd626"
}
```

## Domain Types

### Binary Storage Create Response

- `BinaryStorageCreateResponse object { content_type, md5, sha1, sha256 }`

  - `content_type: string`

  - `md5: string`

  - `sha1: string`

  - `sha256: string`

# Requests

## List Requests

**post** `/accounts/{account_id}/cloudforce-one/requests`

Lists Cloudforce One intelligence requests with filtering and pagination.

### Path Parameters

- `account_id: string`

  Identifier.

### Body Parameters

- `page: number`

  Page number of results.

- `per_page: number`

  Number of results per page.

- `completed_after: optional string`

  Retrieve requests completed after this time.

- `completed_before: optional string`

  Retrieve requests completed before this time.

- `created_after: optional string`

  Retrieve requests created after this time.

- `created_before: optional string`

  Retrieve requests created before this time.

- `request_type: optional string`

  Requested information from request.

- `sort_by: optional string`

  Field to sort results by.

- `sort_order: optional "asc" or "desc"`

  Sort order (asc or desc).

  - `"asc"`

  - `"desc"`

- `status: optional "open" or "accepted" or "reported" or 3 more`

  Request Status.

  - `"open"`

  - `"accepted"`

  - `"reported"`

  - `"approved"`

  - `"completed"`

  - `"declined"`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of ListItem`

  - `id: string`

    UUID.

  - `created: string`

    Request creation time.

  - `priority: "routine" or "high" or "urgent"`

    - `"routine"`

    - `"high"`

    - `"urgent"`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

    Request last updated time.

  - `completed: optional string`

    Request completion time.

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "page": 0,
          "per_page": 10,
          "completed_after": "2022-01-01T00:00:00Z",
          "completed_before": "2024-01-01T00:00:00Z",
          "created_after": "2022-01-01T00:00:00Z",
          "created_before": "2024-01-01T00:00:00Z",
          "request_type": "Victomology",
          "sort_by": "created"
        }'
```

#### Response

```json
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
      "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
      "created": "2022-04-01T00:00:00Z",
      "priority": "routine",
      "request": "Victomology",
      "summary": "DoS attack",
      "tlp": "clear",
      "updated": "2022-04-01T00:00:00Z",
      "completed": "2024-01-01T00:00:00Z",
      "message_tokens": 16,
      "readable_id": "RFI-2022-000001",
      "status": "open",
      "tokens": 0
    }
  ]
}
```

## Get a Request

**get** `/accounts/{account_id}/cloudforce-one/requests/{request_id}`

Retrieves details for a specific Cloudforce One intelligence request.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Item`

  - `id: string`

    UUID.

  - `content: string`

    Request content.

  - `created: string`

  - `priority: string`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

  - `completed: optional string`

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "content": "What regions were most effected by the recent DoS?",
    "created": "2022-04-01T05:20:00Z",
    "priority": "2022-04-01T05:20:00Z",
    "request": "Victomology",
    "summary": "DoS attack",
    "tlp": "clear",
    "updated": "2022-04-01T05:20:00Z",
    "completed": "2022-04-01T05:20:00Z",
    "message_tokens": 1,
    "readable_id": "RFI-2022-000001",
    "status": "open",
    "tokens": 16
  }
}
```

## Create a New Request.

**post** `/accounts/{account_id}/cloudforce-one/requests/new`

Creating a request adds the request into the Cloudforce One queue for analysis. In addition to the content, a short title, type, priority, and releasability should be provided. If one is not provided, a default will be assigned.

### Path Parameters

- `account_id: string`

  Identifier.

### Body Parameters

- `content: optional string`

  Request content.

- `priority: optional string`

  Priority for analyzing the request.

- `request_type: optional string`

  Requested information from request.

- `summary: optional string`

  Brief description of the request.

- `tlp: optional "clear" or "amber" or "amber-strict" or 2 more`

  The CISA defined Traffic Light Protocol (TLP).

  - `"clear"`

  - `"amber"`

  - `"amber-strict"`

  - `"green"`

  - `"red"`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Item`

  - `id: string`

    UUID.

  - `content: string`

    Request content.

  - `created: string`

  - `priority: string`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

  - `completed: optional string`

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/new \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "content": "What regions were most effected by the recent DoS?",
          "priority": "routine",
          "request_type": "Victomology",
          "summary": "DoS attack"
        }'
```

#### Response

```json
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
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "content": "What regions were most effected by the recent DoS?",
    "created": "2022-04-01T05:20:00Z",
    "priority": "2022-04-01T05:20:00Z",
    "request": "Victomology",
    "summary": "DoS attack",
    "tlp": "clear",
    "updated": "2022-04-01T05:20:00Z",
    "completed": "2022-04-01T05:20:00Z",
    "message_tokens": 1,
    "readable_id": "RFI-2022-000001",
    "status": "open",
    "tokens": 16
  }
}
```

## Update a Request

**put** `/accounts/{account_id}/cloudforce-one/requests/{request_id}`

Updating a request alters the request in the Cloudforce One queue. This API may be used to update any attributes of the request after the initial submission. Only fields that you choose to update need to be add to the request body.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

### Body Parameters

- `content: optional string`

  Request content.

- `priority: optional string`

  Priority for analyzing the request.

- `request_type: optional string`

  Requested information from request.

- `summary: optional string`

  Brief description of the request.

- `tlp: optional "clear" or "amber" or "amber-strict" or 2 more`

  The CISA defined Traffic Light Protocol (TLP).

  - `"clear"`

  - `"amber"`

  - `"amber-strict"`

  - `"green"`

  - `"red"`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Item`

  - `id: string`

    UUID.

  - `content: string`

    Request content.

  - `created: string`

  - `priority: string`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

  - `completed: optional string`

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "content": "What regions were most effected by the recent DoS?",
          "priority": "routine",
          "request_type": "Victomology",
          "summary": "DoS attack"
        }'
```

#### Response

```json
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
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "content": "What regions were most effected by the recent DoS?",
    "created": "2022-04-01T05:20:00Z",
    "priority": "2022-04-01T05:20:00Z",
    "request": "Victomology",
    "summary": "DoS attack",
    "tlp": "clear",
    "updated": "2022-04-01T05:20:00Z",
    "completed": "2022-04-01T05:20:00Z",
    "message_tokens": 1,
    "readable_id": "RFI-2022-000001",
    "status": "open",
    "tokens": 16
  }
}
```

## Delete a Request

**delete** `/accounts/{account_id}/cloudforce-one/requests/{request_id}`

Deletes a Cloudforce One intelligence request and all associated data.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
  "success": true
}
```

## Get Request Quota

**get** `/accounts/{account_id}/cloudforce-one/requests/quota`

Retrieves quota usage for Cloudforce One standard requests.

### Path Parameters

- `account_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Quota`

  - `anniversary_date: optional string`

    Anniversary date is when annual quota limit is refreshed.

  - `quarter_anniversary_date: optional string`

    Quarter anniversary date is when quota limit is refreshed each quarter.

  - `quota: optional number`

    Tokens for the quarter.

  - `remaining: optional number`

    Tokens remaining for the quarter.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/quota \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
    "anniversary_date": "2022-04-01T00:00:00Z",
    "quarter_anniversary_date": "2022-04-01T00:00:00Z",
    "quota": 120,
    "remaining": 64
  }
}
```

## Get Request Types

**get** `/accounts/{account_id}/cloudforce-one/requests/types`

Lists available request types for Cloudforce One intelligence requests.

### Path Parameters

- `account_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional RequestTypes`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/types \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
    "Indicators of Compromise",
    "Victomology"
  ]
}
```

## Get Request Priority, Status, and TLP constants

**get** `/accounts/{account_id}/cloudforce-one/requests/constants`

Retrieves constant values used in Cloudforce One requests, including valid statuses and types.

### Path Parameters

- `account_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional RequestConstants`

  - `priority: optional array of "routine" or "high" or "urgent"`

    - `"routine"`

    - `"high"`

    - `"urgent"`

  - `status: optional array of "open" or "accepted" or "reported" or 3 more`

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tlp: optional array of "clear" or "amber" or "amber-strict" or 2 more`

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/constants \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
    "priority": [
      "routine",
      "high",
      "urgent"
    ],
    "status": [
      "open",
      "accepted",
      "reported",
      "approved",
      "completed",
      "declined"
    ],
    "tlp": [
      "clear",
      "green",
      "amber",
      "amber-strict",
      "red"
    ]
  }
}
```

## Domain Types

### Item

- `Item object { id, content, created, 10 more }`

  - `id: string`

    UUID.

  - `content: string`

    Request content.

  - `created: string`

  - `priority: string`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

  - `completed: optional string`

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### List Item

- `ListItem object { id, created, priority, 9 more }`

  - `id: string`

    UUID.

  - `created: string`

    Request creation time.

  - `priority: "routine" or "high" or "urgent"`

    - `"routine"`

    - `"high"`

    - `"urgent"`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

    Request last updated time.

  - `completed: optional string`

    Request completion time.

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Quota

- `Quota object { anniversary_date, quarter_anniversary_date, quota, remaining }`

  - `anniversary_date: optional string`

    Anniversary date is when annual quota limit is refreshed.

  - `quarter_anniversary_date: optional string`

    Quarter anniversary date is when quota limit is refreshed each quarter.

  - `quota: optional number`

    Tokens for the quarter.

  - `remaining: optional number`

    Tokens remaining for the quarter.

### Request Constants

- `RequestConstants object { priority, status, tlp }`

  - `priority: optional array of "routine" or "high" or "urgent"`

    - `"routine"`

    - `"high"`

    - `"urgent"`

  - `status: optional array of "open" or "accepted" or "reported" or 3 more`

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tlp: optional array of "clear" or "amber" or "amber-strict" or 2 more`

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

### Request Types

- `RequestTypes = array of string`

### Request Delete Response

- `RequestDeleteResponse object { errors, messages, success }`

  - `errors: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `messages: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `success: true`

    Whether the API call was successful.

    - `true`

### Request Types Response

- `RequestTypesResponse = string`

  Request Types.

# Message

## List Request Messages

**post** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/message`

Lists messages in a Cloudforce One intelligence request conversation.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

### Body Parameters

- `page: number`

  Page number of results.

- `per_page: number`

  Number of results per page.

- `after: optional string`

  Retrieve mes  ges created after this time.

- `before: optional string`

  Retrieve messages created before this time.

- `sort_by: optional string`

  Field to sort results by.

- `sort_order: optional "asc" or "desc"`

  Sort order (asc or desc).

  - `"asc"`

  - `"desc"`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of Message`

  - `id: number`

    Message ID.

  - `author: string`

    Author of message.

  - `content: string`

    Content of message.

  - `is_follow_on_request: boolean`

    Whether the message is a follow-on request.

  - `updated: string`

    Defines the message last updated time.

  - `created: optional string`

    Defines the message creation time.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/message \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "page": 0,
          "per_page": 10,
          "before": "2024-01-01T00:00:00Z",
          "sort_by": "created"
        }'
```

#### Response

```json
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
      "id": 0,
      "author": "user@domain.com",
      "content": "Can you elaborate on the type of DoS that occurred?",
      "is_follow_on_request": true,
      "updated": "2022-01-01T00:00:00Z",
      "created": "2022-01-01T00:00:00Z"
    }
  ]
}
```

## Create a New Request Message

**post** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/message/new`

Adds a message to a Cloudforce One intelligence request conversation.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

### Body Parameters

- `content: optional string`

  Content of message.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Message`

  - `id: number`

    Message ID.

  - `author: string`

    Author of message.

  - `content: string`

    Content of message.

  - `is_follow_on_request: boolean`

    Whether the message is a follow-on request.

  - `updated: string`

    Defines the message last updated time.

  - `created: optional string`

    Defines the message creation time.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/message/new \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "content": "Can you elaborate on the type of DoS that occurred?"
        }'
```

#### Response

```json
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
    "id": 0,
    "author": "user@domain.com",
    "content": "Can you elaborate on the type of DoS that occurred?",
    "is_follow_on_request": true,
    "updated": "2022-01-01T00:00:00Z",
    "created": "2022-01-01T00:00:00Z"
  }
}
```

## Update a Request Message

**put** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/message/{message_id}`

Updates a message in a Cloudforce One intelligence request thread.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

- `message_id: number`

### Body Parameters

- `content: optional string`

  Content of message.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Message`

  - `id: number`

    Message ID.

  - `author: string`

    Author of message.

  - `content: string`

    Content of message.

  - `is_follow_on_request: boolean`

    Whether the message is a follow-on request.

  - `updated: string`

    Defines the message last updated time.

  - `created: optional string`

    Defines the message creation time.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/message/$MESSAGE_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "content": "Can you elaborate on the type of DoS that occurred?"
        }'
```

#### Response

```json
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
    "id": 0,
    "author": "user@domain.com",
    "content": "Can you elaborate on the type of DoS that occurred?",
    "is_follow_on_request": true,
    "updated": "2022-01-01T00:00:00Z",
    "created": "2022-01-01T00:00:00Z"
  }
}
```

## Delete a Request Message

**delete** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/message/{message_id}`

Removes a message from a Cloudforce One intelligence request thread.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

- `message_id: number`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/message/$MESSAGE_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
  "success": true
}
```

## Domain Types

### Message

- `Message object { id, author, content, 3 more }`

  - `id: number`

    Message ID.

  - `author: string`

    Author of message.

  - `content: string`

    Content of message.

  - `is_follow_on_request: boolean`

    Whether the message is a follow-on request.

  - `updated: string`

    Defines the message last updated time.

  - `created: optional string`

    Defines the message creation time.

### Message Delete Response

- `MessageDeleteResponse object { errors, messages, success }`

  - `errors: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `messages: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `success: true`

    Whether the API call was successful.

    - `true`

# Priority

## Get a Priority Intelligence Requirement

**get** `/accounts/{account_id}/cloudforce-one/requests/priority/{priority_id}`

Retrieves a specific priority intelligence request from Cloudforce One.

### Path Parameters

- `account_id: string`

  Identifier.

- `priority_id: string`

  UUID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Item`

  - `id: string`

    UUID.

  - `content: string`

    Request content.

  - `created: string`

  - `priority: string`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

  - `completed: optional string`

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/priority/$PRIORITY_ID \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "content": "What regions were most effected by the recent DoS?",
    "created": "2022-04-01T05:20:00Z",
    "priority": "2022-04-01T05:20:00Z",
    "request": "Victomology",
    "summary": "DoS attack",
    "tlp": "clear",
    "updated": "2022-04-01T05:20:00Z",
    "completed": "2022-04-01T05:20:00Z",
    "message_tokens": 1,
    "readable_id": "RFI-2022-000001",
    "status": "open",
    "tokens": 16
  }
}
```

## Create a New Priority Intelligence Requirement

**post** `/accounts/{account_id}/cloudforce-one/requests/priority/new`

Creates a new priority intelligence request in Cloudforce One.

### Path Parameters

- `account_id: string`

  Identifier.

### Body Parameters

- `labels: array of Label`

  List of labels.

- `priority: number`

  Priority.

- `requirement: string`

  Requirement.

- `tlp: "clear" or "amber" or "amber-strict" or 2 more`

  The CISA defined Traffic Light Protocol (TLP).

  - `"clear"`

  - `"amber"`

  - `"amber-strict"`

  - `"green"`

  - `"red"`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Priority`

  - `id: string`

    UUID.

  - `created: string`

    Priority creation time.

  - `labels: array of Label`

    List of labels.

  - `priority: number`

    Priority.

  - `requirement: string`

    Requirement.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

    Priority last updated time.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/priority/new \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "labels": [
            "DoS",
            "CVE"
          ],
          "priority": 1,
          "requirement": "DoS attacks carried out by CVEs",
          "tlp": "clear"
        }'
```

#### Response

```json
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
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "created": "2022-04-01T05:20:00Z",
    "labels": [
      "DoS",
      "CVE"
    ],
    "priority": 1,
    "requirement": "DoS attacks carried out by CVEs",
    "tlp": "clear",
    "updated": "2022-04-01T05:20:00Z"
  }
}
```

## Update a Priority Intelligence Requirement

**put** `/accounts/{account_id}/cloudforce-one/requests/priority/{priority_id}`

Updates a priority intelligence request in Cloudforce One.

### Path Parameters

- `account_id: string`

  Identifier.

- `priority_id: string`

  UUID.

### Body Parameters

- `labels: array of Label`

  List of labels.

- `priority: number`

  Priority.

- `requirement: string`

  Requirement.

- `tlp: "clear" or "amber" or "amber-strict" or 2 more`

  The CISA defined Traffic Light Protocol (TLP).

  - `"clear"`

  - `"amber"`

  - `"amber-strict"`

  - `"green"`

  - `"red"`

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Item`

  - `id: string`

    UUID.

  - `content: string`

    Request content.

  - `created: string`

  - `priority: string`

  - `request: string`

    Requested information from request.

  - `summary: string`

    Brief description of the request.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

  - `completed: optional string`

  - `message_tokens: optional number`

    Tokens for the request messages.

  - `readable_id: optional string`

    Readable Request ID.

  - `status: optional "open" or "accepted" or "reported" or 3 more`

    Request Status.

    - `"open"`

    - `"accepted"`

    - `"reported"`

    - `"approved"`

    - `"completed"`

    - `"declined"`

  - `tokens: optional number`

    Tokens for the request.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/priority/$PRIORITY_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "labels": [
            "DoS",
            "CVE"
          ],
          "priority": 1,
          "requirement": "DoS attacks carried out by CVEs",
          "tlp": "clear"
        }'
```

#### Response

```json
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
    "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
    "content": "What regions were most effected by the recent DoS?",
    "created": "2022-04-01T05:20:00Z",
    "priority": "2022-04-01T05:20:00Z",
    "request": "Victomology",
    "summary": "DoS attack",
    "tlp": "clear",
    "updated": "2022-04-01T05:20:00Z",
    "completed": "2022-04-01T05:20:00Z",
    "message_tokens": 1,
    "readable_id": "RFI-2022-000001",
    "status": "open",
    "tokens": 16
  }
}
```

## Delete a Priority Intelligence Requirement

**delete** `/accounts/{account_id}/cloudforce-one/requests/priority/{priority_id}`

Deletes a priority intelligence request from Cloudforce One.

### Path Parameters

- `account_id: string`

  Identifier.

- `priority_id: string`

  UUID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/priority/$PRIORITY_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
  "success": true
}
```

## Get Priority Intelligence Requirement Quota

**get** `/accounts/{account_id}/cloudforce-one/requests/priority/quota`

Retrieves quota usage for Cloudforce One priority requests.

### Path Parameters

- `account_id: string`

  Identifier.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional Quota`

  - `anniversary_date: optional string`

    Anniversary date is when annual quota limit is refreshed.

  - `quarter_anniversary_date: optional string`

    Quarter anniversary date is when quota limit is refreshed each quarter.

  - `quota: optional number`

    Tokens for the quarter.

  - `remaining: optional number`

    Tokens remaining for the quarter.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/priority/quota \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
    "anniversary_date": "2022-04-01T00:00:00Z",
    "quarter_anniversary_date": "2022-04-01T00:00:00Z",
    "quota": 120,
    "remaining": 64
  }
}
```

## Domain Types

### Label

- `Label = string`

### Priority

- `Priority object { id, created, labels, 4 more }`

  - `id: string`

    UUID.

  - `created: string`

    Priority creation time.

  - `labels: array of Label`

    List of labels.

  - `priority: number`

    Priority.

  - `requirement: string`

    Requirement.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

  - `updated: string`

    Priority last updated time.

### Priority Edit

- `PriorityEdit object { labels, priority, requirement, tlp }`

  - `labels: array of Label`

    List of labels.

  - `priority: number`

    Priority.

  - `requirement: string`

    Requirement.

  - `tlp: "clear" or "amber" or "amber-strict" or 2 more`

    The CISA defined Traffic Light Protocol (TLP).

    - `"clear"`

    - `"amber"`

    - `"amber-strict"`

    - `"green"`

    - `"red"`

### Priority Delete Response

- `PriorityDeleteResponse object { errors, messages, success }`

  - `errors: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `messages: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `success: true`

    Whether the API call was successful.

    - `true`

# Assets

## Get a Request Asset

**get** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/asset/{asset_id}`

Retrieves an asset attached to a Cloudforce One intelligence request.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

- `asset_id: string`

  UUID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of object { id, name, created, 2 more }`

  - `id: number`

    Asset ID.

  - `name: string`

    Asset name.

  - `created: optional string`

    Defines the asset creation time.

  - `description: optional string`

    Asset description.

  - `file_type: optional string`

    Asset file type.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/asset/$ASSET_ID \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
      "id": 0,
      "name": "example.docx",
      "created": "2022-01-01T00:00:00Z",
      "description": "example description",
      "file_type": "docx"
    }
  ]
}
```

## List Request Assets

**post** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/asset`

Lists assets attached to a Cloudforce One intelligence request.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

### Body Parameters

- `page: number`

  Page number of results.

- `per_page: number`

  Number of results per page.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional array of object { id, name, created, 2 more }`

  - `id: number`

    Asset ID.

  - `name: string`

    Asset name.

  - `created: optional string`

    Defines the asset creation time.

  - `description: optional string`

    Asset description.

  - `file_type: optional string`

    Asset file type.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/asset \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "page": 0,
          "per_page": 10
        }'
```

#### Response

```json
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
      "id": 0,
      "name": "example.docx",
      "created": "2022-01-01T00:00:00Z",
      "description": "example description",
      "file_type": "docx"
    }
  ]
}
```

## Update a Request Asset

**put** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/asset/{asset_id}`

Updates an asset in a Cloudforce One intelligence request.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

- `asset_id: string`

  UUID.

### Body Parameters

- `source: optional string`

  Asset file to upload.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

- `result: optional object { id, name, created, 2 more }`

  - `id: number`

    Asset ID.

  - `name: string`

    Asset name.

  - `created: optional string`

    Defines the asset creation time.

  - `description: optional string`

    Asset description.

  - `file_type: optional string`

    Asset file type.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/asset/$ASSET_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "source": "@/Users/me/example.docx"
        }'
```

#### Response

```json
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
    "id": 0,
    "name": "example.docx",
    "created": "2022-01-01T00:00:00Z",
    "description": "example description",
    "file_type": "docx"
  }
}
```

## Delete a Request Asset

**delete** `/accounts/{account_id}/cloudforce-one/requests/{request_id}/asset/{asset_id}`

Removes an asset from a Cloudforce One intelligence request.

### Path Parameters

- `account_id: string`

  Identifier.

- `request_id: string`

  UUID.

- `asset_id: string`

  UUID.

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `success: true`

  Whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/requests/$REQUEST_ID/asset/$ASSET_ID \
    -X DELETE \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
```

#### Response

```json
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
  "success": true
}
```

## Domain Types

### Asset Get Response

- `AssetGetResponse object { id, name, created, 2 more }`

  - `id: number`

    Asset ID.

  - `name: string`

    Asset name.

  - `created: optional string`

    Defines the asset creation time.

  - `description: optional string`

    Asset description.

  - `file_type: optional string`

    Asset file type.

### Asset Create Response

- `AssetCreateResponse object { id, name, created, 2 more }`

  - `id: number`

    Asset ID.

  - `name: string`

    Asset name.

  - `created: optional string`

    Defines the asset creation time.

  - `description: optional string`

    Asset description.

  - `file_type: optional string`

    Asset file type.

### Asset Update Response

- `AssetUpdateResponse object { id, name, created, 2 more }`

  - `id: number`

    Asset ID.

  - `name: string`

    Asset name.

  - `created: optional string`

    Defines the asset creation time.

  - `description: optional string`

    Asset description.

  - `file_type: optional string`

    Asset file type.

### Asset Delete Response

- `AssetDeleteResponse object { errors, messages, success }`

  - `errors: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `messages: array of object { code, message, documentation_url, source }`

    - `code: number`

    - `message: string`

    - `documentation_url: optional string`

    - `source: optional object { pointer }`

      - `pointer: optional string`

  - `success: true`

    Whether the API call was successful.

    - `true`

# Threat Events

## Filter and list events

**get** `/accounts/{account_id}/cloudforce-one/events`

Use `datasetId=all` or `datasetId=*` to query all event datasets for the account (limited to 50). When `datasetId` is unspecified, events are listed from the default Cloudforce One Threat Events dataset. To list existing datasets, use the [`List Datasets`](https://developers.cloudflare.com/api/resources/cloudforce_one/subresources/threat_events/subresources/datasets/methods/list/) endpoint.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cache: optional "from-graph"`

  Cache strategy. 'from-graph' serves results from the graph-node KV cache when all requested UUIDs are cached; falls back to normal path on partial/zero hit.

  - `"from-graph"`

- `cursor: optional string`

  Cursor for pagination. When provided, filters are embedded in the cursor so you only need to pass cursor and pageSize. Returned in the previous response's result_info.cursor field. Use cursor-based pagination for deep pagination (beyond 100,000 records) or for optimal performance.

- `datasetId: optional array of string`

  Dataset IDs to query events from (array of UUIDs), or special value 'all' or '*' to query all event datasets for the account. If not provided, uses the default dataset.

- `forceRefresh: optional boolean`

- `format: optional "json" or "stix2" or "taxii"`

  - `"json"`

  - `"stix2"`

  - `"taxii"`

- `order: optional "asc" or "desc"`

  - `"asc"`

  - `"desc"`

- `orderBy: optional string`

- `page: optional number`

  Page number (1-indexed) for offset-based pagination. Limited to offset of 100,000 records. For deep pagination, use cursor-based pagination instead.

- `pageSize: optional number`

  Number of results per page. Maximum 25,000.

- `search: optional array of object { field, op, value }`

  - `field: optional string`

    Event field to search on. Allowed: attacker, attackerCountry, category, createdAt, date, event, indicator, indicatorType, killChain, mitreAttack, tags, targetCountry, targetIndustry, tlp, uuid.

  - `op: optional "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'tags', op:'in', value:['malware','apt']}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: optional string or number or array of string or number`

    Search value. String or number for most operators. Array for 'in' operator (max 100 items).

    - `string`

    - `number`

    - `array of string or number`

      - `string`

      - `number`

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "attacker": "Flying Yeti",
    "attackerCountry": "CN",
    "attackerCountryAlpha3": "CHN",
    "category": "Domain Resolution",
    "datasetId": "dataset-example-id",
    "date": "2022-04-01T00:00:00Z",
    "event": "An attacker registered the domain domain.com",
    "hasChildren": true,
    "indicator": "domain.com",
    "indicatorType": "domain",
    "indicatorTypeId": 5,
    "killChain": 0,
    "mitreAttack": [
      " "
    ],
    "mitreCapec": [
      " "
    ],
    "numReferenced": 0,
    "numReferences": 0,
    "rawId": "453gw34w3",
    "referenced": [
      " "
    ],
    "referencedIds": [
      0
    ],
    "references": [
      " "
    ],
    "referencesIds": [
      0
    ],
    "tags": [
      "malware"
    ],
    "targetCountry": "US",
    "targetCountryAlpha3": "USA",
    "targetIndustry": "Agriculture",
    "tlp": "amber",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "insight": "insight",
    "releasabilityId": "releasabilityId"
  }
]
```

## Reads an event

**get** `/accounts/{account_id}/cloudforce-one/events/{event_id}`

This Method is deprecated. Please use /events/dataset/:dataset_id/events/:event_id instead.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/$EVENT_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "attacker": "Flying Yeti",
  "attackerCountry": "CN",
  "attackerCountryAlpha3": "CHN",
  "category": "Domain Resolution",
  "datasetId": "dataset-example-id",
  "date": "2022-04-01T00:00:00Z",
  "event": "An attacker registered the domain domain.com",
  "hasChildren": true,
  "indicator": "domain.com",
  "indicatorType": "domain",
  "indicatorTypeId": 5,
  "killChain": 0,
  "mitreAttack": [
    " "
  ],
  "mitreCapec": [
    " "
  ],
  "numReferenced": 0,
  "numReferences": 0,
  "rawId": "453gw34w3",
  "referenced": [
    " "
  ],
  "referencedIds": [
    0
  ],
  "references": [
    " "
  ],
  "referencesIds": [
    0
  ],
  "tags": [
    "malware"
  ],
  "targetCountry": "US",
  "targetCountryAlpha3": "USA",
  "targetIndustry": "Agriculture",
  "tlp": "amber",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "insight": "insight",
  "releasabilityId": "releasabilityId"
}
```

## Creates a new event

**post** `/accounts/{account_id}/cloudforce-one/events/create`

To create a dataset, see the [`Create Dataset`](https://developers.cloudflare.com/api/resources/cloudforce_one/subresources/threat_events/subresources/datasets/methods/create/) endpoint. When `datasetId` parameter is unspecified, it will be created in a default dataset named `Cloudforce One Threat Events`.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `category: string`

- `date: string`

- `event: string`

- `raw: object { data, source, tlp }`

  - `data: map[unknown]`

  - `source: optional string`

  - `tlp: optional string`

- `tlp: string`

- `accountId: optional number`

- `attacker: optional string`

- `attackerCountry: optional string`

- `datasetId: optional string`

- `indicator: optional string`

- `indicators: optional array of object { indicatorType, value }`

  Array of indicators for this event. Supports multiple indicators per event for complex scenarios.

  - `indicatorType: string`

    The type of indicator (e.g., DOMAIN, IP, JA3, HASH)

  - `value: string`

    The indicator value (e.g., domain name, IP address, hash)

- `indicatorType: optional string`

- `insight: optional string`

- `tags: optional array of string`

- `targetCountry: optional string`

- `targetIndustry: optional string`

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "category": "Domain Resolution",
          "date": "2022-04-01T00:00:00Z",
          "event": "An attacker registered the domain domain.com",
          "raw": {
            "data": {
              "foo": "bar"
            }
          },
          "tlp": "amber",
          "accountId": 123456,
          "attacker": "Flying Yeti",
          "attackerCountry": "CN",
          "datasetId": "durableObjectName",
          "indicator": "domain.com",
          "indicatorType": "domain",
          "insight": "This domain was likely registered for phishing purposes",
          "targetCountry": "US",
          "targetIndustry": "Agriculture"
        }'
```

#### Response

```json
{
  "attacker": "Flying Yeti",
  "attackerCountry": "CN",
  "attackerCountryAlpha3": "CHN",
  "category": "Domain Resolution",
  "datasetId": "dataset-example-id",
  "date": "2022-04-01T00:00:00Z",
  "event": "An attacker registered the domain domain.com",
  "hasChildren": true,
  "indicator": "domain.com",
  "indicatorType": "domain",
  "indicatorTypeId": 5,
  "killChain": 0,
  "mitreAttack": [
    " "
  ],
  "mitreCapec": [
    " "
  ],
  "numReferenced": 0,
  "numReferences": 0,
  "rawId": "453gw34w3",
  "referenced": [
    " "
  ],
  "referencedIds": [
    0
  ],
  "references": [
    " "
  ],
  "referencesIds": [
    0
  ],
  "tags": [
    "malware"
  ],
  "targetCountry": "US",
  "targetCountryAlpha3": "USA",
  "targetIndustry": "Agriculture",
  "tlp": "amber",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "insight": "insight",
  "releasabilityId": "releasabilityId"
}
```

## Updates an event

**patch** `/accounts/{account_id}/cloudforce-one/events/{event_id}`

Partially updates a threat event in Cloudforce One, modifying specific fields without replacing the entire event.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Body Parameters

- `datasetId: string`

  Dataset ID containing the event to update.

- `attacker: optional string`

- `attackerCountry: optional string`

- `category: optional string`

- `createdAt: optional string`

- `date: optional string`

- `event: optional string`

- `indicator: optional string`

- `indicatorType: optional string`

- `insight: optional string`

- `raw: optional object { data, source, tlp }`

  - `data: optional map[unknown]`

  - `source: optional string`

  - `tlp: optional string`

- `targetCountry: optional string`

- `targetIndustry: optional string`

- `tlp: optional string`

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/$EVENT_ID \
    -X PATCH \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "datasetId": "9b769969-a211-466c-8ac3-cb91266a066a",
          "attacker": "Flying Yeti",
          "attackerCountry": "CN",
          "category": "Domain Resolution",
          "createdAt": "2025-12-19T00:00:00Z",
          "date": "2022-04-01T00:00:00Z",
          "event": "An attacker registered the domain domain.com",
          "indicator": "domain2.com",
          "indicatorType": "domain",
          "insight": "new insight",
          "targetCountry": "US",
          "targetIndustry": "Insurance",
          "tlp": "amber"
        }'
```

#### Response

```json
{
  "attacker": "Flying Yeti",
  "attackerCountry": "CN",
  "attackerCountryAlpha3": "CHN",
  "category": "Domain Resolution",
  "datasetId": "dataset-example-id",
  "date": "2022-04-01T00:00:00Z",
  "event": "An attacker registered the domain domain.com",
  "hasChildren": true,
  "indicator": "domain.com",
  "indicatorType": "domain",
  "indicatorTypeId": 5,
  "killChain": 0,
  "mitreAttack": [
    " "
  ],
  "mitreCapec": [
    " "
  ],
  "numReferenced": 0,
  "numReferences": 0,
  "rawId": "453gw34w3",
  "referenced": [
    " "
  ],
  "referencedIds": [
    0
  ],
  "references": [
    " "
  ],
  "referencesIds": [
    0
  ],
  "tags": [
    "malware"
  ],
  "targetCountry": "US",
  "targetCountryAlpha3": "USA",
  "targetIndustry": "Agriculture",
  "tlp": "amber",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "insight": "insight",
  "releasabilityId": "releasabilityId"
}
```

## Creates bulk events

**post** `/accounts/{account_id}/cloudforce-one/events/create/bulk`

The `datasetId` parameter must be defined. To list existing datasets (and their IDs) in your account, use the [`List Datasets`](https://developers.cloudflare.com/api/resources/cloudforce_one/subresources/threat_events/subresources/datasets/methods/list/) endpoint.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `data: array of object { category, date, event, 13 more }`

  - `category: string`

  - `date: string`

  - `event: string`

  - `raw: object { data, source, tlp }`

    - `data: map[unknown]`

    - `source: optional string`

    - `tlp: optional string`

  - `tlp: string`

  - `accountId: optional number`

  - `attacker: optional string`

  - `attackerCountry: optional string`

  - `datasetId: optional string`

  - `indicator: optional string`

  - `indicators: optional array of object { indicatorType, value }`

    Array of indicators for this event. Supports multiple indicators per event for complex scenarios.

    - `indicatorType: string`

      The type of indicator (e.g., DOMAIN, IP, JA3, HASH)

    - `value: string`

      The indicator value (e.g., domain name, IP address, hash)

  - `indicatorType: optional string`

  - `insight: optional string`

  - `tags: optional array of string`

  - `targetCountry: optional string`

  - `targetIndustry: optional string`

- `datasetId: string`

- `includeCreatedEvents: optional boolean`

  When true, response includes array of created event UUIDs and shard IDs. Useful for tracking which events were created and where.

### Returns

- `createdEventsCount: number`

  Number of events created

- `createdTagsCount: number`

  Number of new tags created in SoT

- `errorCount: number`

  Number of errors encountered

- `queuedIndicatorsCount: number`

  Number of indicators queued for async processing

- `createBulkEventsRequestId: optional string`

  Correlation ID for async indicator processing

- `createdEvents: optional array of object { eventIndex, shardId, uuid }`

  Array of created events with UUIDs and shard locations. Only present when includeCreatedEvents=true

  - `eventIndex: number`

    Original index in the input data array

  - `shardId: string`

    Dataset ID of the shard where the event was created

  - `uuid: string`

    UUID of the created event

- `errors: optional array of object { error, eventIndex }`

  Array of error details

  - `error: string`

    Error message

  - `eventIndex: number`

    Index of the event that caused the error

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/create/bulk \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "data": [
            {
              "category": "Domain Resolution",
              "date": "2022-04-01T00:00:00Z",
              "event": "An attacker registered the domain domain.com",
              "raw": {
                "data": {
                  "foo": "bar"
                }
              },
              "tlp": "amber"
            }
          ],
          "datasetId": "durableObjectName"
        }'
```

#### Response

```json
{
  "createdEventsCount": 0,
  "createdTagsCount": 0,
  "errorCount": 0,
  "queuedIndicatorsCount": 0,
  "createBulkEventsRequestId": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
  "createdEvents": [
    {
      "eventIndex": 0,
      "shardId": "shardId",
      "uuid": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e"
    }
  ],
  "errors": [
    {
      "error": "error",
      "eventIndex": 0
    }
  ]
}
```

## Creates bulk DOS event with relationships and indicators

**post** `/accounts/{account_id}/cloudforce-one/events/create/bulk/relationships`

This method is deprecated. Please use `event_create_bulk` instead

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `data: array of object { category, date, event, 13 more }`

  - `category: string`

  - `date: string`

  - `event: string`

  - `raw: object { data, source, tlp }`

    - `data: map[unknown]`

    - `source: optional string`

    - `tlp: optional string`

  - `tlp: string`

  - `accountId: optional number`

  - `attacker: optional string`

  - `attackerCountry: optional string`

  - `datasetId: optional string`

  - `indicator: optional string`

  - `indicators: optional array of object { indicatorType, value }`

    Array of indicators for this event. Supports multiple indicators per event for complex scenarios.

    - `indicatorType: string`

      The type of indicator (e.g., DOMAIN, IP, JA3, HASH)

    - `value: string`

      The indicator value (e.g., domain name, IP address, hash)

  - `indicatorType: optional string`

  - `insight: optional string`

  - `tags: optional array of string`

  - `targetCountry: optional string`

  - `targetIndustry: optional string`

- `datasetId: string`

### Returns

- `createdEventsCount: number`

  Number of events created

- `createdIndicatorsCount: number`

  Number of indicators created

- `createdRelationshipsCount: number`

  Number of relationships created

- `errorCount: number`

  Number of errors encountered

- `errors: optional array of object { error, eventIndex }`

  Array of error details

  - `error: string`

    Error message

  - `eventIndex: number`

    Index of the event that caused the error

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/create/bulk/relationships \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "data": [
            {
              "category": "Domain Resolution",
              "date": "2022-04-01T00:00:00Z",
              "event": "An attacker registered the domain domain.com",
              "raw": {
                "data": {
                  "foo": "bar"
                }
              },
              "tlp": "amber"
            }
          ],
          "datasetId": "durableObjectName"
        }'
```

#### Response

```json
{
  "createdEventsCount": 0,
  "createdIndicatorsCount": 0,
  "createdRelationshipsCount": 0,
  "errorCount": 0,
  "errors": [
    {
      "error": "error",
      "eventIndex": 0
    }
  ]
}
```

## Domain Types

### Threat Event List Response

- `ThreatEventListResponse = array of object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`

### Threat Event Get Response

- `ThreatEventGetResponse object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`

### Threat Event Create Response

- `ThreatEventCreateResponse object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`

### Threat Event Edit Response

- `ThreatEventEditResponse object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`

### Threat Event Bulk Create Response

- `ThreatEventBulkCreateResponse object { createdEventsCount, createdTagsCount, errorCount, 4 more }`

  Detailed result of bulk event creation with auto-tag management

  - `createdEventsCount: number`

    Number of events created

  - `createdTagsCount: number`

    Number of new tags created in SoT

  - `errorCount: number`

    Number of errors encountered

  - `queuedIndicatorsCount: number`

    Number of indicators queued for async processing

  - `createBulkEventsRequestId: optional string`

    Correlation ID for async indicator processing

  - `createdEvents: optional array of object { eventIndex, shardId, uuid }`

    Array of created events with UUIDs and shard locations. Only present when includeCreatedEvents=true

    - `eventIndex: number`

      Original index in the input data array

    - `shardId: string`

      Dataset ID of the shard where the event was created

    - `uuid: string`

      UUID of the created event

  - `errors: optional array of object { error, eventIndex }`

    Array of error details

    - `error: string`

      Error message

    - `eventIndex: number`

      Index of the event that caused the error

### Threat Event Bulk Create Relationships Response

- `ThreatEventBulkCreateRelationshipsResponse object { createdEventsCount, createdIndicatorsCount, createdRelationshipsCount, 2 more }`

  Result of bulk relationship creation operation

  - `createdEventsCount: number`

    Number of events created

  - `createdIndicatorsCount: number`

    Number of indicators created

  - `createdRelationshipsCount: number`

    Number of relationships created

  - `errorCount: number`

    Number of errors encountered

  - `errors: optional array of object { error, eventIndex }`

    Array of error details

    - `error: string`

      Error message

    - `eventIndex: number`

      Index of the event that caused the error

# Aggregate

## Aggregate events by single or multiple columns with optional date filtering

**get** `/accounts/{account_id}/cloudforce-one/events/aggregate`

Aggregate threat events by one or more columns (e.g., attacker, targetIndustry) with optional date filtering and daily grouping. Supports multi-dimensional aggregation for cross-analysis.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `aggregateBy: string`

  Column(s) to aggregate by - single column or comma-separated list (e.g., 'attacker', 'targetIndustry', 'attacker,targetIndustry')

- `datasetId: optional array of string`

  Dataset ID(s) to filter by. Can be a single dataset ID, comma-separated list, or array. If not provided, uses default dataset

- `endDate: optional string`

  End date for filtering (ISO 8601 format, e.g., '2024-12-31')

- `groupByDate: optional boolean`

  Whether to group results by date (daily aggregation)

- `limit: optional number`

  Maximum number of results to return

- `startDate: optional string`

  Start date for filtering (ISO 8601 format, e.g., '2024-01-01')

### Returns

- `aggregateBy: string`

  Column(s) that were aggregated by

- `aggregations: array of object { count, date }`

  Array of aggregation results with dynamic fields based on aggregateBy columns

  - `count: number`

    Number of events for this aggregation

  - `date: optional string`

    Date (if groupByDate is true)

- `total: number`

  Total number of events in the aggregation

- `dateRange: optional object { endDate, startDate }`

  Date range used for filtering

  - `endDate: optional string`

  - `startDate: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/aggregate \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "aggregateBy": "aggregateBy",
  "aggregations": [
    {
      "count": 0,
      "date": "date"
    }
  ],
  "total": 0,
  "dateRange": {
    "endDate": "endDate",
    "startDate": "startDate"
  }
}
```

## Domain Types

### Aggregate List Response

- `AggregateListResponse object { aggregateBy, aggregations, total, dateRange }`

  - `aggregateBy: string`

    Column(s) that were aggregated by

  - `aggregations: array of object { count, date }`

    Array of aggregation results with dynamic fields based on aggregateBy columns

    - `count: number`

      Number of events for this aggregation

    - `date: optional string`

      Date (if groupByDate is true)

  - `total: number`

    Total number of events in the aggregation

  - `dateRange: optional object { endDate, startDate }`

    Date range used for filtering

    - `endDate: optional string`

    - `startDate: optional string`

# Graphql

## GraphQL endpoint for event aggregation

**post** `/accounts/{account_id}/cloudforce-one/events/graphql`

Execute GraphQL aggregations over threat events. Supports multi-dimensional group-bys, optional date range filtering, and multi-dataset aggregation.

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `data: optional unknown`

- `errors: optional array of unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/graphql \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "data": {},
  "errors": [
    {}
  ]
}
```

## Domain Types

### Graphql Create Response

- `GraphqlCreateResponse object { data, errors }`

  - `data: optional unknown`

  - `errors: optional array of unknown`

# Graph

## Query graph neighborhood from R2 Data Catalog

**get** `/accounts/{account_id}/cloudforce-one/events/graph`

Expands the single-level relationship neighborhood of one or more seed nodes (event, indicator, or tag) from R2 Data Catalog. Seeds use compact id format (type:uuid), e.g. "event:550e8400-...". Multi-seed requests merge and deduplicate results server-side. Hydrates neighbor entities with summary data from Durable Objects. Supports filtering by relationship type and dataset scope.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cursor: optional string`

  Opaque pagination token. Only valid when seeds has exactly 1 entry; 400 otherwise.

- `datasetIds: optional array of string`

  Comma-separated dataset UUIDs to restrict neighbor scope. Intersected with ACL grants.

- `direction: optional string`

  Edge direction relative to each seed: out (seed→neighbors), in (neighbors→seed), both (default).

- `expand: optional array of string`

  Comma-separated list of response sections to expand (hydrate). Allowed: `nodes`. Omitting `expand` returns identifier-only nodes.

- `hydration: optional string`

  Hydration strategy for neighbor nodes when expand=nodes is set. r2_join (default): use R2 JOIN query + DO fallback. do_only: use plain R2 query + hydrate all neighbors via Durable Objects.

- `limit: optional number`

  Max neighbors per seed (default: 100, max: 1000). Values above 1000 return 400.

- `max_nodes: optional number`

  Total accumulated node cap across all seeds (default: 500, max: 1000). Values above 1000 return 400.

- `relationshipTypes: optional array of string`

  Comma-separated relationship types to filter by. Allowed: tagged_with, appears_in, related_to, caused_by, attributed_to.

- `seeds: optional array of string`

  Comma-separated compact seed ids (type:uuid). Example: seeds=event:550e8400-...,indicator:661fa920-... Provide 1–50 entries; omitting seeds returns 400.

### Returns

- `errors: array of map[unknown]`

- `messages: array of map[unknown]`

- `result: object { edges, node, nodes }`

  - `edges: array of object { id, relationshipType, source, 5 more }`

    - `id: string`

      Deterministic composite edge id (source→target:relationshipType)

    - `relationshipType: string`

    - `source: string`

      Compact id of the source node (type:uuid)

    - `sourceId: string`

    - `sourceType: string`

    - `target: string`

      Compact id of the target node (type:uuid)

    - `targetId: string`

    - `targetType: string`

  - `node: map[unknown]`

    Focal node object (legacy single-seed). Null when unavailable.

  - `nodes: array of map[unknown]`

- `success: boolean`

- `result_info: optional object { count, edge_count, query_time_ms, 6 more }`

  - `count: number`

    Number of nodes in result.nodes (seeds + neighbors)

  - `edge_count: number`

    Number of edges in result.edges

  - `query_time_ms: number`

    Total query time in milliseconds

  - `total_count: number`

    Total count of nodes (same as count for this endpoint)

  - `cursor: optional string`

    Opaque pagination cursor for the next page; null when exhausted or for multi-seed requests (single-seed only)

  - `depth_reached: optional number`

    Traversal depth reached (always 1 for single-level)

  - `has_more: optional boolean`

    True when a cursor is available for the next page (single-seed only)

  - `seeds: optional array of string`

    Composite ids of the seed node(s) (type:uuid). Always an array, even for one seed.

  - `truncated: optional boolean`

    True when results were capped (per-seed limit or max_nodes)

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/graph \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [],
  "messages": [],
  "result": {
    "edges": [
      {
        "id": "event:550e8400-e29b-41d4-a716-446655440000→indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04:appears_in",
        "relationshipType": "appears_in",
        "source": "event:550e8400-e29b-41d4-a716-446655440000",
        "sourceId": "550e8400-e29b-41d4-a716-446655440000",
        "sourceType": "event",
        "target": "indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "targetId": "661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "targetType": "indicator"
      }
    ],
    "node": null,
    "nodes": [
      {
        "attacker": "APT28",
        "category": "intrusion",
        "datasetId": "a1b2c3d4-0001-4000-8000-000000000001",
        "date": "2026-06-01",
        "event": "Attacker registered domain evil.example.com",
        "id": "event:550e8400-e29b-41d4-a716-446655440000",
        "role": "focal",
        "type": "event",
        "uuid": "550e8400-e29b-41d4-a716-446655440000"
      },
      {
        "datasetId": "a1b2c3d4-0001-4000-8000-000000000001",
        "id": "indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "indicatorType": "domain",
        "role": "focal",
        "type": "indicator",
        "uuid": "661fa920-bbf3-4e71-9c55-2a3d8e7f1b04",
        "value": "evil.example.com"
      },
      {
        "categoryId": "threat-actor",
        "id": "tag:772af1c8-dc4a-4a29-b3e6-4f8c9d2a6e71",
        "type": "tag",
        "uuid": "772af1c8-dc4a-4a29-b3e6-4f8c9d2a6e71",
        "value": "APT28"
      }
    ]
  },
  "result_info": {
    "count": 3,
    "cursor": null,
    "depth_reached": 1,
    "edge_count": 1,
    "has_more": false,
    "query_time_ms": 890,
    "seeds": [
      "event:550e8400-e29b-41d4-a716-446655440000",
      "indicator:661fa920-bbf3-4e71-9c55-2a3d8e7f1b04"
    ],
    "total_count": 3,
    "truncated": false
  },
  "success": true
}
```

## Domain Types

### Graph List Response

- `GraphListResponse object { edges, node, nodes }`

  - `edges: array of object { id, relationshipType, source, 5 more }`

    - `id: string`

      Deterministic composite edge id (source→target:relationshipType)

    - `relationshipType: string`

    - `source: string`

      Compact id of the source node (type:uuid)

    - `sourceId: string`

    - `sourceType: string`

    - `target: string`

      Compact id of the target node (type:uuid)

    - `targetId: string`

    - `targetType: string`

  - `node: map[unknown]`

    Focal node object (legacy single-seed). Null when unavailable.

  - `nodes: array of map[unknown]`

# Queries

## List all saved event queries

**get** `/accounts/{account_id}/cloudforce-one/events/queries`

Retrieve all saved event queries for the account

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `id: number`

  Unique identifier for the saved query

- `account_id: number`

  Account ID

- `alert_enabled: boolean`

  Whether alerts are enabled

- `alert_rollup_enabled: boolean`

  Whether alert rollup is enabled

- `created_at: string`

  Creation timestamp

- `name: string`

  Name of the saved query

- `query_json: string`

  JSON string containing the query parameters

- `rule_enabled: boolean`

  Whether rule is enabled

- `updated_at: string`

  Last update timestamp

- `user_email: string`

  Email of the user who created the query

- `custom_threat_feed_id: optional number`

  Intel Indicator Feed ID (numeric)

- `rule_list_id: optional string`

  WAF rules list ID for blocking

- `rule_scope: optional string`

  Scope for the rule

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "id": 0,
    "account_id": 0,
    "alert_enabled": true,
    "alert_rollup_enabled": true,
    "created_at": "created_at",
    "name": "name",
    "query_json": "query_json",
    "rule_enabled": true,
    "updated_at": "updated_at",
    "user_email": "user_email",
    "custom_threat_feed_id": 0,
    "rule_list_id": "rule_list_id",
    "rule_scope": "rule_scope"
  }
]
```

## Create a saved event query

**post** `/accounts/{account_id}/cloudforce-one/events/queries/create`

Create a new saved event query for the account

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `alert_enabled: boolean`

  Enable alerts for this query

- `alert_rollup_enabled: boolean`

  Enable alert rollup for this query

- `name: string`

  Unique name for the saved query

- `query_json: string`

  JSON string containing the query parameters

- `rule_enabled: boolean`

  Enable rule for this query

- `rule_scope: optional string`

  Scope for the rule

### Returns

- `id: number`

  Unique identifier for the saved query

- `account_id: number`

  Account ID

- `alert_enabled: boolean`

  Whether alerts are enabled

- `alert_rollup_enabled: boolean`

  Whether alert rollup is enabled

- `created_at: string`

  Creation timestamp

- `name: string`

  Name of the saved query

- `query_json: string`

  JSON string containing the query parameters

- `rule_enabled: boolean`

  Whether rule is enabled

- `updated_at: string`

  Last update timestamp

- `user_email: string`

  Email of the user who created the query

- `custom_threat_feed_id: optional number`

  Intel Indicator Feed ID (numeric)

- `rule_list_id: optional string`

  WAF rules list ID for blocking

- `rule_scope: optional string`

  Scope for the rule

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "alert_enabled": true,
          "alert_rollup_enabled": true,
          "name": "name",
          "query_json": "query_json",
          "rule_enabled": true
        }'
```

#### Response

```json
{
  "id": 0,
  "account_id": 0,
  "alert_enabled": true,
  "alert_rollup_enabled": true,
  "created_at": "created_at",
  "name": "name",
  "query_json": "query_json",
  "rule_enabled": true,
  "updated_at": "updated_at",
  "user_email": "user_email",
  "custom_threat_feed_id": 0,
  "rule_list_id": "rule_list_id",
  "rule_scope": "rule_scope"
}
```

## Read a saved event query

**get** `/accounts/{account_id}/cloudforce-one/events/queries/{query_id}`

Retrieve a saved event query by its ID

### Path Parameters

- `account_id: string`

  Account ID.

- `query_id: number`

  Event query ID

### Returns

- `id: number`

  Unique identifier for the saved query

- `account_id: number`

  Account ID

- `alert_enabled: boolean`

  Whether alerts are enabled

- `alert_rollup_enabled: boolean`

  Whether alert rollup is enabled

- `created_at: string`

  Creation timestamp

- `name: string`

  Name of the saved query

- `query_json: string`

  JSON string containing the query parameters

- `rule_enabled: boolean`

  Whether rule is enabled

- `updated_at: string`

  Last update timestamp

- `user_email: string`

  Email of the user who created the query

- `custom_threat_feed_id: optional number`

  Intel Indicator Feed ID (numeric)

- `rule_list_id: optional string`

  WAF rules list ID for blocking

- `rule_scope: optional string`

  Scope for the rule

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries/$QUERY_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": 0,
  "account_id": 0,
  "alert_enabled": true,
  "alert_rollup_enabled": true,
  "created_at": "created_at",
  "name": "name",
  "query_json": "query_json",
  "rule_enabled": true,
  "updated_at": "updated_at",
  "user_email": "user_email",
  "custom_threat_feed_id": 0,
  "rule_list_id": "rule_list_id",
  "rule_scope": "rule_scope"
}
```

## Update a saved event query

**patch** `/accounts/{account_id}/cloudforce-one/events/queries/{query_id}`

Update an existing saved event query by its ID

### Path Parameters

- `account_id: string`

  Account ID.

- `query_id: number`

  Event query ID

### Body Parameters

- `alert_enabled: optional boolean`

  Enable alerts for this query

- `alert_rollup_enabled: optional boolean`

  Enable alert rollup for this query

- `name: optional string`

  Unique name for the saved query

- `query_json: optional string`

  JSON string containing the query parameters

- `rule_enabled: optional boolean`

  Enable rule for this query

- `rule_scope: optional string`

  Scope for the rule

### Returns

- `id: number`

  Unique identifier for the saved query

- `account_id: number`

  Account ID

- `alert_enabled: boolean`

  Whether alerts are enabled

- `alert_rollup_enabled: boolean`

  Whether alert rollup is enabled

- `created_at: string`

  Creation timestamp

- `name: string`

  Name of the saved query

- `query_json: string`

  JSON string containing the query parameters

- `rule_enabled: boolean`

  Whether rule is enabled

- `updated_at: string`

  Last update timestamp

- `user_email: string`

  Email of the user who created the query

- `custom_threat_feed_id: optional number`

  Intel Indicator Feed ID (numeric)

- `rule_list_id: optional string`

  WAF rules list ID for blocking

- `rule_scope: optional string`

  Scope for the rule

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries/$QUERY_ID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": 0,
  "account_id": 0,
  "alert_enabled": true,
  "alert_rollup_enabled": true,
  "created_at": "created_at",
  "name": "name",
  "query_json": "query_json",
  "rule_enabled": true,
  "updated_at": "updated_at",
  "user_email": "user_email",
  "custom_threat_feed_id": 0,
  "rule_list_id": "rule_list_id",
  "rule_scope": "rule_scope"
}
```

## Delete a saved event query

**delete** `/accounts/{account_id}/cloudforce-one/events/queries/{query_id}`

Delete a saved event query by its ID

### Path Parameters

- `account_id: string`

  Account ID.

- `query_id: number`

  Event query ID

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/queries/$QUERY_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

## Domain Types

### Query List Response

- `QueryListResponse = array of object { id, account_id, alert_enabled, 10 more }`

  - `id: number`

    Unique identifier for the saved query

  - `account_id: number`

    Account ID

  - `alert_enabled: boolean`

    Whether alerts are enabled

  - `alert_rollup_enabled: boolean`

    Whether alert rollup is enabled

  - `created_at: string`

    Creation timestamp

  - `name: string`

    Name of the saved query

  - `query_json: string`

    JSON string containing the query parameters

  - `rule_enabled: boolean`

    Whether rule is enabled

  - `updated_at: string`

    Last update timestamp

  - `user_email: string`

    Email of the user who created the query

  - `custom_threat_feed_id: optional number`

    Intel Indicator Feed ID (numeric)

  - `rule_list_id: optional string`

    WAF rules list ID for blocking

  - `rule_scope: optional string`

    Scope for the rule

### Query Create Response

- `QueryCreateResponse object { id, account_id, alert_enabled, 10 more }`

  - `id: number`

    Unique identifier for the saved query

  - `account_id: number`

    Account ID

  - `alert_enabled: boolean`

    Whether alerts are enabled

  - `alert_rollup_enabled: boolean`

    Whether alert rollup is enabled

  - `created_at: string`

    Creation timestamp

  - `name: string`

    Name of the saved query

  - `query_json: string`

    JSON string containing the query parameters

  - `rule_enabled: boolean`

    Whether rule is enabled

  - `updated_at: string`

    Last update timestamp

  - `user_email: string`

    Email of the user who created the query

  - `custom_threat_feed_id: optional number`

    Intel Indicator Feed ID (numeric)

  - `rule_list_id: optional string`

    WAF rules list ID for blocking

  - `rule_scope: optional string`

    Scope for the rule

### Query Get Response

- `QueryGetResponse object { id, account_id, alert_enabled, 10 more }`

  - `id: number`

    Unique identifier for the saved query

  - `account_id: number`

    Account ID

  - `alert_enabled: boolean`

    Whether alerts are enabled

  - `alert_rollup_enabled: boolean`

    Whether alert rollup is enabled

  - `created_at: string`

    Creation timestamp

  - `name: string`

    Name of the saved query

  - `query_json: string`

    JSON string containing the query parameters

  - `rule_enabled: boolean`

    Whether rule is enabled

  - `updated_at: string`

    Last update timestamp

  - `user_email: string`

    Email of the user who created the query

  - `custom_threat_feed_id: optional number`

    Intel Indicator Feed ID (numeric)

  - `rule_list_id: optional string`

    WAF rules list ID for blocking

  - `rule_scope: optional string`

    Scope for the rule

### Query Edit Response

- `QueryEditResponse object { id, account_id, alert_enabled, 10 more }`

  - `id: number`

    Unique identifier for the saved query

  - `account_id: number`

    Account ID

  - `alert_enabled: boolean`

    Whether alerts are enabled

  - `alert_rollup_enabled: boolean`

    Whether alert rollup is enabled

  - `created_at: string`

    Creation timestamp

  - `name: string`

    Name of the saved query

  - `query_json: string`

    JSON string containing the query parameters

  - `rule_enabled: boolean`

    Whether rule is enabled

  - `updated_at: string`

    Last update timestamp

  - `user_email: string`

    Email of the user who created the query

  - `custom_threat_feed_id: optional number`

    Intel Indicator Feed ID (numeric)

  - `rule_list_id: optional string`

    WAF rules list ID for blocking

  - `rule_scope: optional string`

    Scope for the rule

# Relationships

## Filter and list events related to specific event

**get** `/accounts/{account_id}/cloudforce-one/events/{event_id}/relationships`

The `event_id` must be defined (to list existing events (and their IDs), use the [`Filter and List Events`](https://developers.cloudflare.com/api/resources/cloudforce_one/subresources/threat_events/methods/list/) endpoint). Also, must provide query parameters.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Query Parameters

- `datasetId: string`

  The dataset ID to search within.

- `direction: optional "ancestors" or "descendants" or "both"`

  The direction to traverse the graph. Defaults to 'both' to search all.

  - `"ancestors"`

  - `"descendants"`

  - `"both"`

- `includeParent: optional boolean`

  Whether to include the starting event in the results. Defaults to true.

- `indicatorTypeIds: optional array of string`

  An optional array of indicator type IDs to filter the results by.

- `maxDepth: optional number`

  The maximum depth to traverse. Defaults to 5.

- `page: optional number`

- `pageSize: optional number`

- `relationshipTypes: optional string or array of string`

  An optional array of relationship types to filter by.

  - `string`

  - `array of string`

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/$EVENT_ID/relationships \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "attacker": "Flying Yeti",
    "attackerCountry": "CN",
    "attackerCountryAlpha3": "CHN",
    "category": "Domain Resolution",
    "datasetId": "dataset-example-id",
    "date": "2022-04-01T00:00:00Z",
    "event": "An attacker registered the domain domain.com",
    "hasChildren": true,
    "indicator": "domain.com",
    "indicatorType": "domain",
    "indicatorTypeId": 5,
    "killChain": 0,
    "mitreAttack": [
      " "
    ],
    "mitreCapec": [
      " "
    ],
    "numReferenced": 0,
    "numReferences": 0,
    "rawId": "453gw34w3",
    "referenced": [
      " "
    ],
    "referencedIds": [
      0
    ],
    "references": [
      " "
    ],
    "referencesIds": [
      0
    ],
    "tags": [
      "malware"
    ],
    "targetCountry": "US",
    "targetCountryAlpha3": "USA",
    "targetIndustry": "Agriculture",
    "tlp": "amber",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "insight": "insight",
    "releasabilityId": "releasabilityId"
  }
]
```

## Domain Types

### Relationship List Response

- `RelationshipListResponse = array of object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`

# Indicators

## Lists indicators across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/indicators`

Retrieves indicators across specified datasets, ordered by createdAt descending then UUID, dataset ID, and shard ID ascending. Use datasetIds=all or datasetIds=* to query all datasets for the account. If no datasetIds provided, uses the default dataset.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cache: optional "from-graph"`

  Cache strategy. 'from-graph' serves results from the graph-node KV cache when all requested UUIDs are cached; falls back to normal path on partial/zero hit. Cannot be combined with `cursor`.

  - `"from-graph"`

- `createdAfter: optional string`

  Filter indicators created on or after this date. Must use ISO 8601 format (e.g., '2024-01-15T00:00:00Z').

- `createdBefore: optional string`

  Filter indicators created on or before this date. Must use ISO 8601 format (e.g., '2024-12-31T23:59:59Z').

- `cursor: optional string`

  Opaque cursor from a previous response's `pagination.cursor`. When provided, all filters, datasetIds, page, `pageSize`, `includeTags` and `relatedEventsLimit` come from the cursor — do not resend them. Sending any filter, `page`, `pageSize`, `includeTags`, `relatedEventsLimit`, `includeTotalCount=true`, or `cache=from-graph` alongside a cursor yields a 400 `CursorFilterConflictError`. A cursor issued for a different entity, an unsupported version, or a dataset that has since been reconfigured as analytics-only yields a 400 `InvalidCursorError`.

- `datasetIds: optional array of string`

  Dataset IDs to query indicators from (array of UUIDs), or special value 'all' or '*' to query all datasets. If not provided, uses the default dataset.

- `format: optional "json" or "stix2" or "taxii"`

  Output format for indicator data. 'json' returns the default format, 'stix2' returns STIX 2.1 Indicator SDOs, 'taxii' returns a TAXII 2.1 Envelope with Content-Type application/taxii+json;version=2.1.

  - `"json"`

  - `"stix2"`

  - `"taxii"`

- `includeTags: optional boolean`

  Whether to include full tag details for each indicator. Defaults to true.

- `includeTotalCount: optional boolean`

  Whether to compute total count via COUNT(*). Defaults to false for performance. total_count is null unless this is true and the complete fan-out succeeds.

- `indicatorType: optional string`

- `name: optional string`

  Filter indicators by value using substring match (LIKE). Legacy alternative to structured search.

- `page: optional number`

- `pageSize: optional number`

- `relatedEvents: optional array of string`

  Filter by related event IDs

- `relatedEventsLimit: optional number`

  Limit the number of related events returned per indicator. Default: 2. Set to 0 for none, -1 for all events.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType, uuid. Supports operators: equals, not, contains, startsWith, endsWith, gt, lt, gte, lte, like, in, find. Use the 'in' operator with an array value to bulk-check up to 100 indicators in a single request, e.g. search=[{"field":"value","op":"in","value":["evil.com","bad.org"]}]. Multiple conditions are AND'd together. Max 10 conditions per request.

  - `field: "value" or "indicatorType" or "uuid"`

    The indicator field to search on. Allowed: value, indicatorType, uuid.

    - `"value"`

    - `"indicatorType"`

    - `"uuid"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'value', op:'in', value:['evil.com','bad.org']}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: string or array of string`

    Search value. String for most operators. Array of strings for 'in' operator (max 100 items).

    - `string`

    - `array of string`

- `tags: optional array of string`

  Filter by tag values or UUIDs. Indicators must have at least one of the specified tags (OR logic). Supports both tag UUID and tag value.

- `tagSearch: optional array of object { field, op, value }`

  Structured tag-metadata filter as a JSON array of {field, op, value} objects. Operates against the per-dataset IndicatorTag mirror so you can find indicators by tag attributes (origin country, motive, sophistication, priority, etc.) without a separate Tags lookup. Common dashboard usage: drill from a country into indicators, e.g. tagSearch=[{"field":"originCountryISO","op":"in","value":["IR","CN"]}]. Country values may be passed as alpha-2, alpha-3, name, or alias (e.g. "iran"). Operators: equals, not, gt/gte/lt/lte (numeric only), contains/like/find/startsWith/endsWith (string only), in. AND-joined across entries; combined with `tags`, a matching tag must satisfy both. Max 10 entries per request, max 100 values per 'in'. Performance notes: `originCountryISO` uses its B-tree index for equals/not/in. `priority` uses its B-tree index for numeric comparisons. Other string columns (`actorCategory`, `motive`, etc.) are case-insensitive and unindexed; current catalog size makes this a non-issue. `endsWith` and `aliasGroupNames` contains/like are leading-wildcard scans and slow on large result sets. `aliasGroupNames` matches on the JSON-encoded text, so substrings can cross alias boundaries ("apt28" also matches "apt280" when both appear in the same tag's alias list).

  - `field: "value" or "categoryId" or "actorCategory" or 9 more`

    Tag mirror field to filter on. Allowed: value, categoryId, actorCategory, aliasGroupNames, attributionConfidence, attributionOrganization, motive, opsecLevel, originCountryISO, sophisticationLevel, priority, analyticPriority. Filters operate against the per-dataset IndicatorTag mirror (which is kept in sync with the Tags SoT by the tag-propagation workflow).

    - `"value"`

    - `"categoryId"`

    - `"actorCategory"`

    - `"aliasGroupNames"`

    - `"attributionConfidence"`

    - `"attributionOrganization"`

    - `"motive"`

    - `"opsecLevel"`

    - `"originCountryISO"`

    - `"sophisticationLevel"`

    - `"priority"`

    - `"analyticPriority"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk OR within a single field, e.g. {field:"originCountryISO", op:"in", value:["IR","CN"]}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: optional string or number or array of string or number`

    Search value. String or number for most operators. Array for 'in' (max 100 items). Country values may be passed as alpha-2, alpha-3, name, or common alias (e.g. "iran", "IR", "IRN") and are normalized to alpha-2 server-side.

    - `string`

    - `number`

    - `array of string or number`

      - `string`

      - `number`

### Returns

- `properties: object { completeness, indicators, pagination }`

  - `completeness: object { properties, type }`

    - `properties: object { complete, failedDatasets, failedShards, warnings }`

      - `complete: object { type }`

        - `type: string`

      - `failedDatasets: object { items, type }`

        - `items: object { type }`

          - `type: string`

        - `type: string`

      - `failedShards: object { items, type }`

        - `items: object { properties, type }`

          - `properties: object { datasetId, shardId }`

            - `datasetId: object { type }`

              - `type: string`

            - `shardId: object { type }`

              - `type: string`

          - `type: string`

        - `type: string`

      - `warnings: object { items, type }`

        - `items: object { type }`

          - `type: string`

        - `type: string`

    - `type: string`

  - `indicators: object { items, type }`

    - `items: object { createdAt, indicatorType, updatedAt, 5 more }`

      - `createdAt: string`

      - `indicatorType: string`

      - `updatedAt: string`

      - `uuid: string`

      - `value: string`

      - `datasetId: optional string`

        The dataset ID this indicator belongs to. Included in list responses.

      - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

        - `datasetId: string`

        - `eventId: string`

        - `eventDate: optional string`

          ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

      - `tags: optional array of object { categoryName, uuid, value }`

        - `categoryName: optional string`

        - `uuid: optional string`

        - `value: optional string`

    - `type: string`

  - `pagination: object { properties, type }`

    - `properties: object { count, cursor, has_more, 4 more }`

      - `count: object { type }`

        - `type: string`

      - `cursor: object { description, nullable, type }`

        - `description: string`

        - `nullable: boolean`

        - `type: string`

      - `has_more: object { description, type }`

        - `description: string`

        - `type: string`

      - `page: object { type }`

        - `type: string`

      - `per_page: object { type }`

        - `type: string`

      - `total_count: object { description, nullable, type }`

        - `description: string`

        - `nullable: boolean`

        - `type: string`

      - `total_count_is_exact: object { description, type }`

        - `description: string`

        - `type: string`

    - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "properties": {
    "completeness": {
      "properties": {
        "complete": {
          "type": "boolean"
        },
        "failedDatasets": {
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "failedShards": {
          "items": {
            "properties": {
              "datasetId": {
                "type": "string"
              },
              "shardId": {
                "type": "string"
              }
            },
            "type": "object"
          },
          "type": "array"
        },
        "warnings": {
          "items": {
            "type": "string"
          },
          "type": "array"
        }
      },
      "type": "object"
    },
    "indicators": {
      "items": {
        "createdAt": "2022-04-01T00:00:00Z",
        "indicatorType": "domain",
        "updatedAt": "2022-04-01T00:00:00Z",
        "uuid": "12345678-1234-1234-1234-1234567890ab",
        "value": "malicious-domain.com",
        "datasetId": "dataset-uuid-123",
        "relatedEvents": [
          {
            "datasetId": "dataset-uuid-123",
            "eventId": "event-uuid-456",
            "eventDate": "2024-06-15T00:00:00Z"
          }
        ],
        "tags": [
          {
            "categoryName": "categoryName",
            "uuid": "uuid",
            "value": "value"
          }
        ]
      },
      "type": "array"
    },
    "pagination": {
      "properties": {
        "count": {
          "type": "number"
        },
        "cursor": {
          "description": "Opaque cursor for the next page. Pass back as the `cursor` query param on the next request. `null` when the sequence has ended, when the encoded cursor would exceed the safe URL length, or when this endpoint served the request from a backend that does not support cursor pagination (analytics R2 path).",
          "nullable": true,
          "type": "string"
        },
        "has_more": {
          "description": "True when more pages exist after this one. Present on both offset and cursor paths.",
          "type": "boolean"
        },
        "page": {
          "type": "number"
        },
        "per_page": {
          "type": "number"
        },
        "total_count": {
          "description": "Exact matching count when requested and fan-out is complete; otherwise null.",
          "nullable": true,
          "type": "number"
        },
        "total_count_is_exact": {
          "description": "Whether total_count is exact across the complete query fan-out.",
          "type": "boolean"
        }
      },
      "type": "object"
    }
  },
  "type": "object"
}
```

## Domain Types

### Indicator List Response

- `IndicatorListResponse object { properties, type }`

  - `properties: object { completeness, indicators, pagination }`

    - `completeness: object { properties, type }`

      - `properties: object { complete, failedDatasets, failedShards, warnings }`

        - `complete: object { type }`

          - `type: string`

        - `failedDatasets: object { items, type }`

          - `items: object { type }`

            - `type: string`

          - `type: string`

        - `failedShards: object { items, type }`

          - `items: object { properties, type }`

            - `properties: object { datasetId, shardId }`

              - `datasetId: object { type }`

                - `type: string`

              - `shardId: object { type }`

                - `type: string`

            - `type: string`

          - `type: string`

        - `warnings: object { items, type }`

          - `items: object { type }`

            - `type: string`

          - `type: string`

      - `type: string`

    - `indicators: object { items, type }`

      - `items: object { createdAt, indicatorType, updatedAt, 5 more }`

        - `createdAt: string`

        - `indicatorType: string`

        - `updatedAt: string`

        - `uuid: string`

        - `value: string`

        - `datasetId: optional string`

          The dataset ID this indicator belongs to. Included in list responses.

        - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

          - `datasetId: string`

          - `eventId: string`

          - `eventDate: optional string`

            ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

        - `tags: optional array of object { categoryName, uuid, value }`

          - `categoryName: optional string`

          - `uuid: optional string`

          - `value: optional string`

      - `type: string`

    - `pagination: object { properties, type }`

      - `properties: object { count, cursor, has_more, 4 more }`

        - `count: object { type }`

          - `type: string`

        - `cursor: object { description, nullable, type }`

          - `description: string`

          - `nullable: boolean`

          - `type: string`

        - `has_more: object { description, type }`

          - `description: string`

          - `type: string`

        - `page: object { type }`

          - `type: string`

        - `per_page: object { type }`

          - `type: string`

        - `total_count: object { description, nullable, type }`

          - `description: string`

          - `nullable: boolean`

          - `type: string`

        - `total_count_is_exact: object { description, type }`

          - `description: string`

          - `type: string`

      - `type: string`

  - `type: string`

# Aggregate

## Aggregate indicators by column(s)

**get** `/accounts/{account_id}/cloudforce-one/events/indicators/aggregate`

Aggregate threat indicators by one or more columns (e.g., indicatorType, value) across datasets. Returns top-N groups ordered by count.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `aggregateBy: string`

  Column(s) to aggregate by - single column or comma-separated list (e.g., 'indicatorType', 'value', 'indicatorType,value')

- `createdAfter: optional string or string`

  Filter indicators created after this date/datetime (ISO 8601, e.g., '2024-01-01' or '2024-01-01T00:00:00Z')

  - `string`

  - `string`

- `createdBefore: optional string or string`

  Filter indicators created before this date/datetime (ISO 8601, e.g., '2024-12-31' or '2024-12-31T23:59:59Z')

  - `string`

  - `string`

- `datasetIds: optional array of string`

  Dataset ID(s) to filter by. Can be a single dataset ID or comma-separated list. If not provided, aggregates across all accessible datasets

- `eventDateAfter: optional string`

  For measure=relationships: only count event links whose eventDate is on/after this date (ISO 8601). Use to bound 'top indicator' to recent activity.

- `eventDateBefore: optional string`

  For measure=relationships: only count event links whose eventDate is on/before this date (ISO 8601).

- `limit: optional number`

  Maximum number of aggregation results to return (1-100)

- `measure: optional "indicators" or "relationships"`

  What to count per group: 'indicators' (catalog rows, default) or 'relationships' (linked events per indicator). Use 'relationships' for 'top indicator by event activity'.

  - `"indicators"`

  - `"relationships"`

- `tagUuid: optional string`

  Scope to indicators associated with this tag/actor UUID. Combine with measure=relationships for 'top indicator for an actor'.

### Returns

- `aggregateBy: string`

  Column(s) that were aggregated by

- `aggregations: array of object { count }`

  Array of aggregation results with dynamic fields based on aggregateBy columns

  - `count: number`

    Number of indicators for this aggregation

- `failedDatasets: number`

  Number of datasets whose aggregation failed and were excluded from the result

- `total: number`

  Total count in the aggregation: indicator rows when measure=indicators, or linked-event rows when measure=relationships

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/indicators/aggregate \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "aggregateBy": "aggregateBy",
  "aggregations": [
    {
      "count": 0
    }
  ],
  "failedDatasets": 0,
  "total": 0
}
```

## Domain Types

### Aggregate List Response

- `AggregateListResponse object { aggregateBy, aggregations, failedDatasets, total }`

  - `aggregateBy: string`

    Column(s) that were aggregated by

  - `aggregations: array of object { count }`

    Array of aggregation results with dynamic fields based on aggregateBy columns

    - `count: number`

      Number of indicators for this aggregation

  - `failedDatasets: number`

    Number of datasets whose aggregation failed and were excluded from the result

  - `total: number`

    Total count in the aggregation: indicator rows when measure=indicators, or linked-event rows when measure=relationships

# Types

## Lists indicator types across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/indicator-types`

Lists indicator types across multiple datasets

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query indicator types from. If not provided, queries all datasets for the account.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/indicator-types \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### Type List Response

- `TypeListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

# By Dataset

## Lists indicators

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators`

This method is deprecated. Please use /events/indicators to retrieve a paginated list of indicators.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

### Query Parameters

- `indicatorType: optional string`

- `name: optional string`

  Filter by indicator value (substring match)

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

    - `datasetId: string`

    - `eventId: string`

    - `eventDate: optional string`

      ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "indicators": [
    {
      "createdAt": "2022-04-01T00:00:00Z",
      "indicatorType": "domain",
      "updatedAt": "2022-04-01T00:00:00Z",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "malicious-domain.com",
      "datasetId": "dataset-uuid-123",
      "relatedEvents": [
        {
          "datasetId": "dataset-uuid-123",
          "eventId": "event-uuid-456",
          "eventDate": "2024-06-15T00:00:00Z"
        }
      ],
      "tags": [
        {
          "categoryName": "categoryName",
          "uuid": "uuid",
          "value": "value"
        }
      ]
    }
  ],
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  }
}
```

## Reads an indicator

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators/{indicator_id}`

Retrieves a specific indicator by its UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

- `indicator_id: string`

  Indicator UUID.

### Returns

- `createdAt: string`

- `indicatorType: string`

- `updatedAt: string`

- `uuid: string`

- `value: string`

- `datasetId: optional string`

  The dataset ID this indicator belongs to. Included in list responses.

- `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

  - `datasetId: string`

  - `eventId: string`

  - `eventDate: optional string`

    ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

- `tags: optional array of object { categoryName, uuid, value }`

  - `categoryName: optional string`

  - `uuid: optional string`

  - `value: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators/$INDICATOR_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "createdAt": "2022-04-01T00:00:00Z",
  "indicatorType": "domain",
  "updatedAt": "2022-04-01T00:00:00Z",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "value": "malicious-domain.com",
  "datasetId": "dataset-uuid-123",
  "relatedEvents": [
    {
      "datasetId": "dataset-uuid-123",
      "eventId": "event-uuid-456",
      "eventDate": "2024-06-15T00:00:00Z"
    }
  ],
  "tags": [
    {
      "categoryName": "categoryName",
      "uuid": "uuid",
      "value": "value"
    }
  ]
}
```

## Domain Types

### By Dataset List Response

- `ByDatasetListResponse object { indicators, pagination }`

  - `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

    - `createdAt: string`

    - `indicatorType: string`

    - `updatedAt: string`

    - `uuid: string`

    - `value: string`

    - `datasetId: optional string`

      The dataset ID this indicator belongs to. Included in list responses.

    - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

      - `datasetId: string`

      - `eventId: string`

      - `eventDate: optional string`

        ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`

### By Dataset Get Response

- `ByDatasetGetResponse object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

    - `datasetId: string`

    - `eventId: string`

    - `eventDate: optional string`

      ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

# Tags

## List mirrored tags for an indicator dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators/tags`

Returns all mirrored tags from the indicator dataset (DO mirror table). No pagination.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators/tags \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {}
]
```

## Domain Types

### Tag List Response

- `TagListResponse = array of unknown`

  Array of mirror tag rows

# Attackers

## Lists attackers across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/attackers`

Lists known threat attackers tracked in Cloudforce One threat intelligence.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query attackers from. If not provided, uses the default dataset.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/attackers \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### Attacker List Response

- `AttackerListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

# Categories

## Lists categories across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/categories`

Lists all threat event categories configured for classifying and organizing threat events.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query categories from. If not provided, uses the default dataset.

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "killChain": 0,
    "name": "name",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "mitreAttack": [
      "T1234"
    ],
    "mitreCapec": [
      "123"
    ],
    "shortname": "shortname"
  }
]
```

## Reads a category

**get** `/accounts/{account_id}/cloudforce-one/events/categories/{category_id}`

Retrieves details for a specific threat event category.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_id: string`

  Category UUID.

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/$CATEGORY_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "killChain": 0,
  "name": "name",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "mitreAttack": [
    "T1234"
  ],
  "mitreCapec": [
    "123"
  ],
  "shortname": "shortname"
}
```

## Creates a new category

**post** `/accounts/{account_id}/cloudforce-one/events/categories/create`

Creates a new threat event category in Cloudforce One for organizing and classifying threat events.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `killChain: number`

- `name: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "killChain": 0,
          "name": "name",
          "shortname": "shortname"
        }'
```

#### Response

```json
{
  "killChain": 0,
  "name": "name",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "mitreAttack": [
    "T1234"
  ],
  "mitreCapec": [
    "123"
  ],
  "shortname": "shortname"
}
```

## Updates a category

**patch** `/accounts/{account_id}/cloudforce-one/events/categories/{category_id}`

Partially updates a threat event category in Cloudforce One, modifying specific fields without replacing the entire category.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_id: string`

  Category UUID.

### Body Parameters

- `killChain: optional number`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `name: optional string`

- `shortname: optional string`

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/$CATEGORY_ID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "killChain": 0,
  "name": "name",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "mitreAttack": [
    "T1234"
  ],
  "mitreCapec": [
    "123"
  ],
  "shortname": "shortname"
}
```

## Deletes a category

**delete** `/accounts/{account_id}/cloudforce-one/events/categories/{category_id}`

Removes a threat event category from Cloudforce One.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_id: string`

  Category UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/$CATEGORY_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Domain Types

### Category List Response

- `CategoryListResponse = array of object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Get Response

- `CategoryGetResponse object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Create Response

- `CategoryCreateResponse object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Edit Response

- `CategoryEditResponse object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

### Category Delete Response

- `CategoryDeleteResponse object { uuid }`

  - `uuid: string`

# Catalog

## Lists categories

**get** `/accounts/{account_id}/cloudforce-one/events/categories/catalog`

Lists categories

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `killChain: number`

- `name: string`

- `uuid: string`

- `mitreAttack: optional array of string`

- `mitreCapec: optional array of string`

- `shortname: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/categories/catalog \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "killChain": 0,
    "name": "name",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "mitreAttack": [
      "T1234"
    ],
    "mitreCapec": [
      "123"
    ],
    "shortname": "shortname"
  }
]
```

## Domain Types

### Catalog List Response

- `CatalogListResponse = array of object { killChain, name, uuid, 3 more }`

  - `killChain: number`

  - `name: string`

  - `uuid: string`

  - `mitreAttack: optional array of string`

  - `mitreCapec: optional array of string`

  - `shortname: optional string`

# Countries

## Retrieves countries information for all countries

**get** `/accounts/{account_id}/cloudforce-one/events/countries`

Lists countries referenced in Cloudforce One threat intelligence data.

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `result: array of object { alpha2, alpha3, name }`

  - `alpha2: string`

  - `alpha3: string`

  - `name: string`

- `success: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/countries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "result": [
      {
        "alpha2": "AF",
        "alpha3": "AF",
        "name": "Afghanistan"
      }
    ],
    "success": "true"
  }
]
```

## Domain Types

### Country List Response

- `CountryListResponse = array of object { result, success }`

  - `result: array of object { alpha2, alpha3, name }`

    - `alpha2: string`

    - `alpha3: string`

    - `name: string`

  - `success: string`

# Crons

# Datasets

## Lists all datasets in an account

**get** `/accounts/{account_id}/cloudforce-one/events/dataset`

Lists all threat event datasets configured in Cloudforce One.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `includeDeleted: optional boolean`

  When true, include soft-deleted datasets in the response. Each item includes a `deletedAt` field (ISO 8601 or null). Default: false.

### Returns

- `indicatorWriteMode: "read_only" or "create_only" or "full"`

  Effective indicator mutation capability after account/dataset authorization and dataset storage capability are applied. API Gateway method permissions are separate and must also allow the requested operation.

  - `"read_only"`

  - `"create_only"`

  - `"full"`

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

- `deletedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {
    "indicatorWriteMode": "full",
    "isAnalytics": true,
    "isPublic": true,
    "name": "friendly dataset name",
    "uuid": "12345678-1234-1234-1234-1234567890ab",
    "deletedAt": "deletedAt"
  }
]
```

## Reads a dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}`

Retrieves details for a specific threat event dataset.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

### Returns

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "isAnalytics": true,
  "isPublic": true,
  "name": "friendly dataset name",
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Creates a dataset

**post** `/accounts/{account_id}/cloudforce-one/events/dataset/create`

Creates a new threat event dataset in Cloudforce One for organizing related threat events.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `isPublic: boolean`

  If true, then anyone can search the dataset. If false, then its limited to the account.

- `name: string`

  Used to describe the dataset within the account context.

### Returns

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "isPublic": true,
          "name": "x"
        }'
```

#### Response

```json
{
  "isAnalytics": true,
  "isPublic": true,
  "name": "friendly dataset name",
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Updates an existing dataset

**patch** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}`

Partially updates a threat event dataset in Cloudforce One, modifying specific fields without replacing the entire dataset configuration.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

### Body Parameters

- `isPublic: boolean`

  If true, then anyone can search the dataset. If false, then its limited to the account.

- `name: string`

  Used to describe the dataset within the account context.

### Returns

- `isAnalytics: boolean`

- `isPublic: boolean`

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID \
    -X PATCH \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "isPublic": true,
          "name": "x"
        }'
```

#### Response

```json
{
  "isAnalytics": true,
  "isPublic": true,
  "name": "friendly dataset name",
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Delete a dataset

**delete** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}`

Soft-deletes a dataset given a datasetId.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID to delete

### Returns

- `name: string`

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "name": "friendly dataset name",
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Reads raw data for an event by UUID

**get** `/accounts/{account_id}/cloudforce-one/events/raw/{dataset_id}/{event_id}`

Retrieves the raw data associated with an event. Searches across all shards in the dataset.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

- `event_id: string`

  Event ID.

### Returns

- `id: number`

- `accountId: number`

- `created: string`

- `data: string`

- `source: string`

- `tlp: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/raw/$DATASET_ID/$EVENT_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": 1,
  "accountId": 1234,
  "created": "1970-01-01T00:00:00.000Z",
  "data": "{\"foo\": \"bar\"}",
  "source": "https://example.com",
  "tlp": "amber"
}
```

## Domain Types

### Dataset List Response

- `DatasetListResponse = array of object { indicatorWriteMode, isAnalytics, isPublic, 3 more }`

  - `indicatorWriteMode: "read_only" or "create_only" or "full"`

    Effective indicator mutation capability after account/dataset authorization and dataset storage capability are applied. API Gateway method permissions are separate and must also allow the requested operation.

    - `"read_only"`

    - `"create_only"`

    - `"full"`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

  - `deletedAt: optional string`

### Dataset Get Response

- `DatasetGetResponse object { isAnalytics, isPublic, name, uuid }`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

### Dataset Create Response

- `DatasetCreateResponse object { isAnalytics, isPublic, name, uuid }`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

### Dataset Edit Response

- `DatasetEditResponse object { isAnalytics, isPublic, name, uuid }`

  - `isAnalytics: boolean`

  - `isPublic: boolean`

  - `name: string`

  - `uuid: string`

### Dataset Delete Response

- `DatasetDeleteResponse object { name, uuid }`

  - `name: string`

  - `uuid: string`

### Dataset Raw Response

- `DatasetRawResponse object { id, accountId, created, 3 more }`

  - `id: number`

  - `accountId: number`

  - `created: string`

  - `data: string`

  - `source: string`

  - `tlp: string`

# Health

# Events

## Reads an event

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/events/{event_id}`

Retrieves a specific event by its UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

- `event_id: string`

  Event UUID.

### Returns

- `attacker: string`

- `attackerCountry: string`

- `attackerCountryAlpha3: string`

- `category: string`

- `datasetId: string`

- `date: string`

- `event: string`

- `hasChildren: boolean`

- `indicator: string`

- `indicatorType: string`

- `indicatorTypeId: number`

- `killChain: number`

- `mitreAttack: array of string`

- `mitreCapec: array of string`

- `numReferenced: number`

- `numReferences: number`

- `rawId: string`

- `referenced: array of string`

- `referencedIds: array of number`

- `references: array of string`

- `referencesIds: array of number`

- `tags: array of string`

- `targetCountry: string`

- `targetCountryAlpha3: string`

- `targetIndustry: string`

- `tlp: string`

- `uuid: string`

- `insight: optional string`

- `releasabilityId: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/events/$EVENT_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "attacker": "Flying Yeti",
  "attackerCountry": "CN",
  "attackerCountryAlpha3": "CHN",
  "category": "Domain Resolution",
  "datasetId": "dataset-example-id",
  "date": "2022-04-01T00:00:00Z",
  "event": "An attacker registered the domain domain.com",
  "hasChildren": true,
  "indicator": "domain.com",
  "indicatorType": "domain",
  "indicatorTypeId": 5,
  "killChain": 0,
  "mitreAttack": [
    " "
  ],
  "mitreCapec": [
    " "
  ],
  "numReferenced": 0,
  "numReferences": 0,
  "rawId": "453gw34w3",
  "referenced": [
    " "
  ],
  "referencedIds": [
    0
  ],
  "references": [
    " "
  ],
  "referencesIds": [
    0
  ],
  "tags": [
    "malware"
  ],
  "targetCountry": "US",
  "targetCountryAlpha3": "USA",
  "targetIndustry": "Agriculture",
  "tlp": "amber",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "insight": "insight",
  "releasabilityId": "releasabilityId"
}
```

## Domain Types

### Event Get Response

- `EventGetResponse object { attacker, attackerCountry, attackerCountryAlpha3, 26 more }`

  - `attacker: string`

  - `attackerCountry: string`

  - `attackerCountryAlpha3: string`

  - `category: string`

  - `datasetId: string`

  - `date: string`

  - `event: string`

  - `hasChildren: boolean`

  - `indicator: string`

  - `indicatorType: string`

  - `indicatorTypeId: number`

  - `killChain: number`

  - `mitreAttack: array of string`

  - `mitreCapec: array of string`

  - `numReferenced: number`

  - `numReferences: number`

  - `rawId: string`

  - `referenced: array of string`

  - `referencedIds: array of number`

  - `references: array of string`

  - `referencesIds: array of number`

  - `tags: array of string`

  - `targetCountry: string`

  - `targetCountryAlpha3: string`

  - `targetIndustry: string`

  - `tlp: string`

  - `uuid: string`

  - `insight: optional string`

  - `releasabilityId: optional string`

# Raw

## Reads data for a raw event

**get** `/accounts/{account_id}/cloudforce-one/events/{event_id}/raw/{raw_id}`

Retrieves raw threat event data for a specific event in Cloudforce One.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

- `raw_id: string`

  Raw Event UUID.

### Returns

- `id: string`

- `accountId: number`

- `created: string`

- `data: unknown`

- `source: string`

- `tlp: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/$EVENT_ID/raw/$RAW_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": "1234",
  "accountId": 1234,
  "created": "1970-01-01",
  "data": {},
  "source": "https://example.com",
  "tlp": "amber"
}
```

## Updates a raw event

**patch** `/accounts/{account_id}/cloudforce-one/events/{event_id}/raw/{raw_id}`

Partially updates raw threat event data in Cloudforce One, modifying specific fields of the event.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

- `raw_id: string`

  Raw Event UUID.

### Body Parameters

- `data: optional unknown`

- `source: optional string`

- `tlp: optional string`

### Returns

- `id: string`

- `data: unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/$EVENT_ID/raw/$RAW_ID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": "1234",
  "data": {}
}
```

## Domain Types

### Raw Get Response

- `RawGetResponse object { id, accountId, created, 3 more }`

  - `id: string`

  - `accountId: number`

  - `created: string`

  - `data: unknown`

  - `source: string`

  - `tlp: string`

### Raw Edit Response

- `RawEditResponse object { id, data }`

  - `id: string`

  - `data: unknown`

# Relate

## Removes an event reference

**delete** `/accounts/{account_id}/cloudforce-one/events/relate/{event_id}`

Removes a reference link between related threat events in Cloudforce One.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Returns

- `result: object { success }`

  - `success: boolean`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/relate/$EVENT_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "success": true
  },
  "success": true
}
```

## Domain Types

### Relate Delete Response

- `RelateDeleteResponse object { success }`

  - `success: boolean`

# Tags

## Lists all tags (SoT)

**get** `/accounts/{account_id}/cloudforce-one/events/tags`

Returns all Source-of-Truth tags for an account. Supports legacy free-text `search` on tag value and `categoryUuid` exact match, plus a structured `filters` JSON array for filtering by metadata fields (originCountryISO, actorCategory, motive, priority, etc.). Country values may be passed as alpha-2, alpha-3, name, or common alias.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cache: optional "from-graph"`

  Cache strategy. 'from-graph' serves results from the graph-node KV cache when all requested UUIDs are cached; falls back to normal path on partial/zero hit.

  - `"from-graph"`

- `categoryUuid: optional string`

- `filters: optional array of object { field, op, value }`

  Structured filters as a JSON array of {field, op, value} objects. Searchable fields: uuid, value, actorCategory, actorCategoryConfidence, aliasGroupNames, attributionConfidence, attributionConfidenceScore, attributionOrganization, categoryName, motive, motiveConfidence, opsecLevel, originCountryISO, originCountryConfidence, sophisticationLevel, priority, analyticPriority. Operators: equals, not, contains, startsWith, endsWith, gt, lt, gte, lte, like, in, find. Use 'in' for bulk OR within a single field, e.g. filters=[{"field":"originCountryISO","op":"in","value":["IR","CN"]}]. Multiple entries are AND-joined. Max 10 entries per request, max 100 values per 'in'. Per-field notes: `uuid` accepts only 'equals' and 'in' (other operators throw ValidationError) — matched against the canonical lowercase storage but callers may pass either case (the server lowercases before comparison); index-backed by the column's UNIQUE constraint and intended for batched UUID → tag resolution. `originCountryISO` uses its B-tree index for equals/not/in. `priority` uses its B-tree index for numeric comparisons. Other string columns (`actorCategory`, `motive`, etc.) are case-insensitive and unindexed; current catalog size makes this a non-issue. `endsWith` and `aliasGroupNames` contains/like are leading-wildcard scans and slow on large result sets. `aliasGroupNames` matches on the JSON-encoded text, so substrings can cross alias boundaries (a search for "apt28" will also match "apt280" if both appear in the same tag's alias list).

  - `field: "uuid" or "value" or "actorCategory" or 14 more`

    Tag field to search on. Allowed: uuid, value, actorCategory, actorCategoryConfidence, aliasGroupNames, attributionConfidence, attributionConfidenceScore, attributionOrganization, categoryName, motive, motiveConfidence, opsecLevel, originCountryISO, originCountryConfidence, sophisticationLevel, priority, analyticPriority.

    - `"uuid"`

    - `"value"`

    - `"actorCategory"`

    - `"actorCategoryConfidence"`

    - `"aliasGroupNames"`

    - `"attributionConfidence"`

    - `"attributionConfidenceScore"`

    - `"attributionOrganization"`

    - `"categoryName"`

    - `"motive"`

    - `"motiveConfidence"`

    - `"opsecLevel"`

    - `"originCountryISO"`

    - `"originCountryConfidence"`

    - `"sophisticationLevel"`

    - `"priority"`

    - `"analyticPriority"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk OR within a single field, e.g. {field:"originCountryISO", op:"in", value:["IR","CN"]}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: optional string or number or array of string or number`

    Search value. String or number for most operators. Array for 'in' (max 100 items). Country values may be passed as alpha-2, alpha-3, name, or common alias (e.g. 'iran', 'IR', 'IRN') and are normalized to alpha-2 server-side.

    - `string`

    - `number`

    - `array of string or number`

      - `string`

      - `number`

- `page: optional number`

- `pageSize: optional number`

- `search: optional string`

  Legacy free-text substring match on tag value.

### Returns

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

- `tags: array of object { uuid, value, activeDuration, 25 more }`

  - `uuid: string`

  - `value: string`

  - `activeDuration: optional string`

  - `actorCategory: optional string`

  - `actorCategoryConfidence: optional number`

    Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

  - `aliases: optional array of object { value, confidence, tlp }`

    Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `aliasGroupNames: optional array of string`

  - `aliasGroupNamesInternal: optional array of string`

  - `analyticPriority: optional number`

  - `attributionConfidence: optional string`

  - `attributionConfidenceScore: optional number`

  - `attributionOrganization: optional string`

  - `categoryName: optional string`

  - `categoryUuid: optional string`

  - `dateOfDiscovery: optional string`

  - `externalReferenceLinks: optional array of string`

  - `externalReferences: optional array of object { url, description }`

    Structured external references ({ url, description }). Public: returned to all accounts.

    - `url: string`

    - `description: optional string`

  - `internalAliases: optional array of object { value, confidence, tlp }`

    Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `internalDescription: optional string`

  - `motive: optional string`

  - `motiveConfidence: optional number`

    Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

  - `opsecLevel: optional string`

  - `originCountryConfidence: optional number`

    Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `originCountryISO: optional string`

  - `originCountryISOAlpha3: optional string`

  - `originCountryTlp: optional "red" or "amber" or "green" or "white"`

    TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

  - `priority: optional number`

  - `sophisticationLevel: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  },
  "tags": [
    {
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "APT28",
      "activeDuration": "activeDuration",
      "actorCategory": "actorCategory",
      "actorCategoryConfidence": 7,
      "aliases": [
        {
          "value": "Fancy Bear",
          "confidence": 8,
          "tlp": "amber"
        }
      ],
      "aliasGroupNames": [
        "string"
      ],
      "aliasGroupNamesInternal": [
        "string"
      ],
      "analyticPriority": 0,
      "attributionConfidence": "attributionConfidence",
      "attributionConfidenceScore": 7,
      "attributionOrganization": "attributionOrganization",
      "categoryName": "Nation State",
      "categoryUuid": "12345678-1234-1234-1234-1234567890ab",
      "dateOfDiscovery": "2024-01-15",
      "externalReferenceLinks": [
        "string"
      ],
      "externalReferences": [
        {
          "url": "https://example.com/report",
          "description": "Vendor threat report"
        }
      ],
      "internalAliases": [
        {
          "value": "Fancy Bear",
          "confidence": 8,
          "tlp": "amber"
        }
      ],
      "internalDescription": "internalDescription",
      "motive": "motive",
      "motiveConfidence": 7,
      "opsecLevel": "opsecLevel",
      "originCountryConfidence": 7,
      "originCountryISO": "originCountryISO",
      "originCountryISOAlpha3": "IRN",
      "originCountryTlp": "amber",
      "priority": 0,
      "sophisticationLevel": "sophisticationLevel"
    }
  ]
}
```

## Creates a new tag

**post** `/accounts/{account_id}/cloudforce-one/events/tags/create`

Creates a new tag to be used accross threat events.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `value: string`

- `activeDuration: optional string`

- `actorCategory: optional string`

  Actor variety. Allowed values: Activist, Competitor, Customer, Crime Syndicate, Former Employee, Nation State, Organized Crime, Nation State Affiliated, Terrorist, Unaffiliated.

- `actorCategoryConfidence: optional number`

  Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

- `aliases: optional array of object { value, confidence, tlp }`

  Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `aliasGroupNames: optional array of string`

- `aliasGroupNamesInternal: optional array of string`

- `analyticPriority: optional number`

- `attributionConfidence: optional string`

- `attributionConfidenceScore: optional number`

- `attributionOrganization: optional string`

- `categoryUuid: optional string`

- `dateOfDiscovery: optional string`

  Date the actor was discovered (ISO YYYY-MM-DD).

- `externalReferenceLinks: optional array of string`

- `externalReferences: optional array of object { url, description }`

  Structured external references ({ url, description }). Public: returned to all accounts.

  - `url: string`

  - `description: optional string`

- `internalAliases: optional array of object { value, confidence, tlp }`

  Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `internalDescription: optional string`

- `motive: optional string`

  Actor motive. Allowed values: Convenience, Fear, Fun, Financial, Grudge, Ideology, Espionage.

- `motiveConfidence: optional number`

  Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

- `opsecLevel: optional string`

- `originCountryConfidence: optional number`

  Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

- `originCountryISO: optional string`

- `originCountryTlp: optional "red" or "amber" or "green" or "white"`

  TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `"red"`

  - `"amber"`

  - `"green"`

  - `"white"`

- `priority: optional number`

- `sophisticationLevel: optional string`

### Returns

- `uuid: string`

- `value: string`

- `activeDuration: optional string`

- `actorCategory: optional string`

- `actorCategoryConfidence: optional number`

  Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

- `aliases: optional array of object { value, confidence, tlp }`

  Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `aliasGroupNames: optional array of string`

- `aliasGroupNamesInternal: optional array of string`

- `analyticPriority: optional number`

- `attributionConfidence: optional string`

- `attributionConfidenceScore: optional number`

- `attributionOrganization: optional string`

- `categoryName: optional string`

- `categoryUuid: optional string`

- `dateOfDiscovery: optional string`

- `externalReferenceLinks: optional array of string`

- `externalReferences: optional array of object { url, description }`

  Structured external references ({ url, description }). Public: returned to all accounts.

  - `url: string`

  - `description: optional string`

- `internalAliases: optional array of object { value, confidence, tlp }`

  Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `internalDescription: optional string`

- `motive: optional string`

- `motiveConfidence: optional number`

  Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

- `opsecLevel: optional string`

- `originCountryConfidence: optional number`

  Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

- `originCountryISO: optional string`

- `originCountryISOAlpha3: optional string`

- `originCountryTlp: optional "red" or "amber" or "green" or "white"`

  TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `"red"`

  - `"amber"`

  - `"green"`

  - `"white"`

- `priority: optional number`

- `sophisticationLevel: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "value": "APT28",
          "actorCategory": "Nation State",
          "actorCategoryConfidence": 7,
          "attributionConfidenceScore": 7,
          "categoryUuid": "12345678-1234-1234-1234-1234567890ab",
          "dateOfDiscovery": "2024-01-15",
          "motive": "Espionage",
          "motiveConfidence": 7,
          "originCountryConfidence": 7,
          "originCountryTlp": "amber"
        }'
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "value": "APT28",
  "activeDuration": "activeDuration",
  "actorCategory": "actorCategory",
  "actorCategoryConfidence": 7,
  "aliases": [
    {
      "value": "Fancy Bear",
      "confidence": 8,
      "tlp": "amber"
    }
  ],
  "aliasGroupNames": [
    "string"
  ],
  "aliasGroupNamesInternal": [
    "string"
  ],
  "analyticPriority": 0,
  "attributionConfidence": "attributionConfidence",
  "attributionConfidenceScore": 7,
  "attributionOrganization": "attributionOrganization",
  "categoryName": "Nation State",
  "categoryUuid": "12345678-1234-1234-1234-1234567890ab",
  "dateOfDiscovery": "2024-01-15",
  "externalReferenceLinks": [
    "string"
  ],
  "externalReferences": [
    {
      "url": "https://example.com/report",
      "description": "Vendor threat report"
    }
  ],
  "internalAliases": [
    {
      "value": "Fancy Bear",
      "confidence": 8,
      "tlp": "amber"
    }
  ],
  "internalDescription": "internalDescription",
  "motive": "motive",
  "motiveConfidence": 7,
  "opsecLevel": "opsecLevel",
  "originCountryConfidence": 7,
  "originCountryISO": "originCountryISO",
  "originCountryISOAlpha3": "IRN",
  "originCountryTlp": "amber",
  "priority": 0,
  "sophisticationLevel": "sophisticationLevel"
}
```

## Updates a tag (SoT)

**patch** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}`

Updates a Source-of-Truth tag by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Body Parameters

- `activeDuration: optional string`

- `actorCategory: optional string`

  Actor variety. Allowed values: Activist, Competitor, Customer, Crime Syndicate, Former Employee, Nation State, Organized Crime, Nation State Affiliated, Terrorist, Unaffiliated.

- `actorCategoryConfidence: optional number`

  Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

- `aliases: optional array of object { value, confidence, tlp }`

  Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `aliasGroupNames: optional array of string`

- `aliasGroupNamesInternal: optional array of string`

- `analyticPriority: optional number`

- `attributionConfidence: optional string`

- `attributionConfidenceScore: optional number`

- `attributionOrganization: optional string`

- `categoryUuid: optional string`

- `dateOfDiscovery: optional string`

  Date the actor was discovered (ISO YYYY-MM-DD).

- `externalReferenceLinks: optional array of string`

- `externalReferences: optional array of object { url, description }`

  Structured external references ({ url, description }). Public: returned to all accounts.

  - `url: string`

  - `description: optional string`

- `internalAliases: optional array of object { value, confidence, tlp }`

  Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `internalDescription: optional string`

- `motive: optional string`

  Actor motive. Allowed values: Convenience, Fear, Fun, Financial, Grudge, Ideology, Espionage.

- `motiveConfidence: optional number`

  Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

- `opsecLevel: optional string`

- `originCountryConfidence: optional number`

  Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

- `originCountryISO: optional string`

- `originCountryTlp: optional "red" or "amber" or "green" or "white"`

  TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `"red"`

  - `"amber"`

  - `"green"`

  - `"white"`

- `priority: optional number`

- `sophisticationLevel: optional string`

- `value: optional string`

### Returns

- `uuid: string`

- `value: string`

- `activeDuration: optional string`

- `actorCategory: optional string`

- `actorCategoryConfidence: optional number`

  Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

- `aliases: optional array of object { value, confidence, tlp }`

  Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `aliasGroupNames: optional array of string`

- `aliasGroupNamesInternal: optional array of string`

- `analyticPriority: optional number`

- `attributionConfidence: optional string`

- `attributionConfidenceScore: optional number`

- `attributionOrganization: optional string`

- `categoryName: optional string`

- `categoryUuid: optional string`

- `dateOfDiscovery: optional string`

- `externalReferenceLinks: optional array of string`

- `externalReferences: optional array of object { url, description }`

  Structured external references ({ url, description }). Public: returned to all accounts.

  - `url: string`

  - `description: optional string`

- `internalAliases: optional array of object { value, confidence, tlp }`

  Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

  - `value: string`

  - `confidence: optional number`

  - `tlp: optional "red" or "amber" or "green" or "white"`

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

- `internalDescription: optional string`

- `motive: optional string`

- `motiveConfidence: optional number`

  Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

- `opsecLevel: optional string`

- `originCountryConfidence: optional number`

  Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

- `originCountryISO: optional string`

- `originCountryISOAlpha3: optional string`

- `originCountryTlp: optional "red" or "amber" or "green" or "white"`

  TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `"red"`

  - `"amber"`

  - `"green"`

  - `"white"`

- `priority: optional number`

- `sophisticationLevel: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "value": "APT28",
  "activeDuration": "activeDuration",
  "actorCategory": "actorCategory",
  "actorCategoryConfidence": 7,
  "aliases": [
    {
      "value": "Fancy Bear",
      "confidence": 8,
      "tlp": "amber"
    }
  ],
  "aliasGroupNames": [
    "string"
  ],
  "aliasGroupNamesInternal": [
    "string"
  ],
  "analyticPriority": 0,
  "attributionConfidence": "attributionConfidence",
  "attributionConfidenceScore": 7,
  "attributionOrganization": "attributionOrganization",
  "categoryName": "Nation State",
  "categoryUuid": "12345678-1234-1234-1234-1234567890ab",
  "dateOfDiscovery": "2024-01-15",
  "externalReferenceLinks": [
    "string"
  ],
  "externalReferences": [
    {
      "url": "https://example.com/report",
      "description": "Vendor threat report"
    }
  ],
  "internalAliases": [
    {
      "value": "Fancy Bear",
      "confidence": 8,
      "tlp": "amber"
    }
  ],
  "internalDescription": "internalDescription",
  "motive": "motive",
  "motiveConfidence": 7,
  "opsecLevel": "opsecLevel",
  "originCountryConfidence": 7,
  "originCountryISO": "originCountryISO",
  "originCountryISOAlpha3": "IRN",
  "originCountryTlp": "amber",
  "priority": 0,
  "sophisticationLevel": "sophisticationLevel"
}
```

## Deletes a tag (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}`

Deletes a Source-of-Truth tag by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Domain Types

### Tag List Response

- `TagListResponse object { pagination, tags }`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`

  - `tags: array of object { uuid, value, activeDuration, 25 more }`

    - `uuid: string`

    - `value: string`

    - `activeDuration: optional string`

    - `actorCategory: optional string`

    - `actorCategoryConfidence: optional number`

      Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

    - `aliases: optional array of object { value, confidence, tlp }`

      Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

      - `value: string`

      - `confidence: optional number`

      - `tlp: optional "red" or "amber" or "green" or "white"`

        - `"red"`

        - `"amber"`

        - `"green"`

        - `"white"`

    - `aliasGroupNames: optional array of string`

    - `aliasGroupNamesInternal: optional array of string`

    - `analyticPriority: optional number`

    - `attributionConfidence: optional string`

    - `attributionConfidenceScore: optional number`

    - `attributionOrganization: optional string`

    - `categoryName: optional string`

    - `categoryUuid: optional string`

    - `dateOfDiscovery: optional string`

    - `externalReferenceLinks: optional array of string`

    - `externalReferences: optional array of object { url, description }`

      Structured external references ({ url, description }). Public: returned to all accounts.

      - `url: string`

      - `description: optional string`

    - `internalAliases: optional array of object { value, confidence, tlp }`

      Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

      - `value: string`

      - `confidence: optional number`

      - `tlp: optional "red" or "amber" or "green" or "white"`

        - `"red"`

        - `"amber"`

        - `"green"`

        - `"white"`

    - `internalDescription: optional string`

    - `motive: optional string`

    - `motiveConfidence: optional number`

      Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

    - `opsecLevel: optional string`

    - `originCountryConfidence: optional number`

      Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

    - `originCountryISO: optional string`

    - `originCountryISOAlpha3: optional string`

    - `originCountryTlp: optional "red" or "amber" or "green" or "white"`

      TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

    - `priority: optional number`

    - `sophisticationLevel: optional string`

### Tag Create Response

- `TagCreateResponse object { uuid, value, activeDuration, 25 more }`

  - `uuid: string`

  - `value: string`

  - `activeDuration: optional string`

  - `actorCategory: optional string`

  - `actorCategoryConfidence: optional number`

    Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

  - `aliases: optional array of object { value, confidence, tlp }`

    Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `aliasGroupNames: optional array of string`

  - `aliasGroupNamesInternal: optional array of string`

  - `analyticPriority: optional number`

  - `attributionConfidence: optional string`

  - `attributionConfidenceScore: optional number`

  - `attributionOrganization: optional string`

  - `categoryName: optional string`

  - `categoryUuid: optional string`

  - `dateOfDiscovery: optional string`

  - `externalReferenceLinks: optional array of string`

  - `externalReferences: optional array of object { url, description }`

    Structured external references ({ url, description }). Public: returned to all accounts.

    - `url: string`

    - `description: optional string`

  - `internalAliases: optional array of object { value, confidence, tlp }`

    Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `internalDescription: optional string`

  - `motive: optional string`

  - `motiveConfidence: optional number`

    Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

  - `opsecLevel: optional string`

  - `originCountryConfidence: optional number`

    Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `originCountryISO: optional string`

  - `originCountryISOAlpha3: optional string`

  - `originCountryTlp: optional "red" or "amber" or "green" or "white"`

    TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

  - `priority: optional number`

  - `sophisticationLevel: optional string`

### Tag Edit Response

- `TagEditResponse object { uuid, value, activeDuration, 25 more }`

  - `uuid: string`

  - `value: string`

  - `activeDuration: optional string`

  - `actorCategory: optional string`

  - `actorCategoryConfidence: optional number`

    Confidence (1-10) in the actor variety (actorCategory). CFONE-only: stripped from responses to non-CFONE accounts.

  - `aliases: optional array of object { value, confidence, tlp }`

    Structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: stripped from responses to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `aliasGroupNames: optional array of string`

  - `aliasGroupNamesInternal: optional array of string`

  - `analyticPriority: optional number`

  - `attributionConfidence: optional string`

  - `attributionConfidenceScore: optional number`

  - `attributionOrganization: optional string`

  - `categoryName: optional string`

  - `categoryUuid: optional string`

  - `dateOfDiscovery: optional string`

  - `externalReferenceLinks: optional array of string`

  - `externalReferences: optional array of object { url, description }`

    Structured external references ({ url, description }). Public: returned to all accounts.

    - `url: string`

    - `description: optional string`

  - `internalAliases: optional array of object { value, confidence, tlp }`

    Internal structured aliases ({ value, confidence 1-10, tlp }). CFONE-only: never returned to non-CFONE accounts.

    - `value: string`

    - `confidence: optional number`

    - `tlp: optional "red" or "amber" or "green" or "white"`

      - `"red"`

      - `"amber"`

      - `"green"`

      - `"white"`

  - `internalDescription: optional string`

  - `motive: optional string`

  - `motiveConfidence: optional number`

    Confidence (1-10) in the actor motive. CFONE-only: stripped from responses to non-CFONE accounts.

  - `opsecLevel: optional string`

  - `originCountryConfidence: optional number`

    Confidence (1-10) in the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

  - `originCountryISO: optional string`

  - `originCountryISOAlpha3: optional string`

  - `originCountryTlp: optional "red" or "amber" or "green" or "white"`

    TLP marking for the origin-country attribution. CFONE-only: stripped from responses to non-CFONE accounts.

    - `"red"`

    - `"amber"`

    - `"green"`

    - `"white"`

  - `priority: optional number`

  - `sophisticationLevel: optional string`

### Tag Delete Response

- `TagDeleteResponse object { uuid }`

  - `uuid: string`

# Categories

## Lists all tag categories (SoT)

**get** `/accounts/{account_id}/cloudforce-one/events/tags/categories`

Returns all Source-of-Truth tag categories for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `search: optional string`

### Returns

- `categories: array of object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "categories": [
    {
      "name": "Actor",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "createdAt": "createdAt",
      "description": "description",
      "updatedAt": "updatedAt"
    }
  ]
}
```

## Creates a new tag category (SoT)

**post** `/accounts/{account_id}/cloudforce-one/events/tags/categories/create`

Creates a new Source-of-Truth tag category for an account.

### Path Parameters

- `account_id: string`

  Account ID.

### Body Parameters

- `name: string`

- `description: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "name": "Actor"
        }'
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```

## Updates a tag category (SoT)

**patch** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Updates a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Body Parameters

- `description: optional string`

- `name: optional string`

### Returns

- `name: string`

- `uuid: string`

- `createdAt: optional string`

- `description: optional string`

- `updatedAt: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X PATCH \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "name": "Actor",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "createdAt": "createdAt",
  "description": "description",
  "updatedAt": "updatedAt"
}
```

## Deletes a tag category (SoT)

**delete** `/accounts/{account_id}/cloudforce-one/events/tags/categories/{category_uuid}`

Deletes a Source-of-Truth tag category by UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `category_uuid: string`

  Tag Category UUID.

### Returns

- `uuid: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/categories/$CATEGORY_UUID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "uuid": "12345678-1234-1234-1234-1234567890ab"
}
```

## Domain Types

### Category List Response

- `CategoryListResponse object { categories }`

  - `categories: array of object { name, uuid, createdAt, 2 more }`

    - `name: string`

    - `uuid: string`

    - `createdAt: optional string`

    - `description: optional string`

    - `updatedAt: optional string`

### Category Create Response

- `CategoryCreateResponse object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Category Edit Response

- `CategoryEditResponse object { name, uuid, createdAt, 2 more }`

  - `name: string`

  - `uuid: string`

  - `createdAt: optional string`

  - `description: optional string`

  - `updatedAt: optional string`

### Category Delete Response

- `CategoryDeleteResponse object { uuid }`

  - `uuid: string`

# Indicators

## List indicators related to a tag

**get** `/accounts/{account_id}/cloudforce-one/events/tags/{tag_uuid}/indicators`

Returns indicators associated with the provided tag UUID, with pagination. By default fans out across every indicator dataset the account can read; pass datasetIds to scope to specific datasets.

### Path Parameters

- `account_id: string`

  Account ID.

- `tag_uuid: string`

  Tag UUID.

### Query Parameters

- `datasetIds: optional array of string`

  Dataset UUIDs to scope to (repeat the param for multiple), or 'all' / '*' for every readable indicator dataset. Omit to search all readable datasets.

- `indicatorType: optional string`

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType. Multiple conditions are AND'd together. Max 10 conditions per request.

  - `field: "value" or "indicatorType" or "uuid"`

    The indicator field to search on. Allowed: value, indicatorType, uuid.

    - `"value"`

    - `"indicatorType"`

    - `"uuid"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'value', op:'in', value:['evil.com','bad.org']}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: string or array of string`

    Search value. String for most operators. Array of strings for 'in' operator (max 100 items).

    - `string`

    - `array of string`

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

    - `datasetId: string`

    - `eventId: string`

    - `eventDate: optional string`

      ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/tags/$TAG_UUID/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "indicators": [
    {
      "createdAt": "2022-04-01T00:00:00Z",
      "indicatorType": "domain",
      "updatedAt": "2022-04-01T00:00:00Z",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "malicious-domain.com",
      "datasetId": "dataset-uuid-123",
      "relatedEvents": [
        {
          "datasetId": "dataset-uuid-123",
          "eventId": "event-uuid-456",
          "eventDate": "2024-06-15T00:00:00Z"
        }
      ],
      "tags": [
        {
          "categoryName": "categoryName",
          "uuid": "uuid",
          "value": "value"
        }
      ]
    }
  ],
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  }
}
```

## Domain Types

### Indicator List Response

- `IndicatorListResponse object { indicators, pagination }`

  - `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

    - `createdAt: string`

    - `indicatorType: string`

    - `updatedAt: string`

    - `uuid: string`

    - `value: string`

    - `datasetId: optional string`

      The dataset ID this indicator belongs to. Included in list responses.

    - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

      - `datasetId: string`

      - `eventId: string`

      - `eventDate: optional string`

        ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`

# By Dataset

## List indicators related to a tag within a dataset (deprecated)

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/tags/{tag_uuid}/indicators`

This endpoint is deprecated. Use GET /:account_id/events/tags/:tag_uuid/indicators with the optional datasetIds query parameter instead. Returns indicators associated with the provided tag UUID within a single dataset's indicator shards, with pagination.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

- `tag_uuid: string`

  Tag UUID.

### Query Parameters

- `indicatorType: optional string`

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType. Multiple conditions are AND'd together. Max 10 conditions per request.

  - `field: "value" or "indicatorType" or "uuid"`

    The indicator field to search on. Allowed: value, indicatorType, uuid.

    - `"value"`

    - `"indicatorType"`

    - `"uuid"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'value', op:'in', value:['evil.com','bad.org']}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: string or array of string`

    Search value. String for most operators. Array of strings for 'in' operator (max 100 items).

    - `string`

    - `array of string`

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

    - `datasetId: string`

    - `eventId: string`

    - `eventDate: optional string`

      ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/tags/$TAG_UUID/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "indicators": [
    {
      "createdAt": "2022-04-01T00:00:00Z",
      "indicatorType": "domain",
      "updatedAt": "2022-04-01T00:00:00Z",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "malicious-domain.com",
      "datasetId": "dataset-uuid-123",
      "relatedEvents": [
        {
          "datasetId": "dataset-uuid-123",
          "eventId": "event-uuid-456",
          "eventDate": "2024-06-15T00:00:00Z"
        }
      ],
      "tags": [
        {
          "categoryName": "categoryName",
          "uuid": "uuid",
          "value": "value"
        }
      ]
    }
  ],
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  }
}
```

## Domain Types

### By Dataset List Response

- `ByDatasetListResponse object { indicators, pagination }`

  - `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

    - `createdAt: string`

    - `indicatorType: string`

    - `updatedAt: string`

    - `uuid: string`

    - `value: string`

    - `datasetId: optional string`

      The dataset ID this indicator belongs to. Included in list responses.

    - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

      - `datasetId: string`

      - `eventId: string`

      - `eventDate: optional string`

        ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`

# Event Tags

## Adds a tag to an event

**post** `/accounts/{account_id}/cloudforce-one/events/event_tag/{event_id}/create`

Adds a tag to a threat event in Cloudforce One for classification and filtering.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Body Parameters

- `tags: array of string`

### Returns

- `result: object { success }`

  - `success: boolean`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/event_tag/$EVENT_ID/create \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "tags": [
            "botnet"
          ]
        }'
```

#### Response

```json
{
  "result": {
    "success": true
  },
  "success": true
}
```

## Removes a tag from an event

**delete** `/accounts/{account_id}/cloudforce-one/events/event_tag/{event_id}`

Removes a tag from a threat event in Cloudforce One.

### Path Parameters

- `account_id: string`

  Account ID.

- `event_id: string`

  Event UUID.

### Returns

- `result: object { success }`

  - `success: boolean`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/event_tag/$EVENT_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "result": {
    "success": true
  },
  "success": true
}
```

## Domain Types

### Event Tag Create Response

- `EventTagCreateResponse object { success }`

  - `success: boolean`

### Event Tag Delete Response

- `EventTagDeleteResponse object { success }`

  - `success: boolean`

# Target Industries

## Lists target industries across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/targetIndustries`

Retrieves the catalog of industry classifications used in Cloudforce One threat intelligence.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query target industries from. If not provided, uses the default dataset.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/targetIndustries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### Target Industry List Response

- `TargetIndustryListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

# By Dataset

## Lists all target industries for a specific dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/targetIndustries`

Lists all target industries for a specific dataset

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/targetIndustries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### By Dataset List Response

- `ByDatasetListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

# Catalog

## Lists all target industries from industry map catalog

**get** `/accounts/{account_id}/cloudforce-one/events/targetIndustries/catalog`

Lists all target industries from industry map catalog

### Path Parameters

- `account_id: string`

  Account ID.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/targetIndustries/catalog \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### Catalog List Response

- `CatalogListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

# Insights
