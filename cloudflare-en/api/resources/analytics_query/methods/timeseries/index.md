## Query analytics timeseries

**post** `/accounts/{account_id}/analytics/query/{dataset}/timeseries`

Returns time-bucketed analytics data for a dataset. Includes time slots, each containing the requested stats, group-by dimensions, and resolution-controlled bucket size (e.g. `hour`, `day`).

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

- `resolution: string`

  Time bucket size for grouping results. Controls the granularity of the returned time slots.

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

- `result: object { resolution, slots }`

  - `resolution: string`

    The resolution used for time bucketing.

  - `slots: array of map[unknown]`

    Time-bucketed result rows. Each slot contains a `time_bucket` field plus the requested stats and group-by dimensions.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/analytics/query/$DATASET/timeseries \
    -H 'Content-Type: application/json' \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
    -d '{
          "filters": [
            {
              "name": "allowed",
              "op": "eq",
              "values": [
                true
              ]
            }
          ],
          "from": "2024-11-01T00:00:00Z",
          "groupBy": [
            "country",
            "allowed"
          ],
          "resolution": "day",
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
    "resolution": "hour",
    "slots": [
      {
        "appName": "Slack",
        "bytesTotal": 1048576,
        "time_bucket": "2024-11-05T00:00:00Z"
      },
      {
        "appName": "Slack",
        "bytesTotal": 2097152,
        "time_bucket": "2024-11-05T01:00:00Z"
      }
    ]
  },
  "success": true
}
```
