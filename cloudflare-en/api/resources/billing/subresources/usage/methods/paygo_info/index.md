## Get PayGo Account Billable Usage Info (Version 1, Alpha)

**get** `/accounts/{account_id}/paygo-usage-info`

Returns high-level usage information for the account, including coverage,
and subscription metadata.

### Path Parameters

- `account_id: string`

  Represents a Cloudflare resource identifier tag.

### Returns

- `errors: array of object { message, code }`

  Contains error details if the request failed.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `messages: array of object { message, code }`

  Contains any informational messages from the API.

  - `message: string`

    Describes the error or notice.

  - `code: optional number`

    Identifies the error or notice type.

- `result: object { covered, subscriptions }`

  Contains the paygo usage info.

  - `covered: boolean`

    Indicates whether the account is covered.

  - `subscriptions: array of object { id, billing_cycle_anchor_timestamp, start_timestamp, end_timestamp }`

    List of subscriptions for the account.

    - `id: string`

      The identifier for the Cloudflare subscription.

    - `billing_cycle_anchor_timestamp: string`

      The subscription billing cycle anchor timestamp.

    - `start_timestamp: string`

      The subscription start timestamp.

    - `end_timestamp: optional string`

      The subscription end timestamp. Omitted for active subscriptions; present only when the subscription has been cancelled.

- `success: true`

  Indicates whether the API call was successful.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/paygo-usage-info \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "message": "message",
      "code": 0
    }
  ],
  "messages": [
    {
      "message": "message",
      "code": 0
    }
  ],
  "result": {
    "covered": true,
    "subscriptions": [
      {
        "id": "3F3CD4CQ6N7FXO7IK6NVFJBOYA",
        "billing_cycle_anchor_timestamp": "2023-01-01T00:00:00Z",
        "start_timestamp": "2023-01-01T00:00:00Z",
        "end_timestamp": "2023-12-31T23:59:59Z"
      }
    ]
  },
  "success": true
}
```
