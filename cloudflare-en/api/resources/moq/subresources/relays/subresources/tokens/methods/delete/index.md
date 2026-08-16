## Revoke a token

**delete** `/accounts/{account_id}/moq/relays/{relay_id}/tokens/{jti}`

Revokes a token by removing it from the set the relay accepts. Relays
cache that set, so revocation takes effect within seconds rather than
instantly, and connections already established with the token are not
closed. Revoking an unknown token succeeds, so the call is idempotent.

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
