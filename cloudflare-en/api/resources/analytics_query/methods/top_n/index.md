## Query analytics top-N

**post** `/accounts/{account_id}/analytics/query/{dataset}/top-n`

Returns the top N results for a dataset by a specified stat. Includes an array of result rows, each containing the requested stats and group-by dimensions.

### Path Parameters

- `account_id: string`

- `dataset: string`

### Body Parameters

- `filters: array of object { name, op, values }`

  Filters to apply before aggregating results.

  - `name: string`

    Specifies the column name to filter on. Requires a valid column for the target dataset (e.g. `country`, `allowed`, `appId`).

  - `op: string`

    Filter operator. Common values: `eq`, `neq`, `in`, `not_in`, `gt`, `lt`, `gte`, `lte`.

  - `values: array of string or boolean or number`

    Values to match against. Type depends on the column.

    - `string`

    - `boolean`

    - `number`

- `from: string`

  The start of the query time range (inclusive). RFC3339 format with timezone is required (e.g. `2024-11-05T00:00:00Z`).

- `groupBy: array of string`

  Specifies the column names to group results by. Requires valid columns for the target dataset.

- `n: number`

  Maximum number of results to return.

- `orderBy: string`

  Specifies the stat name for sorting results in descending order. Requires a valid stat for the target dataset.

- `stats: array of string`

  Specifies the stat names to include in results. Requires valid stats for the target dataset (e.g. `attemptsTotal`, `bytesTotal`).

- `to: string`

  Specifies the end of the query time range (exclusive). Requires RFC3339 format with timezone.

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `result: array of map[unknown]`

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/analytics/query/$DATASET/top-n \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "filters": [
            {
              "name": "country",
              "op": "in",
              "values": [
                "US",
                "CA",
                "GB"
              ]
            }
          ],
          "from": "2024-11-05T00:00:00Z",
          "groupBy": [
            "appName",
            "appCategory"
          ],
          "n": 10,
          "orderBy": "bytesTotal",
          "stats": [
            "bytesTotal",
            "requestsTotal"
          ],
          "to": "2024-11-06T00:00:00Z"
        }'
```

#### Response

```json
{
  "errors": [],
  "messages": [
    {
      "code": 1000,
      "message": "API in beta: expect breaking changes."
    }
  ],
  "result": [
    {
      "appCategory": "Collaboration",
      "appName": "Slack",
      "bytesTotal": 10485760,
      "requestsTotal": 1024
    },
    {
      "appCategory": "File Storage",
      "appName": "Dropbox",
      "bytesTotal": 5242880,
      "requestsTotal": 512
    }
  ],
  "success": true
}
```
