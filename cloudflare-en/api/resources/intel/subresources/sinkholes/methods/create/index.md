## Create a new sinkhole for your account

**post** `/accounts/{account_id}/intel/sinkholes`

Create a new sinkhole. Logs of large request bodies will be truncated, but the full request body can be recorded in R2. If you wish to record large request bodies in R2, include the R2 key ID, key secret, and bucket name in the request body.

### Path Parameters

- `account_id: string`

  Identifier.

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

- `result: optional Sinkhole`

  - `id: optional string`

    The unique identifier for the sinkhole.

  - `account_tag: optional string`

    The account tag that owns this sinkhole.

  - `created_on: optional string`

    The date and time when the sinkhole was created.

  - `modified_on: optional string`

    The date and time when the sinkhole was last modified.

  - `name: optional string`

    The name of the sinkhole.

  - `r2_bucket: optional string`

    The name of the R2 bucket to store results.

  - `r2_id: optional string`

    The id of the R2 instance.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/intel/sinkholes \
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
  "result": {
    "id": "93defa6e909e464e8c89a85859f36d3c",
    "account_tag": "233f45e61fd1f7e21e1e154ede4q2859",
    "created_on": "2023-05-12T12:21:56.777653Z",
    "modified_on": "2023-06-18T03:13:34.123321Z",
    "name": "my_sinkhole",
    "r2_bucket": "my_bucket",
    "r2_id": "example_r2_id"
  }
}
```
