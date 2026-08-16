## Get information about the specified slot

**get** `/accounts/{account_id}/cni/slots/{slot}`

Gets information about a specific infrastructure slot allocation.

### Path Parameters

- `account_id: string`

  Customer account tag

- `slot: string`

### Returns

- `id: string`

  Slot ID

- `facility: object { address, name }`

  - `address: array of string`

  - `name: string`

- `occupied: boolean`

  Whether the slot is occupied or not

- `site: string`

- `speed: string`

- `account: optional string`

  Customer account tag

- `ccr_device_name: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cni/slots/$SLOT \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "id": "182bd5e5-6e1a-4fe4-a799-aa6d9a6ab26e",
  "facility": {
    "address": [
      "string"
    ],
    "name": "name"
  },
  "occupied": true,
  "site": "site",
  "speed": "speed",
  "account": "account",
  "ccr_device_name": "ccr_device_name"
}
```
