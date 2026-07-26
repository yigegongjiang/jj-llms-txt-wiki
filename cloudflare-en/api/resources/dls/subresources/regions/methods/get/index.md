## Get a DLS region

**get** `/accounts/{account_id}/dls/regions/{region_id}`

Retrieve a single DLS region (managed or custom) by ID or region key.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

- `region_id: string`

  UUID of the region (custom or managed) or region_key of a managed region.

### Returns

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `result: object { id, created_on, modified_on, 4 more }`

  - `id: string`

  - `created_on: string`

  - `modified_on: string`

  - `name: string`

  - `region_key: string`

  - `version: number`

  - `version_created_on: string`

- `success: boolean`

- `errors: optional array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regions/$REGION_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "result": {
    "id": "id",
    "created_on": "2019-12-27T18:11:19.117Z",
    "modified_on": "2019-12-27T18:11:19.117Z",
    "name": "name",
    "region_key": "x",
    "version": 0,
    "version_created_on": "2019-12-27T18:11:19.117Z"
  },
  "success": true,
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ]
}
```
