# Webhooks

## List webhook configurations

**get** `/accounts/{account_id}/data-security/posture/webhooks`

Retrieves all webhook configurations for the authenticated account.
Returns an array of webhook configurations that can be used to send finding notifications.

### Path Parameters

- `account_id: string`

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

- `result: optional array of object { id, authentication_type, created_at, 6 more }`

  List of webhook configurations.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
  "result": [
    {
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
  ]
}
```

## Create a new webhook configuration

**post** `/accounts/{account_id}/data-security/posture/webhooks`

Creates a new webhook configuration for sending finding notifications to external endpoints.

### Path Parameters

- `account_id: string`

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "authentication_type": "Bearer Auth",
          "destination_url": "https://example.com/webhook",
          "label": "Send to Slack",
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

## Get webhook configuration by ID

**get** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}`

Retrieves a specific webhook configuration by its unique identifier.

### Path Parameters

- `account_id: string`

- `webhook_id: string`

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
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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

## Delete a webhook configuration

**delete** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}`

Soft deletes a webhook configuration by its unique identifier.
The webhook will be marked as deleted and will no longer be available for use.

### Path Parameters

- `account_id: string`

- `webhook_id: string`

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

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID \
    -X DELETE \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
  "success": true
}
```

## Test a webhook configuration before creating it

**post** `/accounts/{account_id}/data-security/posture/webhooks/evaluate`

Sends a test webhook event to the specified destination URL to verify the webhook endpoint
is reachable and properly configured. This allows customers to validate their webhook
configuration before creating the actual webhook resource.

The test payload includes:

- event_type: "webhook.test"
- timestamp: Current UTC timestamp
- message: Test message indicating this is from Cloudflare CASB
- data: Object with test: true

### Path Parameters

- `account_id: string`

### Body Parameters

- `authentication_type: "Basic Auth" or "None" or "Bearer Auth" or 2 more`

  Type of authentication to use for the test webhook request.

  - `"Basic Auth"`

  - `"None"`

  - `"Bearer Auth"`

  - `"Static Headers"`

  - `"HMAC-Signing"`

- `destination_url: string`

  Target URL to send the test webhook event to.

- `headers: optional array of object { key, value }`

  List of custom headers to include in the test webhook request.

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

- `result: optional object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/evaluate \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "authentication_type": "Bearer Auth",
          "destination_url": "https://example.com/webhook",
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
    "message": "Webhook test successful",
    "status_code": 200,
    "success": true
  }
}
```

## Test an existing webhook configuration

**post** `/accounts/{account_id}/data-security/posture/webhooks/{webhook_id}/evaluate`

Sends a test webhook event using an existing webhook configuration.
This allows customers to verify their webhook endpoint is still reachable and properly
configured after creating the webhook resource.

The test payload includes:

- event_type: "webhook.test"
- timestamp: Current UTC timestamp
- message: Test message indicating this is from Cloudflare CASB
- data: Object with test: true

### Path Parameters

- `account_id: string`

- `webhook_id: string`

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

- `result: optional object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/$WEBHOOK_ID/evaluate \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
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
    "message": "Webhook test successful",
    "status_code": 200,
    "success": true
  }
}
```

## Domain Types

### Webhook List Response

- `WebhookListResponse object { id, authentication_type, created_at, 6 more }`

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

### Webhook Create Response

- `WebhookCreateResponse object { id, authentication_type, created_at, 6 more }`

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

### Webhook Get Response

- `WebhookGetResponse object { id, authentication_type, created_at, 6 more }`

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

### Webhook Update Response

- `WebhookUpdateResponse object { id, authentication_type, created_at, 6 more }`

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

### Webhook Delete Response

- `WebhookDeleteResponse object { errors, messages, success }`

  Common response structure for all API endpoints.

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

### Webhook Evaluate Response

