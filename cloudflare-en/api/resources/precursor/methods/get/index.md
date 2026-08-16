## Get Zone Precursor Config

**get** `/zones/{zone_id}/precursor`

Retrieve a zone's Precursor configuration: the zone-level
`default_mode` and the ordered list of `enforcement_rules`.

### Path Parameters

- `zone_id: string`

  Identifier.

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

- `result: optional PrecursorConfig`

  - `default_mode: optional "off" or "min-friction" or "max-security"`

    The zone-level Precursor enforcement mode applied to requests that do
    not match a more specific enforcement rule.

    - `"off"`

    - `"min-friction"`

    - `"max-security"`

  - `enforcement_rules: optional array of EnforcementRule`

    The ordered list of enforcement rules for the zone.

    - `expression: string`

      The filter expression that determines which requests the rule matches.

    - `mode: "min-friction" or "max-security"`

      The override mode Precursor applies to requests matching an enforcement
      rule. Unlike `default_mode`, this cannot be `off`.

      - `"min-friction"`

      - `"max-security"`

    - `id: optional string`

      The read-only identifier that Cloudflare assigns to the rule.

    - `description: optional string`

      An informative description of the rule.

    - `enabled: optional boolean`

      Whether the rule is active.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/precursor \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
    "default_mode": "min-friction",
    "enforcement_rules": [
      {
        "expression": "http.request.uri.path eq \"/shop\"",
        "mode": "max-security",
        "id": "3a03d665bac043e3a684e0d385a4b1e2",
        "description": "Enforce max-security on the shop page",
        "enabled": true
      }
    ]
  }
}
```
