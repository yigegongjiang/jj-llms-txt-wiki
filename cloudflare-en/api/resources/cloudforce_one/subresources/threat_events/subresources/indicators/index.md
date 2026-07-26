# Indicators

## Lists indicators across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/indicators`

Retrieves a paginated list of indicators across specified datasets. Use datasetIds=all or datasetIds=* to query all datasets for the account. If no datasetIds provided, uses the default dataset.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cache: optional "from-graph"`

  Cache strategy. 'from-graph' serves results from the graph-node KV cache when all requested UUIDs are cached; falls back to normal path on partial/zero hit.

  - `"from-graph"`

- `createdAfter: optional string`

  Filter indicators created on or after this date. Must use ISO 8601 format (e.g., '2024-01-15T00:00:00Z').

- `createdBefore: optional string`

  Filter indicators created on or before this date. Must use ISO 8601 format (e.g., '2024-12-31T23:59:59Z').

- `datasetIds: optional array of string`

  Dataset IDs to query indicators from (array of UUIDs), or special value 'all' or '*' to query all datasets. If not provided, uses the default dataset.

- `format: optional "json" or "stix2" or "taxii"`

  Output format for indicator data. 'json' returns the default format, 'stix2' returns STIX 2.1 Indicator SDOs, 'taxii' returns a TAXII 2.1 Envelope with Content-Type application/taxii+json;version=2.1.

  - `"json"`

  - `"stix2"`

  - `"taxii"`

- `includeTags: optional boolean`

  Whether to include full tag details for each indicator. Defaults to true.

- `includeTotalCount: optional boolean`

  Whether to compute accurate total count via COUNT(*). Defaults to false for performance. When false, total_count is an approximation.

- `indicatorType: optional string`

- `name: optional string`

  Filter indicators by value using substring match (LIKE). Legacy alternative to structured search.

- `page: optional number`

- `pageSize: optional number`

- `relatedEvents: optional array of string`

  Filter by related event IDs

- `relatedEventsLimit: optional number`

  Limit the number of related events returned per indicator. Default: 2. Set to 0 for none, -1 for all events.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType, uuid. Supports operators: equals, not, contains, startsWith, endsWith, gt, lt, gte, lte, like, in, find. Use the 'in' operator with an array value to bulk-check up to 100 indicators in a single request, e.g. search=[{"field":"value","op":"in","value":["evil.com","bad.org"]}]. Multiple conditions are AND'd together. Max 10 conditions per request.

  - `field: "value" or "indicatorType" or "uuid"`

    The indicator field to search on. Allowed: value, indicatorType, uuid.

    - `"value"`

    - `"indicatorType"`

    - `"uuid"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk lookup of up to 100 values at once, e.g. {field:'value', op:'in', value:['evil.com','bad.org']}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: string or array of string`

    Search value. String for most operators. Array of strings for 'in' operator (max 100 items).

    - `string`

    - `array of string`

- `source: optional "do" or "r2catalog"`

  Read backend. 'do' (default) reads Durable Object storage. 'r2catalog' reads R2 Data Catalog (admin-only, experimental; supports a subset of search fields).

  - `"do"`

  - `"r2catalog"`

- `tags: optional array of string`

  Filter by tag values or UUIDs. Indicators must have at least one of the specified tags (OR logic). Supports both tag UUID and tag value.

- `tagSearch: optional array of object { field, op, value }`

  Structured tag-metadata filter as a JSON array of {field, op, value} objects. Operates against the per-dataset IndicatorTag mirror so you can find indicators by tag attributes (origin country, motive, sophistication, priority, etc.) without a separate Tags lookup. Common dashboard usage: drill from a country into indicators, e.g. tagSearch=[{"field":"originCountryISO","op":"in","value":["IR","CN"]}]. Country values may be passed as alpha-2, alpha-3, name, or alias (e.g. "iran"). Operators: equals, not, gt/gte/lt/lte (numeric only), contains/like/find/startsWith/endsWith (string only), in. AND-joined across entries; combined with `tags`, a matching tag must satisfy both. Max 10 entries per request, max 100 values per 'in'. Performance notes: `originCountryISO` uses its B-tree index for equals/not/in. `priority` uses its B-tree index for numeric comparisons. Other string columns (`actorCategory`, `motive`, etc.) are case-insensitive and unindexed; current catalog size makes this a non-issue. `endsWith` and `aliasGroupNames` contains/like are leading-wildcard scans and slow on large result sets. `aliasGroupNames` matches on the JSON-encoded text, so substrings can cross alias boundaries ("apt28" also matches "apt280" when both appear in the same tag's alias list).

  - `field: "value" or "categoryId" or "actorCategory" or 9 more`

    Tag mirror field to filter on. Allowed: value, categoryId, actorCategory, aliasGroupNames, attributionConfidence, attributionOrganization, motive, opsecLevel, originCountryISO, sophisticationLevel, priority, analyticPriority. Filters operate against the per-dataset IndicatorTag mirror (which is kept in sync with the Tags SoT by the tag-propagation workflow).

    - `"value"`

    - `"categoryId"`

    - `"actorCategory"`

    - `"aliasGroupNames"`

    - `"attributionConfidence"`

    - `"attributionOrganization"`

    - `"motive"`

    - `"opsecLevel"`

    - `"originCountryISO"`

    - `"sophisticationLevel"`

    - `"priority"`

    - `"analyticPriority"`

  - `op: "equals" or "not" or "gt" or 9 more`

    Search operator. Use 'in' for bulk OR within a single field, e.g. {field:"originCountryISO", op:"in", value:["IR","CN"]}.

    - `"equals"`

    - `"not"`

    - `"gt"`

    - `"gte"`

    - `"lt"`

    - `"lte"`

    - `"like"`

    - `"contains"`

    - `"startsWith"`

    - `"endsWith"`

    - `"in"`

    - `"find"`

  - `value: optional string or number or array of string or number`

    Search value. String or number for most operators. Array for 'in' (max 100 items). Country values may be passed as alpha-2, alpha-3, name, or common alias (e.g. "iran", "IR", "IRN") and are normalized to alpha-2 server-side.

    - `string`

    - `number`

    - `array of string or number`

      - `string`

      - `number`

### Returns

- `properties: object { indicators, pagination }`

  - `indicators: object { items, type }`

    - `items: object { createdAt, indicatorType, updatedAt, 5 more }`

      - `createdAt: string`

      - `indicatorType: string`

      - `updatedAt: string`

      - `uuid: string`

      - `value: string`

      - `datasetId: optional string`

        The dataset ID this indicator belongs to. Included in list responses.

      - `relatedEvents: optional array of object { datasetId, eventId }`

        - `datasetId: string`

        - `eventId: string`

      - `tags: optional array of object { categoryName, uuid, value }`

        - `categoryName: optional string`

        - `uuid: optional string`

        - `value: optional string`

    - `type: string`

  - `pagination: object { properties, type }`

    - `properties: object { count, page, per_page, total_count }`

      - `count: object { type }`

        - `type: string`

      - `page: object { type }`

        - `type: string`

      - `per_page: object { type }`

        - `type: string`

      - `total_count: object { type }`

        - `type: string`

    - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "properties": {
    "indicators": {
      "items": {
        "createdAt": "2022-04-01T00:00:00Z",
        "indicatorType": "domain",
        "updatedAt": "2022-04-01T00:00:00Z",
        "uuid": "12345678-1234-1234-1234-1234567890ab",
        "value": "malicious-domain.com",
        "datasetId": "dataset-uuid-123",
        "relatedEvents": [
          {
            "datasetId": "dataset-uuid-123",
            "eventId": "event-uuid-456"
          }
        ],
        "tags": [
          {
            "categoryName": "categoryName",
            "uuid": "uuid",
            "value": "value"
          }
        ]
      },
      "type": "array"
    },
    "pagination": {
      "properties": {
        "count": {
          "type": "number"
        },
        "page": {
          "type": "number"
        },
        "per_page": {
          "type": "number"
        },
        "total_count": {
          "type": "number"
        }
      },
      "type": "object"
    }
  },
  "type": "object"
}
```

