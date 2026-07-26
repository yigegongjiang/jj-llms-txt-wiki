## Update an existing webhook configuration

**put** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}`

Updates an existing webhook configuration with new settings.

### Path Parameters

- `account_id: string`

- `webhook_id: string`

### Body Parameters

- `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

  Type of authentication used for the webhook.

  - `"Basic Auth"`

  - `"None"`

  - `"Bearer Auth"`

  - `"Static Headers"`

  - `"HMAC-Signing"`

- `destination_url: string`

  Target URL for the webhook configuration. Where resulting data will be sent.

- `label: string`

  Account-specified display label for the webhook configuration.

- `status: "enabled" or "disabled"`

  Status of the webhook configuration.

  - `"enabled"`

  - `"disabled"`

- `headers: optional array of object { key, value }`

  List of custom headers to include in webhook requests.

  - `key: string`

    Header key name.

  - `value: optional string`

    Header value. Required on Create and Evaluate. On Update, omit or set to null to keep existing value.

- `signing_secret: optional string`

  Secret key used for HMAC signing when authentication_type is "HMAC-Signing".

### Returns

- `errors: array of object { code, message, documentation_url, source }`

  - `code: number`

    Error or message code.

  - `message: string`

    Human-readable message.

  - `documentation_url: optional string`

    Link to relevant documentation.

  - `source: optional object { pointer }`

    - `pointer: optional string`

      JSON pointer to the source of the error.

- `messages: array of object { code, message, documentation_url, source }`

  - `code: number`

    Error or message code.

  - `message: string`

    Human-readable message.

  - `documentation_url: optional string`

    Link to relevant documentation.

  - `source: optional object { pointer }`

    - `pointer: optional string`

      JSON pointer to the source of the error.

- `success: boolean`

  Whether the API call was successful.

- `result: optional object { id, authentication_type, created_at, 6 more }`

  Webhook configuration for sending finding notifications.

  - `id: string`

    Unique identifier for the specific webhook configuration.

  - `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

    Type of authentication used for the webhook.

    - `"Basic Auth"`

    - `"None"`

    - `"Bearer Auth"`

    - `"Static Headers"`

    - `"HMAC-Signing"`

  - `created_at: string`

    Timestamp when the webhook configuration was created.

  - `destination_url: string`

    Target URL for the webhook configuration. Where resulting data will be sent.

  - `label: string`

    Account-specified display label for the webhook configuration.

  - `status: "enabled" or "disabled"`

    Current status of the webhook configuration. If disabled, data cannot be sent through this configuration.

    - `"enabled"`

    - `"disabled"`

  - `updated_at: string`

    Timestamp when the webhook configuration was last updated.

  - `version: number`

    Version number of the configuration.

  - `headers: optional array of object { key, value }`

    List of header keys configured for this webhook. Values are not included for security reasons.

    - `key: optional string`

      Header key name (lowercase).

    - `value: optional string`

      Header value. This field is never returned in API responses for security reasons.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "authentication_type": "Bearer Auth",
          "destination_url": "https://example.com/webhook",
          "label": "Send to Slack",
          "status": "enabled",
          "headers": [
            {
              "key": "Authorization",
              "value": "Bearer token123"
            },
            {
              "key": "X-Custom-Header",
              "value": "value"
            }
          ],
          "signing_secret": "my-secret-key"
        }'
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "Request processed successfully",
      "documentation_url": "https://developers.cloudflare.com/api/operations/list-findings",
      "source": {
        "pointer": "/data/attributes/name"
      }
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "Request processed successfully",
      "documentation_url": "https://developers.cloudflare.com/api/operations/list-findings",
      "source": {
        "pointer": "/data/attributes/name"
      }
    }
  ],
  "success": true,
  "result": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "authentication_type": "Bearer Auth",
    "created_at": "2024-01-15T10:30:00Z",
    "destination_url": "https://example.com/webhook",
    "label": "Send to Gmail",
    "status": "enabled",
    "updated_at": "2024-01-20T14:45:00Z",
    "version": 1,
    "headers": [
      {
        "key": "authorization"
      }
    ]
  }
}
```
