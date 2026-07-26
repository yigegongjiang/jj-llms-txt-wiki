## Get a secret by ID

**get** `/accounts/{account_id}/secrets_store/stores/{store_id}/secrets/{secret_id}`

Returns details of a single secret.

### Path Parameters

- `account_id: string`

- `store_id: string`

- `secret_id: string`

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

- `result: optional object { id, created, modified, 5 more }`

  - `id: string`

    Secret identifier tag.

  - `created: string`

    When the secret was created.

  - `modified: string`

    When the secret was modified.

  - `name: string`

    The name of the secret.

  - `status: "pending" or "active" or "deleted"`

    - `"pending"`

    - `"active"`

    - `"deleted"`

  - `store_id: string`

    Store Identifier.

  - `comment: optional string`

    Freeform text describing the secret.

  - `scopes: optional array of "workers" or "ai_gateway" or "dex" or 3 more`

    The list of services that can use this secret.

    - `"workers"`

    - `"ai_gateway"`

    - `"dex"`

    - `"access"`

    - `"containers"`

    - `"websearch"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/secrets_store/stores/$STORE_ID/secrets/$SECRET_ID \
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
    "id": "3fd85f74b32742f1bff64a85009dda07",
    "created": "2023-09-21T18:56:32.624632Z",
    "modified": "2023-09-21T18:56:32.624632Z",
    "name": "MY_API_KEY",
    "status": "pending",
    "store_id": "023e105f4ecef8ad9ca31a8372d0c353",
    "comment": "info about my secret",
    "scopes": [
      "workers",
      "ai_gateway",
      "dex",
      "access",
      "websearch"
    ]
  }
}
```
