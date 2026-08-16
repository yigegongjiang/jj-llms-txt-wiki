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
        host and a scheme the relay can dial: moqt:// (raw QUIC) or https://
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
          host and a scheme the relay can dial: moqt:// (raw QUIC) or https://
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
