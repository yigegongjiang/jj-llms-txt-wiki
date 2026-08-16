## Update CT Alerting Subscription

**patch** `/zones/{zone_id}/ct/alerting`

Create or update the Certificate Transparency alerting subscription for a zone. Enables or disables email notifications when certificates are issued for the zone's domains.
The `enabled` field is required on every request and controls whether the subscription is active. The `emails` field is optional and, when provided, replaces the stored recipient list for the zone. When `emails` is omitted, the stored recipient list is preserved and only the enabled state is toggled. A maximum of 100 email addresses may be configured per zone.
Requests that omit `enabled` are rejected with error code 1008.
Subscribe and unsubscribe notification emails are only sent for recipients whose effective subscription state changes. Idempotent requests (no state change) send no notification email.

### Path Parameters

- `zone_id: string`

  Identifier.

### Body Parameters

- `enabled: boolean`

  Whether CT alerting is enabled for the zone.

- `emails: optional array of string`

  Email addresses that receive CT alert notifications for the zone. A maximum of 100 addresses may be configured. Each address must be a valid RFC 5322 email address and must not contain a comma.

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

- `result: optional CTAlertingSubscription`

  Certificate Transparency alerting subscription settings for a zone.

  - `enabled: boolean`

    Whether CT alerting is enabled for the zone.

  - `emails: optional array of string`

    Email addresses that receive CT alert notifications for the zone. A maximum of 100 addresses may be configured. Each address must be a valid RFC 5322 email address and must not contain a comma.

### Example

```http
curl https://api.cloudflare.com/client/v4/zones/$ZONE_ID/ct/alerting \
    -X PATCH \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "enabled": true,
          "emails": [
            "security@example.com",
            "admin@example.com"
          ]
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
    "enabled": true,
    "emails": [
      "security@example.com",
      "admin@example.com"
    ]
  }
}
```
