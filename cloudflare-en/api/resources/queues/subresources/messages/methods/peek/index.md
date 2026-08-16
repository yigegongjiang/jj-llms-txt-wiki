## Peek Queue Messages

**post** `/accounts/{account_id}/queues/{queue_id}/messages/peek`

Peek messages from a Queue without leasing them. Messages remain available for subsequent peek or pull operations.

### Path Parameters

- `account_id: string`

  A Resource identifier.

- `queue_id: string`

  A Resource identifier.

### Body Parameters

- `batch_size: optional number`

  The maximum number of messages to include in a batch.

### Returns

- `errors: optional array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: optional array of string`

- `result: optional object { messages }`

  - `messages: optional array of object { id, attempts, body, 3 more }`

    - `id: optional string`

    - `attempts: optional number`

    - `body: optional string`

    - `metadata: optional unknown`

    - `ref: optional string`

      An opaque reference to a peeked message. You must hold on to this value and use it to purge the message.

    - `timestamp_ms: optional number`

- `success: optional true`

  Indicates if the API call was successful or not.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/queues/$QUEUE_ID/messages/peek \
    -X POST \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 7003,
      "message": "No route for the URI",
      "documentation_url": "documentation_url",
      "source": {
        "pointer": "pointer"
      }
    }
  ],
  "messages": [
    "string"
  ],
  "result": {
    "messages": [
      {
        "id": "b01b5594f784d0165c2985833f5660dd",
        "attempts": 1,
        "body": "hello world",
        "metadata": {
          "CF-Content-Type": "text",
          "CF-sourceMessageSource": "dash"
        },
        "ref": "eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2Q0JDLUhTNTEyIn0..Q8p21d7dceR6vUfwftONdQ.JVqZgAS-Zk7MqmqccYtTHeeMElNHaOMigeWdb8LyMOg.T2_HV99CYzGaQuhTyW8RsgbnpTRZHRM6N7UoSaAKeK0",
        "timestamp_ms": 1710950954154
      }
    ]
  },
  "success": true
}
```