## Domain Types

### Indicator List Response

- `IndicatorListResponse object { properties, type }`

  - `properties: object { indicators, pagination }`

    - `indicators: object { items, type }`

      - `items: object { createdAt, indicatorType, updatedAt, 5 more }`

        - `createdAt: string`

        - `indicatorType: string`

        - `updatedAt: string`

        - `uuid: string`

        - `value: string`

        - `datasetId: optional string`

          The dataset ID this indicator belongs to. Included in list responses.

        - `relatedEvents: optional array of object { datasetId, eventId }`

          - `datasetId: string`

          - `eventId: string`

        - `tags: optional array of object { categoryName, uuid, value }`

          - `categoryName: optional string`

          - `uuid: optional string`

          - `value: optional string`

      - `type: string`

    - `pagination: object { properties, type }`

      - `properties: object { count, page, per_page, total_count }`

        - `count: object { type }`

          - `type: string`

        - `page: object { type }`

          - `type: string`

        - `per_page: object { type }`

          - `type: string`

        - `total_count: object { type }`

          - `type: string`

      - `type: string`

  - `type: string`

# Aggregate

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

## Domain Types

### Aggregate List Response

- `AggregateListResponse object { aggregateBy, aggregations, failedDatasets, total }`

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

# Types

## Lists indicator types across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/indicator-types`

