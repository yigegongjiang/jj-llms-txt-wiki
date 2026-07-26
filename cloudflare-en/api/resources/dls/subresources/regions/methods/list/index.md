## List DLS regions for an account

**get** `/accounts/{account_id}/dls/regions`

List the DLS regions (managed and custom) available to an account.

### Path Parameters

- `account_id: string`

  Identifier of a Cloudflare account.

### Query Parameters

- `cursor: optional string`

  Opaque token for cursor-based pagination. Omit for the first page. Pass the value from a previous response to fetch the next page.

- `per_page: optional number`

- `type: optional "managed" or "custom"`

  Filter regions by type. Omit to return all regions.

  - `"managed"`

  - `"custom"`

### Returns

- `errors: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `messages: array of object { code, message, error_chain }`

  - `code: number`

  - `message: string`

  - `error_chain: optional array of unknown`

    Optional upstream error context for APIv4 errors that wrap downstream service failures.

- `result: array of object { id, created_on, modified_on, 4 more }`

  - `id: string`

  - `created_on: string`

  - `modified_on: string`

  - `name: string`

  - `region_key: string`

  - `version: number`

  - `version_created_on: string`

- `result_info: object { count, cursor, per_page }`

  - `count: number`

    Number of items in the current page.

  - `cursor: string`

    Opaque cursor for the next page. Empty string when there are no more results.

  - `per_page: number`

    Maximum number of items per page.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/dls/regions \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "messages": [
    {
      "code": 1000,
      "message": "message",
      "error_chain": [
        {}
      ]
    }
  ],
  "result": [
    {
      "id": "id",
      "created_on": "2019-12-27T18:11:19.117Z",
      "modified_on": "2019-12-27T18:11:19.117Z",
      "name": "name",
      "region_key": "x",
      "version": 0,
      "version_created_on": "2019-12-27T18:11:19.117Z"
    }
  ],
  "result_info": {
    "count": 0,
    "cursor": "cursor",
    "per_page": 0
  },
  "success": true
}
```
