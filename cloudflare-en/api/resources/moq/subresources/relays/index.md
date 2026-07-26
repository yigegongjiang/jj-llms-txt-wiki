# Relays

## List relays

**get** `/accounts/{account_id}/moq/relays`

Lists all MoQ relays for the account. Returns only metadata.
Config, status, and tokens are omitted.

Results are cursor-paginated (keyset on the `created` timestamp).
Use `created_before` / `created_after` with the `created` value of the
first/last item in a page to fetch the adjacent page. `result_info`
reports the page `count` and the `total` matching the cursor filters.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

### Query Parameters

- `asc: optional boolean`

  Sort order by `created`. When true, results are returned oldest-first
  (ascending); otherwise newest-first (descending, the default).

- `created_after: optional string`

  Cursor for pagination. Returns relays created strictly after this
  RFC 3339 timestamp (typically the `created` value of the last item
  on the current page, to fetch the next page).

- `created_before: optional string`

  Cursor for pagination. Returns relays created strictly before this
  RFC 3339 timestamp (typically the `created` value of the first item
  on the current page, to fetch the previous page).

- `per_page: optional number`

  Maximum number of relays to return per page.

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

- `result: optional array of object { created, modified, name, uid }`

  - `created: string`

  - `modified: string`

  - `name: string`

  - `uid: string`

- `result_info: optional object { count, total }`

  - `count: optional number`

  - `total: optional number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays \
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
  "success": true,
  "result": [
    {
      "created": "2019-12-27T18:11:19.117Z",
      "modified": "2019-12-27T18:11:19.117Z",
      "name": "name",
      "uid": "a1b2c3d4e5f67890a1b2c3d4e5f67890"
    }
  ],
  "result_info": {
    "count": 0,
    "total": 0
  }
}
```

## Get a relay

**get** `/accounts/{account_id}/moq/relays/{relay_id}`

Retrieves a single MoQ relay including config and status.
Tokens are NOT included.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

- `relay_id: string`

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

- `result: optional object { config, created, modified, 3 more }`

  Full relay details (no tokens).

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
          host and a scheme crique can dial: moqt:// (raw QUIC) or https://
          (WebTransport). Validated on update (PUT); rejected with 21013.

  - `created: string`

  - `modified: string`

  - `name: string`

  - `uid: string`

  - `status: optional "connected"`

    "connected" when active, omitted otherwise.

    - `"connected"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays/$RELAY_ID \
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
    "modified": "2019-12-27T18:11:19.117Z",
    "name": "Production Live Stream",
    "uid": "a1b2c3d4e5f67890a1b2c3d4e5f67890",
    "status": "connected"
  }
}
```

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
          host and a scheme crique can dial: moqt:// (raw QUIC) or https://
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

## Update a relay

**put** `/accounts/{account_id}/moq/relays/{relay_id}`

Updates a relay's name and/or configuration. The relay ID goes in
the URL path — `PUT /accounts/{account_id}/moq/relays/{relay_id}` —
not the request body; there is no collection-level update endpoint.
This is also the only way to set a relay's config (config cannot be
set at create time). Partial updates: omitted fields are preserved;
config sub-objects replace as whole objects when present.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

- `relay_id: string`

### Body Parameters

- `config: optional object { upstreams }`

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
        host and a scheme crique can dial: moqt:// (raw QUIC) or https://
        (WebTransport). Validated on update (PUT); rejected with 21013.

- `name: optional string`

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

- `result: optional object { config, created, modified, 3 more }`

  Full relay details (no tokens).

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
          host and a scheme crique can dial: moqt:// (raw QUIC) or https://
          (WebTransport). Validated on update (PUT); rejected with 21013.

  - `created: string`

  - `modified: string`

  - `name: string`

  - `uid: string`

  - `status: optional "connected"`

    "connected" when active, omitted otherwise.

    - `"connected"`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays/$RELAY_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{}'
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
    "modified": "2019-12-27T18:11:19.117Z",
    "name": "Production Live Stream",
    "uid": "a1b2c3d4e5f67890a1b2c3d4e5f67890",
    "status": "connected"
  }
}
```

## Delete a relay

**delete** `/accounts/{account_id}/moq/relays/{relay_id}`

Soft-deletes a MoQ relay. The relay ID goes in the URL path —
`DELETE /accounts/{account_id}/moq/relays/{relay_id}` — not the
request body; there is no collection-level delete endpoint.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

- `relay_id: string`

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `success: boolean`

- `result: optional unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/moq/relays/$RELAY_ID \
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
  "success": true,
  "result": {}
}
```

## Domain Types

### Relay List Response

- `RelayListResponse object { created, modified, name, uid }`

  Abbreviated relay for list responses.

  - `created: string`

  - `modified: string`

  - `name: string`

  - `uid: string`

### Relay Get Response

- `RelayGetResponse object { config, created, modified, 3 more }`

  Full relay details (no tokens).

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
          host and a scheme crique can dial: moqt:// (raw QUIC) or https://
          (WebTransport). Validated on update (PUT); rejected with 21013.

  - `created: string`

  - `modified: string`

  - `name: string`

  - `uid: string`

  - `status: optional "connected"`

    "connected" when active, omitted otherwise.

    - `"connected"`

### Relay Create Response

- `RelayCreateResponse object { config, created, issuers, 3 more }`

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
          host and a scheme crique can dial: moqt:// (raw QUIC) or https://
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

### Relay Update Response

- `RelayUpdateResponse object { config, created, modified, 3 more }`

  Full relay details (no tokens).

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
          host and a scheme crique can dial: moqt:// (raw QUIC) or https://
          (WebTransport). Validated on update (PUT); rejected with 21013.

  - `created: string`

  - `modified: string`

  - `name: string`

  - `uid: string`

  - `status: optional "connected"`

    "connected" when active, omitted otherwise.

    - `"connected"`

### Relay Delete Response

- `RelayDeleteResponse = unknown`

# Tokens

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

## List tokens

**get** `/accounts/{account_id}/moq/relays/{relay_id}/tokens`

Returns metadata for every token in the relay's registry. Secrets are
never returned. The dashboard derives an `expired` flag by comparing
each token's `expires` to the current time.

### Path Parameters

- `account_id: string`

  Cloudflare account identifier.

- `relay_id: string`

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

## Domain Types

### Token Create Response

- `TokenCreateResponse object { issuers }`

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

### Token List Response

- `TokenListResponse object { issuers }`

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

### Token Delete Response

- `TokenDeleteResponse object { errors, messages, success }`

  - `errors: array of object { code, message }`

    - `code: optional number`

    - `message: optional string`

  - `messages: array of object { code, message }`

    - `code: optional number`

    - `message: optional string`

  - `success: boolean`