List indicator types across one or more datasets for the account.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `datasetIds: optional array of string`

  Array of dataset IDs to query indicator types from. If not provided, queries all datasets for the account.

### Returns

- `items: object { type }`

  - `type: string`

- `type: string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/indicator-types \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "items": {
    "type": "string"
  },
  "type": "array"
}
```

## Domain Types

### Type List Response

- `TypeListResponse object { items, type }`

  - `items: object { type }`

    - `type: string`

  - `type: string`

# By Dataset

## Lists indicators

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators`

This method is deprecated. Please use /events/indicators to retrieve a paginated list of indicators.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

### Query Parameters

- `indicatorType: optional string`

- `name: optional string`

  Filter by indicator value (substring match)

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId }`

    - `datasetId: string`

    - `eventId: string`

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "indicators": [
    {
      "createdAt": "2022-04-01T00:00:00Z",
      "indicatorType": "domain",
      "updatedAt": "2022-04-01T00:00:00Z",
      "uuid": "12345678-1234-1234-1234-1234567890ab",
      "value": "malicious-domain.com",
      "datasetId": "dataset-uuid-123",
      "relatedEvents": [
        {
          "datasetId": "dataset-uuid-123",
          "eventId": "event-uuid-456"
        }
      ],
      "tags": [
        {
          "categoryName": "categoryName",
          "uuid": "uuid",
          "value": "value"
        }
      ]
    }
  ],
  "pagination": {
    "page": 0,
    "pageSize": 0,
    "totalCount": 0,
    "totalPages": 0
  }
}
```

## Reads an indicator

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators/{indicator_id}`

Retrieves a specific indicator by its UUID.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

- `indicator_id: string`

  Indicator UUID.

### Returns

- `createdAt: string`

- `indicatorType: string`

- `updatedAt: string`

- `uuid: string`

- `value: string`

- `datasetId: optional string`

  The dataset ID this indicator belongs to. Included in list responses.

- `relatedEvents: optional array of object { datasetId, eventId }`

  - `datasetId: string`

  - `eventId: string`

- `tags: optional array of object { categoryName, uuid, value }`

  - `categoryName: optional string`

  - `uuid: optional string`

  - `value: optional string`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators/$INDICATOR_ID \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
{
  "createdAt": "2022-04-01T00:00:00Z",
  "indicatorType": "domain",
  "updatedAt": "2022-04-01T00:00:00Z",
  "uuid": "12345678-1234-1234-1234-1234567890ab",
  "value": "malicious-domain.com",
  "datasetId": "dataset-uuid-123",
  "relatedEvents": [
    {
      "datasetId": "dataset-uuid-123",
      "eventId": "event-uuid-456"
    }
  ],
  "tags": [
    {
      "categoryName": "categoryName",
      "uuid": "uuid",
      "value": "value"
    }
  ]
}
```

## Domain Types

### By Dataset List Response

- `ByDatasetListResponse object { indicators, pagination }`

  - `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

    - `createdAt: string`

    - `indicatorType: string`

    - `updatedAt: string`

    - `uuid: string`

    - `value: string`

    - `datasetId: optional string`

      The dataset ID this indicator belongs to. Included in list responses.

    - `relatedEvents: optional array of object { datasetId, eventId }`

      - `datasetId: string`

      - `eventId: string`

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`

### By Dataset Get Response

- `ByDatasetGetResponse object { createdAt, indicatorType, updatedAt, 5 more }`

  - `createdAt: string`

  - `indicatorType: string`

  - `updatedAt: string`

  - `uuid: string`

  - `value: string`

  - `datasetId: optional string`

    The dataset ID this indicator belongs to. Included in list responses.

  - `relatedEvents: optional array of object { datasetId, eventId }`

    - `datasetId: string`

    - `eventId: string`

  - `tags: optional array of object { categoryName, uuid, value }`

    - `categoryName: optional string`

    - `uuid: optional string`

    - `value: optional string`

# Tags

## List mirrored tags for an indicator dataset

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/indicators/tags`

Returns all mirrored tags from the indicator dataset (DO mirror table). No pagination.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset ID.

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/indicators/tags \
    -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN"
```

#### Response

```json
[
  {}
]
```

## Domain Types

### Tag List Response

- `TagListResponse = array of unknown`

  Array of mirror tag rows
