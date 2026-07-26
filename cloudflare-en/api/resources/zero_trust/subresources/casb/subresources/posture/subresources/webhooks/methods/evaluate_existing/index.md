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
