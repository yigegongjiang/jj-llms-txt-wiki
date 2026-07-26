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
