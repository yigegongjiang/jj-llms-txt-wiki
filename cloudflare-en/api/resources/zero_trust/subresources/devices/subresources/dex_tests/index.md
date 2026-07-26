# DEX Tests

## List Device DEX tests

**get** `/accounts/{account_id}/dex/devices/dex_tests`

Fetch all DEX tests.

### Path Parameters

- `account_id: string`

  Unique identifier linked to an account.

### Query Parameters

- `kind: optional "http" or "traceroute"`

  Filter by test type.

  - `"http"`

  - `"traceroute"`

- `page: optional number`

  Page number of paginated results.

- `per_page: optional number`

  Number of results per page.

- `testName: optional string`

  Filter by test name.

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

- `result: optional array of SchemaHTTP`

  - `data: SchemaData`

    The configuration object which contains the details for the WARP client to conduct the test.

    - `host: string`

      The desired endpoint to test.

    - `kind: "http" or "traceroute"`

      The type of test.

      - `"http"`

      - `"traceroute"`

    - `method: optional "GET"`

      The HTTP request method type.

      - `"GET"`

  - `enabled: boolean`

    Determines whether or not the test is active.

  - `interval: string`

    How often the test will run.

  - `name: string`

    The name of the DEX test. Must be unique.

  - `created: optional string`

    Date the test was created, in RFC 3339 format.

  - `description: optional string`

    Additional details about the test.

  - `target_policies: optional array of object { id, default, name }`

    DEX rules targeted by this test

    - `id: string`

      The id of the DEX rule.

    - `default: optional boolean`

      Whether the DEX rule is the account default.

    - `name: optional string`

      The name of the DEX rule.

  - `targeted: optional boolean`

  - `test_id: optional string`

    The unique identifier for the test.

  - `updated: optional string`

    Date the test was last updated, in RFC 3339 format.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dex/devices/dex_tests \
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
      "data": {
        "host": "https://dash.cloudflare.com",
        "kind": "http",
        "method": "GET"
      },
      "enabled": true,
      "interval": "30m",
      "name": "HTTP dash health check",
      "created": "2023-10-11T00:00:00Z",
      "description": "Checks the dash endpoint every 30 minutes",
      "target_policies": [
        {
          "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
          "default": true,
          "name": "name"
        }
      ],
      "targeted": true,
      "test_id": "372e67954025e0ba6aaa6d586b9e0b59",
      "updated": "2023-10-11T00:00:00Z"
    }
  ]
}
```

## Get Device DEX test

**get** `/accounts/{account_id}/dex/devices/dex_tests/{dex_test_id}`

Fetch a single DEX test.

### Path Parameters

- `account_id: string`

  Unique identifier linked to an account.

- `dex_test_id: string`

  The unique identifier for the test.

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

- `result: optional SchemaHTTP`

  - `data: SchemaData`

    The configuration object which contains the details for the WARP client to conduct the test.

    - `host: string`

      The desired endpoint to test.

    - `kind: "http" or "traceroute"`

      The type of test.

      - `"http"`

      - `"traceroute"`

    - `method: optional "GET"`

      The HTTP request method type.

      - `"GET"`

  - `enabled: boolean`

    Determines whether or not the test is active.

  - `interval: string`

    How often the test will run.

  - `name: string`

    The name of the DEX test. Must be unique.

  - `created: optional string`

    Date the test was created, in RFC 3339 format.

  - `description: optional string`

    Additional details about the test.

  - `target_policies: optional array of object { id, default, name }`

    DEX rules targeted by this test

    - `id: string`

      The id of the DEX rule.

    - `default: optional boolean`

      Whether the DEX rule is the account default.

    - `name: optional string`

      The name of the DEX rule.

  - `targeted: optional boolean`

  - `test_id: optional string`

    The unique identifier for the test.

  - `updated: optional string`

    Date the test was last updated, in RFC 3339 format.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dex/devices/dex_tests/$DEX_TEST_ID \
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
    "data": {
      "host": "https://dash.cloudflare.com",
      "kind": "http",
      "method": "GET"
    },
    "enabled": true,
    "interval": "30m",
    "name": "HTTP dash health check",
    "created": "2023-10-11T00:00:00Z",
    "description": "Checks the dash endpoint every 30 minutes",
    "target_policies": [
      {
        "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
        "default": true,
        "name": "name"
      }
    ],
    "targeted": true,
    "test_id": "372e67954025e0ba6aaa6d586b9e0b59",
    "updated": "2023-10-11T00:00:00Z"
  }
}
```

