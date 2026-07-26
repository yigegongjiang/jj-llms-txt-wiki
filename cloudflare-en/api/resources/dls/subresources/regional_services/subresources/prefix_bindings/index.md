# Prefix Bindings

## List DLS prefix bindings for an account

**get** `/accounts/{account_id}/dls/regional_services/prefix_bindings`

List the BYOIP prefix bindings configured for an account.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

### Query Parameters

- `cursor: optional string`

  Opaque token for cursor-based pagination. Omit for the first page. Pass the value from a previous response to fetch the next page.

- `per_page: optional number`

### Returns

- `errors: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `result: array of object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

- `result_info: object { count, cursor, per_page }`

  - `count: number`

    Number of items in the current page.

  - `cursor: string`

    Opaque cursor for the next page. Empty string when there are no more results.

  - `per_page: number`

    Maximum number of items per page.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regional_services/prefix_bindings \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "result": [
    {
      "id": "id",
      "cidr": "cidr",
      "prefix_id": "prefix_id",
      "region_key": "x"
    }
  ],
  "result_info": {
    "count": 0,
    "cursor": "cursor",
    "per_page": 0
  },
  "success": true
}
```

## Get a DLS prefix binding

**get** `/accounts/{account_id}/dls/regional_services/prefix_bindings/{binding_id}`

Retrieve a single BYOIP prefix binding by ID.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

- `binding_id: string`

  Unique identifier for the prefix binding.

### Returns

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `result: object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

- `success: boolean`

- `errors: optional array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regional_services/prefix_bindings/$BINDING_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "result": {
    "id": "id",
    "cidr": "cidr",
    "prefix_id": "prefix_id",
    "region_key": "x"
  },
  "success": true,
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ]
}
```

## Create a DLS prefix binding

**post** `/accounts/{account_id}/dls/regional_services/prefix_bindings`

Bind a CIDR from a BYOIP prefix to a region.

This requires the **IP Prefixes Write** permission in addition to **DLS Write**, because the binding is created against a BYOIP prefix in Addressing.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

### Body Parameters

- `cidr: string`

  IP prefix in CIDR notation to bind.

- `prefix_id: string`

  The ID of the parent IP prefix that contains the CIDR.

- `region_key: string`

  Region key from managed regions (e.g., "us", "eu").

### Returns

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `result: object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

- `success: boolean`

- `errors: optional array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regional_services/prefix_bindings \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "cidr": "10.0.1.0/24",
          "prefix_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
          "region_key": "eu"
        }'
```

#### Response

```json
{
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "result": {
    "id": "id",
    "cidr": "cidr",
    "prefix_id": "prefix_id",
    "region_key": "x"
  },
  "success": true,
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ]
}
```

## Update a DLS prefix binding

**patch** `/accounts/{account_id}/dls/regional_services/prefix_bindings/{binding_id}`

Update the region of an existing BYOIP prefix binding.

Like creating a binding, this requires **IP Prefixes Write** in addition to **DLS Write**.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

- `binding_id: string`

  Unique identifier for the prefix binding.

### Body Parameters

- `region_key: string`

  New region key to assign (e.g., "us", "eu", "cfcanary").

### Returns

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `result: object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

- `success: boolean`

- `errors: optional array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regional_services/prefix_bindings/$BINDING_ID \
    -X PATCH \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "region_key": "eu"
        }'
```

#### Response

```json
{
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "result": {
    "id": "id",
    "cidr": "cidr",
    "prefix_id": "prefix_id",
    "region_key": "x"
  },
  "success": true,
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ]
}
```

## Delete a DLS prefix binding

**delete** `/accounts/{account_id}/dls/regional_services/prefix_bindings/{binding_id}`

Delete a BYOIP prefix binding.

Like creating a binding, this requires **IP Prefixes Write** in addition to **DLS Write**.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

- `binding_id: string`

  Unique identifier for the prefix binding.

### Returns

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `success: boolean`

- `errors: optional array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regional_services/prefix_bindings/$BINDING_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "success": true,
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ]
}
```

## Domain Types

### Prefix Binding List Response

- `PrefixBindingListResponse object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

### Prefix Binding Get Response

- `PrefixBindingGetResponse object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

### Prefix Binding Create Response

- `PrefixBindingCreateResponse object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

### Prefix Binding Edit Response

- `PrefixBindingEditResponse object { id, cidr, prefix_id, region_key }`

  - `id: string`

    The ID of the binding.

  - `cidr: string`

    The CIDR that is bound.

  - `prefix_id: string`

    The ID of the parent prefix.

  - `region_key: string`

    The region key used for the binding.

### Prefix Binding Delete Response

- `PrefixBindingDeleteResponse object { messages, success, errors }`

  - `messages: array of object { code, message, error_chain }`

    - `code: number`

    - `message: string`

    - `error_chain: optional array of unknown`

      Optional upstream error context for APIv4 errors that wrap downstream service failures.

  - `success: boolean`

  - `errors: optional array of object { code, message, error_chain }`

    - `code: number`

    - `message: string`

    - `error_chain: optional array of unknown`

      Optional upstream error context for APIv4 errors that wrap downstream service failures.
