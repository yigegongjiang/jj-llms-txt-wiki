## Get queries

**get** `/accounts/{account_id}/cloudforce-one/v2/brand-protection/domain/queries`

Get all saved brand protection queries for an account

### Path Parameters

- `account_id: string`

### Query Parameters

- `id: optional string`

- `page: optional number`

  Optional page number for paginated list requests. Defaults to 1 when only per_page is supplied. Omit page and per_page to preserve the legacy full-list response.

- `per_page: optional number`

  Optional number of queries per page for paginated list requests. Defaults to 100 when only page is supplied. Maximum 100. Omit page and per_page to preserve the legacy full-list response.

### Returns

- `errors: array of object { message, code }`

  - `message: string`

  - `code: optional string or number`

    - `string`

    - `number`

- `messages: array of object { message, code }`

  - `message: string`

  - `code: optional string or number`

    - `string`

    - `number`

- `result: array of object { created, parameters, query_id, 3 more }  or object { created, parameters, query_id, 3 more }`

  - `array of object { created, parameters, query_id, 3 more }`

    - `created: string`

    - `parameters: object { string_matches, max_time, min_time }`

      - `string_matches: array of object { pattern }`

        - `pattern: string`

      - `max_time: optional string`

      - `min_time: optional string`

    - `query_id: number`

    - `query_tag: string`

    - `scan: boolean`

    - `updated: string`

  - `object { created, parameters, query_id, 3 more }`

    - `created: string`

    - `parameters: object { string_matches, max_time, min_time }`

      - `string_matches: array of object { pattern }`

        - `pattern: string`

      - `max_time: optional string`

      - `min_time: optional string`

    - `query_id: number`

    - `query_tag: string`

    - `scan: boolean`

    - `updated: string`

- `success: boolean`

- `result_info: optional object { count, page, per_page, total_count }`

  Present on paginated list responses when page or per_page is supplied.

  - `count: number`

  - `page: number`

  - `per_page: number`

  - `total_count: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/v2/brand-protection/domain/queries \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "errors": [
    {
      "message": "message",
      "code": "string"
    }
  ],
  "messages": [
    {
      "message": "message",
      "code": "string"
    }
  ],
  "result": [
    {
      "created": "created",
      "parameters": {
        "string_matches": [
          {
            "pattern": "x"
          }
        ],
        "max_time": "max_time",
        "min_time": "min_time"
      },
      "query_id": 0,
      "query_tag": "query_tag",
      "scan": true,
      "updated": "updated"
    }
  ],
  "success": true,
  "result_info": {
    "count": 0,
    "page": 1,
    "per_page": 1,
    "total_count": 0
  }
}
```
