## Create a relay

**post** `/accounts/{account_id}/moq/relays`

Provisions a new MoQ relay instance. Auto-creates a publish+subscribe
token and a subscribe-only token. Token values are included in the
response (shown once). Config is always set to defaults (upstreams
off) and cannot be supplied here — sending a non-empty `config` is
rejected (21014); `null` or `{}` is accepted as absent. Use PUT to
configure the relay after it exists.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

### Body Parameters

- `name: string`

  Human-readable name for the relay.

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

- `result: optional object { config, created, issuers, 3 more }`

  Relay with its auto-created default token pair (one full-access
  [publish, subscribe] and one [subscribe]-only), each with its one-time
  secret, wrapped in the issuers envelope.

  - `config: object { upstreams }`

    - `upstreams: optional object { enabled, upstreams }`

      Upstreams are external MOQT server publishers that a relay falls back
      to when it has no local publisher for a requested namespace/track.

      - `enabled: optional boolean`

      - `upstreams: optional array of object { url }`

        Ordered list of upstream MOQT server publishers. Each entry is an
        object (not a bare string) so per-upstream configuration can be
        added in the future without another breaking change.

        - `url: string`

          Upstream MOQT server publisher URL. Must be an absolute URL with a
          host and a scheme the relay can dial: moqt:// (raw QUIC) or https://
          (WebTransport). Validated on update (PUT); rejected with 21013.

  - `created: string`

  - `issuers: array of object { cloudflare_tokens, issuer, type }`

    Token collection (discriminated union on `type`). On create this
    holds the auto-created default pair, each including its one-time
    secret.

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

  - `modified: string`

  - `name: string`

  - `uid: string`

    Server-generated unique identifier (32 hex chars).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "name": "Production Live Stream"
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
    "config": {
      "upstreams": {
        "enabled": true,
        "upstreams": [
          {
            "url": "https://example.com"
          }
        ]
      }
    },
    "created": "2019-12-27T18:11:19.117Z",
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
    ],
    "modified": "2019-12-27T18:11:19.117Z",
    "name": "Production Live Stream",
    "uid": "a1b2c3d4e5f67890a1b2c3d4e5f67890"
  }
}
```
