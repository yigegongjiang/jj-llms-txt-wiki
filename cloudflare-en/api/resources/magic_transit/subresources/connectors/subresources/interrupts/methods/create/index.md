## Create Interrupt

**post** `/accounts/{account_id}/magic/connectors/{connector_id}/interrupts`

Creates an interrupt for a Magic WAN Connector.

### Path Parameters

- `account_id: string`

  Account identifier

- `connector_id: string`

### Body Parameters

- `reboot: optional object { purge }`

  - `purge: optional boolean`

    Purge connector state.

- `restart: optional object { purge }`

  - `purge: optional boolean`

    Purge connector state.

- `shutdown: optional object { purge }`

  - `purge: optional boolean`

    Purge connector state.

### Returns

- `errors: array of object { code, message }`

  - `code: number`

  - `message: string`

- `messages: array of object { code, message }`

  - `code: number`

  - `message: string`

- `result: object { submitted_at, reboot, restart, 2 more }`

  Interrupt action for a connector.

  - `submitted_at: string`

  - `reboot: optional object { purge }`

    - `purge: optional boolean`

      Purge connector state.

  - `restart: optional object { purge }`

    - `purge: optional boolean`

      Purge connector state.

  - `shutdown: optional object { purge }`

    - `purge: optional boolean`

      Purge connector state.

  - `triggered_at: optional string`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/magic/connectors/$CONNECTOR_ID/interrupts \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{}'
```

#### Response

```json
{
  "errors": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "messages": [
    {
      "code": 0,
      "message": "message"
    }
  ],
  "result": {
    "submitted_at": "submitted_at",
    "reboot": {
      "purge": true
    },
    "restart": {
      "purge": true
    },
    "shutdown": {
      "purge": true
    },
    "triggered_at": "triggered_at"
  },
  "success": true
}
```
