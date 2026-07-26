## Query analytics summary

**post** `/accounts/{account_id}/analytics/query/{dataset}/summary`

Returns aggregate summary stats for a dataset. Includes current-period and previous-period totals for trend comparison.

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

- `result: object { currentTotal, previousTotal }`

  - `currentTotal: array of map[unknown]`

    Aggregated stats for the requested time range.

  - `previousTotal: array of map[unknown]`

    Aggregated stats for the equivalent preceding time range, for trend comparison.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/analytics/query/$DATASET/summary \
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
          "from": "2024-11-01T00:00:00Z",
          "groupBy": [
            "string"
          ],
          "stats": [
            "attemptsTotal"
          ],
          "to": "2024-11-08T00:00:00Z"
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
  "result": {
    "currentTotal": [
      {
        "attemptsTotal": 48291
      }
    ],
    "previousTotal": [
      {
        "attemptsTotal": 41033
      }
    ]
  },
  "success": true
}
```
