## Aggregate indicators by column(s)

**get** `/accounts/{account_id}/cloudforce-one/events/indicators/aggregate`

Aggregate threat indicators by one or more columns (e.g., indicatorType, value) across datasets. Returns top-N groups ordered by count.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `aggregateBy: string`

  Column(s) to aggregate by - single column or comma-separated list (e.g., 'indicatorType', 'value', 'indicatorType,value')

- `createdAfter: optional string`

  Filter indicators created after this date (ISO 8601 format, e.g., '2024-01-01')

- `createdBefore: optional string`

  Filter indicators created before this date (ISO 8601 format, e.g., '2024-12-31')

- `datasetIds: optional array of string`

  Dataset ID(s) to filter by. Can be a single dataset ID or comma-separated list. If not provided, aggregates across all accessible datasets

- `eventDateAfter: optional string`

  For measure=relationships: only count event links whose eventDate is on/after this date (ISO 8601). Use to bound 'top indicator' to recent activity.

- `eventDateBefore: optional string`

  For measure=relationships: only count event links whose eventDate is on/before this date (ISO 8601).

- `limit: optional number`

  Maximum number of aggregation results to return (1-100)

- `measure: optional "indicators" or "relationships"`

  What to count per group: 'indicators' (catalog rows, default) or 'relationships' (linked events per indicator). Use 'relationships' for 'top indicator by event activity'.

  - `"indicators"`

  - `"relationships"`

- `tagUuid: optional string`

  Scope to indicators associated with this tag/actor UUID. Combine with measure=relationships for 'top indicator for an actor'.

### Returns

- `aggregateBy: string`

  Column(s) that were aggregated by

- `aggregations: array of object { count }`

  Array of aggregation results with dynamic fields based on aggregateBy columns

  - `count: number`

    Number of indicators for this aggregation

- `failedDatasets: number`

  Number of datasets whose aggregation failed and were excluded from the result

- `total: number`

  Total count in the aggregation: indicator rows when measure=indicators, or linked-event rows when measure=relationships

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/indicators/aggregate \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "aggregateBy": "aggregateBy",
  "aggregations": [
    {
      "count": 0
    }
  ],
  "failedDatasets": 0,
  "total": 0
}
```
