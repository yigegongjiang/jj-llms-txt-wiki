# By Dataset

## List indicators related to a tag within a dataset (deprecated)

**get** `/accounts/{account_id}/cloudforce-one/events/dataset/{dataset_id}/tags/{tag_uuid}/indicators`

This endpoint is deprecated. Use GET /:account_id/events/tags/:tag_uuid/indicators with the optional datasetIds query parameter instead. Returns indicators associated with the provided tag UUID within a single dataset's indicator shards, with pagination.

### Path Parameters

- `account_id: string`

  Account ID.

- `dataset_id: string`

  Dataset UUID.

- `tag_uuid: string`

  Tag UUID.

### Query Parameters

- `indicatorType: optional string`

- `page: optional number`

- `pageSize: optional number`

- `relatedEvent: optional array of string`

  Filter indicators by related event UUID(s). Multiple UUIDs can be provided by repeating the parameter.

- `search: optional array of object { field, op, value }`

  Structured search as a JSON array of {field, op, value} objects. Searchable fields: value, indicatorType. Multiple conditions are AND'd together. Max 10 conditions per request.

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

### Returns

- `indicators: array of object { createdAt, indicatorType, updatedAt, 5 more }`

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

- `pagination: object { page, pageSize, totalCount, totalPages }`

  - `page: number`

  - `pageSize: number`

  - `totalCount: number`

  - `totalPages: number`

### Example

```http
curl https://api.cloudflare.com/client/v4/accounts/$ACCOUNT_ID/cloudforce-one/events/dataset/$DATASET_ID/tags/$TAG_UUID/indicators \
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

    - `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

      - `datasetId: string`

      - `eventId: string`

      - `eventDate: optional string`

        ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

    - `tags: optional array of object { categoryName, uuid, value }`

      - `categoryName: optional string`

      - `uuid: optional string`

      - `value: optional string`

  - `pagination: object { page, pageSize, totalCount, totalPages }`

    - `page: number`

    - `pageSize: number`

    - `totalCount: number`

    - `totalPages: number`
