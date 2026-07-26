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
