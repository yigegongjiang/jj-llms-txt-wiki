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
