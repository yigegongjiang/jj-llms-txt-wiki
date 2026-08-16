## Lists indicators across multiple datasets

**get** `/accounts/{account_id}/cloudforce-one/events/indicators`

Retrieves indicators across specified datasets, ordered by createdAt descending then UUID, dataset ID, and shard ID ascending. Use datasetIds=all or datasetIds=* to query all datasets for the account. If no datasetIds provided, uses the default dataset.

### Path Parameters

- `account_id: string`

  Account ID.

### Query Parameters

- `cache: optional "from-graph"`

  Cache strategy. 'from-graph' serves results from the graph-node KV cache when all requested UUIDs are cached; falls back to normal path on partial/zero hit. Cannot be combined with `cursor`.

  - `"from-graph"`

- `createdAfter: optional string`

  Filter indicators created on or after this date. Must use ISO 8601 format (e.g., '2024-01-15T00:00:00Z').

- `createdBefore: optional string`

  Filter indicators created on or before this date. Must use ISO 8601 format (e.g., '2024-12-31T23:59:59Z').

- `cursor: optional string`

  Opaque cursor from a previous response's `pagination.cursor`. When provided, all filters, datasetIds, page, `pageSize`, `includeTags` and `relatedEventsLimit` come from the cursor — do not resend them. Sending any filter, `page`, `pageSize`, `includeTags`, `relatedEventsLimit`, `includeTotalCount=true`, or `cache=from-graph` alongside a cursor yields a 400 `CursorFilterConflictError`. A cursor issued for a different entity, an unsupported version, or a dataset that has since been reconfigured as analytics-only yields a 400 `InvalidCursorError`.

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

  Whether to compute total count via COUNT(*). Defaults to false for performance. total_count is null unless this is true and the complete fan-out succeeds.

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

- `properties: object { completeness, indicators, pagination }`

  - `completeness: object { properties, type }`

    - `properties: object { complete, failedDatasets, failedShards, warnings }`

      - `complete: object { type }`

        - `type: string`

      - `failedDatasets: object { items, type }`

        - `items: object { type }`

          - `type: string`

        - `type: string`

      - `failedShards: object { items, type }`

        - `items: object { properties, type }`

          - `properties: object { datasetId, shardId }`

            - `datasetId: object { type }`

              - `type: string`

            - `shardId: object { type }`

              - `type: string`

          - `type: string`

        - `type: string`

      - `warnings: object { items, type }`

        - `items: object { type }`

          - `type: string`

        - `type: string`

    - `type: string`

  - `indicators: object { items, type }`

    - `items: object { createdAt, indicatorType, updatedAt, 5 more }`

      - `createdAt: string`

      - `indicatorType: string`

      - `updatedAt: string`

      - `uuid: string`

      - `value: string`

      - `datasetId: optional string`

        The dataset ID this indicator belongs to. Included in list responses.

      - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

        - `datasetId: string`

        - `eventId: string`

        - `eventDate: optional string`

          ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

      - `tags: optional array of object { categoryName, uuid, value }`

        - `categoryName: optional string`

        - `uuid: optional string`

        - `value: optional string`

    - `type: string`

  - `pagination: object { properties, type }`

    - `properties: object { count, cursor, has_more, 4 more }`

      - `count: object { type }`

        - `type: string`

      - `cursor: object { description, nullable, type }`

        - `description: string`

        - `nullable: boolean`

        - `type: string`

      - `has_more: object { description, type }`

        - `description: string`

        - `type: string`

      - `page: object { type }`

        - `type: string`

      - `per_page: object { type }`

        - `type: string`

      - `total_count: object { description, nullable, type }`

        - `description: string`

        - `nullable: boolean`

        - `type: string`

      - `total_count_is_exact: object { description, type }`

        - `description: string`

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
    "completeness": {
      "properties": {
        "complete": {
          "type": "boolean"
        },
        "failedDatasets": {
          "items": {
            "type": "string"
          },
          "type": "array"
        },
        "failedShards": {
          "items": {
            "properties": {
              "datasetId": {
                "type": "string"
              },
              "shardId": {
                "type": "string"
              }
            },
            "type": "object"
          },
          "type": "array"
        },
        "warnings": {
          "items": {
            "type": "string"
          },
          "type": "array"
        }
      },
      "type": "object"
    },
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
            "eventId": "event-uuid-456",
            "eventDate": "2024-06-15T00:00:00Z"
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
        "cursor": {
          "description": "Opaque cursor for the next page. Pass back as the `cursor` query param on the next request. `null` when the sequence has ended, when the encoded cursor would exceed the safe URL length, or when this endpoint served the request from a backend that does not support cursor pagination (analytics R2 path).",
          "nullable": true,
          "type": "string"
        },
        "has_more": {
          "description": "True when more pages exist after this one. Present on both offset and cursor paths.",
          "type": "boolean"
        },
        "page": {
          "type": "number"
        },
        "per_page": {
          "type": "number"
        },
        "total_count": {
          "description": "Exact matching count when requested and fan-out is complete; otherwise null.",
          "nullable": true,
          "type": "number"
        },
        "total_count_is_exact": {
          "description": "Whether total_count is exact across the complete query fan-out.",
          "type": "boolean"
        }
      },
      "type": "object"
    }
  },
  "type": "object"
}
```
