# Analytics Query

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

## Domain Types

### Analytics Query Summary Response

- `AnalyticsQuerySummaryResponse object { currentTotal, previousTotal }`

  - `currentTotal: array of map[unknown]`

    Aggregated stats for the requested time range.

  - `previousTotal: array of map[unknown]`

    Aggregated stats for the equivalent preceding time range, for trend comparison.

### Analytics Query Timeseries Response

- `AnalyticsQueryTimeseriesResponse object { resolution, slots }`

  - `resolution: string`

    The resolution used for time bucketing.

  - `slots: array of map[unknown]`

    Time-bucketed result rows. Each slot contains a `time_bucket` field plus the requested stats and group-by dimensions.

### Analytics Query Top N Response

- `AnalyticsQueryTopNResponse = map[unknown]`

  Maps field names to values. Keys represent stat names and group-by column names. Values depend on the dataset (strings, numbers, booleans).

# Data Security

# Content Findings

## Top integrations by content findings

**post** `/accounts/{account_id}/analytics/query/data-security/content-findings/top-n`

Returns the top N integrations ranked by total content findings.

### Path Parameters

- `account_id: string`

### Body Parameters

- `filters: array of object { name, op, values }`

  Filters to apply. `findingType = content` is applied automatically for CASB data.

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

  Start of the query time range (inclusive). RFC3339.

- `n: number`

  Maximum number of integrations to return.

- `to: string`

  End of the query time range (exclusive). RFC3339.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/analytics/query/data-security/content-findings/top-n \
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
          "n": 10,
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
  "result": [
    {
      "integrationId": "123e4567-e89b-12d3-a456-426614174000",
      "integrationName": "Google Workspace",
      "total": 42
    },
    {
      "integrationId": "223e4567-e89b-12d3-a456-426614174001",
      "integrationName": "Microsoft 365",
      "total": 17
    }
  ],
  "success": true
}
```

## Domain Types

### Content Finding Top N Response

- `ContentFindingTopNResponse = map[unknown]`

  Maps field names to values. Keys represent stat names and group-by column names. Values depend on the dataset (strings, numbers, booleans).

# Findings

## Data security findings summary

**post** `/accounts/{account_id}/analytics/query/data-security/findings/summary`

Returns aggregate current-period and previous-period totals for CASB findings.

### Path Parameters

- `account_id: string`

### Body Parameters

- `filters: array of object { name, op, values }`

  Filters to apply.

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

  Start of the query time range (inclusive). RFC3339.

- `to: string`

  End of the query time range (exclusive). RFC3339.

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
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/analytics/query/data-security/findings/summary \
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
        "findingProduct": "Cloud",
        "findingType": "Content",
        "findingsTotal": 48291
      },
      {
        "findingProduct": "SaaS",
        "findingType": "Posture",
        "findingsTotal": 1205
      }
    ],
    "previousTotal": [
      {
        "findingProduct": "Cloud",
        "findingType": "Content",
        "findingsTotal": 41033
      },
      {
        "findingProduct": "SaaS",
        "findingType": "Posture",
        "findingsTotal": 982
      }
    ]
  },
  "success": true
}
```

## Data security findings timeseries

**post** `/accounts/{account_id}/analytics/query/data-security/findings/timeseries`

Returns merged time-bucketed CASB findings.

### Path Parameters

- `account_id: string`

### Body Parameters

- `filters: array of object { name, op, values }`

  Filters to apply.

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

  Start of the query time range (inclusive). RFC3339.

- `to: string`

  End of the query time range (exclusive). RFC3339.

### Returns

- `errors: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `messages: array of object { code, message }`

  - `code: optional number`

  - `message: optional string`

- `result: object { slots, resolution }`

  Merged CASB and CDE findings timeseries result.

  - `slots: array of map[unknown]`

    Contains time-bucketed result rows. Each slot includes a `timestamp` plus `content` and `posture` maps with `cloud` and `saas` keys.

  - `resolution: optional string`

    Always null for this endpoint.

- `success: boolean`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/analytics/query/data-security/findings/timeseries \
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
    "slots": [
      {
        "content": {
          "cloud": 150,
          "saas": 23
        },
        "posture": {
          "cloud": 0,
          "saas": 5
        },
        "timestamp": "2024-11-05T00:00:00Z"
      },
      {
        "content": {
          "cloud": 180,
          "saas": 30
        },
        "posture": {
          "cloud": 0,
          "saas": 7
        },
        "timestamp": "2024-11-06T00:00:00Z"
      }
    ]
  },
  "success": true
}
```

## Domain Types

### Finding Summary Response

- `FindingSummaryResponse object { currentTotal, previousTotal }`

  - `currentTotal: array of map[unknown]`

    Aggregated stats for the requested time range.

  - `previousTotal: array of map[unknown]`

    Aggregated stats for the equivalent preceding time range, for trend comparison.

### Finding Timeseries Response

- `FindingTimeseriesResponse object { slots, resolution }`

  Merged CASB and CDE findings timeseries result.

  - `slots: array of map[unknown]`

    Contains time-bucketed result rows. Each slot includes a `timestamp` plus `content` and `posture` maps with `cloud` and `saas` keys.

  - `resolution: optional string`

    Always null for this endpoint.
