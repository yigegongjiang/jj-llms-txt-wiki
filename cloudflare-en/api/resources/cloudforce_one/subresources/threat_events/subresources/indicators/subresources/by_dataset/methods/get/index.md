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

- `relatedEvents: optional array of object { datasetId, eventId, eventDate }`

  - `datasetId: string`

  - `eventId: string`

  - `eventDate: optional string`

    ISO 8601 date of the related event. Null for legacy relationships created before event-date tracking was added.

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
```