## Create Device DEX test

**post** `/accounts/{account_id}/dex/devices/dex_tests`

Create a DEX test.

### Path Parameters

- `account_id: string`

  Unique identifier linked to an account.

### Body Parameters

- `data: SchemaData`

  The configuration object which contains the details for the WARP client to conduct the test.

  - `host: string`

    The desired endpoint to test.

  - `kind: "http" or "traceroute"`

    The type of test.

    - `"http"`

    - `"traceroute"`

  - `method: optional "GET"`

    The HTTP request method type.

    - `"GET"`

- `enabled: boolean`

  Determines whether or not the test is active.

- `interval: string`

  How often the test will run.

- `name: string`

  The name of the DEX test. Must be unique.

- `description: optional string`

  Additional details about the test.

- `target_policies: optional array of object { id, default, name }`

  DEX rules targeted by this test

  - `id: string`

    The id of the DEX rule.

  - `default: optional boolean`

    Whether the DEX rule is the account default.

  - `name: optional string`

    The name of the DEX rule.

- `targeted: optional boolean`

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

- `result: optional SchemaHTTP`

  - `data: SchemaData`

    The configuration object which contains the details for the WARP client to conduct the test.

    - `host: string`

      The desired endpoint to test.

    - `kind: "http" or "traceroute"`

      The type of test.

      - `"http"`

      - `"traceroute"`

    - `method: optional "GET"`

      The HTTP request method type.

      - `"GET"`

  - `enabled: boolean`

    Determines whether or not the test is active.

  - `interval: string`

    How often the test will run.

  - `name: string`

    The name of the DEX test. Must be unique.

  - `created: optional string`

    Date the test was created, in RFC 3339 format.

  - `description: optional string`

    Additional details about the test.

  - `target_policies: optional array of object { id, default, name }`

    DEX rules targeted by this test

    - `id: string`

      The id of the DEX rule.

    - `default: optional boolean`

      Whether the DEX rule is the account default.

    - `name: optional string`

      The name of the DEX rule.

  - `targeted: optional boolean`

  - `test_id: optional string`

    The unique identifier for the test.

  - `updated: optional string`

    Date the test was last updated, in RFC 3339 format.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dex/devices/dex_tests \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "data": {
            "host": "https://dash.cloudflare.com",
            "kind": "http",
            "method": "GET"
          },
          "enabled": true,
          "interval": "30m",
          "name": "HTTP dash health check",
          "description": "Checks the dash endpoint every 30 minutes"
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
    "data": {
      "host": "https://dash.cloudflare.com",
      "kind": "http",
      "method": "GET"
    },
    "enabled": true,
    "interval": "30m",
    "name": "HTTP dash health check",
    "created": "2023-10-11T00:00:00Z",
    "description": "Checks the dash endpoint every 30 minutes",
    "target_policies": [
      {
        "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
        "default": true,
        "name": "name"
      }
    ],
    "targeted": true,
    "test_id": "372e67954025e0ba6aaa6d586b9e0b59",
    "updated": "2023-10-11T00:00:00Z"
  }
}
```

## Update Device DEX test

**put** `/accounts/{account_id}/dex/devices/dex_tests/{dex_test_id}`

Update a DEX test.

### Path Parameters

- `account_id: string`

  Unique identifier linked to an account.

- `dex_test_id: string`

  API Resource UUID tag.

### Body Parameters

- `data: SchemaData`

  The configuration object which contains the details for the WARP client to conduct the test.

  - `host: string`

    The desired endpoint to test.

  - `kind: "http" or "traceroute"`

    The type of test.

    - `"http"`

    - `"traceroute"`

  - `method: optional "GET"`

    The HTTP request method type.

    - `"GET"`

- `enabled: boolean`

  Determines whether or not the test is active.

- `interval: string`

  How often the test will run.

- `name: string`

  The name of the DEX test. Must be unique.

- `description: optional string`

  Additional details about the test.

- `target_policies: optional array of object { id, default, name }`

  DEX rules targeted by this test

  - `id: string`

    The id of the DEX rule.

  - `default: optional boolean`

    Whether the DEX rule is the account default.

  - `name: optional string`

    The name of the DEX rule.

- `targeted: optional boolean`

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

- `result: optional SchemaHTTP`

  - `data: SchemaData`

    The configuration object which contains the details for the WARP client to conduct the test.

    - `host: string`

      The desired endpoint to test.

    - `kind: "http" or "traceroute"`

      The type of test.

      - `"http"`

      - `"traceroute"`

    - `method: optional "GET"`

      The HTTP request method type.

      - `"GET"`

  - `enabled: boolean`

    Determines whether or not the test is active.

  - `interval: string`

    How often the test will run.

  - `name: string`

    The name of the DEX test. Must be unique.

  - `created: optional string`

    Date the test was created, in RFC 3339 format.

  - `description: optional string`

    Additional details about the test.

  - `target_policies: optional array of object { id, default, name }`

    DEX rules targeted by this test

    - `id: string`

      The id of the DEX rule.

    - `default: optional boolean`

      Whether the DEX rule is the account default.

    - `name: optional string`

      The name of the DEX rule.

  - `targeted: optional boolean`

  - `test_id: optional string`

    The unique identifier for the test.

  - `updated: optional string`

    Date the test was last updated, in RFC 3339 format.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dex/devices/dex_tests/$DEX_TEST_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "data": {
            "host": "https://dash.cloudflare.com",
            "kind": "http",
            "method": "GET"
          },
          "enabled": true,
          "interval": "30m",
          "name": "HTTP dash health check",
          "description": "Checks the dash endpoint every 30 minutes"
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
    "data": {
      "host": "https://dash.cloudflare.com",
      "kind": "http",
      "method": "GET"
    },
    "enabled": true,
    "interval": "30m",
    "name": "HTTP dash health check",
    "created": "2023-10-11T00:00:00Z",
    "description": "Checks the dash endpoint every 30 minutes",
    "target_policies": [
      {
        "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
        "default": true,
        "name": "name"
      }
    ],
    "targeted": true,
    "test_id": "372e67954025e0ba6aaa6d586b9e0b59",
    "updated": "2023-10-11T00:00:00Z"
  }
}
```

