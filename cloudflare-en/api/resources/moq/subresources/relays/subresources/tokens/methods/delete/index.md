## Revoke a token

**delete** `/accounts/{account_id}/moq/relays/{relay_id}/tokens/{jti}`

Revokes a token by removing it from the relay's registry. crique rejects
the token within the cache TTL. Idempotent — revoking an unknown token
succeeds.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

- `relay_id: string`

- `jti: string`

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays/$RELAY_ID/tokens/$JTI \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "messages": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "success": true
}
```
