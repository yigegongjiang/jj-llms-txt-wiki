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