## Delete Device DEX test

**delete** `/accounts/{account_id}/dex/devices/dex_tests/{dex_test_id}`

Delete a Device DEX test. Returns the remaining device dex tests for the account.

### Path Parameters

- `account_id: string`

  Unique identifier linked to an account.

- `dex_test_id: string`

  API Resource UUID tag.

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

- `result: optional object { dex_tests }`

  - `dex_tests: optional array of SchemaHTTP`

    - `data: SchemaData`

      The configuration object which contains the details for the WARP client to conduct the test.

      - `host: string`

        The desired endpoint to test.

      - `kind: "http" or "traceroute"`

        The type of test.

        - `"http"`

        - `"traceroute"`

      - `method: optional "GET"`

        The HTTP request method type.

        - `"GET"`

    - `enabled: boolean`

      Determines whether or not the test is active.

    - `interval: string`

      How often the test will run.

    - `name: string`

      The name of the DEX test. Must be unique.

    - `created: optional string`

      Date the test was created, in RFC 3339 format.

    - `description: optional string`

      Additional details about the test.

    - `target_policies: optional array of object { id, default, name }`

      DEX rules targeted by this test

      - `id: string`

        The id of the DEX rule.

      - `default: optional boolean`

        Whether the DEX rule is the account default.

      - `name: optional string`

        The name of the DEX rule.

    - `targeted: optional boolean`

    - `test_id: optional string`

      The unique identifier for the test.

    - `updated: optional string`

      Date the test was last updated, in RFC 3339 format.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dex/devices/dex_tests/$DEX_TEST_ID \
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
  "success": true,
  "result": {
    "dex_tests": [
      {
        "data": {
          "host": "https://dash.cloudflare.com",
          "kind": "http",
          "method": "GET"
        },
        "enabled": true,
        "interval": "30m",
        "name": "HTTP dash health check",
        "created": "2023-10-11T00:00:00Z",
        "description": "Checks the dash endpoint every 30 minutes",
        "target_policies": [
          {
            "id": "f174e90a-fafe-4643-bbbc-4a0ed4fc8415",
            "default": true,
            "name": "name"
          }
        ],
        "targeted": true,
        "test_id": "372e67954025e0ba6aaa6d586b9e0b59",
        "updated": "2023-10-11T00:00:00Z"
      }
    ]
  }
}
```

## Domain Types

### Schema Data

- `SchemaData object { host, kind, method }`

  The configuration object which contains the details for the WARP client to conduct the test.

  - `host: string`

    The desired endpoint to test.

  - `kind: "http" or "traceroute"`

    The type of test.

    - `"http"`

    - `"traceroute"`

  - `method: optional "GET"`

    The HTTP request method type.

    - `"GET"`

### Schema HTTP

- `SchemaHTTP object { data, enabled, interval, 7 more }`

  - `data: SchemaData`

    The configuration object which contains the details for the WARP client to conduct the test.

    - `host: string`

      The desired endpoint to test.

    - `kind: "http" or "traceroute"`

      The type of test.

      - `"http"`

      - `"traceroute"`

    - `method: optional "GET"`

      The HTTP request method type.

      - `"GET"`

  - `enabled: boolean`

    Determines whether or not the test is active.

  - `interval: string`

    How often the test will run.

  - `name: string`

    The name of the DEX test. Must be unique.

  - `created: optional string`

    Date the test was created, in RFC 3339 format.

  - `description: optional string`

    Additional details about the test.

  - `target_policies: optional array of object { id, default, name }`

    DEX rules targeted by this test

    - `id: string`

      The id of the DEX rule.

    - `default: optional boolean`

      Whether the DEX rule is the account default.

    - `name: optional string`

      The name of the DEX rule.

  - `targeted: optional boolean`

  - `test_id: optional string`

    The unique identifier for the test.

  - `updated: optional string`

    Date the test was last updated, in RFC 3339 format.

### DEX Test Delete Response

- `DEXTestDeleteResponse object { dex_tests }`

  - `dex_tests: optional array of SchemaHTTP`

    - `data: SchemaData`

      The configuration object which contains the details for the WARP client to conduct the test.

      - `host: string`

        The desired endpoint to test.

      - `kind: "http" or "traceroute"`

        The type of test.

        - `"http"`

        - `"traceroute"`

      - `method: optional "GET"`

        The HTTP request method type.

        - `"GET"`

    - `enabled: boolean`

      Determines whether or not the test is active.

    - `interval: string`

      How often the test will run.

    - `name: string`

      The name of the DEX test. Must be unique.

    - `created: optional string`

      Date the test was created, in RFC 3339 format.

    - `description: optional string`

      Additional details about the test.

    - `target_policies: optional array of object { id, default, name }`

      DEX rules targeted by this test

      - `id: string`

        The id of the DEX rule.

      - `default: optional boolean`

        Whether the DEX rule is the account default.

      - `name: optional string`

        The name of the DEX rule.

    - `targeted: optional boolean`

    - `test_id: optional string`

      The unique identifier for the test.

    - `updated: optional string`

      Date the test was last updated, in RFC 3339 format.