- `WebhookEvaluateResponse object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

### Webhook Evaluate Existing Response

- `WebhookEvaluateExistingResponse object { message, status_code, success }`

  Response body for webhook evaluation test results.

  - `message: string`

    Human-readable message describing the test result.

  - `status_code: number`

    HTTP status code returned by the webhook endpoint. 0 if connection failed.

  - `success: boolean`

    Whether the webhook test was successful (received 2xx response).

# Jobs

## Create webhook jobs

**post** `/accounts/{account_id}/data-security/posture/webhooks/jobs`

Creates webhook jobs to send a finding instance to one or more configured webhooks.

### Path Parameters

- `account_id: string`

### Body Parameters

- `finding_instance_ids: array of string`

  Array of finding instance IDs to send to the webhooks.

- `webhook_ids: array of string`

  Array of webhook IDs to trigger jobs for.

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

- `result: object { created, failed }`

  - `created: array of object { id, asset_data, created_at, 9 more }`

    Successfully created webhook jobs.

    - `id: string`

      Unique identifier for the webhook job.

    - `asset_data: map[unknown]`

      Asset data associated with this webhook job.

    - `created_at: string`

      When the webhook job was created.

    - `integration_id: string`

      ID of the integration.

    - `last_updated_at: string`

      When the webhook job was last updated.

    - `parameters: object { finding_instance_id }`

      Parameters for a webhook job.

      - `finding_instance_id: string`

        ID of the finding instance.

    - `status: "pending" or "processing" or "completed" or "failed"`

      Status of a webhook job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

    - `triggered_by_actor: "user" or "account_token"`

      Type of actor that triggered the webhook job.

      - `"user"`

      - `"account_token"`

    - `triggered_by_id: string`

      ID of the actor that triggered the job.

    - `webhook_id: string`

      ID of the webhook configuration.

    - `failure_details: optional map[unknown]`

      Additional details about the failure.

    - `failure_reason: optional "Permission Denied" or "Integration Unavailable" or "Service Temporarily Unavailable" or "System Error"`

      Reason for webhook job failure.

      - `"Permission Denied"`

      - `"Integration Unavailable"`

      - `"Service Temporarily Unavailable"`

      - `"System Error"`

  - `failed: array of object { error, finding_instance_id, webhook_id }`

    Failed webhook job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a webhook job.

    - `webhook_id: string`

      ID of the webhook configuration.

- `success: boolean`

  Whether the API call was successful.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/data-security/posture/webhooks/jobs \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "finding_instance_ids": [
            "770e8400-e29b-41d4-a716-446655440002",
            "660e8400-e29b-41d4-a716-446655440001"
          ],
          "webhook_ids": [
            "550e8400-e29b-41d4-a716-446655440000",
            "660e8400-e29b-41d4-a716-446655440001"
          ]
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
  "result": {
    "created": [
      {
        "id": "c416bc38-75db-425f-ae25-c37b5df5c37f",
        "asset_data": {
          "foo": "bar"
        },
        "created_at": "2025-07-07T18:39:13.123456Z",
        "integration_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "last_updated_at": "2025-07-07T18:39:13.123456Z",
        "parameters": {
          "finding_instance_id": "3f7b8c9d-6e5a-4f3b-9c2d-1e0a8b7c6d5e"
        },
        "status": "pending",
        "triggered_by_actor": "user",
        "triggered_by_id": "user@example.com",
        "webhook_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
        "failure_details": {
          "foo": "bar"
        },
        "failure_reason": "Permission Denied"
      }
    ],
    "failed": [
      {
        "error": "Failed to create webhook job",
        "finding_instance_id": "2e6b4c8a-9d1f-4e3b-8c7a-5f9e2d1a6b4c",
        "webhook_id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e"
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Job Create Response

- `JobCreateResponse object { created, failed }`

  - `created: array of object { id, asset_data, created_at, 9 more }`

    Successfully created webhook jobs.

    - `id: string`

      Unique identifier for the webhook job.

    - `asset_data: map[unknown]`

      Asset data associated with this webhook job.

    - `created_at: string`

      When the webhook job was created.

    - `integration_id: string`

      ID of the integration.

    - `last_updated_at: string`

      When the webhook job was last updated.

    - `parameters: object { finding_instance_id }`

      Parameters for a webhook job.

      - `finding_instance_id: string`

        ID of the finding instance.

    - `status: "pending" or "processing" or "completed" or "failed"`

      Status of a webhook job.

      - `"pending"`

      - `"processing"`

      - `"completed"`

      - `"failed"`

    - `triggered_by_actor: "user" or "account_token"`

      Type of actor that triggered the webhook job.

      - `"user"`

      - `"account_token"`

    - `triggered_by_id: string`

      ID of the actor that triggered the job.

    - `webhook_id: string`

      ID of the webhook configuration.

    - `failure_details: optional map[unknown]`

      Additional details about the failure.

    - `failure_reason: optional "Permission Denied" or "Integration Unavailable" or "Service Temporarily Unavailable" or "System Error"`

      Reason for webhook job failure.

      - `"Permission Denied"`

      - `"Integration Unavailable"`

      - `"Service Temporarily Unavailable"`

      - `"System Error"`

  - `failed: array of object { error, finding_instance_id, webhook_id }`

    Failed webhook job creation attempts.

    - `error: string`

      Error message describing the failure.

    - `finding_instance_id: string`

      ID of the finding instance that failed to create a webhook job.

    - `webhook_id: string`

      ID of the webhook configuration.
