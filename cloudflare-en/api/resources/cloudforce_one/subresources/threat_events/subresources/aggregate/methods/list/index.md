## Aggregate events by single or multiple columns with optional date filtering

**get** `/accounts/{account_id}/cloudforce-one/events/aggregate`

Aggregate threat events by one or more columns (e.g., attacker, targetIndustry) with optional date filtering and daily grouping. Supports multi-dimensional aggregation for cross-analysis.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `aggregateBy: string`

  Column(s) to aggregate by - single column or comma-separated list (e.g., 'attacker', 'targetIndustry', 'attacker,targetIndustry')

- `datasetId: optional array of string`

  Dataset ID(s) to filter by. Can be a single dataset ID, comma-separated list, or array. If not provided, uses default dataset

- `endDate: optional string`

  End date for filtering (ISO 8601 format, e.g., '2024-12-31')

- `groupByDate: optional boolean`

  Whether to group results by date (daily aggregation)

- `limit: optional number`

  Maximum number of results to return

- `startDate: optional string`

  Start date for filtering (ISO 8601 format, e.g., '2024-01-01')

### Returns

- `aggregateBy: string`

  Column(s) that were aggregated by

- `aggregations: array of object { count, date }`

  Array of aggregation results with dynamic fields based on aggregateBy columns

  - `count: number`

    Number of events for this aggregation

  - `date: optional string`

    Date (if groupByDate is true)

- `total: number`

  Total number of events in the aggregation

- `dateRange: optional object { endDate, startDate }`

  Date range used for filtering

  - `endDate: optional string`

  - `startDate: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/aggregate \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "aggregateBy": "aggregateBy",
  "aggregations": [
    {
      "count": 0,
      "date": "date"
    }
  ],
  "total": 0,
  "dateRange": {
    "endDate": "endDate",
    "startDate": "startDate"
  }
}
```
