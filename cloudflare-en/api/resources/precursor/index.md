# Precursor

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

## Update Zone Precursor Config

**put** `/zones/{zone_id}/precursor`

Updates the Precursor configuration for a zone.

`default_mode` sets the zone-level enforcement mode. `enforcement_rules`
is the ordered list of rules that override enforcement for matching
requests.

This is a partial update: only the fields present in the request body
are changed.

- Sending an empty array (`[]`) clears all enforcement rules.
- At least one of `default_mode` or `enforcement_rules` must be present;
  an empty body (`{}`) is rejected with `400`.
- Rule `id` is read-only (assigned by Cloudflare) and ignored on input.
- Rule `mode` must be `min-friction` or `max-security` (`off` is not a
  valid rule mode; use `default_mode` to disable enforcement).

### Path Parameters

- `zone_id: string`

  Identifier.

### Body Parameters

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
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "default_mode": "min-friction"
        }'
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

## Domain Types

### Enforcement Rule

- `EnforcementRule object { expression, mode, id, 2 more }`

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

### Precursor Config

- `PrecursorConfig object { default_mode, enforcement_rules }`

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
