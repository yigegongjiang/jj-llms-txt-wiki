## Update a sinkhole

**put** `/accounts/{account_id}/intel/sinkholes/{sinkhole_id}`

Update the name or R2 configuration of the specified sinkhole.

### Path Parameters

- `account_id: string`

  Identifier.

- `sinkhole_id: string`

### Body Parameters

- `name: string`

  The name of the sinkhole.

- `r2_bucket: optional string`

  The name of the R2 bucket to store results. Required if you want to store large request bodies in R2.

- `r2_id: optional string`

  The id of the R2 instance. Required if you want to store large request bodies in R2.

- `r2_secret: optional string`

  The secret key for the R2 API token. Required if you want to store large request bodies in R2.

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

- `result: optional unknown`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes/$SINKHOLE_ID \
    -X PUT \
    -H 'Content-Type: application/json' \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY" \
    -d '{
          "name": "name"
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
  "result": {}
}
```
