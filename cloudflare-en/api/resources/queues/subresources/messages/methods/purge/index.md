## Purge Peeked Queue Messages

**post** `/accounts/{account_id}/queues/{queue_id}/messages/purge`

Delete peeked messages from a Queue by their ref. Purged messages aren't considered delivered, they are instantly deleted from this queue and do not affect metrics.

### Path Parameters

- `account_id: string`

  A Resource identifier.

- `queue_id: string`

  A Resource identifier.

### Body Parameters

- `refs: array of object { ref }`

  - `ref: string`

    An opaque reference to a peeked message. You must hold on to this value and use it to purge the message.

### Returns

- `errors: optional array of ResponseInfo`

  - `code: number`

  - `message: string`

  - `documentation_url: optional string`

  - `source: optional object { pointer }`

    - `pointer: optional string`

- `messages: optional array of string`

- `result: optional object { errors, warnings }`

  - `errors: optional array of object { message }`

    Errors encountered while purging messages.

    - `message: optional string`

  - `warnings: optional map[string]`

    Map of refs to warning messages encountered during purge.

- `success: optional true`

  Indicates if the API call was successful or not.

  - `true`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/queues/$QUEUE_ID/messages/purge \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "refs": [
            {
              "ref": "eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2Q0JDLUhTNTEyIn0..Q8p21d7dceR6vUfwftONdQ.JVqZgAS-Zk7MqmqccYtTHeeMElNHaOMigeWdb8LyMOg.T2_HV99CYzGaQuhTyW8RsgbnpTRZHRM6N7UoSaAKeK0"
            }
          ]
        }'
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
    "errors": [
      {
        "message": "message"
      }
    ],
    "warnings": {
      "foo": "string"
    }
  },
  "success": true
}
```
