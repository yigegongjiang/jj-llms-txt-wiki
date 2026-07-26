## Create a token

**post** `/accounts/{account_id}/moq/relays/{relay_id}/tokens`

Mints a new relay-scoped token and adds it to the relay's accepted-auth
registry. The token value (secret) is shown once in the response. A relay
may hold up to 10 tokens; creating an 11th is rejected.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

- `relay_id: string`

### Body Parameters

- `operations: array of "publish" or "subscribe"`

  Non-empty subset of the V1 roles the token is allowed to
  perform. Signed into the token.

  - `"publish"`

  - `"subscribe"`

- `expires: optional string`

  Optional expiry (RFC 3339). Defaults to 1 year from creation;
  rejected if more than 1 year in the future.

- `label: optional string`

  Optional, customer-set label.

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

- `result: optional object { issuers }`

  A relay's token collection, keyed on issuer `type` (a discriminated
  union). V1 ships exactly one arm (`cloudflare_jwt`). Clients iterate
  `issuers`, switch on `type`, and ignore unknown types — that contract is
  what makes adding or removing an arm non-breaking.

  - `issuers: array of object { cloudflare_tokens, issuer, type }`

    - `cloudflare_tokens: array of object { created, expires, jti, 3 more }`

      Always present ([] when empty).

      - `created: string`

      - `expires: string`

        Mandatory; no more than 1 year after `created`.

      - `jti: string`

        Token identity and registry key (32 hex chars).

      - `operations: array of "publish" or "subscribe"`

        Signed allowlist of what the token may do. V1 coarse roles; the array
        form extends to fine-grained MoQT message names later without a
        breaking change.

        - `"publish"`

        - `"subscribe"`

      - `label: optional string`

        Optional, customer-set.

      - `secret: optional string`

        The signed JWT. Present ONLY in create / auto-create responses (shown
        once); never returned by list, never stored.

    - `issuer: "cloudflare"`

      - `"cloudflare"`

    - `type: "cloudflare_jwt"`

      - `"cloudflare_jwt"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays/$RELAY_ID/tokens \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "operations": [
            "publish",
            "subscribe"
          ],
          "expires": "2027-03-27T15:00:00Z",
          "label": "primary-encoder"
        }'
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
  "success": true,
  "result": {
    "issuers": [
      {
        "cloudflare_tokens": [
          {
            "created": "2019-12-27T18:11:19.117Z",
            "expires": "2019-12-27T18:11:19.117Z",
            "jti": "f3a1b2c3d4e5f67890a1b2c3d4e5f678",
            "operations": [
              "publish",
              "subscribe"
            ],
            "label": "primary-encoder",
            "secret": "eyJhbGciOiJFZDI1NTE5..."
          }
        ],
        "issuer": "cloudflare",
        "type": "cloudflare_jwt"
      }
    ]
  }
}
```
