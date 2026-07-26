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
