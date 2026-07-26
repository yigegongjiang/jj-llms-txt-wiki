## List tag key summary

**get** `/accounts/{account_id}/tags/summary`

Lists all distinct tag keys and their distinct values across resources in an account.

### Path Parameters

- `account_id: string`

  Identifier.

### Query Parameters

- `cursor: optional string`

  Cursor for pagination.

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

- `result: optional array of object { key, values }`

  Contains an array of tag keys with their distinct values.

  - `key: string`

    A tag key.

  - `values: array of string`

    All distinct values for this tag key.

- `result_info: optional object { count, cursor }`

  - `count: optional number`

    Indicates the number of results returned in the current page.

  - `cursor: optional string`

    Provides a cursor for the next page of results. Include this value in the next request to continue pagination.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/tags/summary \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_API_KEY"
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
  "result": [
    {
      "key": "environment",
      "values": [
        "production",
        "staging"
      ]
    },
    {
      "key": "team",
      "values": [
        "engineering",
        "platform"
      ]
    }
  ],
  "result_info": {
    "count": 20,
    "cursor": "eyJhY2NvdW50X2lkIjoxMjM0NTY3ODkwfQ"
  }
}
```
