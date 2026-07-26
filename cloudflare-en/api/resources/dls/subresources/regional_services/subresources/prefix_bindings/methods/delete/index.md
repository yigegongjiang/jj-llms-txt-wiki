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
